use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use creditas_rust_interview::{configuration};

const APPLICATION_PROPERTIES:&str = "application";

#[derive(Eq, Hash, PartialEq, Debug)]
enum ProductType { PHYSICAL, BOOK, DIGITAL, MEMBERSHIP }

#[derive(Clone, Debug)]
struct Address {
    address_line: String,
    postal_code: String,
    city: String,
    country: String,
    state: String,
}

impl Address {
    fn new(address_line: String, postal_code: String, city: String, country: String, state: String) -> Self {
        Self { address_line, postal_code, city, country, state }
    }
}

#[derive(Debug)]
struct Customer {
    first_name: String,
    last_name: String,
    email: String,
    shipping_address: Address,
    billing_address: Address,
}

impl Customer {
    fn new(first_name: String, last_name: String, email: String, shipping_address: Address, billing_address: Address) -> Self {
        Self { first_name, last_name, email, shipping_address, billing_address }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Product {
    code: Uuid,
    name: String,
    description: String,
    type_: ProductType,
    price: i64,
    active: bool,
}

impl Product {
    fn new(code: Uuid, name: String, description: String, type_: ProductType, price: i64, active: bool) -> Self {
        Self { code, name, description, type_, price, active }
    }
}

#[derive(Debug)]
struct PurchaseOrder {
    code: Uuid,
    items: HashMap<Product, i32>,
    amount: i64,
    customer: Customer,
    closed_at: Option<DateTime<Utc>>,
}

impl PurchaseOrder {
    fn default(code: Uuid, customer: Customer) -> Self {
        Self { code, items: HashMap::new(), amount: 0, customer, closed_at: None }
    }
}

#[derive(Debug)]
struct CreditCard {
    issuer: String,
    name: String,
    network: String,
    holderName: String,
    personalAccountNumber: String,
    expirationDate: DateTime<Utc>,
    cvv: String,
}

impl CreditCard {
    fn new(issuer: String, name: String, network: String, holderName: String, personalAccountNumber: String, expirationDate: DateTime<Utc>, cvv: String) -> Self {
        Self { issuer, name, network, holderName, personalAccountNumber, expirationDate, cvv }
    }
}

#[derive(Debug)]
struct Payment {
    amount: i64,
    paid_at: DateTime<Utc>,
    error: String,
    currency: String,
}

trait PaymentMethod {
    fn pay(&self, amount: i64) -> Payment;
}

#[derive(Debug)]
struct CreditCardMethod {
    card: CreditCard
}

impl CreditCardMethod {
    fn new(card: CreditCard) -> Self {
        Self { card }
    }
}

impl PaymentMethod for CreditCardMethod {
    fn pay(&self, amount: i64) -> Payment {
        return Payment { amount: 0, paid_at: Utc::now(), error: String::from(""), currency: String::from("") };
    }
}

#[derive(Debug)]
struct Invoice {
    payment: Payment,
    purchase_order: PurchaseOrder,
    billing_address: Address,
    shipping_address: Address,
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
    
    println!("{:?}", shipping_address);
    
    let billing_address = Address::new(
        String::from("address"), 
        String::from("postalCode"),
        String::from("city"), 
        String::from("country"),
        String::from("active"),
    );
    
    println!("{:?}", billing_address);
    
    let physical_product = ProductType::PHYSICAL;
    println!("{:?}", physical_product);
    
    let book_product = ProductType::BOOK;
    println!("{:?}", book_product);
    
    let digital_product = ProductType::DIGITAL;
    println!("{:?}", digital_product);
    
    let membership_product = ProductType::MEMBERSHIP;
    println!("{:?}", membership_product);
    
    let customer = Customer::new(
        String::from("Alejandro"),
        String::from("Sánchez"),
        String::from("xxxxxx@xxxxxx.xxx"),
        shipping_address.clone(),
        billing_address.clone(),
    );
    
    println!("{:?}", customer);
    
    let flowered_shirt = Product::new(
        Uuid::new_v4(),
        String::from("Flowered t-shirt"),
        String::from("description"),
        ProductType::PHYSICAL,
        35,
        true,
    );
    
    println!("{:?}", flowered_shirt);
    
    let netflix_family_plan = Product::new(
        Uuid::new_v4(),
        String::from("Familiar plan"),
        String::from("description"),
        ProductType::MEMBERSHIP,
        29,
        true,
    );
    
    println!("{:?}", netflix_family_plan);
    
    let guide_galaxy = Product::new(
        Uuid::new_v4(),
        String::from("The Hitchhiker's Guide to the Galaxy"),
        String::from("description"),
        ProductType::BOOK,
        120,
        true,
    );
    
    println!("{:?}", guide_galaxy);
    
    let stairway_to_heaven = Product::new(
        Uuid::new_v4(),
        String::from("Stairway to Heaven"),
        String::from("description"),
        ProductType::DIGITAL,
        5,
        true,
    );
    
    println!("{:?}", stairway_to_heaven);
    
    let mut purchase_order = PurchaseOrder::default(
        Uuid::new_v4(),
        customer,
    );
    
    println!("{:?}", purchase_order);
    
    purchase_order.items.insert(flowered_shirt, 2);
    purchase_order.items.insert(netflix_family_plan, 1);
    purchase_order.items.insert(guide_galaxy, 1);
    purchase_order.items.insert(stairway_to_heaven, 1);
    
    let card = CreditCard::new(
        String::from("issuer"), String::from("name"), String::from("network"), String::from("holderName"), 
        String::from("1111222233334444"), DateTime::<Utc>::MAX_UTC, String::from("123")
    );
    
    println!("{:?}", card);
    
    let payment_method = CreditCardMethod::new(card);
    println!("{:?}", payment_method);
    
    let payment = payment_method.pay(100);
    println!("{:?}", payment);
    
    let invoice = Invoice { payment, purchase_order, 
        billing_address: billing_address.clone(), 
        shipping_address: shipping_address.clone(),
    };
    println!("{:?}", invoice);
}