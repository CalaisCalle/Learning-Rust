use std::f64;
use std::io;

fn main() {
    println!("Please give the values of a, b, and c");

    let mut a = String::new();

    println!("Enter a:");

    io::stdin().read_line(&mut a).expect("Failed to read line");

    let a: f64 = a.trim().parse().expect("ARGH!");

    let mut b = String::new();

    println!("Enter b:");

    io::stdin().read_line(&mut b).expect("Failed to read line");

    let b: f64 = b.trim().parse().expect("ARGH!");

    let mut c = String::new();

    println!("Enter c:");

    io::stdin().read_line(&mut c).expect("Failed to read line");

    let c: f64 = c.trim().parse().expect("ARGH!");

    println!("a = {}, b = {}, c = {}", a, b, c);

    let determinant = b.powi(2) - 4_f64 * a * c;

    if determinant < 0_f64 {
        println!("No real solutions!");
        return;
    }

    let result1 = (-b + determinant) / (2_f64 * a);
    let result2 = (-b - determinant) / (2_f64 * a);

    println!("Results: {}, {}", result1, result2);
}
