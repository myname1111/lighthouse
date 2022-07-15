@echo off
cargo build --release
robocopy shaders target\release\shaders
robocopy data target\release\data