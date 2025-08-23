#A Seer bridging the gap between free speech and free market.
## Tauri + Yew + Orbitdb + hardhat
#
This template should help get you started developing with Tauri and Yew.


```Install a late version of nodejs, rust and wasm```
than run
```cargo tauri dev```
#### - [] debug ai code

## - [] phase 1
### - [] User Authentication and Account Management
#### - [] Allow users to create an account using there ethereum provider.
#### - [] Then prompt them to choose a username, that hasn't been taken.
#### - [] Verify the uniqueness of the chosen username using OrbitDB. If unique, create a  new user document in OrbitDB with the Ethereum address and username. 
#### - [] Implement an authentication mechanism using the metamask for future logins.
#### - [] Store profile information in OrbitDB, linked to the user's Ethereum address

### - [] Wallet 
#### - [] get keys from wallet_gen into orbitdb and to be aviable for the user to use.
#### - [] sign transactions for other currencies 
#### - [] send transactions to the networks.
#### - [] view balances.
#### - [] encrypt/store the keys safely.
#### - [] generate a backup passphrase for accounts.seed phrase and store it securely


### - [] Implement user profile management:
#### - [] Create a profile page where users can add/edit their bio, profile picture, and other details
#### - [] Implement privacy settings to control visibility of profile information
#### - [] Add functionality for users to follow/unfollow other users
#### - [] Create a feed displaying posts from followed users
#### - [] make sure the wallets are properly recieving updates for transactions and implement real-time balance updates

### - Feed and posts.
#### - [] allow users to post text content.
#### - [] edit the CID allowing users to post larger content.
#### - [] using WASM fuzz content especially exceeding 50 MBs.
#### - [] give the user a level of authetication for there posts settings.
#### - [] Create specific routes for financial content.
#### - [] Create different routes for other content types including private ones.

### Notifications
#### - [] add push notification feature for new messages and transactions
#### - [] realtime balance updates
## Recommended IDE Setup

### Escrow service and moderation.
#### - [] generate multisignature wallets for various different crypto's
##### - [] generate addresses for those wallets on demand, especially when the user types in $.
##### - [] recieve notifications for them. 
##### - [] find commission workers & AI to mediate between both.
###### - [] upon recieving notifications give both yourself and escrows ability over transactional status. Defualt being held by escrow. 


### Cybersecurity
#### - [] sanitize and whitelist.
#### - [] Have a decent sized network.
#### - [] Have DDOS protection from too much content or requests sent out.

### DM's
#### - [] Add post quantum DM's to user profiles. 

### - [] Import third party crypto API's.
### - [] recruit other marketplace admins.

## Phase 2
### - [] referal contract for marketing.
### - [] Self hosting contracts.
#### - [] Deadman switch.
#### - [] allow for larger datatypes including video and repos after launching the self hosting contract.
### - [] upgrade encryption if you haven't done so before.

## Phase 3
#### - [] Smart contract marketplace 
##### - [] Testing template smart contracts.
##### - [] long term contracts.
##### - [] Create other forms of conditional contracts such as selling databases and groups.
##### - [] Create an Easy UI for managing groups and there contracts.


[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
