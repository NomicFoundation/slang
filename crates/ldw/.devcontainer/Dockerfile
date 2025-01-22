FROM mcr.microsoft.com/vscode/devcontainers/base:ubuntu

# Install dependencies
RUN apt-get update && apt-get install -y curl xz-utils direnv && rm -rf /var/lib/apt/lists/*

# Install Nix
RUN curl -L https://nixos.org/nix/install | sh -s -- --daemon

# Set up non-root user
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID
RUN groupadd --gid $USER_GID $USERNAME     && useradd -s /bin/bash --uid $USER_UID --gid $USER_GID -m $USERNAME

# Add Nix to PATH for the vscode user
RUN echo '. /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh' >> /home/$USERNAME/.bashrc

# Set up direnv for the vscode user
RUN echo 'eval "$(direnv hook bash)"' >> /home/$USERNAME/.bashrc

# Switch to non-root user
USER $USERNAME

# Install devbox
RUN curl -fsSL https://get.jetpack.io/devbox | bash

# Set up Nix for non-root user
RUN mkdir -p /home/$USERNAME/.nix-profile /home/$USERNAME/.nix-defexpr /home/$USERNAME/.nix-channels && ln -s /nix/var/nix/profiles/per-user/$USERNAME/profile /home/$USERNAME/.nix-profile
