#!/bin/sh
flag=$1

if [ "$flag" == "--release" ]
then
    echo "maturin develop --release"
    maturin develop --release
else
    echo "maturin develop"
    maturin develop
fi
