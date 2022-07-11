# Remote personal secret manager

Secret types
 * private key
 * api key
 * website/username/password

## Cryptography scheme

Upon server initialization:
 * A symmetric keypair is generated to be stored on the server
 * A user passphrase is chosen
 * It is salted and hashed
 * PBKDF is used to generate a symmetric key from the user passphrase
 * That key is used to encrypt the generated symmetric private key
 * PBKDF key is deleted (it is temporary)

Upon client getsecret request:
 * Encrypted private key is sent to the client
 * Requested secret is sent to client (encrypted)
 * The user supplies a password
 * It is salted and hashed, used to generate decryption key (PBKDF)
 * Use PBKDF key to decrypt received private key
 * Use this decrypted private key to decrypt the secret

Upon client putsecret request:
 * Server sends encrypted public key
 * Passphrase is salted and hashed and passed through PBKDF
 * PBKDF key is used to decrypt public key
 * Public key is used to encrypt the file and send to the server
 * Encrypted file is sent to server

Question: Is double-encryption with both the server key and the PBKDF key
necessary?


