import { invoke_proxy } from "./invoke";

export const backend_boot = async () => {
    console.log("Backend booting...");

    await invoke_proxy("backend_boot", undefined);

    console.log("Backend booted.");

    const mods = await invoke_proxy("read_mod_json", undefined);

    console.log("Mods:", mods);
};
