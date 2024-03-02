// Facade is a pattern that simplifies interface to a library.
// In layman terms, it wraps multiple routines into one simple
// routine.
//
// In this example, we have a checkout system that depends on
// ShoppingCart, PaymentGateway, DeliveryService, and InventoryManagement.
// Calling them one-by-one is painful and might pollute up
// your code. The simplest solution to fix this is to create
// a facade CheckoutFacade that provides a method check_out
// which seams the complexity of wiring those 4 dependencies together.
//
// FAQ:
// 1. Constructing the facade is complex!
// - You can combine facade with builder pattern, or if you're lazy, use
// a dependency injection tool.

#[allow(dead_code)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub price: u32,
}

#[allow(dead_code)]
pub struct User {
    pub id: u32,
    pub name: String,
}

#[allow(dead_code, non_camel_case_types)]
pub enum OrderStatus {
    WAITING_FOR_PAYMENT,
    IN_PROCESS,
    DELIVERED,
    COMPLETED,
}

#[allow(dead_code)]
pub struct Order {
    pub number: String,
    pub items: Vec<Item>,
    pub status: OrderStatus,

    pub payment_method: String,
}

pub struct ShoppingCart{}
#[allow(dead_code)]
impl ShoppingCart {
    pub fn complete_order(&self, order: Order) {
        println!("Delivered order {}", order.number);
    }
}

pub struct DeliveryService{}
#[allow(dead_code)]
impl DeliveryService {
    pub fn place_order_for_delivery(&self, _: Order) {
        println!("Delivering order...");
    }
}

pub struct PaymentGateway{}
#[allow(dead_code)]
impl PaymentGateway {
    pub fn pay_order(&self, order: Order, transaction_id: String) {
        println!("Processing payment of {} with {}", order.number, transaction_id);
    }
}

pub struct InventoryManagement{}
#[allow(dead_code)]
impl InventoryManagement {
    pub fn get_stock(&self, _: String) -> u32 {
        8
    }

    pub fn decrease_stock(&self, _: String, __: u32) {
        // check if possible, then reduce the stock
    }
}

// CheckoutFacade is the facade
pub struct CheckoutFacade {
    pub cart: ShoppingCart,
    pub payment_gateway: PaymentGateway,
    pub inventory: InventoryManagement,
    pub delivery: DeliveryService,
}
#[allow(dead_code)]
impl CheckoutFacade {
    pub fn checkout(&self, _: User, __: Order) -> Result<bool, String> {
        // check stock, decrement stock, process payment, change order status, queue it for delivery, change order status
        Ok(true)
    }
}

