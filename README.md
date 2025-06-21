# 🔗 Solana Contracts

Welcome to the **Solana Contracts** — a centralized monorepo of smart contracts built with:

* ⚙️ **Native Solana programs** using `solana-program` (low-level, manual)
* ⚓ **Anchor framework** for rapid and secure Solana development (macro-powered)

Whether you're experimenting with Solana's core primitives or building robust on-chain programs with Anchor, this monorepo is structured to support it all.

---

## 🗂 Project Structure

```txt
solana-contracts/
├── native-contracts/             # 🛠 Native Solana programs (manual, no Anchor)
│   ├── counter/                  # ➕ A simple counter contract
│   └── vault/                    # 🔐 Token vault (upcoming)
│
├── anchor-contracts/             # 🚀 Anchor framework-based programs
│   ├── calculator/               # 🧮 A basic calculator with Anchor
│   └── registry/                 # 📇 On-chain user registry (upcoming)
│
├── .gitignore                    # Optimized to ignore Rust, Anchor, editor junk
└── README.md                     # You're here
```

---

## 🚀 Quick Start

### 🧱 Native Contracts (Manual)

```bash
cd native-contracts/counter
cargo build-bpf           # Compiles to Solana BPF
solana-test-validator     # Run local Solana node
```

Use `cargo test-bpf` or deploy to Devnet with the Solana CLI.

---

### ⚓ Anchor Contracts (Framework)

```bash
cd anchor-contracts/calculator
anchor build              # Compile the program
anchor test               # Run tests
anchor deploy             # Deploy (requires wallet + cluster config)
```

Anchor automatically generates IDL + TypeScript clients for frontend integration.

---

## ➕ Adding New Contracts

### Native:

```bash
cd native-contracts
cargo new my-program --lib
```

### Anchor:

```bash
cd anchor-contracts
anchor init my-anchor-program
```

📝 Don't forget to update this `README.md` with a short description of your new contract!

---

## 📦 Requirements

| Tool       | Description            | Install Command                                                    |      |
| ---------- | ---------------------- | ------------------------------------------------------------------ | ---- |
| Solana CLI | Core CLI tools         | `sh -c "$(curl -sSfL https://release.solana.com/v1.18.0/install)"` |      |
| Rust       | Language & tooling     | \`curl [https://sh.rustup.rs](https://sh.rustup.rs) -sSf           | sh\` |
| Anchor CLI | Anchor CLI & framework | `npm install -g @coral-xyz/anchor`                                 |      |

---

## 👨‍💻 Maintainers

| Name         | GitHub                                         |
| ------------ | ---------------------------------------------- |
| Shrid Mishra | [@shridmishra](https://github.com/shridmishra) |

---

## 📜 License

Licensed under the MIT License.
You are free to use, modify, and distribute this code with attribution.

---



## 🧠 Tip

To clone this monorepo with all its dependencies:

```bash
git clone https://github.com/shridmishra/solana-contracts.git
```

---

Happy hacking! ⚡
