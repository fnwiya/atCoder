function Main(input) {
    var name = input.split("\n")[0];
    var len = name.length;
    for(var i = 0; i < Math.floor(len / 2); i++) {
        if (name[i] !== name[len - 1 - i]) {
            console.log("NO");
            return;
        }
    }
    console.log("YES");
}

Main(require("fs").readFileSync("/dev/stdin", "utf8"));
