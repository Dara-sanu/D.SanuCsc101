struct Laptop {
    brand: String,
    price: u32,
}
 impl Laptop {
    fn new(brand: &str, price: u32)-> Self {
        Laptop{
            brand: brand.to_string(),price,
        }
    }

    fn total_cost(&self, quantity: u32)-> u32 {
        self.price*quantity
    }
 }

fn main() {
    let hp = Laptop::new("HP", 650_000);
    let ibm = Laptop::new("IBM", 755_000);
    let toshiba = Laptop::new("Toshiba", 550_000);
    let dell = Laptop::new("Dell", 850_000);

    let quantity_per_brand = 3;

    let total_cost = hp.total_cost(quantity_per_brand) + ibm.total_cost(quantity_per_brand)+ toshiba.total_cost(quantity_per_brand)+ dell.total_cost(quantity_per_brand);

    println!("Total cost for 3 laptops of each brand: {}", total_cost );
}