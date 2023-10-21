/*
 * @Author: uyplayer
 * @Date: 2023/8/25 13:16
 * @Email: uyplayer@qq.com
 * @File: helper
 * @Software: CLion
 * @Dir: tech_analysis / src/lorentzian_classification
 * @Project_Name: tech_analysis
 * @Description:
 */

//! Helper functions for data manipulation

use polars::export::arrow::array::{Float64Array};
use polars::prelude::*;


/// Normalizes the values of the input series to a given range.
///
/// # Arguments
/// * `src` - The input series
/// * `min_val` - The minimum value of the range to normalize to
/// * `max_val` - The maximum value of the range to normalize to
///
/// # Returns
/// The normalized series.s
pub fn normalizer<'a>(src: &'a Series, min_val: f64, max_val: f64) -> Result<Series, Box<dyn std::error::Error>> {
    let array = src.to_arrow(0);
    let vec_values = match array.as_any().downcast_ref::<Float64Array>() {
        Some(float_array) => {
            let values: &[f64] = float_array.values();
            let vec_values: Vec<f64> = values.to_vec();
            vec_values
        }
        None => return Err("Failed to downcast to Float64Array or Int64Array".into()),
    };
    let actual_min_val = vec_values
        .iter()
        .min_by(|x, y| x.partial_cmp(y).unwrap())
        .ok_or("Failed to find the minimum value in vec_values")?;

    let actual_max_val = vec_values
        .iter()
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .ok_or("Failed to find the maximum value in vec_values")?;

    let scaled_values: Vec<f64> = vec_values
        .iter()
        .map(|&x| (x - actual_min_val) / (actual_max_val - actual_min_val) * (max_val - min_val) + min_val)
        .collect();

    Ok(Series::new("date", scaled_values))


}

/// Rescales the values of the input series from one bounded range to another bounded range.
///
/// # Arguments
/// * `src` - The input series
/// * `old_min` - The minimum value of the range to rescale from
/// * `old_max` - The maximum value of the range to rescale from
/// * `new_min` - The minimum value of the range to rescale to
/// * `new_max` - The maximum value of the range to rescale to
///
/// # Returns
/// The rescaled series
pub  fn rescale<'a>(src: &'a Series, old_min: f64, old_max: f64, new_min: f64, new_max: f64) -> Result<Series, Box<dyn std::error::Error>>  {
    let array = src.to_arrow(0);
    let vec_values = match array.as_any().downcast_ref::<Float64Array>() {
        Some(float_array) => {
            let values: &[f64] = float_array.values();
            let vec_values: Vec<f64> = values.to_vec();
            vec_values
        }
        None => return Err("Failed to downcast to Float64Array or Int64Array".into()),
    };
    let epsilon = 10e-10;
    let vec_values = vec_values.iter()
        .map(|x| new_min + (new_max - new_min) * (x - old_min) / f64::max(old_max - old_min, epsilon))
        .collect::<Vec<f64>>();
    Ok(Series::new("date", vec_values))
}


/// Computes the Rolling Moving Average (RMA) of the input series and then calculates the Exponential
/// Weighted Moving Average (EWMA) using the RMA values.
///
/// # Arguments
///
/// * `src` - The input series.
/// * `length` - The length of the rolling window.
///
/// # Returns
///
/// The series containing the EWMA values.
pub fn rma_indicator<'a>(src: &'a Series, length: i32)->Result<Series, Box<dyn std::error::Error>> {
    let duration = Duration::new(length.into());
    let options = RollingOptionsImpl {
        window_size: duration,
        min_periods: 1,
        weights: None,
        center: false,
        by: None,
        tu: None,
        tz: None,
        closed_window: None,
        fn_params: None,
    };
    let rolling_mean = src.rolling_mean(options)?;

    let alpha = 2.0 / (length as f64 + 1.0);
    let mut prev_ema: Option<f64> = None;
    let mut ewma = Vec::new();
    for opt in rolling_mean.f64()? {
        let val = match opt {
            Some(v) => v,
            None => continue,
        };

        let ema = match prev_ema {
            Some(prev) => alpha * val + (1.0 - alpha) * prev,
            None => val,
        };
        ewma.push(ema);
        prev_ema = Some(ema);
    }
    let ewm_series = Series::new("date", ewma);
    Ok(ewm_series)
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use super::*;

    #[test]
    fn test_normalizer()->Result<(), Box<dyn std::error::Error>>{
        let mut rng = rand::thread_rng();
        let random_data: Vec<f64> = (0..1000).map(|_| rng.gen_range(1.0..2000.0)).collect();
        let src =  Series::new("data", random_data);
        let res = normalizer(&src, -2.0, 2.0)?;
        eprintln!("{:?}", res);
        Ok(())
    }
    #[test]
    fn test_rescale()->Result<(), Box<dyn std::error::Error>>{
        let mut rng = rand::thread_rng();
        let random_data: Vec<f64> = (0..1000).map(|_| rng.gen_range(1.0..2000.0)).collect();
        let old_min = 1.0;
        let old_max = 5.0;
        let new_min = 0.0;
        let new_max = 1.0;
        let src =  Series::new("data", random_data);
        let res  = rescale(&src, old_min, old_max, new_min, new_max)?;
        eprintln!("{:?}", res);
        Ok(())
    }

    #[test]
    fn test_rma_indicator()->Result<(), Box<dyn std::error::Error>>{

        let mut rng = rand::thread_rng();
        let random_data: Vec<f64> = (0..1000).map(|_| rng.gen_range(1.0..2000.0)).collect();
        let src =  Series::new("data", random_data);
        let res = rma_indicator(&src,10)?;
        eprintln!("{:?}", res);
        Ok(())

    }

}