import { useEffect, useState } from "react";
import { Sparkles } from "lucide-react";
import { useConfig } from "@shared/store/config";
import { GeneralSection } from "./sections/General";
import { HotkeySection } from "./sections/Hotkey";
import { ProviderSection } from "./sections/Provider";
import { AppearanceSection } from "./sections/Appearance";
import { AboutSection } from "./sections/About";
import { SideNav, type SectionKey } from "./components/SideNav";

const SECTIONS: { key: SectionKey; label: string }[] = [
  { key: "general", label: "General" },
  { key: "hotkey", label: "Hotkey" },
  { key: "provider", label: "Provider" },
  { key: "appearance", label: "Appearance" },
  { key: "about", label: "About" },
];

export function App() {
  const { config, loading, error, load } = useConfig();
  const [active, setActive] = useState<SectionKey>("general");

  useEffect(() => {
    load();
  }, [load]);

  return (
    <div className="flex h-full w-full bg-[var(--color-bg)] text-[var(--color-text)]">
      <SideNav active={active} onChange={setActive} sections={SECTIONS} />
      <main className="scroll-thin flex-1 overflow-y-auto">
        <div className="mx-auto flex max-w-3xl flex-col gap-8 px-10 py-10">
          <header className="flex items-center gap-2.5">
            <Sparkles size={18} className="text-[var(--color-accent)]" />
            <h1 className="text-xl font-semibold tracking-tight">WispLingua</h1>
            <span className="rounded-full bg-[var(--color-bg-soft)] px-2 py-0.5 text-[10px] uppercase tracking-wider text-[var(--color-text-muted)]">
              Settings
            </span>
          </header>

          {loading && !config && (
            <div className="text-sm text-[var(--color-text-dim)]">Loading…</div>
          )}
          {error && (
            <div className="rounded-lg border border-[var(--color-danger)]/40 bg-[var(--color-danger)]/10 px-3 py-2 text-sm text-[var(--color-danger)]">
              {error}
            </div>
          )}

          {config && (
            <>
              {active === "general" && <GeneralSection />}
              {active === "hotkey" && <HotkeySection />}
              {active === "provider" && <ProviderSection />}
              {active === "appearance" && <AppearanceSection />}
              {active === "about" && <AboutSection />}
            </>
          )}
        </div>
      </main>
    </div>
  );
}
