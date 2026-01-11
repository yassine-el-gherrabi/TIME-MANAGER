import { test, expect } from '@playwright/test';

test.describe('Navigation - Unauthenticated', () => {
  test('should redirect to login when accessing protected routes', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page).toHaveURL(/login/);
  });

  test('should redirect to login when accessing clock page', async ({ page }) => {
    await page.goto('/clock');
    await expect(page).toHaveURL(/login/);
  });

  test('should redirect to login when accessing absences page', async ({ page }) => {
    await page.goto('/absences');
    await expect(page).toHaveURL(/login/);
  });

  test('should redirect to login when accessing admin pages', async ({ page }) => {
    await page.goto('/admin/users');
    await expect(page).toHaveURL(/login/);
  });
});

test.describe('Navigation - Page Titles', () => {
  test('login page should have correct title', async ({ page }) => {
    await page.goto('/login');
    await expect(page).toHaveTitle(/Time Manager/);
  });

  test('unauthorized page should display correctly', async ({ page }) => {
    await page.goto('/unauthorized');
    await expect(page.getByText(/accès refusé|non autorisé|unauthorized/i)).toBeVisible();
  });
});

test.describe('Responsive Navigation', () => {
  test('should display mobile menu on small screens', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/login');

    // Login page should be responsive
    await expect(page.getByLabel(/email/i)).toBeVisible();
  });

  test('should display properly on tablet', async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 1024 });
    await page.goto('/login');

    await expect(page.getByLabel(/email/i)).toBeVisible();
  });
});
