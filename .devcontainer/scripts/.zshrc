#!/bin/zsh

# P10K
source "$HOME/.p10k/powerlevel10k.zsh-theme"
source "$HOME/.p10k-profile.zsh"

# Hermit
eval "$("$HOME/bin/hermit" shell-hooks --print --zsh)"

# ZSH Completions
autoload -Uz compinit && compinit
