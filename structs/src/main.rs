struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    println!("Hello, world!");

    let user1 = User {
        active: true,
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        sign_in_count: 1
    };

    let user2 = build_user(String::from("alice@example.com"), String::from("alice"));

    // moving values from user1 to user3
    let user3 = User {
        sign_in_count: 2,
        .. user1
    };

    // println!("User email: {}", user1.email); this does not work, value is already moved to user3
    println!("User email: {}", user2.email);
    println!("User email: {}", user3.email);

    let black = Color(0, 0, 0);
    let origin = Point(0,0,0);
    print_color_point(black, origin);


}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        sign_in_count: 1,
        username,
        email
    }
}

fn print_color_point(color: Color, point: Point) {
    println!("color: {}.{}.{}", color.0, color.1, color.2);
    println!("point: {}.{}.{}", point.0, point.1, point.2);
}
