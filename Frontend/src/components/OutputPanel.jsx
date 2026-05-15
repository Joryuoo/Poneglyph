import React, { useState, useRef, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';

export default function OutputPanel({ execState, terminalOutput, onScanSubmit }) {
  const [currentInput, setCurrentInput] = useState('');
  const hiddenInputRef = useRef(null);
  const terminalEndRef = useRef(null);
  const containerRef = useRef(null);

  // Auto-scroll to bottom when output changes or when waiting for input
  useEffect(() => {
    if (terminalEndRef.current) {
      terminalEndRef.current.scrollIntoView({ behavior: 'smooth' });
    }
  }, [terminalOutput, execState, currentInput]);

  // Auto-focus the hidden input when waiting for SCAN
  useEffect(() => {
    if (execState === 'waiting' && hiddenInputRef.current) {
      hiddenInputRef.current.focus();
    }
  }, [execState]);

  // Reset input when execution state changes from something else to waiting
  useEffect(() => {
    if (execState === 'waiting') {
      setCurrentInput('');
    }
  }, [execState]);

  // Focus hidden input when clicking anywhere on the terminal
  const handleTerminalClick = () => {
    if (execState === 'waiting' && hiddenInputRef.current) {
      hiddenInputRef.current.focus();
    }
  };

  const handleKeyDown = (e) => {
    if (e.key === 'Enter' && execState === 'waiting') {
      e.preventDefault();
      const submittedInput = currentInput;
      setCurrentInput('');
      onScanSubmit(submittedInput);
    }
  };

  const handleInputChange = (e) => {
    if (execState === 'waiting') {
      setCurrentInput(e.target.value);
    }
  };

  // Determine if we should show the idle placeholder
  const isIdle = execState === 'idle';
  const isRunning = execState === 'running';
  const isWaiting = execState === 'waiting';
  const isDone = execState === 'done' || execState === 'error';
  const hasOutput = terminalOutput && terminalOutput.length > 0;

  return (
    <div className="h-full w-full bg-surface flex flex-col relative overflow-hidden">
      {/* Header */}
      <div className="px-4 py-3 border-b border-subtle flex justify-between items-center shrink-0">
        <div className="flex items-center gap-2">
          <span className="text-xs font-medium uppercase tracking-wider text-secondary">Console Output</span>
          {isWaiting && (
            <span className="text-[10px] font-medium uppercase tracking-wider text-blue-400 animate-pulse">
              · awaiting input
            </span>
          )}
        </div>
      </div>
      
      {/* Terminal Body */}
      <div 
        ref={containerRef}
        className={`flex-1 overflow-auto p-4 font-mono text-[13px] text-primary${isWaiting ? ' cursor-text' : ''}`}
        onClick={handleTerminalClick}
      >
        <AnimatePresence mode="wait">
          {isRunning ? (
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
          ) : isIdle && !hasOutput ? (
            <motion.div
              key="idle"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className="text-secondary italic"
            >
              No output yet. Run the code to see results.
            </motion.div>
          ) : (
            <motion.div
              key="terminal"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className="whitespace-pre-wrap break-words"
            >
              {/* Rendered output text */}
              {terminalOutput}

              {/* Inline input area when waiting for SCAN */}
              {isWaiting && (
                <span className="inline">
                  <span className="text-green-400">{currentInput}</span>
                  <span className="inline-block w-[2px] h-[15px] bg-green-400 align-middle animate-terminal-blink ml-[1px]"></span>
                </span>
              )}

              {/* Scroll anchor */}
              <div ref={terminalEndRef} />
            </motion.div>
          )}
        </AnimatePresence>
      </div>

      {/* Hidden input to capture keystrokes when in waiting state */}
      {isWaiting && (
        <input
          ref={hiddenInputRef}
          type="text"
          value={currentInput}
          onChange={handleInputChange}
          onKeyDown={handleKeyDown}
          className="absolute opacity-0 pointer-events-none"
          style={{ position: 'fixed', top: '-9999px', left: '-9999px' }}
          autoFocus
        />
      )}
    </div>
  );
}
