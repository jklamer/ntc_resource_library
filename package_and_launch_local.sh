#! /bin/sh


_term() {
    kill $PID
    cd $CD
}

trap _term SIGTERM
trap _term SIGINT

./package.sh
CD=$(pwd)
cd dist/
python3 -m http.server 8080 &
PID=$!

wait $PID