#!/usr/bin/env -S just --justfile

# Set shell configurations
set windows-shell := ["powershell"]
set shell := ["bash", "-cu"]

setup:
    curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
    cargo binstall taplo-cli  -y
    @echo '✅ Setup complete!'

ready:
  just fmt    
  just lint 
  @echo '✅ All passed!'

fmt:
    cargo fmt --all -- --emit=files
    taplo fmt **/Cargo.toml
    @echo '✅ Format complete!'

lint: 
    cargo clippy    
    @echo '✅ Lint complete!'

review:
    cargo insta review
    @echo '✅ Review complete!'

doc:
    cargo doc --no-deps --open
    @echo '✅ Doc complete!'
