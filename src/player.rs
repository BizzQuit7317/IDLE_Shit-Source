use serde::{Serialize, Deserialize};
use crate::structs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub Name:String,
    pub Currency:f64,
    pub CPS:f64,
    pub cps_upgrade: f64,
    pub cps_upgrade_cost: f64,
    pub log_off_time: String,
    pub Inventory_Con:Vec<structs::Consumable>,
    pub Inv_Con_Max:usize,
    pub Rebirths:u8,
    pub Rebirth_Processed:bool,
}

impl Player {
    pub fn new(username: &str) -> Player {
        let player_name = username;

        Player {
            Name:player_name.to_string(),
            Currency:0.0,
            CPS:1.0,
            cps_upgrade:2.0,
            cps_upgrade_cost:10.0,
            log_off_time: String::from("None"),
            Inventory_Con:Vec::new(),
            Inv_Con_Max: 7,
            Rebirths:0,   //Setting for testing naming page
            Rebirth_Processed: false,
        }
    }

    pub fn update_currency(&mut self, update: f64) {
        self.Currency += update;
    }

    pub fn update_cps(&mut self, update: f64) {
        self.CPS += update;
    }

    pub fn update_cps_costs(&mut self) {
        self.cps_upgrade_cost *= 5.0;
        self.cps_upgrade *= 2.0;
    }

    pub fn buy_upgrade(&mut self, upgrade_cost: f64) -> bool {
        if self.Currency >= upgrade_cost {
            self.Currency -= upgrade_cost;
            true
        } else {
            println!("Not enough points to purchase the upgrade!");
            false
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::new("Default")
    }
}
