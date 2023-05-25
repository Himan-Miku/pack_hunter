use crate::structs::js_structs::ResponseObject;
use clap::Args;

#[derive(Args, Debug)]
pub struct JsOptions {
    ///package name to search for
    #[arg(short = 'q', long = "query")]
    query: Option<String>,
    ///number of results to display
    #[arg(short = 'n', long = "num_results", default_value_t = 5)]
    num_res: u8,
}

pub async fn search_pack(
    JsOptions { query, num_res }: &JsOptions,
) -> Result<ResponseObject, reqwest::Error> {
    match query {
        Some(s) => {
            let url = format!("https://registry.npmjs.org/-/v1/search?text={}&size={}&popularity=1.0&quality=0.0&maintenance=0.0", s, num_res);

            let client = reqwest::Client::new();

            let response = client.get(&url).send().await?;

            let data = response.json().await?;

            println!("{:#?}", data);

            Ok(data)
        }
        None => {
            let url = format!("https://registry.npmjs.org/-/v1/search?text=react&size=5&popularity=1.0&quality=0.0&maintenance=0.0");

            let client = reqwest::Client::new();

            let response = client.get(&url).send().await?;

            let data = response.json().await?;

            println!("{:#?}", data);

            Ok(data)
        }
    }
}
