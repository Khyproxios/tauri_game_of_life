import { writable } from 'svelte/store';
import {invoke} from "@tauri-apps/api/tauri";

const createBoard = () => {
    const { subscribe, set } = writable({
        width: 10,
        height: 10,
        cells: Array(100).fill(false)
    });

    return {
        subscribe,
        set,
        update: async () => await invoke('update')
            .then(async () => await invoke('get_board')
                .then((newBoard) => set(newBoard)))
            .then(async () => aliveCount.set(await invoke('get_alive_count'))),
        reset: async () => await invoke('reset')
            .then(async () => await invoke('get_board')
                .then((newBoard) => set(newBoard)))
            .then(async () => aliveCount.set(await invoke('get_alive_count')))
    };
};

export const board = createBoard();
export const aliveCount = writable(0);
export const speed = writable(1);
