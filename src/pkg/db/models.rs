use serde::Deserialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, Deserialize)]
pub struct User{
    pub id: String,
    pub name: String,
    pub pic_obj_name: Option<String> 
}


#[derive(FromRow, Debug, Deserialize)]
pub struct Group{
    pub created_by: String,
    pub id: String,
    pub name: String,
    pub member: String 
}


#[derive(FromRow, Debug, Deserialize)]
pub struct Chat{
    pub from: String,
    pub to: String,
    pub body: Vec<u8>
}


