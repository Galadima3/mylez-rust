fn main() {
    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // floating point
    let height: f32 = 4.50;

    // bool
    let is_nysc_fun: bool = false;

    let employee: (&str, &str, i8) = ("James", "Software Developer", 29);
    let (_name, _designation, _age) = employee;

    let worker_name: &str = employee.0;

    println!("Employee name is: {worker_name}"); 


}
