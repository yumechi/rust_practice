struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 構造体は derive を入れないと print できない
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// メソッドを実装する場合は別に切り出す
// impl = implementation, 実装
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // これを呼び出すときは Rectangle::square(4); とか
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

// implは分割できる
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


fn main() {
    let user1 = User {
        email: String::from("oneone@example.com"),
        username: String::from("わんわんお （∪＾ω＾）"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("nyannyan@example.com"),
        username: String::from("にゃんにゃんお （∪・ω・）"),
        active: true,
        sign_in_count: 2,
    };
    user2.email = String::from("nyannyan+changed@example.com");
    println!("{}", user2.email);

    let mut user3 = User {
        email: String::from("nyannyan@example.com"),
        username: String::from("にゃんにゃんお （∪・ω・）"),
        ..user2
    };


    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let rect1 = Rectangle { width: 30, height: 50 };
    println!("面積: {}", area(&rect1));
    println!("面積: {}", rect1.area());

    // 一行で出す場合
    println!("rect1 is {:?}", rect1);

    // 全体的に出す場合
    println!("rect1 is {:#?}", rect1);

    // 包含されるかどうかを判断するメソッドを作ってみよう
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq1 = Rectangle::square(4);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}