use pgp::composed::{Deserializable, Message, SignedPublicKey};
use std::error::Error;
use std::path::Path;

pub fn sign() {}

/// Check that the given artifact was signed by the private key.
pub fn verify<P>(path: P) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let public_key = SignedPublicKey::from_bytes(&include_bytes!("public.key")[..])?;

    let (mut msg, _) = Message::from_armor_file(path.as_ref())?;

    msg.verify_read(&public_key)?;
    Ok(())
}
