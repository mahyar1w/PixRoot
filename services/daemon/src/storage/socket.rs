use std::fs;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};

use rusqlite::Connection;

use crate::repository;
use crate::struct_ser_to_cl::BaseRequest;

pub fn start_socket_server() -> std::io::Result<()> {
    let socket_path = "/tmp/focus_app.sock";
    let _ = fs::remove_file(socket_path);
    let listener = UnixListener::bind(socket_path)?;
    println!("socket started");

    for stream in listener.incoming() {

        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_client(
    mut stream: UnixStream,
) {

    let mut buffer = String::new();
    stream.read_to_string(&mut buffer).unwrap();
    println!("received: {}", buffer);
    let request: BaseRequest =serde_json::from_str(&buffer).unwrap();
println!("action: {}", request.action);
match request.action.as_str() {

    "check_email" => {
    let req: crate::struct_cl_to_ser::CheckEmail =serde_json::from_str(&buffer).unwrap();
    let conn =Connection::open("data.db").unwrap();

    match repository::check_email(
        &conn,
        &req.payload.email,
    ) {

        Ok(Some(_user)) => {
            let response =
                crate::struct_ser_to_cl::CheckEmail {
                action: "check_email".to_string(),
                payload:
                    crate::struct_ser_to_cl::CheckEmailPayload {
                    status: "exists".to_string(),
                    error: "".to_string(),
                },
            };
            let json =serde_json::to_string(&response).unwrap();
            let _ =stream.write_all(json.as_bytes());
        }
        Ok(None) => {
            let response =
                crate::struct_ser_to_cl::CheckEmail {
                action: "check_email".to_string(),
                payload:
                    crate::struct_ser_to_cl::CheckEmailPayload {
                    status: "not_found".to_string(),
                    error: "".to_string(),
                },
            };
            let json =serde_json::to_string(&response).unwrap();
            let _ =stream.write_all(json.as_bytes());
        }
        Err(e) => {

            let response =
                crate::struct_ser_to_cl::CheckEmail {
                action: "check_email".to_string(),
                payload:
                    crate::struct_ser_to_cl::CheckEmailPayload {
                    status: "error".to_string(),
                    error: e.to_string(),
                },
            };
            let json =serde_json::to_string(&response).unwrap();
            let _ =stream.write_all(json.as_bytes());
        }
    }
}

    "signup" => {
    let req: crate::struct_cl_to_ser::SignUp =serde_json::from_str(&buffer).unwrap();
    match repository::sign_up(
        &req.payload.email,
        &req.payload.username,
        &req.payload.password,
    ) {
        Ok(_) => {
            let response =crate::struct_ser_to_cl::SignUp {
                action: "signup".to_string(),
                payload:
                    crate::struct_ser_to_cl::SignUpPayload {
                    status: "ok".to_string(),
                    error: "".to_string(),
                },
            };

            let json =serde_json::to_string(&response).unwrap();
            if let Err(e) =
                stream.write_all(json.as_bytes())
            {
                println!("{}", e);
            }
        }
        Err(e) => {
            let response =
                crate::struct_ser_to_cl::SignUp {
                action: "signup".to_string(),
                payload:
                    crate::struct_ser_to_cl::SignUpPayload {
                    status: "error".to_string(),
                    error: e.to_string(),
                },
            };
            let json =
                serde_json::to_string(&response).unwrap();
            if let Err(e) =
                stream.write_all(json.as_bytes())
            {
                println!("{}", e);
            }
        }
    }
}

    "login" => {
    let req: crate::struct_cl_to_ser::Login =serde_json::from_str(&buffer).unwrap();
    let conn =Connection::open("data.db").unwrap();
    match repository::check_email(
        &conn,
        &req.payload.email,
    ) {
        Ok(Some(user)) => {
            match repository::check_password(
                &user,
                &req.payload.password,
            ) {
                Ok(true) => {
                    repository::set_login(
                        &conn,
                        &user.user_id,
                    ).unwrap();
                    let response =crate::struct_ser_to_cl::Login {
                        action: "login".to_string(),
                        payload:
                            crate::struct_ser_to_cl::LoginPayload {
                            status: "ok".to_string(),
                            username: user.username.clone(),
                            error: "".to_string(),
                        },
                    };
                    println!("login success");
                    let json =serde_json::to_string(&response).unwrap();
                    let _ =stream.write_all(json.as_bytes());
                }
                Ok(false) => {
                    let response =crate::struct_ser_to_cl::Login {
                        action: "login".to_string(),
                        payload:
                            crate::struct_ser_to_cl::LoginPayload {
                            status: "wrong_password".to_string(),
                            username: "".to_string(),
                            error: "".to_string(),
                        },
                    };
                    println!("wrong pass");
                    let json =serde_json::to_string(&response).unwrap();
                    let _ =stream.write_all(json.as_bytes());
                }
                Err(e) => {
                    let response =
                        crate::struct_ser_to_cl::Login {
                        action: "login".to_string(),
                        payload:
                            crate::struct_ser_to_cl::LoginPayload {
                            status: "error".to_string(),
                            username: "".to_string(),
                            error: e.to_string(),
                        },
                    };
                    println!("{}", e);
                    let json =serde_json::to_string(&response).unwrap();
                    let _ =stream.write_all(json.as_bytes());
                }
            }
        }
        Ok(None) => {
            let response =crate::struct_ser_to_cl::Login {
                action: "login".to_string(),
                payload:
                    crate::struct_ser_to_cl::LoginPayload {
                    status: "email_not_found".to_string(),
                    username: "".to_string(),
                    error: "".to_string(),
                },
            };
            println!("email not found");
            let json =serde_json::to_string(&response).unwrap();
            let _ =stream.write_all(json.as_bytes());
        }
        Err(e) => {
            let response =crate::struct_ser_to_cl::Login {
                action: "login".to_string(),
                payload:
                    crate::struct_ser_to_cl::LoginPayload {
                    status: "error".to_string(),
                    username: "".to_string(),
                    error: e.to_string(),
                },
            };
            let json =serde_json::to_string(&response).unwrap();
            let _ =stream.write_all(json.as_bytes());
        }
    }
}
    "check_login" => {
    println!("check login request");
    let conn =Connection::open("data.db").unwrap();
    match repository::get_logged_in_user(&conn) {
        Ok(Some(user)) => {
            let response =
                crate::struct_ser_to_cl::CheckLogin {
                action: "check_login".to_string(),
                payload:
                    crate::struct_ser_to_cl::CheckLoginPayload {
                    status: "ok".to_string(),
                    username: user.username,
                    error: "".to_string(),
                },
            };
            let json =serde_json::to_string(&response).unwrap();
            let _ =
                stream.write_all(json.as_bytes());
        }
        Ok(None) => {
            let response =
                crate::struct_ser_to_cl::CheckLogin {
                action: "check_login".to_string(),
                payload:
                    crate::struct_ser_to_cl::CheckLoginPayload {
                    status: "bad".to_string(),
                    username: "".to_string(),
                    error: "".to_string(),
                },
            };
            let json =serde_json::to_string(&response).unwrap();
            let _ =stream.write_all(json.as_bytes());
        }
        Err(e) => {
            let response =
                crate::struct_ser_to_cl::CheckLogin {
                action: "check_login".to_string(),
                payload:
                    crate::struct_ser_to_cl::CheckLoginPayload {
                    status: "error".to_string(),
                    username: "".to_string(),
                    error: e.to_string(),
                },
            };
            let json =serde_json::to_string(&response).unwrap();
            let _ =stream.write_all(json.as_bytes());
        }
    }
}

    "save_session" => {
    let req: crate::struct_cl_to_ser::SaveSession =serde_json::from_str(&buffer).unwrap();
    let conn =Connection::open("data.db").unwrap();

    match repository::get_logged_in_user(&conn) {
        Ok(Some(user)) => {
            match repository::register_attempt(
                &conn,
                &user.user_id,
                req.payload.success,
                req.payload.time,
            ) {
                Ok(_) => {
                    let response =
                        crate::struct_ser_to_cl::SaveSession {
                        action: "save_session".to_string(),
                        payload:
                            crate::struct_ser_to_cl::SaveSessionPayload {
                            status: "ok".to_string(),
                            error: "".to_string(),
                        },
                    };
                    let json =serde_json::to_string(&response).unwrap();
                    let _ =stream.write_all(json.as_bytes());
                }
                Err(e) => {
                    let response =crate::struct_ser_to_cl::SaveSession {
                        action: "save_session".to_string(),
                        payload:
                            crate::struct_ser_to_cl::SaveSessionPayload {
                            status: "error".to_string(),
                            error: e.to_string(),
                        },
                    };
                    let json =serde_json::to_string(&response).unwrap();
                    let _ =stream.write_all(json.as_bytes());
                }
            }
        }

        Ok(None) => {
            let response =crate::struct_ser_to_cl::SaveSession {
                action: "save_session".to_string(),
                payload:
                    crate::struct_ser_to_cl::SaveSessionPayload {
                    status: "no acount logined".to_string(),
                    error: "".to_string(),
                    },
            };

            let json =
                serde_json::to_string(&response).unwrap();
            let _ =stream.write_all(json.as_bytes());
        }

        Err(e) => {
            let response =crate::struct_ser_to_cl::SaveSession {
                action: "save_session".to_string(),
                payload:
                    crate::struct_ser_to_cl::SaveSessionPayload {
                    status: "error".to_string(),
                    error: e.to_string(),
                },
            };
            let json =serde_json::to_string(&response).unwrap();
            let _ =stream.write_all(json.as_bytes());
        }
    }
}
    _ => {
        println!("unknown action");
    }
}
}

