fn main() {
    let tuple: (u8,bool,f32) = (5,true,2.1);
    println!("first: {}, second: {}, third: {}",tuple.0,tuple.1,tuple.2);
    println!("{:?}",tuple);

    // destructuring or reassigning the value of tuple to a,b,c
    let (a,b,c) = tuple;

    println!("{},{},{}", a,b,c);
}