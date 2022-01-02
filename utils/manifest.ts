export function getManifestLocation(year: number) {
  return `./${year}/manifest.json`;
}

export function addDayToManifest(
  day: number,
  language: string,
  currentManifest: { [key: string]: number[] } = {}
) {
  if (!currentManifest[language]) {
    currentManifest[language] = [day];
  } else if (!currentManifest[language].includes(day)) {
    currentManifest[language].push(day);
  }

  return currentManifest;
}
