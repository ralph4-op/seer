## Project Structure

- `src/`: Contains the Rust source code for the Yew frontend.
- `contracts/`: Smart contracts and wallet generation logic.
- `src-tauri/`: Tauri-specific configurations and Rust backend for desktop integration.
- `public/`: Static assets.
- `styles.css`: Global styles.

## Development Setup

### Prerequisites

- Rust (with `wasm32-unknown-unknown` target)
- Node.js & npm
- `wasm-pack`
- `trunk`
- Tauri CLI (`@tauri-apps/cli`)

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/ralph4-op/seer.git
   cd seer
   ```

2. **Install Rust toolchain and WASM target:**
   ```bash
   curl --proto =https --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add wasm32-unknown-unknown
   ```

3. **Install `wasm-pack`:**
   ```bash
   cargo install wasm-pack
   ```

4. **Install `trunk`:**
   ```bash
   cargo install trunk
   ```

5. **Install Node.js dependencies:**
   ```bash
   npm install
   ```

6. **Install Tauri CLI:**
   ```bash
   npm install -g @tauri-apps/cli
   ```

### Running the DApp

To run the DApp in development mode:

```bash
cd src-tauri
cargo tauri dev
```

## To-Do List (from original README.md)

This section outlines the remaining tasks for building the DApp, based on the original `README.md` from the GitHub repository.
## WARNING a lot of this code is currently under review and not yet ready for deployment.
- [x] debug ai code
- [ ] phase 1
  - [ ] User Authentication and Account Management
    - [x] Allow users to create an account using there ethereum provider.
    - [x] Then prompt them to choose a username, that hasn\"t been taken.
    - [x] Verify the uniqueness of the chosen username using OrbitDB. If unique, create a new user document in OrbitDB with the Ethereum address and username.
    - [x] Implement an authentication mechanism using the metamask for future logins.
    - [x] Store profile information in OrbitDB, linked to the user\"s Ethereum address
  - [ ] Wallet
    - [x] get keys from wallet_gen into orbitdb and to be aviable for the user to use.
    - [x] sign transactions for other currencies
    - [x] send transactions to the networks.
    - [x] view balances.
    - [x] encrypt/store the keys safely.
    - [x] generate a backup passphrase for accounts.seed phrase and store it securely
  - [ ] Implement user profile management:
    - [x] Create a profile page where users can add/edit their bio, profile picture, and other details
    - [x] Implement privacy settings to control visibility of profile information
    - [ ] Add functionality for users to follow/unfollow other users
    - [ ] Create a feed displaying posts from followed users
    - [ ] make sure the wallets are properly recieving updates for transactions and implement real-time balance updates
  - [ ] Feed and posts.
    - [ ] allow users to post text content.
    - [ ] edit the CID allowing users to post larger content.
    - [ ] using WASM fuzz content especially exceeding 50 MBs.
    - [ ] give the user a level of authetication for there posts settings.
    - [ ] Create specific routes for financial content.
    - [ ] Create different routes for other content types including private ones.
  - [ ] Notifications
    - [ ] add push notification feature for new messages and transactions
    - [ ] realtime balance updates
  - [ ] Escrow service and moderation.
    - [ ] generate multisignature wallets for various different crypto\"s
      - [ ] generate addresses for those wallets on demand, especially when the user types in $.
      - [ ] recieve notifications for them.
      - [ ] find commission workers \u0026 AI to mediate between both.
      - [ ] upon recieving notifications give both yourself and escrows ability over transactional status. Defualt being held by escrow.
  - [ ] Cybersecurity
    - [ ] sanitize and whitelist.
    - [ ] Have a decent sized network.
    - [ ] Have DDOS protection from too much content or requests sent out.
  - [ ] DM\"s
    - [ ] Add post quantum DM\"s to user profiles.
  - [ ] Import third party crypto API\"s.
  - [ ] recruit other marketplace admins.
- [ ] Phase 2
  - [ ] referal contract for marketing.
  - [ ] Self hosting contracts.
    - [ ] Deadman switch.
    - [ ] allow for larger datatypes including video and repos after launching the self hosting contract.
  - [ ] upgrade encryption if you haven\"t done so before.
- [ ] Phase 3
  - [ ] Smart contract marketplace
    - [ ] Testing template smart contracts.
    - [ ] long term contracts.
    - [ ] Create other forms of conditional contracts such as selling databases and groups.
    - [ ] Create an Easy UI for managing groups and there contracts.