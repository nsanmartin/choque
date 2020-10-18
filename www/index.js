import { Game } from "wasm-game";

const game = Game.new();

let animationId = null;

const renderLoop = () => {
    game.run_loop();
    animationId = requestAnimationFrame(renderLoop);
};


renderLoop();
