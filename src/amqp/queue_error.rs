/*
Error for use when queue functions fail

Author Andrew Evans
*/

use std::fmt;


/// Thrown when the connection pool is empty
pub struct QueueError;


/// Display implementation for when the pool is empty
impl fmt::Display for QueueError{

    ///Display the standard error message
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Queue Operation Failed.")
    }
}


/// Debut for PoolIsEmptyError
impl fmt::Debug for QueueError{

    /// Display the debug information for the programmer
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
