import { invoke_proxy } from "./invoke";

export async function backend_boot() {
    console.log("Backend booting...");

    await invoke_proxy("backend_boot", undefined);
    
    console.log("Backend booted.");

    let mods = await invoke_proxy("read_mod_json", undefined);
    
    console.log("Mods:", mods);
}
