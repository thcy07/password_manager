# 🦀 Rust CLI Password Manager – Technology Brief

## 📌 Overview

This project is a **Command-Line Interface (CLI) Password Manager** built using the Rust programming language. It allows users to securely store, view, and search for login credentials. The system demonstrates core Rust programming concepts, including ownership, borrowing, data structures, and error handling.

---

## 🎯 Objectives

The purpose of this project is to:

- Demonstrate proficiency in Rust fundamentals
- Implement secure data handling techniques
- Practice working with file persistence and structured data
- Build a functional and interactive CLI application

---

## ⚙️ Technologies Used

### 🦀 Rust

- Systems programming language focused on safety and performance
- Enforces memory safety through ownership and borrowing
- Used to implement all application logic

### 📦 Cargo

- Rust's package manager and build tool
- Handles dependency management and compilation

### 📚 External Crates

| Crate        | Purpose                                              |
|--------------|------------------------------------------------------|
| `serde`      | Serialization and deserialization of data            |
| `serde_json` | Converts Rust data structures to/from JSON           |
| `rpassword`  | Allows secure (hidden) input for sensitive data      |
| `base64`     | Encodes and decodes encrypted password data          |

---

## 🧠 Core Concepts Demonstrated

### 1. Variables (Mutable & Immutable)

- Mutable variables (`mut`) used for dynamic data like user input
- Immutable variables used where data should not change

### 2. Loops

- A continuous loop (`loop {}`) is used to maintain the interactive menu

### 3. Functions & Ownership

- Functions pass data using:
  - Borrowing (`&Vec<T>`, `&str`)
  - Mutable references (`&mut Vec<T>`)
- Demonstrates Rust's ownership model and memory safety

### 4. Vectors (`Vec`)

- Used to store multiple password entries dynamically

```rust
let mut entries: Vec<PasswordEntry> = Vec::new();
```

### 5. Match Statement

- Used for menu selection and control flow

```rust
match action {
    MenuOption::Add    => { ... }
    MenuOption::View   => { ... }
    MenuOption::Search => { ... }
    MenuOption::Exit   => { ... }
}
```

---

## 🧩 Data Structures

### Struct: `PasswordEntry`

```rust
struct PasswordEntry {
    site: String,
    username: String,
    password: String,
}
```

- Represents a single password record
- Stores encrypted password data

---

### Enum: `MenuOption`

```rust
enum MenuOption {
    Add,
    View,
    Search,
    Exit,
    Invalid,
}
```

- Defines possible user actions
- Improves code readability and safety

---

## 🔐 Security Implementation

### Encryption (Simplified)

- Passwords are encoded using Base64 with a key prefix
- Ensures stored passwords are not in plain text

### Decryption

- Requires a correct **admin key**
- Returns `Option<String>`:
  - `Some(password)` if key is valid
  - `None` if incorrect

---

## 🔑 Admin Access

- Users can view encrypted data by default
- Admins can enter a secret key to decrypt passwords
- Input is hidden using `rpassword`

---

## 💾 File Persistence

### JSON Storage

- Password data is saved to `passwords.json`
- Uses `serde_json` for serialization

### Load & Save

- Data is loaded at program start
- Automatically saved after modifications

---

## ⚠️ Error Handling

### Result Type

Used in file operations:

```rust
fs::read_to_string("passwords.json")
```

### Option Type

Used in search and decryption:

```rust
Option<&PasswordEntry>
Option<String>
```

---

## 🔍 Features Implemented

- Add new password entries
- View stored entries (encrypted or decrypted)
- Search by site name
- Persistent storage (JSON file)
- Hidden input for sensitive data
- Basic encryption/decryption system

---

## 🚀 Potential Improvements

- Replace Base64 with real AES encryption
- Hash and secure the admin key
- Add delete/edit functionality
- Improve UI formatting
- Implement password strength validation

---

## 🧪 Testing & Execution

### Run the program

```bash
cargo run
```

### Expected Behavior

- Menu-driven interface
- Data persists between runs
- Secure handling of sensitive input

---

## 🏁 Conclusion

This project demonstrates a strong understanding of Rust programming fundamentals while building a practical and interactive application. It combines systems programming concepts with real-world functionality such as data persistence and basic security mechanisms.
