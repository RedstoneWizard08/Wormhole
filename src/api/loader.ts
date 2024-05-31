import type { ModLoader } from "./bindings/app";

export const getLoader = (loader: ModLoader): string => {
    return Object.keys(loader)[0];
};

export const getMinecraft = (loader: ModLoader): string => {
    switch (getLoader(loader)) {
        case "Vanilla":
            return loader["Vanilla"];
        case "Forge":
            return loader["Forge"][0];
        case "NeoForge":
            return loader["NeoForge"][0];
        case "Fabric":
            return loader["Fabric"][0];
        case "Quilt":
            return loader["Quilt"][0];
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
