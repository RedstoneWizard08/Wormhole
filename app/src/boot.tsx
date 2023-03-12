import { invoke_proxy } from "./invoke";

export async function backend_boot() {
    console.log("Backend booting...");
    await invoke_proxy("backend_boot");
    console.log("Backend booted.");
}