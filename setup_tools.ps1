# ===============================================
# setup_tools.ps1 - 一键初始化 Rust + Python + uv 开发环境
# only for Windows
# ===============================================

Write-Host "=== Installing Rust tools ==="

# 安装 Rust 常用工具
cargo install cargo-generate
cargo install --locked cargo-deny
cargo deny init
cargo deny check
cargo install typos-cli
cargo install git-cliff
git cliff --init
cargo install --locked cargo-nextest

Write-Host "[OK] Rust tools installed successfully!"
Write-Host ""

# ===============================================
$uvInstallDir = "D:\Devs\lang\rust\uv"
$env:UV_INSTALL_DIR = $uvInstallDir
$env:PATH = "$uvInstallDir;$env:PATH"

# === 安装 uv（如未安装）===
if (-not (Get-Command uv -ErrorAction SilentlyContinue)) {
    Write-Host "Installing uv to $uvInstallDir..."
    New-Item -ItemType Directory -Path $uvInstallDir -Force | Out-Null
    irm https://astral.sh/uv/install.ps1 | iex
}

# === 设置 pip 镜像（可选，加速国内安装）===
$env:UV_PYTHON_PIP_INDEX_URL = "https://mirrors.aliyun.com/pypi/simple/"

# === 创建带 pip 的虚拟环境 ===
uv venv --python 3.12 --seed

# === 安装 pre-commit ===
uv pip install pre-commit

# === 安装 Git 钩子 ===
uv run pre-commit install

Write-Host ""
Write-Host "[OK] uv, virtual environment, and pre-commit are ready!"

Write-Host ""
Write-Host "[OK] All tools installed successfully!"
