#!/usr/bin/env -S deno run --allow-write --allow-read

import { parse } from 'https://deno.land/std/flags/mod.ts';
// import { ensureDir, ensureFile } from 'https://deno.land/std/fs/mod.ts';

import allLanguages from './languages/index.ts';

const { year, day, forceLanguage } = parse(Deno.args, {
  string: ['year', 'day', 'part', 'forceLanguage'],
  alias: { forceLanguage: 'force-language' },
});

if (!year || !day) {
  throw new Error('The --year and --day arguments are required.');
}

const SelectedLanguage = (() => {
  if (forceLanguage) {
    return allLanguages.find((language) => language.name === forceLanguage);
  }

  const randomIndex =
    Math.floor(Math.random() * allLanguages.length) % allLanguages.length;
  return allLanguages[randomIndex];
})();

if (!SelectedLanguage) {
  throw new Error(
    'Could not find a supported language that matches the --force-language argument.'
  );
}

console.log(year, day, SelectedLanguage);

// const d${day}p${part}Input = (await Deno.readTextFile('day${day}/input.txt'))
//   .split('\\n')
//   .filter((l) => !!l);
// console.log(d${day}p${part}Input);`;

// // create a directory for the day
// await ensureDir(`./${year}./${year}/day${day}`);

// // create the input file for the day
// await ensureFile(`./${year}/day${day}/input.txt`);

// // create the solution files for the day
// await ensureFile(`./${year}/day${day}/part${part}.ts`);
// await Deno.writeFile(
//   `./day${day}/part${part}.ts`,
//   new TextEncoder().encode(solutionFileContents)
// );
