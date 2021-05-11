#[macro_use]
extern crate serde_derive;

pub const PORT: &str = "hello";

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub sender: User,
    pub text: String,
    pub time: u64,
}
