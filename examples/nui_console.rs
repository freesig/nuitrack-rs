extern crate nuitrack_rs as nui;

fn main() {
    nui::initialize().expect("failed to initialize:");
    nui::create_hand_tracker().expect("failed to create hand tracker");
    println!("finished");
}
