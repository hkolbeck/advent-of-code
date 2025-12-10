const fs = require('node:fs/promises');

// noinspection JSVoidFunctionReturnValueUsed
fs.readFile("./input.txt", {encoding: "utf8"})
    .then(input => {
        let tiles = input.trim().split("\n")
            .map(l => l.trim().split(",")
                .map(c => parseInt(c)))
            .map(([x, y]) => {return {x, y}})

        let maxArea = 0
        let maxCorners = null
        for (let i = 0; i < tiles.length; i++) {
            for (let j = i + 1; j < tiles.length; j++) {
                let area = (Math.abs(tiles[i].x - tiles[j].x) + 1) * (Math.abs(tiles[i].y - tiles[j].y) + 1)
                if (area > maxArea) {
                    maxCorners = [i, j]
                    maxArea = area
                }
            }
        }

        console.log(maxArea)
    })