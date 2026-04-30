import React from 'react';
import { motion, AnimatePresence } from 'framer-motion';

export default function OutputPanel({ output, isExecuting }) {
  return (
    <div className="h-full w-full bg-surface flex flex-col relative overflow-hidden">
      <div className="px-4 py-3 border-b border-subtle bg-gray-50/50 dark:bg-gray-900/50 flex justify-between items-center shrink-0">
        <span className="text-xs font-medium uppercase tracking-wider text-secondary">Console Output</span>
      </div>
      
      <div className="flex-1 overflow-auto p-4 font-mono text-[13px] text-primary">
        <AnimatePresence mode="wait">
          {isExecuting ? (
            <motion.div
              key="loading"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className="flex items-center gap-3 text-secondary"
            >
              <div className="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin"></div>
              <span>Executing Poneglyph script...</span>
            </motion.div>
          ) : (
            <motion.div
              key="output"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className="whitespace-pre-wrap break-words"
            >
              {output || <span className="text-secondary italic">No output yet. Run the code to see results.</span>}
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </div>
  );
}
