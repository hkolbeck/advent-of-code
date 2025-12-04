const fs = require('node:fs/promises');

// noinspection JSVoidFunctionReturnValueUsed
fs.readFile("./input.txt", {encoding: "utf8"})
    .then(input => {
        let instr = input
            .replaceAll("L", "-")
            .replaceAll("R", "")
            .split('\n')
            .filter(s => s !== '')
            .map(s => parseInt(s))
        let zs = 0, dial = 50
        for (let ins of instr) {
            let dz = dial === 0
            zs += Math.floor(Math.abs(ins / 100))
            dial += ins % 100;
            if (dial > 99) {
                zs++;
                dial = dial % 100;
            } else if (dial < 0) {
                if (!dz) {
                    zs++
                }
                dial = (dial + 100) % 100;
            } else if (dial === 0) {
                zs++;
            }

            console.log(`${ins} ${dial} ${zs}`)
        }
        console.log(zs)
    })