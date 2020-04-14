use ws::{listen, connect, CloseCode};
use std::fs::File;
use std::io::{Read, Write, stdin};

fn main() {
    println!("使用するモードを選んでください");
    println!("1->送信モード 2->受信モード");
    let mut mode = String::new();
    stdin().read_line(&mut mode)
        .expect("Failed to read line");
    if mode == "1\n"{
        start_client();
    }else if mode == "2\n" {
        start_server();
    }
}


fn start_server(){

    println!("ファイルの受信待ちです...");
    let ip = "127.0.0.1";
    let port = "8080";
    listen((format!("{}:{}", ip, port)), |out| {
        println!("ファイルを受信しています...");
        move |msg| {
            let contents = string_to_vec(format!("{}", msg));
            let mut file = File::create("hoge.jpg").unwrap();
            file.write_all(&contents).unwrap();
            file.flush().unwrap();
            println!("ファイルの受信が完了しました。");
            out.send(" ");
            out.close(CloseCode::Normal)
        }

    }).unwrap()

}

fn start_client(){

    let ip = "127.0.0.1";
    let port = "8080";
    connect((format!("ws://{}:{}", ip, port)), |out| {
        let mut file_name: String = String::new();
        println!("送信するファイル名を入力してください");
        stdin().read_line(&mut file_name)
            .expect("Failed to read line");
        file_name.retain(|chr| chr != '\n');
        let file_binary = get_file_binary(file_name.clone());
        println!("ファイルを送信しています...");
        out.send(format!("{:?}", file_binary)).unwrap(); 
        move |msg| {
            println!("ファイルの送信を完了しました。");
            out.close(CloseCode::Normal)
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
