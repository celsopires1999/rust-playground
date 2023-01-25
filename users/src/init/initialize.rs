use std::collections::HashMap;

use thiserror::Error;

use crate::{Body, User, Users};

#[derive(Error, Debug)]
pub enum Error {
    #[error("not found")]
    NotFound,
}

pub fn initialize_users() -> HashMap<String, User> {
    let mut map = HashMap::new();

    map.insert(
        "1".to_string(),
        User {
            uid: "1".to_string(),
            email: "1@my.com".to_string(),
            pw: "123".to_string(),
            role: "User".to_string(),
        },
    );

    map.insert(
        "2".to_string(),
        User {
            uid: "2".to_string(),
            email: "2@my.com".to_string(),
            pw: "321".to_string(),
            role: "Admin".to_string(),
        },
    );

    map
}

pub fn find_user(users: Users, body: Body) -> Result<User, Error> {
    let user = users
        .iter()
        .find(|(_uid, user)| user.email == body.email && user.pw == body.pw);

    match user {
        Some((_uid, user)) => Ok(user.clone()),
        None => Err(Error::NotFound),
    }
}
