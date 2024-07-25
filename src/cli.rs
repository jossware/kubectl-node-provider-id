use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Template(String);

impl Default for Template {
    fn default() -> Self {
        Template("{:url}".to_string())
    }
}

impl Display for Template {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Template {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[derive(Debug, Parser)]
pub(crate) struct Cli {
    /// The name of the kubeconfig context to use. Defaults to the current
    /// context
    #[arg(short, long, env("NODEPID_CONTEXT"))]
    pub context: Option<String>,

    /// The template to use when displaying the provider id. Defaults to
    /// "{:url}" which will return the full provider id.
    ///
    /// You can use a template to define how you want information extracted from
    /// `.spec.providerID`. The plugin will parse the `providerID` and make the
    /// discovered information available via tokens you can use in your
    /// templates. It splits the value of the ID (the part after the provider
    /// "protocol") by "/" and it is possible to access individual parts by
    /// index or by named helpers.
    ///
    /// Example with an AWS Provider ID: "aws://us-west-2/i-0abcdef1234567890".
    ///
    /// | Token       | Value                               |
    /// |-------------|-------------------------------------|
    /// | {:provider} | aws                                 |
    /// | {:last}     | i-0abcdef1234567890                 |
    /// | {:first}    | us-west-2                           |
    /// | {:all}      | us-west-2/i-0abcdef1234567890       |
    /// | {:url}      | aws://us-west-2/i-0abcdef1234567890 |
    /// | {0}         | us-west-2                           |
    /// | {1}         | i-0abcdef1234567890                 |
    ///
    /// The default template can also be specified in the configuration file.
    #[arg(short, long, env("NODEPID_TEMPLATE"), verbatim_doc_comment)]
    pub template: Option<Template>,

    /// Output format. Defaults to table. The default output format can also be
    /// specified in the configuration file.
    #[arg(short, long, env("NODEPID_OUTPUT_FORMAT"))]
    pub output_format: Option<OutputFormat>,

    /// The name of the node to get
    pub name: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum OutputFormat {
    Table,
    Plain,
    Json,
    Yaml,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Table
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct OutputConfig {
    pub format: Option<OutputFormat>,
    pub template: Option<Template>,
}
