use std::io;

fn main() {
    println!("Is employee experienced (true/false):");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let experience: bool = input1.trim().parse().expect("Please enter a valid answer (true or false)");

    println!("Enter employee's age:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input enter valid age");
    let age: i64 = input2.trim().parse().expect("Please enter a valid age ");

    let mut incentive = 0;

    if experience  {
        if age >= 40 {
            incentive = 1_560_000;
        }else if age < 40 && age  >= 30 {
            incentive = 1_480_000;
        }else if age < 28 {
            incentive = 1_300_000 * 12;
        }
    } else {
        incentive = 100_000;
    }

    println!("The annual incentive is: N{}", incentive);


}

