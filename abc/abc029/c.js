function Main(input) {
    var n = parseInt(input, 10);
    var abc = "abc";
    var addChar =  function (str, strLen) {
        if (strLen === n) {
            console.log(str);
            return;
        } else {
            for(var i = 0; i < abc.length; i++) {
                addChar(str+abc[i], (str+abc[i]).length);
            }
        }
    };
    addChar("", 0);
}

Main(require("fs").readFileSync("/dev/stdin", "utf8"));
