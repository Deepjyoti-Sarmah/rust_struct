// struct User{
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle{
    fn area(&self ) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle{
    fn square(size: u32) -> Rectangle {
        Rectangle{
            width: size,
            height: size
        }
    } 

    // fn square_area(size: u32) -> u32 {
    //     Rectangle{
    //         width: size,
    //         height: size
    //     };

    //     Rectangle.width * Rectangle.height
    // } 
}


fn main() {
    // let mut user1 = User{
    //     email: String::from("rust@mail"),
    //     username: String::from("rust"),
    //     active: bool = true,
    //     sign_in_count: 1
    // };


    // let name = user1.username;
    // user1.name = String::from("rustling");

    // let user2: User = build_user(email: String::from("Deep@gmail"), username: String::from("deep"));

    // let user3 = User {
    //     email: String::from("james@mail"),
    //     username: String::from("james123"),
    //     ..user2
    // }

    // struct Color(i32, i32, i32);
    // struct Point(i32, i32, i32);

    // let width1: u32 = 30;
    // let height1: u32 = 50;

    // let rect: (u32, u32) = (30, 50);
    let rect: Rectangle = Rectangle{
        width: 30,
        height: 50
    };

    let rect1 = Rectangle{
        width: 20,
        height: 30
    };

    let rect2 = Rectangle{
        width: 40,
        height: 50
    };

    let rect3 = Rectangle::square(24);
    
    // let rect4 = Rectangle::square_area(24);



    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    println!("rect2 can hold rect1: {}", rect2.can_hold(&rect1));

    

    println!("rect: {:#?}", rect);
    

    println!("The area of the rectangle is {} square pixels", rect.area());

    println!("The area of the square is {:#?} square pixels", rect3);

    // println!("The area of the square is {:#?} square pixels", rect4);


}

// fn area(rectangle: &Rectangle) -> u32{
//     rectangle.width * rectangle.height
// }

// fn build_user(email: String, username: String) -> User {
//     User{
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }
