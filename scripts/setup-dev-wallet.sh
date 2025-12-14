#!/bin/bash

echo "🔧 Setting up development environment..."
echo ""

# Create wallets directory
mkdir -p .wallets

# Generate wallet if doesn't exist
if [ ! -f .wallets/devnet-keypair.json ]; then
    echo "🆕 Creating new devnet wallet..."
    solana-keygen new --outfile .wallets/devnet-keypair.json --no-bip39-passphrase
    
    ADDRESS=$(solana address --keypair .wallets/devnet-keypair.json)
    echo "✅ Wallet created: $ADDRESS"
    echo ""
    echo "💰 Get devnet SOL:"
    echo "  solana airdrop 2 $ADDRESS"
    echo ""
else
    ADDRESS=$(solana address --keypair .wallets/devnet-keypair.json)
    echo "📁 Using existing wallet: $ADDRESS"
fi

# Create .env if doesn't exist
if [ ! -f .env ]; then
    echo "📝 Creating .env file from template..."
    cp .env.example .env
    echo "✅ .env file created"
    echo "ℹ️  Edit .env if you need custom settings"
else
    echo "📁 .env already exists"
fi

echo ""
echo "🎯 Setup complete!"
echo "Run: cargo run"
