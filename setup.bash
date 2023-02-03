#!/bin/sh


if [[ "$VIRTUAL_ENV" != "" ]]
then
    echo "Build and test"
    maturin develop --release
    pytest
else
    if [[ -d "./venv" ]]
    then
        echo "Activating venv"
        source venv/bin/activate
        echo "Build package in venv using maturin"
        maturin develop
        echo "Test package"
        pytest
        deactivate
    else
        echo "Setting up virutal enviroment"
        python3 -m venv venv
        source venv/bin/activate
        pip install -r requirements.txt
        echo "Updating Cargo"
        cargo update
        echo "Build package in venv using maturin"
        maturin develop
        echo "Test package"
        pytest
        deactivate
    fi
    echo "Run the command: source venv/bin/activate"
    echo "to access the mediapipe_rotations package."
fi

