export const SNAPSHOT_REGEX = /\d+w\d+[A-Za-z0-9_]+/gm;

export const isSnapshot = (ver: string) => SNAPSHOT_REGEX.test(ver);

export const isPreRelease = (ver: string) =>
    ver.toLowerCase().includes("-pre") || ver.includes("Pre-Release");

export const isReleaseCandidate = (ver: string) => ver.includes("-rc");
export const isRubyDung = (ver: string) => ver.startsWith("rd-");
export const isClassic = (ver: string) => ver.startsWith("c");
export const isInfDev = (ver: string) => ver.startsWith("inf-");
export const isAlpha = (ver: string) => ver.startsWith("a");
export const isBeta = (ver: string) => ver.startsWith("b");

export const isRelease = (ver: string) =>
    !isSnapshot(ver) &&
    !isPreRelease(ver) &&
    !isReleaseCandidate(ver) &&
    !isRubyDung(ver) &&
    !isClassic(ver) &&
    !isInfDev(ver) &&
    !isAlpha(ver) &&
    !isBeta(ver);
