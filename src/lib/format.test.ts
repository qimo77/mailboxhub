import { describe, expect, it } from 'vitest';
import { badgeCount, initials, senderName } from './format';

describe('format helpers', () => {
  it('creates initials from an email account name', () => {
    expect(initials('Katie.Ferguson@outlook.com')).toBe('KF');
  });

  it('caps large unread badge counts', () => {
    expect(badgeCount(120)).toBe('99+');
  });

  it('prefers sender display name over email', () => {
    expect(senderName('OpenAI', 'noreply@example.com')).toBe('OpenAI');
  });
});
