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

        let rects = []
        for (let i = 0; i < tiles.length - 1; i++) {
            for (let j = i + 1; j < tiles.length; j++) {
                rects.push({
                    area: (Math.abs(tiles[i].x - tiles[j].x) + 1) * (Math.abs(tiles[i].y - tiles[j].y) + 1),
                    xFrom: min(tiles[i].x, tiles[j].x),
                    xTo: max(tiles[i].x, tiles[j].x),
                    yFrom: min(tiles[i].y, tiles[j].y),
                    yTo: max(tiles[i].y, tiles[j].y)
                })
            }
        }
        rects.sort((a, b) => b.area - a.area)

        let maxArea = 0
        rectLoop: for (let rect of rects) {
            if (rect.area <= maxArea) {
                break
            }

            for (let y = rect.yFrom + 1; y < rect.yTo; y++) {
                if (room[y][rect.xFrom + 2] || room[y][rect.xTo - 2]) {
                    continue rectLoop;
                }
            }

            for (let x = rect.xFrom + 1; x < rect.xTo; x++) {
                if (room[rect.yFrom + 2][x] || room[rect.yTo - 2][x]) {
                    continue rectLoop;
                }
            }

            maxArea = rect.area
        }

        console.log(maxArea)
        console.log(`In ${Date.now() - start}ms`)
    })
