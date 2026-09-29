export function hasLink(value?: string): boolean {
  return Boolean(value && value.trim().length > 0);
}

export function formatExternalLink(value?: string): string | undefined {
  if (!value) return undefined;
  const trimmed = value.trim();
  if (!trimmed) return undefined;

  if (trimmed.startsWith('/') && !trimmed.startsWith('//')) {
    return trimmed;
  }

  const candidate = /^[a-z][a-z\d+.-]*:/i.test(trimmed) ? trimmed : `https://${trimmed}`;

  try {
    const url = new URL(candidate);
    return ['http:', 'https:', 'mailto:', 'tel:'].includes(url.protocol) ? url.href : undefined;
  } catch {
    return undefined;
  }
}

export function emailLink(email?: string): string | undefined {
  const value = email?.trim();
  if (!value || /[\r\n]/.test(value)) return undefined;
  return formatExternalLink(`mailto:${value}`);
}

export function phoneLink(phone?: string): string | undefined {
  const value = phone?.trim();
  if (!value || /[\r\n]/.test(value)) return undefined;
  return formatExternalLink(`tel:${value}`);
}
