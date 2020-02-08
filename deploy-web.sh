#!/usr/bin/env bash
set -euo pipefail
HOST=pi-radio.localdomain

cd web
ng build --prod
ssh "pi@${HOST}" rm -rf rust-web
scp -r dist/web "pi@${HOST}:/home/pi/rust-web"

set +e
ssh "pi@${HOST}" "killall midori"
set -e
ssh "pi@${HOST}" "rm -rf ~/.cache/midori/"
ssh "pi@${HOST}" "DISPLAY=:0 nohup midori -e fullscreen http://pi-radio.localdomain &"
