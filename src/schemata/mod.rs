use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String, 
    cpf: String, 
    telephone: String,
    email: String,
    password: String,
}