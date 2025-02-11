use serde::{Serialize, Deserialize};  
use bincode::{serialize, deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Creature{
    pub Name:String,
    pub Appendages:f64,
    pub Hunger:f64,
    pub Thirst:f64,
    pub Productivity:f64,
    pub LifeSpan:f64,
    pub Status:bool, //A simple binasry for Alive(true) or Dead(false),
}

impl Creature{
    pub fn new(name: String) -> Creature{
        Creature {
            Name:name,
            Appendages:0.0,
            Hunger:100.0,
            Thirst:100.0,
            Productivity:10.0,
            LifeSpan:86400.0, //1 day in seconds
            Status:true,
        }
    }

    pub fn calculate_productivity(&mut self) {
        self.Productivity = ((self.Hunger*4.0)+(self.Thirst*6.0))/10.0;
    }

    pub fn hunger_and_thirst_drop(&mut self, frame: u8) {
        if frame % 10 == 0 && self.Hunger != 0.0{
            self.Hunger -= 1.0;
        } 
        if frame == 30 && self.Thirst != 0.0{
            self.Thirst -= 1.0;
        }
        if self.Hunger == 0.0 && self.Thirst == 0.0{
            self.death();
        }
    }

    pub fn reduce_lifespan(&mut self, reduction: f64) {
        if self.LifeSpan != 0.0 {
            self.LifeSpan -= reduction;
        } else {
            self.death();
        }
    }
    
    pub fn eat_food_drink(&mut self, Type: bool, Value: f64) -> bool {
        /*
            Needs to return a bool so the games know weather a item was used or an empty 
            spot clicked
         */
        if Value != 0.0 {
            if Type {
                self.Hunger += Value;
            } else {
                self.Thirst += Value;
            }
            true
        } else {
            false
        }
    }
    

    fn death(&mut self) {
        self.Status = false;
    }
}

impl Default for Creature {
    fn default() -> Self {
        Creature::new(String::from("Mike Wazowski"))
    }
}