//in integer 
    //so i16,i32,i64,i128 : signed integer 
    //and u16,u32,u64,u128 : unsigned integer 
    // here the number represnt the size in bit 
    fn main() {
    let x: i32 = -42;
    let y: u64 = 100;
    println!("signed integer: {}",x);
    println!("unsigned integer:{}",y); 
    
    //float 
    //f32 , f64
    let pi : f32 = 3.14;
    println!("float value  : {}",pi);

    //booolean values
    let is_snowing :  bool = true;
    println!("is it snowing : {}",is_snowing);

    //character type 
    // there is only one type

    let character : char = 'a';
    println!("the charatcter is :{}",character);

    }