import React from 'react';
import { motion } from 'framer-motion';

export default function SplashScreen() {
  return (
    <motion.div
      initial={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      transition={{ duration: 0.8, ease: "easeInOut" }}
      className="fixed inset-0 z-50 flex items-center justify-center bg-main"
    >
      <motion.div
        initial={{ opacity: 0, y: 15, filter: "blur(8px)" }}
        animate={{ opacity: 1, y: 0, filter: "blur(0px)" }}
        transition={{ duration: 1.2, ease: "easeOut" }}
        className="flex flex-col items-center gap-4"
      >
        <h1 className="text-3xl md:text-4xl font-light tracking-[0.3em] text-primary uppercase">
          Poneglyph
        </h1>
        <motion.div 
          initial={{ scaleX: 0 }}
          animate={{ scaleX: 1 }}
          transition={{ duration: 1, delay: 0.5, ease: "easeInOut" }}
          className="h-[1px] bg-secondary/50 w-full"
        />
      </motion.div>
    </motion.div>
  );
}
