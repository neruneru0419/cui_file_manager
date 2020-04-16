use ws::{listen, connect, CloseCode};
use std::fs::File;
use std::io::{Read, Write, stdin};

fn main() {
    println!("使用するモードを選んでください");
    println!("1->送信モード 2->受信モード");
    let mode = input();
    if mode == "1"{
        start_client();
    }else if mode == "2" {
        start_server();
    }
}

fn start_client(){
    println!("接続するサーバーのIPアドレスを入力して下さい");
    let ip = input();
    println!("ポートを入力して下さい");
    let port = input();
    connect(format!("ws://{}:{}", ip, port), |out| {
        println!("送信するファイル名を入力してください");
        let file_name: String = input();
        let file_binary = get_file_binary(file_name.clone());
        let file_data = vec![file_name, format!("{:?}", file_binary)];
        println!("ファイルを送信しています...");
        out.send(format!("{:?}",file_data)).unwrap(); 
        move |msg| {
            println!("{}", msg);
            out.close(CloseCode::Normal)
        }
    }).unwrap()

}

fn start_server(){

    println!("サーバーのIPアドレスを入力して下さい");
    let ip = input();
    println!("ポートを入力して下さい");
    let port = input();
    println!("ファイルを受信しています...\n受信を終了する場合はctrl+cを押して下さい");
    listen(format!("{}:{}", ip, port), |out| {
        move |_msg| {
            let file_data = parse_file_data(format!("{}", _msg));
            let file_name = format!("{}", file_data[0]);
            let file_binary = string_to_vec(format!("{}", file_data[1]));
            let mut file = File::create(file_name).unwrap();
            file.write_all(&file_binary).unwrap();
            file.flush().unwrap();
            println!("ファイルの受信が完了しました。");
            out.send("ファイルの送信が完了しました。")
        }
    }).unwrap()
}

fn string_to_vec(mut file_str: String) -> Vec<u8>{
    file_str.retain(|c| c != ']');
    file_str.retain(|c| c != '[');
    file_str.retain(|c| c != ' ');
    let file_vec_str: Vec<&str> = file_str.split(',').collect();
    let mut file_vec: Vec<u8> = Vec::new();
    for i in file_vec_str{
        file_vec.push(i.parse::<u8>().unwrap());
    }

    file_vec
}

fn get_file_binary(filename: String) -> Vec<u8>{
    let mut f = File::open(filename).expect("file dose not exist");
    let mut file_binary = Vec::new();
    f.read_to_end(&mut file_binary)
        .expect("failed to read file");

    file_binary
}

fn input() -> String{
    let mut input_str: String = String::new();
    stdin().read_line(&mut input_str)
        .expect("Failed to read line");
    input_str.retain(|chr| chr != '\n');
    input_str
}

fn parse_file_data(msg: String) -> Vec<String>{
    let mut file_name = String::new();
    let mut file_binary = String::new();
    let mut has_whitespace = false;
    for msg_char in msg.chars(){
        if msg_char == ' ' {has_whitespace = true};
        
        if has_whitespace{
            file_binary += &msg_char.to_string();
        }else{
            file_name += &msg_char.to_string();
        }
    }
    file_name = format!("{}", &file_name[2..(file_name.chars().count() - 2)]);
    file_binary = format!("{}", &file_binary[2..(file_binary.chars().count() - 2)]);
    let file_data: Vec<String> = vec![file_name, file_binary]; 
    
    file_data
}
