use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct BaseRequest {
    pub action: String,
}

#[derive(Serialize, Deserialize)]
pub struct CheckLoginPayload {
    pub status: String,
    pub username: String,
    pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct CheckLogin {
    pub action: String,
    pub payload: CheckLoginPayload,
}

#[derive(Serialize, Deserialize)]
pub struct CheckEmailPayload {
    pub status: String,
    pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct CheckEmail {
    pub action: String,
    pub payload: CheckEmailPayload,
}

#[derive(Serialize, Deserialize)]
pub struct SignUpPayload {
    pub status: String,
    pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignUp {
    pub action: String,
    pub payload: SignUpPayload,
}

#[derive(Serialize, Deserialize)]
pub struct LoginPayload {
    pub status: String,
    pub username: String,
    pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    pub action: String,
    pub payload: LoginPayload,
}

#[derive(Serialize, Deserialize)]
pub struct TotalDayTimePayload {
    pub status: String,
    pub total_focus: i32,
    pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct TotalDayTime {
    pub event: String,
    pub payload: TotalDayTimePayload,
}

#[derive(Serialize, Deserialize)]
pub struct SetWarningPayload {
    pub status: String,
    pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct SetWarning {
    pub action: String,
    pub payload: SetWarningPayload,
}

#[derive(Serialize, Deserialize)]
pub struct GetWarningPayload {
    pub warning: bool,
}

#[derive(Serialize, Deserialize)]
pub struct GetWarning {
    pub action: String,
    pub payload: GetWarningPayload,
}

#[derive(Serialize, Deserialize)]
pub struct SetBlockedAppPayload {
    pub status: String,
    pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct SetBlockedApp {
    pub action: String,
    pub payload: SetBlockedAppPayload,
}

#[derive(Serialize, Deserialize)]
pub struct GetBlockedAppPayload {
    pub blocked_apps: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetBlockedApp {
    pub action: String,
    pub payload: GetBlockedAppPayload,
}

#[derive(Serialize, Deserialize)]
pub struct SaveSessionPayload {
    pub status: String,
    pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct SaveSession {
    pub action: String,
    pub payload: SaveSessionPayload,
}

#[derive(Serialize, Deserialize)]
pub struct LogOutPayload {
    pub status: String,
    pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct LogOut {
    pub action: String,
    pub payload: LogOutPayload,
}

#[derive(Serialize, Deserialize)]
pub struct StartSessionPayload {
    pub status: String,
    pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct StartSession {
    pub action: String,
    pub payload: StartSessionPayload,
}

#[derive(Serialize, Deserialize)]
pub struct EndSessionPayload {
    pub status: String,
    pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct EndSession {
    pub action: String,
    pub payload: EndSessionPayload,
}

#[derive(Serialize, Deserialize)]
pub struct AllAppsPayload {
    pub status: String,
    pub all_apps: Vec<String>,
    pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct AllApps {
    pub action: String,
    pub payload: AllAppsPayload,
}

#[derive(Serialize, Deserialize)]
pub struct InterruptPayload {
    pub warning: bool,
    pub stage: String,
}

#[derive(Serialize, Deserialize)]
pub struct Interrupt {
    pub event: String,
    pub payload: InterruptPayload,
}