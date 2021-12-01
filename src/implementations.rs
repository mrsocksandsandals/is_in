/* ----------------------------------------------------------------- */
/* implementations.rs - Implementations of IsIn for primitive types. */
/* ----------------------------------------------------------------- */

// Import the trait itself.
use super::IsIn;

// Implementations.
impl IsIn<u8> for u8 {
    fn is_in(&self, arr: &[u8]) -> bool {
        for i in arr {
            if self == i {
                return true;
            }
        }
        false
    }
}
impl IsIn<u16> for u16 {
    fn is_in(&self, arr: &[u16]) -> bool {
        for i in arr {
            if self == i {
                return true;
            }
        }
        false
    }
}
impl IsIn<u32> for u32 {
    fn is_in(&self, arr: &[u32]) -> bool {
        for i in arr {
            if self == i {
                return true;
            }
        }
        false
    }
}
impl IsIn<u64> for u64 {
    fn is_in(&self, arr: &[u64]) -> bool {
        for i in arr {
            if self == i {
                return true;
            }
        }
        false
    }
}
impl IsIn<usize> for usize {
    fn is_in(&self, arr: &[usize]) -> bool {
        for i in arr {
            if self == i {
                return true;
            }
        }
        false
    }
}
impl IsIn<i8> for i8 {
    fn is_in(&self, arr: &[i8]) -> bool {
        for i in arr {
            if self == i {
                return true;
            }
        }
        false
    }
}
impl IsIn<i16> for i16 {
    fn is_in(&self, arr: &[i16]) -> bool {
        for i in arr {
            if self == i {
                return true;
            }
        }
        false
    }
}
impl IsIn<i32> for i32 {
    fn is_in(&self, arr: &[i32]) -> bool {
        for i in arr {
            if self == i {
                return true;
            }
        }
        false
    }
}
impl IsIn<i64> for i64 {
    fn is_in(&self, arr: &[i64]) -> bool {
        for i in arr {
            if self == i {
                return true;
            }
        }
        false
    }
}
impl IsIn<isize> for isize {
    fn is_in(&self, arr: &[isize]) -> bool {
        for i in arr {
            if self == i {
                return true;
            }
        }
        false
    }
}
impl IsIn<u128> for u128 {
    fn is_in(&self, arr: &[u128]) -> bool {
        for i in arr {
            if self == i {
                return true;
            }
        }
        false
    }
}
impl IsIn<i128> for i128 {
    fn is_in(&self, arr: &[i128]) -> bool {
        for i in arr {
            if self == i {
                return true;
            }
        }
        false
    }
}
impl IsIn<f32> for f32 {
    fn is_in(&self, arr: &[f32]) -> bool {
        for i in arr {
            if self == i {
                return true;
            }
        }
        false
    }
}
impl IsIn<f64> for f64 {
    fn is_in(&self, arr: &[f64]) -> bool {
        for i in arr {
            if self == i {
                return true;
            }
        }
        false
    }
}
impl IsIn<char> for char {
    fn is_in(&self, arr: &[char]) -> bool {
        for i in arr {
            if self == i {
                return true;
            }
        }
        false
    }
}
impl IsIn<&str> for &str {
    fn is_in(&self, arr: &[&str]) -> bool {
        for i in arr {
            if self == i {
                return true;
            }
        }
        false
    }
}
