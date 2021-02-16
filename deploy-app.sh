#!/usr/bin/env bash
set -euo pipefail
HOST=pi-radio.localdomain
ARCH=armv7-unknown-linux-gnueabihf

cargo build --target=${ARCH}
ssh "pi@${HOST}" "sudo systemctl stop rusty-radio"
scp target/${ARCH}/debug/rusty-radio "pi@${HOST}:/home/pi/rusty-radio"
ssh "pi@${HOST}" "sudo systemctl start rusty-radio"
