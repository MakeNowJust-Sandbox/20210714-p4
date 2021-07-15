import { atom, selector } from "recoil";

import { analyze } from "../pkg";
import { initBoard, play, judge } from "./board";

export const historyState = atom<string>({
  key: "HistoryState",
  default: "",
});

export const autoState = atom<[boolean, boolean]>({
  key: "AutoState",
  default: [false, false],
});

export const guideState = atom<boolean>({
  key: "GuideState",
  default: false,
});

export const boardSelector = selector({
  key: "BoardSelector",
  get: ({ get }) => {
    const history = get(historyState);

    const board: number[][] = initBoard();
    for (let i = 0; i < history.length; i++) {
      play(board, Number.parseInt(history[i], 10), (i % 2) + 1);
    }

    return board;
  },
});

export const winnerSelector = selector({
  key: "WinnerSelector",
  get: ({ get }) => {
    const board = get(boardSelector);
    return judge(board);
  },
});

export const scoresSelector = selector({
  key: "ScoresSelector",
  get: ({ get }) => {
    const history = get(historyState);
    return Array.from(analyze(history));
  },
});
