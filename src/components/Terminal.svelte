<script lang="ts">
  import { onMount } from 'svelte';
  import { Terminal } from '@xterm/xterm';
  import { FitAddon } from '@xterm/addon-fit';
  import '@xterm/xterm/css/xterm.css';

  export let onCommand: (command: string) => void = () => {};

  let terminalContainer: HTMLDivElement;
  let terminal: Terminal;
  let fitAddon: FitAddon;
  let commandHistory: string[] = [];
  let historyIndex = -1;
  let currentLine = '';

  onMount(() => {
    terminal = new Terminal({
      cursorBlink: true,
      cursorStyle: 'block',
      fontFamily: 'Inter, -apple-system, BlinkMacSystemFont, sans-serif',
      fontSize: 15,
      fontWeight: '800',
      fontWeightBold: '900',
      theme: {
        background: '#0D0F16',
        foreground: '#E0E0E0',
        cursor: '#FF6B35',
        cursorAccent: '#FF6B35',
        selectionBackground: 'rgba(255, 107, 53, 0.3)',
        black: '#0F111A',
        red: '#FF6B6B',
        green: '#FF6B35',
        yellow: '#FFD93D',
        blue: '#6BCF7F',
        magenta: '#C084FC',
        cyan: '#22D3EE',
        white: '#E0E0E0',
        brightBlack: '#4B5563',
        brightRed: '#FCA5A5',
        brightGreen: '#FF8C42',
        brightYellow: '#FDE047',
        brightBlue: '#93C5FD',
        brightMagenta: '#E9D5FF',
        brightCyan: '#A5F3FC',
        brightWhite: '#F9FAFB',
      },
      allowProposedApi: true,
    });

    fitAddon = new FitAddon();
    terminal.loadAddon(fitAddon);
    terminal.open(terminalContainer);
    fitAddon.fit();

    // Welcome message
    terminal.writeln('\x1b[1;33m╔══════════════════════════════════════════════════════════╗\x1b[0m');
    terminal.writeln('\x1b[1;33m║                                                          ║\x1b[0m');
    terminal.writeln('\x1b[1;33m║               Welcome to PlayJS Terminal!                ║\x1b[0m');
    terminal.writeln('\x1b[1;33m║                                                          ║\x1b[0m');
    terminal.writeln('\x1b[1;33m║           Type. Run. Experiment. In Browser.             ║\x1b[0m');
    terminal.writeln('\x1b[1;33m║                                                          ║\x1b[0m');
    terminal.writeln('\x1b[1;33m╚══════════════════════════════════════════════════════════╝\x1b[0m');
    terminal.writeln('');
    terminal.writeln('Type \x1b[1;36mhelp\x1b[0m to see available commands.');
    terminal.writeln('');
    prompt();

    // Handle input
    terminal.onData((data) => {
      const code = data.charCodeAt(0);

      if (code === 13) { // Enter
        terminal.writeln('');
        const command = currentLine.trim();
        if (command) {
          commandHistory.push(command);
          historyIndex = commandHistory.length;
          onCommand(command);
        }
        currentLine = '';
        prompt();
      } else if (code === 127) { // Backspace
        if (currentLine.length > 0) {
          currentLine = currentLine.slice(0, -1);
          terminal.write('\b \b');
        }
      } else if (code === 27) { // Escape sequences (arrow keys)
        if (data === '\x1b[A') { // Up arrow
          if (historyIndex > 0) {
            clearCurrentLine();
            historyIndex--;
            currentLine = commandHistory[historyIndex];
            terminal.write(currentLine);
          }
        } else if (data === '\x1b[B') { // Down arrow
          if (historyIndex < commandHistory.length - 1) {
            clearCurrentLine();
            historyIndex++;
            currentLine = commandHistory[historyIndex];
            terminal.write(currentLine);
          } else if (historyIndex === commandHistory.length - 1) {
            clearCurrentLine();
            historyIndex = commandHistory.length;
            currentLine = '';
          }
        }
      } else if (code >= 32) { // Printable characters
        currentLine += data;
        terminal.write(data);
      }
    });

    // Handle resize
    const resizeObserver = new ResizeObserver(() => {
      fitAddon.fit();
    });
    resizeObserver.observe(terminalContainer);

    return () => {
      resizeObserver.disconnect();
      terminal.dispose();
    };
  });

  function prompt() {
    terminal.write('\x1b[1;33m$\x1b[0m ');
  }

  function clearCurrentLine() {
    terminal.write('\r\x1b[K');
    prompt();
  }

  export function writeLine(text: string) {
    if (terminal) {
      terminal.writeln(text);
    }
  }

  export function write(text: string) {
    if (terminal) {
      terminal.write(text);
    }
  }

  export function clear() {
    if (terminal) {
      terminal.clear();
    }
  }

  export function getHistory() {
    return commandHistory;
  }
</script>

<div class="terminal-wrapper">
  <div bind:this={terminalContainer} class="terminal-container"></div>
</div>

<style>
  .terminal-wrapper {
    width: 100%;
    height: 100%;
    background-color: #0D0F16;
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
  }

  .terminal-container {
    width: 100%;
    height: 100%;
    padding: 1rem;
  }

  :global(.xterm) {
    height: 100%;
  }

  :global(.xterm-viewport) {
    overflow-y: auto !important;
  }
</style>
