function Main(n) {
    if (parseInt(n, 10) % 3 === 0) {
        console.log("YES");
    } else{
        console.log("NO");
    }
}

Main(require("fs").readFileSync("/dev/stdin", "utf8"));
