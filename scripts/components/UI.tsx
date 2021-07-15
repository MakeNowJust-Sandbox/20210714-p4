import * as React from "react";
import { useRecoilState, useRecoilValue } from "recoil";
import { COLUMN_NUMBER, ROW_NUMBER } from "../board";

import {
  autoState,
  guideState,
  historyState,
  scoresSelector,
  winnerSelector,
} from "../state";

export const UI = () => {
  const [guide, setGuide] = useRecoilState(guideState);
  const [history, setHistory] = useRecoilState(historyState);
  const [[firstAuto, secondAuto], setAuto] = useRecoilState(autoState);
  const scores = useRecoilValue(scoresSelector);
  const winner = useRecoilValue(winnerSelector);

  const toggleGuide = React.useCallback(() => {
    setGuide((guide) => !guide);
  }, [guide]);

  const resetHistory = React.useCallback(() => {
    setHistory("");
  }, []);
  const backHistory = React.useCallback(() => {
    setHistory(history.slice(0, -1));
  }, [history]);

  React.useEffect(() => {
    if (history.length === COLUMN_NUMBER * ROW_NUMBER || winner !== 0) {
      return;
    }

    if (
      (firstAuto && history.length % 2 === 0) ||
      (secondAuto && history.length % 2 === 1)
    ) {
      const timer = setTimeout(() => {
        const maxScore = Math.max(...scores);
        const columns = scores
          .map((_, i) => i + 1)
          .filter((column) => scores[column - 1] == maxScore);
        setHistory(
          (history) =>
            history + columns[Math.floor(Math.random() * columns.length)]
        );
      }, 500);
      return () => clearTimeout(timer);
    }
  }, [scores, firstAuto, secondAuto, history]);

  const toggleFirstAuto = React.useCallback(() => {
    setAuto(([firstAuto, secondAuto]) => [!firstAuto, secondAuto]);
  }, [firstAuto, secondAuto]);

  const toggleSecondAuto = React.useCallback(() => {
    setAuto(([firstAuto, secondAuto]) => [firstAuto, !secondAuto]);
  }, [firstAuto, secondAuto]);

  return (
    <div className="box">
      <div className="field has-addons">
        <p className="control">
          <button onClick={toggleGuide} className="button">
            評価値を{guide ? "隠す" : "表示"}
          </button>
        </p>
        <p className="control">
          <button onClick={resetHistory} className="button">
            最初から
          </button>
        </p>
        <p className="control">
          <button onClick={backHistory} className="button">
            一手戻る
          </button>
        </p>
      </div>
      <div className="field has-addons">
        <p className="control">
          <button
            onClick={toggleFirstAuto}
            className={"button is-danger" + (firstAuto ? "" : " is-light")}
          >
            先手の自動化
          </button>
        </p>
        <p className="control">
          <button
            onClick={toggleSecondAuto}
            className={"button  is-warning" + (secondAuto ? "" : " is-light")}
          >
            後手の自動化
          </button>
        </p>
      </div>
    </div>
  );
};
