use nui::simple::{SkeletonData, Skeleton, DepthFrame, RGBFrame};
use nui::tdv::nuitrack::{Joint, Color3};
use std::slice;

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
