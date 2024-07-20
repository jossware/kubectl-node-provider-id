mod cli;
mod print;

use clap::Parser;
use cli::{Cli, OutputFormat};
use k8s_openapi::api::core::v1::Node;
use kube::{Api, ResourceExt};
use node_provider_labeler::{provider_id::ProviderID, template::Template};
use serde::Serialize;
use std::str::FromStr;

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
    let cli = Cli::parse();

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

    let template = &cli.template;
    let nodes =
        get_nodes_with_provider_id(kube::Client::try_from(client_config)?, cli.name, template)
            .await?;

    match cli.output_format {
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
        let mut provider_id = node
            .spec
            .as_ref()
            .and_then(|spec| spec.provider_id.as_ref())
            .cloned()
            .unwrap_or("".to_string());

        if !provider_id.is_empty() {
            let pid = ProviderID::new(&provider_id)?;
            provider_id = template.render(&pid)?;
        }

        let name = node.name_any();
        Ok(Self { name, provider_id })
    }
}
