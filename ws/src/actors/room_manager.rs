use std::{collections::HashMap, os::unix::process};
use serde::{Serialize, Deserialize};
use actix::{Actor, Addr, Context, Handler, Message, fut::ok};
use uuid::Uuid;

use crate::Room;



#[derive(Message)]
#[rtype(result="Result<Uuid,String>")]   // Returns room ID on success, error on failure
pub struct JoinRoom{
    pub room_id:Option<Uuid>,
    pub user_id :Uuid,
    pub addr : Addr<WsClient>
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct LeaveRoom{
    pub room_id:Uuid,
    pub user_id:Uuid
}

#[derive(Message)]
#[rtype(result = "Result<(),String>")]//ok uf valid mode ,error with reason if invalid
pub struct PlayerMove{
    pub room_id:Uuid,
    pub user_id : Uuid,
    pub position : usize  //which cell to mark
}

pub struct RoomManager{
    pub rooms:HashMap<Uuid,Room>,
    pub user_room : HashMap<Uuid,Uuid>
}

impl RoomManager{
    pub fn new()->Self{
        Self { 
            rooms:HashMap::new(),
            user_room:HashMap::new()
        }
    }
}
impl Actor for RoomManager{
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("Room manger actor statrted");
    }
}

//handlet for join room message 
impl Handler<JoinRoom> for RoomManager{
    type Result = Result<Uuid,String>;
    fn handle(&mut self, msg: JoinRoom, ctx: &mut Self::Context) -> Self::Result {
        //case 1 user is already in room (reconnecting scenario)
        if let Some(&existing_room_id) = self.user_room.get(&msg.user_id){
          if let Some(room) = self.rooms.get_mut(&existing_room_id){
            room.addrs.insert(msg.user_id,msg.addr.clone())

          }
        }

        
        
    }
}