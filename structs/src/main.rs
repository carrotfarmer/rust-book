struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    length: i32,
    breadth: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        return self.length * self.breadth;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.length > other.length && self.breadth > other.breadth;
    }
}

impl Rectangle {
    fn square(size: i32) -> Rectangle {
        return Rectangle {
            length: size,
            breadth: size
        }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("dhruva@gmail.com"),
        username: String::from("carrotfarmer"),
        sign_in_count: 10,
        active: true,
    };

    let name = user1.username;
    user1.username = String::from("staticvoid");

    let user2 = build_user(
        String::from("dhruva2@gmail.com"),
        String::from("staticvoid176"),
    );

    let user3 = User {
        email: String::from("gmail@gmail.com"),
        username: String::from("gmail"),
        ..user2
    };

    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let rect1 = Rectangle {
        length: 5,
        breadth: 4,
    };

    let rect2 = Rectangle {
        length: 10,
        breadth: 5,
    };

    println!("{:#?}", &rect1);
    println!("{}", &rect1.area());
    println!("{}", &rect1.can_hold(&rect2));

    let sq = Rectangle::square(10);
    println!("{:#?}", &sq);
}

fn build_user(email: String, username: String) -> User {
    return User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    };
}
