fn main() {
    // {
    //     let s = "hello";
    // }
    // println!(s);

    // let mut s = String::from("hello");
    // s.push_str(", world!");

    // println!("{}", s);

    // let x = 5;
    // let y = x;

    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // print!("{}", s1);

    let s = String::from("hello");
    let mut s1 = takes_ownership(takes_and_gives_back(s));

    println!("{}", s1);

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);

    println!("{}", s1);

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let x = 5;
    makes_copy(x);

    let s2 = String::from("hello world");

    println!("{}", first_word(&s2));

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    return some_string;
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();

    length
}

fn change(some_string: &mut String) {
    some_string.push_str(", world")
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
