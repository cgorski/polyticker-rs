use clap::{Parser, Subcommand};
use std::fs::File;
use polyticker_lib::request::market::aggregates::Aggregates;
use polyticker_lib::request::market::grouped_daily::GroupedDaily;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long, env = "POLYGON_API_KEY")]
    polygon_api_key: String,
    #[clap(subcommand)]
    command: Commands,
}

/// Subcommands available
#[derive(Subcommand, Debug)]
enum Commands {
    /// Subcommand for handling tables
    Aggregates {

    },
    GroupedDaily {

    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Aggregates {} => {
            let api_key = cli.polygon_api_key;
            let stocks_ticker = "AAPL";
            let multiplier = "1";
            let timespan = "day";
            let from = "2023-01-09";
            let to = "2023-01-09";
            let adjusted = true;
            let sort = "asc";
            let limit = 120;
            let aggregates = Aggregates::new(api_key);

            match aggregates.get_stock_data(stocks_ticker, multiplier, timespan, from, to, adjusted, sort, limit).await {
                Ok(response) => println!("{:#?}", response),
                Err(e) => println!("Error: {}", e),
            }
        },
        Commands::GroupedDaily {} => {
            let api_key = cli.polygon_api_key;
            let date = "2023-01-09";
            let adjusted = true;
            let include_otc = true;
            let grouped_daily = GroupedDaily::new(api_key);

            match grouped_daily.get_data(date, adjusted, include_otc).await {
                Ok(response) => println!("{:#?}", response),
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}