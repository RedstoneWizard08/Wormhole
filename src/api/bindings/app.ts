import { event } from "@tauri-apps/api";

export type Method = "Create" | "Read" | "Update" | "Delete";
export type Result<T, E> = __Ok__<T> | __Err__<E>;
export type Option<T> = __Some__<T> | __None__;

type __Ok__<T> = { Ok: T };
type __Err__<E> = { Err: E };
type __Some__<T> = { Some: T };
type __None__ = { None: null };

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

interface TauriInvokeInput<T> {
    command: string;
    data: T;
    id: string;
}

interface TauriInvokeOutput<T> {
    command: string;
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
            return "POST";
        case "Update":
            return "PATCH";
    }
};

const responseQueue: Record<string, (data: unknown) => void> = {};

const __rpc_call = async <T, O>(
    routePrefix: string,
    method: Method,
    command: string,
    data: T
): Promise<O> => {
    if (!("__TAURI__" in window)) {
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

const __rpc_invoke = async <T, O>(routePrefix: string, command: string, data: T): Promise<O> => {
    if (!("__TAURI__" in window)) {
        return await fetch(`${routePrefix}/_invoke/${command}`, {
            method: "POST",
            body: JSON.stringify(data),
        }).then((v) => v.json());
    }

    const id = crypto.randomUUID();

    const input: TauriInvokeInput<T> = {
        command,
        id,
        data,
    };

    const promise = new Promise<O>((res, _rej) => {
        responseQueue[input.id] = res as (data: unknown) => void;
    });

    event.emit("plugin:rpc-rs:transport:invoker", input);

    return promise;
};

export const setupTauri = () => {
    if (!("__TAURI__" in window)) return;

    event.listen<TauriOutput<unknown>>("plugin:rpc-rs:transport:resp", ({ payload: data }) => {
        responseQueue[data.id]?.(data.result);
    });

    event.listen<TauriInvokeOutput<unknown>>(
        "plugin:rpc-rs:transport:invoker:resp",
        ({ payload: data }) => {
            responseQueue[data.id]?.(data.result);
        }
    );
};
export const unwrap = <T>(res: Result<T, any> | Option<T>): T => {
    if (res === null || "None" in res) {
        throw new ReferenceError("Tried to unwrap a 'None' value!");
        // biome-ignore lint/style/noUselessElse: IT'S NOT REAL
    } else if ("Some" in res) {
        return res.Some;
        // biome-ignore lint/style/noUselessElse: IT'S NOT REAL
    } else {
        return (res as __Ok__<T>).Ok;
    }
};


export async function __rpc_call_latestLoader_Delete(data: null = null): Promise<string> {
return await __rpc_call("/rpc", "Delete", "latestLoader", data);
}

export async function __rpc_call_latestLoader_Create(data: null = null): Promise<string> {
return await __rpc_call("/rpc", "Create", "latestLoader", data);
}

export async function __rpc_call_latestLoader_Read(data: ModLoaderType): Promise<Result<ModLoader, string>> {
return await __rpc_call("/rpc", "Read", "latestLoader", data);
}

export async function __rpc_call_latestLoader_Update(data: null = null): Promise<string> {
return await __rpc_call("/rpc", "Update", "latestLoader", data);
}

const __rpc_module_latestLoader = {
    create: __rpc_call_latestLoader_Create,
    read: __rpc_call_latestLoader_Read,
    update: __rpc_call_latestLoader_Update,
    delete: __rpc_call_latestLoader_Delete,
};


export async function __rpc_call_instance_Read(data: number): Promise<Option<Instance>> {
return await __rpc_call("/rpc", "Read", "instance", data);
}

export async function __rpc_call_instance_Create(data: InstanceCreation): Promise<Result<bigint, string>> {
return await __rpc_call("/rpc", "Create", "instance", data);
}

export async function __rpc_call_instance_Update(data: InstanceUpdate): Promise<Result<Instance, string>> {
return await __rpc_call("/rpc", "Update", "instance", data);
}

export async function __rpc_call_instance_Delete(data: number): Promise<Result<Instance, string>> {
return await __rpc_call("/rpc", "Delete", "instance", data);
}

const __rpc_module_instance = {
    create: __rpc_call_instance_Create,
    read: __rpc_call_instance_Read,
    update: __rpc_call_instance_Update,
    delete: __rpc_call_instance_Delete,
};


export async function __rpc_call_dirs_Delete(data: null = null): Promise<string> {
return await __rpc_call("/rpc", "Delete", "dirs", data);
}

export async function __rpc_call_dirs_Create(data: null = null): Promise<string> {
return await __rpc_call("/rpc", "Create", "dirs", data);
}

export async function __rpc_call_dirs_Read(data: string): Promise<Dirs> {
return await __rpc_call("/rpc", "Read", "dirs", data);
}

export async function __rpc_call_dirs_Update(data: null = null): Promise<string> {
return await __rpc_call("/rpc", "Update", "dirs", data);
}

const __rpc_module_dirs = {
    create: __rpc_call_dirs_Create,
    read: __rpc_call_dirs_Read,
    update: __rpc_call_dirs_Update,
    delete: __rpc_call_dirs_Delete,
};


export async function __rpc_call_instances_Create(data: InstanceCreation[]): Promise<Result<bigint, string>> {
return await __rpc_call("/rpc", "Create", "instances", data);
}

export async function __rpc_call_instances_Read(data: number): Promise<Instance[]> {
return await __rpc_call("/rpc", "Read", "instances", data);
}

export async function __rpc_call_instances_Update(data: null = null): Promise<string> {
return await __rpc_call("/rpc", "Update", "instances", data);
}

export async function __rpc_call_instances_Delete(data: number[]): Promise<Result<bigint, string>> {
return await __rpc_call("/rpc", "Delete", "instances", data);
}

const __rpc_module_instances = {
    create: __rpc_call_instances_Create,
    read: __rpc_call_instances_Read,
    update: __rpc_call_instances_Update,
    delete: __rpc_call_instances_Delete,
};


export async function __rpc_call_source_Update(data: SourceUpdate): Promise<Result<Source, string>> {
return await __rpc_call("/rpc", "Update", "source", data);
}

export async function __rpc_call_source_Create(data: SourceCreation): Promise<Result<bigint, string>> {
return await __rpc_call("/rpc", "Create", "source", data);
}

export async function __rpc_call_source_Read(data: number): Promise<Option<Source>> {
return await __rpc_call("/rpc", "Read", "source", data);
}

export async function __rpc_call_source_Delete(data: number): Promise<Result<Source, string>> {
return await __rpc_call("/rpc", "Delete", "source", data);
}

const __rpc_module_source = {
    create: __rpc_call_source_Create,
    read: __rpc_call_source_Read,
    update: __rpc_call_source_Update,
    delete: __rpc_call_source_Delete,
};


export async function __rpc_call_loaders_Delete(data: null = null): Promise<string> {
return await __rpc_call("/rpc", "Delete", "loaders", data);
}

export async function __rpc_call_loaders_Read(data: ModLoaderType): Promise<Result<ModLoader[], string>> {
return await __rpc_call("/rpc", "Read", "loaders", data);
}

export async function __rpc_call_loaders_Create(data: null = null): Promise<string> {
return await __rpc_call("/rpc", "Create", "loaders", data);
}

export async function __rpc_call_loaders_Update(data: null = null): Promise<string> {
return await __rpc_call("/rpc", "Update", "loaders", data);
}

const __rpc_module_loaders = {
    create: __rpc_call_loaders_Create,
    read: __rpc_call_loaders_Read,
    update: __rpc_call_loaders_Update,
    delete: __rpc_call_loaders_Delete,
};


export async function __rpc_call_mod_Delete(data: number): Promise<Result<Mod, string>> {
return await __rpc_call("/rpc", "Delete", "mod", data);
}

export async function __rpc_call_mod_Read(data: number): Promise<Option<Mod>> {
return await __rpc_call("/rpc", "Read", "mod", data);
}

export async function __rpc_call_mod_Update(data: ModUpdate): Promise<Result<Mod, string>> {
return await __rpc_call("/rpc", "Update", "mod", data);
}

export async function __rpc_call_mod_Create(data: ModCreation): Promise<Result<bigint, string>> {
return await __rpc_call("/rpc", "Create", "mod", data);
}

const __rpc_module_mod = {
    create: __rpc_call_mod_Create,
    read: __rpc_call_mod_Read,
    update: __rpc_call_mod_Update,
    delete: __rpc_call_mod_Delete,
};


export async function __rpc_call_games_Delete(data: number[]): Promise<Result<bigint, string>> {
return await __rpc_call("/rpc", "Delete", "games", data);
}

export async function __rpc_call_games_Create(data: GameCreation[]): Promise<Result<bigint, string>> {
return await __rpc_call("/rpc", "Create", "games", data);
}

export async function __rpc_call_games_Read(data: null = null): Promise<Game[]> {
return await __rpc_call("/rpc", "Read", "games", data);
}

export async function __rpc_call_games_Update(data: null = null): Promise<string> {
return await __rpc_call("/rpc", "Update", "games", data);
}

const __rpc_module_games = {
    create: __rpc_call_games_Create,
    read: __rpc_call_games_Read,
    update: __rpc_call_games_Update,
    delete: __rpc_call_games_Delete,
};


export async function __rpc_call_mods_Delete(data: number[]): Promise<Result<bigint, string>> {
return await __rpc_call("/rpc", "Delete", "mods", data);
}

export async function __rpc_call_mods_Create(data: ModCreation[]): Promise<Result<bigint, string>> {
return await __rpc_call("/rpc", "Create", "mods", data);
}

export async function __rpc_call_mods_Read(data: number): Promise<Mod[]> {
return await __rpc_call("/rpc", "Read", "mods", data);
}

export async function __rpc_call_mods_Update(data: null = null): Promise<string> {
return await __rpc_call("/rpc", "Update", "mods", data);
}

const __rpc_module_mods = {
    create: __rpc_call_mods_Create,
    read: __rpc_call_mods_Read,
    update: __rpc_call_mods_Update,
    delete: __rpc_call_mods_Delete,
};


export async function __rpc_call_game_Create(data: GameCreation): Promise<Result<bigint, string>> {
return await __rpc_call("/rpc", "Create", "game", data);
}

export async function __rpc_call_game_Update(data: GameUpdate): Promise<Result<Game, string>> {
return await __rpc_call("/rpc", "Update", "game", data);
}

export async function __rpc_call_game_Delete(data: number): Promise<Result<Game, string>> {
return await __rpc_call("/rpc", "Delete", "game", data);
}

export async function __rpc_call_game_Read(data: number): Promise<Option<Game>> {
return await __rpc_call("/rpc", "Read", "game", data);
}

const __rpc_module_game = {
    create: __rpc_call_game_Create,
    read: __rpc_call_game_Read,
    update: __rpc_call_game_Update,
    delete: __rpc_call_game_Delete,
};


export async function __rpc_call_sources_Read(data: null = null): Promise<Source[]> {
return await __rpc_call("/rpc", "Read", "sources", data);
}

export async function __rpc_call_sources_Update(data: null = null): Promise<string> {
return await __rpc_call("/rpc", "Update", "sources", data);
}

export async function __rpc_call_sources_Create(data: SourceCreation[]): Promise<Result<bigint, string>> {
return await __rpc_call("/rpc", "Create", "sources", data);
}

export async function __rpc_call_sources_Delete(data: number[]): Promise<Result<bigint, string>> {
return await __rpc_call("/rpc", "Delete", "sources", data);
}

const __rpc_module_sources = {
    create: __rpc_call_sources_Create,
    read: __rpc_call_sources_Read,
    update: __rpc_call_sources_Update,
    delete: __rpc_call_sources_Delete,
};


export async function __rpc_call_plugins_Read(data: null = null): Promise<Result<PluginInfo[], string>> {
return await __rpc_call("/rpc", "Read", "plugins", data);
}

export async function __rpc_call_plugins_Update(data: null = null): Promise<string> {
return await __rpc_call("/rpc", "Update", "plugins", data);
}

export async function __rpc_call_plugins_Create(data: null = null): Promise<string> {
return await __rpc_call("/rpc", "Create", "plugins", data);
}

export async function __rpc_call_plugins_Delete(data: null = null): Promise<string> {
return await __rpc_call("/rpc", "Delete", "plugins", data);
}

const __rpc_module_plugins = {
    create: __rpc_call_plugins_Create,
    read: __rpc_call_plugins_Read,
    update: __rpc_call_plugins_Update,
    delete: __rpc_call_plugins_Delete,
};


export async function __rpc_invoke_version(data: null = null): Promise<string> {
return await __rpc_invoke("/rpc", "version", data);
}

const __rpc_invokers = {
version: __rpc_invoke_version,
};

export type Dirs = { root: string; data: string; cache: string; temp: string }

export type Game = { id: number; name: string; curseforge: number | null; thunderstore: string | null; spacedock: string | null; ckan: boolean; modrinth: boolean }

export type GameCreation = { name: string }

export type GameUpdate = { id: number; name?: string | null; curseforge?: number | null; thunderstore?: string | null; spacedock?: string | null; ckan?: boolean | null; modrinth?: boolean | null }

export type Instance = { id: number; name: string; gameId: number; dataDir: string; cacheDir: string; installDir: string; description: string; created: string; updated: string; loader: string | null }

export type InstanceCreation = { name: string; gameId: number; dataDir: string; cacheDir: string; installDir: string }

export type InstanceUpdate = { id: number; name?: string | null; gameId?: number | null; dataDir?: string | null; cacheDir?: string | null; installDir?: string | null; description?: string | null; created?: string | null; updated?: string | null; loader?: string | null }

export type Mod = { id: number; mod: string; version: string | null; name: string; file: string; size: number; hash: string | null; installed_files: string; sourceId: number; instanceId: number }

export type ModCreation = { mod: string; name: string; file: string; size: number; installed_files: string; sourceId: number; instanceId: number }

/**
 * The ModLoader type.
 * Each element contains the Minecraft version
 * and then its version.
 */
export type ModLoader = { Vanilla: string } | { Forge: [string, string] } | { NeoForge: [string, string] } | { Fabric: [string, string] } | { Quilt: [string, string] } | 
/**
 * This is for any other game, I just didn't feel
 * like dealing with recursive dependencies.
 */
"None"

export type ModLoaderType = "Vanilla" | "Forge" | "NeoForge" | "Fabric" | "Quilt" | "None"

export type ModUpdate = { id: number; mod?: string | null; version?: string | null; name?: string | null; file?: string | null; size?: number | null; hash?: string | null; installed_files?: string | null; sourceId?: number | null; instanceId?: number | null }

/**
 * A plugin's metadata. This is useful for getting information
 * about the plugin on the frontend.
 */
export type PluginInfo = { 
/**
 * The plugin's identifier.
 */
id: string; 
/**
 * The plugin's game ID.
 */
game: number; 
/**
 * The plugin's display name.
 */
display_name: string; 
/**
 * The plugin's icon URL.
 */
icon_url: string; 
/**
 * The plugin's banner URL.
 */
banner_url: string; 
/**
 * The plugin's fallback mod install directory.
 * If the installer can't automatically determine
 * where to install a mod, this will be used.
 */
fallback_dir: string | null; 
/**
 * The plugin's query resolvers (IDs).
 */
resolvers: Source[] }

export type Source = { id: number; name: string }

export type SourceCreation = { name: string }

export type SourceUpdate = { id: number; name?: string | null }

export const RPC = {
latestLoader: __rpc_module_latestLoader,
instance: __rpc_module_instance,
dirs: __rpc_module_dirs,
instances: __rpc_module_instances,
source: __rpc_module_source,
loaders: __rpc_module_loaders,
mod: __rpc_module_mod,
games: __rpc_module_games,
mods: __rpc_module_mods,
game: __rpc_module_game,
sources: __rpc_module_sources,
plugins: __rpc_module_plugins,
invoke: __rpc_invokers,
};

export default RPC;