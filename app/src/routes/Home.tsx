import "./Home.scss";
import banner from "../assets/background_banner.png";

export const Home = () => {
    return (
        <div class="home-container">
            <img src={banner} alt="Space Warp Logo" class="logo" />

            <br />

            <h1 class="title">Welcome to the Space Warp Installer</h1>
        </div>
    );
};
