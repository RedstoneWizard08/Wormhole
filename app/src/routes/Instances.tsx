import "./Instances.scss";
import { InstanceInfo, KSPGame } from "../api/instance";
import { Instance } from "../components/Instance";
import { useEffect, useState } from "preact/hooks";
import { invoke_proxy } from "../invoke";
import { Dropdown } from "../components/Dropdown";
import { gameItems } from "./mods/Browse";
import { open } from "@tauri-apps/api/dialog";
import { appDir } from "@tauri-apps/api/path";

export const Instances = () => {
    const [adding, setAdding] = useState(false);
    const [deleteing, setDeleteing] = useState(true);
    const [instances, setInstances] = useState<InstanceInfo[]>([]);
    const [instanceToDelete, setInstanceToDelete] =
        useState<InstanceInfo | null>(null);

    const [game, setGame] = useState("ksp1");
    const [gameText, setGameText] = useState("KSP 1");

    const [path, setPath] = useState("C:\\Fakepath");
    const [name, setName] = useState("");

    const [gameId, setGameId] = useState(KSPGame.KSP1);

    useEffect(() => {
        setGameId(game == "ksp1" ? KSPGame.KSP1 : KSPGame.KSP2);
    }, [game]);

    const refreshInstances = async () => {
        const data = await invoke_proxy("get_instances", undefined);

        setInstances(data);
    };

    useEffect(() => {
        refreshInstances().then((r) => r);
    }, []);

    const addInstance = async () => {
        await invoke_proxy("add_instance", {
            gameId,
            name,
            installPath: path,
        });

        setAdding(false);
    };

    const selectFolder = async () => {
        if (import.meta.env.TAURI_WEB_DEV) {
            const handle = await window.showDirectoryPicker({
                mode: "read",
            });

            setPath(handle.name);
        } else {
            const dir = await open({
                directory: true,
                defaultPath: await appDir(),
            });

            setPath(dir as string);
        }
    };

    const deleteInstance = async () => {
        if (instanceToDelete)
            await invoke_proxy("delete_instance", {
                instanceId: instanceToDelete.id,
            });
    };

    return (
        <>
            {adding ? (
                <div className="add-modal-background">
                    <div className="add-modal">
                        <div className="modal-header">
                            <span className="title">Add Instance</span>

                            <i
                                className="fa-solid fa-times close"
                                onClick={() => setAdding(!adding)}
                            />
                        </div>

                        <Dropdown
                            items={gameItems}
                            val={game}
                            setVal={setGame}
                            valText={gameText}
                            setValText={setGameText}
                        />

                        <input
                            type="text"
                            placeholder="Instance name"
                            className="name"
                            value={name}
                            onKeyDown={(e) =>
                                setName((e.target as HTMLInputElement).value)
                            }
                            onChange={(e) =>
                                setName((e.target as HTMLInputElement).value)
                            }
                        />

                        <div className="select-dir">
                            <button
                                type="button"
                                className="select-dir-button"
                                onClick={selectFolder}>
                                Choose a folder...
                            </button>

                            <p className="select-dir-text">{path}</p>
                        </div>

                        <button
                            type="button"
                            className="submit-button"
                            onClick={addInstance}>
                            Continue
                        </button>
                    </div>
                </div>
            ) : (
                <></>
            )}

            {deleteing ? (
                <div className="add-modal-background">
                    <div className="add-modal">
                        <div className="modal-header">
                            <span className="title">Delete Instance</span>

                            <i
                                className="fa-solid fa-times close"
                                onClick={() => setDeleteing(!deleteing)}
                            />
                        </div>

                        <p className="delete-text">
                            Are you sure you want to delete the instance "
                            {instanceToDelete?.name}"?
                        </p>

                        <button
                            type="button"
                            className="yes-button"
                            onClick={deleteInstance}>
                            Yes
                        </button>

                        <button
                            type="button"
                            className="no-button"
                            onClick={() => setDeleteing(!deleteing)}>
                            No
                        </button>
                    </div>
                </div>
            ) : (
                <></>
            )}

            <div className="instances-wrapper">
                <button
                    className="add-instance-button"
                    onClick={() => setAdding(!adding)}>
                    <i className="fa-solid fa-plus" />
                </button>

                <div className="instances-container">
                    {Array.isArray(instances) &&
                        instances.map((info) => (
                            <Instance
                                data={info}
                                key={info.name}
                                setInstanceToDelete={setInstanceToDelete}
                                deleteing={deleteing}
                                setDeleteing={setDeleteing}
                            />
                        ))}
                </div>
            </div>
        </>
    );
};
