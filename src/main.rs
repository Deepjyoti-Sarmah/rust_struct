// struct User{
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32
// }

// impl Rectangle{
//     fn area(&self ) -> u32{
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
// //without using self
// impl Rectangle{
//     fn square(size: u32) -> Rectangle {
//         Rectangle{
//             width: size,
//             height: size
//         }
//     } 

    // fn square_area(size: u32) -> u32 {
    //     Rectangle{
    //         width: size,
    //         height: size
    //     };

    //     Rectangle.width * Rectangle.height
    // } 
// }


// fn main() {
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
    // let rect: Rectangle = Rectangle{
    //     width: 30,
    //     height: 50
    // };

    // let rect1 = Rectangle{
    //     width: 20,
    //     height: 30
    // };

    // let rect2 = Rectangle{
    //     width: 40,
    //     height: 50
    // };

    // let rect3 = Rectangle::square(24);
    
    // let rect4 = Rectangle::square_area(24);



    // println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    // println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    // println!("rect2 can hold rect1: {}", rect2.can_hold(&rect1));

    

    // println!("rect: {:#?}", rect);
    

    // println!("The area of the rectangle is {} square pixels", rect.area());

    // println!("The area of the square is {:#?} square pixels", rect3);

    // println!("The area of the square is {:#?} square pixels", rect4);


// }

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


//quize

// struct Point {

//     x: i32,
//     y: i32,
// }

// fn main() {
//     let mut a = Point { x: 1, y: 2 };
//     a.x += 1;  
//     let b = Point { y: 1, ..a };
//     a.x += 1;
//     println!("{}", b.x);
// }

// The ..a syntax is a shallow copy of each field, so the second a.x += 1 has no effect on b.

// fn main() {

//     let mut p = Point { x: 1, y: 2 };
//     let x = &mut p.x;
//     let y = &mut p.y;  
//     *x += 1;
//     *y += 1;

//     println!("{} {}", p.x, p.y);
// }

// Rust understands that .x refers to a different object than .y, so it is valid to take simultaneous mutable references to both fields.

//quize 
// #[derive(Debug)]

// struct Rectangle {
//   width: u32,
//   height: u32,
// }


// fn main() {
//   let rect1 = Rectangle {
//     width: 30,
//     height: 50,
//   };


//   let a = area(rect1);
//   println!("{} * {} = {}", rect1.width, rect1.height, a);
// }


// fn area(rectangle: Rectangle) -> u32 {

//   rectangle.width * rectangle.height

// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn set_width(&mut self, width: u32) {
//         self.width = width;
//     }
// }

// fn main() {
//     let mut r = Rectangle {
//         width: 1,
//         height: 2
//     };

//     let area1 = r.area();
//     let area2 = Rectangle::area(&r);
//     assert_eq!(area1, area2);

//     r.set_width(2);
//     Rectangle::set_width(&mut r, 2);

//     println!("area1 {} and area2 {}", area1, area2);
// }

//struct
// struct Point(i32, i32);

// fn main() {
//   let p = Point(1, 2);

//   impl p {
//     fn x(&self) -> i32 { self.0 }
//   }

//   println!("{}", p.x());
// }


// Methods and ownership

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {    
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn set_width(&mut self, width: u32) {
    self.width = width;
  }

  fn max(self, other: Self) -> Self {
    let w = self.width.max(other.width);
    let h = self.height.max(other.height);
    Rectangle { 
      width: w,
      height: h
    }
  }
}

fn main() {
  let rect = Rectangle {
    width: 0,
    height: 0
  };
  println!("{}", rect.area());

  let other_rect = Rectangle { width: 1, height: 1 };
  let max_rect = rect.max(other_rect);

  println!("{:?}", max_rect);
}