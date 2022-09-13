use common::com::{Message, MsgType};
use dashmap::DashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::WriteHalf;
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug)]
struct ClientInfo {
    address: SocketAddr,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8282").await.unwrap();

    // Hashmap safe to use with concurrency
    let name_to_ip = Arc::new(DashMap::<String, ClientInfo>::new());

    // Connection loop
    loop {
        let (stream, _addr) = listener.accept().await.unwrap();
        println!("Connection established {:?}", stream);
        let cache = Arc::clone(&name_to_ip);
        tokio::spawn(async move {
            handle_connection(stream, cache).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream, name_to_ip: Arc<DashMap<String, ClientInfo>>) {
    let (mut read, mut write) = stream.split();
    let mut buff = [0u8; 1024];
    println!("{:?}", read);

    // Event loop for handling messages
    loop {
        let len = read.read(&mut buff).await.unwrap();
        let buff_str = String::from_utf8_lossy(&buff[..len]);

        // Ends connection when contact with client is broken
        if len == 0 {
            return;
        }

        let msg: Message = serde_json::from_str::<Message>(&buff_str).unwrap();
        match msg.message_type {
            MsgType::Registration => handle_registration(msg, &mut write, Arc::clone(&name_to_ip)).await,
            MsgType::Lookup => handle_lookup(msg, &mut write, Arc::clone(&name_to_ip)).await,
            _ => continue,
        }
    }
}

async fn handle_registration(
    msg: Message,
    write: &mut WriteHalf<'_>,
    name_to_ip: Arc<DashMap<String, ClientInfo>>,
) {
    println!("Handling registration for {}", msg.sender);

    let m = serde_json::to_string(&Message {
        message_type: MsgType::Registration,
        sender: String::from(""),
        recipient: String::from(""),
        payload: String::from(""),
    });
    println!("Sending response");
    let _ = write.write_all(m.unwrap().as_bytes()).await;
    let client_addr : SocketAddr = serde_json::from_str(&msg.payload).unwrap();
    let client_info = ClientInfo {
        address: client_addr,
    };
    name_to_ip.insert(String::from(msg.sender), client_info);

}

async fn handle_lookup(
    msg: Message,
    write: &mut WriteHalf<'_>,
    name_to_ip: Arc<DashMap<String, ClientInfo>>,
) {
    println!("Looking up {}", msg.payload);
    let info = match name_to_ip.get(&msg.payload) {
        Some(info) => info,
        None => {
            send_lookup_failed(write).await;
            return
        },
    };

    let msg = serde_json::to_string(&Message {
        message_type: MsgType::Lookup,
        sender: String::from(""),
        recipient: String::from(""),
        payload: String::from(&serde_json::to_string(&info.value().address).unwrap()),
    });
    let _ = write.write_all(msg.unwrap().as_bytes()).await;
}

async fn send_lookup_failed(write : &mut WriteHalf<'_>){
    println!("Lookup failed");
    let msg = serde_json::to_string(&Message {
        message_type: MsgType::Lookup,
        sender: String::from(""),
        recipient: String::from(""),
        payload: String::from(""),
    });
    
    let _ = write.write_all(msg.unwrap().as_bytes()).await;
}