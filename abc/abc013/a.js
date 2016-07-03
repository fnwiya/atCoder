function Main(input) {
    console.log("ABCDE".indexOf(input[0])+1);
}

Main(require("fs").readFileSync("/dev/stdin", "utf8"));
