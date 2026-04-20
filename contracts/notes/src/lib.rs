#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Kategori bahan makanan
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum Category {
    Sayuran,
    Daging,
    Bumbu,
    Minuman,
    Lainnya,
}

// Satuan bahan makanan
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum Unit {
    Kg,
    Gram,
    Liter,
    Mililiter,
    Pcs,
}

// Struktur data bahan makanan
#[contracttype]
#[derive(Clone, Debug)]
pub struct Ingredient {
    pub id: u64,
    pub name: String,
    pub quantity: u64,       // jumlah stok saat ini
    pub min_quantity: u64,   // batas minimum sebelum low stock alert
    pub unit: Unit,
    pub category: Category,
    pub last_updated: u64,   // timestamp ledger
}

// Storage key
const STOCK_DATA: Symbol = symbol_short!("STOCK_DAT");

#[contract]
pub struct RestaurantInventory;

#[contractimpl]
impl RestaurantInventory {

    // ── READ ──────────────────────────────────────────────────────────────────

    /// Ambil semua bahan
    pub fn get_all(env: Env) -> Vec<Ingredient> {
        env.storage().instance().get(&STOCK_DATA).unwrap_or(Vec::new(&env))
    }

    /// Ambil bahan yang stoknya di bawah minimum (low stock alert)
    pub fn get_low_stock(env: Env) -> Vec<Ingredient> {
        let all: Vec<Ingredient> = env.storage().instance().get(&STOCK_DATA).unwrap_or(Vec::new(&env));
        let mut low: Vec<Ingredient> = Vec::new(&env);
        for i in 0..all.len() {
            let item = all.get(i).unwrap();
            if item.quantity <= item.min_quantity {
                low.push_back(item);
            }
        }
        low
    }

    /// Cari bahan berdasarkan ID
    pub fn get_by_id(env: Env, id: u64) -> Option<Ingredient> {
        let all: Vec<Ingredient> = env.storage().instance().get(&STOCK_DATA).unwrap_or(Vec::new(&env));
        for i in 0..all.len() {
            let item = all.get(i).unwrap();
            if item.id == id {
                return Some(item);
            }
        }
        None
    }

    // ── WRITE ─────────────────────────────────────────────────────────────────

    /// Tambahkan bahan baru ke inventori
    pub fn add_ingredient(
        env: Env,
        name: String,
        quantity: u64,
        min_quantity: u64,
        unit: Unit,
        category: Category,
    ) -> u64 {
        let mut all: Vec<Ingredient> = env.storage().instance().get(&STOCK_DATA).unwrap_or(Vec::new(&env));

        let id = env.prng().gen::<u64>();
        let ingredient = Ingredient {
            id,
            name,
            quantity,
            min_quantity,
            unit,
            category,
            last_updated: env.ledger().timestamp(),
        };

        all.push_back(ingredient);
        env.storage().instance().set(&STOCK_DATA, &all);

        id // kembalikan id agar mudah direferensikan
    }

    /// Tambah jumlah stok bahan (restock)
    pub fn restock(env: Env, id: u64, amount: u64) -> String {
        let mut all: Vec<Ingredient> = env.storage().instance().get(&STOCK_DATA).unwrap_or(Vec::new(&env));

        for i in 0..all.len() {
            let mut item = all.get(i).unwrap();
            if item.id == id {
                item.quantity += amount;
                item.last_updated = env.ledger().timestamp();
                all.set(i, item);
                env.storage().instance().set(&STOCK_DATA, &all);
                return String::from_str(&env, "Stok berhasil ditambahkan");
            }
        }

        String::from_str(&env, "Bahan tidak ditemukan")
    }

    /// Kurangi stok bahan (pemakaian dapur)
    pub fn use_stock(env: Env, id: u64, amount: u64) -> String {
        let mut all: Vec<Ingredient> = env.storage().instance().get(&STOCK_DATA).unwrap_or(Vec::new(&env));

        for i in 0..all.len() {
            let mut item = all.get(i).unwrap();
            if item.id == id {
                if item.quantity < amount {
                    return String::from_str(&env, "Stok tidak mencukupi");
                }
                item.quantity -= amount;
                item.last_updated = env.ledger().timestamp();
                all.set(i, item);
                env.storage().instance().set(&STOCK_DATA, &all);
                return String::from_str(&env, "Stok berhasil dikurangi");
            }
        }

        String::from_str(&env, "Bahan tidak ditemukan")
    }

    /// Update nama, minimum stok, unit, atau kategori sebuah bahan
    pub fn update_ingredient(
        env: Env,
        id: u64,
        name: String,
        min_quantity: u64,
        unit: Unit,
        category: Category,
    ) -> String {
        let mut all: Vec<Ingredient> = env.storage().instance().get(&STOCK_DATA).unwrap_or(Vec::new(&env));

        for i in 0..all.len() {
            let mut item = all.get(i).unwrap();
            if item.id == id {
                item.name = name;
                item.min_quantity = min_quantity;
                item.unit = unit;
                item.category = category;
                item.last_updated = env.ledger().timestamp();
                all.set(i, item);
                env.storage().instance().set(&STOCK_DATA, &all);
                return String::from_str(&env, "Bahan berhasil diupdate");
            }
        }

        String::from_str(&env, "Bahan tidak ditemukan")
    }

    /// Hapus bahan dari inventori
    pub fn remove_ingredient(env: Env, id: u64) -> String {
        let mut all: Vec<Ingredient> = env.storage().instance().get(&STOCK_DATA).unwrap_or(Vec::new(&env));

        for i in 0..all.len() {
            if all.get(i).unwrap().id == id {
                all.remove(i);
                env.storage().instance().set(&STOCK_DATA, &all);
                return String::from_str(&env, "Bahan berhasil dihapus");
            }
        }

        String::from_str(&env, "Bahan tidak ditemukan")
    }
}

mod test;