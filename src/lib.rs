//! Tell the compiler that you can be trusted.
//! No more errors about unsafe code - just tell the compiler to trust you via the [`trustme`] macro and it will get off your nerves!

/// Convince the compiler to let you write unsafe code.
#[macro_export]
macro_rules! trustme {
    ($($tok:tt)*) => {
        unsafe {
            $($tok)*
        }
    }
}
