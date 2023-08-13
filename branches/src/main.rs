use std::io::stdin;

fn main() {
    let mut num: String = String::new();

    println!("Enter a number:");

    
    stdin().read_line(&mut num).expect("Error reading line");
    

    let num: i32 = num.trim().parse().expect("Not a number");

    if num == 1 {
        println!("Number is one");
    }else{
        println!("Number is not one")
    }

    match num > 1{
        true => println!("Number is greater than one"),
        false => println!("Number quantity is greater than one")
    }

    //You can use if in a right side assignment operation. This is weird?! For example: 
    let my_new_var: &str  = if num>10 { "Greater than 10" } else { "Less than 10" };

    println!("{my_new_var}")

}
