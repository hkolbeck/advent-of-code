const fs = require('node:fs/promises');

let max = Math.max, min = Math.min, abs = Math.abs, pow = Math.pow;

let room = {}
let start = Date.now()
// noinspection JSVoidFunctionReturnValueUsed
fs.readFile("./input.txt", {encoding: "utf8"})
    .then(input => {
        let tiles = input.trim().split("\n")
            .map(l => l.trim().split(",")
                .map(c => parseInt(c)))
            .map(([x, y]) => {
                return {x, y}
            })

        tiles.reduce((prev, tile) => {
            if (!room[tile.y]) {
                room[tile.y] = {}
            }
            room[tile.y][tile.x] = true

            if (tile.x === prev.x) {
                let from = min(tile.y, prev.y), to = max(tile.y, prev.y)
                for (let yy = from + 1; yy < to; yy++) {
                    if (!room[yy]) {
                        room[yy] = {}
                    }
                    room[yy][tile.x] = true
                }
            } else {
                let from = min(tile.x, prev.x), to = max(tile.x, prev.x)
                for (let xx = from + 1; xx < to; xx++) {
                    room[tile.y][xx] = true
                }
            }

            return tile
        }, tiles[tiles.length - 1])

        let maxArea = 0
        for (let i = 0; i < tiles.length - 1; i++) {
            tileLoop: for (let j = i + 1; j < tiles.length; j++) {
                // console.log(i, j)
                let area = (Math.abs(tiles[i].x - tiles[j].x) + 1) * (Math.abs(tiles[i].y - tiles[j].y) + 1)
                if (area > maxArea) {
                    let minX = min(tiles[i].x, tiles[j].x)
                    let maxX = max(tiles[i].x, tiles[j].x);
                    let minY = min(tiles[i].y, tiles[j].y);
                    let maxY = max(tiles[i].y, tiles[j].y);

                    for (let y = minY + 1; y < maxY; y++) {
                        if (room[y][minX + 2] || room[y][maxX - 2]) {
                            continue tileLoop;
                        }
                    }

                    for (let x = minX + 1; x < maxX; x++) {
                        if (room[minY + 2][x] || room[maxY - 2][x]) {
                            continue tileLoop;
                        }
                    }
                    maxArea = area
                }
            }
        }

        console.log(maxArea)
        console.log(`In ${Date.now() - start}ms`)
    })
