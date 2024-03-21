use map_parser::build_region_files;

/// Entry point of map parser program. Builds tile jsons for every region in 
/// the vector below.
fn main() {
    vec![
        "abajos",
        "logan",
        "moab",
        "ogden",
        "provo",
        "salt-lake",
        "skyline",
        "uintas",
    ]
    .iter()
    .for_each(|region| build_region_files(region));
}
