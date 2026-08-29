//structs are similar to tuple but with no type inference 


fn main() {
    
    let mut book1 = Book {
        title : "mybook".to_string() ,
        author : "me".to_string(),
        pages : 176,
        available : true ,

    };

    //struct should be defined outside the main.


    struct Book {
        title : String,
        author : String,
        pages : u32 ,
        available : bool,
    }

    // Assignment happens here, outside the struct initializer
    book1.author = String::from("not me");
    
    println!("Author: {}", book1.author); // prints "not me"

    // //if i want to create a new instance from a existing one

    // let book2 = Book {
    //     title : String::from("New Title"),
    //     ..book1
    // }
    // which will copies everything from book1 


    //tuple struct 

    struct Color  (u32,u32,u32); // onlt accepts the data types like place holders.

    let black = Color (12,11,23);

    //accessing it 
    println!("red {} , green {} , blue {}",black.0 , black.1 , black.2);
}
