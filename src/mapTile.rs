use serde::Serialize;

/// # Map Tile
/// Contains the tile size and a list of points for aspect (for now) over a grid.
#[derive(Serialize, Clone)]
pub struct MapTile {
    pub elevation: Vec<i32>,
    pub slope: Vec<i32>,
    pub aspect: Vec<i32>,
}

impl MapTile {
    /// # Tile Constructor
    /// # Example
    /// ```
    /// // initialize size to 500 and data to all zeroes.
    /// let tile = MapTile::new(500, vec![0; 500 * 500]);
    /// ```
    pub fn new(elevation: Vec<i32>, slope: Vec<i32>, aspect: Vec<i32>) -> Self {
        MapTile {
            elevation,
            slope,
            aspect,
        }
    }
}
