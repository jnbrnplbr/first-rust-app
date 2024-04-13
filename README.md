> This is my first rust written app for my first blockchain app on Solana Ecosystem.


### Trying the newly imported library - "ferris_says"
```rust
use ferris_says::say; // this is a package or library available for rust
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = 12;//message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}
```

### Differentiating Variables
```rust
// FOR NUMBERS
let unsigned: u8 = 10;
let signed: i8 = -10;
let float: f32 = 1.2;

println!("unsign: {} sign: {} float: {}", unsigned, signed, float);

// FOR STRINGS
let letter  =  "i am a letter";
let emoji = "\u{1F600}";
println!("letter: {} emoji: {}",letter,emoji);

// BOOLEAN
let is_true: bool = true;
println!("is true? {}",is_true);
// Note: you can also negate the value by adding "!" like !is_true
```


### Arrays
```rust
let arr: [u8; 3] = [1,2,3];
let other_arr: [u8; 5] = [100;5];

println!("index: {}, length: {}", arr[0], other_arr.len());
println!("{:?}", other_arr); // this line of code prints the current data holds by other_arr variables.
```


### Tuples
```rust
    let tuple: (u8,bool,f32) = (5,true,2.1);
    println!("first: {}, second: {}, third: {}",tuple.0,tuple.1,tuple.2);
    println!("{:?}",tuple);

    // destructuring or reassigning the value of tuple to a,b,c
    let (a,b,c) = tuple;

    println!("{},{},{}", a,b,c);
```