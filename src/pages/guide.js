import { Link } from 'react-router-dom';
import WalletAddress from '../components/WalletAddress';

const Guide = () => {
    return (
        <main>
            <header>
                <Link to="/" className="home-link">
                    <div className="header-titles">
                        <h2 className="nav-home">⌯Scratch Battle⌯</h2>
                    </div>
                </Link>
            </header>

            <div>
                <h3 className="how-to">How to Play</h3>

                <div>
                    <p className="help">
                        Beat down as many Mangy Cats as you can within 15 seconds! But be careful! These mfs are angry and ready to SCRATCH!
                    </p>
                </div>
            </div>
            <WalletAddress />
        </main>
    );
};

export default Guide;