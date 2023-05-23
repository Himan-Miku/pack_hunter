use clap::Args;
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

pub async fn search_pack(
    RsOptions { query, num_res }: &RsOptions,
) -> Result<ResponseObj, reqwest::Error> {
    match query {
        Some(q) => {
            let url = format!(
                "https://crates.io/api/v1/crates?q={}&sort=downloads&per_page=5",
                q
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
            let url = format!("https://crates.io/api/v1/crates?q=clap&sort=downloads&per_page=5");

            let client = reqwest::Client::new();

            let response = client.get(&url).send().await?;

            let data = response.json().await?;

            println!("{:#?}", data);

            Ok(data)
        }
    }
}
