# **Proof of Challenge Example: Epoch Time Conversion**

## **1. Problem Statement:**
   - **Input:** An `i64` integer representing epoch time (number of seconds since January 1, 1970, UTC).
   - **Task:** Convert this epoch time to an RFC 3339 formatted string (e.g., `"2024-08-27T14:12:34Z"`).
   - **Output:** The RFC 3339 string.

## **2. Implementation in Rust:**
   - **Step 1:** Parse the `i64` epoch time.
   - **Step 2:** Convert the epoch time to a `DateTime` object in UTC.
   - **Step 3:** Format the `DateTime` object as an RFC 3339 string.

Here's a basic implementation in Rust:

```rust
use chrono::{DateTime, Utc, TimeZone};

fn epoch_to_rfc3339(epoch: i64) -> String {
    // Convert the epoch time to a DateTime<Utc> object
    let datetime: DateTime<Utc> = Utc.timestamp_opt(epoch, 0).unwrap();
    // Convert the DateTime to an RFC3339 string
    datetime.to_rfc3339()
}

fn main() {
    let epoch_time: i64 = 1693146754; // Example epoch time
    let rfc3339_time = epoch_to_rfc3339(epoch_time);
    println!("RFC 3339 format: {}", rfc3339_time);
}
```

## **3. How It Works:**
   - **Epoch Time:** The input is an epoch time in seconds, which is an `i64` integer.
   - **Conversion:** The `chrono` crate's `Utc.timestamp_opt()` function converts the epoch time into a `DateTime<Utc>` object.
   - **RFC 3339 Format:** The `to_rfc3339()` method converts the `DateTime<Utc>` object to a string in RFC 3339 format.

## **4. Challenge Validation:**
   - To validate the solution, the network could compare the output RFC 3339 string with the expected result.
   - The challenge could involve multiple epochs, requiring participants to correctly convert each one within a certain time frame.

## **Conclusion:**
This is a straightforward but effective challenge that could be used in a proof-of-challenge system. It requires basic time manipulation and formatting skills in Rust, ensuring that participants understand how to work with date and time functions, while also being computationally light enough for quick verification.