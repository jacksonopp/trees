use crate::lines::solid::SolidLine;

#[derive(Debug)]
pub struct SkyLine {
    pub lines: Vec<SolidLine>,
}

impl SkyLine {
    pub fn new(y: f32) -> Self {
        let mut lines = vec![];
        Self { lines }
    }
}

#[derive(Debug)]
pub struct Sky {
    pub skyline: Vec<SkyLine>,
}
