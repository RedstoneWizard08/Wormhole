import { get } from "svelte/store";
import { plugins } from "./stores";
import { RPC, unwrap } from "./bindings/app";

export const boot = async () => {
    console.log("Backend booting...");

    plugins.set(unwrap(await RPC.plugins.read(null)).sort((a, b) => a.game - b.game));

    console.log(get(plugins));
};
