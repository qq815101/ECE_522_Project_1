use crate::analysis::is_volatile;
use crate::stock::StockDay;
use plotters::prelude::*;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;

pub fn draw_chart(ticker: &str, days: &[StockDay], filename: &str) -> Result<(), Box<dyn Error>> {
    // A chart cannot be created without stock data.
    if days.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No stock data available to create the chart.",
        )
        .into());
    }

    // Validate the price values used by the chart.
    if days.iter().any(|day| {
        !day.low.is_finite()
            || !day.high.is_finite()
            || !day.close.is_finite()
            || day.low > day.high
    }) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Stock data contains invalid price values.",
        )
        .into());
    }

    // Find the lowest intraday low and highest intraday high.
    let min_price = days.iter().map(|day| day.low).fold(f64::INFINITY, f64::min);

    let max_price = days
        .iter()
        .map(|day| day.high)
        .fold(f64::NEG_INFINITY, f64::max);

    // Add padding so the price line does not touch the chart edges.
    let price_range = max_price - min_price;

    let padding = if price_range > 0.0 {
        price_range * 0.05
    } else {
        (max_price.abs() * 0.01).max(0.01)
    };

    let y_min = (min_price - padding).max(0.0);
    let y_max = max_price + padding;

    // Use the trading-day index for the horizontal axis.
    let x_max = (days.len() - 1).max(1) as f64;

    // Create the parent directory if the user supplied one.
    let output_path = Path::new(filename);

    if let Some(parent) = output_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    // Create a 1280 x 720 PNG image.
    let root = BitMapBackend::new(filename, (1280, 720)).into_drawing_area();

    root.fill(&WHITE)?;

    // Configure the chart title, axes, and price range.
    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("{ticker} Closing Price - Last Six Months"),
            ("sans-serif", 28).into_font(),
        )
        .margin(15)
        .x_label_area_size(70)
        .y_label_area_size(75)
        .build_cartesian_2d(0.0f64..x_max, y_min..y_max)?;

    chart
        .configure_mesh()
        .x_desc("Trading Date")
        .y_desc("Closing Price")
        .x_labels(8)
        .y_labels(10)
        .x_label_formatter(&|x: &f64| {
            let index = x.round() as usize;

            days.get(index)
                .map(|day| day.date.clone())
                .unwrap_or_default()
        })
        .y_label_formatter(&|price: &f64| format!("{price:.2}"))
        .label_style(("sans-serif", 12))
        .axis_desc_style(("sans-serif", 16))
        .light_line_style(RGBColor(220, 220, 220))
        .draw()?;

    // Draw the daily closing prices as a red line.
    chart.draw_series(LineSeries::new(
        days.iter()
            .enumerate()
            .map(|(index, day)| (index as f64, day.close)),
        &RED,
    ))?;

    // Draw blue error bars for volatile days.
    // Each bar shows the intraday low and high.
    let error_bars = days
        .iter()
        .enumerate()
        .filter(|(_, day)| is_volatile(day))
        .flat_map(|(index, day)| {
            let x = index as f64;
            let style = BLUE.stroke_width(2);

            vec![
                // Vertical bar: low to high.
                PathElement::new(vec![(x, day.low), (x, day.high)], style.clone()),
                // Horizontal cap at the low.
                PathElement::new(
                    vec![(x - 0.25, day.low), (x + 0.25, day.low)],
                    style.clone(),
                ),
                // Horizontal cap at the high.
                PathElement::new(vec![(x - 0.25, day.high), (x + 0.25, day.high)], style),
            ]
        });

    chart.draw_series(error_bars)?;

    // Mark the closing price on every volatile day with a blue circle.
    chart.draw_series(
        days.iter()
            .enumerate()
            .filter(|(_, day)| is_volatile(day))
            .map(|(index, day)| Circle::new((index as f64, day.close), 4, BLUE.filled())),
    )?;

    // Save the finished image and report file-writing errors.
    root.present()?;

    Ok(())
}
