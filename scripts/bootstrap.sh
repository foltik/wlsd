#!/bin/bash
set -euxo pipefail
if [ $# -ne 1 ]; then
    echo "Usage: scripts/deploy.sh <hostname>"
    exit 1
fi

ssh root@$1 <<'EOS'
# update
apt update
apt upgrade -y
apt install -y rsync

# create a user
if ! id wlsd &>/dev/null; then
    adduser --disabled-password --gecos "" wlsd
fi

# add ssh keys
cat > .ssh/authorized_keys <<EOF
ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBHYLLX74GD4EJy2yZyn63AA7XXGoS1AHDrpxh+1lYgO4JeMqUk34S+eiyJ7WpENVKrePUeKhfqfgbqY1f05k37o= foltz@navi
ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBK0kzT3O4qZcnwHQEhocVu7c8ksX1UMgnupSP4tA0CFvapjexRbYBTO4EwZuLJk/Arx/CNB4IVpB8w9tpkXkbFY= actions@github.com/foltz/wlsd
EOF

# setup service
cat > /etc/systemd/system/wlsd.service <<EOF
[Unit]
Description=WLSD
After=network.target

[Service]
Type=simple
User=wlsd
WorkingDirectory=/home/wlsd
ExecStart=/home/wlsd/wlsd /home/wlsd/config/prod.toml
Restart=always

[Install]
WantedBy=multi-user.target
EOF
systemctl daemon-reload
systemctl enable wlsd
#systemctl restart wlsd
EOS
