
pub mod com{
    use serde::{Serialize, Deserialize};
    
    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    pub enum MsgType {
        Registration = 0,
        Lookup = 1,
        Chat = 2,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Message<'a>{
        pub message_type : MsgType,
        pub payload : &'a str,
    }
}