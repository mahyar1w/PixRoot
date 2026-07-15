use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct BaseRequest {
    pub action: String,
}

#[derive(Deserialize)]
pub struct CheckLogin {
    pub action: String,
}

#[derive(Serialize, Deserialize)]
pub struct CheckEmail{
    pub action: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignUp{
    pub action: String,
    pub email: String,
    pub username: String, 
    pub password:String
}

#[derive(Serialize, Deserialize)]
pub struct Login{
    pub action:String,
    pub email:String,
    pub password:String
}

#[derive(Serialize, Deserialize)]
pub struct Warning{
    pub action: String,
    pub warning: bool
} 

#[derive(Serialize, Deserialize)]
pub struct BlockApp{
    pub action: String,
    pub block_app: String
} 