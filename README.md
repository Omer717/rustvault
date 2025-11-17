---
# 🔐 rustvault

A lightweight, secure, encrypted credential vault built in pure Rust.

`rustvault` lets you safely store and manage passwords and service credentials via a simple command-line interface. All data is encrypted using **AES-256-GCM** with a securely derived master key. No cloud. No tracking. Everything stays on your machine — encrypted.
---

## ✨ Features

-   **AES-256-GCM authenticated encryption**
-   **Per-entry unique nonces** for strong security
-   **Salted key derivation** from a master password
-   **Master key verification** using a secure encrypted “vault check”
-   **Encrypted JSON vault file**
-   **Full CLI interface (list, add, edit, remove, get, export)**
-   **Optional decrypted export**
-   **Developer mode** for debugging

---

## 🔒 Security Design

### Master Key Derivation

A random salt is generated when initializing the vault.
Every time the user enters their master password, a key is derived from:

-   The password
-   The stored salt
-   A password-based KDF (PBKDF2 / Argon2 / whichever your `crypto` module uses)

### AES-256-GCM Encryption

Each password entry is encrypted with:

-   The derived master key
-   A fresh random 12-byte nonce
-   AES-GCM for **confidentiality + authentication**

This ensures encrypted data cannot be modified without detection.

### Vault Check

During initialization, the vault creates a special encrypted value containing:

```
"vault check"
```

At runtime, the key verification process:

1. Uses the derived key to decrypt the check value
2. Ensures the plaintext equals `"vault check"`

If either step fails, the password is rejected without exposing anything about the vault content.

---

## 📂 Vault Format

The vault is stored as JSON:

```json
{
    "version": 1,
    "salt": "...",
    "check": { "nonce": "...", "ciphertext": "..." },
    "entries": [
        {
            "service": "github",
            "username": "example",
            "nonce": "...",
            "ciphertext": "..."
        }
    ]
}
```

Passwords are **never stored unencrypted** unless explicitly exported with `--decrypted`.

---

## 🚀 Installation

_(Customize if you plan to publish on crates.io)_

```bash
git clone https://github.com/Omer717/rustvault.git
cd rustvault
cargo build --release
```

---

## 🧰 Usage

### Initialize a vault

```bash
rustvault init
```

### Add a password

```bash
rustvault add github myuser mypassword123
```

### List saved services

```bash
rustvault list
```

### Get and decrypt a password

```bash
rustvault get github
```

### Remove an entry

```bash
rustvault remove github
```

### Edit an entry

```bash
rustvault edit github newname newpassword
```

### Export vault

Encrypted (default):

```bash
rustvault export vault.json
```

Export fully decrypted:

```bash
rustvault export vault.json --decrypted
```

---

## 📦 Project Structure

```
src/
 ├─ crypto.rs      // AES-GCM encryption, key derivation, vault check logic
 ├─ commands.rs    // CLI subcommand implementations
 ├─ storage.rs     // Load/save vault JSON
 ├─ models.rs      // Vault, Entry, and EncryptedData ]
 └─ main.rs        // Command dispatch via clap
```

---

## ❗ Security Notes

-   Do **not** reuse your master password from real password managers.
-   Do **not** store the decrypted vault export unless you know what you are doing.
-   The vault file is only as secure as your master password.

---

## 📜 License

MIT — free to use, modify, and learn from.

---
