# A `command-not-found` for the new nix CLI

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
