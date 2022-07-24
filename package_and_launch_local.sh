#! /bin/sh


term() {
    kill -9 $PID
    cd $CD
}

trap term SIGTERM
trap term SIGINT
trap term SIGKILL

./package.sh
CD=$(pwd)
cd ~/scratch/NTC_resource_library
python3 -m http.server 8080 &
PID=$!

wait $PID
