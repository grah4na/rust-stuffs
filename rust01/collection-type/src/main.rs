/*
collection types in rust 

vectors , utf-8 and hashmaps 
*/


fn main() {

    //> VECTORS 

    let mut _v : Vec<i32> =  Vec :: new();
    let mut _v : Vec<i32> = vec![1,2,3];

    _v.push(5);
    _v.push(6);

    println!("{:?}",_v);

    let third : &i32 = &_v[2]; // direct indexing

    println!("third element through direct indexing : {third}");

    // get method 

    let get_method =  _v.get(2);

    match get_method{
        Some(get_method) => println!("the third element through get method {get_method}"),
        None => println!("there is no element")
    }

    //exaple to conduct actions over vec 

    fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // An example of collecting a vector after mapping.
    // We map each element of the `input` slice to its value plus 1.
    // If the input is `[1, 2, 3]`, the output is `[2, 3, 4]`.
    input.iter().map(|element| element + 1).collect()
}






















    // > UTF -8 
    //Its all about string here 

    let s = "whatever".to_string();
    let s = String ::  from("whatever");

    let mut s1 = String :: from("foot");

    let mut s2 = String :: from("ball");

    // s.push_str("ball");     // for a string slice

    // s.push('!');  // for single char 

    let conc = s1 + &s2; // to add two strings here , also the s1 above lost its ownership 
                                 // and we using &s2 cuz we cant use two ownership  for one strnig

    println!("concatenated string : {conc}");

    // in rust we use "_" to ignore the value 



    



    // how hash map works 


       // use std::collections::HashMap;  we need to import the module from std library

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);



    // accessing key values through loop

        use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }



    






}
