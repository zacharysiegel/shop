use crate::secret::SecretBase64;
use std::collections::BTreeMap;
use std::sync::LazyLock;

pub const SECRETS: LazyLock<BTreeMap<&'static str, SecretBase64>> = LazyLock::new(|| {
    let mut map: BTreeMap<&'static str, SecretBase64> = BTreeMap::new();
    map.insert(
        "ebay_cert_id_zach",
        SecretBase64 {
            nonce: String::from("vzB3tLkRz0xnv4Ud"),
            ciphertext: String::from("BZjNxWZfKjDhbsuB87qPtW1pDGgUd7yI6iHXGDK04TYgTKfLDlbgaGyFKOVlnaBF7YRt7Q=="),
        },
    );
    map
});
