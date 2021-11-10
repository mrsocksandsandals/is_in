/* is_in:
 * A small library, featuring a trait and primitive implementations of that trait.
 * Allows you to check if a value is in an array.
*/

// Implementations for standard library primitives.
mod implementations;
pub use implementations::*;

// The IsIn trait.
pub trait IsIn<T> {
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
