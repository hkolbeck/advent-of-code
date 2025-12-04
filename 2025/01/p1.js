fetch("https://adventofcode.com/2025/day/1/input")
    .then(resp => resp.text())
    .then(input => {
        let zs = input
            .replaceAll("L", "-")
            .replaceAll("R", "")
            .split('\n')
            .map(s => parseInt(s))
            .reduce((acc, v) => {
                acc.unshift((acc[0] + v + 100) % 100);
                console.log(acc[0])
                return acc
            }, [50])
            .filter(v => v === 0)//.length;
        console.log(zs)
    })