#!/usr/bin/env bash
set -euo pipefail
HOST=pi-radio.localdomain

cargo build --target=armv7-unknown-linux-musleabihf
ssh "pi@${HOST}" "sudo systemctl stop rusty-radio"
scp target/armv7-unknown-linux-musleabihf/debug/rusty-radio "pi@${HOST}:/home/pi/rusty-radio"
ssh "pi@${HOST}" "sudo systemctl start rusty-radio"

cd web
ng build --prod
ssh "pi@${HOST}" rm -rf rust-web
scp -r dist/web "pi@${HOST}:/home/pi/rust-web"
