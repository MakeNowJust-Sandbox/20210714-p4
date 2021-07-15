import * as React from "react";
import { RecoilRoot } from "recoil";

import { GameBoard } from "./components/GameBoard";
import { Info } from "./components/Info";
import { Hero } from "./components/Hero";

export const App = () => {
  return (
    <RecoilRoot>
      <div>
        <Hero />
        <div className="container">
          <div className="columns">
            <div className="column is-two-thirds">
              <GameBoard />
            </div>
            <div className="column is-one-third">
              <Info />
            </div>
          </div>
          <div className="box">
            <div className="content">
              <h2>説明</h2>
              <p>
                コネクトフォー (重力付き四目並べ) の手順を<b>完全に</b>
                解析して、最適な手を選び続けたときの結果を表示するツールです。
              </p>
              <h2>機能</h2>
              <p>コマを置きたい列をクリックすることで手番が進みます。</p>
              <ul>
                <li>
                  「評価値を隠す/表示」:
                  マス目に出る評価値の表示を切り替えます。評価値が正の場合勝ち確で、負の場合負け確です。評価値が高いほど早く決着します。
                </li>
                <li>「最初から」: 盤面の状態を初期化します。</li>
                <li>「一手戻す」: 盤面の状態を一手前に戻します。</li>
                <li>「一手戻す」: 盤面の状態を一手前に戻します。</li>
                <li>
                  「先手/後手の自動化」:
                  先手・後手の手番の際に、最適な手が自動で選ばれるようにします。
                </li>
              </ul>
              <h2>実装について</h2>
              <p>
                α-β法による枝刈りを行う反復深化探索によって、可能な局面をすべて探索しています。詳細については、
                <a href="http://blog.gamesolver.org">この記事</a>
                を確認してください。
              </p>
              <p>
                また、実装は Rust で行い、WebAssembly
                にビルドすることでブラウザ上で実行できるようにしています。
              </p>
              <p>
                ソースコードは{" "}
                <a href="https://github.com/MakeNowJust-Labo/p4">
                  https://github.com/MakeNowJust-Labo/p4
                </a>{" "}
                で公開されています。
              </p>
              <p className="has-text-centered">
                <small>2021 (C) TSUYUSATO "MakeNowJust" Kitsune</small>
              </p>
            </div>
          </div>
        </div>
      </div>
    </RecoilRoot>
  );
};
