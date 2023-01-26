use crate::{init::Error, User, Users};

pub fn find_user_by_email((users, email): (Users, impl Into<String>)) -> Result<User, Error> {
    print_message("working with my message");
    let email = email.into();
    let option_user = users.iter().find(|(_id, user)| user.email == email);
    match option_user {
        Some((_id, user)) => Ok(user.clone()),
        None => Err(Error::NotFound),
    }
}

// this fun will not be visible in main fun
fn print_message(message: impl Into<String>) {
    let message = message.into();
    println!("message received: {}", message);
}
