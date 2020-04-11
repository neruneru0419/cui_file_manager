#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

use std::fs::File;
use std::io::{Read, Write};
use std::io::stdin;
use std::collections::HashMap;
use rocket_contrib::json::Json;
use rocket::post;
use rocket::routes;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct FileData {
    file: String,
    file_name: String
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("使用するモードを選んでください");
    println!("1->送信モード 2->受信モード");
    let mut mode = String::new();
    stdin().read_line(&mut mode)
        .expect("Failed to read line");
    if mode == "1\n"{
        start_client().await
            .unwrap();
    }else if mode == "2\n" {
        start_server();
    }
    Ok(())
}

fn string_to_vec(s: String) -> Vec<u8>{
    let mut a = s;
    a.retain(|c| c != ']');
    a.retain(|c| c != '[');
    a.retain(|c| c != ' ');
    let file_vec_str: Vec<&str> = a.split(',').collect();
    let mut file_vec: Vec<u8> = Vec::new();
    for i in file_vec_str{
        file_vec.push(i.parse::<u8>().unwrap());
    }
    file_vec
}

#[post("/", data = "<file>")]
fn get_file(file: Json<FileData>){
    println!("ファイルを受信しました");
    let contents = string_to_vec(file.0.file);
    let mut file = File::create(file.0.file_name).unwrap();
        file.write_all(&contents).unwrap();
        file.flush().unwrap();
}


fn start_server() {
    println!("ファイルの受信待ちです\n終了する時はctrl+cして下さい");
    rocket::ignite()
        .mount("/", routes![get_file])
        .launch();
}

fn get_file_binary(filename: String) -> String{
    let mut f = File::open(filename).expect("file dose not exist");
    let mut file_binary = Vec::new();
    f.read_to_end(&mut file_binary)
        .expect("failed to read file");
    format!("{:?}", file_binary)
}

async fn start_client() -> Result<(), Box<dyn std::error::Error>>{
    let client = reqwest::Client::new();
    let url: &str = "http://localhost:8000";

    let mut file_name: String = String::new();
    println!("送信するファイルのパスを入力してください");
    stdin().read_line(&mut file_name)
        .expect("Failed to read line");
    file_name.retain(|chr| chr != '\n');
    let file_binary = get_file_binary(file_name.clone());


    let mut param = HashMap::new();
    param.insert("file_name", &file_name);
    param.insert("file", &file_binary);

    let _res = client.post(url)
        .json(&param)
        .send()
        .await
        .unwrap();    
    println!("送信が完了しました");
    Ok(())
}