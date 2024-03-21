export interface Plugin {
    id: number;
    name: string;
    display: string;
    icon: string;
    banner: string;
    caps: PluginCaps;
    settings: Record<string, any>;
}

export interface PluginCaps {
    multiple_instances: boolean;
    mods: boolean;
}
