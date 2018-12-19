extern crate nuitrack_rs;
use std::sync::mpsc::sync_channel;

fn main(){
    // Initialized
    let mut nui = nuitrack_rs::init().expect("Failed to initialize nui");
    let mut recorder = nuitrack_rs::record();

    let skeleton_capture = recorder.new_capture();
    // Data Stream Setup
    nui.skeleton_data(move |data| {
        let data = data.skeletons()
            .iter()
            .map(|s| s.make_owned())
            .collect();
        skeleton_capture.capture_skeleton(data);
    }).expect("Failed to add callback");
    
    let depth_capture = recorder.new_capture();
    // Data Stream Setup
    nui.depth_data(move |data| {
        depth_capture.capture_depth(data.frame().to_vec());
    }).expect("Failed to add callback");

    let color_capture = recorder.new_capture();
    let size_capture = recorder.new_capture();
    // Data Stream Setup
    nui.color_data(move |data| {
        size_capture.capture_size((data.rows, data.cols));
        color_capture.capture_color(data.frame().to_vec());
    }).expect("Failed to add callback");

    // Running
    let nui = nui.run().expect("Failed to run nui");

    let (control_tx, control_rx) = sync_channel(0);

    let recorder_join = std::thread::spawn(move|| {
        let mut control = control_rx.try_iter();
        while control.next().is_none() {
            recorder.write();
        }
        recorder.flush();
    });

    for _ in 0..5 {
        nui.update().expect("Failed to update");
    }
    control_tx.send(()).expect("Failed to signal recorder thread");
    recorder_join.join().expect("Failed to join recorder thread");

}
