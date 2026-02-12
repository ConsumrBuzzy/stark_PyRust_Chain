import subprocess
import sys
import os
from pathlib import Path
import venv

REQUIRED_PYTHON_VERSION = (3, 12)

def check_python_version():
    current_version = sys.version_info[:2]
    if current_version != REQUIRED_PYTHON_VERSION:
        print(f"❌ Error: Python {REQUIRED_PYTHON_VERSION[0]}.{REQUIRED_PYTHON_VERSION[1]} is required.")
        print(f"   Current version is {current_version[0]}.{current_version[1]}")
        sys.exit(1)
    print(f"✅ Python {current_version[0]}.{current_version[1]} detected.")

def create_venv(venv_path):
    print(f"🔨 Creating virtual environment at {venv_path}...")
    venv.create(venv_path, with_pip=True)
    print("✅ Virtual environment created.")

def install_dependencies(venv_python):
    print("📦 Installing dependencies...")
    subprocess.check_call([venv_python, "-m", "pip", "install", "--upgrade", "pip"])
    subprocess.check_call([venv_python, "-m", "pip", "install", "maturin", "typer", "rich", "pydantic"])
    print("✅ Dependencies installed.")

def main():
    check_python_version()
    
    project_root = Path(__file__).parent
    venv_dir = project_root / "venv"
    
    if sys.platform == "win32":
        venv_python = venv_dir / "Scripts" / "python.exe"
    else:
        venv_python = venv_dir / "bin" / "python"

    if not venv_dir.exists():
        create_venv(venv_dir)
    
    if not venv_python.exists():
        print(f"❌ Error: Python binary not found at {venv_python}")
        sys.exit(1)

    install_dependencies(str(venv_python))
    
    print("\n🚀 Environment setup complete.")
    print(f"   To activate: source venv/bin/activate (Linux/Mac) or .\\venv\\Scripts\\activate (Windows)")
    print("   Then run: python build_rust.py")

if __name__ == "__main__":
    main()
