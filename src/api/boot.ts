import { commands } from "./bindings/app";
import { plugins } from "./stores";

export const backend_boot = async () => {
    console.log("Backend booting...");

    plugins.set(await commands.getPlugins());
};
