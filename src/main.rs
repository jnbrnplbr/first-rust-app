fn main() {
    let i = 2;
    match i {
        0 => println!("0"),       // if i = 0
        1 | 2 => println!("1,2"), // if i = 1 or 2
        3..=10 => println!("3,4"), // if i = 3 to 4
        _ => println!("default")   // if non of the above
    }
}
