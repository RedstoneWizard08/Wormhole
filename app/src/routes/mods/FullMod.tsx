import "./FullMod.scss";
import { useRouter } from "preact-router";
import { useEffect, useState } from "preact/hooks";
import { finishModInfo, ModInfo } from "../../components/Mod";
import axios from "axios";

const apiUrl = import.meta.env.DEV
    ? "/_spacedock"
    : "https://spacedock.info/api";

export const FullMod = () => {
    const [router, _] = useRouter();
    const modId = router.matches?.mod;

    const [modInfo, setModInfo] = useState<ModInfo | undefined>(undefined);

    const refreshModInfo = async () => {
        const { data } = await axios.get<Partial<ModInfo>>(`${apiUrl}/mod/${modId}`);

        setModInfo(finishModInfo(data, true));
    };

    useEffect(() => {
        (async () => {
            await refreshModInfo();
        })();
    }, [modId]);

    return (
        <div className="full-mod-container">
            <img src={modInfo?.background} className="background" />
        </div>
    );
};
