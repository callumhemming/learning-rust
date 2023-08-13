fn main(){
    // let x : u8 = 256; Limit of an unsigned 8bit integer is 255, this will cause an error. 
    // '-2' signed 8bit, -22222, signed 32 bit, i32
    //ixx means signed, uxx means unsigned: e.g i32, u32 
    // Floating point are fxx, the default is f64 as they're twice as accurate than f32 floats. 
    // char is ''
    let x = 5;
    println!("The value of x is : {x}");
    let x = x+6; // Shadow x
    println!("The value of x is : {x}");

    //Tuple, fixed length compound type
    let my_tup : (i32, u32, f64) = (-20, 200, 20.50);

    //Destructure tuple
    let (first, second, third) = my_tup;

    //Access tuple by index
    let one = my_tup.0;
    let two = my_tup.1;
    let three = my_tup.2;

    println!("First {one} second {two} third {three}");
    println!("First {first} second {second} third {third}");

    //Arrays: Cannot change length, can only store data of the same type, stored on the stack, not heap
    // let my_arr = [1,2,3,4,5];
    // Can also create an array of nth size by providing an inital value ( let a = [2;n] )
    // let five_twos = [5;2];




}