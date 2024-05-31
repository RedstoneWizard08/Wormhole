import type { ModLoader } from "./bindings/app";

export const getLoader = (loader: ModLoader): string => {
    return Object.keys(loader)[0];
};

export const getMinecraft = (loader: ModLoader): string => {
    return Object.values(loader)[0];
};

export const getLoaderVersion = (loader: ModLoader): string | undefined => {
    return Object.values(loader)[1];
};

export const formatLoader = (loader: ModLoader) => {
    let str = getLoader(loader);

    str += `-${getMinecraft(loader)}`;

    if (getLoaderVersion(loader)) {
        str += `-${getLoaderVersion(loader)}`;
    }

    return str;
};
