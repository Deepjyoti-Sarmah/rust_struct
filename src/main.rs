struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}




fn main() {
    let mut user1 = User{
        email: String::from("rust@mail"),
        username: String::from("rust"),
        active: bool = true,
        sign_in_count: 1
    };


    let name = user1.username;
    user1.name = String::from("rustling");

    let user2: User = build_user(email: String::from("Deep@gmail"), username: String::from("deep"));

    let user3 = User {
        email: String::from("james@mail"),
        username: String::from("james123"),
        ..user2
    }
}

fn build_user(email: String, username: String) -> User {
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
