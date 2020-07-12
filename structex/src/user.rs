fn main()
{
    test1();
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn test1()
{
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn te1()
{
    let user1 = User {
        email: String::from("vcvckw@gmail.com"),
        username: String::from("Then"),
        active: true,
        sign_in_count: 1,
    };
    println!("user1.name = {}", user1.username);

    let user3 = User {
        email: String::from("nk@163.com"),
        username: String::from("vcvc"),
        ..user1
    };
}

fn te2() 
{
    let mut user2 = User {
        email: String::from("vcvckw@gmail.com"),
        username: String::from("Then"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("12345@163.com");
}


//one
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

//two simple
fn build_user2(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
