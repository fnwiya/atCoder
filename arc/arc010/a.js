function Main(input) {
    var inputRow = input.split("\n");
    var inputHead = inputRow[0].split(" ");
    var n = parseInt(inputHead[0], 10);
    var m = parseInt(inputHead[1], 10);
    var a = parseInt(inputHead[2], 10);
    var b = parseInt(inputHead[3], 10);
    var cards = n;
    for(var i = 1; i <= m; i++) {
        if (cards <= a) {
            cards += b;
        }
        cards -= parseInt(inputRow[i], 10);
        if (cards < 0) {
            console.log(i);
            return;
        }
    }
    console.log("complete");
}

Main(require("fs").readFileSync("/dev/stdin", "utf8"));
