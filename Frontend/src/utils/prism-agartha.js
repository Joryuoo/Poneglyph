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

export default Prism;
