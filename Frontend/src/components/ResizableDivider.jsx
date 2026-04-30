import React, { useState, useEffect } from 'react';

export default function ResizableDivider({ onResize }) {
  const [isDragging, setIsDragging] = useState(false);

  useEffect(() => {
    const handleMouseMove = (e) => {
      if (!isDragging) return;
      onResize(e.clientX);
    };

    const handleMouseUp = () => {
      if (isDragging) {
        setIsDragging(false);
        document.body.style.cursor = 'default';
        document.body.style.userSelect = 'auto';
      }
    };

    if (isDragging) {
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
    }

    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
  }, [isDragging, onResize]);

  return (
    <div
      className="w-1 lg:w-1.5 cursor-col-resize hover:bg-blue-400/50 active:bg-blue-500 transition-colors bg-divider shrink-0 flex items-center justify-center relative z-10"
      onMouseDown={(e) => {
        e.preventDefault();
        setIsDragging(true);
        document.body.style.cursor = 'col-resize';
        document.body.style.userSelect = 'none';
      }}
    >
      {/* Visual grabber */}
      <div className="h-8 w-1 rounded-full bg-gray-400/30"></div>
    </div>
  );
}
