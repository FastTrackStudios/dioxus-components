import { test, expect } from '@playwright/test';

test('otp typing, backspace, and rejection', async ({ page }) => {
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

test('otp cursor does not drift past typed length', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=otp&', { timeout: 20 * 60 * 1000 });

  const input = page.getByRole('textbox', { name: 'One-time password' });
  await input.focus();
  await page.keyboard.type('12');
  // Three right-arrows from end should be no-ops — cursor is already at end.
  await page.keyboard.press('ArrowRight');
  await page.keyboard.press('ArrowRight');
  await page.keyboard.press('ArrowRight');
  // The next typed digit goes at position 2, not somewhere past it.
  await page.keyboard.type('3');
  await expect(page.locator('#otp-value')).toHaveText('123');
});

test('otp ArrowLeft inserts in the middle', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=otp&', { timeout: 20 * 60 * 1000 });

  const input = page.getByRole('textbox', { name: 'One-time password' });
  await input.focus();
  await page.keyboard.type('12');
  await page.keyboard.press('ArrowLeft');
  await page.keyboard.type('9');
  await expect(page.locator('#otp-value')).toHaveText('192');
});

test('otp paste fills all slots', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=otp&', { timeout: 20 * 60 * 1000 });

  const input = page.getByRole('textbox', { name: 'One-time password' });
  await input.focus();
  // insertText simulates a paste / IME / on-screen-keyboard input event.
  await page.keyboard.insertText('987654');
  await expect(page.locator('#otp-value')).toHaveText('987654');
});
