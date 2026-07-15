use serde::{Serialize, Deserialize};
#[derive(Deserialize)]
pub struct BaseRequest {
    pub action: String,
}

#[derive(Serialize)]
pub struct CheckLogin {
    pub action: String,
    pub status: String,
    pub username: String,
    pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct CheckEmail{
    pub action: String,
    pub status: String,
    pub error: String
}

#[derive(Serialize, Deserialize)]
pub struct SignUp{
    pub action: String,
    pub status: String,
    pub error: String
}

#[derive(Serialize, Deserialize)]
pub struct Login{
    pub action:String,
    pub status: String,
    pub username: String,
    pub error: String
}

#[derive(Serialize, Deserialize)]
pub struct TotalDayTime{
    pub action:String,
    pub status:String,
    pub total_focus: i32,
    pub error:String
}

#[derive(Serialize, Deserialize)]
pub struct Warning{
    pub action: String,
    pub status:String,
    pub warning: bool,
    pub error: String
} 

#[derive(Serialize, Deserialize)]
pub struct BlockApp{
    pub action: String,
    pub status:String,
    pub block_app: String,
    pub error: String
} 