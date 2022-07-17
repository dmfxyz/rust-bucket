#[derive(Debug)]
struct User {
    username: String,
    sign_in_acount: u64,
}

// tuple struct
struct Color(i32, i32, i32);

// unit struct
struct Nil;

fn main() {
    let user = User {
        username: String::from("dmfxyz"),
        sign_in_acount: 1,
    };
    println!("{:?}", user);

    let mut mutable_user = User {
        username: String::from("dmfxyz"),
        sign_in_acount: 1,
    };
    println!("{}", mutable_user.username);
    mutable_user.username = String::from("dmfxyz2");
    println!("{}", mutable_user.username);

    let user2 = build_user_shorthand(String::from("xyzdmf"));
    println!("{:?}", user2);

    let user3 = User {
        username: String::from("dmfxyz"),
        sign_in_acount: 100,
    };

    // Update creation (Note this moves user3 string (non copyable) variables!!)
    let user4 = User {
        username: String::from("xyzdmf"),
        ..user3
    };
    println!("{:?}", user4);
    let white = Color(255, 255, 255);
    let none = Nil;
}

fn build_user(username: String) -> User {
    User {
        username: username,
        sign_in_acount: 1,
    }
}

fn build_user_shorthand(username: String) -> User {
    User {
        username,
        sign_in_acount: 1,
    }
}