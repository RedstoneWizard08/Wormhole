import { get } from "svelte/store";
import { plugins } from "./stores";
import { RPC, unwrap } from "./bindings/app";

export const boot = async () => {
    console.log("Backend booting...");
    console.log(`Using RPC version ${await RPC.invoke.version()}`);

    plugins.set(unwrap(await RPC.plugins.read()).sort((a, b) => a.game - b.game));

    console.log(get(plugins));
};
