import { getAllUniqueTiles } from "mahjong_sdk/dist/tiles";

import { getTileImageName, imageNameToImport } from "../assets";

const tiles = getAllUniqueTiles();
const tileNames = tiles.map((tile) => getTileImageName(tile));

describe("getTileImageName", () => {
  it("generates an unique name", () => {
    const uniqueNames = new Set(tileNames);

    expect(uniqueNames.size).toEqual(tiles.length);
  });
});

describe("imageNameToImport", () => {
  it("contains an entry for each tile", () => {
    tileNames.forEach((tileName) => {
      expect(imageNameToImport[tileName]).toBeDefined();
    });
  });
});
