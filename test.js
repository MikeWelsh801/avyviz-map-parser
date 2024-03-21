// import axios from "axios";
// import tiff from "tiff";
// import * as fs from "fs";

// async function tiffTest() {
//   let path = "UAC/salt-lake";
//   let fileName = `salt-lake_elevation_aspect_slope.tif`;

//   try {
//     // let req = await axios.get(`https://storage.bunnycdn.com/avyviz/${path}/${fileName}`, {
//     //   headers: { AccessKey: "e7eee261-e38b-40bd-bc80414f3fef-c19c-4945" },
//     //   responseType: "arraybuffer", // Use "arraybuffer" instead of "image/tiff"
//     // });

//     // let data = req.data;

//     const file = fs.readFileSync("./salt_lake_region_elevation.tif");
//     // console.log(file.byteLength);
//     // Now you can use the tiff package to decode the TIFF data
//     // We can do this in the browser, too,  the `decode` function takes an ArrayBuffer

//     const image = tiff.decode(file);
//     // get the first IFD block
//     const block = image[0];
//     const pixels = block.data;

//     // let's see what tags apply
//     const fields = block.fields;

//     console.log(fields);
//   } catch (error) {
//     console.error("Error:", error.message);
//   }
// }

// // Call the function
// tiffTest();

import { fromFile } from "geotiff";
import axios from "axios";
import * as fs from "fs";

convertToGeoTiffsToJSON();
//loop through elevation aspect and slope tiffs of each thing

function convertToGeoTiffsToJSON() {
  let regions = ["abajos", "logan", "moab", "ogden", "provo", "salt-lake", "skyline", "uintas"];
  regions.forEach(async (region) => {
    let tiff = await fromFile(`./UAC/${region}/${region}_elevation_aspect_slope.tif`);
    let img = await tiff.getImage();
    let data = await img.readRasters();
    //metadata we need to be able to map file to coords and display on the map
    let fileDir = img.fileDirectory;
    let width = fileDir.ImageWidth;
    let height = fileDir.ImageLength;
    let modelTiePoint = fileDir.ModelTiepoint.toString();
    let modelPixelScale = fileDir.ModelPixelScale.toString();
    let boundingBox = img.getBoundingBox().toString();
    let jsonObject = {
      region: "salt-lake",
      type: "slope",
      width: width,
      height: height,
      modelTiepoint: modelTiePoint,
      modelPixelScale: modelPixelScale,
      boundingBox: boundingBox,
      data: data[0].toString(),
    };
    //console.log(jsonObject);
    // write elevation
    fs.writeFileSync(`UAC_DATA/${region}/${region}_elevation.json`, JSON.stringify(jsonObject));
    // write aspect
    jsonObject.data = data[1].toString();
    fs.writeFileSync(`UAC_DATA/${region}/${region}_aspect.json`, JSON.stringify(jsonObject));
    // write slope
    jsonObject.data = data[2].toString();
    fs.writeFileSync(`UAC_DATA/${region}/${region}_slope.json`, JSON.stringify(jsonObject));
  });
}

//geoTiffTest();
async function geoTiffTest() {
  let tiff = await fromFile("./UAC/salt-lake/salt-lake_slope.tif");
  let img = await tiff.getImage();
  let data = await img.readRasters();
  //metadata we need to be able to map file to coords and display on the map
  let fileDir = img.fileDirectory;
  let width = fileDir.ImageWidth;
  let height = fileDir.ImageLength;
  let modelTiePoint = fileDir.ModelTiepoint.toString();
  let modelPixelScale = fileDir.ModelPixelScale.toString();
  let boundingBox = img.getBoundingBox().toString();
  let jsonObject = {
    region: "salt-lake",
    type: "slope",
    width: width,
    height: height,
    modelTiepoint: modelTiePoint,
    modelPixelScale: modelPixelScale,
    boundingBox: boundingBox,
    data: data.toString(),
  };
  await fs.writeFileSync("./testSize.json", JSON.stringify(jsonObject));
  //console.log(data[0]);
}
