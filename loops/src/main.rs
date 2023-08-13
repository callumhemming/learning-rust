fn main() {

    let mut counter: i32 = 0;

    let count_to_ten: i32 = loop{
        println!("Iteration: {counter}");
        if counter == 10{
            //Can return values from a loop with the break statement
            break counter;
        }

        counter += 1;        
    };

    println!("Loop exited at iteration {count_to_ten}");

    //Named loops let you control which loop breaks or continues in nested loops

    let mut second_counter: i32 = 0;

    'counting_up: loop{
        println!("Iteration {second_counter}!");
        if second_counter == 32{
            break
        }

        loop{
            if second_counter == 10{
                break 'counting_up;
            }else{
                break;
            }

        };

        second_counter += 1;
    };
    //Simple for loop, similar to python syntax
    let my_arr =[1,2,3,4,5,6];
    for element in my_arr {
        println!("The element is {element}");
    }

    //Use the standard library range to create a for loop of a known length. Rev is a method that reverses an iterator.
    // (1..20) will create a range 1 to <20, (1..20) will create a range 1 to <=20(Includes 20)
    for number in (1..=20).rev(){
        println!("Counting down {number}");
    }


}
