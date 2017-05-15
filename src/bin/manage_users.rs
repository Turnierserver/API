extern crate turnierserver;
extern crate diesel;
extern crate rustyline;
extern crate chrono;

use self::turnierserver::*;
use self::turnierserver::models::*;
use self::diesel::prelude::*;
use chrono::offset::utc::UTC;

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
            Err(_) => ::std::process::exit(1),
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
            Err(_) => ::std::process::exit(1),
        }
    }
}

fn main() {
    use turnierserver::schema::users;
    use turnierserver::schema::users::dsl::*;
    use turnierserver::schema::ais;
    use turnierserver::schema::gametypes::dsl::gametypes;
    use turnierserver::schema::games;

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
        let new_user = insert::User {
            username: &*text("Username", "admin".into()),
            email: &*text("E-Mail", "admin@ad.min".into()),
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

        if yesno("set password?", false) {
            user.set_pass(&*text("=> password?", "foobar".into()), &connection).unwrap();
        }
    }

    while yesno("Insert Ai?", false) {
        let user = users
            .filter(username.eq(text("Username", "admin".into())))
            .first::<User>(&connection)
            .unwrap();
        let gametype = gametypes
            .first::<GameType>(&connection)
            .unwrap();
        let new_ai = insert::Ai {
            user_id: user.id,
            name: &*text("Name", "HAL-9000".into()),
            description: None,
            gametype_id: gametype.id,
        };
        let ai = diesel::insert(&new_ai).into(ais::table)
            .get_result::<Ai>(&connection)
            .unwrap();
        println!("{:#?}", ai);
    }

    while yesno("Insert Game?", false) {
        let gametype = gametypes
            .first::<GameType>(&connection)
            .unwrap();
        let new_game = insert::Game {
            gametype_id: gametype.id,
            timestamp: UTC::now(),
        };
        let game = diesel::insert(&new_game).into(games::table)
            .get_result::<Game>(&connection)
            .unwrap();
        println!("{:#?}", game);
    }
}