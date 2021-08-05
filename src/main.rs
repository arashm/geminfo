mod linesplit;

use std::env;
use ansi_term::Colour::{Green, Yellow};
use ansi_term::Style;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Geminfo {
    name: String,
    version: String,
    info: String,
    source_code_uri: String,
    homepage_uri: String,
    version_created_at: String,
    licenses: Vec<String>,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    for gem_name in args[1..].iter() {
        fetch_and_print_gem_info(gem_name).await;
    };
}

async fn fetch_and_print_gem_info(gem_name: &str) {
    let geminfo: Geminfo = fetch_gem_data(gem_name).await.unwrap();
    let parsed_date = DateTime::parse_from_rfc3339(&geminfo.version_created_at)
        .unwrap()
        .format("%d %B %Y")
        .to_string();
    let splitted_info = linesplit::split_by_chars(&geminfo.info, 77).join("\n\t");

    println!(
        "* {} - {} ({})\n\t{}\n\t[ {} | {} ]\n",
        Style::new().bold().fg(Yellow).paint(geminfo.name),
        Style::new().fg(Green).paint(geminfo.version),
        Style::new().fg(Green).paint(parsed_date),
        splitted_info,
        geminfo.source_code_uri,
        geminfo.licenses[0]
    );
}

async fn fetch_gem_data(gem_name: &str) -> Result<Geminfo, reqwest::Error> {
    let url: String = format!("https://rubygems.org/api/v1/gems/{}.json", gem_name.to_string());
    Ok(reqwest::get(url).await?.json::<Geminfo>().await?)
}
