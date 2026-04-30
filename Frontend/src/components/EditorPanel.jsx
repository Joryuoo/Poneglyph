import React from 'react';
import EditorModule from 'react-simple-code-editor';
const Editor = EditorModule.default || EditorModule;
import Prism from 'prismjs';

// Define custom syntax grammar for Agartha Poneglyph
Prism.languages.agartha = {
  'comment': {
    pattern: /%%.*/,
    greedy: true
  },
  'boolean': {
    pattern: /"TRUE"|"FALSE"|TRUE|FALSE/,
    greedy: true
  },
  'string': {
    pattern: /(["'])(?:\\(?:\r\n|[\s\S])|(?!\1)[^\\\r\n])*\1/,
    greedy: true
  },
  'keyword': /\b(?:SCRIPT AREA|START SCRIPT|END SCRIPT|DECLARE|IF|ELSE IF|ELSE|START IF|END IF|FOR|START FOR|END FOR|REPEAT WHEN|START REPEAT|END REPEAT|PRINT|SCAN)\b/,
  'datatype': /\b(?:INT|FLOAT|CHAR|BOOL)\b/,
  'logical': /\b(?:AND|OR|NOT)\b/,
  'symbol': /[$&\[\]]/,
  'number': /\b\d+(?:\.\d+)?\b/,
  'operator': /[=+\-*/<>!]/,
  'punctuation': /[(),.:]/
};
export default function EditorPanel({ code, onChange }) {
  const highlightWithPrism = (code) => {
    return Prism.highlight(code, Prism.languages.agartha, 'agartha');
  };

  return (
    <div className="h-full w-full overflow-auto bg-surface relative">
      <div className="min-h-full p-6 pb-20">
        <Editor
          value={code}
          onValueChange={onChange}
          highlight={highlightWithPrism}
          padding={0}
          className="font-mono text-[15px] leading-relaxed w-full outline-none"
          textareaClassName="outline-none"
          style={{
            fontFamily: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace',
            backgroundColor: 'transparent',
            minHeight: '100%',
          }}
        />
      </div>
    </div>
  );
}
