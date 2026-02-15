<script lang="ts">
  import { onMount } from 'svelte';
  import Terminal from './Terminal.svelte';
  import CodePlayground from './CodePlayground.svelte';
  import LivePreview from './LivePreview.svelte';
  import { CommandProcessor } from '../lib/commands';

  let terminalComponent: any;
  let previewComponent: any;

  let htmlCode = '<!-- Write your HTML here -->\n<div class="container">\n  <h1>Hello, PlayJS!</h1>\n  <p>Start coding and see the magic happen!</p>\n</div>';
  let cssCode = '/* Write your CSS here */\n.container {\n  text-align: center;\n  padding: 2rem;\n}\n\nh1 {\n  color: #FF6B35;\n  font-size: 2.5rem;\n}';
  let jsCode = '// Write your JavaScript here\nconsole.log("Welcome to PlayJS!");';

  const commandProcessor = new CommandProcessor();
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
        const snippet = { html: htmlCode, css: cssCode, js: jsCode };
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

    // Load saved code from localStorage
    const savedCode = localStorage.getItem('playjs-current-code');
    if (savedCode) {
      try {
        const code = JSON.parse(savedCode);
        htmlCode = code.html || htmlCode;
        cssCode = code.css || cssCode;
        jsCode = code.js || jsCode;
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

  function handleCommand(command: string) {
    const result = commandProcessor.execute(
      command,
      terminalComponent?.getHistory() || []
    );

    if (command.toLowerCase() === 'clear') {
      terminalComponent?.clear();
    } else {
      terminalComponent?.writeLine(result.output);
    }
  }

  function handleCodeChange(html: string, css: string, js: string) {
    htmlCode = html;
    cssCode = css;
    jsCode = js;

    // Auto-save to localStorage
    localStorage.setItem(
      'playjs-current-code',
      JSON.stringify({ html, css, js })
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
    {#if !isMobile && view === 'split'}
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
    background-color: #0F111A;
    color: #E0E0E0;
    font-family: 'Inter', sans-serif;
  }

  .header {
    padding: 1rem 2rem;
    background: linear-gradient(135deg, #0F111A 0%, #1a1d29 100%);
    border-bottom: 2px solid #FF6B35;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
  }

  .logo {
    font-size: 1.8rem;
    font-weight: bold;
    margin-bottom: 0.25rem;
  }

  .logo-bracket {
    color: #FF6B35;
  }

  .logo-text {
    color: #E0E0E0;
    margin: 0 0.25rem;
  }

  .tagline {
    font-size: 0.9rem;
    color: #9CA3AF;
    margin-top: 0.25rem;
  }

  .mobile-nav {
    display: flex;
    gap: 0.5rem;
    margin-top: 1rem;
  }

  .nav-btn {
    flex: 1;
    padding: 0.5rem 1rem;
    background-color: transparent;
    border: 1px solid #FF6B35;
    color: #FF6B35;
    border-radius: 4px;
    cursor: pointer;
    font-family: 'Inter', sans-serif;
    font-size: 0.9rem;
    transition: all 0.2s;
  }

  .nav-btn:hover {
    background-color: rgba(255, 107, 53, 0.1);
  }

  .nav-btn.active {
    background-color: #FF6B35;
    color: #0F111A;
  }

  .main-content {
    flex: 1;
    overflow: hidden;
    padding: 1rem;
  }

  .main-content.mobile {
    padding: 0.5rem;
  }

  .split-pane {
    display: flex;
    height: 100%;
    gap: 1rem;
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
    width: 2px;
    background: linear-gradient(to bottom, transparent, #FF6B35, transparent);
    cursor: col-resize;
  }

  .editor-pane {
    flex: 1;
    min-width: 0;
    gap: 1rem;
  }

  .editor-container {
    flex: 1;
    min-height: 0;
  }

  .preview-container {
    flex: 1;
    min-height: 0;
  }

  .mobile-view {
    height: 100%;
  }

  .mobile-editor {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
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
    padding: 0.75rem 2rem;
    background-color: #1a1d29;
    border-top: 1px solid #FF6B35;
    font-size: 0.85rem;
  }

  .shortcuts {
    display: flex;
    gap: 2rem;
    color: #9CA3AF;
  }

  .shortcut-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  kbd {
    padding: 0.125rem 0.375rem;
    background-color: #0F111A;
    border: 1px solid #FF6B35;
    border-radius: 3px;
    font-size: 0.75rem;
    color: #FF6B35;
  }

  @media (max-width: 768px) {
    .header {
      padding: 1rem;
    }

    .logo {
      font-size: 1.4rem;
    }

    .tagline {
      font-size: 0.75rem;
    }

    .footer {
      padding: 0.5rem 1rem;
      font-size: 0.7rem;
    }

    .shortcuts {
      flex-direction: column;
      gap: 0.25rem;
    }
  }
</style>
