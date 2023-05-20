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

struct Package {
    name: String,
    version: String,
    description: String,
    links: Links,
    publisher: Publisher,
}

struct Links {
    npm: String,
    homepage: String,
    repository: String,
    bugs: String,
}

struct Publisher {
    username: String,
    email: String,
}

pub async fn search_pack(JsOptions { query, num_res }: &JsOptions) -> Result<(), reqwest::Error> {
    match query {
        Some(s) => {}
        None => println!("None"),
    }
}
