use crate::models::user_model::User;

pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        UserService
    }

    pub fn get_all_users(&self) -> Vec<User> {
        vec![User { id: 1, name: "John Doe".to_string(), email: "john.doe@example.com".to_string() }]
    }

    pub fn create_user(&self, user: User) -> User {
        user
    }

    pub fn update_user(&self, id: u32, user: User) -> User {
        User { id, ..user }
    }

    pub fn delete_user(&self, id: u32) {
        println!("User with id {} deleted", id);
    }
}
