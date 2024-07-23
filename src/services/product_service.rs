use crate::models::product_model::Product;

pub struct ProductService;

impl ProductService {
    pub fn new() -> Self {
        ProductService
    }

    pub fn get_all_products(&self) -> Vec<Product> {
        vec![Product { id: 1, name: "Product A".to_string(), price: 10.0 }]
    }

    pub fn create_product(&self, product: Product) -> Product {
        product
    }

    pub fn update_product(&self, id: u32, product: Product) -> Product {
        Product { id, ..product }
    }

    pub fn delete_product(&self, id: u32) {
        println!("Product with id {} deleted", id);
    }
}
