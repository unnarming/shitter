## Setup

### Prerequisites
- [Rust](https://rustup.rs)
- [Tauri CLI](https://tauri.app) - `cargo install tauri-cli`
- [VS Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
    - Select "Desktop dev with C++" and "Win 10/11 SDK"
- [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (if not installed)
- [OpenSSL](https://slproweb.com/products/Win32OpenSSL.html) (Win64 full version)
    - Add `C:\Program Files\OpenSSL-Win64\bin` to PATH
    - Set environment variables:
      ```powershell
      [Environment]::SetEnvironmentVariable("OPENSSL_LIB_DIR", "C:\Program Files\OpenSSL-Win64\lib\VC\x64\MD", "User")
      [Environment]::SetEnvironmentVariable("OPENSSL_INCLUDE_DIR", "C:\Program Files\OpenSSL-Win64\include", "User")
      ```

### Build
```bash
cargo build
```

## Running

### Development
```bash
cargo tauri dev
```

### Production Build
```bash
cargo tauri build
```