extern crate nuitrack_rs;

use nuitrack_rs as nui;
use std::thread;
use std::time::Duration;

fn main(){
    // Initialized
    nui::init().expect("Failed to initialize nui");

    // Data Stream Setup
    let cb = nui::skeleton_data(|data| {
        println!("Skeleton: {:?}", data);
    }).expect("Failed to add callback");
    
    // Data Stream Setup
    let cb2 = nui::depth_data(|data| {
        println!("DepthFrame: {:?}", data);
    }).expect("Failed to add callback");
    
    // Data Stream Setup
    let cb3 = nui::color_data(|data| {
        println!("RGBFrame: {:?}", data);
    }).expect("Failed to add callback");

    // Running
    nui::run().expect("Failed to run nui");

    for _ in 0..1000 {
        nui::update().expect("Failed to update");
    }

    // Offline
    nui::release().expect("Failed to release nui");
}
