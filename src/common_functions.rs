extern crate native_windows_gui as nwg;

use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::file_control;
use crate::structs;
use crate::player::Player;
use crate::creature::Creature;

pub fn get_resolution() -> (i32, i32) {
    let output = Command::new("wmic")
        .args(["path", "Win32_VideoController", "get", "CurrentHorizontalResolution,CurrentVerticalResolution"])
        .output()
        .ok(); // If command fails, return None

    if let Some(output) = output {
        let result = String::from_utf8_lossy(&output.stdout);

        for line in result.lines() {
            let values: Vec<&str> = line.split_whitespace().collect();
            if values.len() == 2 {
                if let (Ok(width), Ok(height)) = (values[0].parse::<i32>(), values[1].parse::<i32>()) {
                    return (width, height);
                }
            }
        }
    }
    // Default resolution if anything fails
    (800, 450)
}

pub fn get_current_time() -> String{
    let start = SystemTime::now();
    let mut time_string = String::new();

    match start.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            time_string = duration.as_secs().to_string();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    time_string
}

pub fn set_button_state(buttons: Vec<&nwg::Button>, labels: Vec<&nwg::Label>, textInputs: Vec<&nwg::TextInput>, imageFrames: Vec<&nwg::ImageFrame>, State: bool) {
    /*
        to add vectors of len 0 use function like below
        set_button_state(Vec::new(), Vec::new(), Vec::new(), false);
        that will set all vectors to empty

        Also ensure imageFrames are processed first
        as they need to go to the background
    */
    
    if buttons.len() != 0 {
        for control in buttons {
            control.set_enabled(State);
            control.set_visible(State);
        }
    }
    if labels.len() != 0 {
        for control in labels {
            control.set_enabled(State);
            control.set_visible(State);
        }
    }
    if textInputs.len() != 0 {
        for control in textInputs {
            control.set_enabled(State);
            control.set_visible(State);
        }
    }
    if imageFrames.len() != 0 {
        for control in imageFrames {
            control.set_enabled(State);
            control.set_visible(State);
        }
    }
}

pub fn human_readable_time_from_epoch(epoch_time: f64) -> String {
    if epoch_time != 0.0{
        let seconds = epoch_time;
        let minutes = seconds/60.0;
        let hours = minutes/60.0;
        let days = hours/24.0;
        let weeks = days/7.0;

        let time_vec = vec![seconds, minutes, hours, days, weeks];
        let mut index_track = 0;
        let mut readable_index = 0;
        let mut time_scale: String = String::new();

        for time in &time_vec {
            if *time < 1.0 {
                readable_index = index_track - 1;
                break
            }
            index_track += 1;
        }

        if readable_index == 0 {
            time_scale = "Seconds".to_string();
        } else if readable_index == 1 {
            time_scale = "Minutes".to_string();
        } else if readable_index == 2 {
            time_scale = "Hours".to_string();
        } else if readable_index == 3 {
            time_scale = "Days".to_string();
        } else if readable_index == 4 {
            time_scale = "Weeks".to_string();
        }

        format!("{:.2} {}", time_vec[readable_index], time_scale)
    } else {
        "0.0 seconds".to_string()
    }
}

pub fn read_setting_screen_size() -> (i32, i32) {
    let SETTINGS = file_control::read_json_file::<structs::Settings>("Settings/config.json");

    let mut X = 0; // THIS IS THE DEFAULT X/Y INCASE FAILED TO READ CONFIG
    let mut Y = 0;

    for i in SETTINGS {
        X = i[0].WNDW_X as i32;
        Y = i[0].WNDW_Y as i32;
    }
    
    let WNDW = (X, Y);
    WNDW
}

pub fn write_new_player_binary_file(file_path: &str, userName: String) {
    /*
        This will overwrite what ever is in that directory
        with the data given
    */
    let player = Player::new(&userName);

    let _ = file_control::write_binary_file(file_path, &player);
}

pub fn write_new_creature_binary_file(file_path: &str, creatureName: String) {
    /*
        This will overwrite what ever is in that directory
        with the data given
    */
    let creature = Creature::new(creatureName);

    let _ = file_control::write_binary_file(file_path, &creature);
}

pub fn read_setting_data_paths(Type: bool) -> String {
    /*
        This takes a type to switch between creature and player
        just so I dont have to write 2 identical functions
        thers prolly a better way but fuck it this works
    */
    let SETTINGS = file_control::read_json_file::<structs::Settings>("Settings/config.json");
    let mut PATH = String::new();
    for i in SETTINGS {
        if Type {
            PATH = i[0].DEFAULT_PLAYER_PATH.clone();
        } else {
            PATH = i[0].DEFAULT_CREATURE_PATH.clone();
        }
    }


    PATH
}
