fn main() {
    let num_list = vec![10,20,30,40,50];
    
    let dis = get_largest(num_list);
    println!("largest number is {dis}");

// if we want to find the same logic for another number list we need to create another loop which is a repetition of code 

// fn get_largest (number_list:Vec<i32>) -> i32{
//     let mut largest = number_list[0];

//     for number in number_list {
//         if number > largest{
//             largest = number;
//         }
//     }
//     largest                                      // what if we have other type than vector to add , we cant write another new function
                                                    //this is where we use generic 
// }

fn get_largest<T: PartialOrd + Copy>(number_list:Vec<T>) -> T{
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest{                    // here we cant compare any type so we need "restrict" our generic to needed types so we use traits for that 
            largest = number;
        }
    }
    largest
}


//"A generic struct/enum is a container whose stored type is decided later."

struct Point<T>      // one generic type
struct Point<T, U>   // two generic types

// i have seen generic in enums before like Option and Result which holds the <T>

/*
enum Option<T> {
    Some(T),
    None,
}

    enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

}
