const fs = require('node:fs/promises');

// noinspection JSVoidFunctionReturnValueUsed
fs.readFile("./input.txt", {encoding: "utf8"})
    .then(input => {
        let sum = input.trim()
            .split("\n")
            .map(l => l.trim()
                .split("")
                .map(s => parseInt(s)))
            .map(batts => {
                let result = 0
                let startIdx = 0
                for (let maxIdx = batts.length - 12; maxIdx < batts.length; maxIdx++) {
                    let best = startIdx
                    for (let i = startIdx + 1; i <= maxIdx; i++) {
                        if (batts[i] > batts[best]) {
                            best = i
                        }
                    }
                    startIdx = best + 1;
                    result = result * 10 + batts[best]
                }
                console.log(`${batts.join("")} - ${result}`);
                return result;
            }).reduce((a, b) => a + b);
        console.log(sum);
    })