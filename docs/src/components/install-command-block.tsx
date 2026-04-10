'use client';

import { Check, Copy } from 'lucide-react';
import { useMemo, useState } from 'react';

export type InstallOption = {
  id: string;
  label: string;
  command: string;
  displayLines?: string[];
};

type InstallCommandBlockProps = {
  options: InstallOption[];
};

export function InstallCommandBlock({ options }: InstallCommandBlockProps) {
  const [active, setActive] = useState(options[0]?.id ?? '');
  const [copied, setCopied] = useState(false);

  const activeInstall = useMemo(
    () => options.find((option) => option.id === active) ?? options[0],
    [active, options],
  );

  async function copyInstallCommand() {
    if (!activeInstall) return;

    await navigator.clipboard.writeText(activeInstall.command);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  }

  return (
    <div className="overflow-hidden border border-border/60 bg-card">
      <div className="flex flex-wrap gap-2 border-b border-border/60 bg-muted/20 p-2">
        {options.map((option) => {
          const isActive = option.id === active;

          return (
            <button
              key={option.id}
              type="button"
              onClick={() => setActive(option.id)}
              className={[
                'px-3 py-1.5 font-mono text-xs uppercase tracking-[0.18em] transition',
                isActive
                  ? 'bg-foreground text-background'
                  : 'text-muted-foreground hover:bg-background hover:text-foreground',
              ].join(' ')}
            >
              {option.label}
            </button>
          );
        })}
      </div>

      <div className="flex items-start gap-3 bg-background px-4 py-3">
        <span className="pt-0.5 text-emerald-500">$</span>
        <code className="flex-1 overflow-x-auto font-mono text-sm leading-6 whitespace-pre">
          {activeInstall?.displayLines?.join('\n') ?? activeInstall?.command}
        </code>
        <button
          type="button"
          onClick={copyInstallCommand}
          className="inline-flex h-9 w-9 items-center justify-center border border-border/60 text-muted-foreground transition hover:bg-muted/30 hover:text-foreground"
          aria-label="Copy install command"
        >
          {copied ? <Check className="h-4 w-4" /> : <Copy className="h-4 w-4" />}
        </button>
      </div>
    </div>
  );
}
