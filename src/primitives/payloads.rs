#[derive(Debug)]
pub struct GenericPayload<T> {
    secret: T,
}

#[derive(Debug)]
pub struct CredentialsPayload {
    service: String,
    username: String,
    password: String,
}

#[derive(Debug)]
pub struct KeypairPayload {
    public: Vec<u8>,
    private: Vec<u8>,
}

trait Payload {}

//impl<T> Payload for GenericPayload<T> {}
//impl<T> Payload for CredentialsPayload {}
//impl<T> Payload for KeypairPayload {}
