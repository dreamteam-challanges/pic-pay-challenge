use crate::schemata as sch;
use crate::modelata as m;

pub fn user(user: sch::User) -> m::User {
    //hash password
    m::User::from(user)
}