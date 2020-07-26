fn main() {
    println!("Hello! Which unit would you like to convert from? \
        Enter 'f' for Farenheit, or 'c' for Celsius.");

    let mut unit = String::new();
    std::io::stdin().read_line(&mut unit)
            .expect("You entered an invalid unit.");

    let unit : char = match unit.trim().parse() {
        Ok(c) => c,
        Err(_) => panic!("Invalid unit selected. Exiting.")
    };

    println!("Enter the reading");

    let mut temp = String::new();
    std::io::stdin().read_line(&mut temp).expect("Please enter a valid temperature");

    let temp : f64 = match temp.trim().parse() {
        Ok(d) => d,
        Err(_) => panic!("Invalid temperature value entered. Exiting")
    };

    let new_temp = if unit == 'c' {
        (temp*9.0/5.0)+32.0
    } else {
        (temp-32.0)*5.0/9.0
    };

    let new_unit : char = if unit == 'c' {
        'F'
    } else {
        'C'
    };

    println!("Converted temperature = {}°{}", new_temp, new_unit);
}
