extern crate nuitrack_rs;

use std::time::Instant;

fn main(){
    // Initialized
    let mut nui = nuitrack_rs::init().expect("Failed to initialize nui");

    // Data Stream Setup
    nui.skeleton_data(|data| {
        //println!("ptr: {:?}", data.skeletons);
        for skeleton in data.skeletons() {
            let joints = skeleton.joints();
            println!("skeletons {:?}", joints);
        }
    }).expect("Failed to add callback");
    
    // Data Stream Setup
    nui.user_data(|data| {
        println!("User Frame: {:?}", data);
        for user in data.users() {
            println!("User {:?}", user);
        }
        println!("frame: {:?}", data.frame());
    }).expect("Failed to add callback");

    // Data Stream Setup
    nui.depth_data(|data| {
        //println!("depth: {:?}", data.frame());
    }).expect("Failed to add callback");

    let mut last = Instant::now();
    let mut count = 0;
    // Data Stream Setup
    nui.color_data(move |data| {
        let now = Instant::now();
        let dif = now.duration_since(last);
        count += 1;
        if dif.as_secs() > 0 {
            last = now;
            println!("FPS: {}", count);
            count = 0;
        }
        //println!("depth: {:?}", data.frame());
    }).expect("Failed to add callback");

    // Running
    let nui = nui.run().expect("Failed to run nui");

    for _ in 0..100 {
        nui.update().expect("Failed to update");
    }

}
