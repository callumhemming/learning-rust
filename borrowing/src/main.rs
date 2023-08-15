fn main() {

    //References, to pass data to a variable without transferring ownership, you can pass a reference
    //Instead of creating a new reference in the stack to heap data, we can pass a reference to the data, so the param will not take ownership of the data. 

    fn calcualte_length(s: &String) -> usize{
        s.len() // s goes out of scope, but because it dosen't have ownership of the value it doesnnt get dropped. 
    }

    let s1: String = String::from("Hello");

    let len: usize = calcualte_length(&s1);

    println!("String: {s1} Length: {len}");

    //References are immutable by default, if you want to change them, then you need to use a mutable reference
    //The owner of a mutable refernce must also be mutable

    let mut mutable_string : String = String::from("Hello");

    fn update_hello(str : &mut String) {
        str.push_str(" world!");
    }

    update_hello(&mut mutable_string);

    println!("Value of mutable_string: {mutable_string}");

    //There can only ever be one mutable reference at a time:

    let mut _new_string : String = String::from("Hello");

    // let _rs = &mut new_string;
    // let _rs_two = &mut new_string;

    // println!("{_rs} {_rs_two}"); Errors

    //This prevents data races from occuring. The three conditions for a data race are:
    // Two or more pointers access the same data at the same time.
    // At least one of the pointers is being used to write to the data.
    // There’s no mechanism being used to synchronize access to the data

    //We also can't have an immutable reference if we already have an immutable reference
    //Multiple immuteable references are allowed however, as they'r readonly.

    //References scopes start from when they're introduced to when they're last used. So if you use a mutative reference in a println!, you can create a new mutative reference

    //Dangling refernces. In other languages, a pointer can point to data that has been reallocated, in rust this is not possible as the compiler will guarantee 
    // A reference will always point to the right data whilst it exists, otherwise it will throw an error. 
    
    //For exmple, this will not compile:
    
    fn main() {
        let reference_to_nothing = dangle();
    }
    
    fn dangle() -> &String {
        let s = String::from("hello");
        
        &s
    }
    
    //As the returned reference will be pointing to a variable that is invalidated as its scope ends. Therefore it is a dangling reference as the compiler will error.

    


}