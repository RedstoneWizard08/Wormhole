         // This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

         export const commands = {
async info() : Promise<__Result__<{ id: string; game: number; display_name: string; icon_url: string; banner_url: string; fallback_dir: string | null; resolvers: SourceMapping[] }, boolean>> {
try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:KSP1|info") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async searchMods(resolver: SourceMapping, query: string | null, opts: { page: number; count: number } | null) : Promise<__Result__<{ data: Mod[]; page: number | null; per_page: number | null }, boolean>> {
try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:KSP1|search_mods", { resolver, query, opts }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async getMod(resolver: SourceMapping, id: string) : Promise<__Result__<{ id: string; game_id: number | null; versions: ModVersion[]; name: string; source: number; icon: string | null }, boolean>> {
try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:KSP1|get_mod", { resolver, id }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async getModVersions(resolver: SourceMapping, id: string) : Promise<__Result__<({ id: string; name: string | null; file_name: string | null; size: string | null; hash: string | null; url: string | null })[], boolean>> {
try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:KSP1|get_mod_versions", { resolver, id }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async getModVersion(resolver: SourceMapping, id: string, version: string) : Promise<__Result__<{ id: string; name: string | null; file_name: string | null; size: string | null; hash: string | null; url: string | null }, boolean>> {
try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:KSP1|get_mod_version", { resolver, id, version }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async getDownloadUrl(resolver: SourceMapping, project: string, version: string | null) : Promise<__Result__<string, boolean>> {
try {
    return { status: "ok", data: await TAURI_INVOKE("plugin:KSP1|get_download_url", { resolver, project, version }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
}
}



/** user-defined types **/

export type Mod = { 
/**
 * The mod's ID in its source.
 * This could be an integer or a string,
 * and since we support multiple platforms,
 * a string is the most flexible.
 */
id: string; 
/**
 * The game ID.
 */
game_id: number | null; 
/**
 * The mod's versions.
 */
versions: ModVersion[]; 
/**
 * The mod's name.
 */
name: string; 
/**
 * Where the mod came from.
 * This is a reference to a source in the database.
 */
source: number; 
/**
 * The mod's icon.
 */
icon: string | null }
export type ModVersion = { 
/**
 * The version ID.
 */
id: string; 
/**
 * The version name. Some sources may not have this.
 */
name: string | null; 
/**
 * The file name.
 */
file_name: string | null; 
/**
 * The size in bytes of the file.
 */
size: string | null; 
/**
 * The SHA-512 hash of the file.
 */
hash: string | null; 
/**
 * The download URL.
 */
url: string | null }
export type SourceMapping = "SpaceDock" | "Ckan" | "Wormhole" | "Local" | "CurseForge" | "Modrinth" | "Thunderstore" | "Nexus" | "Unknown"

/** tauri-specta globals **/

         import { invoke as TAURI_INVOKE } from "@tauri-apps/api/tauri";
import * as TAURI_API_EVENT from "@tauri-apps/api/event";
import { type WebviewWindowHandle as __WebviewWindowHandle__ } from "@tauri-apps/api/window";

type __EventObj__<T> = {
  listen: (
    cb: TAURI_API_EVENT.EventCallback<T>
  ) => ReturnType<typeof TAURI_API_EVENT.listen<T>>;
  once: (
    cb: TAURI_API_EVENT.EventCallback<T>
  ) => ReturnType<typeof TAURI_API_EVENT.once<T>>;
  emit: T extends null
    ? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit>
    : (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>;
};

export type __Result__<T, E> =
  | { status: "ok"; data: T }
  | { status: "error"; error: E };

function __makeEvents__<T extends Record<string, any>>(
  mappings: Record<keyof T, string>
) {
  return new Proxy(
    {} as unknown as {
      [K in keyof T]: __EventObj__<T[K]> & {
        (handle: __WebviewWindowHandle__): __EventObj__<T[K]>;
      };
    },
    {
      get: (_, event) => {
        const name = mappings[event as keyof T];

        return new Proxy((() => {}) as any, {
          apply: (_, __, [window]: [__WebviewWindowHandle__]) => ({
            listen: (arg: any) => window.listen(name, arg),
            once: (arg: any) => window.once(name, arg),
            emit: (arg: any) => window.emit(name, arg),
          }),
          get: (_, command: keyof __EventObj__<any>) => {
            switch (command) {
              case "listen":
                return (arg: any) => TAURI_API_EVENT.listen(name, arg);
              case "once":
                return (arg: any) => TAURI_API_EVENT.once(name, arg);
              case "emit":
                return (arg: any) => TAURI_API_EVENT.emit(name, arg);
            }
          },
        });
      },
    }
  );
}

     