const fs = require('node:fs/promises');

// noinspection JSVoidFunctionReturnValueUsed
fs.readFile("./input.txt", {encoding: "utf8"})
    .then(input => {
        let board = input.trim().split('\n')
            .map(l => l.trim().split('')
                .map(c => c === '^'))
            .map(l => [false].concat(l, [false]))

        let width = board[0].length
        let beams  = new Array(width).fill(false)
        beams[input.indexOf('S') + 1] = true;

        let splits = 0;
        for (let step of board) {

            for (let col = 1; col < width - 1; col++) {
                if (beams[col] && step[col]) {
                    beams[col - 1] = true;
                    beams[col + 1] = true;
                    beams[col] = false;
                    splits++
                }
            }
            console.log(step.map((c, i) => c ? "^" : (beams[i] ? "|" : ".")).join(""))
        }

        console.log(splits)
    })