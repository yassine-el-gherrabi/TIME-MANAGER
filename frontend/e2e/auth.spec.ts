import { test, expect } from '@playwright/test';

test.describe('Authentication', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
  });

  test('should display login page', async ({ page }) => {
    await expect(page).toHaveTitle(/Time Manager/);
    await expect(page.getByRole('heading', { name: /login/i })).toBeVisible();
  });

  test('should show email and password fields', async ({ page }) => {
    await expect(page.getByLabel(/email/i)).toBeVisible();
    await expect(page.getByLabel(/password/i)).toBeVisible();
    await expect(page.getByRole('button', { name: /^login$/i })).toBeVisible();
  });

  test('should show validation errors for empty form submission', async ({ page }) => {
    await page.getByRole('button', { name: /^login$/i }).click();
    // Form should show validation errors or remain on login page
    await expect(page).toHaveURL(/login/);
    // Email field should still be visible (form not submitted)
    await expect(page.getByLabel(/email/i)).toBeVisible();
  });

  test('should show error for invalid credentials', async ({ page }) => {
    await page.getByLabel(/email/i).fill('invalid@example.com');
    await page.getByLabel(/password/i).fill('wrongpassword');
    await page.getByRole('button', { name: /^login$/i }).click();

    // Should show error message (adjust selector based on actual implementation)
    await expect(page.getByText(/invalid|error|incorrect/i)).toBeVisible({
      timeout: 10000,
    });
  });

  test('should have password reset link', async ({ page }) => {
    const resetLink = page.getByRole('link', { name: /forgot password/i });
    await expect(resetLink).toBeVisible();
  });

  test('should navigate to password reset page', async ({ page }) => {
    await page.getByRole('link', { name: /forgot password/i }).click();
    await expect(page).toHaveURL(/password-reset/);
  });
});

test.describe('Password Reset Request', () => {
  test('should display password reset request form', async ({ page }) => {
    await page.goto('/password-reset-request');
    await expect(page.getByLabel(/email/i)).toBeVisible();
    await expect(page.getByRole('button', { name: /send|reset/i })).toBeVisible();
  });

  test('should allow entering email and clicking submit', async ({ page }) => {
    await page.goto('/password-reset-request');
    const emailInput = page.getByLabel(/email/i);
    const submitButton = page.getByRole('button', { name: /send|reset/i });

    // Fill in email
    await emailInput.fill('test@example.com');
    await expect(emailInput).toHaveValue('test@example.com');

    // Button should be clickable
    await expect(submitButton).toBeEnabled();
  });
});
