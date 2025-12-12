# 🧠 Brainfuck Interpreter (Rust)

A **Brainfuck interpreter** written in **Rust**, simple, fast, and usable as a **system command**.

---

## ✨ Features

* Full Brainfuck language support (`><+-.,[]`)
* Automatically ignores spaces, tabs, newlines, and comments
* Executable via the `brainfuck` command
* Clean architecture: tokenizer → parser → virtual machine

---

## 📦 Requirements

* Rust installed ([https://rustup.rs](https://rustup.rs))

Check with:

```bash
rustc --version
cargo --version
```

---

## 🚀 Installation

### Local installation

```bash
git clone https://github.com/Clesiaaa/BrainFuck-Interpreter
cd BrainFuck-Interpreter
cargo build --release
```

Then copy the compiled binary to your Cargo bin directory:

```bash
cp target/release/brainfuck ~/.cargo/bin/
```

Make sure `~/.cargo/bin` is in your `$PATH`.

---

### Option 2 — Quick test without installation

```bash
cargo run program.bf
```

---

## ▶️ Usage

```bash
brainfuck program.bf
```

### Example

```bash
echo ">++++[<++++++++>-]<+." > hello.bf
brainfuck hello.bf
```

---

## 🗂 Project structure

```text
src/
├── main.rs              # Entry point, CLI handling
├── tokenizer.rs         # Brainfuck filtering and tokenization
├── parser.rs            # Program construction
└── virtual_machine.rs   # Program execution
```

---

## 🛠 Manual compilation

```bash
cargo build --release
```

Generated binary:

```text
target/release/brainfuck
```

Run directly:

```bash
./target/release/brainfuck program.bf
```

---

## 📚 About Brainfuck

Brainfuck is a minimalist esoteric programming language made of only 8 instructions:

| Instruction | Description                            |
| ----------- | -------------------------------------- |
| `>`         | Move the pointer to the right          |
| `<`         | Move the pointer to the left           |
| `+`         | Increment the current cell             |
| `-`         | Decrement the current cell             |
| `.`         | Output the current cell as a character |
| `,`         | Read one character of input            |
| `[`         | Start loop                             |
| `]`         | End loop                               |

All other characters are ignored.

---

## 🧩 Possible improvements

* Debug mode (`--debug`)
* Step-by-step execution
* Configurable memory size
* Input file support

---

## 📄 License

Free project — do whatever you want 😄

---

## 👤 Author

Developed by ROBERT FOLGA
