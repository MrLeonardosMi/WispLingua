import { useCallback, useEffect, useRef, useState } from "react";
import type { HotkeyTrigger } from "@shared/types";

const MODIFIER_KEYS = new Set(["Control", "Shift", "Alt", "Meta", "OS"]);

function normalizeKey(code: string, key: string): string {
  if (code.startsWith("Key")) return code.slice(3);
  if (code.startsWith("Digit")) return code.slice(5);
  if (code.startsWith("Numpad")) return `Numpad${code.slice(6)}`;
  if (code.startsWith("Arrow")) return code.slice(5);
  if (code.startsWith("F") && /^F\d+$/.test(code)) return code;
  switch (code) {
    case "Space": return "Space";
    case "Enter": return "Enter";
    case "Tab": return "Tab";
    case "Escape": return "Escape";
    case "Backspace": return "Backspace";
    case "Delete": return "Delete";
    case "Slash": return "Slash";
    case "Backslash": return "Backslash";
    case "Period": return "Period";
    case "Comma": return "Comma";
    case "Semicolon": return "Semicolon";
    case "Quote": return "Quote";
    case "BracketLeft": return "BracketLeft";
    case "BracketRight": return "BracketRight";
    case "Minus": return "Minus";
    case "Equal": return "Equal";
    case "Backquote": return "Backquote";
  }
  return key.length === 1 ? key.toUpperCase() : key;
}

export interface RecorderState {
  recording: boolean;
  modifiers: string[];
  key: string | null;
  start: () => void;
  stop: () => void;
  build: (presses: number) => HotkeyTrigger | null;
}

export function useHotkeyRecorder(): RecorderState {
  const [recording, setRecording] = useState(false);
  const [modifiers, setModifiers] = useState<string[]>([]);
  const [key, setKey] = useState<string | null>(null);
  const recordingRef = useRef(false);

  useEffect(() => {
    recordingRef.current = recording;
  }, [recording]);

  useEffect(() => {
    function handler(e: KeyboardEvent) {
      if (!recordingRef.current) return;
      e.preventDefault();
      e.stopPropagation();
      const mods: string[] = [];
      if (e.ctrlKey) mods.push("Ctrl");
      if (e.shiftKey) mods.push("Shift");
      if (e.altKey) mods.push("Alt");
      if (e.metaKey) mods.push("Super");
      setModifiers(mods);
      if (!MODIFIER_KEYS.has(e.key)) {
        setKey(normalizeKey(e.code, e.key));
        setRecording(false);
      }
    }
    window.addEventListener("keydown", handler, true);
    return () => window.removeEventListener("keydown", handler, true);
  }, []);

  const start = useCallback(() => {
    setModifiers([]);
    setKey(null);
    setRecording(true);
  }, []);

  const stop = useCallback(() => setRecording(false), []);

  const build = useCallback(
    (presses: number): HotkeyTrigger | null => {
      if (key) {
        const combo = [...modifiers, key].join("+");
        return { mode: "Combo", combo, presses };
      }
      if (modifiers.length === 1) {
        return { mode: "Modifier", modifier: modifiers[0], presses };
      }
      return null;
    },
    [modifiers, key]
  );

  return { recording, modifiers, key, start, stop, build };
}

export function formatTrigger(trigger: HotkeyTrigger): string {
  const presses = trigger.presses > 1 ? ` ×${trigger.presses}` : "";
  if (trigger.mode === "Combo") return `${trigger.combo}${presses}`;
  return `${trigger.modifier}${presses}`;
}
