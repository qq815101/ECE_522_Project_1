# ECE_522_Project_1
Stock Market Monitor
Stock Monitor (Rust)
A command-line program that downloads six months of daily stock quotes from Yahoo Finance, plots closing prices, marks volatile trading days, and prints the minimum and maximum closing prices with their dates.
1. Crates
Crate	Reason
clap 4.6 (derive)	Parses the required ticker and optional output filename; generates --help.
yahoo_finance_api 5.0	Downloads daily OHLC stock data using Yahoo Finance.
tokio 1	Runs asynchronous HTTP requests made by the Yahoo Finance client.
plotters 0.3	Creates a PNG line plot with volatility high-low bars.


2. Financial-analysis algorithm
For each daily quote, volatility is the difference between its high and low, relative to the daily closing price:
volatility_percent = ((high - low) / close) * 100
The day is marked volatile only when volatility_percent > 2. This uses a strict inequality; exactly 2% is not volatile. The denominator is the unadjusted daily closing price, matching the closing-price series. We find the minimum and maximum closing prices across the downloaded trading days using Rust iterator min_by and max_by with f64::total_cmp.
3. Chart setup
plotters draws a red line for the daily closing prices. On volatile days, blue vertical bars span the intraday low to high and blue circles mark the close. Unlike candlesticks, the chart prioritizes the closing-price trend and uses bars to highlight unusual intraday ranges. The chart title contains the ticker and period; the x-axis is trading date and the y-axis is price. The y-axis automatically expands to include the lowest daily low and highest daily high plus a small margin. Chart dimensions are 1280 × 720 pixels.
4. Setup
Install the stable Rust toolchain with rustup from https://rustup.rs/ and ensure cargo is on your PATH. An internet connection is required for downloading dependencies (first build) and fetching quotes (every run).
From this project directory:
cargo build
cargo test
5. Usage
cargo run -- --help
cargo run -- AAPL
cargo run -- SHOP.TO --output shop_chart.png
The command prints how many daily records were downloaded, the minimum and maximum closing prices with dates, and the number of volatile days. By default, the PNG chart is saved as stock_chart.png in the directory from which the command was executed; --output changes the filename/path. Open the image with your normal image viewer.
Sample output (numbers and dates vary with market data):
Downloading six months of prices for AAPL...
Received 126 daily records.
Minimum close: ... on YYYY-MM-DD
Maximum close: ... on YYYY-MM-DD
Volatile days (>2%): ...
Chart saved to: stock_chart.png
For invalid symbols, Yahoo outages, and empty datasets, the application prints an error instead of panicking. An invalid ticker or failed request yields a nonzero exit code. -h / --help is handled by clap.
6. Module ownership / integration
- src/stock.rs: ticker normalization, Yahoo Finance download, quote validation, shared StockDay record.
- src/main.rs: argument parsing, overall orchestration, and error handling.
- src/analysis.rs: closing_extrema(&[StockDay]) and is_volatile(&StockDay); may be replaced by the analysis teammate.
- src/chart.rs: draw_chart(ticker: &str, days: &[StockDay], filename: &str) -> Result<(), Box<dyn Error>>; may be replaced by the charting teammate.
Note: Yahoo data is third-party market data and availability is not guaranteed. Date strings are derived from the quote timestamp using UTC. In unusual cases, a market's local trading date could differ from UTC; use the exchange timezone if exact local-session date handling is required. Closing values are unadjusted, not dividend/split-adjusted (adjclose).