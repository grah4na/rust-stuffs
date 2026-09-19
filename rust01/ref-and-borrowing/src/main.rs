// fn main(){
//     let mut _x = 5;
//     let _r = &mut _x;
//     *_r += 1;
//     println!("{}",_r);

// }
// here "_r = &x" means i am targeting the x , if "*_r " means targeting the value of x (which is 5)

//demonstration of one mutable ref and many immutable ref

fn main(){
    let mut account =  BankAccount {
    owner : "alice".to_string(),
    bal : 150.55,
    };
// immutable borrow to check balance 
account.check_bal();

// muttable borrow to withdraw money 
account.withdraw(45.5);

account.check_bal();
}
struct BankAccount{
    owner : String,
    bal : f64,
}

impl BankAccount {
    fn withdraw(& mut self,amt : f64){
        println!("withdrawing {} from acc owned by {}", amt ,self.owner);
        self.bal -= amt;
    }

    fn check_bal(&self){
    
        println!("acc owned by {} has a balance of {}", self.owner , self.bal);
    }


}

// OUTPUT : acc owned by alice has a balance of 150.55
// withdrawing 45.5 from acc owned by alice
// acc owned by alice has a balance of 105.05000000000001


fn get_char(data: String)     // here we are giving the ownership of the string to data, the main dont have access to string here the function does 
fn get_char(data: &String)    // "&String" here it just BORROWS it to function , funtions owns nothing 