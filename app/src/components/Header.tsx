import "./Header.scss";
import logo from "../assets/icon.png";
import { Link, useRouter } from "preact-router";
import { useEffect, useState } from "preact/hooks";

export const Header = () => {
    const [router] = useRouter();

    const [instances, setInstances] = useState(false);
    const [mods, setMods] = useState(false);
    const [manage, setManage] = useState(false);
    const [spacewarp, setSpacewarp] = useState(false);
    const [settings, setSettings] = useState(false);

    useEffect(() => {
        setMods(/\/mods?(\/\d+)?/i.test(router.path!));
        setInstances(/\/instances?(\/\d+)?/i.test(router.path!));

        setManage(router.path == "/manage");
        setSpacewarp(router.path == "/spacewarp" || router.path == "/install");
        setSettings(router.path == "/settings");
    }, [router.path]);

    return (
        <div className="header">
            <Link className="logo-link" href="/">
                <img
                    className="logo"
                    src={logo}
                    alt="insert wormhole logo here"
                />
            </Link>

            <Link className={`link ${spacewarp ? "active" : ""}`} href="/spacewarp">
                <i className="icon fa-solid fa-rocket" />
                SpaceWarp
            </Link>

            <Link
                className={`link ${instances ? "active" : ""}`}
                href="/instances">
                <i className="icon fa-solid fa-download" />
                Instances
            </Link>

            <Link className={`link ${mods ? "active" : ""}`} href={"/mods"}>
                <i className="icon fa-solid fa-search" />
                Browse Mods
            </Link>

            <Link className={`link ${manage ? "active" : ""}`} href={"/manage"}>
                <i className="icon fa-solid fa-sliders" />
                Manage Mods
            </Link>

            <Link className={`link ${settings ? "active" : ""}`} href={"/settings"}>
                <i className="icon fa-solid fa-gear" />
                Settings
            </Link>
        </div>
    );
};
