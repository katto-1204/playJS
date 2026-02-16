<script lang="ts">
  import CodeEditor from './CodeEditor.svelte';

  export let htmlCode = '';
  export let cssCode = '';
  export let jsCode = '';
  export let rustCode = '';
  export let onCodeChange: (html: string, css: string, js: string, rust: string) => void = () => {};

  type Tab = 'html' | 'css' | 'javascript' | 'rust';
  let activeTab: Tab = 'html';

  function handleHtmlChange(value: string) {
    htmlCode = value;
    onCodeChange(htmlCode, cssCode, jsCode, rustCode);
  }

  function handleCssChange(value: string) {
    cssCode = value;
    onCodeChange(htmlCode, cssCode, jsCode, rustCode);
  }

  function handleJsChange(value: string) {
    jsCode = value;
    onCodeChange(htmlCode, cssCode, jsCode, rustCode);
  }

  function handleRustChange(value: string) {
    rustCode = value;
    onCodeChange(htmlCode, cssCode, jsCode, rustCode);
  }
</script>

<div class="code-playground">
  <div class="tabs">
    <button
      class="tab"
      class:active={activeTab === 'html'}
      on:click={() => (activeTab = 'html')}
    >
      HTML
    </button>
    <button
      class="tab"
      class:active={activeTab === 'css'}
      on:click={() => (activeTab = 'css')}
    >
      CSS
    </button>
    <button
      class="tab"
      class:active={activeTab === 'javascript'}
      on:click={() => (activeTab = 'javascript')}
    >
      JavaScript
    </button>
    <button
      class="tab"
      class:active={activeTab === 'rust'}
      on:click={() => (activeTab = 'rust')}
    >
      Rust/WASM
    </button>
  </div>

  <div class="editor-area">
    {#if activeTab === 'html'}
      <CodeEditor
        value={htmlCode}
        language="html"
        onChange={handleHtmlChange}
      />
    {:else if activeTab === 'css'}
      <CodeEditor
        value={cssCode}
        language="css"
        onChange={handleCssChange}
      />
    {:else if activeTab === 'javascript'}
      <CodeEditor
        value={jsCode}
        language="javascript"
        onChange={handleJsChange}
      />
    {:else if activeTab === 'rust'}
      <CodeEditor
        value={rustCode}
        language="rust"
        onChange={handleRustChange}
      />
    {/if}
  </div>
</div>

<style>
  .code-playground {
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

  .tabs {
    display: flex;
    background-color: #151515;
    border-bottom: 1px solid #262626;
    padding: 0 1rem;
    gap: 0.25rem;
  }

  .tab {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    background-color: transparent;
    border: none;
    color: #A1A1A1;
    font-family: 'Inter', sans-serif;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    border-radius: 0;
    transition: all 0.15s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    border-bottom: 2px solid transparent;
  }

  .tab-icon {
    font-size: 1rem;
  }

  .tab:hover {
    background-color: #1A1A1A;
    color: #FAFAFA;
  }

  .tab.active {
    background-color: transparent;
    color: #FF6B35;
    border-bottom-color: #FF6B35;
  }

  .editor-area {
    flex: 1;
    overflow: hidden;
  }
</style>
