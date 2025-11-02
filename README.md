# Pirates Quest 🏴‍☠️

A decentralized blockchain-based pirate adventure game built on Solana using the Anchor framework. Embark on high-seas adventures, collect treasure, battle other pirates, and trade resources in a fully on-chain gaming experience.

## 📖 Project Overview

Pirates Quest is a turn-based strategy game where players control pirate ships, explore the seven seas, and compete for resources and glory. All game state, assets, and logic are stored and executed on the Solana blockchain, ensuring true ownership of in-game assets and transparent gameplay mechanics.

### Key Concepts
- **Decentralized Gameplay**: All game logic runs on-chain via Solana smart contracts
- **NFT Assets**: Ships, items, and resources are represented as NFTs
- **Player-Driven Economy**: Trade, battle, and compete in a fully player-driven marketplace
- **Provably Fair**: All random events are verifiable on-chain

## ✨ Features

### Core Gameplay
- **Ship Management**: Mint and upgrade pirate ships with unique attributes
- **Exploration**: Navigate between islands and discover hidden treasures
- **Combat System**: Engage in turn-based battles with other players
- **Resource Collection**: Mine resources, loot treasure chests, and claim bounties
- **Trading System**: Buy, sell, and trade assets in the marketplace
- **Crew Management**: Recruit and manage crew members with special abilities

### Blockchain Features
- **True Asset Ownership**: All in-game items are NFTs owned by players
- **On-Chain Randomness**: Verifiable random number generation for fair gameplay
- **Staking Rewards**: Stake $PLUNDER tokens to earn rewards
- **Governance**: Token holders can vote on game updates and parameters

## 🏗️ Smart Contract Modules

### 1. Player Module (`programs/pirates_quest/src/state/player.rs`)
Handles player account creation, profile management, and experience tracking.

**Instructions:**
- `initialize_player`: Create a new player account
- `update_player_stats`: Update player experience and level
- `claim_rewards`: Claim earned rewards

### 2. Ship Module (`programs/pirates_quest/src/state/ship.rs`)
Manages ship NFTs, attributes, and upgrades.

**Instructions:**
- `mint_ship`: Mint a new pirate ship NFT
- `upgrade_ship`: Enhance ship attributes
- `repair_ship`: Restore ship health
- `transfer_ship`: Transfer ship ownership

### 3. Combat Module (`programs/pirates_quest/src/instructions/combat.rs`)
Handles battle mechanics and outcomes.

**Instructions:**
- `initiate_battle`: Start a battle between two players
- `execute_attack`: Process an attack action
- `resolve_battle`: Finalize battle results and distribute rewards

### 4. Exploration Module (`programs/pirates_quest/src/instructions/explore.rs`)
Manages map navigation and treasure discovery.

**Instructions:**
- `explore_island`: Visit a new island location
- `open_treasure`: Open discovered treasure chests
- `claim_bounty`: Complete and claim bounty missions

### 5. Marketplace Module (`programs/pirates_quest/src/instructions/marketplace.rs`)
Facilitates trading between players.

**Instructions:**
- `list_item`: List an item for sale
- `buy_item`: Purchase a listed item
- `cancel_listing`: Remove an item from sale

### 6. Staking Module (`programs/pirates_quest/src/instructions/staking.rs`)
Manages token staking and reward distribution.

**Instructions:**
- `stake_tokens`: Stake $PLUNDER tokens
- `unstake_tokens`: Withdraw staked tokens
- `claim_staking_rewards`: Claim accumulated rewards

## 📁 Folder Structure

```
pirates_quest/
├── programs/
│   └── pirates_quest/
│       ├── src/
│       │   ├── lib.rs              # Main program entry point
│       │   ├── error.rs            # Custom error definitions
│       │   ├── constants.rs        # Global constants
│       │   ├── state/              # Account state structures
│       │   │   ├── mod.rs
│       │   │   ├── player.rs       # Player account structure
│       │   │   ├── ship.rs         # Ship NFT structure
│       │   │   ├── battle.rs       # Battle state structure
│       │   │   ├── island.rs       # Island data structure
│       │   │   └── marketplace.rs  # Marketplace listing structure
│       │   ├── instructions/       # Program instructions
│       │   │   ├── mod.rs
│       │   │   ├── player.rs       # Player-related instructions
│       │   │   ├── ship.rs         # Ship-related instructions
│       │   │   ├── combat.rs       # Combat instructions
│       │   │   ├── explore.rs      # Exploration instructions
│       │   │   ├── marketplace.rs  # Trading instructions
│       │   │   └── staking.rs      # Staking instructions
│       │   └── utils/              # Helper functions
│       │       ├── mod.rs
│       │       ├── randomness.rs   # RNG utilities
│       │       └── calculations.rs # Game math calculations
│       ├── Cargo.toml              # Rust dependencies
│       └── Xargo.toml              # Build configuration
├── tests/
│   ├── pirates_quest.ts            # Integration tests
│   ├── player.spec.ts              # Player module tests
│   ├── combat.spec.ts              # Combat module tests
│   └── marketplace.spec.ts         # Marketplace tests
├── app/                            # Frontend application
│   ├── src/
│   │   ├── components/             # React components
│   │   ├── hooks/                  # Custom React hooks
│   │   ├── utils/                  # Frontend utilities
│   │   ├── idl/                    # Program IDL files
│   │   └── App.tsx                 # Main app component
│   ├── public/                     # Static assets
│   └── package.json                # Frontend dependencies
├── migrations/
│   └── deploy.ts                   # Deployment scripts
├── target/
│   ├── idl/                        # Generated IDL
│   └── deploy/                     # Compiled programs
├── Anchor.toml                     # Anchor configuration
├── package.json                    # Node.js dependencies
├── tsconfig.json                   # TypeScript configuration
└── README.md                       # This file
```

## 🚀 Setup Instructions

### Prerequisites

Before starting, ensure you have the following installed:

1. **Rust** (v1.70.0 or later)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Solana CLI** (v1.18.0 or later)
   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
   ```

3. **Anchor Framework** (v0.29.0 or later)
   ```bash
   cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
   avm install latest
   avm use latest
   ```

4. **Node.js** (v18 or later) and **Yarn**
   ```bash
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
   nvm install 18
   npm install -g yarn
   ```

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/Abhilashpatel12/pirates_quest.git
   cd pirates_quest
   ```

2. **Install dependencies**
   ```bash
   yarn install
   ```

3. **Build the program**
   ```bash
   anchor build
   ```

4. **Configure Solana CLI**
   ```bash
   # Set to devnet for development
   solana config set --url devnet
   
   # Create a new keypair (or use existing)
   solana-keygen new
   
   # Airdrop SOL for testing
   solana airdrop 2
   ```

5. **Update Program ID**
   ```bash
   # Get your program ID
   anchor keys list
   
   # Update in lib.rs and Anchor.toml with the displayed program ID
   ```

6. **Deploy to devnet**
   ```bash
   anchor deploy
   ```

7. **Run tests**
   ```bash
   anchor test
   ```

### Frontend Setup

1. **Navigate to the app directory**
   ```bash
   cd app
   ```

2. **Install frontend dependencies**
   ```bash
   yarn install
   ```

3. **Copy IDL file**
   ```bash
   cp ../target/idl/pirates_quest.json src/idl/
   ```

4. **Configure environment variables**
   ```bash
   cp .env.example .env.local
   # Edit .env.local with your program ID and RPC endpoint
   ```

5. **Start development server**
   ```bash
   yarn dev
   ```

### Local Validator Setup (Optional)

For local development without using devnet:

```bash
# Start local validator
solana-test-validator

# In a new terminal, configure to use local
solana config set --url localhost

# Deploy to local
anchor deploy --provider.cluster localnet

# Run tests against local
anchor test --skip-local-validator
```

## 🧪 Testing

### Run all tests
```bash
anchor test
```

### Run specific test files
```bash
anchor test --skip-deploy tests/player.spec.ts
```

### Generate test coverage
```bash
cargo tarpaulin --out Html
```

## 📚 Documentation

- [Anchor Framework Documentation](https://www.anchor-lang.com/)
- [Solana Documentation](https://docs.solana.com/)
- [Game Design Document](./docs/GAME_DESIGN.md)
- [API Reference](./docs/API.md)
- [Contributing Guidelines](./CONTRIBUTING.md)

## 🛣️ Roadmap

- [x] Core smart contract architecture
- [x] Player and ship management
- [x] Basic combat system
- [ ] Advanced exploration mechanics
- [ ] Marketplace implementation
- [ ] Token staking and rewards
- [ ] Frontend dApp development
- [ ] Multi-player tournaments
- [ ] Mobile app support

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guidelines](./CONTRIBUTING.md) before submitting PRs.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## 🙏 Acknowledgments

- Solana Foundation for blockchain infrastructure
- Anchor framework team for excellent developer tools
- Metaplex for NFT standards
- The open-source blockchain gaming community

## 📞 Contact

- GitHub: [@Abhilashpatel12](https://github.com/Abhilashpatel12)
- Project Link: [https://github.com/Abhilashpatel12/pirates_quest](https://github.com/Abhilashpatel12/pirates_quest)

---

**⚓ Set sail and claim your fortune on the blockchain seas! ⚓**
