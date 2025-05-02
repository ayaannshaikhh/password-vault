# 🔐 Rust Password Manager (CLI)

A simple command-line password manager written in Rust.  
It stores your account names and passwords in a local JSON file (`passwords.json`).

---

## ✅ Features

- Add a new account and password via command-line arguments  
- Data is saved to a local file in JSON format  
- Great starter project to learn Rust, structs, file I/O, and `serde`

---

## 🚀 Usage

```bash
cargo run -- <account> <password>
```

### Example:
```bash
cargo run -- gmail mySuperSecurePassword123
```

This will create or overwrite `passwords.json` with:
```json
{
  "gmail": "mySuperSecurePassword123"
}
```

> ⚠️ Currently, only **one entry** is stored per run. This will **overwrite** the file every time you run the program.

---

## 🧠 Concepts Practiced

- Command-line argument parsing
- Structs and HashMaps
- File read/write (`std::fs`)
- JSON serialization with `serde`

---

## 📦 Dependencies

In `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```
