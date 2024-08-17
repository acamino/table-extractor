# Shell Completions

`tabx` supports shell completions for Bash, Zsh, Fish, Elvish, and PowerShell.

## Generating Completions

To generate completions, use the `completions` subcommand:

```bash
tabx completions <SHELL>
```

Supported shells:
- `bash`
- `zsh`
- `fish`
- `elvish`
- `powershell`

## Installation

### Bash

#### Linux

```bash
# Generate and install completions
tabx completions bash | sudo tee /etc/bash_completion.d/tabx.bash

# Or for user-only installation
mkdir -p ~/.local/share/bash-completion/completions
tabx completions bash > ~/.local/share/bash-completion/completions/tabx
```

#### macOS (with Homebrew)

```bash
# Install bash-completion first (if not already installed)
brew install bash-completion@2

# Generate and install completions
tabx completions bash > $(brew --prefix)/etc/bash_completion.d/tabx
```

Then add this to your `~/.bash_profile` or `~/.bashrc`:

```bash
# For bash-completion@2
[[ -r "$(brew --prefix)/etc/profile.d/bash_completion.sh" ]] && . "$(brew --prefix)/etc/profile.d/bash_completion.sh"
```

### Zsh

```bash
# Generate completions to a file in your $fpath
tabx completions zsh > ~/.zsh/completions/_tabx

# Make sure the directory is in your fpath (add to ~/.zshrc if needed)
fpath=(~/.zsh/completions $fpath)

# Initialize completions (add to ~/.zshrc)
autoload -Uz compinit && compinit
```

Or with Oh My Zsh:

```bash
# Generate completions to Oh My Zsh custom plugins
tabx completions zsh > ~/.oh-my-zsh/custom/plugins/tabx/_tabx
```

### Fish

```bash
# Generate and install completions
tabx completions fish > ~/.config/fish/completions/tabx.fish
```

The completions will be loaded automatically on the next shell start.

### Elvish

```bash
# Generate completions
tabx completions elvish > ~/.elvish/lib/completions/tabx.elv
```

Then add to your `~/.elvish/rc.elv`:

```elvish
use completions/tabx
```

### PowerShell

```powershell
# Generate completions
tabx completions powershell | Out-String | Invoke-Expression

# To make persistent, add to your PowerShell profile
tabx completions powershell >> $PROFILE
```

## Verifying Installation

After installation, restart your shell or source the configuration file:

```bash
# Bash
source ~/.bashrc  # or ~/.bash_profile

# Zsh
source ~/.zshrc

# Fish
# No action needed - restart the shell
```

Test the completions by typing `tabx` and pressing `Tab`:

```bash
tabx --<TAB>
```

You should see completion suggestions for all available options and subcommands.

## Troubleshooting

### Completions not working in Bash

1. Ensure `bash-completion` is installed:
   ```bash
   # Ubuntu/Debian
   sudo apt-get install bash-completion

   # macOS
   brew install bash-completion@2
   ```

2. Verify bash-completion is sourced in your shell profile (`~/.bashrc` or `~/.bash_profile`)

### Completions not working in Zsh

1. Ensure the completions directory is in your `$fpath`:
   ```bash
   echo $fpath
   ```

2. Try rebuilding the completion cache:
   ```bash
   rm -f ~/.zcompdump; compinit
   ```

### Fish completions not working

1. Check that the completions file exists:
   ```bash
   ls ~/.config/fish/completions/tabx.fish
   ```

2. Restart the Fish shell

## Uninstalling Completions

### Bash

```bash
# Linux
sudo rm /etc/bash_completion.d/tabx.bash
# or
rm ~/.local/share/bash-completion/completions/tabx

# macOS
rm $(brew --prefix)/etc/bash_completion.d/tabx
```

### Zsh

```bash
rm ~/.zsh/completions/_tabx
# or for Oh My Zsh
rm ~/.oh-my-zsh/custom/plugins/tabx/_tabx
```

### Fish

```bash
rm ~/.config/fish/completions/tabx.fish
```

### Elvish

```bash
rm ~/.elvish/lib/completions/tabx.elv
```

### PowerShell

Edit your `$PROFILE` and remove the line that sources the completions.
