use std::collections::HashMap;

// Function to square each element in a vector and return a new vector
pub fn square_elements(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|&x| x * x).collect()
}

// HashMap
//let mut city_population_map : Hashmap<&str, i32> = Hasmap::new();

// Function to print population of a given city or a not found message
pub fn print_population(city_population: &HashMap<String, i32>, city: &str) {
    match city_population.get(city) {
        Some(&population) => println!("Population of {} is {}", city, population),
        None => println!("City {} has no registered population", city),
    }
}

// Function to filter even numbers from a vector using an iterator
pub fn filter_even_numbers(v: &Vec<i32>) -> Vec<i32> {
    v.iter().filter(|&x| x % 2 == 0).collect()
}

// Function to sum odd numbers in a vector using an iterator
pub fn sum_odd_numbers(v: &Vec<i32>) -> i32 {
    v.iter().filter(|&x| x % 2 == 1).sum()
}
