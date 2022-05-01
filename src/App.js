import './App.css';
// Bring in the required hooks and possible wallet states
import { useWallet, WalletStatus } from "@terra-money/wallet-provider";

// Import the Menu file from the components directory
import Menu from './components/Menu';
import WalletAddress from './components/WalletAddress';

function App() {
  // Current wallet status, connect & disconnect functions, available connections
  const { status, connect, disconnect, availableConnectTypes } = useWallet();

  // Let's take a look at what the starting states are
  console.log("Wallet status is ", status);
  console.log("Available connection types: ", availableConnectTypes);

  const renderConnectButton = () => {
    if (status === WalletStatus.WALLET_NOT_CONNECTED) {
      return (
        <div className="connect-wallet-div">
          <button
            type="button"
            key={`connect-EXTENSION`}
            onClick={() => connect("EXTENSION")}
            className="cta-button connect-wallet-button"
          >
            Connect Wallet
          </button>
        </div>
      );
    }
    // Check if wallet is connected
    else if (status === WalletStatus.WALLET_CONNECTED) {
      return (
        <button
          type="button"
          onClick={() => disconnect()}
          className="cta-button, connect-wallet-button"
        >
          Disconnect
        </button>
      );
    }
  };

  return (
    <main className="App">
      <header>
        <div className="header-titles">
          <h2>⌯Scratch Battle⌯</h2>
          {/* <p>featuring</p> */}
        </div>
        <WalletAddress />
      </header>

      {/* If not connected, show the Mangy Cats gif */}
      {status === WalletStatus.WALLET_NOT_CONNECTED && (
        <div>
          <img
            src="https://media.giphy.com/media/N7JID0JtA2tmarMRK0/giphy.gif"
            alt="cat with attitude gif"
          />
        </div>
      )}

      {/* Show the menu after connection */}
      {status === WalletStatus.WALLET_CONNECTED && (
        <div className="game-menu-container">
          <Menu />
        </div>
      )}

      {renderConnectButton()}
    </main>
  );
}

export default App;
