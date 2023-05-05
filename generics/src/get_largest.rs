fn main() {
    let number_list = vec![1, 2, 3, 4, 5];

    let largest = get_largest(number_list);

    println!("{}", largest);

    let char_list: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f'];
    println!("{}", get_largest(char_list));
}

fn get_largest<T: PartialOrd + Copy>(vec: Vec<T>) -> T {
    let mut largest = vec[0];

    for num in vec {
        if num > largest {
            largest = num;
        }
    }

    return largest;
}
