# 🦜 Popat Installation Guide

## Quick Installation (Recommended)

### Option 1: Via Python/pip (Coming Soon)
```bash
pip install popat
popat --start noise  # Start with sassy attitude!
```

### Option 2: From Source (Current)

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Clone and build Popat**:
   ```bash
   git clone <repository-url>
   cd popat
   cargo build --release
   ```

3. **Install globally** (optional):
   ```bash
   cargo install --path cli
   ```

4. **Start using Popat**:
   ```bash
   # Start daemon mode with sassy attitude
   popat start noise
   # Output: 🦜 watching you chill...jeeezzz
   
   # Now run any code and get sassy hints!
   python -c "print('"  # Popat will catch this error!
   ```

## 🚀 Quick Start

1. **Start Popat daemon**:
   ```bash
   popat --start noise
   # 🦜 watching you chill...jeeezzz
   ```

2. **Run code with errors** - Popat automatically catches them:
   ```bash
   python -c "print('"           # Missing quote
   node -e "console.log(x)"      # Undefined variable
   javac MissingFile.java        # File not found
   ```

3. **Get instant sassy feedback**:
   ```
   🦜 Popat squawks: Oh please... 'x' doesn't exist and you know it! 😤
   🔧 Quick fix: Check the spelling of 'x'
   💡 Pro tip: Use your IDE's autocomplete to avoid typos!
   ```

## 🎭 Usage Examples

### Daemon Mode (Background Monitoring)
```bash
# Sassy startup
popat --start --noise
# 🦜 watching you chill...jeeezzz

# Normal startup  
popat --start
# 🦜 Popat daemon starting... ready to catch your mistakes!

# Stop daemon
popat stop
# 🦜 Daemon stopped. No more sassy comments from me!
```

### Direct Error Analysis
```bash
# Analyze specific errors
popat analyze "SyntaxError: unexpected EOF" --language python
popat analyze --file error.log --language javascript

# With different personalities
popat --personality sarcastic analyze "NameError: undefined"
popat --personality encouraging analyze "SyntaxError: missing ;"
```

### Configuration & Setup
```bash
# Set up shell integration
popat setup --shell bash
popat setup --shell zsh
popat setup --shell fish

# View stats and config
popat stats
popat config show

# Interactive testing
popat interactive
```

## 🔧 Shell Integration

Popat can integrate with your shell to automatically catch errors:

### Bash/Zsh
```bash
popat setup --shell bash
source ~/.bashrc  # or ~/.zshrc
```

### Fish
```bash
popat setup --shell fish
```

### PowerShell (Windows)
```powershell
popat setup --shell powershell
```

## 🎯 Language Support

Popat automatically detects and supports:
- **Python**: SyntaxError, NameError, IndentationError
- **JavaScript/Node.js**: ReferenceError, TypeError, SyntaxError  
- **Java**: Compilation errors, ClassNotFoundException
- **Rust**: Compilation errors, borrow checker errors
- **C/C++**: Compilation errors, linker errors
- **Go**: Compilation errors, runtime panics

## 🎭 Personality Types

- **Encouraging** 😊: "You're so close! Keep going!"
- **Sarcastic** 😏: "Oh please... 'x' doesn't exist and you know it!"
- **Educational** 🎓: "This runtime error indicates..."
- **Professional** 💼: "Runtime error: variable not defined"
- **Silly** 🤪: "SQUAWK! Your code flew the coop!"

## 🔍 How It Works

1. **Background Daemon**: Lightweight process monitors terminal activity
2. **Shell Hooks**: Capture command outputs and error codes
3. **Pattern Matching**: Advanced regex identifies error types
4. **Smart Responses**: Context-aware, personality-driven suggestions
5. **Learning**: Improves over time based on your patterns

## 🐛 Troubleshooting

### Daemon Won't Start
```bash
# Check if already running
ps aux | grep popat

# Stop existing daemon
popat stop

# Try starting again
popat start
```

### Shell Integration Not Working
```bash
# Re-run setup
popat setup --shell bash --remove
popat setup --shell bash

# Restart terminal
source ~/.bashrc
```

### Errors Not Being Caught
```bash
# Check daemon status
popat stats

# Verify shell hooks are installed
popat setup --shell bash  # Should say "already installed"

# Test manually
POPAT_ERROR_TEXT="test error" popat
```

## 🤝 Contributing

Want to add more sass? Contribute at: https://github.com/popat-project/popat

---

**Ready to make debugging fun?** Start with `popat --start noise` and let the sassy parrot roast your code! 🦜🔥