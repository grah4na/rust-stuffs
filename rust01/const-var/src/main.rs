//variable , const & mutability 





fn main() {
    //let a = 5; in rust everything is immutable by default.but in variable perticularly we can assign it as mutable with keyword "mut"
    // also in variable the type will be assigned default but in constant we need to explicitly mention that .

    let a = 5;
    println!("value of a is {}",a);

    //const mut b : u16 = 5; 
    // in constants we cant use "mut" also need to specify the data type asw.

    const Y : i16 = 8 ;

    // according to rust-book rule the const name should be in capital letter 

    println!("value of Y is : {}",Y);

    //usize represents sizes and array indexes.


}
