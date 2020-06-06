use crate::schema::*;
use crate::schemata;
use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub email: String,
    pub name: String,
    pub id: uuid::Uuid,
    #[cfg(test)]
    pub password: String,
    #[cfg(not(test))]
    password: String,
    cpf: String,
    phone: String,
}

impl From<schemata::User> for User {
    fn from(msg: schemata::User) -> Self {
        
        Self {
            email: msg.email,
            id: uuid::Uuid::new_v4(),
            cpf: msg.cpf,
            name: msg.name,
            phone: msg.telephone,
            password: msg.password,
        }
    }
}