<script lang="ts">
  import { onMount } from 'svelte';
  import Terminal from './Terminal.svelte';
  import CodePlayground from './CodePlayground.svelte';
  import LivePreview from './LivePreview.svelte';
  import { CommandProcessor } from '../lib/commands';
  import { RustCompiler } from '../lib/rustCompiler';

  let terminalComponent: any;
  let previewComponent: any;

  let htmlCode = '<!-- Write your HTML here -->\n<div class="container">\n  <h1>Hello, PlayJS!</h1>\n  <p>Start coding and see the magic happen!</p>\n</div>';
  let cssCode = '/* Write your CSS here */\n.container {\n  text-align: center;\n  padding: 2rem;\n}\n\nh1 {\n  color: #FF6B35;\n  font-size: 2.5rem;\n}';
  let jsCode = '// Write your JavaScript here\nconsole.log("Welcome to PlayJS!");';
  let rustCode = '// Write your Rust code here\nfn main() {\n    println!("Hello from Rust!");\n}';

  const commandProcessor = new CommandProcessor();
  const rustCompiler = new RustCompiler();
  let showEditor = false;
  let view: 'terminal' | 'editor' | 'split' = 'split';
  let isMobile = false;

  onMount(() => {
    // Register custom commands
    commandProcessor.register({
      name: 'run',
      description: 'Execute the code in the playground and show output',
      usage: '[preview]',
      handler: (args) => {
        if (args[0] === 'preview' || args.length === 0) {
          if (previewComponent) {
            previewComponent.refresh();
          }
          return {
            output: '\x1b[1;33m✓\x1b[0m Code executed successfully! Check the preview pane.',
            success: true,
          };
        }
        return {
          output: '\x1b[1;33mUsage:\x1b[0m run [preview]',
          success: false,
        };
      },
    });

    commandProcessor.register({
      name: 'editor',
      description: 'Toggle code editor visibility',
      usage: '[show|hide]',
      handler: (args) => {
        if (args.length === 0 || args[0] === 'show') {
          showEditor = true;
          return { output: '\x1b[1;33m✓\x1b[0m Editor opened', success: true };
        } else if (args[0] === 'hide') {
          showEditor = false;
          return { output: '\x1b[1;33m✓\x1b[0m Editor closed', success: true };
        }
        return {
          output: '\x1b[1;33mUsage:\x1b[0m editor [show|hide]',
          success: false,
        };
      },
    });

    commandProcessor.register({
      name: 'save',
      description: 'Save current code to localStorage',
      usage: '<name>',
      handler: (args) => {
        if (args.length === 0) {
          return {
            output: '\x1b[1;31mError:\x1b[0m Please provide a name for the snippet.\n\x1b[1;33mUsage:\x1b[0m save <name>',
            success: false,
          };
        }
        const name = args.join(' ');
        const snippet = { html: htmlCode, css: cssCode, js: jsCode, rust: rustCode };
        localStorage.setItem(`playjs-snippet-${name}`, JSON.stringify(snippet));
        return {
          output: `\x1b[1;33m✓\x1b[0m Snippet "${name}" saved successfully!`,
          success: true,
        };
      },
    });

    commandProcessor.register({
      name: 'load',
      description: 'Load a saved code snippet from localStorage',
      usage: '<name>',
      handler: (args) => {
        if (args.length === 0) {
          return {
            output: '\x1b[1;31mError:\x1b[0m Please provide the name of the snippet to load.\n\x1b[1;33mUsage:\x1b[0m load <name>',
            success: false,
          };
        }
        const name = args.join(' ');
        const saved = localStorage.getItem(`playjs-snippet-${name}`);
        if (!saved) {
          return {
            output: `\x1b[1;31mError:\x1b[0m Snippet "${name}" not found.`,
            success: false,
          };
        }
        try {
          const snippet = JSON.parse(saved);
          htmlCode = snippet.html || '';
          cssCode = snippet.css || '';
          jsCode = snippet.js || '';
          rustCode = snippet.rust || '';
          return {
            output: `\x1b[1;33m✓\x1b[0m Snippet "${name}" loaded successfully!`,
            success: true,
          };
        } catch {
          return {
            output: '\x1b[1;31mError:\x1b[0m Failed to load snippet. Data may be corrupted.',
            success: false,
          };
        }
      },
    });

    commandProcessor.register({
      name: 'snippets',
      description: 'List all saved code snippets',
      handler: () => {
        const snippets: string[] = [];
        for (let i = 0; i < localStorage.length; i++) {
          const key = localStorage.key(i);
          if (key?.startsWith('playjs-snippet-')) {
            snippets.push(key.replace('playjs-snippet-', ''));
          }
        }
        if (snippets.length === 0) {
          return { output: 'No saved snippets found.', success: true };
        }
        const output = '\x1b[1;36mSaved Snippets:\x1b[0m\n\n' + snippets.map((s) => `  • ${s}`).join('\n');
        return { output, success: true };
      },
    });

    commandProcessor.register({
      name: 'rust',
      description: 'Compile and execute Rust code from the Rust editor tab',
      usage: '[run]',
      handler: async (args) => {
        if (!rustCode || rustCode.trim() === '' || rustCode === '// Write your Rust code here\nfn main() {\n    println!("Hello from Rust!");\n}') {
          return {
            output: '\x1b[1;31mError:\x1b[0m No Rust code to execute. Please write some code in the Rust tab.',
            success: false,
          };
        }

        terminalComponent?.writeLine('\x1b[1;36m⟳\x1b[0m Compiling and executing Rust code...\n');

        try {
          const result = await rustCompiler.execute(rustCode);
          return {
            output: rustCompiler.formatExecutionOutput(result),
            success: result.success,
          };
        } catch (error) {
          return {
            output: `\x1b[1;31m✗ Error:\x1b[0m ${error instanceof Error ? error.message : 'Unknown error'}`,
            success: false,
          };
        }
      },
    });

    commandProcessor.register({
      name: 'compile',
      description: 'Compile Rust code (alias for rust command)',
      handler: async () => {
        return await commandProcessor.execute('rust', []);
      },
    });

    // Load saved code from localStorage
    const savedCode = localStorage.getItem('playjs-current-code');
    if (savedCode) {
      try {
        const code = JSON.parse(savedCode);
        htmlCode = code.html || htmlCode;
        cssCode = code.css || cssCode;
        jsCode = code.js || jsCode;
        rustCode = code.rust || rustCode;
      } catch {}
    }

    // Check if mobile
    isMobile = window.innerWidth < 768;
    if (isMobile) {
      view = 'terminal';
    }

    window.addEventListener('resize', () => {
      const wasMobile = isMobile;
      isMobile = window.innerWidth < 768;
      if (isMobile && !wasMobile) {
        view = 'terminal';
      } else if (!isMobile && wasMobile) {
        view = 'split';
      }
    });
  });

  async function handleCommand(command: string) {
    const result = await commandProcessor.execute(
      command,
      terminalComponent?.getHistory() || []
    );

    if (command.toLowerCase() === 'clear') {
      terminalComponent?.clear();
    } else {
      terminalComponent?.writeLine(result.output);
    }
  }

  function handleCodeChange(html: string, css: string, js: string, rust: string) {
    htmlCode = html;
    cssCode = css;
    jsCode = js;
    rustCode = rust;

    // Auto-save to localStorage
    localStorage.setItem(
      'playjs-current-code',
      JSON.stringify({ html, css, js, rust })
    );
  }
</script>

<div class="playjs-app">
  <header class="header">
    <div class="logo">
      <span class="logo-bracket">&lt;</span>
      <span class="logo-text">PlayJS</span>
      <span class="logo-bracket">/&gt;</span>
    </div>
    <div class="tagline">Type. Run. Experiment. All in the Browser.</div>

    <div class="view-toggle">
      <button
        class="toggle-btn"
        class:active={view === 'terminal'}
        on:click={() => (view = 'terminal')}
      >
        Terminal
      </button>
      <button
        class="toggle-btn"
        class:active={view === 'editor'}
        on:click={() => (view = 'editor')}
      >
        Editor
      </button>
      <button
        class="toggle-btn"
        class:active={view === 'split'}
        on:click={() => (view = 'split')}
      >
        Split
      </button>
    </div>

    {#if isMobile}
      <div class="mobile-nav">
        <button
          class="nav-btn"
          class:active={view === 'terminal'}
          on:click={() => (view = 'terminal')}
        >
          Terminal
        </button>
        <button
          class="nav-btn"
          class:active={view === 'editor'}
          on:click={() => (view = 'editor')}
        >
          Editor
        </button>
      </div>
    {/if}
  </header>

  <main class="main-content" class:mobile={isMobile}>
    {#if view === 'split' && !isMobile}
      <div class="split-pane">
        <div class="pane terminal-pane">
          <Terminal bind:this={terminalComponent} onCommand={handleCommand} />
        </div>
        <div class="divider"></div>
        <div class="pane editor-pane">
          <div class="editor-container">
            <CodePlayground
              {htmlCode}
              {cssCode}
              {jsCode}
              {rustCode}
              onCodeChange={handleCodeChange}
            />
          </div>
          <div class="preview-container">
            <LivePreview
              bind:this={previewComponent}
              {htmlCode}
              {cssCode}
              {jsCode}
            />
          </div>
        </div>
      </div>
    {:else if view === 'terminal'}
      <div class="single-view">
        <Terminal bind:this={terminalComponent} onCommand={handleCommand} />
      </div>
    {:else if view === 'editor'}
      <div class="single-view editor-only">
        <div class="editor-container">
          <CodePlayground
            {htmlCode}
            {cssCode}
            {jsCode}
            {rustCode}
            onCodeChange={handleCodeChange}
          />
        </div>
        <div class="preview-container">
          <LivePreview
            bind:this={previewComponent}
            {htmlCode}
            {cssCode}
            {jsCode}
          />
        </div>
      </div>
    {:else if isMobile}
      {#if view === 'terminal'}
        <div class="mobile-view">
          <Terminal bind:this={terminalComponent} onCommand={handleCommand} />
        </div>
      {:else if view === 'editor'}
        <div class="mobile-view mobile-editor">
          <div class="mobile-editor-top">
            <CodePlayground
              {htmlCode}
              {cssCode}
              {jsCode}
              {rustCode}
              onCodeChange={handleCodeChange}
            />
          </div>
          <div class="mobile-editor-bottom">
            <LivePreview
              bind:this={previewComponent}
              {htmlCode}
              {cssCode}
              {jsCode}
            />
          </div>
        </div>
      {/if}
    {/if}
  </main>

  <footer class="footer">
    <div class="shortcuts">
      <span class="shortcut-item"><kbd>Ctrl</kbd> + <kbd>K</kbd> Clear Terminal</span>
      <span class="shortcut-item">Type <kbd>help</kbd> for commands</span>
    </div>
  </footer>
</div>

<style>
  .playjs-app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: radial-gradient(ellipse at top, var(--gradient-from), var(--gradient-via) 50%, var(--gradient-to));
    color: var(--foreground);
    font-family: var(--font-sans);
  }

  .header {
    padding: 1.5rem 2rem;
    background: linear-gradient(135deg, rgba(26, 10, 0, 0.8) 0%, rgba(10, 10, 10, 0.9) 100%);
    backdrop-filter: blur(10px);
    border-bottom: 1px solid var(--border);
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
  }

  .logo {
    font-size: 2rem;
    font-weight: 900;
    margin-bottom: 0.5rem;
    letter-spacing: -0.02em;
    font-family: 'JetBrains Mono', 'Fira Code', 'Courier New', monospace;
  }

  .logo-bracket {
    color: var(--primary);
  }

  .logo-text {
    color: var(--foreground);
    margin: 0 0.25rem;
  }

  .tagline {
    font-size: 0.9375rem;
    color: var(--muted-foreground);
    margin-top: 0.25rem;
    font-weight: 500;
    font-family: 'JetBrains Mono', 'Fira Code', 'Courier New', monospace;
  }

  .view-toggle {
    display: flex;
    gap: 0.5rem;
    margin-top: 1.25rem;
  }

  @media (max-width: 768px) {
    .view-toggle {
      display: none;
    }
  }

  .toggle-btn {
    padding: 0.5rem 1rem;
    background-color: transparent;
    border: 1px solid var(--border);
    color: var(--muted-foreground);
    border-radius: var(--radius);
    cursor: pointer;
    font-family: 'JetBrains Mono', 'Fira Code', 'Courier New', monospace;
    font-size: 0.875rem;
    font-weight: 600;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .toggle-btn:hover {
    background-color: var(--card);
    border-color: var(--primary);
    color: var(--primary);
    transform: translateY(-1px);
  }

  .toggle-btn.active {
    background-color: var(--primary);
    color: white;
    border-color: var(--primary);
    box-shadow: 0 4px 14px 0 rgba(255, 107, 53, 0.4);
  }

  .mobile-nav {
    display: flex;
    gap: 0.75rem;
    margin-top: 1.25rem;
  }

  .nav-btn {
    flex: 1;
    padding: 0.625rem 1.25rem;
    background-color: transparent;
    border: 1px solid var(--border);
    color: var(--muted-foreground);
    border-radius: var(--radius);
    cursor: pointer;
    font-family: 'JetBrains Mono', 'Fira Code', 'Courier New', monospace;
    font-size: 0.9375rem;
    font-weight: 600;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .nav-btn:hover {
    background-color: var(--card);
    border-color: var(--primary);
    color: var(--primary);
    transform: translateY(-1px);
  }

  .nav-btn.active {
    background-color: var(--primary);
    color: white;
    border-color: var(--primary);
    box-shadow: 0 4px 14px 0 rgba(255, 107, 53, 0.4);
  }

  .main-content {
    flex: 1;
    overflow: hidden;
    padding: 1.5rem;
  }

  .main-content.mobile {
    padding: 0.75rem;
  }

  .split-pane {
    display: flex;
    height: 100%;
    gap: 1.5rem;
  }

  .pane {
    display: flex;
    flex-direction: column;
  }

  .terminal-pane {
    flex: 1;
    min-width: 0;
  }

  .divider {
    width: 3px;
    background: linear-gradient(
      to bottom,
      transparent,
      rgba(255, 107, 53, 0.3),
      var(--primary),
      rgba(255, 107, 53, 0.3),
      transparent
    );
    cursor: col-resize;
    border-radius: 9999px;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .divider:hover {
    background: linear-gradient(
      to bottom,
      transparent,
      rgba(255, 107, 53, 0.5),
      var(--primary),
      rgba(255, 107, 53, 0.5),
      transparent
    );
    width: 4px;
    box-shadow: 0 0 20px rgba(255, 107, 53, 0.4);
  }

  .editor-pane {
    flex: 1;
    min-width: 0;
    gap: 1.5rem;
  }

  .editor-container {
    flex: 1;
    min-height: 0;
  }

  .preview-container {
    flex: 1;
    min-height: 0;
  }

  .single-view {
    height: 100%;
    width: 100%;
  }

  .single-view.editor-only {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .single-view.editor-only .editor-container {
    flex: 1;
    min-height: 0;
  }

  .single-view.editor-only .preview-container {
    flex: 1;
    min-height: 0;
  }

  .mobile-view {
    height: 100%;
  }

  .mobile-editor {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .mobile-editor-top {
    flex: 1;
    min-height: 0;
  }

  .mobile-editor-bottom {
    flex: 1;
    min-height: 0;
  }

  .footer {
    padding: 1rem 2rem;
    background: linear-gradient(135deg, rgba(26, 10, 0, 0.6) 0%, rgba(10, 10, 10, 0.8) 100%);
    backdrop-filter: blur(10px);
    border-top: 1px solid var(--border);
    font-size: 0.875rem;
  }

  .shortcuts {
    display: flex;
    gap: 2.5rem;
    color: var(--muted-foreground);
  }

  .shortcut-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-weight: 500;
    font-family: 'JetBrains Mono', 'Fira Code', 'Courier New', monospace;
  }

  kbd {
    padding: 0.25rem 0.5rem;
    background-color: var(--card);
    border: 1px solid var(--border);
    border-radius: 0.375rem;
    font-size: 0.75rem;
    color: var(--primary);
    font-weight: 600;
    font-family: 'JetBrains Mono', 'Fira Code', 'Courier New', monospace;
    box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);
  }

  @media (max-width: 768px) {
    .header {
      padding: 1.25rem 1rem;
    }

    .logo {
      font-size: 1.5rem;
    }

    .tagline {
      font-size: 0.8125rem;
    }

    .main-content {
      padding: 0.75rem;
    }

    .footer {
      padding: 0.75rem 1rem;
      font-size: 0.75rem;
    }

    .shortcuts {
      flex-direction: column;
      gap: 0.5rem;
    }
  }
</style>
