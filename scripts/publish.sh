#!/bin/bash

# Publishing script for clau.rs workspace

set -e

echo "🚀 Publishing clau.rs to crates.io..."

# Check if logged in to crates.io
echo "📋 Checking crates.io login..."
if ! cargo login --help > /dev/null 2>&1; then
    echo "❌ Please login to crates.io first with: cargo login YOUR_API_TOKEN"
    exit 1
fi

# Dry run first (only for crates with no dependencies)
echo "🧪 Performing dry run..."
cargo publish --dry-run -p clau-core --allow-dirty
cargo publish --dry-run -p clau-mcp --allow-dirty
cargo publish --dry-run -p clau-macros --allow-dirty

echo "ℹ️  Note: clau-runtime and clau cannot be dry-run tested until dependencies are published"

echo "✅ Dry run successful!"

# Confirm before publishing
read -p "🤔 Ready to publish? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Publishing cancelled"
    exit 0
fi

echo "📦 Publishing crates in dependency order..."

# Publish in dependency order
echo "Publishing clau-core..."
cargo publish -p clau-core

echo "⏳ Waiting for clau-core to be available..."
sleep 30

echo "Publishing clau-mcp..."
cargo publish -p clau-mcp

echo "Publishing clau-macros..."
cargo publish -p clau-macros

echo "⏳ Waiting for dependencies to be available..."
sleep 30

echo "Publishing clau-runtime..."
cargo publish -p clau-runtime

echo "⏳ Waiting for clau-runtime to be available..."
sleep 30

echo "Publishing main clau crate..."
cargo publish -p clau

echo "🎉 All crates published successfully!"
echo "📚 Documentation will be available at https://docs.rs/clau in a few minutes"
echo "📦 Your crate is available at https://crates.io/crates/clau"