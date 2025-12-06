set -euo pipefail

# Install ZSH
curl -sSL https://github.com/deluan/zsh-in-docker/releases/download/v1.2.0/zsh-in-docker.sh | \
    bash -s -- -t robbyrussell -p git -p node -x
