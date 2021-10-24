pub(crate) fn function_test() {
    test_1();
    test_2();
    test_3();
}

fn test_1() {
    let ret = test_func(10);
    println!("The return value of test_func is: {}", ret);
}

fn test_func(x: i32) -> i32 {
    println!("The value of x: {}", x);

    let x = 9;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of x is: {}, of y is: {}", x, y);
    // return 777;
    666
}

fn test_2() {
    let fahrenheit = 90.0;
    let celsius = temperature_converter(fahrenheit);
    println!(
        "The fahrenheit is: {}, the celsius is: {}",
        fahrenheit, celsius
    );
}

fn temperature_converter(fahrenheit: f64) -> f64 {
    println!("temperature converter");
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn test_3() {
    for num in 1..11 {
        println!("The {}th number in Fibonacci is: {}", num, fibonacci(num));
    }
}

fn fibonacci(num: i8) -> i8 {
    // if num == 1 || num == 2 {
    //     return 1;
    // }
    // fibonacci(num - 1) + fibonacci(num - 2)
    if num <= 2 {
        1
    } else {
        fibonacci(num - 1) + fibonacci(num - 2)
    }
}
