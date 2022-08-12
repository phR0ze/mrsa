//! `mrsa-core` provides a clean API for manipulating media
//!
//! ### Example
//! ```
//! use mrsa_core::prelude::*;
//! ```
pub mod image;

/// All essential symbols in a simple consumable form
///
/// ### Examples
/// ```
/// use mrsa_core::prelude::*;
/// ```
pub mod prelude {
    pub use crate::image::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
