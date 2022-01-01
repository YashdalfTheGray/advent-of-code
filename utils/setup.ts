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

const selected = new SelectedLanguage(year, day);
const subprocess = Deno.run({ cmd: selected.getSetupCommand() });

await subprocess.status();
