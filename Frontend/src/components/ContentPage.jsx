import React from 'react';

export default function ContentPage({ title, children }) {
  return (
    <div className="flex-1 flex flex-col m-0 sm:m-4 lg:m-8 sm:rounded-2xl lg:rounded-3xl shadow-apple dark:shadow-apple-dark overflow-y-auto bg-surface border border-subtle p-8 lg:p-12">
      <div className="max-w-3xl mx-auto w-full">
        <h2 className="text-3xl font-bold text-primary mb-6 tracking-tight">{title}</h2>
        <div className="prose dark:prose-invert prose-blue max-w-none text-secondary leading-relaxed">
          {children}
        </div>
      </div>
    </div>
  );
}
