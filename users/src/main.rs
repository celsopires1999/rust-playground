mod init;

use std::{collections::HashMap, sync::Arc};

use init::{find_user, initialize_users, Error};

type Users = Arc<HashMap<String, User>>;

pub struct Body {
    pub email: String,
    pub pw: String,
}

#[derive(Clone, Debug)]
pub struct User {
    pub uid: String,
    pub email: String,
    pub pw: String,
    pub role: String,
}

fn main() {
    let users = Arc::new(initialize_users());

    let body = Body {
        email: "2@my.com".to_string(),
        pw: "123".to_string(),
    };
    let user_result = find_user(users, body);

    match user_result {
        Ok(user) => println!("{}", user.email),
        Err(e) => println!("Error: {}", e),
    };
}
