/*
 * @Author: uyplayer
 * @Date: 2023/8/25 13:04
 * @Email: uyplayer@qq.com
 * @File: lib
 * @Software: CLion
 * @Dir: Tech-Analysis / src
 * @Project_Name: Tech-Analysis
 * @Description:
 */



// unsafe code not allowed
#![forbid(unsafe_code)]


//! this is a rust library implement various technical analysis for struck and cryptocurrency market


mod lorentzian_classification;
pub use lorentzian_classification::{rational_quadratic,rational_quadratic_tv};

