use clap::Args;
use rand::Rng;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

use crate::structs::rs_structs::ResponseObj;

#[derive(Args)]
pub struct RsOptions {
    ///crate name to search for
    #[arg(short = 'q', long = "query")]
    query: Option<String>,
    ///number of results to display
    #[arg(short = 'n', long = "num_results", default_value_t = 5)]
    num_res: u8,
}

fn get_random_crate() -> String {
    let crates = vec![
        "tokio",
        "rocket",
        "miette",
        "hyper",
        "axum",
        "serde",
        "serde_json",
        "anyhow",
        "thiserror",
        "log",
        "env_logger",
        "pretty_env_logger",
        "tracing",
        "crono",
        "nom",
        "textwrap",
        "reqwest",
        "rand",
    ];

    let random_crate = crates[rand::thread_rng().gen_range(0..crates.len())];

    random_crate.to_owned()
}

pub async fn search_pack(
    RsOptions { query, num_res }: &RsOptions,
) -> Result<ResponseObj, reqwest::Error> {
    match query {
        Some(q) => {
            let url = format!(
                "https://crates.io/api/v1/crates?q={}&sort=downloads&per_page={}",
                q, num_res
            );

            let client = reqwest::Client::new();

            let mut headers = HeaderMap::new();
            headers.insert(USER_AGENT, HeaderValue::from_static("himan-crawler"));

            let response = client.get(&url).headers(headers).send().await?;

            let data = response.json().await?;

            println!("{:#?}", data);

            Ok(data)
        }
        None => {
            let crate_q = get_random_crate();

            let url = format!(
                "https://crates.io/api/v1/crates?q={}&sort=downloads&per_page=5",
                crate_q
            );

            let client = reqwest::Client::new();

            let mut headers = HeaderMap::new();
            headers.insert(USER_AGENT, HeaderValue::from_static("himan-crawler"));

            let response = client.get(&url).headers(headers).send().await?;

            let data = response.json().await?;

            println!("{:#?}", data);

            Ok(data)
        }
    }
}
