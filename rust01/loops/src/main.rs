/* 
rust has 3 types of loops 
1. loops 
2, for loop
3. while loop
 */
fn main() {
    lup();
    forloop();
    whileloop();
}

fn lup (){
    let mut counter  = 0;
    let result = loop {
        counter +=1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is {result}")
}

fn forloop(){
    let a = [1,2,3,4,5,6];

    for element in a{
        println!("{element}");
    };
}

fn whileloop(){
    let mut number = 3;
    while number != 0{
        println!("{number}");
        number -= 1;
        break ;
    };
    println!("while loop ended ");
}