use std::any::Any;
//
use orchard::bundle::Flags;
use orchard::tree::{MerkleHashOrchard,Anchor};
use orchard::value::NoteValue;
use orchard::Address;
use orchard::keys::{SpendingKey, FullViewingKey, Scope, OutgoingViewingKey};
use orchard::builder::{Builder, OutputError};
use rand::Rng;
use zcash_primitives::merkle_tree::{MerklePath, self};
use zcash_primitives::transaction::components::amount;
use zcash_primitives::transaction::components::amount::Amount;
use orchard::Note;
use zcash_primitives::sapling::Node;

pub fn generate_zcash_wallet() -> (String, String) {
    let mut rng = rand::thread_rng();
    let mut random_bytes = [0u8; 32];
    rng.fill(&mut random_bytes);

    let sk = SpendingKey::from_bytes(random_bytes).expect("Failed to create spending key");
    let fvk = FullViewingKey::from(&sk);
    let address = fvk.address_at(0u32, Scope::External);

    let sk_encoded = base58::ToBase58::to_base58(&random_bytes); // Encode the key
    let addr_string = format!("{:?}", address);

    (sk_encoded, addr_string)
}



pub fn sign_transaction(
    sk: SpendingKey, 
    recipient: Address, 
    value: Amount, 
    memo: Option<String>
) -> Result<(), OutputError> {
    let fvk = FullViewingKey::from(&sk);
    let ovk = Some(fvk.to_ovk(Scope::External));

    // Example: Replace with actual anchor calculation
    let anchor = Anchor::from(MerkleHashOrchard::default()); 

    let mut builder = Builder::new(
        Flags::from_parts(true, true), // Spends and outputs enabled
        anchor
    );

    let note_value = NoteValue::from_raw(value.into());

    let memo_bytes = memo.map(|s| {
        let mut bytes = [0u8; 512];
        bytes[..s.len()].copy_from_slice(s.as_bytes());
        bytes
    });

    builder.add_recipient(ovk, recipient, note_value, memo_bytes)?;
    Ok(())
}
