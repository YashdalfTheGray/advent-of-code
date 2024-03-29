#!/usr/bin/env -S deno run --allow-write --allow-read --allow-run

import { parse } from 'https://deno.land/std@0.123.0/flags/mod.ts';
import { ensureDir, ensureFile } from 'https://deno.land/std@0.123.0/fs/mod.ts';

import allLanguages from './languages/index.ts';
import { getManifestLocation, addDayToManifest } from './manifest.ts';
import { getHelpMessage } from './help.ts';

const runCommands = (commands: string[]) =>
  Promise.all(
    commands
      .join(' ')
      .split('&&')
      .map((c) => c.trim().split(' '))
      .map((cmd) => Deno.run({ cmd }))
      .map((p) => p.status())
  );

const { yearStr, dayStr, forceLanguage, help } = parse(Deno.args, {
  string: ['year', 'day', 'part', 'forceLanguage'],
  alias: {
    help: 'h',
    yearStr: ['year', 'y'],
    dayStr: ['day', 'd'],
    forceLanguage: ['force-language', 'f'],
  },
});

if (help) {
  if (yearStr || dayStr || forceLanguage) {
    console.warn('WARN - Help flag detected, ignoring other flag');
  }
  console.log(getHelpMessage());
  Deno.exit(0);
}

const year = Number(yearStr);
const day = Number(dayStr);

if (!year || !day || isNaN(year) || isNaN(day)) {
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

// build out the year folder and create a manifest.json file
await ensureDir(`./${year}`);
await ensureFile(getManifestLocation(year));

const selected = new SelectedLanguage(year, day);

// run the setup commands
await runCommands(selected.getSetupCommand());

// make sure that the directory and an input file exists
const { inputFile, solutionFile, ...others } = selected.getFileNames();
await ensureDir(selected.getSolutionRootPath());
await ensureFile(inputFile);

// edit or write out the solutions file
const content = selected.getFileContents(solutionFile);
await Deno.writeTextFile(solutionFile, content);

// write out any other files that we may require
Object.values(others).forEach(async (file) => {
  await ensureFile(file);
  await Deno.writeTextFile(file, selected.getFileContents(file));
});

// run any post install commands
if (selected.getPostInstallCommand().length > 0) {
  await runCommands(selected.getPostInstallCommand());
}

// read and append to the manifest file
const manifestStr = await Deno.readTextFile(`./${year}/manifest.json`);
const manifest = (() => {
  try {
    return JSON.parse(manifestStr);
  } catch (_) {
    return {};
  }
})();

const updatedManifest = addDayToManifest(
  day,
  SelectedLanguage.language[0],
  manifest
);

await Deno.writeTextFile(
  getManifestLocation(year),
  JSON.stringify(updatedManifest, null, 2)
);
