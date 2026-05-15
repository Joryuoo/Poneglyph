import React, { useState, useCallback, useEffect } from 'react';
import init, { run_agartha_code } from './pkg/Agartha.js';
import Navbar from './components/Navbar';
import TopBar from './components/TopBar';
import EditorPanel from './components/EditorPanel';
import OutputPanel from './components/OutputPanel';
import ResizableDivider from './components/ResizableDivider';
import SplashScreen from './components/SplashScreen';
import ContentPage from './components/ContentPage';
import Prism from './utils/prism-agartha';
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


const CodeBlock = ({ code }) => {
  const html = Prism.highlight(code, Prism.languages.agartha, 'agartha');
  return (
    <pre className="language-agartha shadow-sm">
      <code className="language-agartha" dangerouslySetInnerHTML={{ __html: html }} />
    </pre>
  );
};

function App() {
  const [code, setCode] = useState(INITIAL_CODE);
  const [currentView, setCurrentView] = useState('playground'); // playground | about | architecture
  const [isOutputVisible, setIsOutputVisible] = useState(true);
  const [editorWidth, setEditorWidth] = useState(50);
  const [showSplash, setShowSplash] = useState(true);
  const [isReady, setIsReady] = useState(false);

  // Interactive terminal state machine
  const [execState, setExecState] = useState('idle'); // idle | running | waiting | done | error
  const [terminalOutput, setTerminalOutput] = useState('');
  const [accumulatedInputs, setAccumulatedInputs] = useState([]);

  useEffect(() => {
    init()
      .then(() => setIsReady(true))
      .catch((err) => {
        console.error("Failed to initialize Wasm:", err);
        setTerminalOutput(`> Error loading Wasm engine:\n${err}`);
      });
  }, []);

  useEffect(() => {
    const timer = setTimeout(() => {
      setShowSplash(false);
    }, 2500);
    return () => clearTimeout(timer);
  }, []);

  // Core execution function — runs WASM with all accumulated inputs
  const executeWithInputs = useCallback((inputs) => {
    setExecState('running');
    setTimeout(() => {
      try {
        const rawResult = run_agartha_code(code, inputs.join('\n'));
        let result;
        try {
          result = JSON.parse(rawResult);
        } catch {
          // Fallback if JSON parsing fails (shouldn't happen)
          setTerminalOutput(rawResult);
          setExecState('done');
          return;
        }

        setTerminalOutput(result.output || '');

        if (result.status === 'scan_wait') {
          setExecState('waiting');
        } else if (result.status === 'error') {
          setExecState('error');
        } else {
          setExecState('done');
        }
      } catch (err) {
        setTerminalOutput(`> Runtime Error:\n${err}`);
        setExecState('error');
      }
    }, 50);
  }, [code]);

  // Execute button handler — fresh execution
  const handleExecute = useCallback(() => {
    setAccumulatedInputs([]);
    setTerminalOutput('');
    setIsOutputVisible(true);
    executeWithInputs([]);
  }, [executeWithInputs]);

  // SCAN input submit handler — append input and re-execute
  const handleScanSubmit = useCallback((input) => {
    const newInputs = [...accumulatedInputs, input];
    setAccumulatedInputs(newInputs);
    executeWithInputs(newInputs);
  }, [accumulatedInputs, executeWithInputs]);

  const handleToggleOutput = () => {
    if (execState !== 'running') {
      setIsOutputVisible(!isOutputVisible);
    }
  };

  const handleResize = useCallback((clientX) => {
    const newWidth = (clientX / window.innerWidth) * 100;
    if (newWidth > 20 && newWidth < 80) {
      setEditorWidth(newWidth);
    }
  }, []);

  const isExecuting = execState === 'running';

  return (
    <div className="h-screen w-screen flex flex-col bg-main text-primary overflow-hidden">
      <AnimatePresence>
        {showSplash && <SplashScreen />}
      </AnimatePresence>

      <Navbar currentView={currentView} setCurrentView={setCurrentView} />

      {currentView === 'playground' ? (
        <div className="flex-1 flex flex-col m-0 sm:m-4 lg:m-8 sm:rounded-2xl lg:rounded-3xl shadow-apple dark:shadow-apple-dark overflow-hidden bg-surface border border-subtle">
          {/* Container simulating a standalone app window if we want, but edge-to-edge is cleaner */}
          
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
              <OutputPanel 
                execState={execState}
                terminalOutput={terminalOutput}
                onScanSubmit={handleScanSubmit}
              />
            </motion.div>

          </div>
        </div>
      ) : currentView === 'about' ? (
        <ContentPage title="About Agartha Poneglyph">
          <p>
            <strong>Agartha Poneglyph</strong> is a web-based implementation of <strong>LEXOR</strong>, a strongly typed educational programming language created for <strong>CS322 – Programming Languages</strong>. It is designed to help students understand the fundamentals of programming language design, including lexical analysis, parsing, abstract syntax trees, runtime execution, type checking, input/output handling, arithmetic evaluation, logical expressions, and control flow.
          </p>

          <p>
            LEXOR follows a strict program structure. Every program must begin with <code>SCRIPT AREA</code>, place executable code inside <code>START SCRIPT</code> and <code>END SCRIPT</code>, and declare variables immediately after <code>START SCRIPT</code> before any executable statement. Each line is treated as a single statement, comments begin with <code>%%</code>, reserved words are written in uppercase, <code>$</code> represents a newline, <code>&amp;</code> is used for concatenation, and square brackets are used as escape codes.
          </p>

          <h3>Basic Syntax</h3>
          <CodeBlock code={`SCRIPT AREA
START SCRIPT
DECLARE INT x = 10
PRINT: "Value: " & x
END SCRIPT`} />

          <h3>Program Structure</h3>
          <CodeBlock code={`SCRIPT AREA
START SCRIPT
%% declarations must come first
DECLARE INT x
DECLARE FLOAT y
DECLARE CHAR c
DECLARE BOOL flag

%% executable statements come after declarations
x = 10
PRINT: x
END SCRIPT`} />

          <h3>Data Types</h3>
          <p>LEXOR supports four declared data types:</p>
          <table>
            <thead>
              <tr>
                <th>Data Type</th>
                <th>Description</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td><code>INT</code></td>
                <td>Whole number with no decimal part</td>
              </tr>
              <tr>
                <td><code>FLOAT</code></td>
                <td>Number with a decimal part</td>
              </tr>
              <tr>
                <td><code>CHAR</code></td>
                <td>A single symbol or character</td>
              </tr>
              <tr>
                <td><code>BOOL</code></td>
                <td>Boolean value represented by <code>TRUE</code> or <code>FALSE</code></td>
              </tr>
            </tbody>
          </table>
          <p>
            Although string literals can be used in output and concatenation, <code>STRING</code> is not treated as a valid declared data type in the refactored interpreter. The official LEXOR data type list contains <code>INT</code>, <code>CHAR</code>, <code>BOOL</code>, and <code>FLOAT</code>.
          </p>

          <h3>Reserved Keywords</h3>
          <p>Common LEXOR reserved keywords include:</p>
          <CodeBlock code={`SCRIPT AREA
START SCRIPT
END SCRIPT
DECLARE
INT
FLOAT
CHAR
BOOL
PRINT
SCAN
IF
ELSE IF
ELSE
START IF
END IF
FOR
START FOR
END FOR
REPEAT WHEN
START REPEAT
END REPEAT
AND
OR
NOT
TRUE
FALSE`} />

          <h3>Operators</h3>
          <p>LEXOR supports arithmetic, relational, logical, unary, and concatenation operators.</p>
          <table>
            <thead>
              <tr>
                <th>Category</th>
                <th>Operators</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>Arithmetic</td>
                <td><code>+</code>, <code>-</code>, <code>*</code>, <code>/</code>, <code>%</code></td>
              </tr>
              <tr>
                <td>Relational</td>
                <td><code>&gt;</code>, <code>&lt;</code>, <code>&gt;=</code>, <code>&lt;=</code>, <code>==</code>, <code>&lt;&gt;</code></td>
              </tr>
              <tr>
                <td>Logical</td>
                <td><code>AND</code>, <code>OR</code>, <code>NOT</code></td>
              </tr>
              <tr>
                <td>Unary</td>
                <td><code>+</code>, <code>-</code></td>
              </tr>
              <tr>
                <td>Concatenation</td>
                <td><code>&amp;</code></td>
              </tr>
              <tr>
                <td>Newline</td>
                <td><code>$</code></td>
              </tr>
            </tbody>
          </table>
          <p>
            The language specification defines arithmetic operators, comparison operators, logical operators, and unary operators for positive and negative values.
          </p>

          <h3>Input and Output</h3>
          <p>Output is handled using <code>PRINT:</code>.</p>
          <CodeBlock code={`PRINT: "Hello, LEXOR"
PRINT: "Age: " & 20
PRINT: "Line 1" & $ & "Line 2"`} />

          <p>Input is handled using <code>SCAN:</code>.</p>
          <CodeBlock code={`DECLARE INT x
DECLARE INT y
SCAN: x, y
PRINT: x & "," & y`} />
          <p>
            <code>SCAN</code> accepts one or more variables separated by commas, and the user must provide input values in the same order.
          </p>

          <h3>Control Flow</h3>
          <p>LEXOR supports conditional and looping structures.</p>

          <h4>Conditional Statement</h4>
          <CodeBlock code={`IF (x > 10)
START IF
PRINT: "Greater"
END IF
ELSE
START IF
PRINT: "Not greater"
END IF`} />

          <h4>FOR Loop</h4>
          <CodeBlock code={`FOR (x = 0, x < 5, x = x + 1)
START FOR
PRINT: x
END FOR`} />

          <h4>REPEAT WHEN Loop</h4>
          <CodeBlock code={`REPEAT WHEN (x < 5)
START REPEAT
PRINT: x
x = x + 1
END REPEAT`} />
          <p>
            The specification includes <code>IF</code>, <code>ELSE IF</code>, <code>ELSE</code>, <code>FOR</code>, and <code>REPEAT WHEN</code> as the main control flow structures.
          </p>

          <h3>Purpose</h3>
          <p>
            Poneglyph was built as both an interpreter and a learning tool. It allows users to write and run LEXOR programs directly in the browser while demonstrating how a programming language moves from source code to tokens, from tokens to an Abstract Syntax Tree, and from the AST to runtime execution.
          </p>
        </ContentPage>
      ) : (
        <ContentPage title="LEXOR Interpreter Architecture">
          <h3>Project Overview</h3>
          <p>
            The <strong>Agartha Poneglyph Interpreter</strong> is a Rust-based implementation of the LEXOR programming language for <strong>CS322 – Programming Languages</strong>. It follows a traditional interpreter pipeline where source code is processed through lexical analysis, syntax analysis, AST construction, and runtime execution.
          </p>
          <p>
            The interpreter is designed around a tree-walk architecture. Instead of compiling LEXOR into machine code, the system directly executes the parsed Abstract Syntax Tree while maintaining runtime memory for variables, values, and declared types.
          </p>

          <h3>System Pipeline</h3>
          <CodeBlock code={`Source Code
    ↓
Lexer
    ↓
Tokens
    ↓
Parser
    ↓
Abstract Syntax Tree
    ↓
Interpreter
    ↓
Runtime Output`} />

          <h3>1. Source Code</h3>
          <p>
            The source code is the raw LEXOR program written by the user in the editor. It follows the required LEXOR structure:
          </p>
          <CodeBlock code={`SCRIPT AREA
START SCRIPT
DECLARE INT x = 10
PRINT: x
END SCRIPT`} />
          <p>
            The source code is passed into the lexer as plain text.
          </p>

          <h3>2. Lexer</h3>
          <p>
            The lexer reads the raw source code character by character and converts it into tokens. Tokens represent meaningful pieces of the language, such as keywords, identifiers, literals, operators, and symbols.
          </p>
          <p>
            Examples of tokens include:
          </p>
          <CodeBlock code={`DECLARE
INT
Identifier(x)
Assign
IntLiteral(10)
PRINT
Colon
END SCRIPT`} />
          <p>
            The lexer is responsible for recognizing:
          </p>
          <CodeBlock code={`- reserved words
- identifiers
- comments
- integer literals
- float literals
- string literals
- character literals
- boolean literals
- arithmetic operators
- relational operators
- logical operators
- delimiters
- special symbols such as $, &, and brackets`} />

          <h3>3. Parser</h3>
          <p>
            The parser receives the token stream and checks whether the program follows valid LEXOR grammar. It enforces required structure such as <code>SCRIPT AREA</code>, <code>START SCRIPT</code>, and <code>END SCRIPT</code>.
          </p>
          <p>
            The parser also prevents invalid statement placement, such as declarations appearing after executable statements. It enforces the one-statement-per-line rule and builds structured AST nodes from the token stream.
          </p>
          <p>
            Common AST statement nodes include:
          </p>
          <CodeBlock code={`Declaration
Assignment
Print
Scan
If
For
Repeat`} />
          <p>
            Common AST expression nodes include:
          </p>
          <CodeBlock code={`IntType
FloatType
CharType
BoolType
StringType
Identifier
BinaryOp
UnaryOp`} />
          <p>
            The refactored parser limits declared variable types to valid LEXOR types:
          </p>
          <CodeBlock code={`INT
FLOAT
CHAR
BOOL`} />
          <p>
            String literals are still valid expressions, but <code>STRING</code> is not accepted as a declared data type.
          </p>

          <h3>4. Abstract Syntax Tree</h3>
          <p>
            The Abstract Syntax Tree, or AST, is the internal representation of the program. It organizes the source code into structured nodes that the interpreter can execute.
          </p>
          <p>
            The AST separates expressions from statements. Expressions represent values or computations, such as integers, floats, booleans, identifiers, binary operations, and unary operations. Statements represent executable actions, such as declarations, assignments, print statements, scan statements, conditionals, and loops.
          </p>
          <p>
            Example AST concept:
          </p>
          <CodeBlock code={`Print(
    BinaryOp(
        StringLiteral("Value: "),
        Concat,
        Identifier("x")
    )
)`} />

          <h3>5. Interpreter</h3>
          <p>
            The interpreter walks through the AST and executes each statement from top to bottom. It maintains runtime memory, evaluates expressions, performs type checking, handles input/output, and executes control flow.
          </p>
          <p>
            In the refactored interpreter, runtime memory stores both the declared type and the current value of each variable. This prevents invalid type mutation after declaration and makes declarations, assignments, and input validation more reliable.
          </p>
          <p>
            Example runtime memory concept:
          </p>
          <CodeBlock code={`x → declared type: INT, value: 10
f → declared type: FLOAT, value: 67.0
b → declared type: BOOL, value: TRUE`} />

          <h3>6. Runtime Output</h3>
          <p>
            For the web version, output is collected into a frontend-facing output buffer instead of being printed directly to the terminal. Runtime errors are also appended to the output stream, making the interpreter suitable for browser execution through WebAssembly.
          </p>

          <h3>WebAssembly Integration</h3>
          <p>
            The web version of Poneglyph uses Rust as the interpreter core and exposes execution results to the browser through WebAssembly. This allows LEXOR programs to run directly in the frontend while keeping the interpreter logic written in Rust.
          </p>
          <p>
            The frontend execution model is different from a CLI interpreter because browser-based programs cannot rely on normal terminal input and output. Instead, the interpreter stores printed output in an internal output buffer and handles <code>SCAN</code> input through pre-provided frontend input values.
          </p>

          <h3>Runtime Responsibilities</h3>
          <p>
            The interpreter is responsible for:
          </p>
          <CodeBlock code={`- executing declarations
- assigning values to variables
- validating declared types
- evaluating arithmetic expressions
- evaluating logical expressions
- evaluating relational expressions
- handling unary operations
- executing PRINT statements
- handling SCAN input
- executing IF, ELSE IF, and ELSE blocks
- executing FOR loops
- executing REPEAT WHEN loops
- reporting syntax and runtime errors`} />

          <h3>Type System Responsibilities</h3>
          <p>
            The refactored runtime type system improves semantic correctness by enforcing declared variable types during declaration, assignment, and input.
          </p>
          <p>
            Important type rules include:
          </p>
          <CodeBlock code={`- INT accepts integer values only
- FLOAT accepts float values and can promote INT values to FLOAT
- CHAR accepts one character only
- BOOL accepts only TRUE or FALSE
- BOOL values are case-sensitive
- STRING cannot be declared as a variable type
- string literals are allowed in PRINT and concatenation
- numeric operations are allowed between INT and FLOAT
- non-numeric values cannot be used in arithmetic operations`} />

          <h3>Design Goal</h3>
          <p>
            The main goal of this architecture is to make the LEXOR interpreter easy to understand, test, and extend. Each stage has a clear responsibility: the lexer recognizes tokens, the parser validates grammar and builds the AST, and the interpreter executes the AST.
          </p>
          <p>
            This separation reflects the standard structure of many real programming language implementations while keeping the project simple enough for educational use.
          </p>
        </ContentPage>
      )}
    </div>
  );
}

export default App;
