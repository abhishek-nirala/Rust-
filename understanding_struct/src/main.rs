#[derive(Debug)]
struct User {
    id: u64,
    username: String,
    email: String,
    password: String,
    is_verified: bool,
}

struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    //this is called method.
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other_rects: &Rectangle) -> bool {
        //self.area() > (rects.height * rects.width)    //more expensive.

        self.width > other_rects.width && self.height > other_rects.height
    }
}

fn main() {
    println!("Start!");
    let user1 = User {
        id: 1,
        username: String::from("tofu"),
        email: String::from("tofu@email.com"),
        password: String::from("@random123"),
        is_verified: true,
    };
    println!("User : {user1:#?}");

    let user2 = User {
        email: String::from("tofu-tikki@mail.com"),
        ..user1
    };

    //println!("user : {}",user1.username);

    let rect1 = Rectangle {
        height: 50,
        width: 30,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("Area of Rectangle is {}", rect1.area());

    println!("Finished!");
}

//fn area_of_rectangle(area: &Rectangle) -> u32 {
//    area.height * area.width
//}
