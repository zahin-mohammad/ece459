use crate::utils::invoice::{
    create_invoice, pay_invoice, publish_updated_invoice, recreate_invoice_with_vat,
};
use crate::utils::types::Invoice;
use crate::utils::{
    check_for_fraud, check_vat_number, get_vat_number_for_company, upload_fraud_result,
};
use rand::{thread_rng, Rng};
use std::thread;
mod utils;

const FRAUD_CHECK_MINIMUM: f32 = 1000.00;

fn main() {
    for i in generate_invoices() {
        process_invoice(i)
    }
}

fn process_invoice(invoice: Invoice) {
    let inv = invoice.clone();
    // Speculative Execution for Threads
    let t = thread::spawn(move || recreate_invoice_with_vat(inv));

    let company_id = invoice.customer_id;
    let vat_number = get_vat_number_for_company(company_id);

    let vat_ok = if vat_number.is_ok() {
        println!("Checking VAT number for company {}", company_id);
        check_vat_number(vat_number.unwrap())
    } else {
        println!("No VAT number found for company {}", company_id);
        false
    };
    let final_invoice = if vat_ok {
        invoice
    } else {
        println!("Recreating invoice for company {} with VAT.", company_id);
        t.join().unwrap()
    };
    println!(
        "Paying invoice for company {} with amount {:.2}",
        company_id, final_invoice.total
    );
    let t = thread::spawn(move || check_for_fraud(company_id));
    let updated_invoice = pay_invoice(final_invoice);
    // Only do fraud analysis if the invoice is successfully paid and above the threshold
    if updated_invoice.balance == 0.0 && updated_invoice.subtotal > FRAUD_CHECK_MINIMUM {
        let fraud_result = t.join().unwrap();
        if fraud_result.fraud_suspected {
            println!("Fraud suspected on company {}.", company_id);
            upload_fraud_result(fraud_result);
        }
    }
    publish_updated_invoice(updated_invoice);
    println!("Invoice processing for company {} complete.", company_id);
}

fn generate_invoices() -> Vec<Invoice> {
    let mut v = Vec::new();
    for i in 0..100 {
        let customer_id = thread_rng().gen_range(1000000u64..99999999u64);
        let inv = create_invoice(customer_id, i);
        v.push(inv);
    }
    v
}
