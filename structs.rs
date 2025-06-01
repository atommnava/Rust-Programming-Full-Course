

fn main()
{
    struct Book {
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
        sign_in_count: 1,
    }
    
    let mut user1: User = User {
        active: true,
        username: String::from("atom"),
        email: String::from("atom@gmail.com"),
        sign_in_count: 1,
    }
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            email,
            username,
            sign_in_count: 1,
        }
    }
    let user2: User = User {
        email: String::from("Another@ggmail.com")
        ..user1
    }
    // Tuple Structs
    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);

    let black: Color = Color(0,0,0);
    let white: Color = Color(255,255,255);

    // Unit-like struct
    struct AlwaysEqual;
    let subject: AlwaysEqual = AlwaysEqual;
} 