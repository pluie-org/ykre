#!/bin/bash

cargo install --git https://github.com/pluie-org/ykre.git --root ./
sudo mv ./bin/ykre /usr/local/bin/
rm -rf ./bin
