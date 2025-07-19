use crate::secret::SecretBase64;
use std::collections::BTreeMap;
use std::sync::LazyLock;

pub const SECRETS: LazyLock<BTreeMap<&'static str, SecretBase64>> = LazyLock::new(|| {
    let mut map: BTreeMap<&'static str, SecretBase64> = BTreeMap::new();
    map.insert(
        "ebay__zach.sandbox.cert_id",
        SecretBase64 {
            nonce: String::from("vzB3tLkRz0xnv4Ud"),
            ciphertext: String::from("BZjNxWZfKjDhbsuB87qPtW1pDGgUd7yI6iHXGDK04TYgTKfLDlbgaGyFKOVlnaBF7YRt7Q=="),
        },
    );
    map.insert(
        "authelia__identity_validation.reset_password.jwt_secret",
        SecretBase64 {
            nonce: String::from("itPCR0zyhoX9prEn"),
            ciphertext: String::from("648vVA5gA/DH5f+ipxJFUjZYssu5hudOA8t2DiTkNLfgyo7yC8gbuXUKVbPKtp/OzCpVqpNTSOuHJfXU7s7QZTYmafKar1PwL663V6wlc2Q="),
        },
    );
    map.insert(
        "authelia__session.secret",
        SecretBase64 {
            nonce: String::from("c72hP+mWcw8lBW1N"),
            ciphertext: String::from("Zwz06lw20bV0wUjqR8wL5immtGofakPaPJETRtOBKguaocbIlnjGXjRrtfSvWho14CfK5eJk+YRAHy5bApkdapZ/YLmw8EX6gWam0/W05YE="),
        },
    );
    map.insert(
        "authelia__storage.encryption_key",
        SecretBase64 {
            nonce: String::from("sMZkqdpKs5XxNC5q"),
            ciphertext: String::from("tUoAtBedRLKweHrXcN8JA3zFvp1AZBmiZvQ3bxD+mJPBcj8uWZLphpA7ojtqhdb8LoXIGXnaS4D8v2veIu/4AX8y2clyBkooIKLs1EW886I="),
        },
    );
    map.insert(
        "authelia__storage.postgres.password",
        SecretBase64 {
            nonce: String::from("I/F3oqpYXEmTO7O4"),
            ciphertext: String::from("CoK1ebFHA/PvE5FMnWpOqGvV0HXiDKvILhxQ/2h6kH4+og60hS+34eP7zlwLj2rJgr78QKhQs2S/JdEfn8JZ1P3kFZ6b0FZGJRzVxd95TqY="),
        },
    );
    map.insert(
        "postgres__user.shop.password",
        SecretBase64 {
            nonce: String::from("mBwYxQkQv1e7itjC"),
            ciphertext: String::from("mmkNZyyYI+YWwhIJ2OQSqv9Hd+G0FiVlUYnXx53nfoMBtqKFcDfV1TEu380NzSMBjPNfG0Ca2SM4LobpjG4pSbtNmlJoWfsPuqhMqVQRT/0="),
        },
    );
    map
});
