const fs = require('node:fs/promises');

let max = Math.max, min = Math.min, abs = Math.abs, pow = Math.pow;

let dist2 = (a, b) => pow(a.x - b.x, 2) + pow(a.y - b.y, 2) + pow(a.z - b.z, 2)

class Heap {
    h = []

    add(conn) {
        this.h.push(conn)
        for (let i = this.h.length - 1; i >= 0;) {
            let parent = Math.floor((i - 1) / 2)
            if (parent >= 0 && this.h[i].d < this.h[parent].d) {
                [this.h[i], this.h[parent]] = [this.h[parent], this.h[i]]
                i = parent
            } else {
                break;
            }
        }
    }

    pop() {
        if (this.h.length === 1) {
            return this.h.pop()
        }

        let ret = this.h[0]
        this.h[0] = this.h.pop()

        for (let i = 0; 2 * i + 1 < this.h.length;) {
            let candidates = 2 * i + 2 === this.h.length ? [i, 2 * i + 1] : [i, 2 * i + 1, 2 * i + 2]
            let smallest = candidates.sort((a, b) => this.h[b].d - this.h[a].d).pop()
            if (smallest === i) {
                break;
            } else {
                [this.h[i], this.h[smallest]] = [this.h[smallest], this.h[i]]
                i = smallest
            }
        }

        return ret
    }
}
let start = Date.now()
// noinspection JSVoidFunctionReturnValueUsed
fs.readFile("./input.txt", {encoding: "utf8"})
    .then(input => {
        let boxes = input.trim().split("\n")
            .map(l => l.trim().split(",").map(v => parseInt(v)))
            .map((l, i) => {
                return {idx: i, x: l[0], y: l[1], z: l[2], conns: [], circuit: i}
            })

        let heap = new Heap()

        for (let i = 0; i < boxes.length; i++) {
            for (let j = i + 1; j < boxes.length; j++) {
                heap.add({d: dist2(boxes[i], boxes[j]), ai: boxes[i].idx, bi: boxes[j].idx})
            }
        }

        let circuits = boxes.map(b => [b.idx])
        let last = null
        for (let conn = heap.pop(); conn; conn = heap.pop()) {
            let a = boxes[conn.ai], b = boxes[conn.bi]
            if (a.circuit !== b.circuit) {
                let oldB = b.circuit
                circuits[a.circuit].push(...circuits[b.circuit])
                circuits[b.circuit].forEach(i => boxes[i].circuit = a.circuit)
                circuits[oldB] = []

                if (circuits[a.circuit].length === boxes.length) {
                    last = conn
                    break
                }
            }
        }

        console.log(boxes[last.ai].x * boxes[last.bi].x)
        console.log(`Took ${Date.now() - start}ms`)
    })