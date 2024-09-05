import type { ModLoader } from "./bindings/app";

export const getLoader = (loader: ModLoader): string => {
    return Object.keys(loader)[0];
};

export const getMinecraft = (loader: ModLoader): string => {
    switch (getLoader(loader)) {
        case "Vanilla":
            return (loader as any).Vanilla;
        case "Forge":
            return (loader as any).Forge[0];
        case "NeoForge":
            return (loader as any).NeoForge[0];
        case "Fabric":
            return (loader as any).Fabric[0];
        case "Quilt":
            return (loader as any).Quilt[0];
        default:
            return "latest";
    }
};

export const getLoaderVersion = (loader: ModLoader): string | undefined => {
    return Object.values(loader)[0][1];
};

export const formatLoader = (loader: ModLoader) => {
    let str = getLoader(loader);

    str += `-${getMinecraft(loader)}`;

    if (getLoaderVersion(loader)) {
        str += `-${getLoaderVersion(loader)}`;
    }

    return str;
};
