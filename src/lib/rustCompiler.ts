export interface CompileResult {
  success: boolean;
  output: string;
  error?: string;
  wasm?: Uint8Array;
}

export class RustCompiler {
  private playgroundApiUrl = 'https://play.rust-lang.org';

  /**
   * Compile Rust code to WebAssembly using the Rust Playground API
   */
  async compile(code: string): Promise<CompileResult> {
    try {
      // Prepare the code for WASM compilation
      const wasmCode = this.wrapCodeForWasm(code);

      // Use the Rust Playground API to compile
      const response = await fetch(`${this.playgroundApiUrl}/execute`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          channel: 'stable',
          mode: 'release',
          edition: '2021',
          crateType: 'bin',
          tests: false,
          code: wasmCode,
          backtrace: false,
        }),
      });

      if (!response.ok) {
        throw new Error(`Compilation failed: ${response.statusText}`);
      }

      const result = await response.json();

      if (result.success) {
        return {
          success: true,
          output: result.stdout || 'Compilation successful!',
        };
      } else {
        return {
          success: false,
          output: '',
          error: result.stderr || 'Compilation failed',
        };
      }
    } catch (error) {
      return {
        success: false,
        output: '',
        error: error instanceof Error ? error.message : 'Unknown compilation error',
      };
    }
  }

  /**
   * Execute Rust code directly (without WASM compilation)
   * This uses the Rust Playground to run the code and return output
   */
  async execute(code: string): Promise<CompileResult> {
    try {
      const response = await fetch(`${this.playgroundApiUrl}/execute`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          channel: 'stable',
          mode: 'debug',
          edition: '2021',
          crateType: 'bin',
          tests: false,
          code: code,
          backtrace: false,
        }),
      });

      if (!response.ok) {
        throw new Error(`Execution failed: ${response.statusText}`);
      }

      const result = await response.json();

      return {
        success: result.success || false,
        output: result.stdout || '',
        error: result.stderr || undefined,
      };
    } catch (error) {
      return {
        success: false,
        output: '',
        error: error instanceof Error ? error.message : 'Unknown execution error',
      };
    }
  }

  /**
   * Wrap user code for WASM compilation
   */
  private wrapCodeForWasm(code: string): string {
    // Check if code already has a main function
    if (code.includes('fn main')) {
      return code;
    }

    // Wrap bare code in main function
    return `fn main() {\n${code}\n}`;
  }

  /**
   * Format compiler output for terminal display
   */
  formatOutput(result: CompileResult): string {
    if (result.success) {
      return `\x1b[1;32m✓ Compilation successful!\x1b[0m\n\n${result.output}`;
    } else {
      return `\x1b[1;31m✗ Compilation failed\x1b[0m\n\n${result.error || result.output}`;
    }
  }

  /**
   * Format execution output for terminal display
   */
  formatExecutionOutput(result: CompileResult): string {
    let output = '';

    if (result.success) {
      output += '\x1b[1;32m✓ Program executed successfully!\x1b[0m\n\n';
      if (result.output) {
        output += '\x1b[1;36mOutput:\x1b[0m\n';
        output += result.output;
      }
    } else {
      output += '\x1b[1;31m✗ Execution failed\x1b[0m\n\n';
      if (result.error) {
        output += '\x1b[1;31mError:\x1b[0m\n';
        output += result.error;
      }
      if (result.output) {
        output += '\n\n\x1b[1;36mOutput:\x1b[0m\n';
        output += result.output;
      }
    }

    return output;
  }
}
