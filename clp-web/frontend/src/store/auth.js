import { writable } from 'svelte/store';
import { idlFactory } from '../../declarations/vault/vault.did.js';
import { Actor, HttpAgent } from '@dfinity/agent';

/**
 * Creates an actor for the Backend canister
 *
 * @param {{agentOptions: import("@dfinity/agent").HttpAgentOptions, actorOptions: import("@dfinity/agent").ActorConfig}} options
 * @returns {import("@dfinity/agent").ActorSubclass<import("../../../declarations/defi_dapp/defi_dapp.did")._SERVICE>}
 */
export function createActor(options) {
  const hostOptions = {
    host: process.env.DFX_NETWORK === 'ic' ? `https://${process.env.VAULT_CANISTER_ID}.ic0.app` : 'http://localhost:4944',
  };
  if (!options) {
    options = {
      agentOptions: hostOptions,
    };
  } else if (!options.agentOptions) {
    options.agentOptions = hostOptions;
  } else {
    options.agentOptions.host = hostOptions.host;
  }

  const agent = new HttpAgent({ ...options.agentOptions });

  // Fetch root key for certificate validation during development
  if (process.env.DFX_NETWORK === 'local') {
    console.log('fetchRootKey');
    agent.fetchRootKey().catch((err) => {
      console.warn('Unable to fetch root key. Check to ensure that your local replica is running');
      console.error(err);
    });
  }

  // Creates an actor with using the candid interface and the HttpAgent
  return Actor.createActor(idlFactory, {
    agent,
    canisterId: process.env.VAULT_CANISTER_ID,
    ...options?.actorOptions,
  });
}

export const auth = writable({
  loggedIn: false,
  principal: '',
  actor: createActor(),
});

export const VAULT_CANISTER_ID = process.env.VAULT_CANISTER_ID;
export const LEDGER_CANISTER_ID = process.env.LEDGER_CANISTER_ID;
export const CKBTC_CANISTER_ID = process.env.CKBTC_CANISTER_ID;
export const CKETH_CANISTER_ID = process.env.CKETH_CANISTER_ID;
export const CUSD_CANISTER_ID = process.env.CUSD_CANISTER_ID;
export const POOL_CANISTER_ID = process.env.RESERVE-POOL_CANISTER_ID;
export const STAKING_CANISTER_ID = process.env.STAKING_CANISTER_ID;


export const CLPT_CANISTER_ID = process.env.CLPT_CANISTER_ID;

export const whitelist = [VAULT_CANISTER_ID, LEDGER_CANISTER_ID,CKBTC_CANISTER_ID,CKETH_CANISTER_ID,CUSD_CANISTER_ID];

export const host = process.env.DFX_NETWORK === 'ic' ? `https://ic0.app` : `http://localhost:4944`;

export const plugWallet = writable({
  extensionInstalled: false,
  isConnected: false,
  whiteList: whitelist,
  host: host,
  principal: '',
  plugActor: null,
  plugAkitaActor: null,
  plugGoldenActor: null,
  plugLedgerActor: null,
});

export const anonymous = writable({
  actor: createActor(),
});
export const verifyConnection = async () => {
  console.log({host})
  const connected = await window.ic.plug.isConnected();
  if (!connected) await window.ic.plug.requestConnect({ whitelist, host });
};
