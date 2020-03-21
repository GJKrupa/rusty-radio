#!/usr/bin/env bash
set -euo pipefail
HOST=pi-radio.localdomain

cd web
ng build --prod
ssh "pi@${HOST}" rm -rf rust-web
scp -r dist/web "pi@${HOST}:/home/pi/rust-web"

set +e
echo Killing old browser
ssh "pi@${HOST}" "killall midori"
set -e
echo Deleting cache
ssh "pi@${HOST}" "rm -rf ~/.cache/midori/"
echo Starting new browser
ssh "pi@${HOST}" "DISPLAY=:0 nohup midori -e fullscreen http://pi-radio.localdomain >/dev/null 2>&1 &"
