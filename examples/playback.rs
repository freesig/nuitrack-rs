extern crate nuitrack_rs;

use std::env;

fn main(){
    // Initialized
    let mut path = env::current_dir().expect("Couldn't find current directory");
    path.push("recording-1545192362.snap");

    let mut nui = nuitrack_rs::playback(path, false).expect("Couldn't create player");

    // Data Stream Setup
    nui.skeleton_data(|data| {
        //println!("ptr: {:?}", data.skeletons);
        for skeleton in data.skeletons() {
            let joints = skeleton.joints();
            println!("skeletons {:?}", joints);
        }
    }).expect("Failed to add callback");
    
    // Data Stream Setup
    nui.depth_data(|data| {
        println!("depth: {:?}", data.frame());
    }).expect("Failed to add callback");

    // Data Stream Setup
    nui.color_data(move |data| {
        println!("color: {:?}", data.frame());
    }).expect("Failed to add callback");

    for _ in 0..100 {
        nui.update().expect("Failed to update");
    }

}
