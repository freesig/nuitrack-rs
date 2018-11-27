use nui::simple::SkeletonData;

impl Iterator for SkeletonData {
    type Item = SkeletonData;

    fn next(&mut self) -> Option<Self::Item> {
        // Need to keep track of current pointer in the 
        // pointers and move it along
        unimplemented!()
    }
}
