//compound data types
// totally there are 4 kind of types 
//array,tuple, slice ,string slice and one thing in rust every data type is immutable unless we explicitly mentioned that "mut"

//array :
fn main(){
let number : [i32;5] = [1,2,3,4,5];
println!("number array  :{:?}",number);

let fruits : [&str;3] = ["apple","banana","mango"];
println!("fruits array  :{:?}",fruits);

//accessing a single element 
println!("1st element of the array :{}",fruits[0]);

//tuples
let human = ("alice",30,false);
println!("tuples : {:?}",human);

let humans:(String,i32,bool) = ("nandan".to_string(),43,true); 

//used ".to_string()" to change it or else we can continue with "&str" asw.

println!("tuples : {:?}",humans);

}




