extern crate nuitrack_rs;

use nuitrack_rs as nui;

fn main(){
    // Initialized
    nui::init().expect("Failed to initialize nui");

    // Data Stream Setup
    let _cb = nui::skeleton_data(|data| {
        println!("ptr: {:?}", data.skeletons);
        for skeleton in data.skeletons() {
            let joints = skeleton.joints();
            println!("skeletons {:?}", joints);
        }
    }).expect("Failed to add callback");
    
    // Data Stream Setup
    let _cb2 = nui::depth_data(|data| {
        //println!("depth: {:?}", data.frame());
    }).expect("Failed to add callback");

    // Data Stream Setup
    let _cb3 = nui::color_data(|data| {
        //println!("depth: {:?}", data.frame());
    }).expect("Failed to add callback");

    // Running
    nui::run().expect("Failed to run nui");

    for _ in 0..10 {
        nui::update().expect("Failed to update");
    }

    // Offline
    nui::release().expect("Failed to release nui");
}
