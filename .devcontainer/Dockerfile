FROM rust:1.91.1-bullseye AS base

SHELL ["/bin/bash", "-c"]

# Setup dev user
RUN adduser --disabled-password --gecos "" dev && usermod -aG sudo dev && passwd -d dev

# Install root-level dependencies
COPY --chmod=700 --chown=root:root ./install-root-dependencies.sh /tmp/install-root-dependencies.sh
RUN /tmp/install-root-dependencies.sh



USER dev

# Install user-level dependencies
COPY --chmod=777 --chown=dev:dev ./install-user-dependencies.sh /tmp/install-user-dependencies.sh
RUN /tmp/install-user-dependencies.sh



WORKDIR /workspace


ENTRYPOINT ["/usr/bin/zsh", "-c"]
CMD ["tail -f /dev/null"]
