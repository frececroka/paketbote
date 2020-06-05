use fehler::throws;
use serde::Deserialize;

use crate::error::Error;

#[derive(Debug, Deserialize)]
struct Response<T> {
    results: Vec<T>
}

#[derive(Debug, Deserialize)]
struct Info {
    #[serde(rename(deserialize = "ID"))]
    id: u32,
    #[serde(rename(deserialize = "Name"))]
    name: String,
    #[serde(rename(deserialize = "Version"))]
    version: String,
}

#[throws]
pub fn check_aur_version(packages: &[&str]) -> Vec<(String, String)> {
    let client = reqwest::blocking::Client::new();

    let mut query = vec![("v", "5"), ("type", "info")];
    for package in packages {
        query.push(("arg[]", package));
    }

    let response = client
        .get("https://aur.archlinux.org/rpc.php")
        .query(&query)
        .send()?;
    if !response.status().is_success() {
        Err(format!("response failed: {}", response.status()))?
    }

    let response: Response<Info> = response.json()?;
    response.results.into_iter()
        .map(|p| (p.name, p.version))
        .collect()
}
