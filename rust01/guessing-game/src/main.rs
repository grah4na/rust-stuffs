
use std::io; // this is the i/o library it comes from std library 
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    
    println!("guessing the number !");

    /*
    prelude is the list of things that rust automatically imports
    into every rust program .
     */

    let secret_number =  rand:: thread_rng().gen_range(1..=100);

    /*
    we call the rand::thread_rng function that gives us the particular random number generator we’re going to use: 
    one that is local to the current thread of execution and is seeded by the operating system. Then, we call the 
    gen_range method on the random number generator. This method is defined by the Rng trait that we brought into 
    scope with the use rand::Rng; statement. The gen_range method takes a range expression as an argument and generates
     a random number in the range.
     */

    

    loop {

    println!("please input your guess.");

    let mut guess =  String :: new(); // here the string is a string type provided by std librray 
    // also "new" is associated function that comes with String. the above line is just created a empty string 


    // we can still use it with "std::io::stdin" , even without importing that above lib
    io::stdin()
    .read_line(&mut guess) // here the read_line can take whatever the user input and appned it as string without overwriting the content of it .
    .expect("failed to read line");

    // we could write this as "io::stdin().read_line(&mut guess).expect("Failed to read line");"

    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };
    
    // here we used shadoing to change the type of "guess".

    // the trim method eliminates spaces in both starting and ending 
    // also the trim method eliminates "/n" "/r"
    //the parse method converts the type , here we mentioned "u32" 
    
    
    
    println!("you guessed : {guess}");

    match guess.cmp(&secret_number){

        // the above code here comparing guess with secret number 
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => {
            println!("you won !!!");
            break;
    }

}

}

}
