use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use std::collections::HashMap;
use std::str::Split;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    if http_request[0] == "GET /ping HTTP/1.1"
    {
        println!("{}", "HTTP/1.1 200 OK");
        for i in 1..http_request.len()
        {
            let split: Vec<&str> = http_request[i].split(":").collect();
            let mut map = HashMap::new();
            map.insert(String::from(split[0]), split[1]);
            for (key, value) in map {
                let res = &format!("{}{}", key, value);
                println!("{}", res);
            }
            
        }
        
    } else {
        println!("HTTP/1.1 404 NOT FOUND");
    }
}

//env::var VARIABLE_ENVIRONNEMENT
//format!("0.0.0.0:{val}
//boucle sur le map
//format! -> créer une string a partir de variable