use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    /// The name of the kubeconfig context to use.
    #[arg(long)]
    pub context: Option<String>,
}
