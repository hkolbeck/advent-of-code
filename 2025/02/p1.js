const fs = require('node:fs/promises');

// noinspection JSVoidFunctionReturnValueUsed
fs.readFile("./input.txt", {encoding: "utf8"})
    .then(input => {
        let ranges = input.trim().split(",").map(r => r.split("-").map(v => parseInt(v)))
        let maxLen = Math.ceil(Math.log10(
            ranges.reduce((acc, v) => Math.max(acc, v[1]), 0)
        ) / 2);
        console.log(ranges)

        let sum = 0;
        for (let numLen = 1; numLen <= maxLen; numLen++) {
            for (let num = Math.pow(10, numLen - 1); num < Math.pow(10, numLen); num++) {
                let repNum = num * Math.pow(10, numLen) + num;
                for (let range of ranges) {
                    if (range[0] <= repNum && repNum <= range[1]) {
                        console.log(`${repNum} - ${JSON.stringify(range)}`);
                        sum += repNum;
                        break;
                    }
                }
            }
        }

        console.log(sum);
    })