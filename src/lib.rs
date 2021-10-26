//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, my_crate::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}


////! # Art
////!
////! A library for modeling artistic concepts.
////! #芸術
////!
////! 芸術的な概念をモデル化するライブラリ。

//pub mod kinds {
//    /// The primary colors according to the RYB color model.
//    /// RYBカラーモデルによる主色
//    pub enum PrimaryColor {
//        Red,
//        Yellow,
//        Blue,
//    }

//    /// The secondary colors according to the RYB color model.
//    /// RYBカラーモデルによる副色
//    pub enum SecondaryColor {
//        Orange,
//        Green,
//        Purple,
//    }
//}

//pub mod utils {
//    use kinds::*;

//    /// Combines two primary colors in equal amounts to create
//    /// a secondary color.
//    ///2つの主色を同じ割合で混合し、副色にする
//    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
//        // --snip--
//    }
//}

