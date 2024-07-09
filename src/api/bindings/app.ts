import { event } from "@tauri-apps/api";

type Method = "Create" | "Read" | "Update" | "Delete";

interface TauriInput<T> {
    command: string;
    method: Method;
    data: T;
    id: string;
}

interface TauriOutput<T> {
    command: string;
    method: Method;
    result: T;
    id: string;
}

const mapMethod = (method: Method) => {
    switch (method) {
        case "Create":
            return "PUT";
        case "Delete":
            return "DELETE";
        case "Read":
            return "GET";
        case "Update":
            return "POST";
    }
};

// Huge thanks to @darkylmnx: https://gist.github.com/tjmehta/9204891?permalink_comment_id=3527084#gistcomment-3527084
const serializeQuery = (initialObj: any) => {
    const reducer =
        (obj: any, parentPrefix: string | null = null) =>
        (prev: string[], key: string) => {
            const val = obj[key];
            key = encodeURIComponent(key);
            const prefix = parentPrefix ? `${parentPrefix}[${key}]` : key;

            if (val == null || typeof val === "function") {
                prev.push(`${prefix}=`);
                return prev;
            }

            if (["number", "boolean", "string"].includes(typeof val)) {
                prev.push(`${prefix}=${encodeURIComponent(val)}`);
                return prev;
            }

            prev.push(Object.keys(val).reduce(reducer(val, prefix), []).join("&"));
            return prev;
        };

    return Object.keys(initialObj).reduce(reducer(initialObj), []).join("&");
};

const responseQueue: Record<string, (data: unknown) => void> = {};

export const setupTauri = () => {
    event.listen<TauriOutput<unknown>>("plugin:rpc-rs:transport:resp", ({ payload: data }) => {
        responseQueue[data.id]?.(data.result);
    });
};

const __rpc_call = async <T, O>(
    routePrefix: string,
    method: Method,
    command: string,
    data: T
): Promise<O> => {
    if (!("__TAURI__" in window)) {
        if (method === "Read") {
            return await fetch(`${routePrefix}/${command}?${serializeQuery(data)}`, {
                method: "GET",
            }).then((v) => v.json());
        }

        return await fetch(`${routePrefix}/${command}`, {
            method: mapMethod(method),
            body: JSON.stringify(data),
        }).then((v) => v.json());
    }

    const id = crypto.randomUUID();

    const input: TauriInput<T> = {
        command,
        id,
        method,
        data,
    };

    const promise = new Promise<O>((res, _rej) => {
        responseQueue[input.id] = res as (data: unknown) => void;
    });

    event.emit("plugin:rpc-rs:transport", input);

    return promise;
};


export async function __rpc_call_mod_Create(data: ModCreation): Promise<bigint | string> {
return await __rpc_call("/rpc", "Create", "mod", data);
}

export async function __rpc_call_mod_Update(data: ModUpdate): Promise<Mod | string> {
return await __rpc_call("/rpc", "Update", "mod", data);
}

export async function __rpc_call_mod_Delete(data: number): Promise<Mod | string> {
return await __rpc_call("/rpc", "Delete", "mod", data);
}

export async function __rpc_call_mod_Read(data: number): Promise<Mod | null> {
return await __rpc_call("/rpc", "Read", "mod", data);
}

const __rpc_module_mod = {
    create: __rpc_call_mod_Create,
    read: __rpc_call_mod_Read,
    update: __rpc_call_mod_Update,
    delete: __rpc_call_mod_Delete,
};


export async function __rpc_call_mods_Create(data: ModCreation[]): Promise<bigint | string> {
return await __rpc_call("/rpc", "Create", "mods", data);
}

export async function __rpc_call_mods_Delete(data: number[]): Promise<bigint | string> {
return await __rpc_call("/rpc", "Delete", "mods", data);
}

export async function __rpc_call_mods_Read(data: null): Promise<Mod[]> {
return await __rpc_call("/rpc", "Read", "mods", data);
}

export async function __rpc_call_mods_Update(data: null): Promise<string> {
return await __rpc_call("/rpc", "Update", "mods", data);
}

const __rpc_module_mods = {
    create: __rpc_call_mods_Create,
    read: __rpc_call_mods_Read,
    update: __rpc_call_mods_Update,
    delete: __rpc_call_mods_Delete,
};


export async function __rpc_call_sources_Read(data: null): Promise<Source[]> {
return await __rpc_call("/rpc", "Read", "sources", data);
}

export async function __rpc_call_sources_Delete(data: number[]): Promise<bigint | string> {
return await __rpc_call("/rpc", "Delete", "sources", data);
}

export async function __rpc_call_sources_Create(data: SourceCreation[]): Promise<bigint | string> {
return await __rpc_call("/rpc", "Create", "sources", data);
}

export async function __rpc_call_sources_Update(data: null): Promise<string> {
return await __rpc_call("/rpc", "Update", "sources", data);
}

const __rpc_module_sources = {
    create: __rpc_call_sources_Create,
    read: __rpc_call_sources_Read,
    update: __rpc_call_sources_Update,
    delete: __rpc_call_sources_Delete,
};


export async function __rpc_call_version_Read(data: null): Promise<string> {
return await __rpc_call("/rpc", "Read", "version", data);
}

export async function __rpc_call_version_Delete(data: null): Promise<string> {
return await __rpc_call("/rpc", "Delete", "version", data);
}

export async function __rpc_call_version_Update(data: null): Promise<string> {
return await __rpc_call("/rpc", "Update", "version", data);
}

export async function __rpc_call_version_Create(data: null): Promise<string> {
return await __rpc_call("/rpc", "Create", "version", data);
}

const __rpc_module_version = {
    create: __rpc_call_version_Create,
    read: __rpc_call_version_Read,
    update: __rpc_call_version_Update,
    delete: __rpc_call_version_Delete,
};


export async function __rpc_call_instance_Update(data: InstanceUpdate): Promise<Instance | string> {
return await __rpc_call("/rpc", "Update", "instance", data);
}

export async function __rpc_call_instance_Delete(data: number): Promise<Instance | string> {
return await __rpc_call("/rpc", "Delete", "instance", data);
}

export async function __rpc_call_instance_Create(data: InstanceCreation): Promise<bigint | string> {
return await __rpc_call("/rpc", "Create", "instance", data);
}

export async function __rpc_call_instance_Read(data: number): Promise<Instance | null> {
return await __rpc_call("/rpc", "Read", "instance", data);
}

const __rpc_module_instance = {
    create: __rpc_call_instance_Create,
    read: __rpc_call_instance_Read,
    update: __rpc_call_instance_Update,
    delete: __rpc_call_instance_Delete,
};


export async function __rpc_call_instances_Read(data: null): Promise<Instance[]> {
return await __rpc_call("/rpc", "Read", "instances", data);
}

export async function __rpc_call_instances_Create(data: InstanceCreation[]): Promise<bigint | string> {
return await __rpc_call("/rpc", "Create", "instances", data);
}

export async function __rpc_call_instances_Update(data: null): Promise<string> {
return await __rpc_call("/rpc", "Update", "instances", data);
}

export async function __rpc_call_instances_Delete(data: number[]): Promise<bigint | string> {
return await __rpc_call("/rpc", "Delete", "instances", data);
}

const __rpc_module_instances = {
    create: __rpc_call_instances_Create,
    read: __rpc_call_instances_Read,
    update: __rpc_call_instances_Update,
    delete: __rpc_call_instances_Delete,
};


export async function __rpc_call_games_Delete(data: number[]): Promise<bigint | string> {
return await __rpc_call("/rpc", "Delete", "games", data);
}

export async function __rpc_call_games_Read(data: null): Promise<Game[]> {
return await __rpc_call("/rpc", "Read", "games", data);
}

export async function __rpc_call_games_Update(data: null): Promise<string> {
return await __rpc_call("/rpc", "Update", "games", data);
}

export async function __rpc_call_games_Create(data: GameCreation[]): Promise<bigint | string> {
return await __rpc_call("/rpc", "Create", "games", data);
}

const __rpc_module_games = {
    create: __rpc_call_games_Create,
    read: __rpc_call_games_Read,
    update: __rpc_call_games_Update,
    delete: __rpc_call_games_Delete,
};


export async function __rpc_call_game_Update(data: GameUpdate): Promise<Game | string> {
return await __rpc_call("/rpc", "Update", "game", data);
}

export async function __rpc_call_game_Read(data: number): Promise<Game | null> {
return await __rpc_call("/rpc", "Read", "game", data);
}

export async function __rpc_call_game_Delete(data: number): Promise<Game | string> {
return await __rpc_call("/rpc", "Delete", "game", data);
}

export async function __rpc_call_game_Create(data: GameCreation): Promise<bigint | string> {
return await __rpc_call("/rpc", "Create", "game", data);
}

const __rpc_module_game = {
    create: __rpc_call_game_Create,
    read: __rpc_call_game_Read,
    update: __rpc_call_game_Update,
    delete: __rpc_call_game_Delete,
};


export async function __rpc_call_source_Delete(data: number): Promise<Source | string> {
return await __rpc_call("/rpc", "Delete", "source", data);
}

export async function __rpc_call_source_Create(data: SourceCreation): Promise<bigint | string> {
return await __rpc_call("/rpc", "Create", "source", data);
}

export async function __rpc_call_source_Update(data: SourceUpdate): Promise<Source | string> {
return await __rpc_call("/rpc", "Update", "source", data);
}

export async function __rpc_call_source_Read(data: number): Promise<Source | null> {
return await __rpc_call("/rpc", "Read", "source", data);
}

const __rpc_module_source = {
    create: __rpc_call_source_Create,
    read: __rpc_call_source_Read,
    update: __rpc_call_source_Update,
    delete: __rpc_call_source_Delete,
};


export type Game = { id: number; name: string; curseforge: number | null; thunderstore: string | null; spacedock: string | null; ckan: boolean; modrinth: boolean }

export type GameCreation = { name: string }

export type GameUpdate = { id: number; name?: string | null; curseforge?: number | null; thunderstore?: string | null; spacedock?: string | null; ckan?: boolean | null; modrinth?: boolean | null }

export type Instance = { id: number; name: string; gameId: number; dataDir: string; cacheDir: string; installDir: string; description: string; created: string; updated: string; loader: string | null }

export type InstanceCreation = { name: string; gameId: number; dataDir: string; cacheDir: string; installDir: string }

export type InstanceUpdate = { id: number; name?: string | null; gameId?: number | null; dataDir?: string | null; cacheDir?: string | null; installDir?: string | null; description?: string | null; created?: string | null; updated?: string | null; loader?: string | null }

export type Mod = { id: number; mod: string; version: string | null; name: string; file: string; size: number; hash: string | null; installed_files: string; sourceId: number; instanceId: number }

export type ModCreation = { mod: string; name: string; file: string; size: number; installed_files: string; sourceId: number; instanceId: number }

export type ModUpdate = { id: number; mod?: string | null; version?: string | null; name?: string | null; file?: string | null; size?: number | null; hash?: string | null; installed_files?: string | null; sourceId?: number | null; instanceId?: number | null }

export type Source = { id: number; name: string }

export type SourceCreation = { name: string }

export type SourceUpdate = { id: number; name?: string | null }

export const RPC = {
mod: __rpc_module_mod,
mods: __rpc_module_mods,
sources: __rpc_module_sources,
version: __rpc_module_version,
instance: __rpc_module_instance,
instances: __rpc_module_instances,
games: __rpc_module_games,
game: __rpc_module_game,
source: __rpc_module_source,
};

export default RPC;