import { test, expect } from '@playwright/test';

// ── Plans Page ────────────────────────────────────────────────────────────────

test.describe('Plans Page', () => {
  test('loads and displays plans heading', async ({ page }) => {
    await page.goto('/plans');
    await expect(page).toHaveTitle(/plan|internet/i);

    // Should show plans heading
    const heading = page.locator('h1, h2, h3').filter({ hasText: /plan|speed|internet/i }).first();
    await expect(heading).toBeVisible();
  });

  test('displays plan cards or list', async ({ page }) => {
    await page.goto('/plans');

    // Wait for lazy-loaded content
    await page.waitForLoadState('networkidle');

    // Should have at least one plan card/section
    const planElements = page.locator('[class*="plan"], [class*="card"], [data-testid*="plan"]');
    const count = await planElements.count();
    expect(count).toBeGreaterThanOrEqual(1);
  });

  test('has pricing information visible', async ({ page }) => {
    await page.goto('/plans');
    await page.waitForLoadState('networkidle');

    // Should contain pricing text (₹ symbol or number)
    const body = page.locator('body');
    await expect(body).toContainText(/₹|\b\d+\/mo|\bper month|\bspeed|\bmbps/i);
  });
});

// ── Contact Page ──────────────────────────────────────────────────────────────

test.describe('Contact Page', () => {
  test('loads and displays contact form', async ({ page }) => {
    await page.goto('/contact');
    await expect(page).toHaveTitle(/contact/i);

    // Should have a form with input fields
    const nameInput = page.locator('input[placeholder*="Name"], input[placeholder*="name"]').first();
    await expect(nameInput).toBeVisible();
  });

  test('contact form has required fields', async ({ page }) => {
    await page.goto('/contact');

    // Name field
    const nameInput = page.locator('input[placeholder*="Name"], input[placeholder*="name"]').first();
    await expect(nameInput).toBeVisible();

    // Email field
    const emailInput = page.locator('input[type="email"], input[placeholder*="Email"], input[placeholder*="email"]').first();
    await expect(emailInput).toBeVisible();

    // Phone field
    const phoneInput = page.locator('input[type="tel"], input[placeholder*="Phone"], input[placeholder*="phone"]').first();
    await expect(phoneInput).toBeVisible();

    // Message textarea
    const messageInput = page.locator('textarea').first();
    await expect(messageInput).toBeVisible();
  });

  test('contact information is displayed', async ({ page }) => {
    await page.goto('/contact');
    await page.waitForLoadState('networkidle');

    // Should show contact details (phone, email, address)
    const body = page.locator('body');
    await expect(body).toContainText(/phone|email|address|contact/i);
  });

  test('form validation prevents empty submission', async ({ page }) => {
    await page.goto('/contact');

    // Try to submit empty form
    const submitButton = page.locator('button[type="submit"], button').filter({ hasText: /send|submit|message/i }).first();
    if (await submitButton.isVisible()) {
      await submitButton.click();

      // Form should not navigate away (still on /contact)
      expect(page.url()).toContain('/contact');
    }
  });
});

// ── Check Availability Page ───────────────────────────────────────────────────

test.describe('Check Availability Page', () => {
  test('loads and displays search input', async ({ page }) => {
    await page.goto('/check-availability');
    await expect(page).toHaveTitle(/availability|cover/i);

    const searchInput = page.locator('input[placeholder*="area"], input[placeholder*="location"], input[placeholder*="zip"], input[placeholder*="Enter"]').first();
    await expect(searchInput).toBeVisible();
  });

  test('coverage check button is visible', async ({ page }) => {
    await page.goto('/check-availability');

    const checkButton = page.locator('button').filter({ hasText: /check|coverage|search/i }).first();
    await expect(checkButton).toBeVisible();
  });

  test('displays coverage areas', async ({ page }) => {
    await page.goto('/check-availability');
    await page.waitForLoadState('networkidle');

    // Should show coverage areas section
    const body = page.locator('body');
    await expect(body).toContainText(/coverage|area|available|jalgaon/i);
  });
});

// ── Cross-Page Navigation ─────────────────────────────────────────────────────

test.describe('Full Navigation Flow', () => {
  test('can navigate through all main pages', async ({ page }) => {
    // Start on homepage
    await page.goto('/');
    await expect(page).toHaveTitle(/aeroxe/i);

    // Navigate to Plans
    await page.goto('/plans');
    await expect(page).toHaveTitle(/plan/i);

    // Navigate to Contact
    await page.goto('/contact');
    await expect(page).toHaveTitle(/contact/i);

    // Navigate to About
    await page.goto('/about');
    await expect(page).toHaveTitle(/about/i);

    // Navigate to Support
    await page.goto('/support');
    await expect(page).toHaveTitle(/support/i);

    // Navigate to Check Availability
    await page.goto('/check-availability');
    await expect(page).toHaveTitle(/availability|cover/i);
  });

  test('SPA routing works without full page reload', async ({ page }) => {
    await page.goto('/');

    // Track network requests
    const requests: string[] = [];
    page.on('request', (req) => {
      if (req.resourceType() === 'document' && req.url().includes('/')) {
        requests.push(req.url());
      }
    });

    // Navigate within SPA
    const plansLink = page.locator('a[href="/plans"], a').filter({ hasText: /plans?/i }).first();
    if (await plansLink.isVisible()) {
      await plansLink.click();
      await page.waitForURL(/.*plan/);

      // Should not have triggered a full page reload
      const reloadRequests = requests.filter((r) => !r.includes('/api'));
      expect(reloadRequests.length).toBe(0);
    }
  });

  test('browser back/forward works', async ({ page }) => {
    await page.goto('/');
    await page.goto('/plans');
    await page.goto('/contact');

    // Go back twice
    await page.goBack();
    expect(page.url()).toContain('/plans');

    await page.goBack();
    expect(page.url()).toMatch(/\/?$/);

    // Go forward
    await page.goForward();
    expect(page.url()).toContain('/plans');
  });
});

// ── Legal Pages ───────────────────────────────────────────────────────────────

test.describe('Legal Pages', () => {
  test('privacy policy loads', async ({ page }) => {
    await page.goto('/privacy');
    await expect(page).toHaveTitle(/privacy/i);

    const body = page.locator('body');
    await expect(body).toContainText(/privacy|personal|data|policy/i);
  });

  test('terms of service loads', async ({ page }) => {
    await page.goto('/terms');
    await expect(page).toHaveTitle(/term/i);

    const body = page.locator('body');
    await expect(body).toContainText(/terms|service|agreement/i);
  });

  test('refund policy loads', async ({ page }) => {
    await page.goto('/refund');
    await expect(page).toHaveTitle(/refund/i);

    const body = page.locator('body');
    await expect(body).toContainText(/refund|return|cancellation/i);
  });
});

// ── Performance & Quality ─────────────────────────────────────────────────────

test.describe('Performance', () => {
  test('pages load within 5 seconds', async ({ page }) => {
    const pages = ['/', '/plans', '/contact', '/about', '/support'];

    for (const path of pages) {
      const start = Date.now();
      await page.goto(path);
      await page.waitForLoadState('domcontentloaded');
      const elapsed = Date.now() - start;

      expect(elapsed).toBeLessThan(5000);
    }
  });

  test('no unhandled JS errors on any page', async ({ page }) => {
    const errors: string[] = [];
    page.on('pageerror', (error) => {
      errors.push(error.message);
    });

    const pages = ['/', '/plans', '/contact', '/about', '/check-availability'];

    for (const path of pages) {
      await page.goto(path);
      await page.waitForLoadState('networkidle');
    }

    const realErrors = errors.filter(
      (e) => !e.includes('favicon') && !e.includes('ResizeObserver')
    );

    expect(realErrors).toHaveLength(0);
  });
});

// ── Responsive Design ─────────────────────────────────────────────────────────

test.describe('Responsive Design', () => {
  test('pages are usable on mobile viewport', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });

    const pages = ['/', '/plans', '/contact'];

    for (const path of pages) {
      await page.goto(path);
      await page.waitForLoadState('domcontentloaded');

      // Content should still be visible
      const body = page.locator('body');
      await expect(body).toBeVisible();

      // No horizontal overflow
      const scrollWidth = await page.evaluate(() => document.body.scrollWidth);
      const viewportWidth = await page.evaluate(() => window.innerWidth);
      expect(scrollWidth).toBeLessThanOrEqual(viewportWidth + 20); // Small tolerance
    }
  });

  test('navigation is accessible on mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/');

    // Should have a mobile menu toggle or visible nav
    const nav = page.locator('nav, [data-testid*="nav"], header').first();
    await expect(nav).toBeVisible();
  });
});
