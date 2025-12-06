set -euo pipefail

export DEBIAN_FRONTEND=noninteractive

apt update -qq
apt install -y sudo git git-lfs curl zsh postgresql-client

apt clean
