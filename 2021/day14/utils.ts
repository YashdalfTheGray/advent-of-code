export const getPairsFrom = (state: string) => {
  const pairs = [];
  const splitState = state.split('');

  while (splitState.length > 1) {
    const pairStart = splitState.shift()!;
    const pairEnd = splitState[0];
    pairs.push(pairStart + pairEnd);
  }

  return pairs;
};

export const checkAndInsertElement = (
  pair: string,
  insertionMap: Map<string, string>
) => {
  const [start, end] = pair.split('');
  const insertion = insertionMap.get(pair);
  return start + (insertion ?? '') + end;
};
