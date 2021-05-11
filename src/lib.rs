pub const PORT: &str = "hello";

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub name: String,
}

pub struct Message {
    sender: User,
    text: String,
    time: u64,
}
