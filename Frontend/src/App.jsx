import React, { useState, useCallback, useEffect } from 'react';
import init, { run_agartha_code } from './pkg/Agartha.js';
import TopBar from './components/TopBar';
import EditorPanel from './components/EditorPanel';
import OutputPanel from './components/OutputPanel';
import ResizableDivider from './components/ResizableDivider';
import SplashScreen from './components/SplashScreen';
import { motion, AnimatePresence } from 'framer-motion';

const INITIAL_CODE = `%% Welcome to the Agartha Poneglyph IDE
SCRIPT AREA
START SCRIPT

%% Declaring different datatypes
DECLARE INT level = 100
DECLARE FLOAT power = 99.9
DECLARE CHAR rank = 'S'
DECLARE BOOL is_active = "TRUE"

%% Print Hello World and our variables
PRINT: "Hello, World!" & $
PRINT: "User Level: " & level & $ & "Power: " & power & $ & "Rank: " & rank

END SCRIPT
`;

function App() {
  const [code, setCode] = useState(INITIAL_CODE);
  const [output, setOutput] = useState('');
  const [isExecuting, setIsExecuting] = useState(false);
  const [isOutputVisible, setIsOutputVisible] = useState(true);
  const [editorWidth, setEditorWidth] = useState(50); // percentage
  const [showSplash, setShowSplash] = useState(true);
  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
    init()
      .then(() => setIsReady(true))
      .catch((err) => {
        console.error("Failed to initialize Wasm:", err);
        setOutput(`> Error loading Wasm engine:\n${err}`);
      });
  }, []);

  useEffect(() => {
    const timer = setTimeout(() => {
      setShowSplash(false);
    }, 2500); // 2.5s splash screen duration
    return () => clearTimeout(timer);
  }, []);

  const handleExecute = () => {
    setIsExecuting(true);
    // Ensure output is visible when we run
    setIsOutputVisible(true);
    
    // Give UI a moment to update before blocking the thread with WASM
    setTimeout(() => {
      try {
        const result = run_agartha_code(code);
        setOutput(result);
      } catch (err) {
        setOutput(`> Runtime Error:\n${err}`);
      } finally {
        setIsExecuting(false);
      }
    }, 50);
  };

  const handleToggleOutput = () => {
    if (!isExecuting) {
      setIsOutputVisible(!isOutputVisible);
    }
  };

  const handleResize = useCallback((clientX) => {
    const newWidth = (clientX / window.innerWidth) * 100;
    // Constrain width between 20% and 80%
    if (newWidth > 20 && newWidth < 80) {
      setEditorWidth(newWidth);
    }
  }, []);

  return (
    <div className="h-screen w-screen flex flex-col bg-main text-primary overflow-hidden">
      <AnimatePresence>
        {showSplash && <SplashScreen />}
      </AnimatePresence>

      {/* Container simulating a standalone app window if we want, but edge-to-edge is cleaner */}
      <div className="flex-1 flex flex-col m-0 sm:m-4 lg:m-8 sm:rounded-2xl lg:rounded-3xl shadow-apple dark:shadow-apple-dark overflow-hidden bg-surface border border-subtle">
        
        <TopBar
          isExecuting={isExecuting}
          isReady={isReady}
          onExecute={handleExecute}
          isOutputVisible={isOutputVisible}
          onToggleOutput={handleToggleOutput}
        />

        <div className="flex-1 flex flex-col lg:flex-row overflow-hidden relative">
          
          {/* Editor Panel */}
          <motion.div 
            className="h-1/2 lg:h-full flex-shrink-0"
            animate={{ 
              width: window.innerWidth >= 1024 ? (isOutputVisible ? `${editorWidth}%` : '100%') : '100%',
              height: window.innerWidth < 1024 ? (isOutputVisible ? '50%' : '100%') : '100%'
            }}
            transition={{ type: 'spring', bounce: 0, duration: 0.3 }}
          >
            <EditorPanel code={code} onChange={setCode} />
          </motion.div>

          {/* Divider (Desktop only when output is visible) */}
          {isOutputVisible && (
            <div className="hidden lg:block">
              <ResizableDivider onResize={handleResize} />
            </div>
          )}

          {/* Output Panel */}
          <motion.div
            className="lg:h-full flex-1 border-t lg:border-t-0 lg:border-l border-subtle overflow-hidden relative"
            initial={false}
            animate={{
              opacity: isOutputVisible ? 1 : 0,
              flex: isOutputVisible ? 1 : 0,
              width: window.innerWidth >= 1024 ? (isOutputVisible ? `${100 - editorWidth}%` : '0%') : '100%',
              height: window.innerWidth < 1024 ? (isOutputVisible ? '50%' : '0%') : '100%'
            }}
            transition={{ type: 'spring', bounce: 0, duration: 0.3 }}
            style={{
              pointerEvents: isOutputVisible ? 'auto' : 'none'
            }}
          >
            <OutputPanel output={output} isExecuting={isExecuting} />
          </motion.div>

        </div>
      </div>
    </div>
  );
}

export default App;
