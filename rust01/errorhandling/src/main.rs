// error handling has two types 

//approch 1 : Result (T,E)
//approach 2 : option (T)

/*
1: enum Option<T>{
some(T), #represnt a value 
None ,   #represnt the absesnce of value 
}

2 : enum Result <T,E>{
Ok(T), //represnt a value 
Err(E), // represnts an error 
}
*/

//example :

fn DivideOption(num : f64 , denom : f64) -> Option<f64> {
    if denom == 0.0 {
        None 
    } else {
        Some(num / denom)
    }
}

fn DivideResult (num : f64,denom : f64) -> Result<f64,String>{
    if denom == 0.0{
        Err("cannot devide by zero ".to_string())
    } else {
        Ok(num/denom)
    }
}

fn main() {

    let result = DivideOption(10.0, 0.0);

    match result {
        Some(x) => println!("result {}",x),
        None => println!("denom cant be zero !!! "),

    };

    // Direct match on return value - no `let` needed before match
    // `Ok(result)` and `Err(err)` are PATTERNS that CREATE new variables:
    // - `result` binds the f64 inside Ok, lives only in this arm (=> ...)
    // - `err` binds the String inside Err, lives only in this arm
    // You could name them anything: Ok(val), Ok(x), Ok(v)
    match DivideResult(100.0 , 0.0){
        Ok(result) =>println!("result {}", result), // `result` created here by destructuring Ok(T)
        Err(err) => println!("error {}",err),       // `err` created here by destructuring Err(E)
    }
       

    /*
    fn main (){
        panic!("its a panic macro we use to create warning")
    }

    if we need to know which function causing the panic we can run program in this manner : RUST_BACKTRACE=1 CARGO RUN  which will backtrace the panic in this program

    
     */
     

//      fn test() -> Result<i32, String> {
//     let x = divide(10, 0).unwrap();
//     Ok(x)                                // here this function crashes due to unwrap 
// }



// fn test() -> Result<i32, String> {
//     let x = divide(10, 0)?;
//     Ok(x)                                // here returns the error from the current function due to "?"
// }

}
