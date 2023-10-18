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



use polars::prelude::*;
use polars::error::PolarsResult;
use polars::frame::DataFrame;
use polars::prelude::CsvReader;
use std::time::Instant;
use polars::export::num::Pow;
use polars::export::num::real::Real;


// kernel functions calculate rational quadratic curve , gaussian curve


/// Calculates the rational quadratic value for a given set of parameters.
/// improved version from formula in tradingview.com
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
/// use tech_analysis::rational_quadratic;
/// let src = Series::new("data",vec![1.0, 2.0, 3.0, 4.0, 5.0]);
/// let result = rational_quadratic(&src, 2, 3.0, 1);
/// eprintln!("{:?}", result);
/// println!("{:?}", result);
/// ```
pub fn rational_quadratic<'a>(src: &'a Series, look_back: i32, relative_weight: f32, start_at_bar: i32) -> Result<Series, Box<dyn std::error::Error + 'a>> {
    let size = (start_at_bar + 2) as usize;
    let num_windows = src.len() - size + 1;
    let windows: Vec<Series> = (0..num_windows)
        .map(|i| {
            src.slice(i as i64, (size) as i64 as usize)
        })
        .collect();
    let weight: Vec<_> = (0..size)
        .map(|i| {
            let i_squared = (i as f64).powi(2);
            (1.0 + i_squared / (f64::powi(look_back as f64, 2) * 2.0 * relative_weight as f64)).powf(-relative_weight as f64)
        })
        .collect();
    let weight = Series::new("data", weight);
    let current_weight: Vec<f64> = (0..num_windows).map(|i| {
        let reversed_window = windows[i].reverse();
        let weighted_sum = reversed_window.multiply(&weight).unwrap();
        weighted_sum.sum().expect("weighted_sum collection error")
    }).collect();
    let current_weight = Series::new("data", current_weight);
    let cumulative_weight: Vec<f64> = (0..num_windows).map(|_| {
        weight.sum().expect("cumulative_weight collection error")
    }).collect();
    let cumulative_weight = Series::new("data", cumulative_weight);
    let kernel_line = current_weight.divide(&cumulative_weight)?;
    let zero = vec![0.0; size - 1];
    let mut previous = Series::new("data", zero);
    let kernel_line = (*previous.extend(&kernel_line)?).clone().into();
    Ok(kernel_line)
}

/// Calculates the rational quadratic value for a given set of parameters.
/// original version in in tradingview.com
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
/// use tech_analysis::rational_quadratic_tv;
/// let src = Series::new("data",vec![1.0, 2.0, 3.0, 4.0, 5.0]);
/// let result = rational_quadratic_tv(&src, 2, 3.0, 1);
/// println!("{:?}", result);
/// ```
pub fn rational_quadratic_tv<'a>(src: &'a Series, look_back: i32, relative_weight: f32, start_at_bar: i32) -> Result<Series, Box<dyn std::error::Error + 'a>> {
    let mut val = vec![0.0; src.len()];

    for bar_index in (start_at_bar + 1)..src.len() as i32 {
        let mut current_weight = 0.0;
        let mut cumulative_weight = 0.0;

        for i in 0..(start_at_bar + 2) {
            let y: AnyValue = src.get((bar_index - i) as usize).expect("loop error");
            let w = (1.0 + (i.pow(2) as f64) / (look_back.pow(2) as f64 * 2.0 * relative_weight as f64)).powf(-relative_weight as f64);
            current_weight += y.try_extract::<f64>().expect("casting error") * w;
            cumulative_weight += w;
        }
        val[bar_index as usize] = current_weight / cumulative_weight;
    }

    Ok(Series::new("data", val))
}

/// Performs a Gaussian operation on a given time series.
/// improved version from formula in tradingview.com
///
/// This function calculates a Gaussian operation on the input time series data.
/// The Gaussian operation is performed using a specified look-back window
/// and starting point in the time series.
///
/// # Arguments
///
/// * `src` - A reference to the input Series containing the data.
/// * `look_back` - An integer representing the look-back value for the Gaussian operation.
/// * `start_at_bar` - An integer representing the starting point in the time series.
///
/// # Returns
///
/// A Result containing a new Series representing the Gaussian operation result, or an error.
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
/// use tech_analysis::gaussian;
/// let src = Series::new("data",vec![1.0, 2.0, 3.0, 4.0, 5.0]);
/// let result = gaussian(&src, 2, 3);
/// println!("{:?}", result);
/// ```
///


pub fn gaussian<'a >(src: &'a Series, look_back: i32, start_at_bar: i32)->Result<Series,Box<dyn std::error::Error + 'a>> {
    let size = start_at_bar + 2;
    let num_windows = src.len() - size + 1;
    let windows: Vec<Series> = (0..num_windows).map(|i| {
        src.slice(i as i64, size as usize)
    }).collect();

    let mut weight: Vec<f64> = Vec::new();
    for i in 0..size {
        let weight_val = (-((i as f64).powi(2)) / (2.0 * (look_back.pow(2)) as f64)).exp();
        weight.push(weight_val);
    }
    let weight = Series::new("data", weight);

    let current_weight = (0..num_windows).map(|i| {
        let reversed_window = windows[i].reverse();
        let weighted_sum = reversed_window.multiply(&weight).unwrap();
        weighted_sum.sum().expect("cumulative_weight collection error")
    }).collect();
    let current_weight = Series::new("data", current_weight);

    let cumulative_weight: Vec<f64> = (0..num_windows).map(|_| {
        weight.sum().expect("cumulative_weight collection error")
    }).collect();
    let cumulative_weight = Series::new("data", cumulative_weight);

    let gaussian_line = current_weight.divide(&cumulative_weight)?;


    let zero = vec![0.0; (size - 1) as usize];
    let mut previous = Series::new("data", zero);
    let gaussian_line = (*previous.extend(&gaussian_line)?).clone().into();
    Ok(gaussian_line)

}

/// Performs a Gaussian operation on a given time series using a different approach.
/// original version in in tradingview.com
///
/// This function calculates a Gaussian operation on the input time series data using
/// an alternative algorithm. The Gaussian operation is performed using a specified
/// look-back window and starting point in the time series.
///
/// # Arguments
///
/// * `src` - A reference to the input Series containing the data.
/// * `look_back` - An integer representing the look-back value for the Gaussian operation.
/// * `start_at_bar` - An integer representing the starting point in the time series.
///
/// # Returns
///
/// A Result containing a new Series representing the Gaussian operation result, or an error.
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
/// use tech_analysis::gaussian_tv;
/// let src = Series::new("data",vec![1.0, 2.0, 3.0, 4.0, 5.0]);
/// let result = gaussian_tv(&src, 2, 3);
/// println!("{:?}", result);
/// ```
///
pub fn gaussian_tv<'a >(src: &'a Series, look_back: i32, start_at_bar: i32)->Result<Series,Box<dyn std::error::Error + 'a>> {
    let mut val = vec![0.0; src.len()];
    use std::f64::consts::E;
    for bar_index in (start_at_bar + 1)..src.len() as i32 {

        let mut current_weight = 0.0;
        let mut cumulative_weight = 0.0;
        for i in 0..start_at_bar+2{
            let y = src.get((bar_index - i) as usize)?;
            let w =  f64::exp(-(i as f64).powi(2) /2.0 * look_back.powi(2));
            current_weight += y * w;
            cumulative_weight += w;
        }
        val[bar_index] = current_weight / cumulative_weight;
    }
    Ok(Series::new("data", val))
}


// unit test
#[cfg(test)]
mod tests {
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
        let start1 = Instant::now();
        let kernel_line = rational_quadratic(&close, 8, 1.0, 25);
        let duration = start1.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
        let rational_quadratic = binding.column("rational_quadratic").unwrap();
        // Calculate the squared differences between predicted and actual values
        let squared_errors = kernel_line.unwrap().subtract(rational_quadratic).unwrap();
        let squared_errors = squared_errors.multiply(&squared_errors).unwrap();
        let mean_squared_error = squared_errors.slice(27, squared_errors.len() - 27).sum_as_series() / (squared_errors.len() - 27);
        println!("Mean Squared Error: {}", mean_squared_error);
        let start2 = Instant::now();
        let kernel_line_tv = rational_quadratic_tv(&close, 8, 1.0, 25);
        let duration = start1.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
        // Calculate the squared differences between predicted and actual values
        let squared_errors = kernel_line_tv.unwrap().subtract(rational_quadratic).unwrap();
        let squared_errors = squared_errors.multiply(&squared_errors).unwrap();
        let mean_squared_error = squared_errors.slice(27, squared_errors.len() - 27).sum_as_series() / (squared_errors.len() - 27);
        println!("Mean Squared Error: {}", mean_squared_error);
    }

    #[test]
    fn test_rational_gaussian(){
        let df = example().unwrap();
        // println!("{:?}", df);
        let binding = df.clone();
        // println!("{:?}", binding.describe(None));
        let close = binding.column("close").unwrap();

    }

}