extern crate nuitrack_rs;
use nuitrack_rs as nui;

fn main(){
    // Initialized
    nui::init().expect("Failed to initialize nui");

    // Data Stream Setup
    nui::skeleton_data(|data| {
        println!("Skeleton {:?}", data);
    });

    // Running
    nui::run().expect("Failed to run nui");

    // Offline
    nui::release().expect("Failed to release nui");
}
