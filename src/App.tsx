import { Chessboard } from "react-chessboard";
import { Chess } from "chess.js";
import { useEffect, useState } from "react";
import type { Square } from "chess.js";

import { get_move } from "rsw-hello";

export function App() {
  const [game, setGame] = useState(new Chess());
  function update() {
    setGame(new Chess(game.fen()));
  }

  // useEffect(() => {
  //   const move = () => {
  //     const engineMove = get_move(game.fen());
  //     game.move(engineMove);
  //     update();
  //     setTimeout(move, 100);
  //   };
  //   move();
  // }, []);

  const onPieceDrop = (sourceSquare: Square, targetSquare: Square) => {
    try {
      const move = game.move({
        from: sourceSquare,
        to: targetSquare,
        promotion: "q",
      });

      const engineMove = get_move(game.fen());
      game.move(engineMove);

      update();

      return true;
    } catch (e) {
      alert("Invalid move");
      return false;
    }
  };

  return <Chessboard position={game.fen()} onPieceDrop={onPieceDrop} />;
}
