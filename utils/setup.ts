#!/usr/bin/env -S deno run --allow-write --allow-read --allow-run

import { parse } from 'https://deno.land/std/flags/mod.ts';
import { ensureDir, ensureFile } from 'https://deno.land/std/fs/mod.ts';

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
    return allLanguages.find((l) => l.language.includes(forceLanguage));
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

const selected = new SelectedLanguage(year, day);

// run the setup commands
const subprocess = Deno.run({ cmd: selected.getSetupCommand() });
await subprocess.status();

// make sure that the directory and an input file exists
const { inputFile, solutionFile, ...others } = selected.getFileNames();
await ensureDir(selected.getSolutionRootPath());
await ensureFile(inputFile);

// edit or write out the solutions file
const content = selected.getFileContents(solutionFile);
await Deno.writeFile(solutionFile, new TextEncoder().encode(content));

// write out any other files that we may require
Object.values(others).forEach(async (file) => {
  await ensureFile(file);
  Deno.writeFile(
    file,
    new TextEncoder().encode(selected.getFileContents(file))
  );
});
