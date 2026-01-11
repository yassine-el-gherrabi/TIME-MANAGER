import { test, expect } from '@playwright/test';

test.describe('Authentication', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
  });

  test('should display login page', async ({ page }) => {
    await expect(page).toHaveTitle(/Time Manager/);
    await expect(page.getByRole('heading', { name: /connexion/i })).toBeVisible();
  });

  test('should show email and password fields', async ({ page }) => {
    await expect(page.getByLabel(/email/i)).toBeVisible();
    await expect(page.getByLabel(/mot de passe/i)).toBeVisible();
    await expect(page.getByRole('button', { name: /se connecter/i })).toBeVisible();
  });

  test('should show validation errors for empty form submission', async ({ page }) => {
    await page.getByRole('button', { name: /se connecter/i }).click();
    // Form should show validation errors or prevent submission
    await expect(page.getByLabel(/email/i)).toBeFocused();
  });

  test('should show error for invalid credentials', async ({ page }) => {
    await page.getByLabel(/email/i).fill('invalid@example.com');
    await page.getByLabel(/mot de passe/i).fill('wrongpassword');
    await page.getByRole('button', { name: /se connecter/i }).click();

    // Should show error message (adjust selector based on actual implementation)
    await expect(page.getByText(/identifiants invalides|erreur|invalid/i)).toBeVisible({
      timeout: 10000,
    });
  });

  test('should have password reset link', async ({ page }) => {
    const resetLink = page.getByRole('link', { name: /mot de passe oublié/i });
    await expect(resetLink).toBeVisible();
  });

  test('should navigate to password reset page', async ({ page }) => {
    await page.getByRole('link', { name: /mot de passe oublié/i }).click();
    await expect(page).toHaveURL(/password-reset/);
  });
});

test.describe('Password Reset Request', () => {
  test('should display password reset request form', async ({ page }) => {
    await page.goto('/password-reset-request');
    await expect(page.getByLabel(/email/i)).toBeVisible();
    await expect(page.getByRole('button', { name: /envoyer|réinitialiser/i })).toBeVisible();
  });

  test('should show confirmation after submitting valid email', async ({ page }) => {
    await page.goto('/password-reset-request');
    await page.getByLabel(/email/i).fill('test@example.com');
    await page.getByRole('button', { name: /envoyer|réinitialiser/i }).click();

    // Should show confirmation or success message
    await expect(page.getByText(/email envoyé|vérifiez|succès/i)).toBeVisible({
      timeout: 10000,
    });
  });
});
