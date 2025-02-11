use serde::{Deserialize, Serialize};
/*
    This file will hold all the single structs that dont need 
    any implementations
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub WNDW_X:u32,
    pub WNDW_Y:u32,
    pub DEFAULT_PLAYER_PATH:String,
    pub DEFAULT_CREATURE_PATH:String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Consumable{
    pub Name:String,
    pub Type:bool, //Since there are only Food(true) and Drink(false)
    pub Value:f64,
    pub Cost:f64,
    pub img_path:String,
}
