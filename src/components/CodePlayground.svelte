<script lang="ts">
  import CodeEditor from './CodeEditor.svelte';

  export let htmlCode = '';
  export let cssCode = '';
  export let jsCode = '';
  export let onCodeChange: (html: string, css: string, js: string) => void = () => {};

  type Tab = 'html' | 'css' | 'javascript';
  let activeTab: Tab = 'html';

  function handleHtmlChange(value: string) {
    htmlCode = value;
    onCodeChange(htmlCode, cssCode, jsCode);
  }

  function handleCssChange(value: string) {
    cssCode = value;
    onCodeChange(htmlCode, cssCode, jsCode);
  }

  function handleJsChange(value: string) {
    jsCode = value;
    onCodeChange(htmlCode, cssCode, jsCode);
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
    {/if}
  </div>
</div>

<style>
  .code-playground {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    background-color: #0F111A;
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
  }

  .tabs {
    display: flex;
    background-color: #1a1d29;
    border-bottom: 2px solid #00FFA3;
    padding: 0.5rem 1rem 0 1rem;
    gap: 0.5rem;
  }

  .tab {
    padding: 0.5rem 1.5rem;
    background-color: transparent;
    border: none;
    color: #9CA3AF;
    font-family: 'Fira Code', 'JetBrains Mono', monospace;
    font-size: 0.9rem;
    font-weight: 500;
    cursor: pointer;
    border-radius: 6px 6px 0 0;
    transition: all 0.2s;
    position: relative;
  }

  .tab:hover {
    background-color: rgba(0, 255, 163, 0.1);
    color: #E0E0E0;
  }

  .tab.active {
    background-color: #0F111A;
    color: #00FFA3;
  }

  .tab.active::after {
    content: '';
    position: absolute;
    bottom: -2px;
    left: 0;
    right: 0;
    height: 2px;
    background-color: #00FFA3;
  }

  .editor-area {
    flex: 1;
    overflow: hidden;
  }
</style>
