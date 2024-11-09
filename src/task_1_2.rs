// Function: factorial (recursive)
pub fn factorial(n: u32) -> u32 {
    if n <= 1 { 
        1
    } else {
        n * factorial(n-1)
    }
}

// Function: is_prime (using factorial function)
pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false; 
    } else {
        factorial(n-1) % n == n - 1
    }
}

// Ownership and Borrowing

// Function: reverse_string (mutably borrow and reverse in place)
pub fn reverse_string(s: &mut String) {
    unsafe {
        let vec = s.as_mut_vec();
        vec.reversed();
    }
}

// Function: concat_strings (concatenates two &str and returns a String)
pub fn concat_strings(s1: &str, s2: &str) -> String {
    s1.to_string() + s2    
}

// Slices
// Function: find_max (finds the maximum value in a slice of integers)
pub fn find_max(slice: &[i32]) -> Option<i32> {
    numbers.iter().cloned().max()
}
