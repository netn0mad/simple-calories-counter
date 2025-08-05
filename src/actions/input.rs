use std::io::{self, Write};

pub fn get_i32(field_name: String) -> i32 {
    loop {
        print!("{} (integer): ", field_name);
        io::stdout().flush().expect("Error buffer flush.");

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Error. Bad input");

        return match number.trim().parse::<i32>() {
            Ok(number) => number,
            Err(_) => {
                println!("Error input.");
                continue;
            }, 
        }
    }
}

pub fn get_f32(field_name: String) -> f32 {
    loop {
        print!("{} (float): ", field_name);
        io::stdout().flush().expect("Error buffer flush.");

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Error. Bad input");

        return match number.trim().parse::<f32>() {
            Ok(number) => number,
            Err(_) => {
                println!("Error input.");
                continue;
            },
        };
    }
}

pub fn get_string(field_name: String) -> String {
    loop {
        print!("{}: ", field_name);
        io::stdout().flush().expect("Error buffer flush.");

        let mut str = String::new();

        io::stdin()
            .read_line(&mut str)
            .expect("Error. Bad input");

        let product_name = str.trim().to_string();

        if str.is_empty() {
            println!("Please enter a non-empty string");
            continue;
        }
        
        return product_name;
    }
}
