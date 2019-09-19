use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ClusterData<'a> {
    pub server: &'a str,
}
#[derive(Debug, Deserialize)]
pub struct Cluster<'a> {
    pub name: &'a str,
    pub cluster: ClusterData<'a>,
}

#[derive(Debug, Deserialize)]
pub struct User<'a> {
    pub name: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct ContextData<'a> {
    pub cluster: &'a str,
    pub user: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct Context<'a> {
    pub name: &'a str,
    pub context: ContextData<'a>,
}

#[derive(Debug, Deserialize)]
pub struct Kubeconfig<'a> {
    // kind: &'a str,
    // apiVersion: &'a str,
    // preferences: std::collections::HashMap<&'a str, serde_json::Value>,
    pub clusters: Vec<Cluster<'a>>,
    pub users: Vec<User<'a>>,
    pub contexts: Vec<Context<'a>>,
    #[serde(rename = "current-context")]
    pub current_context: &'a str,
}

pub fn read<'a>(config: &'a [u8]) -> Result<Kubeconfig<'a>, Box<dyn Error>> {
    let kcfg = serde_json::from_slice::<Kubeconfig>(config)?;
    Ok(kcfg)
}