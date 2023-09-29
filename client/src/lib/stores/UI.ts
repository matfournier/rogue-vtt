import { writable, type Writable } from 'svelte/store';
import { type SelectedTile } from '../domain/SelectedTile';
import { type RichEntity, type Sidebar } from '../domain/EntityRenderer';
import { LoginStateType, type LoginState } from '../login/loginstate';

export const actorTileStore: Writable<SelectedTile> = writable({
    sheet: "actor",
    idx: 0
});

export const selectedTileStore: Writable<SelectedTile> = writable({
    sheet: "dungeon",
    idx: 0
});


export const entityEventStore: Writable<Array<Event>> = writable([]);

export const entityStore: Writable<Sidebar> = writable({ players: new Array<RichEntity>(), npcs: new Array<RichEntity>() });

export const modal = writable(null);

export const loginStateStore: Writable<LoginState> = writable({ kind: LoginStateType.Login });