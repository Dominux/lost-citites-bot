import { writable } from 'svelte/store'

import { MoveProcess } from '../entities/move_process'

const moveProcess = new MoveProcess('Player1')

export const moveProcessStore = writable(moveProcess)
