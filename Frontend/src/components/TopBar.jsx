import React from 'react';
import { Play, ChevronRight, ChevronLeft, Loader2 } from 'lucide-react';

export default function TopBar({
  isExecuting,
  isReady,
  onExecute,
  isOutputVisible,
  onToggleOutput
}) {
  return (
    <div className="flex items-center justify-between px-4 h-14 bg-surface border-b border-subtle shrink-0">
      <div className="flex items-center gap-4">
        {/* Fake window controls for iOS aesthetic */}
        <div className="flex items-center gap-2 mr-2">
          <div className="w-3 h-3 rounded-full bg-red-500"></div>
          <div className="w-3 h-3 rounded-full bg-yellow-500"></div>
          <div className="w-3 h-3 rounded-full bg-green-500"></div>
        </div>
        
        <div className="flex flex-col">
          <span className="text-sm font-semibold text-primary">Playground</span>
        </div>
      </div>

      <div className="flex items-center gap-2">
        <button
          onClick={onExecute}
          disabled={isExecuting || !isReady}
          className="flex items-center justify-center gap-2 px-4 py-1.5 rounded-full bg-[#007aff] text-white text-sm font-medium hover:bg-[#0056b3] transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {!isReady ? (
            <>
              <Loader2 className="w-4 h-4 animate-spin" />
              <span>Initializing...</span>
            </>
          ) : isExecuting ? (
            <>
              <Loader2 className="w-4 h-4 animate-spin" />
              <span>Running...</span>
            </>
          ) : (
            <>
              <Play className="w-4 h-4" fill="currentColor" />
              <span>Execute</span>
            </>
          )}
        </button>

        <button
          onClick={onToggleOutput}
          disabled={isExecuting}
          className={`p-2 text-secondary hover:text-primary transition-colors rounded-full hover:bg-gray-100 dark:hover:bg-gray-800 ${
            isExecuting ? 'opacity-50 cursor-not-allowed' : ''
          }`}
          title={isOutputVisible ? "Collapse Output" : "Expand Output"}
        >
          {isOutputVisible ? <ChevronRight className="w-5 h-5" /> : <ChevronLeft className="w-5 h-5" />}
        </button>
      </div>
    </div>
  );
}
