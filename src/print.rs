use k8s_openapi::serde_json;

use crate::NodeProviderID;
use std::io::Write;

pub fn table(nodes: Vec<NodeProviderID>) -> color_eyre::Result<()> {
    let mut tw = tabwriter::TabWriter::new(std::io::stdout()).padding(3);
    tw.write_all(b"NODE\tPROVIDER ID\n")?;
    for node in nodes {
        let l = format!("{}\t{}\n", node.name, node.provider_id);
        tw.write_all(l.as_bytes())?;
    }
    tw.flush()?;

    Ok(())
}

pub fn plain(nodes: Vec<NodeProviderID>) -> color_eyre::Result<()> {
    for node in nodes {
        println!("{}", node.provider_id);
    }
    Ok(())
}

pub fn json(nodes: Vec<NodeProviderID>) -> color_eyre::Result<()> {
    let json = serde_json::to_string(&nodes)?;
    println!("{}", json);
    Ok(())
}

pub fn yaml(nodes: Vec<NodeProviderID>) -> color_eyre::Result<()> {
    let yaml = serde_yaml::to_string(&nodes)?;
    println!("{}", yaml);
    Ok(())
}
