import { writable, type Writable } from 'svelte/store';
import { type SelectedTile } from '../domain/SelectedTile';

export const dungeonTileStore: Writable<SelectedTile> = writable({
    sheet: "dungeon",
    idx: 0
});

export const featureTileStore: Writable<SelectedTile> = writable({
    sheet: "feature",
    idx: 0
});

export const actorTileStore: Writable<SelectedTile> = writable({
    sheet: "actor",
    idx: 0
});

export const selectedTileStore: Writable<SelectedTile> = writable({
    sheet: "dungeon",
    idx: 0
});
