import React, { useEffect, useState } from "react";
import { initNear } from "./near";

function App() {
  // State variables to store near, contract, and accountId
  const [near, setNear] = useState(null);
  const [contract, setContract] = useState(null);
  const [accountId, setAccountId] = useState(null);

  // Initialize the NEAR connection on component mount
  useEffect(() => {
    async function init() {
      const { near, contract, accountId } = await initNear();
      setNear(near);
      setContract(contract);
      setAccountId(accountId);
    }
    init();
  }, []);

  // Login function to authenticate the user
  async function login() {
    if (!near) return;
    await near.walletConnection.requestSignIn(nearConfig.contractName, "NEAR Tic Tac Toe");
  }

  // Logout function to sign out the user
  async function logout() {
    if (!near) return;
    near.walletConnection.signOut();
    setAccountId(null);
  }

  // Function to start a new game for the logged-in user
  async function startGame() {
    if (!contract || !accountId) return;
    await contract.new({ account_id_str: accountId });
  }

  // Function to fetch the current game state
  async function fetchGameState() {
    if (!contract) return;
    const gameState = await contract.get_game_state();
    console.log("Game State:", gameState);
  }

  // Function to increase the user's prestige level
  async function increasePrestige() {
    if (!contract) return;
    await contract.increase_prestige();
  }

  // Function to spread love and handle the boss fight
  async function spreadLove() {
    if (!contract) return;
    await contract.spread_love();
  }

  // Render your components and use the methods as needed
  return (
    <div className="App">
      {/* Show buttons based on user login status */}
      {accountId ? (
        <div>
          <p>Logged in as {accountId}</p>
          <button onClick={logout}>Logout</button>
          <button onClick={startGame}>Start Game</button>
          <button onClick={fetchGameState}>Fetch Game State</button>
          <button onClick={increasePrestige}>Increase Prestige</button>
          <button onClick={spreadLove}>Spread Love</button>
        </div>
      ) : (
        <button onClick={login}>Login</button>
      )}
    </div>
  );
}

export default App;
