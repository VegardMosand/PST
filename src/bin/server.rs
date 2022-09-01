use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use common::com::{Message, MsgType};
use std::net::SocketAddr;
use tokio::net::tcp::WriteHalf;
use std::sync::Arc;
use dashmap::DashMap;

#[derive(Debug)]
struct ClientInfo{
    adress : SocketAddr
}

#[tokio::main]
async fn main() { 
    let listener = TcpListener::bind("127.0.0.1:8282").await.unwrap();

    // Hashmap safe to use with concurrency
    let name_to_ip = Arc::new(DashMap::<String, ClientInfo>::new());
    
    // Connection loop
    loop{
        let (stream, _addr) = listener.accept().await.unwrap();
        println!("Connection established");
        let t = Arc::clone(&name_to_ip);
        tokio::spawn(async move {
            handle_connection(stream, t).await;
        });
    }
}

async fn handle_connection(mut stream : TcpStream, name_to_ip : Arc<DashMap<String, ClientInfo>>){
    let (mut read, mut write) = stream.split();
    //read.local_addr().unwrap().port();
    let mut buff = String::new();

    // Event loop for handling messages
    println!("Entering event loop");
    loop{
        let len = read.read_to_string(&mut buff).await.expect("Erroaa");
        
        // Ends connection when contact with client is broken
        if len == 0 {
            return;
        }

        println!("Len: {}", len);
        let buff_str = &buff;
        println!("{:#?}", buff_str);
        let msg : Message = serde_json::from_str::<Message>(buff_str).unwrap();
        match msg.message_type {
            MsgType::Registration => handle_registration(msg, &mut write, Arc::clone(&name_to_ip)),
            MsgType::Lookup => handle_lookup(msg, &mut write, Arc::clone(&name_to_ip)),
            _ => continue,
        }
    }
}

fn handle_registration(msg : Message, write : &mut WriteHalf, name_to_ip : Arc<DashMap<String, ClientInfo>>){
    println!("Handling registration for {}", msg.payload);

    let m = serde_json::to_string(&Message{
        message_type : MsgType::Registration,
        payload : "",
    });
    let _ = write.write_all(m.unwrap().as_bytes());
        let client_info = ClientInfo{
        adress : write.local_addr().unwrap(),
    };
    name_to_ip.insert(String::from(msg.payload), client_info);
    
    println!("{:?}", name_to_ip);
}

fn handle_lookup(msg : Message, write : &mut WriteHalf, name_to_ip : Arc<DashMap<String, ClientInfo>>){
    println!("Looking up {}", msg.payload);
    let info = match name_to_ip.get(msg.payload) {
        Some(info) => info,
        None => return,
    };

    let msg = serde_json::to_string(&Message{
        message_type : MsgType::Lookup,
        payload : &serde_json::to_string(&info.value().adress).unwrap(),
    });
    println!("{:?}", msg);
    let _ = write.write_all(msg.unwrap().as_bytes());
}
