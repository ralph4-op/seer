## Project Structure

- `src/`: Contains the Rust source code for the Yew frontend.
- `contracts/`: Smart contracts and wallet generation logic.
- `src-tauri/`: Tauri-specific configurations and Rust backend for desktop integration.
- `public/`: Static assets.
- `styles.css`: Global styles.
- `src/scripts`: contains logic for the database and p2p network.
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
   cargo install tauri-cli

   ```

### Running the DApp

To run the DApp in development mode:

```bash
cd src-tauri
cargo tauri dev
```

###
The contracts folder is for all things web3 except connecting to the networks or databases.