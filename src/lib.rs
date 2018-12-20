//! This is a wrapper around the nuitrack SDK.
//!
//! It allows you to get skeleton tracking, RBG and
//! depth data feeds.
//!
//! You can also record and playback these data feeds.
//!
//! # Examples
//! ## Live
//! ```rust
//! # use nuitrack_rs::{self, Nui, Initialized, Running};
//! // Initialize nuitrack
//! let mut nui: Nui<Initialized> = nuitrack_rs::init().expect("Failed to initialize nui");
//! // Data Stream Setup
//! nui.skeleton_data(|data| {
//!     for skeleton in data.skeletons() {
//!         let joints = skeleton.joints();
//!         // Use joint data
//!     }
//! }).expect("Failed to add callback");
//!
//! // Data Stream Setup
//! nui.depth_data(|data| {
//!     let depth_frame = data.frame();
//!     // Use depth data
//! }).expect("Failed to add callback");
//!
//! // Data Stream Setup
//! nui.color_data(move |data| {
//!     let rgb_frame = data.frame();
//!     // Use depth data
//! }).expect("Failed to add callback");
//!
//! // Running
//! let nui: Nui<Running> = nui.run().expect("Failed to run nui");
//!
//! // Get 100 updates from nui then quit
//! // Clean up is done when nui drops
//! for _ in 0..100 {
//!     nui.update().expect("Failed to update");
//! }
//! ```
//! ## Recording
//! ```rust
//! # use nuitrack_rs::{self, Nui, Initialized, Running};
//! # let mut nui: Nui<Initialized> = nuitrack_rs::init().expect("Failed to initialize nui");
//! // This is the same as above but with the following additions
//!
//! // Create a recorder
//! let mut recorder = nuitrack_rs::record();
//! 
//! // Create a capture for skeleton data
//! let skeleton_capture = recorder.new_capture();
//!
//! // Collect the skeleton data and make it owned.
//! nui.skeleton_data(move |data| {
//! let data = data.skeletons()
//!         .iter()
//!         .map(|s| s.make_owned())
//!         .collect();
//!     skeleton_capture.capture_skeleton(data);
//! }).expect("Failed to add callback");
//!
//! // Create a capture for depth
//! let depth_capture = recorder.new_capture();
//!
//! // Collect the depth data and make it an owned vec.
//! nui.depth_data(move |data| {
//!     depth_capture.capture_depth(data.frame().to_vec());
//! }).expect("Failed to add callback");
//!
//! // Create a capture for color
//! let color_capture = recorder.new_capture();
//!
//! // Create rows and cols capture
//! let size_capture = recorder.new_capture();
//!
//! // Collect the color data and make it an owned vec.
//! // Collect the frame size
//! nui.color_data(move |data| {
//!     size_capture.capture_size((data.rows, data.cols));
//!     color_capture.capture_color(data.frame().to_vec());
//! }).expect("Failed to add callback");
//! # let nui: Nui<Running> = nui.run().expect("Failed to run nui");
//! // Call write() after each frame.
//! // Probably on a different thread.
//! // This will collect data but only write to disk when a chunk of
//! // data is collected
//! recorder.write();
//! // Call flush at the end incase there is some data that isn't wrtitten
//! recorder.flush();
//! ```
//! ## Playback 
//! ```rust
//! // Playback is similar but there is a few differences.
//! // No call to run and the types for Nui have changed.
//! // The callbacks are identical to live though.
//! # use nuitrack_rs::{self, Nui, Player};
//! # use std::env;
//! // Get the path to the recorded data.
//! // By default it is in the current_dir().
//! // Recording have the format recording-{timestamp}.snap
//! let mut path = env::current_dir().expect("Couldn't find current directory");
//! path.push("recording-1545179088.snap");
//! 
//! // Create the nui player.
//! // The second parameter is whether or not to loop the playback.
//! // Looping is good if the recording is short.
//! let mut nui: Nui<Player> = nuitrack_rs::playback(path, false).expect("Couldn't create player");
//!
//! // All the callbacks are identical to live.
//! nui.skeleton_data(|data| {
//!     for skeleton in data.skeletons() {
//!         let joints = skeleton.joints();
//!         // Use joint data
//!     }
//! }).expect("Failed to add callback");
//!
//! // Data Stream Setup
//! nui.depth_data(|data| {
//!     let depth_frame = data.frame();
//!     // Use depth data
//! }).expect("Failed to add callback");
//!
//! // Data Stream Setup
//! nui.color_data(move |data| {
//!     let rgb_frame = data.frame();
//!     // Use depth data
//! }).expect("Failed to add callback");
//!
//! // No call to run
//!
//! // Update is the same as live
//! for _ in 0..100 {
//!     nui.update().expect("Failed to update");
//! }
//! ```

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate snap;

mod nui_import;
mod error_conversion;
mod errors;
mod callbacks;
mod data;
mod recorder;
mod player;

use errors::NuiError;
use error_conversion::NuiResult;
use nui_import::root as nui;
use std::marker::PhantomData;
use std::path::PathBuf;  
use player::Content;
pub use nui::tdv::nuitrack::Color3;
pub use nui::simple::{SkeletonData, DepthFrame, RGBFrame, Skeleton};
pub use callbacks::CallBack;
pub use recorder::{Recorder, TimePoint};

pub struct Nui<T> {
    state: T,
    callbacks: Vec<CallBackHolder>,
}

unsafe impl<T> Send for Nui<T> {}

pub struct State<T> {
    phantom: PhantomData<T>,
}
pub struct Initialized {
    clean_up: fn() -> (),
}
pub struct Running;
pub struct Offline;
pub struct Player {
    content: Content,
}

enum CallBackHolder {
    Skeleton(CallBack<SkeletonData>),
    Depth(CallBack<DepthFrame>),
    Color(CallBack<RGBFrame>),
    PSkeleton(Box<FnMut(SkeletonData) -> () + Send + 'static>),
    PDepth(Box<FnMut(DepthFrame) -> () + Send + 'static>),
    PColor(Box<FnMut(RGBFrame) -> () + Send + 'static>),
}

pub fn init() -> Result<Nui<Initialized>, NuiError> {
    Nui::<Offline>::new()
}

pub fn record() -> Recorder {
    Recorder::new()
}

pub fn playback(path: PathBuf, loop_player: bool) -> Result<Nui<Player>, NuiError> {
    Nui::<Player>::new(path, loop_player)
}

impl Nui<Offline> {
    pub fn new() -> Result<Nui<Initialized>, NuiError> {
        unsafe{
            nui::nui_init()
                .to_result()
                .map(|_|Nui{state: Initialized{clean_up: release_nui}, callbacks: Vec::new()})
        }
    }
}

impl Nui<Player> {
    pub fn new(path: PathBuf, loop_player: bool) -> Result<Nui<Player>, NuiError> {
        let content = player::read_in(path, loop_player);
        Ok(Nui{state: Player{content}, callbacks: Vec::new()})
    }
    
    pub fn skeleton_data<F>(&mut self, cb: F)
        -> Result<(), NuiError>
        where
        F: FnMut(SkeletonData) -> () + Send + 'static
        {
            self.callbacks.push(CallBackHolder::PSkeleton(Box::new(cb)));
            Ok(())

        }

    pub fn depth_data<F>(&mut self, cb: F)
        -> Result<(), NuiError>
        where
        F: FnMut(DepthFrame) -> () + Send + 'static
        {
            self.callbacks.push(CallBackHolder::PDepth(Box::new(cb)));
            Ok(())
        }

    pub fn color_data<F>(&mut self, cb: F)
        -> Result<(), NuiError>
        where
        F: FnMut(RGBFrame) -> () + Send + 'static
        {
            self.callbacks.push(CallBackHolder::PColor(Box::new(cb)));
            Ok(())
        }
    
    pub fn update(&mut self) -> Result<(), NuiError> {
        if let Some(content) = self.state.content.next() {
            let TimePoint {
                skeleton,
                mut depth,
                mut color,
                rows,
                cols,
            } = content;
            let mut skeletons: Vec<Skeleton> = data::feed_to_ptr(&skeleton);
            for cb in self.callbacks.iter_mut() {
                match cb {
                    CallBackHolder::PSkeleton(cb) => (*cb)((&mut skeletons).into()),
                    CallBackHolder::PDepth(cb) => (*cb)((&mut depth, rows, cols).into()),
                    CallBackHolder::PColor(cb) => (*cb)((&mut color, rows, cols).into()),
                    _ => eprintln!("Wrong type of playback callback"),
                }
            }
        }
        Ok(())
    }
}

impl Nui<Initialized> {
    pub fn skeleton_data<F>(&mut self, cb: F)
        -> Result<(), NuiError>
        where
        F: FnMut(SkeletonData) -> () + Send + 'static
        {
            CallBack::<SkeletonData>::new(cb)
                .map(|cbw| self.callbacks.push(CallBackHolder::Skeleton(cbw)))

        }

    pub fn depth_data<F>(&mut self, cb: F)
        -> Result<(), NuiError>
        where
        F: FnMut(DepthFrame) -> () + Send + 'static
        {
            CallBack::<DepthFrame>::new(cb)
                .map(|cbw| self.callbacks.push(CallBackHolder::Depth(cbw)))
        }

    pub fn color_data<F>(&mut self, cb: F)
        -> Result<(), NuiError>
        where
        F: FnMut(RGBFrame) -> () + Send + 'static
        {
            CallBack::<RGBFrame>::new(cb)
                .map(|cbw| self.callbacks.push(CallBackHolder::Color(cbw))) 
        }

    /// Sets the cameras rotation in degrees.
    /// Call after init() and before run()
    pub fn set_camera_rotation(&self, rotation: i32) -> Result<(), NuiError> {
        unsafe {
            nui::nui_set_rotation(rotation)
                .to_result()
                .map(|_|())
        }
    }

    pub fn run(mut self) -> Result<Nui<Running>, NuiError> {
        unsafe{
            fn none(){};
            self.state.clean_up = none;
            nui::nui_run()
                .to_result()
                .map(|_|Nui{state: Running{}, callbacks: self.callbacks})
        }
    }
}

impl Nui<Running> {
    pub fn update(&self) -> Result<(), NuiError> {
        unsafe{
            nui::nui_update().to_result().map(|_|())
        }
    }
}

impl Drop for Running {
    fn drop(&mut self) {
        release_nui();
    }
}

impl Drop for Initialized {
    fn drop(&mut self) {
        (self.clean_up)();
    }
}

fn release_nui() {
    unsafe{
        match nui::nui_release().to_result() {
            Ok(_) => (),
            Err(e) => eprintln!("Error releasing nuitrack: {}", e),
        }
    }
}
