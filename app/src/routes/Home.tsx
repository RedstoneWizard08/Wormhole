import "./Home.scss";
import banner from "../assets/background_banner.png";

export const Home = () => {
    return (
        <div className="home-container">
            <img src={banner} alt="Space Warp Logo" className="logo" />

            <br />

            <h1 className="title">Welcome to the Space Warp Installer</h1>
        </div>
    );
};
