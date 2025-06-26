# A `command-not-found` for the new nix CLI

[![CI status](https://github.com/goj/command-not-found/workflows/CI/badge.svg)](https://github.com/goj/command-not-found/actions)

## Installation

```zsh
nix profile install github:goj/command-not-found
```

## Configuration

### zsh

Add this to your `~/.zshrc`

```zsh
if (( $+commands[command-not-found] )); then
  command_not_found_handler () {
      command-not-found "$@"
  }
fi
```
