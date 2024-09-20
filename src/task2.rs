#[test]

fn main() {
    let mut x = 1;  // Declare a mutable variable
    x += 2;         // Increment the value of x by 2

    assert_eq!(x, 3); // Check that x equals 3
    println!("Success!");
}
