use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    id: i32,
    name: String,
    calories: i32,
    protein: f32,
    fat: f32,
    carbohydrates: f32,
}

pub fn insert_product(product: Product) -> Result<(), rusqlite::Error> {
    let connection = Connection::open(DB_NAME)?;

    connection.execute(
        "insert into products (name, calories, protein, fat, carbohydrates) values (?, ?, ?, ?, ?)",
        [product.name, product.calories, product.protein, product.fat, product.carbohydrates],
    )?;

    Ok(())
}