struct User {
    email: String,
    password: String,
    balance: u32,
}
impl User {
    fn validate_password(&self) -> bool {
        if self.password.is_empty() {
            return false;
        }
        true
    }
    fn validated_email(&self) -> bool {
        if self.email.is_empty() {
            return true;
        }
        false
    }
}
fn trasaction(from: &mut User, to: &mut User, amount: u32) -> String {
    if from.balance < amount {
        return "Your balance is lower than amout".to_string();
    } else {
        from.balance = from.balance - amount;
        to.balance = to.balance + amount
    }
    format!("The value {} is aready completed", amount)
}
fn main() {
    let mut user_one = User {
        email: String::from("g@gmail.com"),
        password: String::from("123"),
        balance: 1000,
    };
    let mut user_two = User {
        email: String::from("r@gmail.com"),
        password: String::from("1234"),
        balance: 1000,
    };
    println!("{}", trasaction(&mut user_one, &mut user_two, 2000));
    println!("New balance {}", user_one.balance)
}
