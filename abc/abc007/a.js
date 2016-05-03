function Main(n) {
    console.log(parseInt(n, 10) - 1);
}

Main(require("fs").readFileSync("/dev/stdin", "utf8"));
