export const COLUMN_NUMBER = 7;
export const ROW_NUMBER = 6;

export type Board = number[][];

export const initBoard = () => {
  const board: Board = [];
  for (let y = 0; y < ROW_NUMBER; y++) {
    board.push([]);
    for (let x = 0; x < COLUMN_NUMBER; x++) {
      board[y].push(0);
    }
  }
  return board;
};

export const canPlay = (board: Board, column: number) =>
  board[0][column - 1] === 0;

export const play = (board: Board, column: number, player: number) => {
  for (let y = ROW_NUMBER - 1; y >= 0; y--) {
    if (board[y][column - 1] === 0) {
      board[y][column - 1] = player;
      return;
    }
  }
};

const JUDGE_DIRS = [
  [1, -1],
  [1, 0],
  [1, 1],
  [0, 1],
  [-1, 1],
  [-1, 0],
  [-1, -1],
];

export const judge = (board: Board) => {
  for (let x = 0; x < COLUMN_NUMBER; x++) {
    for (let y = 0; y < ROW_NUMBER; y++) {
      if (board[y][x] !== 0) {
        for (const [dx, dy] of JUDGE_DIRS) {
          let n = 1;
          for (; n < 4; n++) {
            const x1 = x + dx * n;
            if (x1 < 0 || COLUMN_NUMBER <= x1) {
              break;
            }
            const y1 = y + dy * n;
            if (y1 < 0 || ROW_NUMBER <= y1) {
              break;
            }

            if (board[y][x] !== board[y1][x1]) {
              break;
            }
          }
          if (n === 4) {
            return board[y][x];
          }
        }
      }
    }
  }
  return 0;
};
