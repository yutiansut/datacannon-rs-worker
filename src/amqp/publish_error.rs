/*
Error for use when publishing fails

Author Andrew Evans
*/

use std::fmt;


/// Thrown when the connection pool is empty
pub struct PublishError;


/// Display implementation for when the pool is empty
impl fmt::Display for PublishError{

    ///Display the standard error message
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to Publish on Channel")
    }
}


/// Debut for PoolIsEmptyError
impl fmt::Debug for PublishError{

    /// Display the debug information for the programmer
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
