# 项目配置
APP_NAME := Hedwig
TARGET_DIR := target/release/bundle/osx
SOURCE_APP := $(TARGET_DIR)/$(APP_NAME).app
INSTALL_DIR := /Applications
DMG_NAME := $(APP_NAME)_Installer.dmg

# 默认任务：执行检查并打包
all: check bundle

# ==============================================================================
# 开发检查 (Code Checks)
# ==============================================================================
 
.PHONY: check clippy fmt udeps lint debug

debug:
	RUST_LOG=debug cargo run

update:
	cargo upgrade --compatible

update-dry-run:
	cargo upgrade --dry-run --compatible

# 基础编译检查
check:
	@echo "🔍 Running cargo check..."
	cargo check

# Linter 检查 (Clippy) - 甚至会把警告当做错误处理，保证代码质量
clippy:
	@echo "🔍 Running cargo clippy..."
	cargo clippy --all-targets --all-features -- -D warnings

# 格式检查 (Format) - 检查代码是否符合标准格式
fmt:
	@echo "🔍 Checking code formatting..."
	cargo fmt --all -- --check

# 未使用依赖检查 (Udeps) - 需要 Nightly 工具链
# 前置条件: cargo install cargo-udeps
udeps:
	@echo "🔍 Checking for unused dependencies..."
	cargo +nightly udeps

# 综合检查：一次性运行所有静态分析
lint: fmt clippy udeps

# ==============================================================================
# 构建与打包 (Build & Bundle)
# ==============================================================================

.PHONY: build bundle clean

# 普通发布版构建
build:
	@echo "🦀 Building release binary..."
	cargo build --release

# macOS App 打包
bundle:
	@echo "📦 Bundling macOS .app..."
	cargo bundle --release

# 清理构建产物
clean:
	@echo "🧹 Cleaning target directory..."
	cargo clean

# ==============================================================================
# 安装与卸载 (Install & Uninstall)
# ==============================================================================

.PHONY: install uninstall run

# 自动安装到 /Applications
# 依赖 'bundle' 任务，确保安装的是最新构建的版本
install: bundle
	@echo "Installing $(APP_NAME) to $(INSTALL_DIR)..."
	@if [ -d "$(INSTALL_DIR)/$(APP_NAME).app" ]; then \
		echo "   Removing existing app..."; \
		rm -rf "$(INSTALL_DIR)/$(APP_NAME).app"; \
	fi
	@cp -R "$(SOURCE_APP)" "$(INSTALL_DIR)/"
	@echo "Installation complete! You can find $(APP_NAME) in Launchpad."

# 自动卸载
uninstall:
	@echo "Uninstalling $(APP_NAME)..."
	@rm -rf "$(INSTALL_DIR)/$(APP_NAME).app"
	@echo "Uninstalled successfully."

# 运行已打包的 App (用于测试打包后的效果)
run: bundle
	@echo "Running $(APP_NAME).app..."
	@open "$(SOURCE_APP)"

dmg: bundle
	@echo "💿 Creating DMG installer..."
	@rm -f "$(TARGET_DIR)/$(DMG_NAME)"
	@create-dmg \
	  --volname "$(APP_NAME) Installer" \
	  --volicon "./src/assets/app_icon.icns" \
	  --window-pos 200 120 \
	  --window-size 600 300 \
	  --icon-size 100 \
	  --icon "$(APP_NAME).app" 175 120 \
	  --hide-extension "$(APP_NAME).app" \
	  --app-drop-link 425 120 \
	  "$(TARGET_DIR)/$(DMG_NAME)" \
	  "$(SOURCE_APP)"
	@echo "✅ DMG created at: $(TARGET_DIR)/$(DMG_NAME)"
