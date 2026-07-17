use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EmptyPayload {}

#[derive(Deserialize)]
pub struct BaseRequest {
    pub action: String,
}

#[derive(Serialize, Deserialize)]
pub struct CheckEmailPayload {
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct CheckEmail {
    pub action: String,
    pub payload: CheckEmailPayload,
}

#[derive(Serialize, Deserialize)]
pub struct SignUpPayload {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignUp {
    pub action: String,
    pub payload: SignUpPayload,
}

#[derive(Serialize, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    pub action: String,
    pub payload: LoginPayload,
}

#[derive(Serialize, Deserialize)]
pub struct SetWarningPayload {
    pub warning: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SetWarning {
    pub action: String,
    pub payload: SetWarningPayload,
}

#[derive(Serialize, Deserialize)]
pub struct GetWarning {
    pub action: String,
    pub payload: EmptyPayload,
}

#[derive(Serialize, Deserialize)]
pub struct SetBlockedAppPayload {
    pub blocked_apps: String,
}

#[derive(Serialize, Deserialize)]
pub struct SetBlockedApp {
    pub action: String,
    pub payload: SetBlockedAppPayload,
}

#[derive(Serialize, Deserialize)]
pub struct GetBlockedApp {
    pub action: String,
    pub payload: EmptyPayload,
}

#[derive(Serialize, Deserialize)]
pub struct CheckLogin {
    pub action: String,
    pub payload: EmptyPayload,
}

#[derive(Serialize, Deserialize)]
pub struct SaveSessionPayload {
    pub success:bool,
    pub time:i32
}

#[derive(Serialize, Deserialize)]
pub struct SaveSession {
    pub action: String,
    pub payload: SaveSessionPayload,
}

#[derive(Serialize, Deserialize)]
pub struct StartSession {
    pub action: String,
    pub payload: EmptyPayload,
}

#[derive(Serialize, Deserialize)]
pub struct EndSessionPayload {
    pub duration: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct EndSession {
    pub action: String,
    pub payload: EndSessionPayload,
}

#[derive(Serialize, Deserialize)]
pub struct AllApps {
    pub action: String,
    pub payload: EmptyPayload,
}

#[derive(Serialize, Deserialize)]
pub struct LogOut {
    pub action: String,
    pub payload: EmptyPayload,
}
