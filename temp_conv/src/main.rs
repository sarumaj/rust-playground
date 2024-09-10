fn main() {
    println!("Enter the temperature to convert, supply °C or °F to define the unit!");

    let mut input = String::new();
    loop {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");

        let input = input.trim();
        if input.is_empty() || input.len() <= "°".len() + 1 {
            println!("No input provided!");
            continue;
        }

        let (temperature, unit) = input.split_at(input.len() - 1 - "°".len());
        let temperature: f64 = match temperature.trim().parse() {
            Ok(temperature) => temperature,
            Err(_) => {
                println!("Invalid value: {temperature}, expected a valid floating point number!");
                continue;
            }
        };

        let result = match unit {
            "°C" => celsius_to_fahrenheit(temperature),
            "°F" => fahrenheit_to_celsius(temperature),
            _ => {
                println!("Invalid unit: {unit}, expected: \"°C\" or \"°F\"!");
                continue;
            }
        };

        println!(
            "Result: {:.2}{}",
            result,
            if unit == "°C" { "°F" } else { "°C" }
        );
        break;
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    return celsius * 9.0 / 5.0 + 32.0;
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    return (fahrenheit - 32.0) * 5.0 / 9.0;
}
