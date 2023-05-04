fn main() {
    // ----- OWNERSHIP RULES -----
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    let s1 = String::from("hello");
    
    // takes_ownership(s1);
    // this will error out because s1 is moved to the function

    let s2 = s1.clone(); // here we are cloning the heap data of s1 to s2
                         // if we don't clone, s1 will be moved to s2 and s1 will be invalid 

    println!("{}", s2);

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    // calling reference functions
    let mut s2 = String::from("hello");
    let len = calculate_len(&s2);

    println!("{} - {}", s2, len);

    change(&mut s2);
    println!("{}", s2);
}

// ----- REFERENCES AND BORROWING -----

// fn calculate_len(s: String) -> (String, usize) {
//     let length = s.len();
//     return (s, length);
// }

fn calculate_len(s: &String) -> usize {
    let length = s.len();
    return length;
}

fn change(s: &mut String) {
    s.push_str(", world");
}

// ----- OWNERSHIP FUNCTIONS -----
fn takes_ownership(some_str: String) {
    println!("{}", some_str);
}

fn gives_ownership() -> String {
    let some_str = String::from ("hello");
    return some_str;
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
