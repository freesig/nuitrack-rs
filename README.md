# nuitrack-rs

This is a wrapper around the nuitrack SDK.

It allows you to get skeleton tracking, RBG and
depth data feeds.

## Examples
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
