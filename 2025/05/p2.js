const fs = require('node:fs/promises');

let sep = (r1, r2) => (r1.high < r2.low || r2.high < r1.low)

// noinspection JSVoidFunctionReturnValueUsed
fs.readFile("./input.txt", {encoding: "utf8"})
    .then(input => {
        let ranges = input.split("\n\n")[0].trim()
            .split("\n")
            .map(range => range.split("-")
                .map(v => parseInt(v)))
            .map(r => {return {low: r[0], high: r[1]}});

        for (let i = 0; i < ranges.length; i++) {
            let curr = i;
            for (let j = i + 1; j < ranges.length; j++) {
                if (!sep(ranges[curr], ranges[j])) {
                    ranges[j] = {
                        low: Math.min(ranges[curr].low, ranges[j].low),
                        high: Math.max(ranges[curr].high, ranges[j].high)
                    };
                    ranges[curr] = {low: 1, high: 0};
                    curr = j;
                }
            }
        }

        console.log(ranges);
        let tot = ranges.map(r => r.high - r.low + 1).reduce((a, b) => a + b);
        console.log(tot);
    })