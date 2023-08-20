import { execSync } from "child_process";
import fs from "fs";
import { Tile } from "mahjong_sdk/dist/core";
import { getTileInfo } from "mahjong_sdk/dist/tile-content";
import { getAllUniqueTiles } from "mahjong_sdk/dist/tiles";

import { getTileImageName } from "../src/lib/assets";

const list: [string, Tile][] = getAllUniqueTiles().map((tile) => [
  getTileImageName(tile),
  tile,
]);

const getSvgFilePage = (fileName: string) => `./assets/svgs/${fileName}.svg`;
const getPngFilePage = (fileName: string) => `./assets/pngs/${fileName}.png`;

const main = async () => {
  await list.reduce(async (promise, [imageName, tile]) => {
    await promise;
    const svgPath = getSvgFilePage(imageName);
    const svgExists = fs.existsSync(svgPath);
    const tileInfo = getTileInfo(tile);
    if (!tileInfo) {
      throw new Error(`Tile info not found: ${imageName}`);
    }

    if (!svgExists) {
      const [tileUrl] = tileInfo;
      const command = `wget -O ${svgPath} ${tileUrl}`;
      console.log(`Downloading: ${svgPath}`);
      execSync(command, { stdio: "ignore" });
    }

    const pngPath = getPngFilePage(imageName);
    const pngExists = fs.existsSync(pngPath);

    if (!pngExists) {
      const command = `inkscape -w 100 ${svgPath} -o ${pngPath}`;
      console.log(`Generating: ${pngPath}`);
      execSync(command, { stdio: "ignore" });
      console.log(`Generated: ${pngPath}`);
    }
  }, Promise.resolve());
};

main().catch((e) => {
  console.error("Error", e);
  process.exit(1);
});

export {};
