# nuitrack-rs

This is a wrapper around the nuitrack SDK.

It allows you to get skeleton tracking, RBG and
depth data feeds.

You can also record and playback these data feeds.

## Examples
### Live
```rust
// Initialize nuitrack
let mut nui: Nui<Initialized> = nuitrack_rs::init().expect("Failed to initialize nui");
// Data Stream Setup
nui.skeleton_data(|data| {
    for skeleton in data.skeletons() {
        let joints = skeleton.joints();
        // Use joint data
    }
}).expect("Failed to add callback");

// Data Stream Setup
nui.depth_data(|data| {
    let depth_frame = data.frame();
    // Use depth data
}).expect("Failed to add callback");

// Data Stream Setup
nui.color_data(move |data| {
    let rgb_frame = data.frame();
    // Use depth data
}).expect("Failed to add callback");

// Running
let nui: Nui<Running> = nui.run().expect("Failed to run nui");

// Get 100 updates from nui then quit
// Clean up is done when nui drops
for _ in 0..100 {
    nui.update().expect("Failed to update");
}
```
### Recording
```rust
// This is the same as above but with the following additions

// Create a recorder
let mut recorder = nuitrack_rs::record();

// Create a capture for skeleton data
let skeleton_capture = recorder.new_capture();

// Collect the skeleton data and make it owned.
nui.skeleton_data(move |data| {
let data = data.skeletons()
        .iter()
        .map(|s| s.make_owned())
        .collect();
    skeleton_capture.capture_skeleton(data);
}).expect("Failed to add callback");

// Create a capture for depth
let depth_capture = recorder.new_capture();

// Collect the depth data and make it an owned vec.
nui.depth_data(move |data| {
    depth_capture.capture_depth(data.frame().to_vec());
}).expect("Failed to add callback");

// Create a capture for color
let color_capture = recorder.new_capture();

// Create rows and cols capture
let size_capture = recorder.new_capture();

// Collect the color data and make it an owned vec.
// Collect the frame size
nui.color_data(move |data| {
    size_capture.capture_size((data.rows, data.cols));
    color_capture.capture_color(data.frame().to_vec());
}).expect("Failed to add callback");
// Call write() after each frame.
// Probably on a different thread.
// This will collect data but only write to disk when a chunk of
// data is collected
recorder.write();
// Call flush at the end incase there is some data that isn't wrtitten
recorder.flush();
```
### Playback
```rust
// Playback is similar but there is a few differences.
// No call to run and the types for Nui have changed.
// The callbacks are identical to live though.
// Get the path to the recorded data.
// By default it is in the current_dir().
// Recording have the format recording-{timestamp}.snap
let mut path = env::current_dir().expect("Couldn't find current directory");
path.push("recording-1545179088.snap");

// Create the nui player.
// The second parameter is whether or not to loop the playback.
// Looping is good if the recording is short.
let mut nui: Nui<Player> = nuitrack_rs::playback(path, false).expect("Couldn't create player");

// All the callbacks are identical to live.
nui.skeleton_data(|data| {
    for skeleton in data.skeletons() {
        let joints = skeleton.joints();
        // Use joint data
    }
}).expect("Failed to add callback");

// Data Stream Setup
nui.depth_data(|data| {
    let depth_frame = data.frame();
    // Use depth data
}).expect("Failed to add callback");

// Data Stream Setup
nui.color_data(move |data| {
    let rgb_frame = data.frame();
    // Use depth data
}).expect("Failed to add callback");

// No call to run

// Update is the same as live
for _ in 0..100 {
    nui.update().expect("Failed to update");
}
```

## Installation Guide
__Ubuntu 18.04__
1. Download the SDK from the [Nuitrack website](https://nuitrack.com). 
_If you are short on space you only need the NuitrackSDK/Nuitrack folder (you can delete the rest of the folders)_
2. Unzip it the SDK somewhere eg. `~/nuitrack/` 
3. Follow the ubuntu instractions [here](http://download.3divi.com/Nuitrack/doc/Installation_page.html)
_Make sure to get `libpng12-0`. It's avalible [here](https://packages.ubuntu.com/xenial/amd64/libpng12-0/download) 
if you can't find in apt-get_
4. Set the environment variable `NUI_SDK_DIR` to you sdk root directory. 
eg. `NUI_SDK_DIR = /home/user/nuitrack` _note not `/home/user/nuitrack/Nuitrack`
5. Then you should be able to build your project with `cargo build --release`.
To test try `cargo run --release --example skeleton` with a camera attached.
_Please open an issue if you get stuck on any step_

__Orbtec Astra__
I've only tested this with the Orbec Astra. 
To install the Orbtec SDK follow [these instructions.](https://astra-wiki.readthedocs.io/en/latest/installation.html)

__Recording / Playback__
These features are useful if you need to test when you don't
have access to a camera all the time.
You can record some data from the camera and then play it back
at a later time.
The playback API is very close to the live API. The callbacks are
identical. See the examples for usage.
