struct User {
    username: String,
    email: String,
    sign_in_count: usize,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("somusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    struct AlwaysEqual;
}
