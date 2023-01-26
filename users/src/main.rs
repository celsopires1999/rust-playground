mod init;
mod utils;

use std::{collections::HashMap, sync::Arc};

use init::{find_user, initialize_users};
use utils::find_user_by_email;

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

    // #region:     find by e-mail and password
    let body = Body {
        email: "2@my.com".to_string(),
        pw: "123".to_string(),
    };
    let user_result = find_user(users.clone(), body);

    match user_result {
        Ok(user) => println!("{}", user.email),
        Err(e) => println!("Error: {}", e),
    };
    // #endregion:  find by e-mail and password

    // #region:     find by e-mail
    let user_result = find_user_by_email((users.clone(), "1@my.com"));
    match user_result {
        Ok(user) => println!("{}", user.email),
        Err(e) => println!("Error: {}", e),
    };
    // #endregion:  find by e-mail
}
