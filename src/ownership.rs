use std::any::{Any, TypeId};

pub(crate) fn ownership_test() {
    let s1 = "hello";
    if s1.type_id() == TypeId::of::<str>() {
        println!("s1 is str");
    }
    let mut s = String::from("hello");
    if s.type_id() == TypeId::of::<String>() {
        println!("s is string");
    }
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1);
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, success!", s1);

    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);

    let x = 5;
    make_copy(x);
    println!("{}", x);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    let r3 = &mut s;
    println!("{}", r3);

    // let reference_to_nothing = dangle();
    // let iii = test_dangle();

    let mut i = 10;
    let i2 = &mut i;
    let i1 = & i;

    println!("{}", i1);

    let mut s = String::from("hello world");
    // let word = first_word(&s);
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}, {}", hello, world);

    let arr = [10, 20, 30, 40, 50];
    let a = &arr[0..3];
    for i in a {
        println!("{}", i);
    }

    let hello = slice_first_word(&mut s);
    println!("{}", hello);

    let world = slice_second_word(&mut s);
    println!("{}", world);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn change(some_string: &mut String) {
    some_string.push_str(", change!");
    println!("{}", some_string);
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// fn test_dangle() -> &u16 {
//     let i = 5;
//     &i
// }

fn first_word(s:& String) -> usize {
    let bytes = s.as_bytes();

    for (i, &iterm) in bytes.iter().enumerate() {
        if iterm == b' ' {
            return i;
        }
    }
    s.len()
}

fn slice_first_word(s: &mut String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn slice_second_word(s: &mut String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i+1..];
        }
    }
    &s[..]
}
