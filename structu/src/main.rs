
struct User
{
    username: String,
    sign_in_count: u64,
    email: String,
    active: bool,
}

fn main()
{
    let user1 = User
    {
        email: String::from("5435@163.com"),
        username: String::from("Then"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User
    {
        email: String::from("5435@163.com"),
        username: String::from("TK"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("5555@163.com");

//    println!("u1.email = {}, u1.name = {}, u2.email = {}", user1.email, user1.username, user2.email);

    /*
    let user3 = build_user("qq@qq.com", "Tn");
    println!("u3.email = {}, u3.name = {}", user3.email, user3.username);
    */

    let user4 = User
    {
        email: String::from("123@163.com"),
        username: String::from("Tw"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user5 = User
    {
        email: String::from("u4@163.com"),
        username: String::from("U4e"),
        ..user1
    };

    println!("user4.active = {}, user5.sign_in_count = {}", user4.active, user5.sign_in_count);

}


fn build_user(email: String, username: String) -> User
{
    
    /*
        //complex     
    User
    {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
    */

    /*
     * simple
     */
    User
    {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

