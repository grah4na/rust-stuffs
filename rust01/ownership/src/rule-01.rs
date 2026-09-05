// ownership in rust 
// 3  rules in rust-ownership
//1. each value in rust has an owner 
//2. there can be only one owner at a time 
//3. when the owner goes out of scope the value will be dropped.
fn main() {
    let s1 = String :: from ("rust");
    let len = calc_len(&s1);
    // i am accessing the value "&s1" without taking ownership which is called reference.
    //Because the reference does not own it, the value it points to will not be dropped when the reference stops being used.

    /*
     The ownership of a variable follows the same pattern every time: Assigning a value to another
     variable moves it. When a variable that includes data on the heap goes out of scope, 
     the value will be cleaned up by drop unless ownership of the data has been moved to 
     another variable.
     */

    // let s2 = s1;
    // println!("{}",s1);
    // it wont run , here 's2' got the ownership so s1 is now invalid.
    println!("lenght of '{}' is : {}",s1,len);



    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

    //in this example , nothing is refering to the heap of original value of "s" ,
    // so rust auto calls drop function and the org "s" becomes invalid.
    
        let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}"); // in this method the heap data get copied.



    {
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
    } // it looks like contradict but the ineteger data types is known so rust stores this in stack
    // so there is no reason to call clone here .

    
}
fn calc_len(s:&String)->usize {
    s.len()
}

// fn lost(s:&String){
//     println!("{}",&s1);
// }
// RULE 03 : this function wont work , here the ownership is out of scope, so the value will be dropped.


    // {
    //     let s = String::from("hello"); // s is valid from this point forward

    //     // do stuff with s
    // }                                  // this scope is now over, and s is no
    //when a variable goes out of scope rust auto calls a function which name is "drop"
    // which frees the heap memory 


    /*
    rust has a special annotation called 'copy', which can be stored in stack 

    but it has some condition where we can use the "copy" anootation
    Here are some of the types that implement Copy:

    All the integer types, such as u32.
    The Boolean type, bool, with values true and false.
    All the floating-point types, such as f64.
    The character type, char.
    Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, 
    but (i32, String) does not.

     */

    