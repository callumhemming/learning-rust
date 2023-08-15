fn first_word(s: &String) -> &str{ // Returning a string slice rference 
    let bytes: &[u8] = s.as_bytes(); // Convert string to an array of unsighned 8bit bytes as references

    for (i, &item) in bytes.iter().enumerate(){ //Return each item in the array, use enumerate to wrap the return value of iter ina  tuple, where the first is the index the secong is a referance of value
    if item == b' ' {
        return &s[0..i]; // Find the first space and return everything before it as a string slice 
    }

} //Now instead of returning a usize that is not connected to the memory of our string, we return a slice reference to the actual string in memory. If we use the slice string and the string is invalid, the compiler will throw an error 

    &s[..]
}


fn main() {

    let mut s = String::from("Hello world");

    let word = first_word(&s); // Returns to the value of 5 (The end of the first word is 5)

    s.clear(); //Empties the string, makes it equal to ""

    //As the word variable is new memory and not conntected to the state of s, it's now invalid without any obivous erros. However we can create a slice reference using the range syntax which creates a pointer on the stack that has information on the start and length of a slice.

    let _hello_slice = &s[0..5]; // Creates a slice that points to values 0-5
    // You can also drop the beginning if you want to start beginning of a string: [..5] and drop the end if you want to slice until the end: [5..]. You can also slice eveything with : [..]
    // 

    //You use references because if you pass ownership to a function, the variable will be invalid when the function goes out of scope]

    //You'll often take string slices references as function params instead of String because you can use the same function on both 

    //Slices can be made from other data types such as arrays;

    let my_arr = [1,2,3,4,5] ;// [i32; 5] Array of signed 32 bit integers with a length of 5

    let my_arr_slice = &my_arr[1..2]; // &[i32]
    
    
}
