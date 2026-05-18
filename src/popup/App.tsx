import { useEffect } from "react";
import { AnimatePresence, motion } from "framer-motion";
import { PopupShell } from "./components/PopupShell";
import { useTranslation } from "./store";

export function App() {
  const init = useTranslation((s) => s.init);
  const dispose = useTranslation((s) => s.dispose);
  const visible = useTranslation((s) => s.visible);

  useEffect(() => {
    init();
    return () => {
      dispose();
    };
  }, [init, dispose]);

  return (
    <div className="popup-root h-full w-full p-2">
      <AnimatePresence>
        {visible && (
          <motion.div
            key="popup"
            initial={{ opacity: 0, y: -6, scale: 0.985 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            exit={{ opacity: 0, y: -4, scale: 0.99 }}
            transition={{ duration: 0.14, ease: [0.16, 1, 0.3, 1] }}
            className="h-full w-full"
          >
            <PopupShell />
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
