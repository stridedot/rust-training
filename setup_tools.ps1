# ===============================================
# setup_tools.ps1 - 一键初始化 Rust + Python + Rye 开发环境
# only for Windows
# ===============================================

Write-Host "=== Installing Rust tools ==="

# 安装 Rust 常用工具
cargo install cargo-generate
cargo install --locked cargo-deny -and cargo deny init -and cargo deny check
cargo install typos-cli
cargo install git-cliff -and git cliff --init
cargo install --locked cargo-nextest

Write-Host "[OK] Rust tools installed successfully!"
Write-Host ""

# ===============================================
Write-Host "=== Setting up Rye ==="

# 检查 Rye 是否存在
if (-not (Get-Command rye -ErrorAction SilentlyContinue)) {
    Write-Host "Installing Rye..."
    irm https://rye.astral.sh/get | iex
}

# 设置 RYE_HOME
$env:RYE_HOME = "D:\Rust\rye"

# 固定 Python 3.12
rye pin cpython@3.12

# 初始化并同步 Python 环境
Write-Host "Running 'rye sync'..."
try {
    rye sync -v
    Write-Host "[OK] Rye environment synchronized successfully!"
}
catch {
    Write-Host "[Error] Rye sync failed: $($_.Exception.Message)"
}

Write-Host ""

# ===============================================
Write-Host "=== Installing Python tools ==="

# 使用国内 PyPI 镜像加速
$env:PIP_INDEX_URL = "https://mirrors.tuna.tsinghua.edu.cn/pypi/web/simple"

try {
    # 确保 pip 存在并是最新的
    rye run python -m ensurepip --upgrade
    rye run python -m pip install --upgrade pip setuptools wheel
    Write-Host "[OK] pip installed/upgraded successfully!"

    # 直接安装 pre-commit
    rye run python -m pip install pre-commit
    Write-Host "[OK] pre-commit installed successfully!"

    # 安装 pre-commit 钩子
    rye run pre-commit install
    Write-Host "[OK] pre-commit hooks installed successfully!"
}
catch {
    Write-Host "[Error] Failed to install pre-commit: $($_.Exception.Message)"
}

Write-Host ""
Write-Host "[OK] All tools installed successfully!"
