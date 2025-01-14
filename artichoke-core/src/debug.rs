//! Routines for debugging and printing exception messages.

use crate::value::Value;

/// Debugging and `Exception` message support.
pub trait Debug {
    /// Concrete type for return values from eval.
    type Value: Value;

    /// Return a name for thie given value's type suitable for using in an
    /// `Exception` message.
    ///
    /// Some immediate types like `true`, `false`, and `nil` are shown by value
    /// rather than by class.
    ///
    /// This function suppresses all errors and returns an empty string on error.
    fn inspect_type_name_for_value(&mut self, value: Self::Value) -> &str;
}
