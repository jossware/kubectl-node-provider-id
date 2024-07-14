use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
pub struct Cli {
    /// The name of the kubeconfig context to use.
    #[arg(long)]
    pub context: Option<String>,

    /// The name of the node to get
    pub name: Option<String>,

    /// Output format.
    #[arg(short, long, default_value = "table")]
    pub output_format: OutputFormat,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    Table,
    Plain,
    Json,
    Yaml,
}
