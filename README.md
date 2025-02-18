#A social media DApp bridging the gap between free speech and free market.
## Tauri + Yew + Orbitdb + hardhat
#
This template should help get you started developing with Tauri and Yew.
#### - [] get keys from wallet_gen
#### - [] sign transactions for other currencies 
#### - [] send transactions to the networks.
#### - [] encrypt/store the keys safely.
#### - [] generate a backup passphrase for keys.seed phrase and store it securely

### - [] Allow users to create an account using there ethereum provider. Then prompt them to choose a username, that hasn't been taken.. Verify the uniqueness of the chosen username using OrbitDB. If unique, create a new user document in OrbitDB with the Ethereum address and username. Implement a secure authentication mechanism using the Ethereum provider for future logins.
### - [] Implement user profile management:
- Create a profile page where users can add/edit their bio, profile picture, and other details
- Store profile information in OrbitDB, linked to the user's Ethereum address
- Implement privacy settings to control visibility of profile information
- Add functionality for users to follow/unfollow other users
- Create a feed displaying posts from followed users
#### - [] make sure the wallets are properly recieving updates for transactionsand implement real-time balance updates

## - orbitdb[] add push notification feature for new messages and transactions

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
