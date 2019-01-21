
/// A readable form of the joint type.
///
/// Can be converted from `u32` via the `from_u32` method.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum JointType {
    None = 0,
    Head,
    Neck,
    Torso,
    Waist,
    LeftCollar,
    LeftShoulder,
    LeftElbow,
    LeftWrist,
    LeftHand,
    LeftFingertip, // Possibly unused in current version.
    RightCollar,
    RightShoulder,
    RightElbow,
    RightWrist,
    RightHand,
    RightFingertip, // Possibly unused in current version.
    LeftHip,
    LeftKnee,
    LeftAnkle,
    LeftFoot, // Possibly unused in current version.
    RightHip,
    RightKnee,
    RightAnkle,
    RightFoot, // Possibly unused in current version.
}

impl JointType {
    /// Convert from the nuitrack unsigned integer representation.
    ///
    /// Returns `None` if the integer doesn't match any valid joint types.
    pub fn from_u32(u: u32) -> Option<Self> {
        let ty = match u {
            0 => JointType::None,
            1 => JointType::Head,
            2 => JointType::Neck,
            3 => JointType::Torso,
            4 => JointType::Waist,
            5 => JointType::LeftCollar,
            6 => JointType::LeftShoulder,
            7 => JointType::LeftElbow,
            8 => JointType::LeftWrist,
            9 => JointType::LeftHand,
            10 => JointType::LeftFingertip,
            11 => JointType::RightCollar,
            12 => JointType::RightShoulder,
            13 => JointType::RightElbow,
            14 => JointType::RightWrist,
            15 => JointType::RightHand,
            16 => JointType::RightFingertip,
            17 => JointType::LeftHip,
            18 => JointType::LeftKnee,
            19 => JointType::LeftAnkle,
            20 => JointType::LeftFoot,
            21 => JointType::RightHip,
            22 => JointType::RightKnee,
            23 => JointType::RightAnkle,
            24 => JointType::RightFoot,
            _ => return None,
        };
        Some(ty)
    }
}
