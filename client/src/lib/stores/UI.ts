import { writable, type Writable } from 'svelte/store';
import { type SelectedTile } from '../domain/SelectedTile';

export const actorTileStore: Writable<SelectedTile> = writable({
    sheet: "actor",
    idx: 0
});

export const selectedTileStore: Writable<SelectedTile> = writable({
    sheet: "dungeon",
    idx: 0
});

export const modal = writable(null);
