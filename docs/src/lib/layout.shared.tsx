import type { BaseLayoutProps } from 'fumadocs-ui/layouts/shared';
import { BookOpen, Github } from 'lucide-react';
import { Logo } from '@/components/logo';

export function baseOptions(): BaseLayoutProps {
  return {
    nav: {
      title: <Logo />,
    },
    links: [
      {
        type: 'button',
        text: 'Docs',
        icon: <BookOpen className="h-4 w-4" />,
        url: '/docs',
        active: 'nested-url',
        on: 'nav',
      },
      {
        type: 'button',
        text: 'Repo',
        icon: <Github className="h-4 w-4" />,
        url: 'https://github.com/Alpha-Innovation-Labs/agent-sight',
        external: true,
        active: 'none',
        on: 'nav',
      },
    ],
  };
}
