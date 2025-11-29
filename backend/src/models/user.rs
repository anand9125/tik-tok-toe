use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub username: String,
    pub password: String
}

#[derive(Serialize,Deserialize)]
pub struct UserResponse{
    pub id : String
}

#[derive(Serialize,Deserialize)]
pub struct SigninResponse {
    pub token: String
}

#[derive(Serialize,Deserialize)]
pub struct Claims {
    pub sub : String,
    pub exp : usize

}

impl Claims{
    pub fn new(sub:String) ->Self{
        Self { 
            sub,
            exp:1000000000000,
        }
    }
}