use serde::Serialize;

/// # Header
/// Contains information about a map region.
#[derive(Serialize)]
pub struct Header {
    pub region: String,
    pub height: usize,
    pub width: usize,
    pub modelTiepoint: String,
    pub modelPixelScale: String,
    pub boundingBox: String,
    pub tileSize: usize,
}

impl Header {
    /// # Header Constructor
    /// # Example
    /// ```
    /// let header = Header::new("salt-lake", 500, 300, "coordinates", "number", "coordinates");
    /// ```
    pub fn new(
        region: String,
        height: usize,
        width: usize,
        modelTiepoint: String,
        modelPixelScale: String,
        boundingBox: String,
        tileSize: usize,
    ) -> Self {
        Header {
            region,
            height,
            width,
            modelTiepoint,
            modelPixelScale,
            boundingBox,
            tileSize,
        }
    }
}
