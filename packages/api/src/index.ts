import {
    InstalledModCreation,
    InstalledModUpdate,
    InstanceCreation,
    InstanceUpdate,
    Option,
    RPC,
    Result,
} from "./bindings";

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
    public static INSTANCE = new Wormhole();

    // ============= Instances =============

    public async getInstances() {
        return await RPC.instances.read();
    }

    public async getInstance(id: number) {
        return toNullable(await RPC.instance.read(id));
    }

    public async createInstance(data: InstanceCreation) {
        return toNullable(await RPC.instance.create(data));
    }

    public async updateInstance(data: InstanceUpdate) {
        return toNullable(await RPC.instance.update(data));
    }

    public async deleteInstance(id: number) {
        return toNullable(await RPC.instance.delete(id));
    }

    // ============= Mods =============

    public async getMods(instance: number) {
        return await RPC.mods.read(instance);
    }

    public async getMod(id: number) {
        return toNullable(await RPC.mod.read(id));
    }

    public async createMod(data: InstalledModCreation) {
        return toNullable(await RPC.mod.create(data));
    }

    public async updateMod(data: InstalledModUpdate) {
        return toNullable(await RPC.mod.update(data));
    }

    public async deleteMod(id: number) {
        return toNullable(await RPC.mod.delete(id));
    }
}
