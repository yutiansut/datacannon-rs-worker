/*
Send error for the broker

Author Andrew Evans
*/

use std::fmt;


pub struct SendTaskError;


impl fmt::Display for SendTaskError{

    /// Display the standard error message
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to Send Task to Broker!")
    }
}


impl fmt::Debug for SendTaskError{

    /// Show the debug information
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
