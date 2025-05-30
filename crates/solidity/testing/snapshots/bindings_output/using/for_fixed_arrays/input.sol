library ArrayHelper {
    function asArray(string[1] memory items) internal {}
}

using ArrayHelper for string[1];

function test() {
    ["foo"].asArray();
}
