// in rust we use snake case
//we can declare the function anywhere in the program and can call it is known as Hoisting

fn main(){
hh(172);
human("baro" , 37);
let _Y = {
    let kg = 10;
    let mg = 5 ;
    kg * mg   // new feature of rust we dont need to use ";" here to end it just returns that value without error also we can use "return" key word if we want
};
println!("result of y : {}", _Y);

let x = add(2,2);  // we can assign a function value to the variable 
println!("result of x:{}",x);

println!("value of function 'add' : {}",add(3,3));

let wght = 23.6;
let hgt=122.7;
let bmi = calc(wght,hgt);
println!("calculated bmi is : {:.2}",bmi);

}

fn hh(height : u32 ){
    println!("my height is {} cm", height);
}

// function with more parameters
fn human(name : &str,age : u32 ){
    println!("my name is {} and i am {} year old.",name , age);
}

//expression and statement in programming language
// expression : means anything that returns a value (ex : add(5,4), if condition etc)
// statements : anything that doesnt return a value (let x= 5 "here it is not retuening anything")

fn add(a:i32, b : i32) -> i32{
    a+b
}

fn calc(wt : f64 , ht : f64) -> f64{
    wt / (ht * ht)
}