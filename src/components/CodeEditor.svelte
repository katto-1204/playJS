<script lang="ts">
  import { onMount } from 'svelte';
  import { EditorView, basicSetup } from 'codemirror';
  import { EditorState } from '@codemirror/state';
  import { oneDark } from '@codemirror/theme-one-dark';
  import { html } from '@codemirror/lang-html';
  import { css } from '@codemirror/lang-css';
  import { javascript } from '@codemirror/lang-javascript';

  export let value = '';
  export let language: 'html' | 'css' | 'javascript' = 'html';
  export let onChange: (value: string) => void = () => {};

  let editorContainer: HTMLDivElement;
  let editorView: EditorView;

  const languageExtensions = {
    html: html(),
    css: css(),
    javascript: javascript(),
  };

  onMount(() => {
    const startState = EditorState.create({
      doc: value,
      extensions: [
        basicSetup,
        oneDark,
        languageExtensions[language],
        EditorView.updateListener.of((update) => {
          if (update.docChanged) {
            const newValue = update.state.doc.toString();
            onChange(newValue);
          }
        }),
        EditorView.theme({
          '&': {
            height: '100%',
            fontSize: '14px',
            fontFamily: 'Fira Code, JetBrains Mono, monospace',
          },
          '.cm-scroller': {
            overflow: 'auto',
            fontFamily: 'Fira Code, JetBrains Mono, monospace',
          },
          '.cm-content': {
            fontFamily: 'Fira Code, JetBrains Mono, monospace',
          },
        }),
      ],
    });

    editorView = new EditorView({
      state: startState,
      parent: editorContainer,
    });

    return () => {
      editorView.destroy();
    };
  });

  export function setValue(newValue: string) {
    if (editorView) {
      editorView.dispatch({
        changes: {
          from: 0,
          to: editorView.state.doc.length,
          insert: newValue,
        },
      });
    }
  }

  export function getValue(): string {
    return editorView ? editorView.state.doc.toString() : '';
  }
</script>

<div bind:this={editorContainer} class="editor-container"></div>

<style>
  .editor-container {
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

  :global(.cm-editor) {
    height: 100%;
  }

  :global(.cm-scroller) {
    overflow: auto;
  }
</style>
