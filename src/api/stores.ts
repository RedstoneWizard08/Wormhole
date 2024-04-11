import { writable } from "svelte/store";
import type { PluginInfo } from "./bindings/app";

export const plugins = writable<PluginInfo[]>([]);
