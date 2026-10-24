mod analysis;
mod chart;
mod stock;

use clap::Parser;

use analysis::{closing_extrema, is_volatile};
use stock::{fetch_six_months, normalize_ticker};

// Download and chart the last six months of daily stock prices.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    // Stock symbol, for example AAPL or SHOP.TO
    ticker: String,

    // Path of the PNG chart file
    #[arg(short, long, default_value = "stock_chart.png")]
    output: String,
}

#[tokio::main]
async fn main() {
    // clap handles -h, --help, missing arguments, and unknown options.
    let args = Cli::parse();

    if let Err(message) = run(args).await {
        eprintln!("Error: {message}");
        std::process::exit(1);
    }
}

async fn run(args: Cli) -> Result<(), String> {
    let ticker = normalize_ticker(&args.ticker)?;
    println!("Downloading six months of prices for {ticker}...");

    let days = fetch_six_months(&ticker).await?;
    println!("Received {} daily records.", days.len());

    // Analysis module: teammate can extend this without changing the fetch code.
    let (minimum, maximum) = closing_extrema(&days)
        .ok_or_else(|| format!("No prices available to analyze for {ticker}."))?;
    println!("Minimum close: {:.2} on {}", minimum.close, minimum.date);
    println!("Maximum close: {:.2} on {}", maximum.close, maximum.date);
    println!(
        "Volatile days (>2%): {}",
        days.iter().filter(|day| is_volatile(day)).count()
    );

    // Chart module: teammate can replace draw_chart while keeping its signature.
    chart::draw_chart(&ticker, &days, &args.output)
        .map_err(|e| format!("Failed to create chart '{}': {e}", args.output))?;
    println!("Chart saved to: {}", args.output);

    Ok(())
}
