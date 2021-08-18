var ffi = require('ffi');
var path = require('path');
var lib = ffi.Library(path.join(__dirname, './target/release/libffi_fibonacci_10'), {
    fibonacci: ['int', ['int']]
});

console.time();
let rustFib = lib.fibonacci(30);
console.timeEnd();

function fibonacci(n) {
    if (n <= 2) {
        return 1
    } else {
        return fibonacci(n-1) + fibonacci(n-2)
    }
}

console.time();
let nodeFib = fibonacci(30);
console.timeEnd();

console.log(rustFib, nodeFid);
