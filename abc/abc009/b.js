function Main(input) {
    var inputRow = input.split("\n");
    var n = parseInt(inputRow[0], 10);
    var a = [];
    for(var i = 0; i < n; i++) {
        a[i] = parseInt(inputRow[i + 1], 10);
    }
    console.log(getSecondMax(a));
}

function getSecondMax(ary) {
    // 降順にソート
    ary.sort(function(a,b){
        if( a > b ) return -1;
        if( a < b ) return 1;
        return 0;
    });
    for(var i = 0; i < ary.length; i++) {
        if (ary[i] > ary[i + 1]) return ary[i + 1];
    }
}

Main(require("fs").readFileSync("/dev/stdin", "utf8"));
