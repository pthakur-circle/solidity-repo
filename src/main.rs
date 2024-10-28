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
