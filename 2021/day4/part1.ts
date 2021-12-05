// Day 4 part 1
// https://adventofcode.com/2021/day/4
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

let winnerFound = false;

while (!winnerFound) {
  const lastNumberCalled = numbersCalled.shift()!;
  for (let i = 0; i < bingoCards.length; i++) {
    const bc = bingoCards[i];

    bc.check(lastNumberCalled);
    winnerFound = bc.hasBingo;

    if (winnerFound) {
      console.log('\n');
      console.log(
        `Winner found! Card score is ${bc.getCardScore(lastNumberCalled)}`
      );
      console.log(`Last number called was ${lastNumberCalled}`);
      console.log(`Winning bingo card is ${i}`);
      break;
    }
  }
}
