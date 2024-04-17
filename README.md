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
    
let arr: [u8; 3] = [1,2,3]; //where type: u8, size: 3
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

### Functions
```rust
//  num represent a variable and bool is what value it should return.
pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0 //this part returns a boolean
}
//usage: println!("{}",is_even(3));
//output: false
```
### Mutability
```rust
// On this part, ever variables in rust cannot be re-assign unless mutable

//Sample1: this sample will run error, since you can not assign another value on the num variable unless you include mutability.
    let num = 5;
    num = 3;
    println!("{}",num);

//Sample2:
    let mut num = 5;
    num = 3;
    println!("{}",num);
```

### Arrays + Slices
```rust
fn main() {
    let arr = [0,4,7,3];        //array with size of 4 and 4 data
    let slice = &arr[2 .. 4];   // 2 index of the start and 4 is the end. simply from 2nd index to the 4th index will be the value of the slice variable.
    borrowing_slice(arr, slice);
}

fn borrowing_slice(arr: [u8; 4], slice: &[u8]) {
    println!("{:?}", arr); // prints everything inside the arr
    println!("{:?}", slice); // prints everything inside the slice 
    println!("length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}
```

### Strings
```rust
    let str: &str = "hello world";
    let mut string: String = String::from("Hello World");
    let slice = &str[.. 6]; // get from 0 to 6 only | [3 ..]
    string.push('1'); // .push for character only
    string.push_str("! Bob"); // .push_str for string. 

    println!("{}", str);
    println!("{}", string);
    println!("{}", slice);
```
### If Statement
```rust
    let n = 3;

    if n > 0 {
        println!("greater than 0");
    } else if n < 0 {
        println!("less than 0");
    } else {
        println!("is 0");
    }
```
### For Loop
```rust
    for i in 0..6 {
        println!("{}",i);
    }
```
### While Loop
```rust
    let mut i = 0;
    while i < 5 {
        println!("{}",i);
        i += 1;
        if i == 3 {
            println!("exit at {}", i);
            break; // stop the loop
            //continue; // skip the current loop
        }
    }
```
### Match Statement
```rust
    let i = 2;
    match i {
        0 => println!("0"),       // if i = 0
        1 | 2 => println!("1,2"), // if i = 1 or 2
        3..=10 => println!("3,4"), // if i = 3 to 4
        _ => println!("default")   // if non of the above
    }
```