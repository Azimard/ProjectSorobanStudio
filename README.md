# 🍽️ Restaurant Ingredient Inventory — Soroban Smart Contract

A smart contract built on the **Stellar blockchain** using **Soroban SDK** to manage restaurant ingredient inventory in a transparent, decentralized, and tamper-proof way.

---

## 📋 Description

This project is a modification of a basic Notes contract, transformed into a **Restaurant Ingredient Inventory System**. It allows restaurant operators to track ingredient stock levels, receive low-stock alerts, and manage all inventory data on-chain.

---

## ✨ Features

| Function | Type | Description |
|---|---|---|
| `add_ingredient` | Write | Add a new ingredient to the inventory |
| `get_all` | Read | Retrieve all ingredients in the inventory |
| `get_by_id` | Read | Find a specific ingredient by its ID |
| `get_low_stock` | Read | Get all ingredients below minimum stock level |
| `restock` | Write | Increase stock quantity (e.g. after purchasing) |
| `use_stock` | Write | Decrease stock quantity (e.g. kitchen usage) |
| `update_ingredient` | Write | Update ingredient name, unit, category, or minimum quantity |
| `remove_ingredient` | Write | Remove an ingredient from the inventory |

### Data Structure

Each ingredient stores:
- `id` — unique random identifier (u64)
- `name` — ingredient name
- `quantity` — current stock amount
- `min_quantity` — minimum threshold before low stock alert
- `unit` — unit of measurement (Kg, Gram, Liter, Mililiter, Pcs)
- `category` — ingredient category (Sayuran, Daging, Bumbu, Minuman, Lainnya)
- `last_updated` — ledger timestamp of last update

---

## 🔗 Smart Contract Info

- **Network**: Stellar Testnet
- **Contract ID**: `CCJN5WK3DJ4Y4KZ34F7YVS6XO6NJRMAQGB7C5NFUCOYEZHO4ZWXGF4EU`
- **Explorer**: [View on Stellar Lab](https://lab.stellar.org/smart-contracts/contract-explorer?$=network$id=testnet&label=Testnet&horizonUrl=https:////horizon-testnet.stellar.org&rpcUrl=https:////soroban-testnet.stellar.org&&smartContracts$contractExplorer$contractId=CCJN5WK3DJ4Y4KZ34F7YVS6XO6NJRMAQGB7C5NFUCOYEZHO4ZWXGF4EU)

---

## 📸 Screenshots
<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/56db427f-d902-4d3b-82d4-9ba1a755c3d3" />


---

## 🛠️ Tech Stack

- **Blockchain**: Stellar
- **Smart Contract Framework**: Soroban SDK
- **Language**: Rust
- **IDE**: Soroban Studio
- **Wallet**: Freighter

---

## 🚀 How to Run

1. Clone this repository
2. Open in [Soroban Studio](https://soroban.studio)
3. Build the contract:
   ```
   stellar contract build
   ```
4. Deploy to testnet:
   ```
   stellar contract deploy --source-account YOUR_WALLET --network testnet
   ```
5. Invoke functions via [Stellar Lab](https://lab.stellar.org) or CLI

---

## 👤 Author

Built with ❤️ during the Stellar Workshop by Rise In.
