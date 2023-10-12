use clap::{Parser, Subcommand};
use polyticker_lib::common::trade::Trade;
use polyticker_lib::exchange::Bucket;
use polyticker_lib::request::stocks::aggregates::Aggregates;
use polyticker_lib::request::stocks::grouped_daily::GroupedDaily;
use polyticker_lib::websocket::crypto::{Crypto, CryptoTradeEvent};
use polyticker_lib::websocket::stocks::Stocks;
use std::fs::File;

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
    Aggregates {},
    GroupedDaily {},
    WebSocket {},
    ExchangeBuckets {
        #[arg(short, long, default_value = "1")]
        refresh_rate: u64,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

            match aggregates
                .get_stock_data(
                    stocks_ticker,
                    multiplier,
                    timespan,
                    from,
                    to,
                    adjusted,
                    sort,
                    limit,
                )
                .await
            {
                Ok(response) => println!("{:#?}", response),
                Err(e) => println!("Error: {}", e),
            }
        }
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
        Commands::WebSocket {} => {
            let api_key = cli.polygon_api_key;

            //    let mut channel = Stocks::open_data_channel(api_key, 1000).await;
            let mut channel = Crypto::open_data_channel(api_key, "XT.*".to_string(), 1000).await;

            while let Some(event) = channel.recv().await {
                println!("{:#?}", event);
            }
        }
        Commands::ExchangeBuckets { refresh_rate } => {
            let api_key = cli.polygon_api_key;

            //    let mut channel = Stocks::open_data_channel(api_key, 1000).await;
            let mut channel = Crypto::open_data_channel(api_key, "XT.*".to_string(), 1000).await;

            let mut bucket = Bucket::new("BTC");
            // start a time to print buckets every "refresh_rate" seconds
            let mut interval =
                tokio::time::interval(tokio::time::Duration::from_secs(refresh_rate));
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        bucket.print_trades()?;
                    }
                    event = channel.recv() => {
                        match &event {
                            Some(event) => process_trade(event.to_owned(), &mut bucket)?,
                            None => break,
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn process_trade(event: CryptoTradeEvent, bucket: &mut Bucket) -> anyhow::Result<()> {
    let trade = event.get_trade()?;
    if trade.symbol == "BTC" {
        bucket.add_trade(Box::new(event));
    }
    Ok(())
}
