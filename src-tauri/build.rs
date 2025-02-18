use std::process::Command;

fn main() {
    Command::new("npx")
        .arg("ipfs")
        .arg("daemon")
        .spawn()
        .expect("Failed to start IPFS daemon");

    std::thread::sleep(std::time::Duration::from_secs(3)); // Wait for IPFS to start

    // Start OrbitDB
    Command::new("npx")
        .arg("orbit-db-server")
        .spawn()
        .expect("Failed to start OrbitDB server");

    // Start Hardhat for local Ethereum dev network
    Command::new("npx")
        .arg("hardhat")
        .arg("node")
        .spawn()
        .expect("Failed to start Hardhat node");

    // Start Tauri app
    tauri_build::build()
}
