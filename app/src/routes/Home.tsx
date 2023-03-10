import "./Home.scss";
import banner from "../assets/background_banner.png";

export const Home = () => {
    const doInstall = () => {};

    return (
        <div className="home-container">
            <img src={banner} alt="Space Warp Logo" className="logo" />

            <br />

            <h1 className="title">Welcome to Wormhole</h1>

            <button type="button" className="action" onClick={doInstall}>Install SpaceWarp</button>
        </div>
    );
};
