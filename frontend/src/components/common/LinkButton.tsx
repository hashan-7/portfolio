import type { ReactNode } from 'react';

interface LinkButtonProps {
  href?: string;
  children: ReactNode;
  className?: string;
}

function LinkButton({ href, children, className = 'card-link' }: LinkButtonProps) {
  if (!href) {
    return null;
  }

  return (
    <a className={className} href={href} target="_blank" rel="noreferrer">
      {children}
    </a>
  );
}

export default LinkButton;