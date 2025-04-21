// Prevents additional console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// Checks if a given unsigned 64-bit integer `n` is a prime number.
///
/// This function implements an optimized primality test. It first handles base cases
/// for numbers less than or equal to 3. Then, it quickly eliminates multiples of 2 and 3.
///
/// For numbers greater than 3, it uses the property that all prime numbers greater than 3
/// can be expressed in the form `6k ± 1` for some integer `k`. Therefore, it only needs
/// to check for divisibility by numbers of the form `6k - 1` and `6k + 1`.
/// The loop starts checking from 5 and increments by 6 in each step, checking `i` (form `6k - 1`)
/// and `i + 2` (form `6k + 1`) as potential factors.
///
/// The check only goes up to the square root of `n`, as any larger factor would necessarily
/// have a corresponding smaller factor that would have already been found.
///
/// # Arguments
///
/// * `n` - The `u64` number to check for primality.
///
/// # Returns
///
/// * `true` if `n` is a prime number, `false` otherwise.
fn is_prime(n: u64) -> bool {
    // Base case: 0 and 1 are not prime numbers.
    if n <= 1 {
        return false;
    }

    // Base case: 2 and 3 are prime numbers.
    if n <= 3 { // 2 and 3 are prime
        return true;
    }
    
    // Optimization: If n is divisible by 2 or 3, it's not prime (excluding 2 and 3 handled above).
    // This step is crucial for the 6k ± 1 optimization to work correctly.
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    // Calculate the limit for checking divisors: the square root of n.
    // We only need to check for factors up to sqrt(n).
    let limit = (n as f64).sqrt() as u64;

    // Start checking potential factors from 5.
    // All primes > 3 are of the form 6k ± 1. The loop increments by 6,
    // checking numbers of the form 6k-1 and 6k+1.
    let mut i = 5;
    while i <= limit {
        // Check if n is divisible by i (form 6k - 1) or i + 2 (form 6k + 1).
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        // Increment by 6 to check the next pair of potential factors (6(k+1)-1, 6(k+1)+1).
        i += 6;
    }

    // If no divisors were found up to the limit, n is prime.
    true
}

/// This function is called from the JS side of the Tauri application.
/// It takes a string input, attempts to parse it as a u64, and checks if it's prime.
/// It returns a string indicating whether the number is prime or not.
#[tauri::command]
fn check_prime(number_to_check: String) -> String {
    // Try to parse the input string as a u64.
    // If parsing fails, it means the input was not a valid positive integer.
    match number_to_check.parse::<u64>() {
        Ok(num) => {
            // If parsing is successful, check if the number is prime.
            if is_prime(num) {
                format!("{} is prime ✅", num)
            } else {
                format!("{} is not prime ❌​", num)
            }
        }
        Err(_) => {
            // If parsing fails, return an error message.
            format!("'{}' is not a valid positive integer.", number_to_check)
        }
    }
}

fn main() {
    // Initialize the Tauri application.
    // The `tauri::Builder::default()` creates a new Tauri application instance.
    // The `invoke_handler` method registers the command handlers that can be called from the frontend.
    tauri::Builder::default()
        // Register the command handler for checking prime numbers.
        // The `check_prime` function is registered as a command that can be invoked from the frontend.
        .invoke_handler(tauri::generate_handler![check_prime])
        .run(tauri::generate_context!()) // Run the Tauri application with the generated context.
        .expect("Error while running Tauri application"); // Handle any errors that occur during the run.
}