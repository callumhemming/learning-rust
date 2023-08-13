//Expressions do no have an ending semicolon, or they become a statement
//Last expressions are returned implicitly, or can be returned early with the return statement
//Use -> after the params to type the return value: fn myFunc() -> i32{}
fn main() {
    println!("Hello, world!");

    another_function(3, 'r');

    let two_plus_five: i32 = param_plus_five(2);

    println!("{two_plus_five}")
}

fn another_function(amount : u32, symbol: char){
    println!("Another function! {amount} {symbol}");
}

fn param_plus_five(num: i32 ) -> i32{
    //Return statement not needed but will work anyway, can also add a semicolon with return
    num+5
}