use std::net::TcpStream;
use std::io;
use std::io::Write;
use std::io::Read;
use serde::{Serialize, Deserialize};
use common::com::{Message, MsgType};
use std::net::SocketAddr;

fn main() {
      
    println!("Welcome to the Public \"Secure\" Transportation chating service!\nPlease type in your username: ");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Could not read user input\n");
    let m = serde_json::to_string(&Message{
        message_type : MsgType::Registration,
        payload : &username,
    });

    println!("{:#?}", m);
    // Send Registration package
    let mut buff : String = String::new();
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8282"){
        println!("IP = {}", stream.local_addr().unwrap().ip());
        let _ = stream.write(m.unwrap().as_bytes());

        event_loop(&mut stream);
    }
}

fn event_loop(stream : &mut TcpStream){
    println!("Great! You are now registered in the system.\nType h for help\n");
    let mut buffer = String::new();

    loop{
        io::stdin().read_line(&mut buffer).expect("Could not read user input\n");
        let mut buff_iterator = buffer.chars();
       // println!("bi {}", buff_iterator.next().unwrap());
        println!("{}", '@' == '@');
        match buff_iterator.next().unwrap(){
            'h' => print_help(),
            '@' => handle_send_message(stream, buff_iterator.collect::<String>()),
            _ =>  {println!("Incorrect syntax!"); print_help();},
        };
    }
}

fn print_help(){
    println!("To send a message: @user message");
}

fn handle_send_message(stream : &mut TcpStream, buff_iterator : String){
    let (dest_user, message) = match buff_iterator.split_once(" "){
        Some((d, m)) => (d, m),
        None => return,
    };

    send_lookup(stream, dest_user);

    let mut response = String::new();
    let _ = stream.read_to_string(&mut response);
    let response_message = serde_json::from_str::<Message>(&response).unwrap();
    let adress = match serde_json::from_str::<SocketAddr>(response_message.payload){
        Ok(x) => x,
        _ => {println!("Could not find anyone with that username"); return;},
    };
    send_message(adress, message)
}

fn send_lookup(stream : &mut TcpStream, user : &str){
    let m = serde_json::to_string(&Message{
        message_type : MsgType::Lookup,
        payload : &user,
    });
    println!("lookup {:?}", m);
    let _ = stream.write(&m.unwrap().as_bytes());
}

fn send_message(adress : SocketAddr, message : &str){
    let mut stream = TcpStream::connect(adress).unwrap();
    let m = serde_json::to_string(&Message{
        message_type : MsgType::Chat,
        payload : message,
    });
    println!("message {:?}", m);
    
    let _ = stream.write(&m.unwrap().as_bytes());
}
// stream.read_to_string(&mut buff);
// let msg : Message = serde_json::from_str::<Message>(&buff).unwrap();
// if msg.message_type == MsgType::Registration{

// }