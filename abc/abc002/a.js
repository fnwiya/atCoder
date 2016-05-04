function Main(input) {
    var xy = input.split(" ");
    console.log(Math.max(parseInt(xy[0], 10), parseInt(xy[1], 10)));
}

Main(require("fs").readFileSync("/dev/stdin", "utf8"));
