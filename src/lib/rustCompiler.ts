export interface CompileResult {
  success: boolean;
  output: string;
  error?: string;
}

export class RustCompiler {
  private playgroundApiUrl = 'https://play.rust-lang.org';

  /**
   * Execute Rust code using the Rust Playground API
   * @param code The Rust source code to compile and execute
   * @returns A promise resolving to the compilation result
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
        return {
          success: false,
          output: '',
          error: `API request failed with status ${response.status}`,
        };
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
        error: error instanceof Error ? error.message : 'Unknown error occurred',
      };
    }
  }

  /**
   * Format the execution output with ANSI color codes for terminal display
   * @param result The compilation result to format
   * @returns Formatted output string with ANSI escape codes
   */
  formatExecutionOutput(result: CompileResult): string {
    if (result.success) {
      const header = '\x1b[1;32m✓ Program executed successfully!\x1b[0m\n';
      const output = result.output ? `\n${result.output}` : '\n(No output)';
      return header + output;
    } else {
      const header = '\x1b[1;31m✗ Execution failed\x1b[0m\n';
      const error = result.error || result.output || '(No error details available)';
      return header + `\n\x1b[31m${error}\x1b[0m`;
    }
  }

  /**
   * Load and execute a Rust example file
   * @param exampleName Name of the example file (without .rs extension)
   * @returns Promise resolving to the loaded code or null if not found
   */
  async loadExample(exampleName: string): Promise<string | null> {
    try {
      const response = await fetch(`/examples/${exampleName}.rs`);
      if (!response.ok) {
        return null;
      }
      return await response.text();
    } catch {
      return null;
    }
  }

  /**
   * Get list of available Rust examples
   * @returns Array of example file names
   */
  getAvailableExamples(): string[] {
    return [
      'hello',
      'sorting',
      'calculator',
      'data_structures',
    ];
  }
}
