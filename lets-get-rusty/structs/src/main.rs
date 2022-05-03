struct User {
    username: String,
    email: String,
    signed_in_count: u64,
    active: bool,
}

#[derive(Debug)] //helps in printing -> {:?}
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    //method has self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //method has self
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }
}

// one struct can have any no of implementations
impl Rectangle {
    // square can also be written in above implementation block
    // below function is a Associated function. Above are mothds
    // way to diff b/w methods and associate function is : associate functions doesnt take &self
    // assocaited function can be called using :  Rectangle::square(30) ---> check below
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("abc@gmail.com"),
        username: String::from("krishna"),
        signed_in_count: 1,
        active: true,
    };

    let name = user1.username;
    user1.username = String::from("Saikrishna");

    let user2 = build_user(String::from("edf@gmail.com"), String::from("krishna"));

    let user3: User = User {
        email: String::from("abc@gmail.com"),
        username: String::from("krishna"),
        ..user2
    };

    let rect: Rectangle = Rectangle {
        height: 80,
        width: 40,
    };

    // #[derive(Debug)] helps in printing -> {:?}
    println!("RECT : {:#?}", rect);

    println!("Area is : {}", rect.area());

    let rect1: Rectangle = Rectangle {
        height: 80,
        width: 40,
    };
    let rect2: Rectangle = Rectangle {
        height: 70,
        width: 30,
    };

    println!("Can rect1 hold rec2? : {}", rect1.can_hold(&rect2));

    // way to call Associated function is :
    let square = Rectangle::square(30);
    println!("Square is : {:#?}", square);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        signed_in_count: 1,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
