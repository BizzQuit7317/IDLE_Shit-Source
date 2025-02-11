mod page_struct;
mod common_functions;
mod file_control;
mod structs;
mod player;
mod creature;

extern crate native_windows_gui as nwg;

use nwg::NativeUi;
use page_struct::BasicApp;

fn main() {

    nwg::init().expect("Failed to init Native Windows GUI");

    let mut _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}