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

    const iframeDoc = iframeElement.contentDocument || iframeElement.contentWindow?.document;
    if (!iframeDoc) return;

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

    iframeDoc.open();
    iframeDoc.write(content);
    iframeDoc.close();
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
    sandbox="allow-scripts allow-same-origin"
  ></iframe>
</div>

<style>
  .preview-wrapper {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    background-color: #0A0A0A;
    border-radius: 0.5rem;
    overflow: hidden;
    border: 1px solid #262626;
    box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px -1px rgba(0, 0, 0, 0.1);
  }

  .preview-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1rem;
    background-color: #151515;
    border-bottom: 1px solid #262626;
  }

  .preview-title {
    font-family: 'Inter', sans-serif;
    font-size: 0.875rem;
    font-weight: 600;
    color: #FAFAFA;
  }

  .refresh-btn {
    background-color: transparent;
    border: 1px solid #262626;
    color: #A1A1A1;
    padding: 0.375rem 0.75rem;
    border-radius: 0.375rem;
    cursor: pointer;
    font-size: 1.125rem;
    transition: all 0.15s cubic-bezier(0.4, 0, 0.2, 1);
    font-weight: 600;
  }

  .refresh-btn:hover {
    background-color: #1A1A1A;
    border-color: #FF6B35;
    color: #FF6B35;
  }

  .preview-iframe {
    flex: 1;
    width: 100%;
    border: none;
    background-color: #ffffff;
  }
</style>
