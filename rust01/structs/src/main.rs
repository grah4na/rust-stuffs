//structs are similar to tuple but with no type inference 

// rust has 3 types of struct 
// 1.regular struct 2.tuple struct(basically just named struct ) 3.unit strcut (These don't have any fields and are useful for generics.)


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


    fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,           // it is called field init shorthand 
        email,
        sign_in_count: 1,
    }
}


//tyep 


}


// use of structs in a program

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }




// rust will give some functionality to debug issue here are some

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(&rect1);
// }

/*
unit like struct 



*/