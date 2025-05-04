use std::io;

fn main() {
    /*
    // Constants
     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

     // floating point
     let height: f32 = 4.50;

     // bool
     let is_nysc_fun: bool = false;
    */

    let a = false;  
    // Tuple
    let employee: (&str, &str, i8) = ("James", "Software Developer", 29);
    let (_name, _designation, _age) = employee;
    let worker_name: &str = employee.0;
    println!("Employee name is: {worker_name}");

    // Arrays
    let colors: [&str; 5] = ["red", "purple", "yellow", "blue", "green"];

    loop {
        println!("Enter enter an array index ...");
        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to get input... ");

        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };

        if index >= colors.len() {
            println!("Index out of bounds! Max index is {}", colors.len() - 1);
            continue;
        } else {
            let element = colors[index];
            println!("The value of the element at index {index} is: {element}");
            break;
        }
    }
}
