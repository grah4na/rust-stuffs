//shadowing in rust 
/*
let x = 5;
let x = x + 1 ;   here we using x value from above and adding +1 to it , which will create a new 
                  value "6" , so now this x is shadowing the above x.

*/
/* 
fn main() {
    let x = 5;
    let x = x + 1; here it is shadowing by creating a new variable 
    let x = 12;
    x = 13 ; here it will return error cuz we are assigning new value 
    {
        let x = x * 2;
        println!(" x in innerscope : {x}");
    }

    println!("x in outer scope : {x}");
    }

*/  

// control flow in rust 

fn main (){
    let age = 17;
    if age >= 18 {
        println!("you can drive ");
    } else if age < 18 {
        println!("you cannot drive  ");
    }
}