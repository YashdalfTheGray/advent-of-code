import Language from './Language.ts';

export default class Rust implements Language {
  public static language = ['rust'];

  private extension = 'rs';
  private solutionFilePath: string;
  private utilsFilePath: string;

  constructor(private year: number, private day: number) {
    this.solutionFilePath = `${this.year}/day${this.day}/src/main.${this.extension}`;
    this.utilsFilePath = `${this.year}/day${this.day}/src/utils.${this.extension}`;
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

  public getPostInstallCommand(): string[] {
    return [];
  }

  public getFileNames(): {
    [key: string]: string;
    inputFile: string;
    solutionFile: string;
  } {
    return {
      inputFile: `${this.year}/day${this.day}/input.txt`,
      solutionFile: this.solutionFilePath,
      utilsFilePath: this.utilsFilePath,
    };
  }

  public getFileContents(path: string): string {
    if (path === this.solutionFilePath) {
      return this.getSolutionFileContents();
    } else if (path === this.utilsFilePath) {
      return this.getUtilsFileContents();
    } else {
      throw new Error(`Unknown path: ${path}`);
    }
  }

  private getSolutionFileContents(): string {
    return `
// This file is generated by the setup script located in the utils folder in the repository root.
// Day ${this.day}
// https://adventofcode.com/${this.year}/day/${this.day}
// input: day${this.day}/input.txt

mod utils;

fn main() {
    let lines = utils::lines_from_file("input.txt").expect("Could not load lines");

    for line in lines {
        println!("{:?}", line);
    }
}
    `
      .trim()
      .concat('\n');
  }

  private getUtilsFileContents(): string {
    return `
// This file is generated by the setup script located in the utils folder in the repository root.
// Run the tests using cargo test

use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lines_from_file() {
        match lines_from_file("input.txt") {
            Ok(lines) => {
                assert_ne!(lines.len(), 0);
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
}
    `
      .trim()
      .concat('\n');
  }
}
