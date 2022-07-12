struct GenericPayload<T> {
    secret: T,
}

struct CredentialsPayload {
    service: String,
    username: String,
    password: String,
}

struct KeypairPayload {
    public: Vec<u8>,
    private: Vec<u8>,
}
