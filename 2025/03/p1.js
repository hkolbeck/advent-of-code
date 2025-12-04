const fs = require('node:fs/promises');

// noinspection JSVoidFunctionReturnValueUsed
fs.readFile("./input.txt", {encoding: "utf8"})
    .then(input => {
        let sum = input.trim().split("\n").map(l => l.trim().split("").map(s => parseInt(s)))
            .map(batts => {
                let curFirst = batts[0];
                let curSecond = -1;
                for (let i = 1; i < batts.length; i++) {
                    if (batts[i] > curFirst && i < batts.length - 1) {
                        curFirst = batts[i];
                        curSecond = -1
                    } else if (batts[i] > curSecond) {
                        curSecond = batts[i];
                    }
                }
                if (curSecond === -1) {
                    curSecond = batts[batts.length - 1]
                }

                console.log(`${batts.join("")} - ${curFirst}${curSecond}`);
                return curFirst * 10 + curSecond;
            }).reduce((a, b) => a + b);
        console.log(sum);
    })