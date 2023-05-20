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

pub fn search_pack(JsOptions { query, num_res }: &JsOptions) {
    match query {
        Some(s) => {
            println!("Hello query")
        }
        None => println!("None"),
    }
}
