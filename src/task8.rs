#[test]

// Fix the error below with least amount of modification
fn main() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

//виникає помилка через те, що значення змінної x оголошено як незмінну (immutable) за допомогою ключового слова let, і спроба змінити його викликає помилку.
//
// Щоб виправити помилку з мінімальними змінами, потрібно зробити x змінною (mutable) за допомогою ключового слова mut.