import { get } from "svelte/store";
import { commands } from "./bindings/app";
import { plugins } from "./stores";
import { unwrap } from "./util";

export const boot = async () => {
    console.log("Backend booting...");

    plugins.set(unwrap(await commands.getPlugins(null)).sort((a, b) => a.game - b.game));

    console.log(get(plugins));
};
