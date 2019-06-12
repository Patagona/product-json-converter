#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::Path;
use std::error::Error;
use csv::Writer;
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Product {
    gtin: i64,
    product_id: Option<String>,
    description: String,
    min_price_boundary: f32,
    max_price_boundary: f32,
    reference_price: f32,
    tags: Vec<RawTag>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawTag {
    label: String,
    value: String
}

fn main() -> Result<(), Box<dyn Error>>{

    let products: Vec<Product> = serde_json::from_reader(std::io::stdin())?;

    let tag_headers: HashSet<String> = products.iter().flat_map(|product| {
        product.tags.iter().map(|tag| { tag.label.clone()})
    }).collect();

    let mut writer = Writer::from_writer(std::io::stdout());

    let mut headers = vec!["gtin".to_string(), "productId".to_string(), "description".to_string(), "minPriceBoundary".to_string(), "maxPriceBoundary".to_string(), "referencePrice".to_string()];

    headers.extend(tag_headers.iter().cloned());

    writer.write_record(headers);

    for product in products {
        writer.write_record(product_2_row(&product, &tag_headers));
    }

    Ok(())
}

fn product_2_row(product: &Product, tag_headers: &HashSet<String>) -> Vec<String> {
    let mut product_vector = vec![
        format!("{}", product.gtin),
        product.product_id.clone().unwrap_or(String::new()).clone(),
        product.description.clone(),
        format!("{}", product.min_price_boundary),
        format!("{}", product.max_price_boundary),
        format!("{}", product.reference_price),
    ];

    tag_headers.iter().for_each(|label| {
       product_vector.push(product.tags.iter().find(|tag| {
           tag.label == *label
       }).map(|t| t.value.clone()).unwrap_or(String::new()));
    });

    product_vector
}
