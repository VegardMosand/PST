use tokio::net::{ TcpListener, TcpStream };
use std::io;
use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use common::com::{Message, MsgType};
use dashmap::DashMap;
use std::sync::Arc;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let cache = Arc::new(DashMap::<String, SocketAddr>::new());
    println!("Welcome to the Public \"Secure\" Transportation chating service!\nPlease type in your username: ");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Could not read user input\n");
    let m = serde_json::to_string(&Message{
        message_type : MsgType::Registration,
        payload : &username,
    });

    let cache_client = Arc::clone(&cache);

    if let Ok(mut client_stream) = TcpListener::bind("127.0.0.1:8281").await{
        tokio::spawn(async move {
            let _ = in_event_loop(&mut client_stream, cache_client);
        });
    }
    
    let cache_server = Arc::clone(&cache);

    // Send Registration package
    if let Ok(mut server_stream) = TcpStream::connect("127.0.0.1:8282").await{
        println!("IP = {}", server_stream.local_addr().unwrap().ip());
        let _ = server_stream.write(m.unwrap().as_bytes());
        
        tokio::spawn(async move {
            let _ = out_event_loop(&mut server_stream, cache_server);
        });
        
        
    } else{
        println!("Failed to connect to server! Exiting...");
    }
}

// Handles server communication and outgoing messages to other clients
async fn out_event_loop(stream : &mut TcpStream, cache : Arc<DashMap<String, SocketAddr>>){
    println!("Great! You are now registered in the system.\nType h for help\n");
    let mut buffer = String::new();

    loop{
        io::stdin().read_line(&mut buffer).expect("Could not read user input\n");
        let mut buff_iterator = buffer.chars();
       // println!("bi {}", buff_iterator.next().unwrap());
        println!("{}", '@' == '@');
        match buff_iterator.next().unwrap(){
            'h' => print_help(),
            '@' => handle_send_message(stream, buff_iterator.collect::<String>(), Arc::clone(&cache)).await,
            _ =>  {println!("Incorrect syntax!"); print_help();},
        };
    }
}

fn print_help(){
    println!("To send a message: @user message");
}

async fn handle_send_message(stream : &mut TcpStream, buff_iterator : String, cache : Arc<DashMap<String, SocketAddr>>){
    let (dest_user, message) = match buff_iterator.split_once(" "){
        Some((d, m)) => (d, m),
        None => return,
    };

    // Get destination address and port from the cache or look it up
    let dest_addr : SocketAddr = match cache.get(dest_user) {
        Some(addr) => *addr.value(),
        None => send_lookup(stream, dest_user, Arc::clone(&cache)).await,
    };

    send_message(dest_addr, message).await
}

async fn send_lookup(stream : &mut TcpStream, user : &str, cache : Arc<DashMap<String, SocketAddr>>) -> SocketAddr {
    let m = serde_json::to_string(&Message{
        message_type : MsgType::Lookup,
        payload : &user,
    });
    println!("lookup {:?}", m);
    let _ = stream.write(&m.unwrap().as_bytes());
    let addr : SocketAddr = await_lookup_response(stream).await;
    cache.insert(String::from(user), addr);
    return addr
}

async fn await_lookup_response(stream : &mut TcpStream) -> SocketAddr {
    let mut buff = String::new();
    stream.read_to_string(&mut buff).await.expect("Could not read lookup response");

    let msg : Message = serde_json::from_str::<Message>(&buff).unwrap();
    
    return serde_json::from_str(msg.payload).unwrap();
}

async fn send_message(adress : SocketAddr, message : &str){
    let mut stream = TcpStream::connect(adress).await.unwrap();
    let m = serde_json::to_string(&Message{
        message_type : MsgType::Chat,
        payload : message,
    });
    println!("message {:?}", m);
    
    let _ = stream.write(&m.unwrap().as_bytes());
}


// Handles incoming messages from other users
async fn in_event_loop(stream : &mut TcpListener, cache : Arc<DashMap<String, SocketAddr>>){
    loop{
        
    }
}