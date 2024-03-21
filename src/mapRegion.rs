use serde::Deserialize;

/// # Original File
/// This is currently just from json, but I'm working on deserializing tiffs.
#[derive(Deserialize, Debug)]
pub struct MapRegion {
    pub region: String,
    pub height: usize,
    pub width: usize,
    pub modelTiepoint: String,
    pub modelPixelScale: String,
    pub boundingBox: String,
    pub data: String,
}
