import { setupTauri } from "./bindings/app";
import { boot } from "./boot";

export const setup = async () => {
    if (import.meta.env.TAURI_WEB_DEV) {
        const eruda = (await import("eruda")).default;

        eruda.init();
        eruda.position({ x: 10, y: window.innerHeight - 45 });
    }

    setupTauri();
    await boot();
};
