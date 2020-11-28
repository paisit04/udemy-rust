//! This module contains English phrases.
//!
//! # Examples
//! ```
//! let username = "John";
//! println!("{}, {}",
//!   phrases::greetings::english::hello(),
//!   username
//! );
//! ```

// comment

/*
 multiple-line
 commend
 */

/// Applies to code that follows it.
/// In this case, it's our `hello()` function. 
pub fn hello() -> String { "hello".to_string() /* here */ }

pub fn goodbye() -> String { "goodbye".to_string() }