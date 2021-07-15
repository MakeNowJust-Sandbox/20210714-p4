import * as React from "react";
import { useRecoilState, useRecoilValue } from "recoil";

import { canPlay, COLUMN_NUMBER, ROW_NUMBER } from "../board";
import {
  autoState,
  boardSelector,
  guideState,
  historyState,
  scoresSelector,
  winnerSelector,
} from "../state";

const CELL_SIZE = 125;
const WIDTH = CELL_SIZE * COLUMN_NUMBER;
const HEIGHT = CELL_SIZE * ROW_NUMBER;

const FONT_SIZE = 32;

// https://material.io/design/color/the-color-system.html
const BG_COLOR = "#2196F3"; // blue 500
const LINE_COLOR = "#0D47A1"; // blue 900
const EMPTY_COLOR = "#ECEFF1"; // blue gray 50
const FIRST_COLOR = "#F44336"; // red 500
const SECOND_COLOR = "#FFEB3B"; // yellow 500
const SCORE_COLOR = "#212121"; // gray 900

export const GameBoard = () => {
  const canvas = React.useRef<HTMLCanvasElement>(null);

  const [history, setHistory] = useRecoilState(historyState);
  const board = useRecoilValue(boardSelector);
  const winner = useRecoilValue(winnerSelector);
  const scores = useRecoilValue(scoresSelector);
  const [firstAuto, secondAuto] = useRecoilValue(autoState);
  const guide = useRecoilValue(guideState);

  const onClick = React.useCallback(
    (event: React.MouseEvent<HTMLCanvasElement>) => {
      if (canvas.current === null || winner !== 0) {
        return;
      }
      if (
        (firstAuto && history.length % 2 === 0) ||
        (secondAuto && history.length % 2 === 1)
      ) {
        return;
      }

      const rect = canvas.current.getBoundingClientRect();
      const mouseX = event.clientX - rect.x;
      const column = Math.floor(mouseX / (WIDTH / COLUMN_NUMBER)) + 1;

      if (column <= 0 || COLUMN_NUMBER < column || !canPlay(board, column)) {
        return;
      }

      setHistory((history) => history + String(column));
    },
    [canvas, board, winner, history, firstAuto, secondAuto]
  );

  React.useEffect(() => {
    if (canvas.current === null) {
      return;
    }

    const ctx = canvas.current.getContext("2d");
    if (ctx === null) {
      return;
    }
    ctx.fillStyle = BG_COLOR;
    ctx.fillRect(0, 0, WIDTH, HEIGHT);

    for (let x = 1; x < COLUMN_NUMBER; x++) {
      ctx.beginPath();
      ctx.moveTo((x / COLUMN_NUMBER) * WIDTH - 1, 0);
      ctx.lineTo((x / COLUMN_NUMBER) * WIDTH - 1, HEIGHT);
      ctx.lineTo((x / COLUMN_NUMBER) * WIDTH + 1, HEIGHT);
      ctx.lineTo((x / COLUMN_NUMBER) * WIDTH + 1, 0);
      ctx.closePath();
      ctx.fillStyle = LINE_COLOR;
      ctx.fill();
    }

    for (let y = 1; y < ROW_NUMBER; y++) {
      ctx.beginPath();
      ctx.moveTo(0, (y / ROW_NUMBER) * HEIGHT - 1);
      ctx.lineTo(WIDTH, (y / ROW_NUMBER) * HEIGHT - 1);
      ctx.lineTo(WIDTH, (y / ROW_NUMBER) * HEIGHT + 1);
      ctx.lineTo(0, (y / ROW_NUMBER) * HEIGHT + 1);
      ctx.closePath();
      ctx.fillStyle = LINE_COLOR;
      ctx.fill();
    }

    for (let y = 0; y < ROW_NUMBER; y++) {
      for (let x = 0; x < COLUMN_NUMBER; x++) {
        ctx.beginPath();
        ctx.arc(
          ((x + 0.5) / COLUMN_NUMBER) * WIDTH,
          ((y + 0.5) / ROW_NUMBER) * HEIGHT,
          WIDTH / COLUMN_NUMBER / 2.25,
          0,
          (360 * Math.PI) / 180,
          false
        );
        switch (board[y][x]) {
          case 0:
            ctx.fillStyle = EMPTY_COLOR;
            break;
          case 1:
            ctx.fillStyle = FIRST_COLOR;
            break;
          case 2:
            ctx.fillStyle = SECOND_COLOR;
            break;
        }
        ctx.fill();

        if (
          guide &&
          board[y][x] === 0 &&
          (y === ROW_NUMBER - 1 || board[y + 1][x] !== 0)
        ) {
          const score = scores[x];
          const scoreText = score > 0 ? `+${score}` : String(score);
          ctx.font = `${FONT_SIZE}px sans-serif`;
          const { width } = ctx.measureText(scoreText);
          ctx.fillStyle = SCORE_COLOR;
          ctx.fillText(
            scoreText,
            ((x + 0.5) / COLUMN_NUMBER) * WIDTH - width / 2,
            ((y + 0.5) / ROW_NUMBER) * HEIGHT + FONT_SIZE / 2
          );
        }
      }
    }
  }, [canvas, board, scores, guide]);

  return (
    <div className="container p-4">
      <canvas
        style={{ maxWidth: "100%" }}
        onClick={onClick}
        width={WIDTH}
        height={HEIGHT}
        ref={canvas}
      />
    </div>
  );
};
