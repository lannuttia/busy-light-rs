FROM mcr.microsoft.com/vscode/devcontainers/base:ubuntu-18.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt update \
  #
  # Used to add additional repositories
  && apt install -y --no-install-recommends \
    ca-certificates \
    curl \
    apt-transport-https \
    lsb-release \
    gnupg \
  #
  # Add Microsoft's Azure CLI repository
  && curl -sL https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor | tee /etc/apt/trusted.gpg.d/microsoft.asc.gpg > /dev/null \
  && echo "deb [arch=$(dpkg --print-architecture)] https://packages.microsoft.com/repos/azure-cli/ $(lsb_release -cs) main" > /etc/apt/sources.list.d/azure-cli.list \
  #
  # Add Docker's repository
  && curl -sL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor | tee /etc/apt/trusted.gpg.d/docker.asc.gpg > /dev/null \
  && echo "deb [arch=$(dpkg --print-architecture)] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" > /etc/apt/sources.list.d/docker.list \
  #
  # Add Azure IoT SDK
  && apt-key adv --keyserver keyserver.ubuntu.com --recv-keys AF1A6BA067E3641215DF0ECBFDA6A393E4C2257F \
  && echo "deb [arch=$(dpkg --print-architecture)] http://ppa.launchpad.net/aziotsdklinux/ppa-azureiot/ubuntu $(lsb_release -cs) main" > /etc/apt/sources.list.d/azure-iot.list \
  #
  # Pull in newly added repositories
  && apt update \
  && apt install -y \
    #
    # Install C toolchain
    build-essential \
    #
    # Required by openssl-sys
    pkg-config \
    libssl-dev \
    #
    # For debugging network related issues
    dnsutils \
    #
    # Used to interface with Azure
    azure-cli \
    #
    # Used to interface with docker daemon running on the host
    docker-ce-cli \
    #
    # Install Azure IoT device SDK for C library with header files
    azure-iot-sdk-c-dev \
  #&& su --login vscode --shell /bin/bash -c " \
  #
  # Add non-root user to the docker group
  && groupadd docker \
  && usermod -aG docker vscode \
  && touch /var/run/docker.sock \
  && chown root:docker /var/run/docker.sock \
  #
  # Clean up
  && apt -y autoremove \
  && apt -y clean \
  && rm -rf /var/lib/apt/lists/*

USER vscode

RUN curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path

# Switch back to dialog for any ad-hoc use of apt
ENV DEBIAN_FRONTEND=dialog \
    PATH=/home/vscode/.cargo/bin:$PATH
