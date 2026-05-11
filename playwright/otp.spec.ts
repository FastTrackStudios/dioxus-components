import { test, expect } from '@playwright/test';

test('otp', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=otp&', { timeout: 20 * 60 * 1000 });

  const input = page.getByRole('textbox', { name: 'One-time password' });
  await expect(input).toBeVisible();

  await input.focus();
  await page.keyboard.type('123456');
  await expect(page.locator('#otp-value')).toHaveText('123456');

  await page.keyboard.press('Backspace');
  await expect(page.locator('#otp-value')).toHaveText('12345');

  await page.keyboard.press('a');
  await expect(page.locator('#otp-value')).toHaveText('12345');
});
