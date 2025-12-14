## Quick Start

```bash
# 1. Clone and setup
git clone <repo>
cd trading-signals-backend

# 2. Auto-setup development wallet
bash scripts/setup-dev-wallet.sh

# 3. Fund your devnet wallet (if needed)
solana airdrop 2 $(solana address --keypair .wallets/devnet-keypair.json)

# 4. Build and run
cargo run