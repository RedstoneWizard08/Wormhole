import { Link, useRouter } from "preact-router";
import { useEffect, useState } from "preact/hooks";
import { InstanceInfo, KSPGame } from "../api/instance";
import { invoke_proxy } from "../invoke";
import ksp1logo from "../assets/ksp.png";
import ksp2logo from "../assets/ksp2.png";
import "./Instance.scss";
import { marked } from "marked";
import { createRef } from "preact";

export const Instance = () => {
    const [router] = useRouter();
    const [instances, setInstances] = useState(false);

    useEffect(() => {
        setInstances(/\/instances?(\/\d+)?/i.test(router.path!));
    }, [router.path]);

    const [instanceInfo, setInstanceInfo] = useState<InstanceInfo | undefined>(
        undefined
    );

    const [background, setBackground] = useState<string | undefined>(undefined);
    const [executable, setExecutable] = useState<string | undefined>(undefined);
    const [editing, setEditing] = useState(false);
    const editor = createRef<HTMLTextAreaElement>();

    const id = router.matches?.instance;

    useEffect(() => {
        (async () => {
            const info = await invoke_proxy("get_instance_info", {
                instanceId: parseInt(id || "-1", 10),
            });

            setInstanceInfo(info);
            setBackground(info.game == KSPGame.KSP1 ? ksp1logo : ksp2logo);
            setExecutable(info.install_path);
        })();
    }, [id]);

    const save = async () => {
        if (instanceInfo) {
            setInstanceInfo({
                ...instanceInfo,

                description: editor.current?.value || instanceInfo.description,
            });

            await invoke_proxy("update_description", {
                instanceId: instanceInfo.id,
                description: (editor.current?.value || instanceInfo.description)!,
            });
        }

        setEditing(false);
    };

    const edit = () => {
        setEditing(true);
        
        setTimeout(() => {
            editor.current?.focus();
        });
    };

    const launch = async () => {
        await invoke_proxy("launch", {
            gameId: KSPGame[instanceInfo?.game || "KSP1"] as number,
        });
    };

    return (
        <>
            <div className="full-instance-container">
                <Link
                    className={`link ${instances ? "active" : ""}`}
                    href="/instances">
                    <div className="return-container">
                        <div className="return-arrow">
                            <i className="fa-solid fa-long-arrow-left" />
                        </div>
                        <div className="return-circle" />
                    </div>
                </Link>
                <div className="instance">
                    <img
                        src={background}
                        className="background"
                        alt="background"
                    />

                    <div className="infos">
                        <div className="left">
                            <p className="name">{instanceInfo?.name}</p>
                        </div>

                        <div className="right">
                            <p className="time">
                                <i className="fa-solid fa-clock" />
                                &nbsp;&nbsp;
                                {instanceInfo?.time_played || "0 minutes"}
                            </p>

                            {editing ? (
                                <button
                                    type="button"
                                    className="edit"
                                    onClick={save}>
                                    <i className="fa-solid fa-save" />
                                    &nbsp; Save
                                </button>
                            ) : (
                                <button
                                    type="button"
                                    className="edit"
                                    onClick={edit}>
                                    <i className="fa-solid fa-pencil" />
                                    &nbsp; Edit
                                </button>
                            )}
                        </div>
                    </div>

                    {editing ? (
                        <textarea
                            className="editor"
                            defaultValue={instanceInfo?.description}
                            ref={editor}
                        />
                    ) : (
                        <p
                            className="description"
                            dangerouslySetInnerHTML={{
                                __html: marked(
                                    instanceInfo?.description || "",
                                    {
                                        sanitize: true,
                                    }
                                ),
                            }}
                        />
                    )}
                </div>
            </div>
            <div className="actions">
                <button type="button" className="action" onClick={launch}>
                    <i className="icon fa-solid fa-rocket" />
                    &nbsp; Launch
                </button>

                <p className="executable">{executable}</p>
            </div>
        </>
    );
};
