//compound data types
// totally there are 4 kind of types 
//array,tuple, slice ,string slice and one thing in rust every data type is immutable unless we explicitly mentioned that "mut"

//array :
fn main(){
let number : [i32;5] = [1,2,3,4,5];
println!("number array  :{:?}",number);

//every element in an array should have same length also arrays are fixed length 

let a = [3; 5];
// this is same as writing a = [3,3,3,3,3]

let fruits : [&str;3] = ["apple","banana","mango"];
println!("fruits array  :{:?}",fruits);

//accessing a single element 
println!("1st element of the array :{}",fruits[0]);

//tuples
let human = ("alice",30,false);
println!("tuples : {:?}",human);

//once we declare the tuples , it cant be shrinked or grow  
//tuples can have diff types and 

let humans:(String,i32,bool) = ("nandan".to_string(),43,true); 

//used ".to_string()" to change it or else we can continue with "&str" asw.

println!("tuples : {:?}",humans);


fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    // we can access the tuple value like this 





//slices :
let numberslices : &[i32]= &[1,2,3,4,5];
let animalslices : &[&str] = &["lion","tiger","elephant"];

println!("slices : {:?}",numberslices);
println!("slices : {:?}",animalslices);

//stings : which are mutable ,growable and owned string type

let mut travis : String = String :: from("loneliness has followed me my whole life,");
//travis.push_str("everywhere"); here we cannot borrow as mutable , 
travis.push_str("everywhere");
println!("speak: {}",travis);

//what is string slice :

let string : String = String :: from ("hello world");
let slice : &str = &string[0..5];

println!("slice is :{}",slice)

}



// diff btw string ,string slice & slice
// String` → Owns the text, stored on the heap, and can grow/shrink.
//`&str` (string slice) → Borrows text; it’s a view into existing UTF-8 string data and doesn’t own it.
//`&[T]` (slice) → Borrows a portion of an array/vector of any type `T`, without owning it.

//Simple analogy: `String` = owning a book, `&str` = reading a page from it, `&[T]` = viewing part of any list.

