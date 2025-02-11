extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use std::thread::current;
use std::time::Duration;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::fs::{self, File};
use crate::{common_functions, file_control};
use crate::player::Player;
use crate::creature::Creature;

/*
    Setting lazy statics to use during running
*/
lazy_static!{
    static ref RUNNING_STATE: Mutex<bool> = Mutex::new(false);

    static ref FRAME: Mutex<u8> = Mutex::new(0);

    static ref PLAYER_DATA: Mutex<Player> = Mutex::new(Player {
        Name: String::from("Unknown"),
        Currency: 0.0,
        CPS: 0.0,
        cps_upgrade: 0.0,
        cps_upgrade_cost: 0.0,
        log_off_time: String::from("Never"),
        Inventory_Con: Vec::new(),
        Inv_Con_Max:7,
        Rebirths:0,
        Rebirth_Processed:false,
    });

    static ref CREATURE_DATA: Mutex<Creature> = Mutex::new(Creature {
        Name:String::from("Unknown"),
        Appendages:0.0,
        Hunger:100.0,
        Thirst:100.0,
        Productivity:10.0,
        LifeSpan:86400.0,
        Status:true,        
    });
}

/*
    Creating the structure of all the pages
*/
#[derive(Default, NwgUi)]
pub struct BasicApp {
    /*
        Setting the basic controls of the page
    */
    #[nwg_control(size: common_functions::read_setting_screen_size(), position: (200, 200), title: "IDLE Shit", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::exit_routine] )]
    window: nwg::Window,

    /*
        Everpresent Controls
    */
    #[nwg_resource(source_file: Some("Assets/Icons/settings.bmp"))]
    settings_bitmap: nwg::Bitmap,
    #[nwg_control(text: " ", size: (100, 100), position: (1180, 0), bitmap: Some(&data.settings_bitmap))]
    #[nwg_events(OnButtonClick: [BasicApp::settings_page])]
    settings_button: nwg::Button,

    /*
        Start Page
    */
    #[nwg_control(text: "New", size: (300, 300), position: (200, 320))]
    #[nwg_events(OnButtonClick: [BasicApp::create_player_page])]
    new_button: nwg::Button,

    #[nwg_control(text: "Load", size: (300, 300), position: (780, 320))]
    #[nwg_events(OnButtonClick: [BasicApp::load_game])]
    load_button: nwg::Button,

    #[nwg_control(text: "Your creature can die in many ways try to keep that little piece of shit alive for as long as possible", size: (100, 400), position: (590, 320))]
    loading_message: nwg::Label,

    #[nwg_resource(source_file: Some("Assets/Logo/logo_start_page.bmp"))]
    start_page_logo_bitmap: nwg::Bitmap,
    #[nwg_control(size: (430, 260), position: (425,10), bitmap: Some(&data.start_page_logo_bitmap))]
    start_page_logo_image_frame: nwg::ImageFrame,

    /*
        Settings Page
    */
    #[nwg_control(text: "<-", size: (100, 100), position: (10, 10), flags: "DISABLED")]
    #[nwg_events(OnButtonClick: [BasicApp::start_page])]
    back_start_button: nwg::Button,

    #[nwg_control(text: "<-", size: (100, 100), position: (10, 10), flags: "DISABLED")]
    #[nwg_events(OnButtonClick: [BasicApp::creature_page])]
    back_main_button: nwg::Button,

    #[nwg_control(text: "Slaughter Creature", size: (426, 100), position: (426, 300), flags: "DISABLED")]
    #[nwg_events(OnButtonClick: [BasicApp::test_button])]
    kill_creature_button: nwg::Button,

    #[nwg_control(text: "Test Function", size: (426, 100), position: (426, 450), flags: "DISABLED")]
    #[nwg_events(OnButtonClick: [BasicApp::test_button])]
    test_button: nwg::Button,

    /*
        Create New Player Page
    */
    #[nwg_control(text: "Username", size: (426, 25), position: (426, 300), flags: "DISABLED")]
    username_input: nwg::TextInput,

    #[nwg_control(text: "Ok", size: (426, 100), position: (426, 350), flags: "DISABLED")]
    #[nwg_events(OnButtonClick: [BasicApp::confirm_username])]
    enter_name_button: nwg::Button,

    #[nwg_control(text: "Cancel", size: (426, 100), position: (426, 500), flags: "DISABLED")]
    #[nwg_events(OnButtonClick: [BasicApp::start_page])]
    cancel_name_button: nwg::Button,

    /*
        Offline Page
    */
    #[nwg_control(text: "Ok", size: (426, 100), position: (426, 550), flags: "DISABLED")]
    #[nwg_events(OnButtonClick: [BasicApp::creature_page])]
    offline_update_button: nwg::Button,

    #[nwg_control(text: "", size: (280, 200), position: (426, 350), flags: "DISABLED")]
    offline_update_lable: nwg::Label,

    /*
        Side Menu
    */
    #[nwg_control(text: "Creature", size: (280, 60), position: (10, 100), flags: "DISABLED")]
    #[nwg_events(OnButtonClick: [BasicApp::creature_page])]
    creature_button: nwg::Button,

    #[nwg_control(text: "Store", size: (280, 60), position: (10, 200), flags: "DISABLED")]
    #[nwg_events(OnButtonClick: [BasicApp::test_button])]
    store_button: nwg::Button,

    #[nwg_control(text: "Save", size: (280, 60), position: (10, 300), flags: "DISABLED")]
    #[nwg_events(OnButtonClick: [BasicApp::save_game])]
    save_button: nwg::Button,

    /*
        Creature Page
    */
    #[nwg_control(text: "", size: (280, 60), position: (10, 10), flags: "DISABLED")]
    player_stats: nwg::Label,

    #[nwg_control(text: "", size: (100, 60), position: (400, 50), flags: "DISABLED")]
    player_points: nwg::Label,

    #[nwg_control(text: "", size: (150, 80), position: (400, 100), flags: "DISABLED")]
    creature_stats: nwg::Label,

    #[nwg_control(text: "", size: (150, 60), position: (400, 180), flags: "DISABLED")]
    creature_productivity: nwg::Label,

    #[nwg_resource(source_file: Some("Assets/Creatures/chad_a.bmp"))]
    creature_bitmap: nwg::Bitmap,
    #[nwg_control(size: (200, 200), position: (300, 220), bitmap: Some(&data.creature_bitmap), flags: "DISABLED")]
    creature_image_frame: nwg::ImageFrame,

    #[nwg_control(text: "None", size: (100, 60), position: (590, 100), flags: "DISABLED")]
    #[nwg_events(OnButtonClick: [BasicApp::use_inv_0_button])]
    inv_0_button: nwg::Button,

    #[nwg_control(text: "None", size: (100, 60), position: (690, 100), flags: "DISABLED")]
    #[nwg_events(OnButtonClick: [BasicApp::use_inv_1_button])]
    inv_1_button: nwg::Button,

    #[nwg_control(text: "None", size: (100, 60), position: (590, 160), flags: "DISABLED")]
    #[nwg_events(OnButtonClick: [BasicApp::use_inv_2_button])]
    inv_2_button: nwg::Button,

    #[nwg_control(text: "None", size: (100, 60), position: (690, 160), flags: "DISABLED")]
    #[nwg_events(OnButtonClick: [BasicApp::use_inv_3_button])]
    inv_3_button: nwg::Button,

    #[nwg_control(text: "None", size: (100, 60), position: (590, 220), flags: "DISABLED")]
    #[nwg_events(OnButtonClick: [BasicApp::use_inv_4_button])]
    inv_4_button: nwg::Button,

    #[nwg_control(text: "None", size: (100, 60), position: (690, 220), flags: "DISABLED")]
    #[nwg_events(OnButtonClick: [BasicApp::use_inv_5_button])]
    inv_5_button: nwg::Button,

    #[nwg_control(text: "None", size: (100, 60), position: (590, 280), flags: "DISABLED")]
    #[nwg_events(OnButtonClick: [BasicApp::use_inv_6_button])]
    inv_6_button: nwg::Button,

    #[nwg_control(text: "None", size: (100, 60), position: (690, 280), flags: "DISABLED")]
    #[nwg_events(OnButtonClick: [BasicApp::use_inv_7_button])]
    inv_7_button: nwg::Button,

    

    /*
        Main frame ticker
    */
    #[nwg_control(interval: Duration::from_secs(1))]  // Timer interval in milliseconds (1000ms = 1 second)
    #[nwg_events(OnTimerTick: [BasicApp::update_ticker])]
    ticker_timer: nwg::AnimationTimer
}

impl BasicApp {
    /*
        Routines for recurring task
    */
    fn exit_routine(&self) {
        /*
            This is the standard exit routine when closing the game
            will save the game to the running user by default
        */
        nwg::simple_message("Exit", "Closing Programme");
        self.save_game();
        nwg::stop_thread_dispatch();
    }

    fn disable_all_controls(&self) {
        /*
            This is a controller function, its main purpose is to hold 
            all the lists of nwg_controls. When adding new pages you
            must add the controls into here. You can exclude buttons like 
            settings that are permenantly on the screen
        */
        let buttons = vec![&self.save_button, &self.offline_update_button, &self.store_button, &self.creature_button, &self.inv_0_button, &self.inv_1_button, &self.inv_2_button, &self.inv_3_button, &self.inv_4_button, &self.inv_5_button, &self.inv_6_button, &self.inv_7_button, &self.enter_name_button, &self.cancel_name_button, &self.kill_creature_button, &self.test_button, &self.back_main_button, &self.back_start_button, &self.new_button, &self.load_button];
        let labels = vec![&self.offline_update_lable, &self.loading_message, &self.player_points, &self.player_stats, &self.creature_stats, &self.creature_productivity];
        let textInputs = vec![&self.username_input];
        let imageFrames = vec![&self.start_page_logo_image_frame, &self.creature_image_frame];

        common_functions::set_button_state(buttons, labels, textInputs, imageFrames, false);
    }

    /*
        Button functions for changing to specific pages
    */
    fn start_page(&self) {
        self.disable_all_controls();

        let buttons = vec![&self.new_button, &self.load_button];
        let labels = vec![&self.loading_message];
        let textInputs = vec![];
        let imageFrames = vec![&self.start_page_logo_image_frame];
        

        common_functions::set_button_state(buttons.clone(), labels, textInputs, imageFrames, true);
    }

    fn settings_page(&self) {
        /*
            The running state will dictate weather to show the back 
            button with the START SCREEN or with the MAIN PAGE
        */
        self.disable_all_controls();
        let mut STATE = RUNNING_STATE.lock().unwrap();

        let mut buttons = vec![&self.kill_creature_button, &self.test_button];
        let labels = vec![];
        let textInputs = vec![];
        let imageFrames = vec![&self.start_page_logo_image_frame];

        if *STATE {
            buttons.push(&self.back_main_button);
        } else {
            buttons.push(&self.back_start_button);
        }

        common_functions::set_button_state(buttons, labels, textInputs, imageFrames, true);

    }

    fn create_player_page(&self) {
        self.disable_all_controls();

        let buttons = vec![&self.enter_name_button, &self.cancel_name_button];
        let labels = vec![];
        let textInputs = vec![&self.username_input];
        let imageFrames = vec![&self.start_page_logo_image_frame];

        common_functions::set_button_state(buttons, labels, textInputs, imageFrames, true);
    }

    fn creature_page(&self) {
        /*
            Set labels and buttons before changing screen over
        */
        let _ = &self.set_creature_page_buttons();

        self.disable_all_controls();

        let buttons = vec![&self.inv_0_button, &self.inv_1_button, &self.inv_2_button, &self.inv_3_button, &self.inv_4_button, &self.inv_5_button, &self.inv_6_button, &self.inv_7_button];
        let labels = vec![&self.player_points, &self.player_stats, &self.creature_stats, &self.creature_productivity];
        let textInputs = vec![];
        let imageFrames = vec![&self.creature_image_frame];

        common_functions::set_button_state(buttons, labels, textInputs, imageFrames, true);
        &self.side_buttons(); //Call second to enable buttons along side
    }

    fn offline_page(&self) {
        self.disable_all_controls();

        let buttons = vec![&self.offline_update_button];
        let labels = vec![&self.offline_update_lable];
        let textInputs = vec![];
        let imageFrames = vec![&self.start_page_logo_image_frame];

        common_functions::set_button_state(buttons, labels, textInputs, imageFrames, true);
    }

    fn side_buttons(&self) {
        let buttons = vec![&self.store_button, &self.creature_button, &self.save_button];
        let labels = vec![];
        let textInputs = vec![];
        let imageFrames = vec![];

        common_functions::set_button_state(buttons, labels, textInputs, imageFrames, true);
    }

    /*
        Functional buttons
    */
    fn confirm_username(&self) {
        /*
            First handle the creation of the new player and creature
            then once all files are made switch the page
        */
        let mut GAME_STATE = RUNNING_STATE.lock().unwrap();
        let _ = *GAME_STATE = true; //start the game running, this also sets the settings back to creature page
        &self.ticker_timer.start(); //start the game ticker

        let _ = common_functions::write_new_player_binary_file(&common_functions::read_setting_data_paths(true), self.username_input.text()); //Create new player
        let _ = common_functions::write_new_creature_binary_file(&common_functions::read_setting_data_paths(false), String::from("Chad")); //Create new creature

        let _ = &self.load_player_to_memory();
        let _ = &self.load_creature_to_memory();

        //Logic from here switches page
        self.creature_page();

    }

    fn load_game(&self) {
        /*
            The first part needs to read the player file
            and setup memory, then it needs to call the 
            creature page
        */
        let mut GAME_STATE = RUNNING_STATE.lock().unwrap();
        let _ = *GAME_STATE = true; //start the game running, this also sets the settings back to creature page
        &self.ticker_timer.start();

        self.load_player_to_memory();
        self.load_creature_to_memory();

        self.set_offline_page_buttons();

        //Logic from here switches page
        self.offline_page();
    }

    /*
        Setting Buttons
    */
    fn set_creature_page_buttons(&self) {
        let PLAYER = PLAYER_DATA.lock().unwrap();
        let player_stat_text = format!("Name: {}\nCPS: {}", PLAYER.Name, PLAYER.CPS);

        let CREATURE = CREATURE_DATA.lock().unwrap();
        let human_readable_creature_life = common_functions::human_readable_time_from_epoch(CREATURE.LifeSpan);
        let creature_stat_text = format!("{}\nHunger: {}\nThirst: {}\nLife: {}", CREATURE.Name, CREATURE.Hunger, CREATURE.Thirst, human_readable_creature_life);
        let creature_productivity_text = format!("Productivity: {}", CREATURE.Productivity);
        let player_points = format!("Creature Points: {}", PLAYER.Currency);

        let _ = &self.player_stats.set_text(&player_stat_text);
        let _ = &self.creature_stats.set_text(&creature_stat_text);
        let _ = &self.creature_productivity.set_text(&creature_productivity_text);
        let _ = &self.player_points.set_text(&player_points);
    }

    fn set_offline_page_buttons(&self) {
        let CREATURE = CREATURE_DATA.lock().unwrap();
        let mut PLAYER = PLAYER_DATA.lock().unwrap();

        let current_time = common_functions::get_current_time().parse::<f64>().unwrap();
        let last_time = PLAYER.log_off_time.parse::<f64>().unwrap();
        let time_diff = current_time - last_time;

        let points_earned = PLAYER.CPS * time_diff;
        let human_time_diff = common_functions::human_readable_time_from_epoch(time_diff);

        let _ = PLAYER.update_currency(points_earned); //Make sure to update the player points

        let offline_update_text = format!("You were away for {}\nWhile away {} produced {} creature points\n\n{} also lost {} of life!", human_time_diff, CREATURE.Name, points_earned, CREATURE.Name, human_time_diff);
        &self.offline_update_lable.set_text(&offline_update_text);

    }

    /*
        Memory Management
    */
    fn load_player_to_memory(&self) {
        /*
            Read the player binary and updates the running memory
        */
        let player = file_control::read_binary_file::<Player>(&common_functions::read_setting_data_paths(true)).unwrap(); //read the new player

        let mut TMP_PLAYER = PLAYER_DATA.lock().unwrap();
        *TMP_PLAYER = player; //Update the memory with the player data
    }

    fn load_creature_to_memory(&self) {
        /*
            Read the creature binary and updates the running memory
        */
        let creature = file_control::read_binary_file::<Creature>(&common_functions::read_setting_data_paths(false)).unwrap(); //read the new player

        let mut TMP_CREATURE = CREATURE_DATA.lock().unwrap();
        *TMP_CREATURE = creature; //Update the memory with the player data
    }

    fn save_game(&self) {
        let mut PLAYER = PLAYER_DATA.lock().unwrap();
        let CREATURE = CREATURE_DATA.lock().unwrap();

        let _ = PLAYER.log_off_time = common_functions::get_current_time();

        let _ = file_control::write_binary_file(&common_functions::read_setting_data_paths(true), &*PLAYER);
        let _ = file_control::write_binary_file(&common_functions::read_setting_data_paths(false), &*CREATURE);
    }

    /*
        Functions for player inventory buttons
    */
    fn use_inv_1_button(&self) {
        self.test_button();
    }
    fn use_inv_2_button(&self) {
        self.test_button();
    }
    fn use_inv_3_button(&self) {
        self.test_button();
    }
    fn use_inv_4_button(&self) {
        self.test_button();
    }
    fn use_inv_5_button(&self) {
        self.test_button();
    }
    fn use_inv_6_button(&self) {
        self.test_button();
    }
    fn use_inv_7_button(&self) {
        self.test_button();
    }
    fn use_inv_0_button(&self) {
        self.test_button();
    }



    /*
        MAIN GAME LOOP
    */
    fn update_ticker(&self) {
        let mut CURRENT_FRAME = FRAME.lock().unwrap();
        {
            let mut PLAYER = PLAYER_DATA.lock().unwrap();
            let mut CREATURE = CREATURE_DATA.lock().unwrap();

            let _ = PLAYER.Currency += PLAYER.CPS;
            let _ = CREATURE.calculate_productivity();
            let _ = CREATURE.hunger_and_thirst_drop(*CURRENT_FRAME);
            let _ = CREATURE.reduce_lifespan(1.0);
        } //sets the scope so the PLAYER is unlocked when trying to set buttons

        let _ = &self.set_creature_page_buttons();

        /*
            Update the frame
        */
        *CURRENT_FRAME += 1;
        if *CURRENT_FRAME == 61 {
            *CURRENT_FRAME = 0;
        }
    }

    /*
        A TEST FUNCTION TO ENSURE BUTTONS ARE WORKING
    */
    fn test_button(&self) {
        println!("Clicked the button!!!");
        let x = common_functions::read_setting_data_paths(true);
    }
}