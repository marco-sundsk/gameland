import { connect, Contract, keyStores, WalletConnection } from 'near-api-js'
import getConfig from './config'

const nearConfig = getConfig('development')

console.log(nearConfig)

// Initialize contract & set global variables
export async function initContract() {
  // Initialize connection to the NEAR testnet
  const near = await connect(Object.assign({ deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() } }, nearConfig))

  // Initializing Wallet based Account. It can work with NEAR testnet wallet that
  // is hosted at https://wallet.testnet.near.org
  window.walletConnection = new WalletConnection(near)

  // Getting the Account ID. If still unauthorized, it's just empty string
  window.accountId = window.walletConnection.getAccountId()

  // Initializing our contract APIs by contract name and configuration
  window.contract = await new Contract(window.walletConnection.account(), nearConfig.contractName, {
    // View methods are read only. They don't modify the state, but usually return some value.
    viewMethods: ['get_greeting','get_account_dice_count','get_win_history','get_contract_info'],
    // Change methods can modify the state. But you don't receive the returned value when called.
    changeMethods: ['set_greeting','roll_dice','buy_dice'],
  })

  // platform contract
  window.contract_platform = await new Contract(window.walletConnection.account(), gameland.testnet, {
    // View methods are read only. They don't modify the state, but usually return some value.
    viewMethods: ['list_registers','list_shops','metadata','get_shop', 'get_register'],
    // Change methods can modify the state. But you don't receive the returned value when called.
    changeMethods: ['buy_playtoken', 'sell_playtoken', 'play','sponsor','register_shop', 'resovle_register'],
  })

  // gamecoin contract
  window.contract_gamecoin = await new Contract(window.walletConnection.account(), playtoken.testnet, {
    // View methods are read only. They don't modify the state, but usually return some value.
    viewMethods: ['ft_balance_of', 'ft_total_supply'],
  })

  // game contract
  window.contract_game = await new Contract(window.walletConnection.account(), neardice.testnet, {
    // View methods are read only. They don't modify the state, but usually return some value.
    viewMethods: ['gl_metadata', 'gl_pub_state', 'gl_user_state', 'get_win_history','get_contract_info'],
  })
}

export function logout() {
  window.walletConnection.signOut()
  // reload page
  window.location.replace(window.location.origin + window.location.pathname)
}

export function login() {
  // Allow the current app to make calls to the specified contract on the
  // user's behalf.
  // This works by creating a new access key for the user's account and storing
  // the private key in localStorage.
  window.walletConnection.requestSignIn("gameland.testnet")
}
