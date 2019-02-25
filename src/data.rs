use nui::simple::{SkeletonData, Skeleton, DepthFrame, RGBFrame, UserFrame};
use nui::tdv::nuitrack::{Joint, Color3, Vector3, Orientation, User};
use nui_import::root;
use std::slice;

#[derive(Serialize, Deserialize, Clone)]
pub struct SkeletonFeed {
    pub id: i32,
    #[serde(with = "joint_vec")]
    pub joints: Vec<Joint>,
}

impl SkeletonData {
    pub fn skeletons(&self) -> &[Skeleton] {
        unsafe {
            slice::from_raw_parts(self.skeletons, self.len)
        }
    }
}

impl Skeleton {
    pub fn joints(&self) -> &[Joint] {
        unsafe {
            slice::from_raw_parts(self.joints, self.num_joints)
        }
    }

    pub fn make_owned(&self) -> SkeletonFeed {
        let joints = unsafe {
            slice::from_raw_parts(self.joints, self.num_joints)
        }.to_vec();
        SkeletonFeed{ id: self.id, joints }
    }
}

impl DepthFrame {
    pub fn frame(&self) -> &[u16] {
        unsafe {
            slice::from_raw_parts(self.data, (self.rows * self.cols) as usize)
        }
    }
}

impl RGBFrame {
    pub fn frame(&self) -> &[Color3] {
        unsafe {
            slice::from_raw_parts(self.data, (self.rows * self.cols) as usize)
        }
    }
}

impl UserFrame {
    pub fn users(&self) -> &[User] {
        unsafe {
            slice::from_raw_parts(self.users, self.num_users)
        }
    }

    pub fn frame(&self) -> &[u16] {
        unsafe {
            slice::from_raw_parts(self.data, (self.rows * self.cols) as usize)
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Vector3")]
pub struct Vector3Def {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Orientation")]
pub struct OrientationDef {
    /// @brief Flattened 3x3 rotation matrix.
    pub matrix: [f32; 9usize],
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Color3")]
pub struct Color3Def {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Joint")]
pub struct JointDef {
    /// @brief %Joint type.
    pub type_: root::tdv::nuitrack::JointType,
    /// @brief %Joint confidence from 0.0 to 1.0. Larger value means more confident joint.
    pub confidence: f32,
    /// @brief %Joint position in real world coordinates.
    #[serde(with = "Vector3Def")]
    pub real: root::tdv::nuitrack::Vector3,
    /// @brief %Joint position in normalized projective coordinates
    /// (x, y from 0.0 to 1.0, z is real).
    #[serde(with = "Vector3Def")]
    pub proj: root::tdv::nuitrack::Vector3,
    /// @brief %Joint orientation.
    #[serde(with = "OrientationDef")]
    pub orient: root::tdv::nuitrack::Orientation,
}

mod joint_vec {
    use super::JointDef;
    use nui::tdv::nuitrack::Joint;
    use serde::{Serializer, Deserialize, Deserializer};
    pub fn serialize<S>(array: &[Joint], serializer: S) -> Result<S::Ok, S::Error>
        where
        S: Serializer,
        {
            #[derive(Serialize)]
            struct W<'a>(#[serde(with = "JointDef")] &'a Joint);

            let map = array.iter().map(|& ref n| W(n));
            serializer.collect_seq(map)
        }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Joint>, D::Error>
        where
        D: Deserializer<'de>,
        {
            #[derive(Deserialize)]
            struct W(#[serde(with = "JointDef")] Joint);

            let joints = Vec::<W>::deserialize(deserializer)?;
            Ok(joints.into_iter().map(|n| n.0).collect())
        }
}

pub mod color3_vec {
    use super::Color3Def;
    use nui::tdv::nuitrack::Color3;
    use serde::{Serializer, Deserialize, Deserializer};
    pub fn serialize<S>(array: &[Color3], serializer: S) -> Result<S::Ok, S::Error>
        where
        S: Serializer,
        {
            #[derive(Serialize)]
            struct W<'a>(#[serde(with = "Color3Def")] &'a Color3);

            let map = array.iter().map(|& ref n| W(n));
            serializer.collect_seq(map)
        }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Color3>, D::Error>
        where
        D: Deserializer<'de>,
        {
            #[derive(Deserialize)]
            struct W(#[serde(with = "Color3Def")] Color3);

            let colors = Vec::<W>::deserialize(deserializer)?;
            Ok(colors.into_iter().map(|n| n.0).collect())
        }
}

impl From<&mut Vec<Skeleton>> for SkeletonData {
    fn from(item: &mut Vec<Skeleton>) -> Self {
        let len = item.len();
        let skeletons = item.as_mut_ptr();
        SkeletonData{ len, skeletons }
    }
}

pub fn feed_to_ptr(item: &Vec<SkeletonFeed>) -> Vec<Skeleton> {
    item.into_iter()
        .map(|sf| {
            Skeleton{
                id: sf.id,
                num_joints: sf.joints.len(),
                joints: sf.joints.as_ptr(),
            }
        })
    .collect()
}

impl From<(&mut Vec<u16>, i32, i32)> for DepthFrame {
    fn from(item: (&mut Vec<u16>, i32, i32)) -> Self {
        let rows = item.1;
        let cols = item.2;
        let data = item.0.as_ptr();
        DepthFrame{rows, cols, id: 0, data, time_stamp: 0}
    }
}

impl From<(&mut Vec<Color3>, i32, i32)> for RGBFrame {
    fn from(item: (&mut Vec<Color3>, i32, i32)) -> Self {
        let rows = item.1;
        let cols = item.2;
        let data = item.0.as_ptr();
        RGBFrame{rows, cols, id: 0, data, time_stamp: 0}
    }
}
