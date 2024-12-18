import { Chessboard } from "react-chessboard";
import { Chess } from "chess.js";
import { useEffect, useState } from "react";
import type { Square } from "chess.js";

import { get_move, get_eval } from "rsw-hello";

export function App() {
  const [game, setGame] = useState(new Chess());
  const [thinking, setThinking] = useState(false);
  function update() {
    setGame(new Chess(game.fen()));
  }

  const onPieceDrop = (sourceSquare: Square, targetSquare: Square) => {
    try {
      game.move({
        from: sourceSquare,
        to: targetSquare,
        promotion: "q",
      });

      update();

      setThinking(true);
      setTimeout(() => {
        const engineMove = get_move(game.fen());
        game.move(engineMove);
        update();
        setThinking(false);
      }, 100);

      return true;
    } catch (e) {
      return false;
    }
  };

  return (
    <div
      style={{
        width: "400px",
      }}
    >
      <Chessboard
        boardOrientation="black"
        position={game.fen()}
        onPieceDrop={onPieceDrop}
        boardWidth={400}
      />
      {thinking && <div>Thinking...</div>}
    </div>
  );
}

export function EvalTest() {
  const [currentEval, setEval] = useState(0);
  const [fen, setFen] = useState(new Chess().fen());

  useEffect(() => {
    setEval(get_eval(fen));
  }, [fen]);

  return (
    <div>
      <Chessboard
        boardWidth={400}
        getPositionObject={(pos) => {
          const game = new Chess();
          game.clear();
          for (const [square, piece] of Object.entries(pos)) {
            const [file, rank] = square.split("") as [string, string];
            const [colorChar, typeChar] = piece.split("") as [string, string];

            const type = typeChar.toLowerCase() as
              | "p"
              | "n"
              | "b"
              | "r"
              | "q"
              | "k";
            game.put({ color: colorChar as "w" | "b", type }, square as Square);
          }

          setFen(game.fen());
        }}
      />
      <div>{fen}</div>
      <div>{currentEval}</div>
    </div>
  );
}
