import Language from './Language.ts';

export default class TypescriptDeno implements Language {
  public static language = ['typescript', 'ts'];

  private extension = 'ts';
  private solutionFilePath: string;
  private vscodeConfigFilePath: string;

  constructor(private year: number, private day: number) {
    this.solutionFilePath = `./${this.year}/day${this.day}/solution.${this.extension}`;
    this.vscodeConfigFilePath = `./${this.year}/day${this.day}/.vscode/settings.json`;
  }

  public getSolutionRootPath(): string {
    return `./${this.year}/day${this.day}`;
  }

  public getSetupCommand(): string[] {
    return ['mkdir', '-p', this.getSolutionRootPath()];
  }

  public getFileNames() {
    return {
      inputFile: `./${this.year}/day${this.day}/input.txt`,
      solutionFile: this.solutionFilePath,
      vscodeConfigFile: this.vscodeConfigFilePath,
    };
  }

  public getFileContents(path: string): string {
    if (path === this.solutionFilePath) {
      return this.getSolutionFileContents();
    } else if (path === this.vscodeConfigFilePath) {
      return this.getVSCodeConfigFileContents();
    } else {
      throw new Error(`Unknown path: ${path}`);
    }
  }

  private getVSCodeConfigFileContents(): string {
    return `
{
  "deno.enable": true,
  "deno.lint": true,
  "deno.unstable": true
}
    `.trim();
  }
  private getSolutionFileContents(): string {
    return `
// This file is generated by the setup script located in the utils folder in the repository root.
// Day ${this.day}
// https://adventofcode.com/${this.year}/day/${this.day}
// input: day${this.day}/input.txt

const d${this.day}Input = (await Deno.readTextFile('day${this.day}/input.txt'))
  .split('\\n')
  .filter((l) => !!l);

console.log(d${this.day}Input);
    `.trim();
  }
}
