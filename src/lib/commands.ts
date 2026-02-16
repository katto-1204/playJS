export interface CommandResult {
  output: string;
  success: boolean;
}

export interface Command {
  name: string;
  description: string;
  usage?: string;
  handler: (args: string[]) => CommandResult | Promise<CommandResult>;
}

export class CommandProcessor {
  private commands: Map<string, Command> = new Map();

  constructor() {
    this.registerDefaultCommands();
  }

  private registerDefaultCommands() {
    // Help command
    this.register({
      name: 'help',
      description: 'Display all available commands',
      handler: () => {
        let output = '\x1b[1;36mAvailable Commands:\x1b[0m\n\n';
        this.commands.forEach((cmd) => {
          output += `  \x1b[1;33m${cmd.name}\x1b[0m`;
          if (cmd.usage) {
            output += ` ${cmd.usage}`;
          }
          output += `\n    ${cmd.description}\n\n`;
        });
        return { output, success: true };
      },
    });

    // Clear command
    this.register({
      name: 'clear',
      description: 'Clear the terminal screen',
      handler: () => {
        return { output: '\x1b[2J\x1b[H', success: true };
      },
    });

    // Echo command
    this.register({
      name: 'echo',
      description: 'Print text to the terminal',
      usage: '<text>',
      handler: (args) => {
        const text = args.join(' ');
        return { output: text || '', success: true };
      },
    });

    // Date command
    this.register({
      name: 'date',
      description: 'Display current date and time',
      handler: () => {
        const now = new Date();
        const dateStr = now.toLocaleString('en-US', {
          weekday: 'long',
          year: 'numeric',
          month: 'long',
          day: 'numeric',
          hour: '2-digit',
          minute: '2-digit',
          second: '2-digit',
        });
        return { output: dateStr, success: true };
      },
    });

    // History command
    this.register({
      name: 'history',
      description: 'Show command history',
      handler: () => {
        const history = this.getHistory();
        if (history.length === 0) {
          return { output: 'No commands in history yet.', success: true };
        }
        const output = history
          .map((cmd, index) => `  ${index + 1}  ${cmd}`)
          .join('\n');
        return { output, success: true };
      },
    });
  }

  register(command: Command) {
    this.commands.set(command.name, command);
  }

  async execute(input: string, history: string[] = []): Promise<CommandResult> {
    const parts = input.trim().split(/\s+/);
    const commandName = parts[0].toLowerCase();
    const args = parts.slice(1);

    const command = this.commands.get(commandName);

    if (!command) {
      return {
        output: `\x1b[1;31mCommand not found:\x1b[0m ${commandName}\nType \x1b[1;36mhelp\x1b[0m to see available commands.`,
        success: false,
      };
    }

    try {
      // Special handling for history command
      if (commandName === 'history') {
        const output = history
          .map((cmd, index) => `  ${index + 1}  ${cmd}`)
          .join('\n');
        return { output: output || 'No commands in history yet.', success: true };
      }

      const result = command.handler(args);

      // Check if result is a promise
      if (result instanceof Promise) {
        return await result;
      }

      return result;
    } catch (error) {
      return {
        output: `\x1b[1;31mError executing command:\x1b[0m ${error instanceof Error ? error.message : String(error)}`,
        success: false,
      };
    }
  }

  private getHistory(): string[] {
    return [];
  }

  getCommands(): Command[] {
    return Array.from(this.commands.values());
  }
}
