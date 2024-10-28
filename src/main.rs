// fn main() {
//     let x = 10;
//     let y = 20; // Clippy will warn about `y` being unused
//     let sum = x + 5;

//     let value: Option<i32> = None;
        
//     // Clippy will warn about using `unwrap()` on an `Option` that can be `None`.
//     let z = value.unwrap();  // This will panic if `value` is `None`
    

//     println!("The sum is: {}", sum);
//     println!("{}", z);

// }

// // Tarpaulin
// /// A simple function that adds two numbers.
// pub fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_add() {
//         assert_eq!(add(2, 3), 5);
//         assert_eq!(add(-1, 1), 0);
//     }

//     // #[test]
//     // fn test_failure() {
//     //     assert_eq!(add(1, 1), 3); // This test will fail
//     // }
// }


// use rusqlite::{params, Connection, Result};
// use std::thread;
// use std::sync::{Arc, Mutex};

// fn main() {
//     // Example of Passing Wrong Type / Unit Value
//     let unit_value = (); // This represents the unit value (an empty tuple)
//     risky_function(unit_value); // Passing a unit value (which might make no sense in a real scenario)

//     // SQL Injection Example
//     let username = "John'; DROP TABLE users; --";  // Malicious input for SQL injection
//     match query_user_by_username(username) {
//         Ok(_) => println!("Query executed successfully."),
//         Err(err) => eprintln!("Error: {}", err),
//     }

//     // Improper Error Handling Example
//     if let Err(err) = cause_error() {
//         eprintln!("Application error: {:?}", err);  // Sensitive error information exposed
//     }

//     // Data Race Example with Threads
//     let data = Arc::new(Mutex::new(0));  // Shared data protected by a Mutex
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let data_clone = Arc::clone(&data);
//         let handle = thread::spawn(move || {
//             let mut num = data_clone.lock().unwrap();
//             *num += 1;  // Data race could happen if not properly synchronized
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Final value after data race example: {}", *data.lock().unwrap());

//     // Lack of Input Validation Example
//     let user_input = "1000a";  // Invalid input
//     match process_input(user_input) {
//         Ok(num) => println!("Processed number: {}", num),
//         Err(err) => eprintln!("Error: {}", err),  // Output invalid input error
//     }

//     // Panic on Unwrap Example
//     let value: Option<i32> = None;
//     let x = value.unwrap();  // This will panic since value is None
//     println!("Value: {}", x);

//     // Unused results from function calls
//     if let Err(err) = unused_function() {
//         eprintln!("Error occurred: {}", err);
//     }

//     // Reuse of mutable variables in closures
//     let mut closure_variable = 0;
//     let closure = || {
//         closure_variable += 1;  // Reusing mutable variable, can lead to bugs
//     };
//     closure();

//     // Calling unwrap on Option type without checking
//     let another_option: Option<i32> = Some(10);
//     let another_value = another_option.unwrap(); // May panic if `another_option` is None
//     println!("Another Value: {}", another_value);
// }

// // Function with no return value, just to showcase risky usage of unit value
// fn risky_function(_: ()) {
//     eprintln!("Function accepted a unit value."); // Demonstrating risk without significance
// }

// // Vulnerable to SQL injection
// fn query_user_by_username(username: &str) -> Result<()> {
//     let conn = Connection::open("vulnerable.db")?;
    
//     // Vulnerable to SQL injection
//     let query = format!("SELECT * FROM users WHERE username = '{}'", username);
    
//     conn.execute(&query, params![])?;
//     Ok(())
// }

// // Dummy error for improper handling
// fn cause_error() -> Result<(), String> {
//     Err("An unexpected error occurred".into())
// }

// // Lack of input validation
// fn process_input(input: &str) -> Result<i32, String> {
//     input.parse::<i32>().map_err(|_| "Invalid input".to_string())  // Lack of input validation
// }

// // Function that returns an error but is not used
// fn unused_function() -> Result<(), String> {
//     Err("This function's result is unused".into())
// }

use rusqlite::{params, Connection, Result};
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    // Example of Passing Wrong Type / Unit Value
    let unit_value = (); // This represents the unit value (an empty tuple)
    risky_function(unit_value); // Passing a unit value (which might make no sense in a real scenario)

    // SQL Injection Example
    let username = "John'; DROP TABLE users; --";  // Malicious input for SQL injection
    match query_user_by_username(username) {
        Ok(_) => println!("Query executed successfully."),
        Err(err) => eprintln!("Error: {}", err),
    }

    // Improper Error Handling Example
    if let Err(err) = cause_error() {
        eprintln!("Application error: {:?}", err);  // Sensitive error information exposed
    }

    // Data Race Example with Threads
    let data = Arc::new(Mutex::new(0));  // Shared data protected by a Mutex
    let mut handles = vec![];

    for _ in 0..10 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut num = data_clone.lock().unwrap();
            *num += 1;  // Data race could happen if not properly synchronized
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final value after data race example: {}", *data.lock().unwrap());

    // Lack of Input Validation Example
    let user_input = "1000a";  // Invalid input
    match process_input(user_input) {
        Ok(num) => println!("Processed number: {}", num),
        Err(err) => eprintln!("Error: {}", err),  // Output invalid input error
    }

    // Panic on Unwrap Example
    let value: Option<i32> = None;
    let x = value.unwrap();  // This will panic since value is None
    println!("Value: {}", x);

    // Vulnerable call using option_env!
    let _env_value = option_env!("MY_ENV_VAR").unwrap(); // This can panic if MY_ENV_VAR is not set

    // Potentially unsafe usage of Arc::get_mut
    let mut shared_data = Arc::new(Mutex::new(0));
    let data_mutable_ref = Arc::get_mut(&mut shared_data); // Potentially unsafe in older Rust versions
    if let Some(mut data) = data_mutable_ref {
        *data.lock().unwrap() += 1;
    }

    // Detected conversion between differently sized raw slices
    let a: &[u8] = &[1, 2, 3]; // Size 3
    let b: &[u16] = unsafe { std::mem::transmute::<&[u8], &[u16]>(a) }; // Unsafe conversion

    // Called mem::forget or mem::drop on a reference
    let reference = &mut 42;
    std::mem::forget(reference); // This does nothing meaningful, the reference is forgotten

    // Found occurrence of .step_by(0)
    let numbers = (0..10).step_by(0); // This will panic at runtime
}

fn risky_function(_: ()) {
    // Accepting unit value, which is generally not useful
    eprintln!("Function accepted a unit value."); // Demonstrating risk without significance
}

fn query_user_by_username(username: &str) -> Result<()> {
    let conn = Connection::open("vulnerable.db")?;
    
    // Vulnerable to SQL injection
    let query = format!("SELECT * FROM users WHERE username = '{}'", username);
    
    conn.execute(&query, params![])?;
    Ok(())
}

fn cause_error() -> Result<(), String> {
    Err("An unexpected error occurred".into())  // Dummy error for improper handling
}

fn process_input(input: &str) -> Result<i32, String> {
    input.parse::<i32>().map_err(|_| "Invalid input".to_string())  // Lack of input validation
}
