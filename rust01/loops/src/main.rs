/* 
rust has 3 types of loops 
1. loops 
2, for loop
3. while loop
 */
fn main() {
    lup();
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

