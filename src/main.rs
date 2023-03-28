use std::io;

fn main() {
    println!("Enter the interval to calculate the integral:");
    println!("Starting point: ");
    
    let mut start = String::new();
    io::stdin().read_line(&mut start).expect("Failed to read string");
    let start: f64 = start.trim().parse().expect("Please enter a number");
    
    println!("End point: ");
    
    let mut end = String::new();
    io::stdin().read_line(&mut end).expect("Failed to read string");
    let end: f64 = end.trim().parse().expect("Please enter a number");
    
    println!("Enter the number of sections for the trapezium method: ");
    
    let mut num_divisions = String::new();
    io::stdin().read_line(&mut num_divisions).expect("Failed to read string");
    let num_divisions: f64 = num_divisions.trim().parse().expect("Please enter a number");
    
    let dx = (end - start) / num_divisions;
    
    let mut integral = 0.0;
    
    for i in 0..(num_divisions as i32) {
        let x_i = start + i as f64 * dx;
        let x_iplus1 = start + (i+1) as f64 * dx;
        let f_xi = func(x_i);
        let f_xiplus1 = func(x_iplus1);
        
        integral += dx * (f_xi + f_xiplus1) / 2.0;
    }
    
    println!("Integral = {}", integral);
}

fn func(x: f64) -> f64 {
    x.powi(2) + 1.0
}
