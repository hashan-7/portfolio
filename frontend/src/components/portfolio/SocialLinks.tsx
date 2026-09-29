import type { SocialItem, SocialKey } from '../../utils/social';

interface SocialLinksProps {
  items: SocialItem[];
  className?: string;
  exclude?: SocialKey[];
}

function SocialLinks({ items, className = 'social-links', exclude = [] }: SocialLinksProps) {
  const visibleItems = items.filter((item) => !exclude.includes(item.key));

  if (visibleItems.length === 0) return null;

  return (
    <div className={className} aria-label="Profile and contact links">
      {visibleItems.map((item) => {
        const opensNewTab = !item.href.startsWith('mailto:') && !item.href.startsWith('tel:');

        return (
          <a
            href={item.href}
            target={opensNewTab ? '_blank' : undefined}
            rel={opensNewTab ? 'noreferrer' : undefined}
            key={item.key}
          >
            <span>{item.label}</span>
            {opensNewTab && <span className="external-mark" aria-hidden="true">↗</span>}
          </a>
        );
      })}
    </div>
  );
}

export default SocialLinks;
