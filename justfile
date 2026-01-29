# Entropy Zero Development Justfile
# Usage: just <command>
# Install just: cargo install just

# Default: show help
default:
    @just --list

# 🌐 Start Trunk dev server with hot reload (MAIN WORKFLOW)
web:
    @echo "🚀 Starting Trunk dev server..."
    @echo "   → Browser will auto-refresh on code changes"
    @echo "   → Press F12 to see Rust logs in browser console"
    @echo ""
    cd apps/web && trunk serve --open

# 🖥️ Run native desktop version (release)
native:
    cargo run -p entropy_zero_web --release

# 🐛 Run native debug (faster compile)
debug:
    cargo run -p entropy_zero_web

# ✅ Check all crates compile
check:
    cargo check --workspace

# 🧪 Run all tests
test:
    cargo test --workspace

# 📦 Build optimized WASM for production
build:
    cd apps/web && trunk build --release

# 🧹 Clean everything
clean:
    cargo clean
    rm -rf apps/web/dist

# 👁️ Watch mode (recompile on save)
watch:
    cargo watch -x 'check --workspace'

# 📊 Run clippy lints
lint:
    cargo clippy --workspace -- -W clippy::all

# 📝 Format all code
fmt:
    cargo fmt --all

# 🔧 Fix common issues automatically
fix:
    cargo fix --workspace --allow-dirty
    cargo fmt --all
