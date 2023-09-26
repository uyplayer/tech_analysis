/*
 * @Author: uyplayer
 * @Date: 2023/8/25 13:14
 * @Email: uyplayer@qq.com
 * @File: kernel
 * @Software: CLion
 * @Dir: tech_analysis / src/lorentzian_classification
 * @Project_Name: tech_analysis
 * @Description:
 */

// unsafe code not allowed
#![forbid(unsafe_code)]

use polars::prelude::*;
use polars::error::PolarsResult;
use polars::frame::DataFrame;
use polars::prelude::CsvReader;

// kernel functions calculate rational quadratic curve , gaussian curve


/// Calculates the rational quadratic value for a given set of parameters.
///
/// This function takes a series of values, a lookback period, a relative weight,
/// and a starting point. It then calculates the rational quadratic value for each
/// element in the series based on the provided parameters.
///
/// # Arguments
///
/// * `src` - The input series of values.
/// * `look back` - The look back period used in the calculation.
/// * `relative_weight` - The relative weight used in the calculation.
/// * `start_at_bar` - The starting point for the calculation.
///
/// # Returns
///
/// A new series containing the calculated rational quadratic values.
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
///
/// let src = Series::new("data",vec![1.0, 2.0, 3.0, 4.0, 5.0]);
/// let result = rational_quadratic(src, 2, 3, 1);
/// println!("{:?}", result);
/// ```
pub fn rational_quadratic(src:&Series, look_back: i32, relative_weight: f32, start_at_bar: i32) -> Series {
    let size = (start_at_bar + 2) as usize;
    let num_windows = src.len() - size + 1;
    let windows:Vec<Series> = (0..num_windows)
        .map(|i| {
            src.slice(i as i64, (size) as i64 as usize)
        })
        .collect();
    let weight :Vec<_>= (0..size)
        .map(|i| {
            let i_squared = (i as f64).powi(2);
            (1.0 + i_squared / (f64::powi(look_back as f64, 2) * 2.0 * relative_weight as f64)).powf(-relative_weight as f64)
        })
        .collect();
    let weight = Series::new("data",weight);
    let current_weight:Vec<f64> =(0..num_windows).map(|i| {
        let reversed_window = windows[i].reverse();
        let weighted_sum = reversed_window.multiply(&weight).unwrap();
        weighted_sum.sum().unwrap()
    }).collect();
    let current_weight = Series::new("data",current_weight);
    let cumulative_weight:Vec<f64> =(0..num_windows).map(|_|{
        weight.sum().unwrap()
    }).collect();
    let cumulative_weight = Series::new("data",cumulative_weight);
    let kernel_line = current_weight.divide(&cumulative_weight).unwrap();
    let zero = vec![0.0;size-1];
    let mut previous = Series::new("data", zero);
    let kernel_line = (*previous.extend(&kernel_line).unwrap()).clone().into();
    kernel_line
}

/// Calculates the rational quadratic value for a given set of parameters.
///
/// This function takes a series of values, a look back period, a relative weight,
/// and a starting point. It then calculates the rational quadratic value for each
/// element in the series based on the provided parameters.
///
/// # Arguments
///
/// * `src` - The input series of values.
/// * `look back` - The look back period used in the calculation.
/// * `relative_weight` - The relative weight used in the calculation.
/// * `start_at_bar` - The starting point for the calculation.
///
/// # Returns
///
/// A new series containing the calculated rational quadratic values.
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
///
/// let src = Series::new("data",vec![1.0, 2.0, 3.0, 4.0, 5.0]);
/// let result = rational_quadratic_tv(src, 2, 3, 1);
/// println!("{:?}", result);
/// ```
pub fn rational_quadratic_tv(src:&Series, look_back: i32, relative_weight: f32, start_at_bar: i32)->Series{
    let mut val  = vec![0.0; src.len()];
    for bar_index in (start_at_bar + 1)..src.len() as i32 {
        let mut current_weight = 0.0;
        let mut cumulative_weight = 0.0;

        for i in 0..(start_at_bar + 2) {
            let y: AnyValue = src.get((bar_index - i) as usize).unwrap();
            let w = (1.0 + (i.pow(2) as f64) / (look_back.pow(2) as f64 * 2.0 * relative_weight as f64)).powf(-relative_weight as f64);
            current_weight += y.try_extract::<f64>().unwrap() * w;
            cumulative_weight += w;
        }
        val[bar_index as usize] = current_weight / cumulative_weight
    }
    Series::new("data",val)

}

/// Performs a Gaussian operation.
#[allow(dead_code)]
fn gaussian() {

}

// unit test
#[cfg(test)]
mod test{
    use super::*;
    fn example() -> PolarsResult<DataFrame> {
        use std::env;
        use std::path::PathBuf;
        let mut path = PathBuf::new();
        path.push(env::current_dir().unwrap());
        path.push("src/lorentzian_classification/data/BINANCE_BTCUSDT, 15 (1)rational_guesss.csv");
        let path = PathBuf::from(path);
        let data = CsvReader::from_path(&path)?.infer_schema(None).has_header(true).finish();
        data


    }
    #[test]
    fn test_rational_quadratic() {
        let df = example().unwrap();
        // println!("{:?}", df);
        let binding = df.clone();
        // println!("{:?}", binding.describe(None));
        let close = binding.column("close").unwrap();
        let kernel_line = rational_quadratic(&close,8,1.0,25);
        let rational_quadratic = binding.column("rational_quadratic").unwrap();
        // Calculate the squared differences between predicted and actual values
        let squared_errors = kernel_line.subtract(rational_quadratic).unwrap();
        let squared_errors = squared_errors.multiply(&squared_errors).unwrap();
        let mean_squared_error = squared_errors.slice(27,squared_errors.len()-27).sum_as_series()/(squared_errors.len()-27);
        println!("Mean Squared Error: {}", mean_squared_error);
        let kernel_line_tv = rational_quadratic_tv(&close,8,1.0,25);
        // Calculate the squared differences between predicted and actual values
        let squared_errors = kernel_line_tv.subtract(rational_quadratic).unwrap();
        let squared_errors = squared_errors.multiply(&squared_errors).unwrap();
        let mean_squared_error = squared_errors.slice(27,squared_errors.len()-27).sum_as_series()/(squared_errors.len()-27);
        println!("Mean Squared Error: {}", mean_squared_error);
    }
}