use creditas_rust_interview::{configuration};

const APPLICATION_PROPERTIES:&str = "application";

#[derive(Debug)]
enum ProductType { PHYSICAL, BOOK, DIGITAL, MEMBERSHIP }

struct Address {
    address_line: String,
    postal_code: String,
    city: String,
    country: String,
    state: String,
}

impl Address {
    fn new(address_line: String, postal_code: String, city: String, country: String, state: String) -> Self {
        Address { address_line, postal_code, city, country, state }
    }
}

fn main() {
    println!("Bootstrap application");
    let config = configuration::get_config(APPLICATION_PROPERTIES).expect("Failed to read properties");
    println!("{}", config.language);
    println!("{}", config.country);
    // TODO: Set locale
    
    let shipping_address = Address::new(
        String::from("address"), 
        String::from("postalCode"),
        String::from("city"), 
        String::from("country"),
        String::from("active"),
    );
    
    println!("{}", shipping_address.address_line);
    println!("{}", shipping_address.postal_code);
    println!("{}", shipping_address.city);
    println!("{}", shipping_address.country);
    println!("{}", shipping_address.state);
    
    let billing_address = Address::new(
        String::from("address"), 
        String::from("postalCode"),
        String::from("city"), 
        String::from("country"),
        String::from("active"),
    );
    
    println!("{}", billing_address.address_line);
    println!("{}", billing_address.postal_code);
    println!("{}", billing_address.city);
    println!("{}", billing_address.country);
    println!("{}", billing_address.state);
    
    let physical_product = ProductType::PHYSICAL;
    println!("{:?}", physical_product);
    
    let book_product = ProductType::BOOK;
    println!("{:?}", book_product);
    
    let digital_product = ProductType::DIGITAL;
    println!("{:?}", digital_product);
    
    let membership_product = ProductType::MEMBERSHIP;
    println!("{:?}", membership_product);
}