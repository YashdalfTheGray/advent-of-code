import Language from './Language.ts';

export default class Rust implements Language {
  public static language = ['rust'];

  private extension = 'rs';
  private solutionFilePath: string;

  constructor(private year: number, private day: number) {
    this.solutionFilePath = `${this.year}/day${this.day}/src/main.${this.extension}`;
  }

  public getSolutionRootPath(): string {
    return `${this.year}/day${this.day}`;
  }
  public getSetupCommand(): string[] {
    return [
      'mkdir',
      '-p',
      this.getSolutionRootPath(),
      '&&',
      'cargo',
      'init',
      '--name',
      `day${this.day}`,
      this.getSolutionRootPath(),
    ];
  }
  public getFileNames(): {
    [key: string]: string;
    inputFile: string;
    solutionFile: string;
  } {
    return {
      inputFile: `${this.year}/day${this.day}/input.txt`,
      solutionFile: this.solutionFilePath,
    };
  }
  public getFileContents(path: string): string {
    if (path === this.solutionFilePath) {
      return this.getSolutionFileContents();
    } else {
      throw new Error(`Unknown path: ${path}`);
    }
  }

  private getSolutionFileContents(): string {
    return `
use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let lines = lines_from_file("input.txt").expect("Could not load lines");

    for line in lines {
        println!("{:?}", line);
    }
}
    `
      .trim()
      .concat('\n');
  }
}