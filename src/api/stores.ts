import { writable } from "svelte/store";

type PluginInfo = any;

export const plugins = writable<PluginInfo[]>([]);
