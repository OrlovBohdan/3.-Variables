#[test]

// Remove a line in the code to make it compile
fn main() {
    let mut x = 1;
    x = 7;
    // Shadowing and re-binding
    x += 3;


    let y = 4;

    let y = "I can also be bound to text!";


    println!("Success!");
}