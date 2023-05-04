// fn main() {
//     let x: i32 = 5;
//     println!("the value of x is: {}", x);

//     let x = "six";
//     println!("the value of x is: {}", x);

//     const COUNT: u32 = 100_000;
// }

fn main() {
    // compound types
    let tup = ("koenigsegg regera", 3_277_636);
    let (car, price) = tup;
    let car_price = tup.1;

    println!("{} = {}", car, price);
    println!("{}", car_price);

    let error_codes = [404, 401, 500];
    let not_found = error_codes[0];

    let sum = my_func(6, 9);
    println!("{}", sum);

    loop_fn();

    let a = [1, 2, 3, 4, 5, 6, 7];

    for i in a.iter() {
        println!("the value is: {}", i);
    }

    // range
    for num in 1..6 {
        println!("range val: {}", num);
    }
}

fn my_func(x: i32, y: i32) -> i32 {
    println!("my function");
    println!("{} {}", x, y);

    return x + y;
}

fn loop_fn() {
    let mut count = 0;

    let num = loop {
        count += 1;
        
        if (count == 10) {
            break count;
        }
    };
}
