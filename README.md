# 🦀 Rust CLI Password Manager

## Project Description

This project is a **Command-Line Interface (CLI) Password Manager** built using Rust. The application allows users to store, view, and search for login credentials in a secure and structured way.

The program demonstrates core Rust concepts such as ownership, borrowing, vectors, structs, enums, and error handling. It also includes file persistence using JSON and a basic encryption system with admin-controlled decryption.

---

## Features

- Add new password entries (site, username, password)
- View stored passwords (encrypted or decrypted)
- Search for entries by site name
- Save and load data from a JSON file
- Basic encryption for stored passwords
- Admin key required to decrypt passwords
- Hidden input for sensitive data

---

## Technologies Used

| Technology       | Purpose                                |
|------------------|----------------------------------------|
| **Rust**         | Core programming language              |
| **Cargo**        | Build system and package manager       |
| **serde / serde_json** | Data serialization and file storage |
| **rpassword**    | Secure hidden input                    |
| **base64**       | Basic encoding for password protection |

---

## Project Structure

```
password_manager/
├── src/
│   └── main.rs
├── Cargo.toml
└── passwords.json    (created after running)
```

---

## How to Build and Run

### 1. Clone or Download the Project

### 2. Navigate to the Project Directory

```bash
cd password_manager
```

### 3. Run the Program

```bash
cargo run
```

---

## Usage

When the program runs, you will see a menu:

```
1. Add password
2. View passwords
3. Search
4. Exit
```

- **Add password** → stores encrypted data
- **View passwords** → prompts for admin key to decrypt
- **Search** → finds a specific entry
- **Exit** → closes the program

---

## File Persistence

- Data is saved to `passwords.json`
- The file is automatically created and updated
- Data is loaded each time the program starts

---

## Security Notes

- Passwords are not stored in plain text
- A basic encoding method is used for encryption
- Admin key is required to decrypt stored passwords

> ⚠️ **Note:** This project uses simplified encryption for learning purposes and is not intended for real-world security.

---

## Future Improvements

- Implement stronger encryption (AES)
- Add delete/edit functionality
- Improve CLI interface formatting
- Add password strength validation
- Hash and secure the admin key

---

## Author

**Cameron** — Applied Programming (Rust Module)
