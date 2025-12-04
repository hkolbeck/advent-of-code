const fs = require('node:fs/promises');

const deltas = [
    [-1, -1], [+0, -1], [+1, -1],
    [-1, +0], /*     */ [+1, +0],
    [-1, +1], [+0, +1], [+1, +1]
]

// noinspection JSVoidFunctionReturnValueUsed
fs.readFile("./input.txt", {encoding: "utf8"})
    .then(input => {
        let raw = input.trim().split("\n").map(line => line.trim().split(""));
        let board = [new Array(raw[0].length + 2).fill(false)]
        for (let line of raw) {
            board.push([false].concat(line.map(c => c === '@')).concat([false]));
        }
        board.push(new Array(raw[0].length + 2).fill(false))

        let removed = [];
        let numRemoved = 0;
        do {
            numRemoved += removed.length
            for (let [x, y] of removed) {
                board[y][x] = false;
                raw[y - 1][x - 1] = '.'
            }
            removed = []

            for (let x = 1; x <= raw[0].length; x++) {
                square:
                    for (let y = 1; y <= raw.length; y++) {
                        if (!board[y][x]) continue;

                        let bales = 0;
                        for (let delta of deltas) {
                            if (board[y + delta[1]][x + delta[0]]) {
                                if (++bales >= 4) continue square;
                            }
                        }

                        raw[y - 1][x - 1] = 'x'
                        removed.push([x, y])
                    }
            }
            console.log(`Removed ${removed.length}\n${raw.map(l => l.join('')).join("\n")}`);
        } while (removed.length > 0);


        console.log(numRemoved);
    })