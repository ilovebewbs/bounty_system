use crate::schema::contracts;
use crate::schema::members;
#[derive(Debug,Queryable)]
pub struct Contract {
    pub id: i32,
    pub the_owner: String,
    pub the_target: String,
    pub bounty: i32,
    pub finished: bool,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "contracts"]
pub struct NewContract<'a> {
    pub the_owner: &'a str,
    pub the_target: &'a str,
    pub bounty: i32,
}

#[derive(Debug,Queryable)]
pub struct Member {
    pub id: i32,
    pub fullname: String,
    pub city: String,
    pub phone: i64,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "members"]
pub struct NewMember<'a> {
    pub fullname: &'a str,
    pub city: &'a str,
    pub phone: i64,
}
