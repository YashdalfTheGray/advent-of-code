export const openingBrackets = ['(', '[', '{', '<'];
export const closingBrackets = [')', ']', '}', '>'];
export const syntaxCheckerScoringMatrix = {
  ')': 3,
  ']': 57,
  '}': 1197,
  '>': 25137,
};
export const autocompleteScoringMatrix = {
  ')': 1,
  ']': 2,
  '}': 3,
  '>': 4,
};

export const getOpeningBracketFor = (closingBracket: string) => {
  switch (closingBracket) {
    case '}':
      return '{';
    case ']':
      return '[';
    case ')':
      return '(';
    case '>':
      return '<';
    default:
      throw new Error(`Invalid closing bracket: ${closingBracket}`);
  }
};

export const getClosingBracketFor = (openingBracket: string) => {
  switch (openingBracket) {
    case '{':
      return '}';
    case '[':
      return ']';
    case '(':
      return ')';
    case '<':
      return '>';
    default:
      throw new Error(`Invalid opening bracket: ${openingBracket}`);
  }
};

export const countBrackets = (input: string): string[] => {
  const bracketStack: string[] = [];

  for (let i = 0; i < input.length; i++) {
    const current = input[i];
    if (openingBrackets.includes(current)) {
      bracketStack.push(current);
    } else if (closingBrackets.includes(current)) {
      const peek = bracketStack.pop()!;
      if (peek !== getOpeningBracketFor(current)) {
        bracketStack.push(peek);
        bracketStack.push(current);
        break;
      }
    }
  }

  return bracketStack;
};

export const generateClosingBracketString = (bracketStack: string[]) =>
  bracketStack.reverse().map(getClosingBracketFor).join('');

export const scoreCorruptedLinesReducer = (score: number, stack: string[]) =>
  score +
  syntaxCheckerScoringMatrix[
    stack[stack.length - 1] as keyof typeof syntaxCheckerScoringMatrix
  ];

export const scoreGeneratedBracketString = (bracketString: string) =>
  bracketString
    .split('')
    .reduce(
      (score, b) =>
        score * 5 +
        autocompleteScoringMatrix[b as keyof typeof autocompleteScoringMatrix],
      0
    );
