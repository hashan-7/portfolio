import type { SocialLinks } from '../types';
import { emailLink, formatExternalLink, phoneLink } from './links';

export type SocialKey =
  | 'github'
  | 'linkedin'
  | 'huggingface'
  | 'kaggle'
  | 'email'
  | 'phone'
  | 'resume'
  | 'website';

export interface SocialItem {
  key: SocialKey;
  label: string;
  href: string;
}

export function getSocialItems(links?: SocialLinks): SocialItem[] {
  if (!links) return [];

  const external = (key: SocialKey, label: string, value?: string): SocialItem | undefined => {
    const href = formatExternalLink(value);
    return href ? { key, label, href } : undefined;
  };
  const emailHref = emailLink(links.email);
  const phoneHref = phoneLink(links.phone);

  return [
    external('github', 'GitHub', links.github),
    external('linkedin', 'LinkedIn', links.linkedin),
    external('huggingface', 'Hugging Face', links.huggingface),
    external('kaggle', 'Kaggle', links.kaggle),
    emailHref ? { key: 'email' as const, label: 'Email', href: emailHref } : undefined,
    phoneHref ? { key: 'phone' as const, label: 'Phone', href: phoneHref } : undefined,
    external('resume', 'CV / Resume', links.resume),
    external('website', 'Website', links.website),
  ].filter((item): item is SocialItem => Boolean(item));
}
