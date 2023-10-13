use crate::common::trade::{Trade, TradeData};
use prettytable::row;
use std::collections::BTreeMap;

pub struct Bucket {
    symbol: String,
    currency: String,
    data: BTreeMap<i64, Box<dyn Trade>>,
}

impl Bucket {
    pub fn new(symbol: &str, currency: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            currency: currency.to_string(),
            data: BTreeMap::new(),
        }
    }
    // add_trade, error out if symbol is different, the i64 is the exchange number
    pub fn add_trade(&mut self, trade: Box<dyn Trade>) -> anyhow::Result<()> {
        let trade_data = trade.get_trade()?;
        if trade_data.symbol != self.symbol && trade_data.currency != self.currency {
            return Err(anyhow::Error::msg("Symbol mismatch"));
        }
        self.data.insert(trade_data.exchange_id, trade);
        Ok(())
    }

    // print the trades in order of exchange id, in a grid with prettytables-rs
    pub fn print_trades(&self) -> anyhow::Result<()> {
        let mut table = prettytable::Table::new();
        table.add_row(row![
            "Exchange ID",
            "Symbol",
            "Currency",
            "Price",
            "Timestamp"
        ]);
        for (_, trade) in self.data.iter() {
            let trade_data = trade.get_trade()?;
            table.add_row(row![
                trade_data.exchange_id,
                trade_data.symbol,
                trade_data.currency,
                trade_data.price,
                trade_data.timestamp
            ]);
        }
        table.printstd();
        Ok(())
    }
}
