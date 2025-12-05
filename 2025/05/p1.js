const fs = require('node:fs/promises');

// noinspection JSVoidFunctionReturnValueUsed
fs.readFile("./input.txt", {encoding: "utf8"})
    .then(input => {
        let [ranges, ids] = input.split("\n\n");
        ranges = ranges.trim().split("\n").map(range => range.split("-").map(v => parseInt(v)));
        ids = ids.trim().split("\n").map(v => parseInt(v));

        let fresh = 0;
        for (let id of ids) {
            for (let range of ranges) {
                if (range[0] <= id && id <= range[1]) {
                    fresh++;
                    break;
                }
            }
        }

        console.log(fresh);
    })