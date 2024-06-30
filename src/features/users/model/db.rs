use crate::features::users::controller::schema::User;

pub fn find_users() -> Vec<User> {
    let user_one = User {
        _id: String::from("1234"),
        username: String::from("luan"),
        password: String::from("luanpass")
    };
    let user_two = User {
        _id: String::from("4321"),
        username: String::from("maria"),
        password: String::from("mariapass")
    };
    
    vec![user_one, user_two]
}