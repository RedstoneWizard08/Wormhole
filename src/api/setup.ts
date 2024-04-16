import { boot } from "./boot";
import { createMockAPI } from "./dev";

export const setup = async () => {
    if (import.meta.env.TAURI_WEB_DEV) {
        const eruda = (await import("eruda")).default;

        eruda.init();
        eruda.position({ x: 10, y: window.innerHeight - 45 });

        createMockAPI();
    }

    await boot();
};
