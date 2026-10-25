use crate::stock::StockDay;

pub fn is_volatile(day: &StockDay) -> bool {
    (day.high - day.low) / day.close > 0.02
}

pub fn closing_extrema(days: &[StockDay]) -> Option<(&StockDay, &StockDay)> {
    let minimum = days.iter().min_by(|a, b| a.close.total_cmp(&b.close))?;
    let maximum = days.iter().max_by(|a, b| a.close.total_cmp(&b.close))?;
    Some((minimum, maximum))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_volatile() {
        let day1 = StockDay {
            timestamp: 0,
            date: "2026-10-20".to_string(),
            open: 246.73,
            high: 249.86,
            low: 246.11,
            close: 248.37,
        };

        let day2 = StockDay {
            timestamp: 0,
            date: "2026-10-21".to_string(),
            open: 250.18,
            high: 252.37,
            low: 247.37,
            close: 250.0,
        };

        let day3 = StockDay {
            timestamp: 0,
            date: "2026-10-19".to_string(),
            open: 252.41,
            high: 258.96,
            low: 251.32,
            close: 255.84,
        };

        assert!(!is_volatile(&day1));
        assert!(!is_volatile(&day2));
        assert!(is_volatile(&day3));
    }

    #[test]
    fn test_closing_extrema() {
        let day1 = StockDay {
            timestamp: 0,
            date: "2026-10-21".to_string(),
            open: 251.92,
            high: 255.63,
            low: 250.84,
            close: 253.81,
        };

        let day2 = StockDay {
            timestamp: 0,
            date: "2026-10-19".to_string(),
            open: 248.74,
            high: 251.38,
            low: 247.93,
            close: 249.26,
        };

        let day3 = StockDay {
            timestamp: 0,
            date: "2026-10-20".to_string(),
            open: 248.19,
            high: 249.17,
            low: 245.31,
            close: 246.47,
        };

        let days = vec![day1, day2, day3];
        let (minimum, maximum) = closing_extrema(&days).unwrap();

        assert_eq!(minimum.close, 246.47);
        assert_eq!(minimum.date, "2026-10-20");
        assert_eq!(maximum.close, 253.81);
        assert_eq!(maximum.date, "2026-10-21");
    }

    #[test]
    fn test_empty_data() {
        assert!(closing_extrema(&[]).is_none());
    }
}
