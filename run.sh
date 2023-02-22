set -ex

wasm-pack build --target web --out-dir www/pkg $1 $2

printf "\nNow serving at \n\t http://127.0.0.1:8080/html \n\n"

http -a 127.0.0.1 -p 8080 www

