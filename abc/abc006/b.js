function Main(input) {
    var n = parseInt(input, 10);
    console.log(getTribonach(n));
}

function getTribonach(n) {
    var triboAry = [0, 0, 1];
    for(var i = 3; i < n; i++) {
        triboAry[i] = (triboAry[i - 1] + triboAry[i - 2] + triboAry[i - 3])  % 10007;
    }
    return triboAry[n-1];
}

Main(require("fs").readFileSync("/dev/stdin", "utf8"));
