#!/bin/bash
set -euxo pipefail
if [ $# -ne 1 ]; then
    echo "Usage: scripts/deploy.sh <hostname>"
    exit 1
fi

cargo build --profile prod

rsync -Pavzr --delete assets templates config target/prod/wlsd root@$1:/home/wlsd/
ssh root@$1 <<'EOS'
apt-get update
apt-get upgrade -y
setcap 'cap_net_bind_service=+ep' /home/wlsd/wlsd
systemctl restart wlsd
EOS
