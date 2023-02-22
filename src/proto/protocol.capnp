# What is transferred:
# * EncServerKey
#   - make get_key route
# * EncSecret
#   - put (from client to server)
#     - will be encrypted on the client side
#   - get (from server to client)
#     - will be decrypted on the client side

@0xf07e9fe635552f88;

interface Keyserver {
    enum Tag {
        apikey      @0;
        publickey   @1;
        privatekey  @2;
        keypair     @3;
        credentials @4;
        other       @5;
    }

    struct Timestamp {
        upper @0 :UInt64; # upper half of u128
        lower @1 :UInt64; # lower half of u128
    }

    enum Scope {
        public @0;
        local  @1;
        custom @2;
    }

    # A secret header (never encrypted)
    struct Header {
        id        @0 :Data;
        label     @1 :Text;
        desc      @2 :Text;
        tag       @3 :Tag;
        creation  @4 :Timestamp;
        scope     @5 :Scope;
    }

    struct MasterKey {
        rawkey @0 :Data;
        nonce   @1 :Data;
        salt    @2 :Data;
    }

    # An encrypted secret
    struct Secret {
        header    @0 :Header;
        rawsecret @1 :Data;
    }

    getMaster @0 () -> (key: MasterKey); # server returns the master key (encrypted)
    list @1 () -> (keys: List(Header)); # server returns list of all secrets
    put @2 (secret: Secret) -> ();
    get @3 (header: Header) -> (secret: Secret); # could be partial header
    delete @4 (header: Header) -> (secret: Secret);
}
