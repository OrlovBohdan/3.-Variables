#[test]


// Fix the error with the use of define_x


fn main() {
    let x = define_x(); // Get the value of x from the define_x function
    println!("{}, world", x);
}

fn define_x() -> &'static str {
    let x = "hello";
    x // Return x
}
