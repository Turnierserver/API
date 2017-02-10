extern crate turnierserver;
extern crate diesel;
extern crate rustyline;

use self::turnierserver::*;
use self::turnierserver::models::*;
use self::diesel::prelude::*;

fn yesno(choice: &str, default: bool) -> bool {
    let mut rl = rustyline::Editor::<()>::new();
    let default_str = if default {
        "Y/n"
    } else {
        "y/N"
    };
    let s = format!("{} [{}] ", choice, default_str);
    loop {
        let readline = rl.readline(&*s);
        match readline {
            Ok(line) => {
                match &*line {
                    "" => return default,
                    "Y" | "y" => return true,
                    "N" | "n" => return false,
                    _ => println!("Invalid choice"),
                }
            },
            Err(_) => println!("TODO: abort"),
        }
    }
}

fn text(desc: &str, default: String) -> String {
    let mut rl = rustyline::Editor::<()>::new();
    let s = format!("{} [{}] ", desc, default);
    loop {
        let readline = rl.readline(&*s);
        match readline {
            Ok(line) => {
                if line.is_empty() {
                    return default;
                }
                return line;
            },
            Err(_) => println!("TODO: abort"),
        }
    }
}

fn main() {
    use turnierserver::schema::users;
    use turnierserver::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(10)
        .load::<User>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} users:", results.len());
    for user in &results {
        println!("{:?}", user);
    }
    println!("--------------------");

    while yesno("Insert user?", false) {
        let new_user = NewUser {
            username: &*text("Username", "admin".into()),
            email: &*text("E-Mail", "admin@ad.min".into()),
            pwhash: "pwhash",
            admin: yesno("Admin?", true)
        };
        let user = diesel::insert(&new_user).into(users::table)
            .get_result::<User>(&connection)
            .unwrap();
        println!("{:#?}", user);
    }

    while yesno("Edit users?", false) {
        let name = text("Username", "admin".into());
        let mut user = users
            .filter(username.eq(name))
            .first::<User>(&connection)
            .unwrap();
        
        user.username = text("=> new username:", user.username.clone());
        user.email = text("=> new email:", user.email.clone());
        user.admin = yesno("=> admin?", user.admin);
        user.save_changes::<User>(&connection).unwrap();
    }
}