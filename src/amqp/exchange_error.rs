/*
Exchange related error

Author Andrew Evans
*/

use std::fmt;


/// Thrown when the connection pool is empty
pub struct ExchangeError;


/// Display implementation for when the pool is empty
impl fmt::Display for ExchangeError{

    ///Display the standard error message
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to Perform Operation on Exchange")
    }
}


/// Debut for PoolIsEmptyError
impl fmt::Debug for ExchangeError{

    /// Display the debug information for the programmer
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
