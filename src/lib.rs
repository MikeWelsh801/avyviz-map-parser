#![allow(non_snake_case)]
#![allow(dead_code)]
use crate::{mapHeader::Header, mapRegion::MapRegion, mapTile::MapTile};
use std::{
    fs::{self, File},
    io::Write,
};

mod mapHeader;
mod mapRegion;
mod mapTile;

/* Constants used for building the tiles. We can tweak align to see if it impacts
 * read perfomance */
const ALIGN: usize = 500;
const TILESIZE: usize = ALIGN * ALIGN;

/// Creates tiled json files from UAC region data. Assumes there are is a folder
/// of json files in 'UAC_DATA/<region>/<region>_<aspect/elevation/slope>.json'
/// from the project root. It creates and writes into to a folder 'UAC_TILED_DATA'
/// builds subdirectory named <region> containing all the tiled file jsons/headers
/// for the region.
///
/// * `region`: a UAC region (e.g. salt-lake)
pub fn build_region_files(region: &str) {
    let aspect_json: MapRegion = serde_json::from_slice(
        &fs::read(format!("UAC_DATA/{}/{}_aspect.json", region, region))
            .expect("Couln't read aspect file."),
    )
    .expect("Missing or incorrect fields in apsect");

    let elevation_json: MapRegion = serde_json::from_slice(
        &fs::read(format!("UAC_DATA/{}/{}_elevation.json", region, region))
            .expect("Couln't read elevation file."),
    )
    .expect("Missing or incorrect fields in elevation");

    let slope_json: MapRegion = serde_json::from_slice(
        &fs::read(format!("UAC_DATA/{}/{}_slope.json", region, region))
            .expect("Couln't read slope file."),
    )
    .expect("Missing or incorrect fields in slope");

    // double check that the dimensions are equal
    assert_eq!(aspect_json.height, elevation_json.height);
    assert_eq!(aspect_json.height, slope_json.height);
    assert_eq!(aspect_json.width, elevation_json.width);
    assert_eq!(aspect_json.width, slope_json.width);

    // The size of the padded NxN grid (i.e. grid_size = N)
    let grid_size = get_padded_size(aspect_json.height, aspect_json.width);
    // how much to pad the rows and collumns by
    let col_pad = grid_size - aspect_json.width;
    let row_pad = (grid_size - aspect_json.height) * grid_size;

    // padd the grid and build tiles, these will be returned as integer arrays
    let aspect_arr = extract_array(&aspect_json.data, aspect_json.width, col_pad, row_pad);
    let elevation_arr = extract_array(&elevation_json.data, elevation_json.width, col_pad, row_pad);
    let slope_arr = extract_array(&slope_json.data, slope_json.width, col_pad, row_pad);

    let tile_arr = build_tiles(&aspect_arr, &elevation_arr, &slope_arr, grid_size);

    // tile_arr.iter().for_each(|tile| {
    //     pretty_print(&tile.slope, ALIGN);
    // });

    let num_tiles = grid_size / ALIGN;

    let header = Header::new(
        aspect_json.region,
        aspect_json.height,
        aspect_json.width,
        aspect_json.modelTiepoint,
        aspect_json.modelPixelScale,
        aspect_json.boundingBox,
        ALIGN,
    );

    write_json_tiles(String::from(region), &tile_arr, num_tiles, &header);
}

/// Writes the tiles and the header into files in a region folder.
fn write_json_tiles(region: String, tile_arr: &[MapTile], num_tiles: usize, header: &Header) {
    // loop over all of the tiles, serialize them as json, and write to a file
    (0..num_tiles).for_each(|i| {
        (0..num_tiles).for_each(|j| {
            let start = (i * ALIGN, j * ALIGN);
            let end = (start.0 + ALIGN - 1, start.1 + ALIGN - 1);

            /* Testing the edges of tiles are correct for abajos */
            // if 4710 >= start.1 && 4710 <= end.1 {
            //     pretty_print(&tile_arr[i * num_tiles + j].slope, ALIGN);
            // }

            // the name has the starting and ending position for easy reading
            // when we need to locate a tile
            let mut file = File::create(format!(
                "UAC_TILED_DATA/{}/{}_({},{})_({},{}).json",
                region, region, start.1, start.0, end.1, end.0
            ))
            .expect("error creating tile file");

            file.write_all(
                &serde_json::to_vec(&tile_arr[i * num_tiles + j])
                    .expect(&format!("Error parsing tile at {}, {}", i, j)),
            )
            .expect("error writing tile file");
        });
    });
    // create the header file
    let mut file = File::create(format!("UAC_TILED_DATA/{}/{}_header.json", region, region))
        .expect("error creating header file");

    file.write_all(&serde_json::to_vec(&header).expect("error converting header to json"))
        .expect("error writing header file");
}

/// Creates tiles over the input grid and returns them as a vector.
///
/// Uses global variables TILESIZE and ALIGN to determine the size of the
/// tiles.
///
/// # Example
/// ```
/// // map region must be grid_size * grid_size in length.
/// let tiles: Vec<MapTile> = build_tiles(&map_region, grid_size);
/// ```
fn build_tiles(
    apsect_arr: &[i32],
    elevation_arr: &[i32],
    slope_arr: &[i32],
    grid_size: usize,
) -> Vec<MapTile> {
    // just initialize all the tiles with zero data arrays
    let num_tiles = apsect_arr.len() / TILESIZE;
    let mut tile_arr =
        vec![MapTile::new(vec![0; TILESIZE], vec![0; TILESIZE], vec![0; TILESIZE]); num_tiles];

    // loop over the original array and fill up all of the tile data
    (0..grid_size).for_each(|i| {
        (0..grid_size).for_each(|j| {
            let grid_idx = i * grid_size + j;
            let tile_idx = (i % ALIGN) * ALIGN + (j % ALIGN);
            let tile_num = (i / ALIGN) * (grid_size / ALIGN) + (j / ALIGN);

            tile_arr[tile_num].aspect[tile_idx] = apsect_arr[grid_idx];
            tile_arr[tile_num].elevation[tile_idx] = elevation_arr[grid_idx];
            tile_arr[tile_num].slope[tile_idx] = slope_arr[grid_idx];
        });
    });
    tile_arr
}

/// Extracts a padded array of i32 values from the input data string. Padded array
/// means that the array grid's width and height are padded with input params.
/// -1's are padded allong rows and columns to get the desired width and height.
///
/// Needs to know the width of the original grid.
///
/// # Example
/// ```
/// let arr: Vec<i32> = extract_array(&json.data, json.width, col_pad, row_pad);
/// ```
fn extract_array(data: &str, width: usize, col_pad: usize, row_pad: usize) -> Vec<i32> {
    let mut data_array = Vec::new();

    data.split(',').enumerate().for_each(|(index, num)| {
        // add column padding before starting a new row
        if index % width == 0 && index != 0 {
            padd_array(col_pad, &mut data_array);
        }
        data_array.push(num.parse().expect("found element that wasn't a number"));
    });
    // add last column's padding
    padd_array(col_pad, &mut data_array);
    // add row padding
    padd_array(row_pad, &mut data_array);
    data_array
}

/// Debugging helper function that prints the "edges" of the
/// tile to check that the padding and tiling was correct.
///
/// Caller needs to ensure that this is an "edge" tile
///
/// * `data_array`: the tile's data
/// * `grid_size`: the NxN size of the tile
fn pretty_print(data_array: &[i32], grid_size: usize) {
    (0..grid_size).for_each(|i| {
        (grid_size - 300..grid_size - 220).for_each(|j| {
            let data = data_array[i * grid_size + j];
            match data {
                -1 => print!("."),
                _ => print!("#"),
            };
        });
        println!();
    })
}

/// Adds padding -1's to the input vector.
fn padd_array(padding: usize, array: &mut Vec<i32>) {
    (0..padding).for_each(|_| array.push(-1));
}

/// Calculates how much padding to add given the height and width of an input
/// grid. Padding is so that grid becomes a square of NxN where N is a multiple
/// of ALIGN (global variable).
fn get_padded_size(height: usize, width: usize) -> usize {
    if height > width {
        height + (ALIGN - (height % ALIGN))
    } else {
        width + (ALIGN - (width % ALIGN))
    }
}
