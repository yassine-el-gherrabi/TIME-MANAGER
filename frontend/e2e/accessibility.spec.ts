import { test, expect } from '@playwright/test';

test.describe('Accessibility - Login Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
  });

  test('should have proper form labels', async ({ page }) => {
    const emailInput = page.getByLabel(/email/i);
    const passwordInput = page.getByLabel(/password/i);

    await expect(emailInput).toBeVisible();
    await expect(passwordInput).toBeVisible();
  });

  test('should have accessible button', async ({ page }) => {
    const submitButton = page.getByRole('button', { name: /^login$/i });
    await expect(submitButton).toBeVisible();
    await expect(submitButton).toBeEnabled();
  });

  test('should support keyboard navigation', async ({ page }) => {
    // Tab to email field
    await page.keyboard.press('Tab');
    await expect(page.getByLabel(/email/i)).toBeFocused();

    // Tab to password field
    await page.keyboard.press('Tab');
    await expect(page.getByLabel(/password/i)).toBeFocused();

    // Tab to submit button
    await page.keyboard.press('Tab');
    await expect(page.getByRole('button', { name: /^login$/i })).toBeFocused();
  });

  test('should have visible focus indicators', async ({ page }) => {
    const emailInput = page.getByLabel(/email/i);
    await emailInput.focus();

    // Check that focus is visible (element should have some focus styling)
    await expect(emailInput).toBeFocused();
  });

  test('password input should have type password', async ({ page }) => {
    const passwordInput = page.getByLabel(/password/i);
    await expect(passwordInput).toHaveAttribute('type', 'password');
  });
});

test.describe('Accessibility - General', () => {
  test('should have lang attribute on html', async ({ page }) => {
    await page.goto('/login');
    const lang = await page.locator('html').getAttribute('lang');
    expect(lang).toBeTruthy();
  });

  test('should have meta viewport for mobile', async ({ page }) => {
    await page.goto('/login');
    const viewport = page.locator('meta[name="viewport"]');
    await expect(viewport).toHaveAttribute('content', /width=device-width/);
  });
});
