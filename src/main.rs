use std::io::{self, Write};
use num::rational::Ratio;


const BLUE: &str = "\x1b[34m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[32m";
const CIAN: &str =  "\x1b[36m";
const WHITE: &str = "\x1b[37m";

fn main() {
    println!("{}𝑎𝑥²+𝑏𝑥+𝑐=0{}", YELLOW, RESET);
    let (_a, _b, _c) = get_coefficients();

    println!("{}Numbers entered: a = {}, b = {}, c = {}{}", CIAN, _a, _b, _c, RESET);

    //Call funtion
    quadratic_formula(_a, _b, _c);
}

fn quadratic_formula (a: f64,b: f64, c: f64) {
    let discriminant = b.powi(2) - 4.0 * a * c; // Discriminante

    if discriminant < 0.0 {
        complex_numbers(a, b,  discriminant)
    }else {
        real_numbers(a, b,  discriminant);
    }
}

fn complex_numbers(a: f64, b: f64, discriminant: f64) {
    let sqrt_discriminant_i = discriminant.abs().sqrt();
    let x_i = sqrt_discriminant_i / (2.0 * a);
    let x_r = -b / (2.0 * a);

    let approx_x_i: Ratio<i64> = Ratio::approximate_float(x_i).unwrap().reduced();
    let approx_x_r: Ratio<i64> = Ratio::approximate_float(x_r).unwrap().reduced();

    if x_i == 1.0{
        println!("Result complex number: {}x = {} ± i{}",GREEN, approx_x_r, RESET);

    }else{
        println!("Result complex number: {}x = {} ± {}i{}",GREEN, approx_x_r, approx_x_i, RESET);
    }
  
}

fn real_numbers(a: f64, b: f64, discriminant: f64){
    let sqrt_discriminant = discriminant.sqrt();
    
    let x1 = (-b + sqrt_discriminant) / (2.0 * a);
    let x2 = (-b - sqrt_discriminant) / (2.0 * a);

    // Convert to simplified fractions
    let approx_x1: Ratio<i64> = Ratio::approximate_float(x1).unwrap().reduced();
    let approx_x2: Ratio<i64> = Ratio::approximate_float(x2).unwrap().reduced();
    
    println!("{}Result real number:{} {}x1 = {}{}, {}x2 = {}{}", WHITE, RESET, GREEN, approx_x1, RESET, YELLOW, approx_x2, RESET);
}


fn get_coefficients() -> (f64, f64, f64) {
    let _a = read_int("a");
    let _b = read_int("b");
    let _c = read_int("c");

    (_a, _b, _c)
}

fn read_int(name: &str) -> f64 {
    loop {
        print!("{}Enter the coefficient {}: {}", BLUE, name, RESET);
        io::stdout().flush().unwrap(); //on the same line

        let mut input = String::new();
        if let Err(_) = io::stdin().read_line(&mut input) {
            println!("{}Error reading entry. Please try again.{}", RED, RESET);
            continue;
        }

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("{}Invalid input. Please enter an integer.{}", RED, RESET),
        }
    }
}
