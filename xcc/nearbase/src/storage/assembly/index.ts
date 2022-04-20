import { context, u128, PersistentMap, logging, ContractPromise } from "near-sdk-as";
import { AddArgs } from "./models";

const sample1 = 'dev-1650465405897-76865791768218'

export class GamePlayer {
    play(guess: u32, gameId: u32): ContractPromise {
        let args: AddArgs = { guess, gameId }
        let promise = ContractPromise.create(sample1, "makeAGuess", args.encode(), 100000000000000)
        logging.log("OTHER_CONTRACT: " + "(" + sample1 + ")")
        return promise;
    }
    init(): ContractPromise {
        let promise = ContractPromise.create(sample1, "createGame", "", 100000000000000)
        logging.log("OTHER_CONTRACT: " + "(" + sample1 + ")")
        return promise;
    }
}

export function start(): void {
    let gamer = new GamePlayer()
    let promise = gamer.init()
    logging.log(promise)
    promise.returnAsResult()
}

export function playGame(guess: u32, gameId: u32): void {
    let gamer = new GamePlayer();
    let promise = gamer.play(guess, gameId)
    logging.log(promise)
    promise.returnAsResult()
}