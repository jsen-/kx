use itertools::Itertools;
use serde::Deserialize;
use skim::{Skim, SkimOptionsBuilder};
use std::error::Error;
use std::io::Cursor;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct ClusterData<'a> {
    server: &'a str,
}
#[derive(Debug, Deserialize)]
struct Cluster<'a> {
    name: &'a str,
    cluster: ClusterData<'a>,
}

#[derive(Debug, Deserialize)]
struct User<'a> {
    name: &'a str,
}

#[derive(Debug, Deserialize)]
struct ContextData<'a> {
    cluster: &'a str,
    user: &'a str,
}

#[derive(Debug, Deserialize)]
struct Context<'a> {
    name: &'a str,
    context: ContextData<'a>,
}

#[derive(Debug, Deserialize)]
struct Kubeconfig<'a> {
    // kind: &'a str,
    // apiVersion: &'a str,
    // preferences: std::collections::HashMap<&'a str, serde_json::Value>,
    clusters: Vec<Cluster<'a>>,
    users: Vec<User<'a>>,
    contexts: Vec<Context<'a>>,
    #[serde(rename = "current-context")]
    current_context: &'a str,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let output = Command::new("kubectl")
        .args(&["config", "view", "-o", "json"])
        .output()?;

    let kubeconfig = serde_json::from_slice::<Kubeconfig>(&output.stdout)?;

    let cluster_selection = kubeconfig
        .clusters
        .iter()
        .map(|cl| cl.name)
        .intersperse("\n")
        .flat_map(|s| s.bytes())
        .collect::<Vec<u8>>();

    // println!("{:#?}", kubeconfig);

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .build()
        .unwrap();

    let selected_items = Skim::run_with(&options, Some(Box::new(Cursor::new(cluster_selection))))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
        print!("{}: {}{}", item.get_index(), item.get_output_text(), "\n");
    }

    // let input = "11111\n22222\n333333333".to_string();

    // let selected_items = Skim::run_with(&options, Some(Box::new(Cursor::new(input))))
    //     .map(|out| out.selected_items)
    //     .unwrap_or_else(|| Vec::new());

    // for item in selected_items.iter() {
    //     print!("{}: {}{}", item.get_index(), item.get_output_text(), "\n");
    // }
    Ok(())
}
