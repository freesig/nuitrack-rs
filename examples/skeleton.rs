extern crate nuitrack_rs;

use nuitrack_rs as nui;

fn main(){
    // Initialized
    nui::init().expect("Failed to initialize nui");

    // Data Stream Setup
    let _cb = nui::skeleton_data(|data| {
        println!("Skeleton: {:?}", data);
    }).expect("Failed to add callback");
    
    // Data Stream Setup
    let _cb2 = nui::depth_data(|data| {
        println!("DepthFrame: {:?}", data);
    }).expect("Failed to add callback");
    
    // Data Stream Setup
    let _cb3 = nui::color_data(|data| {
        println!("RGBFrame: {:?}", data);
    }).expect("Failed to add callback");

    // Running
    nui::run().expect("Failed to run nui");

    for _ in 0..10 {
        nui::update().expect("Failed to update");
    }

    // Offline
    nui::release().expect("Failed to release nui");
}
