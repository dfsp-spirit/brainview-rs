//! Errors one may encounter when using libbrainview.


use quick_error::quick_error;
use std::io::Error as IOError;

quick_error! {
    /// Error type for all error variants originated by this crate.
    #[derive(Debug)]
    pub enum BrainviewError {
        
        InvalidCurvFormat {
            display("Invalid Curv file")
        }


        /// I/O Error
        Io(err: IOError) {
            from()
            source(err)
        }
    }
}

/// Alias type for results originated from this crate.
pub type Result<T> = ::std::result::Result<T, BrainviewError>;

