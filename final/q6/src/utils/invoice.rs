use crate::utils::types::Invoice;
use std::thread::sleep;
use std::time::Duration;
use chrono::Utc;
use rand::{thread_rng, Rng};

pub fn create_invoice(customer_id: u64, invoice_id: u32) -> Invoice {
    let subtotal = thread_rng().gen_range(5f32 .. 5000f32);
    return Invoice {
        invoice_id: invoice_id,
        customer_id: customer_id,
        subtotal: subtotal,
        tax: 0.0,
        total: subtotal,
        currency: "EUR".to_string(),
        datetime: Utc::now(),
        balance: subtotal,
        comments: "Hello There!".to_string()
    }
}

pub fn recreate_invoice_with_vat(original: Invoice) -> Invoice {
    // Producing invoices is hard. Pretend it is.
    sleep(Duration::from_millis(100));
    let tax_amount = original.subtotal * 0.13;
    return Invoice {
        invoice_id: original.invoice_id,
        customer_id: original.customer_id,
        subtotal: original.subtotal,
        tax: tax_amount,
        total: original.subtotal + tax_amount,
        currency: original.currency,
        datetime: Utc::now(),
        balance: original.subtotal + tax_amount,
        comments: original.comments
    }
}

pub fn pay_invoice(to_pay: Invoice) -> Invoice {
    sleep(Duration::from_millis(75));
    return Invoice {
        invoice_id: to_pay.invoice_id,
        customer_id: to_pay.customer_id,
        subtotal: to_pay.subtotal,
        tax: to_pay.tax,
        total: to_pay.total,
        currency: to_pay.currency,
        datetime: Utc::now(),
        balance: 0.0,
        comments: to_pay.comments
    }
}

pub fn publish_updated_invoice(to_publish: Invoice) {
    println!("Publishing invoice with id {}", to_publish.invoice_id);
    sleep(Duration::from_millis(30));
    // This does not actually do anything
}