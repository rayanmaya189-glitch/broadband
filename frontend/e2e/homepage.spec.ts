import { test, expect } from '@playwright/test';

test.describe('Homepage', () => {
  test('loads and displays main content', async ({ page }) => {
    await page.goto('/');

    // Page title should contain the brand
    await expect(page).toHaveTitle(/AeroXe/i);

    // Main heading or brand should be visible
    const heading = page.locator('h1, [data-testid="hero-heading"]').first();
    await expect(heading).toBeVisible();
  });

  test('navigation links are present', async ({ page }) => {
    await page.goto('/');

    // Check for main navigation links
    const nav = page.locator('nav').first();
    await expect(nav).toBeVisible();

    // Should have at least a few nav links
    const links = nav.locator('a');
    const count = await links.count();
    expect(count).toBeGreaterThanOrEqual(3);
  });

  test('plans section is visible', async ({ page }) => {
    await page.goto('/');

    // Plans section should be present
    const plansSection = page.locator('[class*="plan"], [id*="plan"], section').filter({ hasText: /plan|speed|mbps/i }).first();
    await expect(plansSection).toBeVisible();
  });

  test('footer is present', async ({ page }) => {
    await page.goto('/');

    // Footer should be present
    const footer = page.locator('footer').first();
    await expect(footer).toBeVisible();
  });

  test('page has no console errors', async ({ page }) => {
    const errors: string[] = [];
    page.on('console', (msg) => {
      if (msg.type() === 'error') {
        errors.push(msg.text());
      }
    });

    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Filter out known noise (e.g., favicon 404, third-party scripts)
    const realErrors = errors.filter(
      (e) => !e.includes('favicon') && !e.includes('analytics') && !e.includes('404')
    );

    expect(realErrors).toHaveLength(0);
  });
});

test.describe('Navigation', () => {
  test('can navigate to plans page', async ({ page }) => {
    await page.goto('/');

    // Click on Plans link if it exists
    const plansLink = page.locator('a').filter({ hasText: /plans?/i }).first();
    if (await plansLink.isVisible()) {
      await plansLink.click();
      await page.waitForURL(/.*plan/);
      expect(page.url()).toContain('plan');
    }
  });

  test('404 page shows for unknown routes', async ({ page }) => {
    await page.goto('/nonexistent-page-12345');

    // Should show 404 or NotFound page
    const content = page.locator('body');
    await expect(content).toContainText(/404|not found|page not found/i);
  });
});
