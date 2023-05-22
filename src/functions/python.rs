use clap::Args;

#[derive(Args)]
pub struct PyOptions {
    ///package name to search for
    #[arg(short = 'q', long = "query")]
    query: Option<String>,
    ///number of results to display
    #[arg(short = 'n', long = "num_results", default_value_t = 5)]
    num_res: u8,
}

pub async fn search_pack(PyOptions { query, num_res }: &PyOptions) {}
