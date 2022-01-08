export function getHelpMessage() {
  return `
Usage: setup.ts --year <year> --day <day> [--force-language <language>]

This script sets up an advent of code day given the year and the day. It
also creates a manifest.json that contains a map of the languages to
days that use that particular language. It will also create the folder for
the year and the input file for each day. The solution files start with
boilerplate code for being able to read the input file and they also contain
some tests that can be run with the language appropriate test command.

The script assumes the following directory structure

<project-root>
|-<year>
| |-<day>

If the --force-language argument is provided, it will bootstrap the given
day with the given language. It is provided as a break-glass method to
override a day's language. By default, one at random from the known languages
will be selected and bootstraped. This is to add a bit of variety to each
day. Known languages are listed in the utils/languages/index.ts file.
  `;
}
