/*
Errors for the connection pool

Author Andrew Evans
*/

use std::fmt;

/// Thrown when the connection pool is empty
pub struct PoolIsEmptyError;


/// Display implementation for when the pool is empty
impl fmt::Display for PoolIsEmptyError{

    ///Display the standard error message
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Connection Pool was Empty")
    }
}


/// Debut for PoolIsEmptyError
impl fmt::Debug for PoolIsEmptyError{

    /// Display the debug information for the programmer
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
