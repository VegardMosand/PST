use tokio::net::{TcpListener, TcpStream};
use std::io;
use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use common::com::{Message, MsgType};
use dashmap::DashMap;
use std::sync::Arc;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Create cache for storing other clients
    let cache = Arc::new(DashMap::<String, TcpStream>::new());

    // Get username
    println!("Please type in your username: ");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Could not read user input\n");
    username.pop();


    // Start listening for incoming messages
    let listener = create_listener(1024).await;

    let addr = serde_json::to_string(&listener.local_addr().unwrap());
    tokio::spawn(async move {
        incoming_message_listen_loop(listener).await;
    });

    // Send Registration package and enter event loop
    let reg = serde_json::to_string(&Message{
        message_type : MsgType::Registration,
        sender : String::from(&username),
        recipient : String::from(""),
        payload : addr.unwrap(), 
    }).unwrap();   
    let cache_clone= Arc::clone(&cache);
    if let Ok(mut server_stream) = TcpStream::connect("127.0.0.1:8282").await{
        println!("IP = {}, Port = {}", server_stream.local_addr().unwrap().ip(), server_stream.local_addr().unwrap().port());
        let _ = server_stream.write_all(reg.as_bytes()).await;

        let mut buff = [0u8; 1024];
        let len = server_stream.read(&mut buff).await.unwrap();
        let buff_str = String::from_utf8_lossy(&buff[..len]);
        let msg : Message = serde_json::from_str::<Message>(&buff_str).unwrap();

        if msg.message_type == MsgType::Registration {
            println!("Great! You are now registered in the system.\nType h for help\n");
            event_loop(&mut server_stream, cache_clone, &username).await;
        }
    } else{
        println!("Failed to connect to server! Exiting...");
    }
}

// Handles server communication and outgoing messages to other clients
async fn event_loop(stream : &mut TcpStream, cache : Arc<DashMap<String, TcpStream>>, username : &String){
    let mut user_buffer = String::new();

    loop{
        // User input
        io::stdin().read_line(&mut user_buffer).expect("Could not read user input\n");
        let mut buff_iterator = user_buffer.chars();
        match buff_iterator.next().unwrap(){
            'h' => print_help(),
            '@' => handle_send_message(stream, buff_iterator.collect::<String>(), Arc::clone(&cache), username).await,
            'q' => break,
            _ =>  {println!("Incorrect syntax!"); print_help();},
        };
        user_buffer.clear();
    
    }
}



fn print_help(){
    println!("To send a message: @user message");
}

async fn handle_send_message(stream : &mut TcpStream, buff_iterator : String, cache : Arc<DashMap<String, TcpStream>>, username : &String){
    let (dest_user, message) = match buff_iterator.split_once(" "){
        Some((d, m)) => (d, m),
        None => return,
    };
    
    
    // Get destination address and port from the cache or look it up
    if cache.get_mut(dest_user).is_none() && !send_lookup(stream, dest_user, Arc::clone(&cache), username).await {
        println!("That host is not registered in the system");
        return;
    }
    let mut addr = cache.get_mut(dest_user).unwrap();
    send_message(addr.value_mut(), message, dest_user, username).await;
}

async fn send_lookup(stream : &mut TcpStream, dest_user : &str, cache : Arc<DashMap<String, TcpStream>>, username : &String) -> bool {
    let m = serde_json::to_string(&Message{
        message_type : MsgType::Lookup,
        sender : String::from(username),
        recipient : String::from(""),
        payload : String::from(dest_user),
    });
    let _ = stream.write_all(&m.unwrap().as_bytes()).await;
    return await_lookup_response(stream, cache, dest_user).await;
    
}

async fn await_lookup_response(stream : &mut TcpStream, cache : Arc<DashMap<String, TcpStream>>, dest_user : &str) -> bool {
    let mut buff = [0u8; 1024];
    let len = stream.read(&mut buff).await.unwrap();
    let buff_str = String::from_utf8_lossy(&buff[..len]);
    let msg : Message = serde_json::from_str::<Message>(&buff_str).unwrap();


    let addr= serde_json::from_str(&msg.payload) as Result<SocketAddr, serde_json::Error>;

    let addr = match addr {
        Ok(addr) => addr,
        Err(_) => return false,
    };
    cache.insert(String::from(dest_user), TcpStream::connect(addr).await.unwrap());
    return true;
}

async fn send_message(stream : &mut TcpStream, message : &str, recipient : &str, username : &str){
    let m = serde_json::to_string(&Message{
        message_type : MsgType::Chat,
        sender : String::from(username),
        recipient : String::from(recipient),
        payload : String::from(message),
    });
    
    let _ = stream.write_all(&m.unwrap().as_bytes()).await;
}


/* Finds an available port to create a TcpListener */
async fn create_listener(mut port : u16) -> TcpListener{
    const RESERVED : u16 = 1024;
    loop{
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port.to_string())).await;
        match listener{
            Ok(listener) => return listener,
            Err(_) => port = ((port+1) % (u16::MAX-RESERVED)) + RESERVED,
        }
    }
}

/* Detects and handles incoming messages */
async fn incoming_message_listen_loop(listener : TcpListener) {
    loop {
        let (stream, _addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}

async fn handle_connection(mut stream : TcpStream){
    loop{
        let mut buff = [0u8; 1024];
        let len = stream.read(&mut buff).await.unwrap();
        if len == 0 {
            break;
        }
        let buff_str = String::from_utf8_lossy(&buff[..len]);
        let msg : Message = serde_json::from_str::<Message>(&buff_str).unwrap();

        match msg.message_type{
            MsgType::Chat => handle_message(&msg),
            _ => println!("Invalid message type"),
        }
    }
}

fn handle_message(msg : &Message){
    print!("{}: {}", msg.sender, msg.payload);
}