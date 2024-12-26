#!/bin/bash
set -euxo pipefail
if [ $# -ne 1 ]; then
    echo "Usage: scripts/deploy.sh <user>@<hostname>"
    exit 1
fi

cargo build --profile prod --target aarch64-unknown-linux-gnu

ls -l target/aarch64-unknown-linux-gnu/
ls -l target/aarch64-unknown-linux-gnu/*
rsync --rsync-path="sudo rsync" -Pavzr --delete assets templates config target/aarch64-unknown-linux-gnu/prod/wlsd $1:/home/wlsd/
ssh $1 <<'EOS'
sudo setcap 'cap_net_bind_service=+ep' /home/wlsd/wlsd
sudo systemctl restart wlsd
EOS
