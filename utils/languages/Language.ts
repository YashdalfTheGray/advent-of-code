export default abstract class Language {
  public static language: string[];

  public abstract getSolutionRootPath(): string;
  public abstract getSetupCommand(): string[];
  public abstract getPostInstallCommand(): string[];
  public abstract getFileNames(): {
    inputFile: string;
    solutionFile: string;
    [key: string]: string;
  };
  public abstract getFileContents(path: string): string;
}
