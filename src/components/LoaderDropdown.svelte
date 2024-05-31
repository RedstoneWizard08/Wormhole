<script lang="ts">
import type { ModLoader } from "$api/bindings/app";
import { getMinecraft } from "$api/loader";
    import { onMount } from "svelte";

let vanillaVer: string | undefined;
let vanilla: ModLoader[] = [];
let forge: ModLoader[] = [];
let neoforge: ModLoader[] = [];
let fabric: ModLoader[] = [];
let quilt: ModLoader[] = [];

$: compatibleForge = forge.filter((v) => getMinecraft(v) === vanillaVer);
$: compatibleNeoForge = neoforge.filter((v) => getMinecraft(v) === vanillaVer);

onMount(async () => {});
</script>

<style scoped lang="scss">
    
</style>
