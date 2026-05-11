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

test('otp on_complete fires only when the value reaches maxlength', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=otp&', { timeout: 20 * 60 * 1000 });

  const input = page.getByRole('textbox', { name: 'One-time password' });
  const complete = page.locator('#otp-complete');

  await input.focus();
  await page.keyboard.type('12345');
  await expect(page.locator('#otp-value')).toHaveText('12345');
  await expect(complete).toHaveText('');

  await page.keyboard.type('6');
  await expect(complete).toHaveText('123456');

  // Editing back below maxlength should not re-fire complete; the last value sticks.
  await page.keyboard.press('Backspace');
  await expect(page.locator('#otp-value')).toHaveText('12345');
  await expect(complete).toHaveText('123456');
});

test('otp on_complete does not re-fire when editing a full buffer', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=otp&', { timeout: 20 * 60 * 1000 });

  const input = page.getByRole('textbox', { name: 'One-time password' });
  const complete = page.locator('#otp-complete');

  await input.focus();
  await page.keyboard.type('123456');
  await expect(complete).toHaveText('123456');

  // Move into the middle of the full buffer and type. The keydown handler inserts
  // and truncates, keeping length at maxlength but changing the value. This is
  // NOT a transition to maxlength, so on_complete must not fire again.
  await page.keyboard.press('Home');
  await page.keyboard.press('9');
  await expect(page.locator('#otp-value')).toHaveText('912345');
  await expect(complete).toHaveText('123456');
});

test('otp disabled state blocks input', async ({ page }) => {
  await page.goto('http://127.0.0.1:8080/component/?name=otp&', { timeout: 20 * 60 * 1000 });

  const input = page.getByRole('textbox', { name: 'One-time password' });
  await page.locator('#otp-toggle-disabled').click();
  await expect(input).toBeDisabled();

  // Typing into a disabled input is a no-op in the browser.
  await input.focus({ timeout: 1000 }).catch(() => { /* focus may be refused while disabled */ });
  await page.keyboard.type('123');
  await expect(page.locator('#otp-value')).toHaveText('');

  // Re-enable and confirm input works again.
  await page.locator('#otp-toggle-disabled').click();
  await expect(input).toBeEnabled();
  await input.focus();
  await page.keyboard.type('123');
  await expect(page.locator('#otp-value')).toHaveText('123');
});
