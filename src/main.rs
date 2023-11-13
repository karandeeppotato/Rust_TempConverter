use std::io;

fn f_to_c() {
    println!("Enter the temperature value in fahrenhiet: ");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read temp value");
    let temp: f32 = temp.trim().parse().expect("Please enter a floating point number");
    let result = (temp - 32.0) * (5.0 / 9.0);

    println!("The celcius value is: {result}");
}

fn c_to_f() {
    println!("Enter the temperature value in celcius: ");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read temp value");
    let temp: f32 = temp.trim().parse().expect("Please enter a number");
    let result = temp * (9.0 / 5.0) + 32.0;

    println!("The fahrenhiet value is: {result}");
}

fn main() {
    println!("Temperature Converter");
    loop {
        println!("1. Fahrenheit to Celcius");
        println!("2. Celcius to Fahrenhiet");
        println!("3. Exit");
        println!("Enter your option: ");

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read line");
        let option: i32 = option.trim().parse().expect("Please enter a number");
        if option == 1 {
            f_to_c();
        } else if option == 2 {
            c_to_f();
        } else if option == 3 {
            break
        }else {
            println!("Invalid Option !! Program quiting....");
        }
    }
}
