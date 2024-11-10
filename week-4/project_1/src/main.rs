use std::io;

fn main() {
    println!("Enter the coefficients a, b, and c of the quadratic equation");

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f64 = input1.trim().parse().expect("Not a valid number");

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f64 = input2.trim().parse().expect("Not a valid number");

    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f64 = input3.trim().parse().expect("Not a valid number");

    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The are 2 ditinct roots ");
        println!("Root 1 = {}, Root 2 = {}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("There is exactly one real root");
        println!("Root = {}", root);
    } else {
        println!("They are no real roots");
        
    }


}
