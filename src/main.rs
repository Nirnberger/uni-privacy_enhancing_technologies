mod task_1_1;
mod task_1_2;
mod task_1_3;
mod task_1_4;
mod task_1_5;

use std::collections::HashMap;

use task_1_5::read_and_sum_integers;
use crate::task_1_1::{countdown, day_of_week, max_of_three, sum_even_numbers};
use crate::task_1_3::{safe_divide, Student, TrafficLight};
use crate::task_1_5::{parse_integer, read_file_lines, write_file};

fn main() {
    test_task_1();

    test_task_2();

    test_task_4();

    test_task_5();
}


fn test_task_1() {
    println!("\tmax_of_three");
    if max_of_three(1, 3, 2) == 3 {
        print_success();
    } else {
        print_error("3", &max_of_three(1, 3, 2).to_string(), "max_of_three(1, 3, 2)");
    }

    if max_of_three(3, 2, 1) == 3 {
        print_success();
    } else {
        print_error("3", &max_of_three(3, 2, 1).to_string(), "max_of_three(3, 2, 1)");
    }

    if max_of_three(3, 3, 1) == 3 {
        print_success();
    } else {
        print_error("3", &max_of_three(3, 3, 1).to_string(), "max_of_three(3, 3, 1)");
    }

    // Test for countdown (assuming it prints output)
    println!("\tcountdown");
    countdown();

    // Test for sum_even_numbers
    println!("\tsum_even_numbers");
    let sum_result = sum_even_numbers();
    if sum_result == 2550 {
        print_success();
    } else {
        print_error("2550", &sum_result.to_string(), "sum_even_numbers()");
    }

    // Tests for day_of_week
    println!("\tday_of_week");

    let tests = [
        (1, "Monday"),
        (2, "Tuesday"),
        (3, "Wednesday"),
        (4, "Thursday"),
        (5, "Friday"),
        (6, "Saturday"),
        (7, "Sunday"),
        (8, "Invalid day"),
        (0, "Invalid day"),
        (-1, "Invalid day"),
    ];

    for (input, expected) in tests.iter() {
        let actual = day_of_week(*input);
        if actual == *expected {
            print_success();
        } else {
            print_error(expected, &actual, &format!("day_of_week({})", input));
        }
    }
}

fn test_task_2() {
    println!("Test Task_2:");

    println!("Factorial of 12: {}", task_1_2::factorial(12));

    println!("Is 12: {}", task_1_2::is_prime(12));
    println!("Is 7 prime: {}", task_1_2::is_prime(7));

    let mut my_string = String::from("Hello World!"); // Convert literal to a mutable String
    task_1_2::reverse_string(&mut my_string);
    println!("Reverse of \"Hello World!\": {}", my_string);

    println!("Concatenated string: {}", task_1_2::concat_strings("Hello", " World!"));

    let numbers = [3, 213, 99, 56, 32, 1, 1, 0];
    let max_value = task_1_2::find_max(&numbers);
    match max_value {
        Some(max) => println!("The maximum value is: {}", max),
        None => println!("The slice is empty, no maximum value."),
    }
}

fn test_task_3(){
    println!("\tStudent Tests");

    let student = Student::new_student("Alice".to_string(), 20, 3.8);
    if student.name == "Alice" && student.age == 20 && (student.gpa - 3.8).abs() < f32::EPSILON {
        print_success();
    } else {
        print_error("name: Alice, age: 20, gpa: 3.8",
                    &format!("name: {}, age: {}, gpa: {}", student.name, student.age, student.gpa),
                    "Student::new_student Test 1");
    }


    println!("Display Student Info:");
    student.display();

    // Tests für TrafficLight::light_duration
    println!("\tTrafficLight Tests");

    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;

    if red_light.light_duration() == 60 {
        print_success();
    } else {
        print_error("60", &red_light.light_duration().to_string(), "TrafficLight::Red light_duration");
    }

    if yellow_light.light_duration() == 5 {
        print_success();
    } else {
        print_error("5", &yellow_light.light_duration().to_string(), "TrafficLight::Yellow light_duration");
    }

    if green_light.light_duration() == 30 {
        print_success();
    } else {
        print_error("30", &green_light.light_duration().to_string(), "TrafficLight::Green light_duration");
    }

    // Tests für safe_divide
    println!("\tsafe_divide Tests");

    let divide_tests = [
        (10, 2, Some(5)),
        (9, 3, Some(3)),
        (1, 0, None),  // Division durch 0 sollte None zurückgeben
        (-10, 2, Some(-5)),
        (7, 3, Some(2)) // Integer-Division, erwartet 2 (7 / 3)
    ];

    for (a, b, expected) in divide_tests.iter() {
        let actual = safe_divide(*a, *b);
        if actual == *expected {
            print_success();
        } else {
            print_error(&format!("{:?}", expected), &format!("{:?}", actual), &format!("safe_divide({}, {})", a, b));
        }
    }

}

fn test_task_4() {
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

fn test_task_5() {
    println!("Test Task_5:");

    let file_path = "numbers.txt";

    match read_and_sum_integers(file_path) {
        Ok(sum) => println!("The sum of integers in the file is: {}", sum),
        Err(e) => println!("Error: {}", e),
    }


    println!("\tparseInt");
    let convertables = [
        ("6", Ok(6)),
        ("12", Ok(12)),
        ("12345", Ok(12345)),
        ("someRand", Err(-1)),
        ("12F4", Err(-1))
    ];

    for (s, e) in convertables{
        let actual = parse_integer(s);
        if(actual.is_ok() && e.is_ok()){
            let act = actual.unwrap();
            let exp = e.unwrap();
            if(act == exp){
                print_success()
            }
            else {
                print_error(stringify!(exp), stringify!(act), "parseInt")
            }

        }
        else if actual.is_err() && e.is_err() {
            print_success()
        }
        else {
            eprintln!("error")
        }
    }

println!("\tread_file_lines");
let result_a = read_file_lines("test_files/test_a.txt");
let result_b = read_file_lines("test_files/test_b.txt");
let result_c = read_file_lines("test_files/test_ab.txt");

if(result_a.is_ok()){
let ra = result_a.unwrap();
if ra.len() == 3 && ra.contains(&"1".to_string()) && ra.contains(&"2".to_string()) && ra.contains(&"3".to_string()){
print_success()
}
} else {
print_error("values read from file", "error", "read_file_lines")
}
if(result_b.is_ok()){
print_success()
} else {
print_error("empty file", "error", "read_file_lines")
}

if(result_c.is_err()){
print_success()
} else {
print_error("error cause files does not exist", "success", "read_file_lines")
}

println!("\twrite_file");


let rand_num = "some_txt";
let written = write_file("test_files/test_w.txt", stringify!(rand_num));
if written.is_err(){
print_error("successful write to file w", "error when writing", "write_file")
}
//manually check if written

println!("\tread_and_sum_integers");
let sumed = read_and_sum_integers("test_files/test_a.txt");
if(sumed.is_ok() && sumed.unwrap() == 6){
print_success()
} else {
print_error("successful read and file and sum 6", "error or not 6", "read_and_sum_integers")
}



}

fn print_success() {
    println!("success");
}

fn print_error(expected: &str, actual: &str, test_name: &str) {
    eprintln!("error in {}:\texpected '{}', but got '{}'", test_name, expected, actual);
}

