use std::default::Default;
use std::net::Ipv4Addr;

/// The scope of which systems are allowed to access a secret
enum Scope {
    /// Any client
    Public,

    /// Only systems on the local network
    Local,

    /// A custom set of IPs
    Custom(Vec<Ipv4Addr>),
}

/// A unique secret ID
#[derive(Default)]
struct SecretID(Vec<u8>);

/// Secret metadata information contained in every secret
struct Header {
    /// The unique identifier
    id: SecretID,

    /// A label
    label: String,

    /// A description of the secret
    description: String,

    /// Epoch in ms of creation time
    creation: u128,

    /// Expiration time (0 for no expiration)
    expiration: u64,

    /// The secret's scope
    scope: Scope,

    /// This secret's tag
    tag: Tag,
}

/// Tag different kinds of secrets for sorting
enum Tag {
    // General payload types
    APIKey(Header, GenericPayload<String>),
    PublicKey(Header, GenericPayload<Vec<u8>>),
    PrivateKey(Header, GenericPayload<Vec<u8>>),
    Other(Header, GenericPayload<Vec<u8>>),

    // Specific payload types
    Keypair(Header, KeypairPayload),
    Credentials(Header, CredentialsPayload),
}
