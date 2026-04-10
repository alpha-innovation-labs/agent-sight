import Link from 'next/link';

type LogoProps = {
  className?: string;
};

export function Logo({ className }: LogoProps) {
  return (
    <Link
      href="/"
      className={`font-mono text-sm font-semibold uppercase tracking-[0.24em] text-foreground ${className ?? ''}`}
    >
      Agent Sight
    </Link>
  );
}
