# Polyticker

Polyticker is a powerful command line tool written in Rust, designed to work seamlessly with the Polygon.io API for efficient retrieval and display of stock data. It features a dual architecture with both a binary tool (`polyticker`) and a Rust library (`polyticker-lib`) for extensibility and integration with other tools.

This project is open-source and dual licensed under both Apache 2 and MIT licenses.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
  - [Exchange Buckets](#exchange-buckets)
  - [Aggregate Information](#aggregate-information)
  - [Web Socket Stream](#web-socket-stream)
- [Integration with `polyticker-lib`](#integration-with-polyticker-lib)
- [Contribute](#contribute)
- [License](#license)

## Installation

```
cargo install polyticker
```

## Usage

### Exchange Buckets

To get a continuous stream of the latest quotes from various exchanges, use:

```
polyticker exchange-buckets
```

This command will display output in a tabular format, similar to:

```
+-------------+--------+----------+----------+-------------------------+
| Exchange ID | Symbol | Currency | Price    | Timestamp               |
+-------------+--------+----------+----------+-------------------------+
| 1           | BTC    | USD      | 26717.31 | 2023-10-12 21:15:38 UTC |
| ...                                                             |
+-------------+--------+----------+----------+-------------------------+
```

This view helps users in comparing the latest stock data across different exchanges at a glance.

### Aggregate Information

For fetching aggregate stock information:

```
polyticker --aggregate
```

Sample output:

```json
ApiResponse {
    "ticker": "AAPL",
    ...
    "results": [
        {
            "close_price": 130.15,
            ...
            "volume_weighted_avg_price": 131.6292,
        },
    ],
    "next_url": None,
}
```

This gives an in-depth aggregate data of the stock.

### Web Socket Stream

For real-time trade data using WebSocket:

```
polyticker --web-socket
```

Sample output:

```json
CryptoTradeEvent {
    "event_type": "XT",
    "pair": "BTC-USD",
    ...
    "received_timestamp": "2023-10-12T22:45:18Z",
}
```

This provides real-time updates on various crypto trades.

## Integration with `polyticker-lib`

For developers looking to extend the capabilities of polyticker or to integrate it with other tools, the `polyticker-lib` library offers a suite of functionalities out of the box. More information and documentation on this can be found in the `polyticker-lib` directory.

## Contribute

Contributions are welcome! If you have a feature request, bug report, or wish to contribute to the code:

1. Fork this repository.
2. Create a new branch for your changes.
3. Submit a pull request.

Make sure to write tests for any new features or changes and ensure all tests are passing.

## License

This project is dual-licensed under the Apache 2 and MIT licenses. You can choose between one of them if you use this work.

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for more details. 

---

For any additional help or questions, feel free to open an issue or reach out to the maintainers. Happy trading!
