use std::{collections::HashMap, fmt::Display, iter::Sum};

use lazy_static::lazy_static;

#[derive(Debug)]
struct Basket<'a> {
    products: HashMap<&'a Product, u32>,
    deals: Vec<&'a Deal>,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Product {
    name: String,
    price: Currency,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Currency(u32);

#[derive(Debug)]
struct Deal {
    product: String,
    kind: DealKind,
}

#[derive(Debug)]
enum DealKind {
    Buy1Get1Free,
    PercentageDiscount(u32),
}

impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.0 / 100))?;
        f.write_str(".")?;
        f.write_str(&format!("{}", self.0 % 100))
    }
}

impl Sum for Currency {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Currency(iter.map(|p| p.0).sum())
    }
}

impl Product {
    pub fn new(name: String, price: u32) -> Self {
        Self {
            name,
            price: Currency(price),
        }
    }
}

impl<'a> Basket<'a> {
    pub fn new() -> Self {
        Basket {
            products: HashMap::new(),
            deals: Vec::new(),
        }
    }

    pub fn scan(&mut self, product_name: &str) -> Result<(), ()> {
        let product = INVENTORY.get(product_name).ok_or(())?;

        self.products
            .entry(product)
            .and_modify(|quantity| *quantity += 1)
            .or_insert(1);

        Ok(())
    }

    pub fn add_deal(&mut self, deal: &'a Deal) {
        self.deals.push(deal);
    }

    pub fn total(&self) -> Currency {
        let total = self
            .products
            .iter()
            .map(|(product, quantity)| {
                for deal in &self.deals {
                    if deal.product == product.name {
                        return match deal.kind {
                            DealKind::Buy1Get1Free => {
                                Currency(quantity.div_ceil(2) * product.price.0)
                            }
                            DealKind::PercentageDiscount(percentage) => {
                                Currency(quantity * product.price.0 * (100 - percentage) / 100)
                            }
                        };
                    }
                }

                Currency(quantity * product.price.0)
            })
            .sum();

        total
    }
}

lazy_static! {
    static ref INVENTORY: HashMap<String, Product> = {
        vec![
            Product::new("A0001".to_string(), 1299),
            Product::new("A0002".to_string(), 399),
        ]
        .into_iter()
        .map(|p| (p.name.clone(), p))
        .collect::<HashMap<String, Product>>()
    };
    static ref DEAL1: Deal = {
        Deal {
            product: "A0002".to_string(),
            kind: DealKind::Buy1Get1Free,
        }
    };
    static ref DEAL2: Deal = {
        Deal {
            product: "A0001".to_string(),
            kind: DealKind::PercentageDiscount(10),
        }
    };
}

fn main() {
    let mut basket1 = Basket::new();

    let _ = basket1.scan("A0002");
    let _ = basket1.scan("A0001");
    let _ = basket1.scan("A0002");

    basket1.add_deal(&DEAL1);

    println!("Buy1Get1Free Total: {}", &basket1.total());

    let mut basket2 = Basket::new();

    let _ = basket2.scan("A0002");
    let _ = basket2.scan("A0001");
    let _ = basket2.scan("A0002");

    basket2.add_deal(&DEAL2);

    println!("10Percent Total: {}", &basket2.total());
}

#[cfg(test)]
mod tests {
    use crate::{Basket, Currency, DEAL1, DEAL2};

    #[test]
    fn test_total_without_products() {
        let basket = Basket::new();

        assert_eq!(Currency(0), basket.total());
    }

    #[test]
    fn test_total_with_products() {
        let mut basket = Basket::new();

        let _ = basket.scan("A0001");
        let _ = basket.scan("A0002");

        assert_eq!(Currency(1698), basket.total());
    }

    #[test]
    fn test_deal1() {
        let mut basket = Basket::new();

        let _ = basket.scan("A0002");
        let _ = basket.scan("A0001");
        let _ = basket.scan("A0002");

        basket.add_deal(&DEAL1);

        assert_eq!(Currency(1698), basket.total());
    }

    #[test]
    fn test_deal2() {
        let mut basket = Basket::new();

        let _ = basket.scan("A0002");
        let _ = basket.scan("A0001");
        let _ = basket.scan("A0002");

        basket.add_deal(&DEAL2);

        assert_eq!(Currency(1967), basket.total());
    }
}
