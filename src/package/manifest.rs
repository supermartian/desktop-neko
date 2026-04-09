use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Manifest {
    pub package: PackageInfo,
    pub sprite: SpriteInfo,
    pub animations: Vec<AnimationInfo>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub author: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SpriteInfo {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AnimationInfo {
    pub name: String,
    pub sheet: String,
    pub frames: usize,
    pub fps: Option<u32>,
}
