struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub(crate) fn structs_test() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("some_username"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("another@example.com");

    let user2 = build_user(String::from("test@example.com"), String::from("test"));
    println!("email: {}, username: {}, sing_in_count: {}, activate: {}",
             user2.email, user2.username, user2.sign_in_count, user2.active);

    let user3 = update_struct(user1, String::from("update@example.com"),
                              String::from("update"));
    println!("email: {}, username: {}, sing_in_count: {}, activate: {}",
             user3.email, user3.username, user3.sign_in_count, user3.active);

    tuple_structs();

    struct UnitStruct {}
    let unit_struct = UnitStruct {};

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:#?}", rect1);
    println!("rect1 is {:?} in another format", rect1);

    println!("The area of the rectangle is {}", rect1.area());

    let rect2 = Rectangle {
        width: 40,
        height: 60,
    };

    let rect3 = Rectangle {
        width: 10,
        height: 30,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(30);
    println!("sq is {:#?}", sq);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 如果要在函数中改变参数的值，入参改为&mut self
    // 如果需要把self转为其他内容，并希望阻止其调用函数保留原值，这时才改为去掉引用的self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 其他参数必须加到self之后
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 参数列表中可以没有self，这种称为associated function，不是method。通常用来构造一个新的实例。
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: false,
        sign_in_count: 2,
    }
}

fn update_struct(user1: User, email: String, username: String) -> User {
    User {
        email,
        username,
        ..user1
    }
}

fn tuple_structs() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(1, 2, 3);
    let origin = Point(1, 2, 3);

    // 只能传入一个struct Color的变量
    fn test_tuple_structs(tuple_structs: Color) {
        // let (x, y, z) = tuple_structs; // error: expected struct `Color`, found tuple
        println!("The second value in tuple structs is: {}", tuple_structs.1);
    }

    test_tuple_structs(black);
    // test_tuple_structs(origin); // error: expected struct `Color`, found struct `Point`
}

// struct UserRef {
//     username: &str,
//     sign_in_count: u64,
//     active: bool,
// }
