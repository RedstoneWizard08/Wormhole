import "./Header.scss";
import logo from "../assets/icon.png";
import { Link, useRouter } from "preact-router";
import { useEffect, useState } from "preact/hooks";

export const Header = () => {
    const [router] = useRouter();

    const [instances, setInstances] = useState(false);
    const [mods, setMods] = useState(false);
    const [manage, setManage] = useState(false);

    useEffect(() => {
        setInstances(router.path == "/instances");
        setMods(router.path == "/mods");
        setManage(router.path == "/manage");
    }, [router.path]);

    return (
        <div className="header">
            <img className="logo" src={logo} />

            <Link href="/" className="title-wrapper">
                <p className="title">SpaceWarp</p>
            </Link>

            <Link
                className={`link ${instances ? "active" : ""}`}
                href="/instances">
                <i className="icon fa-solid fa-rocket" />
                Instances
            </Link>

            <Link className={`link ${mods ? "active" : ""}`} href="/mods">
                <i className="icon fa-solid fa-search" />
                Browse Mods
            </Link>

            <Link className={`link ${manage ? "active" : ""}`} href="/manage">
                <i className="icon fa-solid fa-cog" />
                Manage Mods
            </Link>
        </div>
    );
};
