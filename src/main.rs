mod cli;
mod print;

use clap::Parser;
use cli::{Cli, OutputConfig, OutputFormat};
use color_eyre::eyre::Context;
use etcetera::{choose_base_strategy, BaseStrategy};
use k8s_openapi::api::core::v1::Node;
use kube::{Api, ResourceExt};
use node_provider_labeler::{provider_id::ProviderID, template::Template};
use serde::Serialize;
use std::{path::Path, str::FromStr};

fn init_tracing() {
    std::env::set_var(
        "RUST_LOG",
        std::env::var("RUST_LOG").unwrap_or_else(|_| "warn,kube_client=error".to_string()),
    );
    tracing_subscriber::fmt::init();
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    init_tracing();
    color_eyre::config::HookBuilder::default()
        .panic_section("consider reporting the bug on github")
        .install()?;
    run().await
}

async fn run() -> color_eyre::Result<()> {
    let base_strategy = choose_base_strategy().wrap_err("discovering configuration directory")?;
    let config_file = Path::new(&base_strategy.config_dir()).join("kubectl-node-provider-id.yaml");
    let output_config = {
        if config_file.exists() {
            let data =
                std::fs::read_to_string(config_file).wrap_err("reading configuration file")?;
            serde_yaml::from_str(&data).wrap_err("loading configuration from file")?
        } else {
            OutputConfig::default()
        }
    };

    let cli = Cli::parse();

    let template = cli
        .template
        .unwrap_or(output_config.template.unwrap_or_default());
    let output_format = cli
        .output_format
        .unwrap_or(output_config.format.unwrap_or_default());

    let client_config = match cli.context {
        Some(ref context) => {
            kube::Config::from_kubeconfig(&kube::config::KubeConfigOptions {
                context: Some(context.clone()),
                ..Default::default()
            })
            .await?
        }
        None => kube::Config::infer().await?,
    };

    let nodes = get_nodes_with_provider_id(
        kube::Client::try_from(client_config)?,
        cli.name,
        &template.to_string(),
    )
    .await?;

    match output_format {
        OutputFormat::Plain => print::plain(nodes)?,
        OutputFormat::Json => print::json(nodes)?,
        OutputFormat::Yaml => print::yaml(nodes)?,
        _ => print::table(nodes)?,
    }

    Ok(())
}

async fn get_nodes_with_provider_id(
    client: kube::Client,
    node_name: Option<String>,
    template: &str,
) -> color_eyre::Result<Vec<NodeProviderID>> {
    let nodes: Api<Node> = Api::all(client);

    let t = node_provider_labeler::template::AnnotationTemplate::from_str(template)?;
    let nodes = {
        if let Some(node_name) = node_name {
            let node = nodes.get(&node_name).await?;
            vec![NodeProviderID::new(&node, &t)?]
        } else {
            let list = nodes.list(&Default::default()).await?;
            list.items
                .iter()
                .map(|n| NodeProviderID::new(n, &t))
                .collect::<color_eyre::Result<Vec<_>>>()?
        }
    };

    Ok(nodes)
}

#[derive(Serialize)]
struct NodeProviderID {
    name: String,
    provider_id: String,
}

impl NodeProviderID {
    fn new(node: &Node, template: &dyn Template) -> color_eyre::Result<Self> {
        let node_name = node.name_any();
        let mut provider_id = node
            .spec
            .as_ref()
            .and_then(|spec| spec.provider_id.as_ref())
            .cloned()
            .unwrap_or("".to_string());

        if !provider_id.is_empty() {
            let pid = ProviderID::new(&node_name, &provider_id)?;
            provider_id = template.render(&pid)?;
        }

        let name = node.name_any();
        Ok(Self { name, provider_id })
    }
}
