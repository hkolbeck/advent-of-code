const fs = require('node:fs/promises');

let add = (a, b) => a + b;
let mult = (a, b) => a * b;

// noinspection JSVoidFunctionReturnValueUsed
fs.readFile("./input.txt", {encoding: "utf8"})
    .then(input => {
        let parts = input.trim().split("\n")
            .map(line => line.trim().split(/\s+/g))
        let ops = parts.pop().map(os => os === "+" ? [add, 0] : [mult, 1])

        let total = 0
        for (let col = 0; col < parts[0].length; col++) {
            let colVal = ops[col][1]
            for (let line = 0; line < parts.length; line++) {
                colVal = ops[col][0](colVal, parseInt(parts[line][col]));
            }
            total += colVal
        }

        console.log(total)
    })