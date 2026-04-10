import { ArrowRight, Github } from 'lucide-react';
import Link from 'next/link';
import { Footer } from '@/components/footer';
import { InstallCommandBlock } from '@/components/install-command-block';
import { Navbar } from '@/components/navbar';

const INSTALL_OPTIONS = [
  {
    id: 'npm',
    label: 'npm',
    command: 'npm install -g agent-sight',
  },
];

export default function HomePage() {
  return (
    <div className="flex min-h-screen flex-col bg-background">
      <Navbar />

      <main className="flex-1 bg-background">
        <div className="mx-auto w-full max-w-[60rem] px-5 pb-16 sm:px-6 sm:pb-24 lg:px-7 lg:pb-28">
          <div className="space-y-6 pt-12 pb-5 text-center sm:space-y-7 sm:pt-16 sm:pb-6 md:pt-20 md:pb-7">
            <h1 className="text-4xl font-normal tracking-tight text-foreground sm:text-5xl">
              Query local agent history without digging through raw files
            </h1>

            <p className="mx-auto max-w-2xl text-sm leading-relaxed text-foreground/70 sm:text-[15px]">
              Agent Sight is a Rust CLI for reading OpenCode SQLite sessions and Claude history,
              then returning structured JSON for scripting, analysis, and local tooling.
            </p>

            <div className="flex flex-wrap items-center justify-center gap-3">
              <Link
                href="/docs"
                className="inline-flex items-center gap-2 border border-fd-primary bg-fd-primary px-4 py-2 text-sm font-medium text-fd-primary-foreground transition hover:opacity-90"
              >
                Documentation
                <ArrowRight className="h-4 w-4" />
              </Link>

              <a
                href="https://github.com/Alpha-Innovation-Labs/agent-sight"
                target="_blank"
                rel="noreferrer"
                className="inline-flex items-center gap-2 border border-fd-border px-4 py-2 text-sm font-medium transition hover:bg-fd-accent"
              >
                <Github className="h-4 w-4" />
                View on GitHub
              </a>
            </div>
          </div>

          <InstallCommandBlock options={INSTALL_OPTIONS} />

          <div className="mt-8 space-y-4 sm:mt-10 sm:space-y-5">
            <div className="overflow-hidden rounded-xl border border-fd-border/80 bg-fd-card shadow-[0_28px_80px_rgba(0,0,0,0.18)]">
              <div className="flex items-center gap-2 border-b border-fd-border/80 bg-fd-muted/30 px-4 py-3">
                <span className="h-3 w-3 rounded-full bg-[#ff6b6b]" />
                <span className="h-3 w-3 rounded-full bg-[#ffd166]" />
                <span className="h-3 w-3 rounded-full bg-[#06d6a0]" />
                <span className="ml-3 font-mono text-[11px] uppercase tracking-[0.28em] text-fd-muted-foreground/80">
                  Demo Terminal
                </span>
              </div>

              <div className="grid min-h-[420px] place-items-center bg-[radial-gradient(circle_at_top,rgba(81,109,255,0.12),transparent_45%),linear-gradient(180deg,rgba(17,24,39,0.96),rgba(10,14,24,1))] px-6 py-10 text-left text-white">
                <div className="w-full max-w-3xl rounded-lg border border-white/10 bg-black/40 p-5 shadow-2xl backdrop-blur-sm">
                  <div className="overflow-hidden rounded-md border border-white/10 bg-[#0b1020]">
                    <div className="flex items-center gap-2 border-b border-white/10 bg-white/5 px-4 py-2.5">
                      <span className="h-2.5 w-2.5 rounded-full bg-[#ff6b6b]" />
                      <span className="h-2.5 w-2.5 rounded-full bg-[#ffd166]" />
                      <span className="h-2.5 w-2.5 rounded-full bg-[#06d6a0]" />
                      <span className="ml-3 font-mono text-[11px] uppercase tracking-[0.28em] text-white/50">
                        agent-sight-demo.tape
                      </span>
                    </div>

                    <div className="space-y-3 px-5 py-5 font-mono text-sm leading-6 text-white/90">
                      <div className="text-emerald-400">$ just cli query --since 24h --source claude --full</div>
                      <div className="text-white/55">{`{`}</div>
                      <div className="pl-4 text-sky-200/90">"source": "claude",</div>
                      <div className="pl-4 text-sky-200/90">"conversation_count": 2,</div>
                      <div className="pl-4 text-sky-200/90">"message_count": 6,</div>
                      <div className="pl-4 text-sky-200/90">"conversations": [</div>
                      <div className="pl-8 text-white/80">{'{"session_id": "project#1712841942000", "user_message_count": 4},'}</div>
                      <div className="pl-8 text-white/80">{'{"session_id": "project#1712845519000", "user_message_count": 2}'}</div>
                      <div className="pl-4 text-sky-200/90">]</div>
                      <div className="text-white/55">{`}`}</div>
                    </div>
                  </div>

                  <div className="mt-8 border-t border-white/10 pt-6">
                    <p className="font-mono text-xs uppercase tracking-[0.28em] text-sky-200/70">
                      Demo Recording
                    </p>
                    <p className="mt-2 text-lg font-medium text-white">Coming soon</p>
                    <p className="mt-2 max-w-xl text-sm leading-relaxed text-white/65">
                      This section will hold the terminal walkthrough once a recorded demo is ready.
                      For now, the layout matches the same centered terminal showcase pattern as the
                      lazyskills site.
                    </p>
                  </div>
                </div>
              </div>
            </div>

            <p className="text-left text-sm leading-relaxed text-foreground/70 sm:text-[15px]">
              Agent Sight gives you a clean way to inspect local OpenCode and Claude history without
              manually opening SQLite databases or parsing JSONL files yourself. The goal is a small,
              scriptable CLI with predictable output rather than a full interactive interface.
            </p>
          </div>
        </div>
      </main>

      <Footer />
    </div>
  );
}
