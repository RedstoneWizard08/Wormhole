export interface Instance {
    id?: number;
    name: string;
}

export interface InstanceMeta {
    id?: number;
    instance_id: number;
    game_id: number;
    data_dir: string;
    cache_dir: string;
    description: string;
    created: BigInt;
    updated: BigInt;
}
