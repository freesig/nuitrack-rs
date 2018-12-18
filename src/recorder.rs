use std::time::{SystemTime, UNIX_EPOCH};
use std::env;
use std::fs::File;
use std::sync::mpsc::{channel, Receiver, Sender};
use data::{SkeletonFeed, color3_vec};
use nui::tdv::nuitrack::Color3;

const BUFFER_SIZE: usize = 50;

pub struct Recorder {
    captures: Vec<Receiver<DataMsg>>,
    file: File,
    data: Vec<TimePoint>,
}

pub struct Capture {
    tx: Sender<DataMsg>,
}

enum DataMsg {
    Skeleton(Vec<SkeletonFeed>),
    Depth(Vec<u16>),
    Color(Vec<Color3>),
}

#[derive(Serialize, Deserialize)]
struct TimePoint {
    skeleton: Vec<SkeletonFeed>,
    depth: Vec<u16>,
    #[serde(with = "color3_vec")]
    color: Vec<Color3>,
}

impl Recorder {
    pub fn new() -> Self {
        let mut path = env::current_dir().expect("Could find current directory");
        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("failed to get time").as_secs();
        path.push(format!("recording-{}.json", now));
        let file = File::create(path).expect("failed to create file");
        Recorder{ 
            captures: Vec::new(),
            file,
            data: Vec::with_capacity(BUFFER_SIZE),
        }
    }

    pub fn new_capture(&mut self) -> Capture {
        let (tx, rx) = channel();
        self.captures.push(rx);
        Capture{ tx }
    }

    pub fn write(&mut self) {
        if self.data.len() > BUFFER_SIZE {
            for d in self.data.iter() {
                serde_json::to_writer(&self.file, d).expect("failed to write to file");
            }
            self.data.clear();
        }
        let mut sk_data = None;
        let mut d_data = None;
        let mut c_data = None;
        for c in self.captures.iter() {
            let msg = c.recv().expect("failed to recv capture");
            match msg {
                DataMsg::Skeleton(s) => sk_data = Some(s),
                DataMsg::Depth(d) => d_data = Some(d),
                DataMsg::Color(c) => c_data = Some(c),
            }
        }
        self.data.push(
            TimePoint {
                skeleton: sk_data.expect("did recieve skeleton capture"),
                depth: d_data.expect("did recieve skeleton capture"),
                color: c_data.expect("did recieve skeleton capture"),
            });
    }

    pub fn flush(&mut self) {
        for d in self.data.iter() {
            serde_json::to_writer(&self.file, d).expect("failed to write to file");
        }
        self.data.clear();
    }
}

impl Capture {
    pub fn capture_skeleton(&self, data: Vec<SkeletonFeed>) {
        self.tx.send(DataMsg::Skeleton(data)).expect("Failed to send skeleton data");
    }
    
    pub fn capture_depth(&self, data: Vec<u16>) {
        self.tx.send(DataMsg::Depth(data)).expect("Failed to send depth data");
    }
    
    pub fn capture_color(&self, data: Vec<Color3>) {
        self.tx.send(DataMsg::Color(data)).expect("Failed to send color data");
    }
}
