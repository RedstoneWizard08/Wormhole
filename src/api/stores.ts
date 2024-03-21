import { writable } from "svelte/store";
import type { Plugin } from "./models/plugin";
import type { InstanceInfo } from "./instance";

export const plugins = writable<Record<number, Plugin>>({});
export const activeInstance = writable<InstanceInfo | undefined>();
