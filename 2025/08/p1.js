const fs = require('node:fs/promises');

let max = Math.max, min = Math.min, abs = Math.abs, pow = Math.pow;

let dist2 = (a, b) => pow(a.x - b.x, 2) + pow(a.y - b.y, 2) + pow(a.z - b.z, 2)

// children at indices 2i + 1 and 2i + 2
// its parent at index floor((i − 1) / 2).
let start = Date.now()
class Heap {
    h = []
    ct

    constructor(ct) {
        this.ct = ct
    }

    getSorted() {
        return this.h.slice().sort((a, b) => a.d - b.d)
    }

    add(conn) {
        if (this.h.length >= this.ct) {
            if (conn.d < this.h[0].d) {
                this.h[0] = conn
                for (let i = 0; 2 * i + 1 < this.h.length;) {
                    let candidates = 2 * i + 2 === this.h.length ? [i, 2 * i + 1] : [i, 2 * i + 1, 2 * i + 2]
                    let largest = candidates.sort((a, b) => this.h[a].d - this.h[b].d).pop()
                    if (largest === i) {
                        break;
                    } else {
                        [this.h[i], this.h[largest]] = [this.h[largest], this.h[i]]
                        i = largest
                    }
                }
            } //else discard
        } else {
            this.h.push(conn)
            for (let i = this.h.length - 1; i >= 0;) {
                let parent = Math.floor((i - 1) / 2)
                if (parent >= 0 && this.h[i].d > this.h[parent].d) {
                    [this.h[i], this.h[parent]] = [this.h[parent], this.h[i]]
                    i = parent
                } else {
                    break;
                }
            }
        }
    }
}

// noinspection JSVoidFunctionReturnValueUsed
fs.readFile("./input.txt", {encoding: "utf8"})
    .then(input => {
        let boxes = input.trim().split("\n")
            .map(l => l.trim().split(",").map(v => parseInt(v)))
            .map((l, i) => {
                return {idx: i, x: l[0], y: l[1], z: l[2], conns: [], circuit: -1}
            })

        let heap = new Heap(1000)

        for (let i = 0; i < boxes.length; i++) {
            for (let j = i + 1; j < boxes.length; j++) {
                heap.add({d: dist2(boxes[i], boxes[j]), ai: boxes[i].idx, bi: boxes[j].idx})
            }
        }

        let shortest = heap.getSorted()
        for (let conn of shortest) {
            boxes[conn.ai].conns.push(conn.bi)
            boxes[conn.bi].conns.push(conn.ai)
        }

        let circuits = []
        for (let box of boxes) {
            if (box.circuit === -1) {
                box.circuit = circuits.length
                let toExplore = box.conns.slice()
                let connected = [box]
                while (toExplore.length > 0) {
                    let nextBox = boxes[toExplore.pop()]
                    if (nextBox.circuit === -1) {
                        nextBox.circuit = circuits.length
                        connected.push(nextBox)
                        nextBox.conns.forEach(c => toExplore.push(c))
                    }
                }
                circuits.push(connected)
            }
        }

        circuits.sort((a, b) => b.length - a.length)
        // console.log(circuits[0].map(c => c.idx).sort((a, b) => a - b))
        // console.log(circuits[1].map(c => c.idx).sort((a, b) => a - b))
        // console.log(circuits[2].map(c => c.idx).sort((a, b) => a - b))
        console.log(circuits[0].length * circuits[1].length * circuits[2].length)
        console.log(`Took ${Date.now() - start}ms`)

    })

// let minMaxes = boxes.reduce((m, b) => {
//     return {
//         minX: min(m.minX, b.x), maxX: max(m.maxX, b.x),
//         minY: min(m.minY, b.y), maxY: max(m.maxY, b.y),
//         minZ: min(m.minZ, b.z), maxZ: max(m.maxZ, b.z)
//     }
// }, {minX: Infinity, maxX: 0, minY: Infinity, maxY: 0, minZ: Infinity, maxZ: 0})

// let xs = new Array(boxes.length).map((a, i) => i).sort((a, b) => boxes[a].x - boxes[b].x)
// xs.forEach((i, xi) => boxes[i].xi = xi)
// let ys = new Array(boxes.length).map((a, i) => i).sort((a, b) => boxes[a].y - boxes[b].y)
// ys.forEach((i, yi) => boxes[i].yi = yi)
// let zs = new Array(boxes.length).map((a, i) => i).sort((a, b) => boxes[a].z - boxes[b].z)
// zs.forEach((i, zi) => boxes[i].zi = zi)