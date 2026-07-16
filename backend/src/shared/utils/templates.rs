/// Handlebars template engine for multi-channel notifications.
///
/// Per §23 Notifications doc: supports Email, SMS, WhatsApp, Push, In-App
/// with Handlebars templates for variable substitution.

use handlebars::Handlebars;
use serde_json::Value;
use tracing::debug;

use crate::shared::errors::AppError;

/// Template engine wrapper.
pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
}

impl TemplateEngine {
    /// Create a new template engine with built-in templates.
    pub fn new() -> Self {
        let mut hb = Handlebars::new();
        hb.set_strict_mode(true);

        // ── Built-in templates ──

        // Invoice email template
        let _ = hb.register_template_string("invoice_email",
            "Subject: Invoice {{invoice_number}} from AeroXe Broadband\n\n\
             Dear {{customer_name}},\n\n\
             Your invoice {{invoice_number}} for ₹{{total_amount}} is due on {{due_date}}.\n\n\
             Please make the payment to avoid service interruption.\n\n\
             Payment Link: {{payment_url}}\n\n\
             Thank you,\nAeroXe Broadband Team");

        // Payment reminder
        let _ = hb.register_template_string("payment_reminder",
            "Subject: Payment Reminder - Invoice {{invoice_number}}\n\n\
             Dear {{customer_name}},\n\n\
             This is a friendly reminder that your invoice {{invoice_number}} for \
             ₹{{total_amount}} was due on {{due_date}}. It is now {{days_overdue}} days overdue.\n\n\
             Please pay immediately to avoid service suspension.\n\n\
             Payment Link: {{payment_url}}\n\n\
             AeroXe Broadband Team");

        // Welcome email
        let _ = hb.register_template_string("welcome_email",
            "Subject: Welcome to AeroXe Broadband!\n\n\
             Dear {{customer_name}},\n\n\
             Welcome to AeroXe Broadband! Your account has been activated.\n\n\
             Plan: {{plan_name}}\n\
             PPPOE Username: {{pppoe_username}}\n\n\
             If you have any questions, please contact support.\n\n\
             Best regards,\nAeroXe Broadband Team");

        // OTP SMS (concise)
        let _ = hb.register_template_string("otp_sms",
            "Your AeroXe verification code is {{otp}}. Valid for 5 minutes. Do not share.");

        // Installation notification
        let _ = hb.register_template_string("installation_notify",
            "Subject: Installation Scheduled\n\n\
             Dear {{customer_name}},\n\n\
             Your broadband installation has been scheduled for:\n\
             Date: {{scheduled_date}}\n\
             Time: {{scheduled_time_slot}}\n\
             Technician: {{technician_name}}\n\n\
             Please ensure someone is available at the installation address.\n\n\
             AeroXe Broadband Team");

        // Ticket confirmation
        let _ = hb.register_template_string("ticket_confirm",
            "Subject: Support Ticket #{{ticket_number}} Created\n\n\
             Dear {{customer_name}},\n\n\
             Your support ticket has been created:\n\
             Ticket #: {{ticket_number}}\n\
             Subject: {{subject}}\n\
             Category: {{category}}\n\n\
             Our team will respond within the SLA timeframe.\n\n\
             AeroXe Broadband Team");

        // Referral reward notification
        let _ = hb.register_template_string("referral_reward",
            "Subject: Referral Reward!\n\n\
             Dear {{referrer_name}},\n\n\
             Congratulations! You've earned a referral reward of ₹{{reward_amount}}.\n\
             Reward Type: {{reward_type}}\n\n\
             Thank you for spreading the word about AeroXe Broadband!\n\n\
             AeroXe Broadband Team");

        Self { handlebars: hb }
    }

    /// Render a named template with variable data.
    pub fn render(&self, template_name: &str, data: &Value) -> Result<String, AppError> {
        self.handlebars
            .render(template_name, data)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Template render error: {}", e)))
    }

    /// Register a custom template at runtime.
    pub fn register_template(&mut self, name: &str, template_str: &str) -> Result<(), AppError> {
        self.handlebars
            .register_template_string(name, template_str)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Template registration error: {}", e)))?;
        debug!(template = name, "Registered custom notification template");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_render_invoice_email() {
        let engine = TemplateEngine::new();
        let data = json!({
            "invoice_number": "INV-202607-0001",
            "customer_name": "Rahul Sharma",
            "total_amount": "708.00",
            "due_date": "2026-07-25",
            "payment_url": "https://pay.aeroxe.com/inv123"
        });
        let rendered = engine.render("invoice_email", &data).unwrap();
        assert!(rendered.contains("INV-202607-0001"));
        assert!(rendered.contains("Rahul Sharma"));
        assert!(rendered.contains("708.00"));
    }

    #[test]
    fn test_render_otp_sms() {
        let engine = TemplateEngine::new();
        let data = json!({ "otp": "482916" });
        let rendered = engine.render("otp_sms", &data).unwrap();
        assert!(rendered.contains("482916"));
    }

    #[test]
    fn test_missing_template() {
        let engine = TemplateEngine::new();
        let data = json!({});
        let result = engine.render("nonexistent_template", &data);
        assert!(result.is_err());
    }
}
