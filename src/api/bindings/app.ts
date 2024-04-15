         // This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

         export const commands = {
async launch(instanceId: number) : Promise<null> {
return await TAURI_INVOKE("launch", { instanceId });
},
async getInstances(gameId: number) : Promise<__Result__<({ id: number | null; name: string; game_id: number; data_dir: string; cache_dir: string; install_dir: string; description: string; created: number; updated: number })[], boolean>> {
try {
    return { status: "ok", data: await TAURI_INVOKE("get_instances", { gameId }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async getInstance(instanceId: number) : Promise<__Result__<{ id: number | null; name: string; game_id: number; data_dir: string; cache_dir: string; install_dir: string; description: string; created: number; updated: number }, boolean>> {
try {
    return { status: "ok", data: await TAURI_INVOKE("get_instance", { instanceId }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async getDistance(modName: string, query: string) : Promise<__Result__<number, string>> {
try {
    return { status: "ok", data: await TAURI_INVOKE("get_distance", { modName, query }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async getPlugins() : Promise<PluginInfo[]> {
return await TAURI_INVOKE("get_plugins");
}
}



/** user-defined types **/

export type PluginInfo = { id: string; game: number; display_name: string; icon_url: string; banner_url: string; fallback_dir: string | null; resolvers: SourceMapping[] }
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

     