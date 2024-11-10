// Conditional Statements: max_of_three function
pub fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    if a > b {
        if a > c {
            a
        } else {
            c
        }
    } else {
        if b > c {
            b
        } else {
            c
        }
    }
}

// Loops: sum_even_numbers function
pub fn sum_even_numbers() -> i32 {
    let mut sum = 0;
    for i in 1..51 {
       sum += i*2; 
    }
    sum
}

// Loops: Countdown function with while loop
pub fn countdown() {
    let mut counter = 10;
    while counter > 0 {
        println!("{}",counter);
        counter -=1;
    }
    println!("Liftoff");
}

// Match Statement: day_of_week function
pub fn day_of_week(day: i32) -> &'static str {
    match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day",

    }
}
