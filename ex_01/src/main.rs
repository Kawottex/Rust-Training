use std::io;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let test_values = vec![1, 2, 3, 1, 2, 3, 1, 1];
    let median = get_median(&test_values);
    let mode = get_mode(&test_values);

    println!("Values: {test_values:?}\n Median: {median}, Mode: {mode}");

    let str_test_1 = String::from("test");
    let str_test_2 = String::from("apple");
    let str_test_3 = String::from("baaaa");
    let pig_latin_test_1 = convert_to_pig_latin(&str_test_1);
    let pig_latin_test_2 = convert_to_pig_latin(&str_test_2);
    let pig_latin_test_3 = convert_to_pig_latin(&str_test_3);

    println!("pig_latin values: {str_test_1}: {pig_latin_test_1}, {str_test_2}: {pig_latin_test_2}, {str_test_3}: {pig_latin_test_3}");

    company_department_game();
}

fn get_median(values: &Vec<i32>) -> i32 {
    let mid_index = values.len() / 2;
    values[mid_index]
}

fn get_mode(values: &Vec<i32>) -> i32 {
    let mut count_map: HashMap<&i32, i32> = HashMap::new();
    let mut current_mode: (i32, i32) = (0, 0);

    for val in values {
        let count = count_map.entry(val).or_insert(0);
        *count += 1;

        if current_mode.1 < *count {
            current_mode = (*val, *count);
        }
    }

    current_mode.0
}

fn convert_to_pig_latin(val: &String) -> String {
    let mut result: String = String::default();
    let mut chars: std::str::Chars<'_> = val.chars();
    let first_char: Option<char> = chars.next();

    if first_char == None {
        return result;
    }
    
    let first_char: char = first_char.unwrap();
    let mut is_vowel: bool = false;
    let vowels: [char; 12] = ['a', 'e', 'i', 'o', 'u', 'y', 'A', 'E', 'I', 'O', 'U', 'Y'];
    for vow in vowels {
        if first_char == vow {
            is_vowel = true;
            break;
        }
    }

    if is_vowel {
        result = format!("{val}-hay");
    } else {
        let rem_first_char_val = chars.as_str();
        result = format!("{rem_first_char_val}-{first_char}ay");
    }
    return result;
}

fn company_department_game() {
    let mut company_db: HashMap<String, String> = HashMap::new();

    println!("List of commands:\n");
    println!("- Add {{Name}} to {{Service}}\n");
    println!("- List {{Service}}\n");
    println!("- List all employees");

    loop {
        println!("Please input a command");
        let mut command: String = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let parsed_command: Vec<&str> = command.trim().split(' ').collect();
        let first_word: &str = parsed_command[0];
        if first_word.eq("Add") {
            add_company_employee(&parsed_command, &mut company_db);
        } else if first_word.eq("List") {
            list_company_employee(&parsed_command, &company_db);
        } else {
            println!("Incorrect command");
        }
    }
}

fn add_company_employee(parsed_command: &Vec<&str>, company_db: &mut HashMap<String, String>) {
    let employee_name = parsed_command.get(1);
    let service_name = parsed_command.get(3);

    if employee_name.is_some() && service_name.is_some() {
        let employee_name = employee_name.unwrap();
        let service_name = service_name.unwrap();
        company_db.insert(employee_name.to_string(), service_name.to_string());
        println!("Correctly added {employee_name} in service {service_name} to database\n");
    } else {
        println!("Incorrect add employee command\n");
    }
}

fn list_company_employee(parsed_command: &Vec<&str>, company_db: &HashMap<String, String>) {
    let service_name: Option<&&str> = parsed_command.get(1);

    if service_name.is_some() {
        let service_name = service_name.unwrap();
        let all_service = String::from("all");

        println!("List of employees for {service_name} service(s):\n");
        for (name, service) in company_db {
            if service_name.eq(&all_service) || service_name.eq(service) {
                println!("Employee {name} in service {service}\n");
            }
        }
    }
}