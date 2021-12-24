/* is_in:
 * A small library, featuring a trait and primitive implementations of that trait.
 * Allows you to check if a value is in an array.
*/

//! # `is_in`
//! > A small crate featuring a nice way to check if a value is in an array.
//! ### Basic usage.
//! ```rust
//! use is_in::IsIn;
//!
//! fn main() {
//!     let data: &[u8; 3] = &[1, 2, 3];
//!     let two: u8 = 2;
//!     println!("{}", two.is_in(data)); // prints true
//! }
//! ```
//! The crate provides implementations on most primitives (all unsigned integers, all signed integers, floats, and chars).
#![deny(missing_docs)]

/// Implementations for standard library primitives.
mod implementations;
pub use implementations::*;

/// The IsIn trait.
/// ### Implementing `IsIn`:
/// ```rust
/// #[derive(PartialEq, Eq)]
/// struct MyStruct(u8);
/// use is_in::IsIn;
/// impl IsIn<MyStruct> for MyStruct {
///     fn is_in(&self, arr: &[MyStruct]) -> bool {
///         for i in arr {
///             if self == i {
///                 return true;
///             }
///         }
///         false
///     }
/// }
/// ```
pub trait IsIn<T> {
    /// The method used to check if a value is in an array.
    fn is_in(&self, arr: &[T]) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_int() {
        assert_eq!(8.is_in(&[2, 4, 7, 8]), true);
    }

    #[test]
    fn test_char() {
        assert_eq!('c'.is_in(&['a', 'c', 'd', 'b']), true);
    }
}
