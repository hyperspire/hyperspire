#!/bin/bash
# Install and start the hyperspire systemd service

echo "Building release binary..."
cargo build --release

echo "Installing systemd service..."
sudo cp hyperspire.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable hyperspire
sudo systemctl start hyperspire

echo "Service hyperspire installed and started."
