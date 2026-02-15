<script lang="ts">
  import { onMount } from 'svelte';

  export let htmlCode = '';
  export let cssCode = '';
  export let jsCode = '';

  let iframeElement: HTMLIFrameElement;

  onMount(() => {
    updatePreview();
  });

  $: if (iframeElement && (htmlCode || cssCode || jsCode)) {
    updatePreview();
  }

  function updatePreview() {
    if (!iframeElement) return;

    const document = iframeElement.contentDocument;
    if (!document) return;

    const content = `
      <!DOCTYPE html>
      <html lang="en">
        <head>
          <meta charset="UTF-8">
          <meta name="viewport" content="width=device-width, initial-scale=1.0">
          <style>
            * {
              margin: 0;
              padding: 0;
              box-sizing: border-box;
            }
            body {
              font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
              padding: 1rem;
              background-color: #ffffff;
              color: #000000;
            }
            ${cssCode}
          </style>
        </head>
        <body>
          ${htmlCode}
          <script>
            // Console intercept for showing logs in the preview
            (function() {
              const originalLog = console.log;
              const originalError = console.error;
              const originalWarn = console.warn;

              console.log = function(...args) {
                originalLog.apply(console, args);
                window.parent.postMessage({ type: 'console', level: 'log', message: args }, '*');
              };

              console.error = function(...args) {
                originalError.apply(console, args);
                window.parent.postMessage({ type: 'console', level: 'error', message: args }, '*');
              };

              console.warn = function(...args) {
                originalWarn.apply(console, args);
                window.parent.postMessage({ type: 'console', level: 'warn', message: args }, '*');
              };

              window.addEventListener('error', function(e) {
                window.parent.postMessage({
                  type: 'console',
                  level: 'error',
                  message: [e.message + ' at ' + e.filename + ':' + e.lineno + ':' + e.colno]
                }, '*');
              });
            })();

            try {
              ${jsCode}
            } catch (error) {
              console.error('Runtime Error:', error.message);
            }
          <\/script>
        </body>
      </html>
    `;

    document.open();
    document.write(content);
    document.close();
  }

  export function refresh() {
    updatePreview();
  }
</script>

<div class="preview-wrapper">
  <div class="preview-header">
    <span class="preview-title">Preview</span>
    <button class="refresh-btn" on:click={updatePreview} title="Refresh preview">
      ↻
    </button>
  </div>
  <iframe
    bind:this={iframeElement}
    class="preview-iframe"
    title="Code Preview"
    sandbox="allow-scripts"
  ></iframe>
</div>

<style>
  .preview-wrapper {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    background-color: #0F111A;
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
  }

  .preview-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1rem;
    background-color: #1a1d29;
    border-bottom: 2px solid #00FFA3;
  }

  .preview-title {
    font-family: 'Fira Code', 'JetBrains Mono', monospace;
    font-size: 0.9rem;
    font-weight: 600;
    color: #00FFA3;
  }

  .refresh-btn {
    background-color: transparent;
    border: 1px solid #00FFA3;
    color: #00FFA3;
    padding: 0.25rem 0.75rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 1.2rem;
    transition: all 0.2s;
  }

  .refresh-btn:hover {
    background-color: rgba(0, 255, 163, 0.1);
  }

  .preview-iframe {
    flex: 1;
    width: 100%;
    border: none;
    background-color: #ffffff;
  }
</style>
