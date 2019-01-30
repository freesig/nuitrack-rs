use std::fs::File;
use std::vec::IntoIter;
use recorder::TimePoint;    
use std::iter::Cycle;   
use std::path::PathBuf; 
use serde_json::Deserializer;
use snap;

pub struct Content {
    data: Play,
}

enum Play {
    Straight(IntoIter<TimePoint>),
    Loop(Cycle<IntoIter<TimePoint>>),
}

impl Iterator for Content {
    type Item = TimePoint;

    fn next(&mut self) -> Option<Self::Item> {
        match self.data {
            Play::Straight(ref mut d) => d.next(),
            Play::Loop(ref mut d) => d.next(),
        }
    }
}

pub fn read_in(path: PathBuf, loop_player: bool) -> Content {
    let file = File::open(path).expect("Couldn't open recording file");
    let snappy = snap::Reader::new(file);
    let content: Vec<TimePoint> = Deserializer::from_reader(snappy)
        .into_iter::<TimePoint>()
        .map(Result::unwrap)
        .collect();

    let data = if loop_player {
        Play::Loop(content.into_iter().cycle())
    } else {
        Play::Straight(content.into_iter())
    };
    Content{ data }

}
