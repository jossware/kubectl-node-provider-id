mod cli;
mod errors;

use std::io::Write;

use clap::Parser;
use cli::{Cli, OutputFormat};
use k8s_openapi::api::core::v1::Node;
use kube::{Api, ResourceExt};

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

    let nodes = nodes(kube::Client::try_from(client_config)?, cli.name).await?;

    match cli.output_format {
        OutputFormat::Plain => print_plain(nodes)?,
        _ => print_table(nodes)?,
    }

    Ok(())
}

fn print_table(nodes: Vec<NodeProviderID>) -> color_eyre::Result<()> {
    let mut tw = tabwriter::TabWriter::new(std::io::stdout()).padding(3);
    tw.write_all(b"NODE\tPROVIDER ID\n")?;
    for node in nodes {
        let l = format!("{}\t{}\n", node.node.name_any(), node.provider_id);
        tw.write_all(l.as_bytes())?;
    }
    tw.flush()?;

    Ok(())
}

fn print_plain(nodes: Vec<NodeProviderID>) -> color_eyre::Result<()> {
    for node in nodes {
        println!("{}", node.provider_id);
    }
    Ok(())
}

async fn nodes(
    client: kube::Client,
    node_name: Option<String>,
) -> color_eyre::Result<Vec<NodeProviderID>> {
    let nodes: Api<Node> = Api::all(client);

    let nodes = {
        if let Some(node_name) = node_name {
            let node = nodes.get(&node_name).await?;
            vec![NodeProviderID::new(&node)?]
        } else {
            let list = nodes.list(&Default::default()).await?;
            list.items
                .iter()
                .map(NodeProviderID::new)
                .collect::<color_eyre::Result<Vec<_>>>()?
        }
    };

    Ok(nodes)
}

struct NodeProviderID {
    provider_id: String,
    node: Node,
}

impl NodeProviderID {
    fn new(node: &Node) -> color_eyre::Result<Self> {
        let provider_id = node
            .spec
            .as_ref()
            .and_then(|spec| spec.provider_id.as_ref())
            .cloned()
            .unwrap_or("".to_string());

        Ok(Self {
            provider_id,
            node: node.clone(),
        })
    }
}
