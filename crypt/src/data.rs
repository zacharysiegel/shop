use crate::secret::SecretBase64;
use std::collections::BTreeMap;
use std::sync::LazyLock;

pub const SECRETS: LazyLock<BTreeMap<&'static str, SecretBase64>> = LazyLock::new(|| {
    let mut map: BTreeMap<&'static str, SecretBase64> = BTreeMap::new();
    map.insert(
        "ebay_cert_id_zach",
        SecretBase64 {
            nonce: String::from("mXMDz6u6Ddp/h/qf"),
            ciphertext: String::from("OEFWMp+4gpbv9Ma/YBXM9R2du+SPzj4c7mSNUBqj5Yt7VFwv4a2ABmpoSrp+q+CemAbN4Q=="),
        },
    );
    map
});
