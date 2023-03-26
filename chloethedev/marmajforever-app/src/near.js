import { connect, Contract, keyStores, WalletConnection } from "near-api-js";
import nearConfig from "./nearConfig";

// Initialize connection to the NEAR Protocol
export async function initNear() {
  const near = await connect(
    Object.assign({ deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() } }, nearConfig)
  );
  const walletConnection = new WalletConnection(near);
  const accountId = walletConnection.getAccountId();
  const contract = new Contract(walletConnection.account(), nearConfig.contractName, {
    viewMethods: ["get_game_state", "get_love", "get_prestige", "get_game_time", "get_boss"],
    changeMethods: ["new", "update_game_time", "increase_prestige", "spread_love"],
  });

  return { near, walletConnection, accountId, contract };
}
