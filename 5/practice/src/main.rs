#![warn(clippy::all, clippy::pedantic)]

struct User {
    is_active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        is_active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Point(i32, i32, i32);

fn main() {
    let user_a = &build_user(String::from("User A"), String::from("userA@example.com"));

    println!(
        "User A: {}, {}, {}, {}",
        user_a.username, user_a.email, user_a.is_active, user_a.sign_in_count
    );

    // Won't work! user_a is not mutable
    // user_a.email = String::from("ExampleAA@example.com");

    // Won't work! user_a is not mutable
    //let a_ref = &mut user_a;
    let b_ref = &user_a;
    let c_ref = &user_a;

    println!(
        "User A: {}, {}, {}, {}",
        c_ref.username, c_ref.email, c_ref.is_active, c_ref.sign_in_count
    );
    println!(
        "User A: {}, {}, {}, {}",
        b_ref.username, b_ref.email, b_ref.is_active, b_ref.sign_in_count
    );

    let point1 = Point(1, 2, 3);
    let Point(x, y, z) = point1;

    println!("point 1: {} - {} - {}", point1.0, point1.1, point1.2);
    println!("{x} - {y} - {z}");

}
