// Reproductions for the four issues flagged in the PR #265 review.
// Each test asserts the buggy state — passing means the bug is present.

import { test, expect } from '@playwright/test';

const URL = 'http://127.0.0.1:8080/component/?name=context_menu&';

test('ISSUE 1: trigger sets touch-action: none, blocking page scroll on the trigger', async ({ page }) => {
  await page.goto(URL, { timeout: 20 * 60 * 1000 });
  const trigger = page.getByRole('button', { name: 'right click here' });
  const touchAction = await trigger.evaluate((el) => getComputedStyle(el).touchAction);
  // touch-action: none disables browser-driven panning that starts on this element.
  // For large triggers (cards/rows), the user can't scroll the page if their finger
  // first lands on the trigger. `pan-y` would keep long-press working while allowing
  // vertical pan, and the move-tolerance code already cancels mid-scroll.
  expect(touchAction).toBe('none');
});

test('ISSUE 2: backdrop is interactive but has no aria-hidden / role semantics', async ({ page }) => {
  await page.goto(URL, { timeout: 20 * 60 * 1000 });
  const trigger = page.getByRole('button', { name: 'right click here' });
  await trigger.click({ button: 'right' });
  await expect(page.getByRole('menu')).toHaveAttribute('data-state', 'open');

  // The backdrop is the only position:fixed div that fills the viewport and is not
  // the menu itself. Inspect it.
  const backdropInfo = await page.evaluate(() => {
    const fixedFull = Array.from(document.querySelectorAll('div')).filter((d) => {
      const s = getComputedStyle(d);
      if (s.position !== 'fixed') return false;
      const r = d.getBoundingClientRect();
      return r.width >= window.innerWidth - 1 && r.height >= window.innerHeight - 1 && d.getAttribute('role') !== 'menu';
    });
    if (fixedFull.length !== 1) return { count: fixedFull.length };
    const el = fixedFull[0];
    return {
      count: 1,
      ariaHidden: el.getAttribute('aria-hidden'),
      role: el.getAttribute('role'),
      ariaLabel: el.getAttribute('aria-label'),
    };
  });
  expect(backdropInfo.count).toBe(1);
  // An invisible full-viewport interactive div with no semantics — screen-reader users
  // get nothing, and the click-target has no announced purpose.
  expect(backdropInfo.ariaHidden).toBeNull();
  expect(backdropInfo.role).toBeNull();
});

test('ISSUE 3: opening the menu no longer locks html overflow, so the page scrolls underneath the fixed menu', async ({ page }) => {
  await page.goto(URL, { timeout: 20 * 60 * 1000 });
  // Make the document tall enough to scroll.
  await page.evaluate(() => {
    const spacer = document.createElement('div');
    spacer.style.height = '4000px';
    document.body.appendChild(spacer);
  });

  const trigger = page.getByRole('button', { name: 'right click here' });
  await trigger.click({ button: 'right' });
  const menu = page.getByRole('menu');
  await expect(menu).toHaveAttribute('data-state', 'open');

  const before = await menu.boundingBox();
  const overflow = await page.evaluate(() => getComputedStyle(document.documentElement).overflow);
  // The previous body-scroll-lock effect was removed; html overflow is no longer hidden.
  expect(overflow).not.toBe('hidden');

  // Now scroll the page and confirm the trigger moves but the fixed menu stays put —
  // the visual decoupling the review flagged.
  const triggerBefore = await trigger.boundingBox();
  await page.evaluate(() => window.scrollBy(0, 300));
  await page.waitForTimeout(50);
  const menuAfter = await menu.boundingBox();
  const triggerAfter = await trigger.boundingBox();

  if (!before || !triggerBefore || !menuAfter || !triggerAfter) throw new Error('missing box');
  // Menu's fixed position is unchanged in viewport coords.
  expect(Math.abs(menuAfter.y - before.y)).toBeLessThan(2);
  // Trigger has moved up by ~300px in viewport coords.
  expect(triggerBefore.y - triggerAfter.y).toBeGreaterThan(250);
});

test('ISSUE 5: backdrop closes the menu when a pointerdown is dispatched at the long-press location', async ({ page }) => {
  // Real iOS Safari sometimes issues a pointercancel on the trigger and a
  // fresh pointerdown on the newly-topmost element when the DOM topology
  // changes mid-touch (which happens the instant the backdrop mounts), or
  // dispatches a compat-mouse-promoted pointerdown ~300ms after the touch
  // ends. Either way, the result is a pointerdown landing on the full-viewport
  // backdrop at the finger location — and the backdrop's close_on_outside
  // handler fires unconditionally, dismissing the menu the user just opened.
  await page.goto(URL, { timeout: 20 * 60 * 1000 });
  const trigger = page.getByRole('button', { name: 'right click here' });
  // Use right-click to get the menu open synchronously, then synthesize the
  // hostile pointerdown on the backdrop area to prove the mechanism.
  await trigger.click({ button: 'right' });
  await expect(page.getByRole('menu')).toHaveAttribute('data-state', 'open');

  const box = await trigger.boundingBox();
  if (!box) throw new Error('no box');
  // Dispatch pointerdown at the trigger's *center* — exactly where the user's
  // finger is during a long-press. The backdrop covers this location.
  await page.evaluate(({ x, y }) => {
    const el = document.elementFromPoint(x, y);
    if (!el) throw new Error('no element at finger location');
    el.dispatchEvent(new PointerEvent('pointerdown', {
      pointerId: 6060, pointerType: 'touch', isPrimary: true,
      clientX: x, clientY: y, button: 0, buttons: 1,
      bubbles: true, cancelable: true,
    }));
  }, { x: box.x + box.width / 2, y: box.y + box.height / 2 });

  // Menu got dismissed by its own backdrop — the bug.
  await expect(page.getByRole('menu')).toHaveCount(0);
});

test('ISSUE 4: long-press test passes only because toHaveAttribute retries — a tight non-retrying check fails', async ({ page }) => {
  await page.goto(URL, { timeout: 20 * 60 * 1000 });
  const trigger = page.getByRole('button', { name: 'right click here' });
  const box = await trigger.boundingBox();
  if (!box) throw new Error('no box');
  const x = box.x + box.width / 2;
  const y = box.y + box.height / 2;
  const pointerId = 9999;

  await trigger.evaluate((el, { x, y, pointerId }) => {
    el.dispatchEvent(new PointerEvent('pointerdown', {
      pointerId, pointerType: 'touch', isPrimary: true,
      clientX: x, clientY: y, button: 0, buttons: 1,
      bubbles: true, cancelable: true,
    }));
  }, { x, y, pointerId });

  // The long-press timer is 500ms. Sample synchronously at +100ms — menu must not be open yet.
  await page.waitForTimeout(100);
  const earlyCount = await page.getByRole('menu').count();
  expect(earlyCount).toBe(0);

  // Confirm it WILL be open after the threshold elapses — to prove the retry was hiding this.
  await page.waitForTimeout(600);
  await expect(page.getByRole('menu')).toHaveAttribute('data-state', 'open');
});
