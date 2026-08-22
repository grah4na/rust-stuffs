// ownership in rust 
// 3  rules in rust-ownership
//1. each value in rust has an owner 
//2. there can be only one owner at a time 
//3. when the owner goes out of scope the value will be dropped.
fn main() {
    let s1 = String :: from ("rust");
    let len = calc_len(&s1);
    // i am accessing the value "&s1" without taking ownership which is called reference.

    // let s2 = s1;
    // println!("{}",s1);
    // it wont run , here 's2' got the ownership so s1 is now invalid.
    println!("lenght of '{}' is : {}",s1,len);
    
}
fn calc_len(s:&String)->usize {
    s.len()
}

// fn lost(s:&String){
//     println!("{}",&s1);
// }
// RULE 03 : this function wont work , here the ownership is out of scope, so the value will be dropped.