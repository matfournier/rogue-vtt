#!/bin/sh

trap on_ctrl_c INT

PROJECT_ROOT=$(dirname $(readlink -f "$0"))

unset -v VITE_DEV_PID

on_ctrl_c() {
    if [ ! -z ${VITE_DEV_PID+x} ]; then
        echo "killing dev"
        kill -s INT $VITE_DEV_PID
    fi
    exit
}

cd $PROJECT_ROOT/app && npm run dev &
VITE_DEV_PID=$!

cd $PROJECT_ROOT && cargo run dev
