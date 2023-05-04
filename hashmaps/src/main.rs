use std::collections::HashMap;

fn main() {
    let blue = String::from("blue");
    let yellow = String::from("yellow");
    
    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("blue");
    let score = scores.get(&team_name);
}
