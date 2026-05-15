import React, { useEffect, useState } from 'react';
import { Moon, Sun } from 'lucide-react';

export default function Navbar({ currentView, setCurrentView }) {
  const [isDarkMode, setIsDarkMode] = useState(false);

  useEffect(() => {
    // Check initial dark mode state from class or system preference
    if (document.documentElement.classList.contains('dark')) {
      setIsDarkMode(true);
    } else if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
      setIsDarkMode(true);
      document.documentElement.classList.add('dark');
    }
  }, []);

  const toggleDarkMode = () => {
    const nextDarkMode = !isDarkMode;
    setIsDarkMode(nextDarkMode);
    if (nextDarkMode) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  };

  return (
    <nav className="flex items-center justify-between px-6 py-4 bg-surface border-b border-subtle shrink-0">
      <div className="flex items-center gap-8">
        <h1 
          className="text-xl font-bold text-primary tracking-wide cursor-pointer"
          onClick={() => setCurrentView('playground')}
        >
          Poneglyph
        </h1>
        <div className="flex items-center gap-6 text-sm font-medium text-secondary">
          <button 
            onClick={() => setCurrentView('about')} 
            className={`hover:text-primary transition-colors ${currentView === 'about' ? 'text-primary font-bold' : ''}`}
          >
            About
          </button>
          <button 
            onClick={() => setCurrentView('architecture')} 
            className={`hover:text-primary transition-colors ${currentView === 'architecture' ? 'text-primary font-bold' : ''}`}
          >
            Architecture
          </button>
        </div>
      </div>
      <div className="flex items-center gap-4">
        <button
          onClick={toggleDarkMode}
          className="p-2 text-secondary hover:text-primary transition-colors rounded-full hover:bg-gray-100 dark:hover:bg-gray-800"
          aria-label="Toggle Dark Mode"
        >
          {isDarkMode ? <Sun className="w-5 h-5" /> : <Moon className="w-5 h-5" />}
        </button>
      </div>
    </nav>
  );
}
