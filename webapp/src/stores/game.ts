import { writable } from 'svelte/store'
import Game from '../entities/game'

const game = new Game()

export const gameStore = writable(game)
