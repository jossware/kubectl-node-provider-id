use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
pub struct Cli {
    /// The name of the kubeconfig context to use.
    #[arg(short, long, env("NODEPID_CONTEXT"))]
    pub context: Option<String>,

    /// The template to use when displaying the provider id
    #[arg(short, long, env("NODEPID_TEMPLATE"), default_value = "{:url}")]
    pub template: String,

    /// Output format.
    #[arg(short, long, env("NODEPID_OUTPUT_FORMAT"), default_value = "table")]
    pub output_format: OutputFormat,

    /// The name of the node to get
    pub name: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    Table,
    Plain,
    Json,
    Yaml,
}
