use printpdf::*;

use crate::modules::billing::domain::entities::invoice;

/// A single printable invoice line item.
/// (description, hsn_sac, taxable, total, tax_type, cgst, sgst, igst)
type LineItem = (
    String,
    String,
    rust_decimal::Decimal,
    rust_decimal::Decimal,
    String,
    rust_decimal::Decimal,
    rust_decimal::Decimal,
    rust_decimal::Decimal,
);

/// Generate a GST-compliant tax invoice PDF.
/// Returns the raw PDF bytes ready for storage or download.
pub fn generate_invoice_pdf(
    inv: &invoice::Model,
    company_name: &str,
    company_address: &str,
    company_gstin: &str,
    customer_name: &str,
    customer_address: &str,
    customer_gstin: &Option<String>,
    line_items: &[LineItem],
) -> Result<Vec<u8>, String> {
    let (doc, page1, layer1) = PdfDocument::new(
        format!("Tax Invoice - {}", inv.invoice_number),
        Mm(210.0),
        Mm(297.0),
        "Layer 1",
    );

    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Use built-in Helvetica font
    let font = doc
        .add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| e.to_string())?;
    let font_bold = doc
        .add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| e.to_string())?;

    let mut y = 260.0;

    // Title
    current_layer.use_text("TAX INVOICE", 18.0, Mm(10.0), Mm(y), &font_bold);
    y -= 15.0;

    // Company info
    current_layer.use_text(company_name, 11.0, Mm(10.0), Mm(y), &font_bold);
    y -= 5.0;
    current_layer.use_text(company_address, 9.0, Mm(10.0), Mm(y), &font);
    y -= 5.0;
    current_layer.use_text(
        format!("GSTIN: {}", company_gstin),
        9.0,
        Mm(10.0),
        Mm(y),
        &font,
    );
    y -= 12.0;

    // Invoice details
    current_layer.use_text(
        format!("Invoice No: {}", inv.invoice_number),
        10.0,
        Mm(10.0),
        Mm(y),
        &font,
    );
    current_layer.use_text(
        format!("Date: {}", inv.created_at.format("%d-%m-%Y")),
        10.0,
        Mm(110.0),
        Mm(y),
        &font,
    );
    y -= 6.0;
    current_layer.use_text(
        format!("Due Date: {}", inv.due_date.format("%d-%m-%Y")),
        10.0,
        Mm(10.0),
        Mm(y),
        &font,
    );
    current_layer.use_text(
        format!(
            "Period: {} to {}",
            inv.billing_period_start.format("%d-%m-%Y"),
            inv.billing_period_end.format("%d-%m-%Y")
        ),
        10.0,
        Mm(110.0),
        Mm(y),
        &font,
    );
    y -= 12.0;

    // Bill To
    current_layer.use_text("Bill To:", 10.0, Mm(10.0), Mm(y), &font_bold);
    y -= 5.0;
    current_layer.use_text(customer_name, 9.0, Mm(10.0), Mm(y), &font);
    y -= 5.0;
    current_layer.use_text(customer_address, 9.0, Mm(10.0), Mm(y), &font);
    y -= 5.0;
    if let Some(gstin) = customer_gstin {
        current_layer.use_text(format!("GSTIN: {}", gstin), 9.0, Mm(10.0), Mm(y), &font);
    }
    y -= 10.0;

    // Table header
    let header_y = y;
    current_layer.use_text("SNo", 8.0, Mm(10.0), Mm(header_y), &font_bold);
    current_layer.use_text("Description", 8.0, Mm(25.0), Mm(header_y), &font_bold);
    current_layer.use_text("HSN/SAC", 8.0, Mm(110.0), Mm(header_y), &font_bold);
    current_layer.use_text("Taxable", 8.0, Mm(130.0), Mm(header_y), &font_bold);
    current_layer.use_text("CGST", 8.0, Mm(150.0), Mm(header_y), &font_bold);
    current_layer.use_text("SGST", 8.0, Mm(165.0), Mm(header_y), &font_bold);
    current_layer.use_text("IGST", 8.0, Mm(180.0), Mm(header_y), &font_bold);
    current_layer.use_text("Amount", 8.0, Mm(195.0), Mm(header_y), &font_bold);
    y -= 5.0;

    // Table rows
    for (i, (desc, hsn, taxable, total, _tax_type, cgst, sgst, igst)) in
        line_items.iter().enumerate()
    {
        if y < 40.0 {
            break;
        }
        y -= 5.0;
        let row_y = y;
        current_layer.use_text((i + 1).to_string(), 8.0, Mm(10.0), Mm(row_y), &font);
        current_layer.use_text(desc, 8.0, Mm(25.0), Mm(row_y), &font);
        current_layer.use_text(hsn, 8.0, Mm(110.0), Mm(row_y), &font);
        current_layer.use_text(format!("₹{}", taxable), 8.0, Mm(130.0), Mm(row_y), &font);
        current_layer.use_text(format!("₹{}", cgst), 8.0, Mm(150.0), Mm(row_y), &font);
        current_layer.use_text(format!("₹{}", sgst), 8.0, Mm(165.0), Mm(row_y), &font);
        current_layer.use_text(format!("₹{}", igst), 8.0, Mm(180.0), Mm(row_y), &font);
        current_layer.use_text(format!("₹{}", total), 8.0, Mm(195.0), Mm(row_y), &font);
    }

    y -= 10.0;

    // Totals
    current_layer.use_text(
        format!("Subtotal: ₹{}", inv.subtotal),
        9.0,
        Mm(130.0),
        Mm(y),
        &font,
    );
    y -= 5.0;
    current_layer.use_text(
        format!("Discount: ₹{}", inv.discount_amount),
        9.0,
        Mm(130.0),
        Mm(y),
        &font,
    );
    y -= 5.0;

    if inv.cgst_amount > rust_decimal::Decimal::ZERO
        || inv.sgst_amount > rust_decimal::Decimal::ZERO
    {
        current_layer.use_text(
            format!("CGST: ₹{}", inv.cgst_amount),
            9.0,
            Mm(130.0),
            Mm(y),
            &font,
        );
        y -= 5.0;
        current_layer.use_text(
            format!("SGST: ₹{}", inv.sgst_amount),
            9.0,
            Mm(130.0),
            Mm(y),
            &font,
        );
        y -= 5.0;
    }
    if inv.igst_amount > rust_decimal::Decimal::ZERO {
        current_layer.use_text(
            format!("IGST: ₹{}", inv.igst_amount),
            9.0,
            Mm(130.0),
            Mm(y),
            &font,
        );
        y -= 5.0;
    }

    if inv.late_fee_subtotal > rust_decimal::Decimal::ZERO {
        current_layer.use_text(
            format!("Late Fee: ₹{}", inv.late_fee_subtotal),
            9.0,
            Mm(130.0),
            Mm(y),
            &font,
        );
        y -= 5.0;
        current_layer.use_text(
            format!("Late Fee GST: ₹{}", inv.late_fee_gst),
            9.0,
            Mm(130.0),
            Mm(y),
            &font,
        );
        y -= 5.0;
    }

    y -= 3.0;
    current_layer.use_text(
        format!("Total Amount: ₹{}", inv.total_amount),
        12.0,
        Mm(130.0),
        Mm(y),
        &font_bold,
    );
    y -= 12.0;

    // Footer
    current_layer.use_text(
        format!("Place of Supply: {}", inv.place_of_supply_state),
        8.0,
        Mm(10.0),
        Mm(y),
        &font,
    );
    if inv.reverse_charge {
        current_layer.use_text("Reverse Charge: Yes", 8.0, Mm(10.0), Mm(y - 5.0), &font);
    }

    y -= 12.0;
    current_layer.use_text(
        "Terms: Payment due within 30 days. Late fee of 2% applies after grace period.",
        7.0,
        Mm(10.0),
        Mm(y),
        &font,
    );
    y -= 5.0;
    current_layer.use_text(
        "This is a computer-generated invoice. No signature required.",
        7.0,
        Mm(10.0),
        Mm(y),
        &font,
    );

    // Save to bytes
    let pdf_bytes = doc.save_to_bytes().map_err(|e| e.to_string())?;
    Ok(pdf_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_generate_invoice_pdf() {
        let inv = invoice::Model {
            id: 1,
            invoice_number: "INV-2026-001".to_string(),
            customer_id: 1,
            branch_id: 1,
            subscription_id: 1,
            billing_period_start: chrono::NaiveDate::from_ymd_opt(2026, 7, 1).unwrap(),
            billing_period_end: chrono::NaiveDate::from_ymd_opt(2026, 7, 31).unwrap(),
            subtotal: dec!(999.00),
            discount_amount: dec!(0),
            tax_amount: dec!(179.82),
            total_amount: dec!(1178.82),
            currency: "INR".to_string(),
            status: "pending".to_string(),
            due_date: chrono::NaiveDate::from_ymd_opt(2026, 8, 15).unwrap(),
            paid_at: None,
            payment_method: None,
            payment_reference: None,
            created_by: None,
            reviewed_by: None,
            reviewed_at: None,
            review_notes: None,
            approved_by: None,
            approved_at: None,
            notes: None,
            review_status: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            cgst_amount: dec!(89.91),
            sgst_amount: dec!(89.91),
            igst_amount: dec!(0),
            place_of_supply_state: "Maharashtra".to_string(),
            supplier_gstin: Some("27AABCT1234F1Z5".to_string()),
            reverse_charge: false,
            late_fee_subtotal: dec!(0),
            late_fee_gst: dec!(0),
        };

        let line_items = vec![(
            "Broadband 100 Mbps - Monthly".to_string(),
            "998421".to_string(),
            dec!(999.00),
            dec!(1178.82),
            "CGST_SGST".to_string(),
            dec!(89.91),
            dec!(89.91),
            dec!(0),
        )];

        let result = generate_invoice_pdf(
            &inv,
            "AeroXe Broadband Pvt Ltd",
            "123 Tech Park, Pune, Maharashtra 411001",
            "27AABCT1234F1Z5",
            "Rahul Sharma",
            "456 Residency Road, Mumbai, Maharashtra 400001",
            &Some("27DEFGH5678I1Z8".to_string()),
            &line_items,
        );

        assert!(result.is_ok());
        let pdf_bytes = result.unwrap();
        assert!(pdf_bytes.len() > 100);
        // PDF starts with %PDF
        assert_eq!(&pdf_bytes[..5], b"%PDF-");
    }
}
