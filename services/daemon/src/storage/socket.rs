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
        let req:crate::struct_cl_to_ser::CheckEmail =
        serde_json::from_str(&buffer).unwrap();
        let conn = Connection::open("data.db").unwrap();

        match repository::check_email(&conn,&req.email){
            Ok(Some(_user)) => {
                let response =crate::struct_ser_to_cl::CheckEmail{
                    action:"check_email".to_string(),
                    status:"exists".to_string(),
                    error:"".to_string(),
                };
                let json =
                serde_json::to_string(&response).unwrap();
                stream.write_all(json.as_bytes()).unwrap();
            }
            Ok(None)=>{
                let response =crate::struct_ser_to_cl::CheckEmail{
                    action:"check_email".to_string(),
                    status:"not_found".to_string(),
                    error:"".to_string(),
                };
                let json =
                serde_json::to_string(&response).unwrap();
                stream.write_all(json.as_bytes()).unwrap();
            }
            Err(e)=>{
                let response =crate::struct_ser_to_cl::CheckEmail{
                    action:"check_email".to_string(),
                    status:"error".to_string(),
                    error:e.to_string(),
                };
                let json =
                serde_json::to_string(&response).unwrap();
                stream.write_all(json.as_bytes()).unwrap();
            }
        }
    }

    "signup" => {
        let req:crate::struct_cl_to_ser::SignUp=
        serde_json::from_str(&buffer).unwrap();
        match repository::sign_up(&req.email,&req.username,&req.password,){
            Ok(_)=>{
                let response=crate::struct_ser_to_cl::SignUp{
                    action:"signup".to_string(),
                    status:"ok".to_string(),
                    error:"".to_string(),
                };
                let json =
                serde_json::to_string(&response).unwrap();
                if let Err(e) =stream.write_all(json.as_bytes()){println!("{}", e);}
            }
            Err(e)=>{
                let response=crate::struct_ser_to_cl::SignUp{
                    action:"signup".to_string(),
                    status:"error".to_string(),
                    error:e.to_string(),
                };
                let json =
                serde_json::to_string(&response).unwrap();
                stream.write_all(json.as_bytes()).unwrap();    
            }
        }
    }

    "login" => {
        let req: crate::struct_cl_to_ser::Login=
        serde_json::from_str(&buffer).unwrap();
        let conn =Connection::open("data.db").unwrap();

        match repository::check_email(&conn,&req.email){
            Ok(Some(user))=>{
                match repository::check_password(&user,&req.password){
                    Ok(true)=>{
                        repository::set_login(&conn,&user.user_id,).unwrap();
                        let response=crate::struct_ser_to_cl::Login{
                            action:"login".to_string(),
                            status:"ok".to_string(),
                            username:user.username.clone(),
                            error:"".to_string()
                        };
                        println!("login success");
                        let json =serde_json::to_string(&response).unwrap();
                        let _ =stream.write_all(json.as_bytes());
                    }
                    Ok(false)=>{
                        let response =
                        crate::struct_ser_to_cl::Login {
                        action: "login".to_string(),
                        status: "wrong_password".to_string(),
                        username: "".to_string(),
                        error: "".to_string(),
                    };
                        println!("wrong pass");
                        let json =serde_json::to_string(&response).unwrap();
                        let _ =stream.write_all(json.as_bytes());
                    }
                    Err(e)=>{
                        let response =crate::struct_ser_to_cl::Login {
                        action: "login".to_string(),
                        status: "error".to_string(),
                        username: "".to_string(),
                        error: e.to_string(),
                    };
                        println!("{}",e);
                        let json =serde_json::to_string(&response).unwrap();
                        let _ =stream.write_all(json.as_bytes());
                    }    
                }
            }
            Ok(None)=>{
                let response =
                crate::struct_ser_to_cl::Login {
                action: "login".to_string(),
                status: "email_not_found".to_string(),
                username: "".to_string(),
                error: "".to_string(),
                };
                println!("emial not found");
                let json =serde_json::to_string(&response).unwrap();
                let _ =stream.write_all(json.as_bytes());
            }
            Err(e)=>{
                let response =
                crate::struct_ser_to_cl::Login {
                action: "login".to_string(),
                status: "error".to_string(),
                username: "".to_string(),
                error: e.to_string(),
                };
                let json =serde_json::to_string(&response).unwrap();
                let _ =stream.write_all(json.as_bytes());
            }
        }
    }
    "check_login" => {
        println!("check login request");
        let conn = Connection::open("data.db").unwrap();

        match repository::get_logged_in_user(&conn) {
        Ok(Some(user)) => {
        let response =crate::struct_ser_to_cl::CheckLogin{
            action:"check_login".to_string(),
            status:"ok".to_string(),
            username: user.username,
            error:"".to_string(),
        };
        let json = serde_json::to_string(&response).unwrap();
        stream.write_all(json.as_bytes()).unwrap();
        }

        Ok(None) => {
        let response=crate::struct_ser_to_cl::CheckLogin{
            action:"check_login".to_string(),
            status:"bad".to_string(),
            username:"".to_string(),
            error:"".to_string(),
        };
        let json = serde_json::to_string(&response).unwrap();
        stream.write_all(json.as_bytes()).unwrap();
        }

        Err(e) => {
        let response=crate::struct_ser_to_cl::CheckLogin{
            action:"check_login".to_string(),
            status:"error".to_string(),
            username:"".to_string(),
            error:e.to_string(),
        };
        let json = serde_json::to_string(&response).unwrap();
        stream.write_all(json.as_bytes()).unwrap();
        }
    }
    }
    _ => {
        println!("unknown action");
    }
}
}

