#!/bin/bash
set -euxo pipefail
if [ $# -ne 1 ]; then
    echo "Usage: scripts/deploy.sh <user>@<hostname>"
    exit 1
fi

ssh $1 <<'EOS'
# update
sudo yum update -y

# create a user
if ! id wlsd &>/dev/null; then
    sudo adduser wlsd
fi

# add ssh keys
cat > .ssh/authorized_keys <<EOF
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILd0t8vieD+N6tL23X7NAS3bIm69dcq27eOoqjHT8ae5 foltz
ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBHYLLX74GD4EJy2yZyn63AA7XXGoS1AHDrpxh+1lYgO4JeMqUk34S+eiyJ7WpENVKrePUeKhfqfgbqY1f05k37o= foltz@navi
ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBK0kzT3O4qZcnwHQEhocVu7c8ksX1UMgnupSP4tA0CFvapjexRbYBTO4EwZuLJk/Arx/CNB4IVpB8w9tpkXkbFY= actions@github.com/foltz/wlsd
EOF

# setup service
sudo tee /etc/systemd/system/wlsd.service <<EOF >/dev/null
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
sudo systemctl daemon-reload
sudo systemctl enable wlsd
sudo systemctl restart wlsd
EOS
