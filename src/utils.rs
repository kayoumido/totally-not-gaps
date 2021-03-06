/*!
 * This file contains all the `utility` functions that may be needed in
 * multiple other functions.
 *
 * # Author
 * Doran Kayoumi <doran.kayoumi@heig-vd.ch>
 */

use sodiumoxide::crypto::pwhash::argon2id13;

/// Hash a password (or any other String) using argon2id13
///
/// # Arguments
///
/// * `passwd` - The password/string to hash
///
pub fn hash(passwd: &str) -> String {
    let pwh = argon2id13::pwhash(
        passwd.as_bytes(),
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    )
    .unwrap();

    std::str::from_utf8(&pwh.0).unwrap().to_string()
}

/// Verify that a passwords matches a hash
///
/// # Arguments
///
/// * `og_hash` - the hash that the passwords needs to match
/// * `passwd` - the password that needs to match
///
pub fn verify_hash(hash: &str, passwd: &str) -> bool {
    let hash = pad_hash(hash);
    match argon2id13::HashedPassword::from_slice(hash.as_ref()) {
        Some(hp) => argon2id13::pwhash_verify(&hp, passwd.as_bytes()),
        _ => false,
    }
}

fn pad_hash(hash: &str) -> Vec<u8> {
    let mut padded = [0u8; argon2id13::HASHEDPASSWORDBYTES];
    hash.as_bytes().iter().enumerate().for_each(|(i, val)| {
        padded[i] = val.clone();
    });

    padded.to_vec()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash() {
        let pw1 = "passwd";
        let pw2 = "verySecurePassword";

        let pwh1 = hash(pw1);
        let pwh2 = hash(pw2);

        let pwh11 = hash(pw1);
        let pwh22 = hash(pw2);

        assert_ne!(pwh1, pwh11);
        assert_ne!(pwh2, pwh22);

        assert_ne!(pwh1, pwh2);
    }

    #[test]
    fn test_verify_hash() {
        let pw1 = "passwd";
        let pw2 = "verySecurePassword";

        let pwh1 = hash(pw1);
        let pwh2 = hash(pw2);

        assert_eq!(verify_hash(&pwh1, pw1), true);
        assert_eq!(verify_hash(&pwh2, pw2), true);

        assert_eq!(verify_hash(&pwh1, pw2), false);
        assert_eq!(verify_hash(&pwh2, pw1), false);
    }

    #[test]
    fn test_pad_hash() {
        let hash = "$argon2id$v=19$m=65536,t=2,p=1$bir6dVhkHcJxiiXkWq4qjA$ON44zHno26tU7vYN4IbZ/Ifca37MArcvdZZl9iO0OSc";

        assert_ne!(hash.len(), argon2id13::HASHEDPASSWORDBYTES);
        assert_eq!(pad_hash(hash).len(), argon2id13::HASHEDPASSWORDBYTES);
    }
}
