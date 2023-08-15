fn main() {
    //Stack: Last in, first out, known size and shape, fast and organised 
    //Heap, less organised, allocated memory to fit types of unknown size and shape. Stores pointers in the stack

    //String literal: Known size and value, stored on the stack: let a = "Hello, world!"
    //The String type, stored on the heap, allocates memory for unknown size and shape of string (Such as storing user input in a string, unknown at compile time)

    //String stored on the heap.
    let _a = String::from("Hello!");

    //As it allocated memory in the heap, this string can be mutated, whilst string literals: "Hello" cannot
    let mut b:String = String::from("Hello,");

    //Mutates string type
    b.push_str(" world!");

    println!("Mutated string: {b}");

    //We request an allocating of memory from the memory allocator at run time with the String struct, this will then need to be returned when we're done with it. 
    // Rust will automatically return the memory once the variable goes out of scope. 

    //Let's make two strings, the second copies the first
    let first: String = String::from("Hello");
    let _second: String = first;

    //The data of this are stored in the heap, in allocated memory, the pointer, length and capacity are stored in the stack, with the pointer pointing to the data in the heap.
    //Effectively, the stack stores metadata that points to the data in the heap
    // When we copy this data in second, we create a new variable in the stack, that points to the same data in the heap.
    //It's important to stress when we use a previous variable to create a new variable, the data is NOT copied in the heap. 

    //As they both go out of scope, they will both try to clean up the same data in the heap, which is known as a double free error, which can cause corruption and vulnerabilities.
    //To address this, first is removed from the stack after second=first 
    // println!("This is invalid: {first}");
    //This is known as a move, similar to a shallow copy, but the first variable is invaldiated on the stack

    //If we want to deeply copy the heap data in first to second, we can use the .copy() method. Both variables will be valid

    let some_data : String = String::from("This is some data in the heap");
    let copy_some_data : String = some_data.clone();

    println!("Both variables are valid: {} {}", some_data, copy_some_data);

    //HOWEVER: The previous section refers to data that is of an unknown size and shape at compile time, therefore the data is stored in the heap
    //with a reference in the stack. Data that is known at compile time such as integers, string literals are stored only on the stack, therefore, we don't need to worry about
    //a move or shallow copying and thus double free errors. These data types can copy eachother without invalidating the previous variables

    let y: i32 = 2;
    let x: i32 = y;

    println!("Both variables are valid as we don't need to invalidate y: {} {}", y,x );

    //These values that don't allocate data in the heap have a copy trait
    //All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating-point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

    //Functions
    //Passing a variable to a function has the same effect as assigning it to a variable. Therefore, if the variable has data in the heap, the variable will be invalid;
    //Ownership will move to the function param

    fn pass_to_me(var: String) -> String {
        var
    }

    let my_string: String = String::from("Hello world!");

    let my_string_clone : String = my_string.clone();
    pass_to_me(my_string);

    // println!("This is invalid: {my_string}");
    println!("This is valid: {my_string_clone}");

}
