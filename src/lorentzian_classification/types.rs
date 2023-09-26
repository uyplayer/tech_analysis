/*
 * @Author: uyplayer
 * @Date: 2023/8/25 13:15
 * @Email: uyplayer@qq.com
 * @File: types
 * @Software: CLion
 * @Dir: tech_analysis / src/lorentzian_classification
 * @Project_Name: tech_analysis
 * @Description:
 */

//! all type declared here using in lorentzian classification



// settings
/// Settings struct representing settings for a certain functionality for classification.
# [derive(Debug)]
pub struct Settings<'a>{
    /// The data source for the functionality.
    pub source:&'a str,
    /// The number of neighbors to consider.
    pub neighbors_count:i8,
    /// The maximum number of bars to look back.
    pub max_bars_back:i32,
    /// Flag to indicate whether exits should be shown.
    pub show_exits:bool,
    /// Flag to indicate whether dynamic exits should be used.
    pub use_dynamic_exits:bool,
    /// Flag to indicate whether an EMA filter should be used.
    pub use_ema_filter:bool,
    /// The period for the EMA filter.
    pub ema_period:i32,
    /// Flag to indicate whether an SMA filter should be used.
    pub use_sma_filter:bool,
    /// The period for the SMA filter.
    pub sma_period:i32,

}
// check setting params
/// checking  Settings params validation
impl<'a> Settings<'a> {
    fn check_settings(&self) {
        // Access each member to trigger compile-time checks
        let source  =  ["close","open","high","low","volume","vol"];
        assert!(source.contains(&self.source));
        if self.neighbors_count <= 0 {
            panic!(" neighbors_count must be bigger than zero ")
        }
        if self.max_bars_back <= 0 {
            panic!(" max_bars_back must be bigger than zero ")
        }
        if self.ema_period <= 1 {
            panic!(" ema_period must be bigger than one ")
        }
        if self.sma_period <= 1 {
            panic!(" sma_period must be bigger than one ")
        }
    }
}

// test for settings


// filter setting
/// a set of filters struct  used for classification.
pub struct Filters{
    /// Indicates whether the volatility filter is being used.
    pub use_volatility_filter: bool,
    /// Indicates whether the regime filter is being used.
    pub use_regime_filter: bool,
    /// Indicates whether the ADX (Average Directional Index) filter is being used.
    pub use_adx_filter: bool,
    /// The threshold value for the regime filter.
    pub regime_threshold: f32,
    /// The threshold value for the ADX filter.
    pub adx_threshold: i32,
}

impl Filters {
    fn check_filters(&self) {
        // Access each member to trigger compile-time checks
        if !(self.regime_threshold >= -10.0 && self.regime_threshold <= 10.0) {
            panic!("regime_threshold must be between -10.0 and 10.0");
        }
        if !(self.adx_threshold >= 0 && self.adx_threshold <= 100) {
            panic!("adx_threshold must be equal to 0");
        }
    }
}


// kernel filter
/// Represents a kernel filter used for data smoothing and estimation.
pub struct KernelFilter {
    /// Indicates whether the kernel estimate should be shown.
    pub show_kernel_estimate: bool,
    /// Indicates whether kernel smoothing should be used.
    pub use_kernel_smoothing: bool,
    /// The size of the look-back window used for smoothing and estimation.
    pub look_back_window: i32,
    /// The relative weight parameter used for smoothing.
    pub relative_weight: f32,
    /// The level of regression used in the filter.
    pub regression_level: f32,
    /// The lag for crossover operations in the filter.
    pub crossover_lag: i32,
}


// market trend direction
pub enum Direction{
    LONG = 1,
    SHORT = -1,
    NEUTRAL = 0,
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_settings() {
        let settings = Settings {
            source: "close",
            neighbors_count: 5,
            max_bars_back: 18,
            show_exits: true,
            use_dynamic_exits: false,
            use_ema_filter: false,
            ema_period: 20,
            use_sma_filter: true,
            sma_period: 20,
        };
        settings.check_settings();
    }
    #[test]
    fn test_filters() {
        let filters = Filters{
            use_volatility_filter: false,
            use_regime_filter: false,
            use_adx_filter: false,
            regime_threshold: 0.0,
            adx_threshold: 10,
        };
        filters.check_filters();
    }
    #[test]
    fn test_kernel_filter(){
        let _ = KernelFilter{
            show_kernel_estimate: false,
            use_kernel_smoothing: false,
            look_back_window: 0,
            relative_weight: 0.0,
            regression_level: 0.0,
            crossover_lag: 0,
        };
    }
    #[test]
    fn test_direction(){
        let _ = Direction::LONG;
        let _ = Direction::SHORT;
        let _ = Direction::NEUTRAL;
    }
}
