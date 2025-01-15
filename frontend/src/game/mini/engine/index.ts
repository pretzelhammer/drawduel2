import { Game, ServerEvent } from './mini_game';

export * from './mini_game';

export function newGame(): Game {
    return Game.create();
}

export function advanceGame(game: Game, serverEvent: ServerEvent) {}
