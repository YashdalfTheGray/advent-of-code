#!/usr/bin/env -S deno run --allow-write --allow-read --allow-run

import { parse } from 'https://deno.land/std/flags/mod.ts';
import { ensureDir, ensureFile } from 'https://deno.land/std/fs/mod.ts';

import allLanguages from './languages/index.ts';
import { getManifestLocation, addDayToManifest } from './manifest.ts';

const { yearStr, dayStr, forceLanguage } = parse(Deno.args, {
  string: ['year', 'day', 'part', 'forceLanguage'],
  alias: {
    yearStr: ['year', 'y'],
    dayStr: ['day', 'd'],
    forceLanguage: ['force-language', 'f'],
  },
});

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
const subprocess = Deno.run({ cmd: selected.getSetupCommand() });
await subprocess.status();

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

// read and append to the manifest file
const manifestStr = await Deno.readTextFile('./2022/manifest.json');
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
