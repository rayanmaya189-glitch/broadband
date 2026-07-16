-- AeroXe Backend Migration 017: Seed Initial Plans
-- Default ISP internet plans with pricing

-- Seed initial plans
INSERT INTO plans (slug, name, description, speed_label, download_mbps, upload_mbps, burst_mbps,
                   data_quota, qos_priority, sla_uptime_percent, is_popular, is_active, review_status, approved_at)
VALUES
('basic-50', 'Basic', 'Affordable internet for light usage', '50 Mbps', 50, 25, 75,
 'unlimited', 'standard', 99.0, FALSE, TRUE, 'approved', NOW()),
('standard-100', 'Standard', 'Perfect for streaming and browsing', '100 Mbps', 100, 50, 150,
 'unlimited', 'standard', 99.5, TRUE, TRUE, 'approved', NOW()),
('premium-150', 'Premium', 'High-speed for power users', '150 Mbps', 150, 75, 200,
 'unlimited', 'premium', 99.5, FALSE, TRUE, 'approved', NOW()),
('pro-200', 'Pro', 'Ultra-fast for gaming and 4K streaming', '200 Mbps', 200, 100, 250,
 'unlimited', 'premium', 99.9, FALSE, TRUE, 'approved', NOW()),
('ultimate-300', 'Ultimate', 'Maximum speed for enterprise use', '300 Mbps', 300, 150, 400,
 'unlimited', 'business', 99.9, FALSE, TRUE, 'approved', NOW()),
('business-500', 'Business', 'Enterprise-grade connectivity', '500 Mbps', 500, 250, 600,
 'unlimited', 'business', 99.99, FALSE, TRUE, 'approved', NOW())
ON CONFLICT (slug) DO NOTHING;

-- Seed plan pricing
-- Basic 50
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 1, 400.00, 0, TRUE FROM plans p WHERE p.slug = 'basic-50'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 3, 1150.00, 50.00, TRUE FROM plans p WHERE p.slug = 'basic-50'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 6, 2250.00, 150.00, TRUE FROM plans p WHERE p.slug = 'basic-50'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 12, 4300.00, 500.00, TRUE FROM plans p WHERE p.slug = 'basic-50'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;

-- Standard 100
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 1, 600.00, 0, TRUE FROM plans p WHERE p.slug = 'standard-100'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 3, 1700.00, 100.00, TRUE FROM plans p WHERE p.slug = 'standard-100'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 6, 3350.00, 250.00, TRUE FROM plans p WHERE p.slug = 'standard-100'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 12, 6400.00, 800.00, TRUE FROM plans p WHERE p.slug = 'standard-100'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;

-- Premium 150
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 1, 800.00, 0, TRUE FROM plans p WHERE p.slug = 'premium-150'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 3, 2300.00, 100.00, TRUE FROM plans p WHERE p.slug = 'premium-150'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 6, 4550.00, 250.00, TRUE FROM plans p WHERE p.slug = 'premium-150'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 12, 8700.00, 900.00, TRUE FROM plans p WHERE p.slug = 'premium-150'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;

-- Pro 200
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 1, 1000.00, 0, TRUE FROM plans p WHERE p.slug = 'pro-200'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 3, 2850.00, 150.00, TRUE FROM plans p WHERE p.slug = 'pro-200'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 6, 5650.00, 350.00, TRUE FROM plans p WHERE p.slug = 'pro-200'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 12, 10800.00, 1200.00, TRUE FROM plans p WHERE p.slug = 'pro-200'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;

-- Ultimate 300
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 1, 1300.00, 0, TRUE FROM plans p WHERE p.slug = 'ultimate-300'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 3, 3700.00, 200.00, TRUE FROM plans p WHERE p.slug = 'ultimate-300'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 6, 7350.00, 450.00, TRUE FROM plans p WHERE p.slug = 'ultimate-300'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 12, 14000.00, 1600.00, TRUE FROM plans p WHERE p.slug = 'ultimate-300'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;

-- Business 500
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 1, 2500.00, 0, TRUE FROM plans p WHERE p.slug = 'business-500'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 3, 7200.00, 300.00, TRUE FROM plans p WHERE p.slug = 'business-500'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 6, 14100.00, 900.00, TRUE FROM plans p WHERE p.slug = 'business-500'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;
INSERT INTO plan_pricing (plan_id, billing_period_months, price, savings, is_active)
SELECT p.id, 12, 27000.00, 3000.00, TRUE FROM plans p WHERE p.slug = 'business-500'
ON CONFLICT (plan_id, billing_period_months) DO NOTHING;

-- Seed speed profiles
INSERT INTO speed_profiles (plan_id, name, download_limit_kbps, upload_limit_kbps,
                           burst_download_kbps, burst_upload_kbps, burst_duration_seconds,
                           priority_queue, device_type)
SELECT p.id, 'basic-50-mikrotik', 50000, 25000, 75000, 37500, 30, 2, 'mikrotik'
FROM plans p WHERE p.slug = 'basic-50'
ON CONFLICT DO NOTHING;

INSERT INTO speed_profiles (plan_id, name, download_limit_kbps, upload_limit_kbps,
                           burst_download_kbps, burst_upload_kbps, burst_duration_seconds,
                           priority_queue, device_type)
SELECT p.id, 'standard-100-mikrotik', 100000, 50000, 150000, 75000, 30, 2, 'mikrotik'
FROM plans p WHERE p.slug = 'standard-100'
ON CONFLICT DO NOTHING;

INSERT INTO speed_profiles (plan_id, name, download_limit_kbps, upload_limit_kbps,
                           burst_download_kbps, burst_upload_kbps, burst_duration_seconds,
                           priority_queue, device_type)
SELECT p.id, 'premium-150-mikrotik', 150000, 75000, 200000, 100000, 30, 1, 'mikrotik'
FROM plans p WHERE p.slug = 'premium-150'
ON CONFLICT DO NOTHING;

INSERT INTO speed_profiles (plan_id, name, download_limit_kbps, upload_limit_kbps,
                           burst_download_kbps, burst_upload_kbps, burst_duration_seconds,
                           priority_queue, device_type)
SELECT p.id, 'pro-200-mikrotik', 200000, 100000, 250000, 125000, 30, 1, 'mikrotik'
FROM plans p WHERE p.slug = 'pro-200'
ON CONFLICT DO NOTHING;

INSERT INTO speed_profiles (plan_id, name, download_limit_kbps, upload_limit_kbps,
                           burst_download_kbps, burst_upload_kbps, burst_duration_seconds,
                           priority_queue, device_type)
SELECT p.id, 'ultimate-300-mikrotik', 300000, 150000, 400000, 200000, 30, 1, 'mikrotik'
FROM plans p WHERE p.slug = 'ultimate-300'
ON CONFLICT DO NOTHING;

INSERT INTO speed_profiles (plan_id, name, download_limit_kbps, upload_limit_kbps,
                           burst_download_kbps, burst_upload_kbps, burst_duration_seconds,
                           priority_queue, device_type)
SELECT p.id, 'business-500-mikrotik', 500000, 250000, 600000, 300000, 30, 1, 'mikrotik'
FROM plans p WHERE p.slug = 'business-500'
ON CONFLICT DO NOTHING;

-- Seed initial branches
INSERT INTO branches (name, slug, code, city, state, is_active)
VALUES
('Jalgaon Main', 'jalgaon-main', 'JLG', 'Jalgaon', 'Maharashtra', TRUE),
('Bhusawal', 'bhusawal', 'BHL', 'Bhusawal', 'Maharashtra', TRUE),
('Mumbai', 'mumbai', 'MUM', 'Mumbai', 'Maharashtra', TRUE),
('Navi Mumbai', 'navi-mumbai', 'NNM', 'Navi Mumbai', 'Maharashtra', TRUE)
ON CONFLICT (slug) DO NOTHING;

-- Seed default approval workflows
INSERT INTO approval_workflows (name, operation, required_approver_roles, timeout_hours, is_active)
VALUES
('OLT Firmware Update', 'firmware_update', '["network_admin", "isp_owner"]', 24, TRUE),
('Bulk Customer Suspension', 'bulk_suspension', '["finance_manager", "isp_owner"]', 48, TRUE),
('Network Config Change', 'network_config_change', '["network_admin", "noc_engineer"]', 12, TRUE),
('Large Refund', 'large_refund', '["billing_operator", "finance_manager"]', 72, TRUE),
('Device Removal', 'device_removal', '["noc_engineer", "network_admin"]', 24, TRUE),
('Plan Pricing Change', 'plan_pricing_change', '["finance_manager", "isp_owner"]', 48, TRUE)
ON CONFLICT (operation) DO NOTHING;

-- Seed notification channels
INSERT INTO notification_channels (channel, provider, config, is_active)
VALUES
('email', 'smtp', '{"host": "${SMTP_HOST}", "port": 587, "username": "${SMTP_USERNAME}", "from_email": "${SMTP_FROM_EMAIL}"}', TRUE),
('sms', 'msg91', '{"api_key": "${SMS_API_KEY}", "sender_id": "${SMS_SENDER_ID}"}', TRUE),
('whatsapp', 'business_api', '{"phone_number_id": "", "access_token": ""}', FALSE),
('push', 'fcm', '{"server_key": ""}', FALSE)
ON CONFLICT (channel) DO NOTHING;
