// Day 4 part 2
// https://adventofcode.com/2021/day/4#part2
// input: day4/input.txt

import BingoCard from './BingoCard.ts';

const bingoDetails = (await Deno.readTextFile('day4/input.txt')).split('\n\n');

const numbersCalled = bingoDetails
  .shift()!
  .split(',')
  .map((ns) => parseInt(ns, 10));
const bingoCards = bingoDetails
  .map((c) => c.split('\n'))
  .map((b) => new BingoCard(b));

const winningCards: Array<{ cardNumber: number; score: number }> = [];

while (numbersCalled.length > 0) {
  const lastNumberCalled = numbersCalled.shift()!;
  for (let i = 0; i < bingoCards.length; i++) {
    const bc = bingoCards[i];

    bc.check(lastNumberCalled);

    if (bc.hasBingo && !winningCards.map((c) => c.cardNumber).includes(i)) {
      const cardScore = bc.getCardScore(lastNumberCalled);
      console.log(
        `Bingo card #${i} wins at ${lastNumberCalled} with score ${cardScore}`
      );
      winningCards.push({ cardNumber: i, score: cardScore });
    }
  }
}

const lastWinningCard = winningCards.pop()!;

console.log('\n');
console.log(
  `The last card to win is #${lastWinningCard.cardNumber} with score ${lastWinningCard.score}`
);
