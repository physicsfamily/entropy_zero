# Entropy Zero Development Workflow

.PHONY: help web native check test clean

# Default target
help:
	@echo "╔═══════════════════════════════════════════════════════════════╗"
	@echo "║           Entropy Zero - Development Commands                 ║"
	@echo "╠═══════════════════════════════════════════════════════════════╣"
	@echo "║  make web      - Start Trunk dev server (hot reload)          ║"
	@echo "║  make native   - Run native desktop version                   ║"
	@echo "║  make check    - Check all crates compile                     ║"
	@echo "║  make test     - Run all tests                                ║"
	@echo "║  make clean    - Clean build artifacts                        ║"
	@echo "╚═══════════════════════════════════════════════════════════════╝"

# 🌐 Web development with hot reload (THE MAIN WORKFLOW)
web:
	@echo "🚀 Starting Trunk dev server..."
	@echo "   → Browser will auto-refresh on code changes"
	@echo "   → Press F12 to see Rust logs in browser console"
	@echo "   → Use egui panels to tweak parameters live"
	@echo ""
	cd apps/web && trunk serve --open

# 🖥️ Native desktop (faster iteration, no WASM overhead)
native:
	@echo "🖥️  Running native desktop version..."
	cargo run -p entropy_zero_web --release

# Native debug build (faster compile, slower runtime)
native-debug:
	@echo "🐛 Running native debug version..."
	cargo run -p entropy_zero_web

# ✅ Quick compile check (no linking)
check:
	@echo "🔍 Checking all crates..."
	cargo check --workspace

# 🧪 Run all tests
test:
	@echo "🧪 Running tests..."
	cargo test --workspace

# 📦 Build for production (WASM)
build-web:
	@echo "📦 Building optimized WASM..."
	cd apps/web && trunk build --release

# 🧹 Clean build artifacts
clean:
	@echo "🧹 Cleaning..."
	cargo clean
	rm -rf apps/web/dist

# 📊 Watch mode for a specific crate (recompile on change)
watch:
	@echo "👁️  Watching for changes..."
	cargo watch -x 'check --workspace'
