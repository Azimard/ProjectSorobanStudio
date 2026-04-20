#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

// Helper: buat env dan client
fn setup() -> (Env, RestaurantInventoryClient<'static>) {
    let env = Env::default();
    let contract_id = env.register(RestaurantInventory, ());
    let client = RestaurantInventoryClient::new(&env, &contract_id);
    (env, client)
}

#[test]
fn test_add_and_get_all() {
    let (env, client) = setup();

    client.add_ingredient(
        &String::from_str(&env, "Ayam"),
        &50,
        &10,
        &Unit::Kg,
        &Category::Daging,
    );

    client.add_ingredient(
        &String::from_str(&env, "Bawang Merah"),
        &20,
        &5,
        &Unit::Kg,
        &Category::Bumbu,
    );

    let all = client.get_all();
    assert_eq!(all.len(), 2);
}

#[test]
fn test_restock() {
    let (env, client) = setup();

    let id = client.add_ingredient(
        &String::from_str(&env, "Beras"),
        &30,
        &10,
        &Unit::Kg,
        &Category::Lainnya,
    );

    client.restock(&id, &20);

    let item = client.get_by_id(&id).unwrap();
    assert_eq!(item.quantity, 50);
}

#[test]
fn test_use_stock() {
    let (env, client) = setup();

    let id = client.add_ingredient(
        &String::from_str(&env, "Minyak Goreng"),
        &10,
        &2,
        &Unit::Liter,
        &Category::Lainnya,
    );

    client.use_stock(&id, &4);

    let item = client.get_by_id(&id).unwrap();
    assert_eq!(item.quantity, 6);
}

#[test]
fn test_use_stock_insufficient() {
    let (env, client) = setup();

    let id = client.add_ingredient(
        &String::from_str(&env, "Garam"),
        &3,
        &1,
        &Unit::Kg,
        &Category::Bumbu,
    );

    let result = client.use_stock(&id, &10);
    assert_eq!(result, String::from_str(&env, "Stok tidak mencukupi"));
}

#[test]
fn test_low_stock_alert() {
    let (env, client) = setup();

    // Stok 5, minimum 10 → masuk low stock
    client.add_ingredient(
        &String::from_str(&env, "Tomat"),
        &5,
        &10,
        &Unit::Kg,
        &Category::Sayuran,
    );

    // Stok 100, minimum 10 → aman
    client.add_ingredient(
        &String::from_str(&env, "Air Mineral"),
        &100,
        &10,
        &Unit::Liter,
        &Category::Minuman,
    );

    let low = client.get_low_stock();
    assert_eq!(low.len(), 1);
    assert_eq!(low.get(0).unwrap().name, String::from_str(&env, "Tomat"));
}

#[test]
fn test_remove_ingredient() {
    let (env, client) = setup();

    let id = client.add_ingredient(
        &String::from_str(&env, "Telur"),
        &100,
        &20,
        &Unit::Pcs,
        &Category::Lainnya,
    );

    client.remove_ingredient(&id);

    let all = client.get_all();
    assert_eq!(all.len(), 0);
}

#[test]
fn test_update_ingredient() {
    let (env, client) = setup();

    let id = client.add_ingredient(
        &String::from_str(&env, "Wortel"),
        &15,
        &5,
        &Unit::Kg,
        &Category::Sayuran,
    );

    client.update_ingredient(
        &id,
        &String::from_str(&env, "Wortel Organik"),
        &8,
        &Unit::Kg,
        &Category::Sayuran,
    );

    let item = client.get_by_id(&id).unwrap();
    assert_eq!(item.name, String::from_str(&env, "Wortel Organik"));
    assert_eq!(item.min_quantity, 8);
}