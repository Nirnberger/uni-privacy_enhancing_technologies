mod task_1_1;
mod task_1_2;
mod task_1_3;
mod task_1_4;
mod task_1_5;

use std::collections::HashMap;

use task_1_5::read_and_sum_integers;

fn main() {

    test_task_2();

    test_task_4();

    test_task_5();
}

fn test_task_2(){
    println!("Test Task_2:");

    println!("Factorial of 12: {}", task_1_2::factorial(12));

    println!("Is 12: {}", task_1_2::is_prime(12));
    println!("Is 7 prime: {}", task_1_2::is_prime(7));
    
    let mut my_string = String::from("Hello World!"); // Convert literal to a mutable String
    task_1_2::reverse_string(&mut my_string);
    println!("Reverse of \"Hello World!\": {}", my_string);

    println!("Concatenated string: {}", task_1_2::concat_strings("Hello", " World!"));

    let numbers = [3,213,99,56,32,1,1,0];
    let max_value = task_1_2::find_max(&numbers);
    match max_value {
        Some(max) => println!("The maximum value is: {}", max),
        None => println!("The slice is empty, no maximum value."),
    }
}

fn test_task_4(){
    println!("Test Calls for Task_4:");

    let vector: Vec<i32> = (1..=10).collect();
    println!("vector= {:?}", vector);
    println!("vector squared= {:?}", task_1_4::square_elements(&vector));

    let mut city_map = HashMap::new();
    city_map.insert(String::from("Wien"), 200_000);
    city_map.insert(String::from("Graz"), 303_000);
    city_map.insert(String::from("Klagenfurt"), 104_000);
    task_1_4::print_population(&city_map, "Klagenfurt");  // Should print population
    task_1_4::print_population(&city_map, "Innsbruck"); // Should print "not found" message
   
    println!("Even numbers: {:?}", task_1_4::filter_even_numbers(&vector));
    println!("Sum of odd numbers: {:?}", task_1_4::sum_odd_numbers(&vector));
}

fn test_task_5(){

    println!("Test Task_5:");

    let file_path = "numbers.txt";

    match read_and_sum_integers(file_path) {
        Ok(sum) => println!("The sum of integers in the file is: {}", sum),
        Err(e) => println!("Error: {}", e),
    }
}
