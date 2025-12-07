const fs = require('node:fs/promises');

let add = (a, b) => a + b;
let mult = (a, b) => a * b;

// noinspection JSVoidFunctionReturnValueUsed
fs.readFile("./input.txt", {encoding: "utf8"})
    .then(input => {
        let parts = input.trim().split("\n")
            .map(line => line.split(""))
        // for (let part of parts) {
        //     console.log(part.length);
        // }

        // console.log(parts.map(part => part.length).reduce((a, l) => a + l, 0))
        let maxLen = parts.reduce((m, line) => Math.max(m, line.length), 0)
        // console.log(maxLen)

        for (let line of parts) {
            while (line.length < maxLen) {
                line.push(" ")
            }
        }
        // console.log(parts.map(part => part.length).reduce((a, l) => a + l, 0))

        let problems = parts.pop().map((c, i) => {
            switch (c) {
                case " ": return null;
                case "*": return {col: i, op: mult, start: 1, s: '*'};
                case "+": return {col: i, op: add, start: 0, s: "+"}
            }
        }).filter(v => v !== null)

        let numbers = []
        for (let col = 0; col < maxLen; col++) {
            let str = ""
            for (let line = 0; line < parts.length; line++) {
                str += parts[line][col]
            }
            str = str.trim();
            if (str === "") {
                numbers.push(null)
            } else {
                numbers.push(parseInt(str))
            }
        }

        let total = 0
        for (let prob of problems) {
            let val = prob.start
            let log = `${val}`
            for (let col = prob.col; col < numbers.length && numbers[col] !== null; col++) {
                val = prob.op(val, numbers[col])
                log += ` ${prob.s} ${numbers[col]}`
            }
            console.log(`${log} = ${val}`)
            total += val
        }

        console.log(total)
    })