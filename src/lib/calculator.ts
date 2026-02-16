export interface CalculationResult {
  result: number;
  expression: string;
  success: boolean;
  error?: string;
}

export class Calculator {
  private memory: number = 0;
  private history: string[] = [];
  private maxHistory: number = 50;

  /**
   * Evaluate a mathematical expression
   * Supports: +, -, *, /, ^, %, (, )
   */
  calculate(expression: string): CalculationResult {
    try {
      const sanitized = this.sanitizeExpression(expression);
      const result = this.evaluateExpression(sanitized);

      if (!isFinite(result)) {
        return {
          result: 0,
          expression,
          success: false,
          error: 'Result is not finite (overflow or division by zero)',
        };
      }

      const historyEntry = `${expression} = ${result}`;
      this.addToHistory(historyEntry);
      this.memory = result;

      return {
        result,
        expression,
        success: true,
      };
    } catch (error) {
      return {
        result: 0,
        expression,
        success: false,
        error: error instanceof Error ? error.message : 'Invalid expression',
      };
    }
  }

  /**
   * Sanitize and prepare expression for evaluation
   */
  private sanitizeExpression(expr: string): string {
    // Remove whitespace
    let sanitized = expr.replace(/\s+/g, '');

    // Replace ^ with ** for exponentiation
    sanitized = sanitized.replace(/\^/g, '**');

    // Validate characters (only allow numbers, operators, parentheses, decimal point)
    if (!/^[0-9+\-*/()\\.%]+$/.test(sanitized)) {
      throw new Error('Invalid characters in expression');
    }

    return sanitized;
  }

  /**
   * Evaluate the sanitized expression
   */
  private evaluateExpression(expr: string): number {
    // Use Function constructor for safe evaluation
    // This is safer than eval() but still requires sanitized input
    try {
      const func = new Function('return ' + expr);
      const result = func();

      if (typeof result !== 'number') {
        throw new Error('Expression did not evaluate to a number');
      }

      return result;
    } catch {
      throw new Error('Failed to evaluate expression');
    }
  }

  /**
   * Add entry to calculation history
   */
  private addToHistory(entry: string): void {
    this.history.push(entry);
    if (this.history.length > this.maxHistory) {
      this.history.shift();
    }
  }

  /**
   * Get calculation history
   */
  getHistory(): string[] {
    return [...this.history];
  }

  /**
   * Get current memory value
   */
  getMemory(): number {
    return this.memory;
  }

  /**
   * Set memory value
   */
  setMemory(value: number): void {
    this.memory = value;
  }

  /**
   * Clear memory
   */
  clearMemory(): void {
    this.memory = 0;
  }

  /**
   * Clear history
   */
  clearHistory(): void {
    this.history = [];
  }

  /**
   * Clear everything
   */
  clearAll(): void {
    this.memory = 0;
    this.history = [];
  }

  /**
   * Format result for display
   */
  formatResult(result: CalculationResult): string {
    if (!result.success) {
      return `\x1b[1;31m✗ Error:\x1b[0m ${result.error}`;
    }

    const formattedResult = Number.isInteger(result.result)
      ? result.result.toString()
      : result.result.toFixed(6).replace(/\.?0+$/, '');

    return `\x1b[1;33m${result.expression}\x1b[0m = \x1b[1;32m${formattedResult}\x1b[0m`;
  }

  /**
   * Format history for display
   */
  formatHistory(): string {
    if (this.history.length === 0) {
      return '\x1b[2mNo calculation history.\x1b[0m';
    }

    const header = '\x1b[1;36m=== Calculation History ===\x1b[0m\n\n';
    const entries = this.history
      .map((entry, index) => `  \x1b[2m${index + 1}.\x1b[0m ${entry}`)
      .join('\n');

    return header + entries;
  }
}
