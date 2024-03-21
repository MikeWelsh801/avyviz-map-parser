# Map Parser

This project contains the code needed to parse and build json tiles for Utah map
regions. The files will be built localy and uploaded to our CDN, where users of 
AvyViz can download and use the maps in our app. Tiling is done to make reading 
and accessing avalanche related geodata more efficient, so it can be done by a phone
offline.

NOTE: In order to build the files from tiffs locally, you must have a directory 
called UAC in the root with subdirectories for each Utah avalanche region. Each
subdirectory must contain the appropriate 3-band elevation/aspect/slope tif file
(e.g. abajos_elevation_aspect_slope.tif). You can use the run.sh build script to
run all file parsing/building code, and it will create the tile directories.
