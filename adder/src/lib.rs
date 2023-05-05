#[derive(Debug)]
struct Rectangle {
    length: u32,
    breadth: u32 
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.breadth > other.breadth && self.length > other.length
    } 
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1.");
        } else if value > 100 {
            panic!("Guess value must be lesser than or equal to 100.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 60,
            breadth: 9,
        };

        let smaller = Rectangle {
            length: 40,
            breadth: 2,
        };

        assert!(larger.can_hold(&smaller));
    }

     #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            length: 60,
            breadth: 9,
        };

        let smaller = Rectangle {
            length: 40,
            breadth: 2,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(420), 422);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Dubba");
        assert!(
            result.contains("Dubba"),
            "Greeting did not contain the name. Value was: `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be lesser than or equal to 100.")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(()) 
        } else {
            Err(String::from("two plus two does not equal 4"))
        } 
    }
}
