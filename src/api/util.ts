export const formatBytes = (n: number) => {
    const k = n > 0 ? Math.floor(Math.log2(n) / 10) : 0;
    const rank = (k > 0 ? "KMGTPE"[k - 1] : "") + "b";
    const count = (n / Math.pow(1024, k)).toFixed(1);
    return count + " " + rank;
};
