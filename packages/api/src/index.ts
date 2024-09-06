import {
    InstalledModCreation,
    InstalledModUpdate,
    InstanceCreation,
    InstanceUpdate,
    Option,
    RPC,
    Result,
} from "./bindings";

export * from "./bindings";

const toNullable = <T, E = any>(o: Option<T> | Result<T, E>): T | undefined => {
    if ("Some" in o) {
        return o.Some;
    } else if ("Ok" in o) {
        return o.Ok;
    } else {
        return undefined;
    }
};

export class Wormhole {
    // ============= Instances =============

    public static async getInstances(game: number) {
        return await RPC.instances.read(game);
    }

    public static async getInstance(id: number) {
        return toNullable(await RPC.instance.read(id));
    }

    public static async createInstance(data: InstanceCreation) {
        return toNullable(await RPC.instance.create(data));
    }

    public static async updateInstance(data: InstanceUpdate) {
        return toNullable(await RPC.instance.update(data));
    }

    public static async deleteInstance(id: number) {
        return toNullable(await RPC.instance.delete(id));
    }

    // ============= Mods =============

    public static async getMods(instance: number) {
        return await RPC.mods.read(instance);
    }

    public static async getMod(id: number) {
        return toNullable(await RPC.mod.read(id));
    }

    public static async createMod(data: InstalledModCreation) {
        return toNullable(await RPC.mod.create(data));
    }

    public static async updateMod(data: InstalledModUpdate) {
        return toNullable(await RPC.mod.update(data));
    }

    public static async deleteMod(id: number) {
        return toNullable(await RPC.mod.delete(id));
    }

    // ============= Utils =============

    public static async getAppDir() {
        return await RPC.invoke.dataDir();
    }

    public static async getVersion() {
        return await RPC.invoke.version();
    }

    public static async gameDir(game: string) {
        return await RPC.invoke.gameDir(game);
    }
}
