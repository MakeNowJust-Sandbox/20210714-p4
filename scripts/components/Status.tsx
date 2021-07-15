import * as React from "react";
import { useRecoilValue } from "recoil";

import { COLUMN_NUMBER, ROW_NUMBER } from "../board";
import { historyState, winnerSelector, scoresSelector } from "../state";

export const Status = () => {
  const history = useRecoilValue(historyState);
  const winner = useRecoilValue(winnerSelector);
  const scores = useRecoilValue(scoresSelector);

  const moves = history.length;
  const player = 1 - (history.length % 2) * 2;
  const scoreToMoves = (score: number) => {
    if (score === 0) return COLUMN_NUMBER * ROW_NUMBER;

    const n = score * player;
    return COLUMN_NUMBER * ROW_NUMBER - (Math.abs(n) - 1) * 2 - (n > 0 ? 1 : 0);
  };

  let status = "";
  switch (winner) {
    case 0:
      if (history.length === 42) {
        status = "引き分け";
      } else {
        const maxScore = Math.max(...scores);
        status = `${scoreToMoves(maxScore) - moves} 手後に`;
        if (maxScore > 0) {
          status += "勝つ";
        } else if (maxScore < 0) {
          status += "負ける";
        } else {
          status += "引き分ける";
        }
        status += history.length % 2 === 0 ? "先手" : "後手";
      }
      break;
    case 1:
      status = "先手の勝ち";
      break;
    case 2:
      status = "後手の勝ち";
      break;
  }

  const scoreText = (score: number) => {
    if (score > 0) {
      return `${scoreToMoves(score) - moves} 手後に勝ち`;
    }
    if (score < 0) {
      return `${scoreToMoves(score) - moves} 手後に負け`;
    }
    return `${scoreToMoves(score) - moves} 手後に引き分け`;
  };

  const scoresList = scores.map((score, i) => {
    if (score === -1000) {
      return null;
    }

    return (
      <li key={i}>
        {i + 1} 列目に置く場合、{scoreText(score)}
      </li>
    );
  });

  return (
    <div className="box">
      <div className="content">
        <h3>ネタバレ</h3>
        <p>{status}</p>
        {winner !== 0 || history.length == ROW_NUMBER * COLUMN_NUMBER ? null : (
          <>
            <h3>解析結果</h3>
            <ul>{scoresList}</ul>
          </>
        )}
      </div>
    </div>
  );
};
