// //! # minigrep
// //! 
// //! `minigrep` is a collection of utilities to make performing certain
// //! calculations more convenient

// /// Adds one to the number given.
// /// 
// /// # Example
// /// 
// /// ```
// /// let arg = 5;
// /// let answer = minigrep::add_one(arg);
// /// 
// /// assert_eq!(6, answer)
// /// ```
// pub fn add_one(x: i32) -> i32 {
//   x+1
// }

//! # Art
//! 
//! A library modeling artistic concept

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
  /// The primary colors according to the RYB color model
  pub enum PrimaryColor {
    Red,
    Yellow,
    Blue
  }

  // Then secondary colors according to the RYB color model
  pub enum SecondaryColor {
    Orange,
    Green,
    Purple,
  }
}

pub mod utils {
  use crate::kinds::*;

  /// Combines two primary colrs in equal amounts to create
  /// a secondary color
  pub fn mix(c1: PrimaryColor, c2: SecondaryColor) -> SecondaryColor {
    c2
  }
}