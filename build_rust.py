import subprocess
import sys
import os
from pathlib import Path

def main():
    print("🦀 Building stark_PyRust_Chain Rust Core...")
    
    # Ensure we are in the project root
    project_root = Path(__file__).parent
    os.chdir(project_root)
    
    # Check for virtual environment
    if sys.prefix == sys.base_prefix:
        print("⚠️  Warning: Not running in a virtual environment.")
        
    try:
        # Build with Maturin
        # Using --release for optimized build, remove for debug
        cmd = [sys.executable, "-m", "maturin", "develop", "--release"]
        subprocess.check_call(cmd)
        print("✅ Rust extension built and installed successfully.")
        
    except subprocess.CalledProcessError as e:
        print(f"❌ Build failed with exit code {e.returncode}")
        sys.exit(1)
    except Exception as e:
        print(f"❌ An error occurred: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
