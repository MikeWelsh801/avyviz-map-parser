#!/bin/bash
regions=( abajos logan moab ogden provo salt-lake skyline uintas )
jsonDir=UAC_DATA
tileDir=UAC_TILED_DATA

# build appropriate directories
echo "checking and building correct directores..."
if [ ! -d "$jsonDir" ]; then
  echo "creating $jsonDir"
  mkdir "$jsonDir"
fi

if [ ! -d "$tileDir" ]; then
  echo "creating $tileDir"
  mkdir "$tileDir"
fi

for region in "${regions[@]}"; do
  if [ ! -d "$jsonDir/$region" ]; then
    echo "creating $jsonDir/$region"
    mkdir "$jsonDir/$region"
  fi

  if [ ! -d "$tileDir/$region" ]; then
    echo "creating $tileDir/$region"
    mkdir "$tileDir/$region"
  fi
done

# run the build scripts
echo "building json files from tiffs..."
node test.js
echo "json regions done"
echo "tiling json files and building region files..."
cargo run --release
echo "tiles done"
