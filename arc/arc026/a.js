function Main(input) {
    var inputHead = input.split(" ");
    var n = parseInt(inputHead[0], 10);
    var a = parseInt(inputHead[1], 10);
    var b = parseInt(inputHead[2], 10);
    var totalTime = 0;
    var poseCnt = 0;
    for(var i = 0; i < n; i++) {
        if (poseCnt < 5) {
            totalTime += b;
            poseCnt++;
        } else {
            totalTime += a;
        }
    }
    console.log(totalTime);
}

Main(require("fs").readFileSync("/dev/stdin", "utf8"));
