import { writable } from 'svelte/store';

export const canisters = writable(
    [
        // {symbol: 'AKI', canisterName: 'AkitaDIP20', canisterId: process.env.AKITADIP20_CANISTER_ID},
        // {symbol: 'GLD', canisterName: 'GoldenDIP20', canisterId: process.env.GOLDENDIP20_CANISTER_ID},
        {symbol: 'ICP', canisterName: 'ICP', canisterId: process.env.LEDGER_CANISTER_ID},
        {symbol: 'vault', canisterName: 'vault', canisterId: process.env.VAULT_CANISTER_ID}
    ]
);

export const isConnected = writable();
export const iiPrincipal = writable("");
export const userPrincipal = writable("");

export const ckbtcActor = writable();
export const actorList = writable({
    cusdActor:{},
    ckethActor:{},
    ckbtcActor:{},
    poolActor:{},
    vaultActor:{},
    stakeActor:{},
    clptActor:{},
    isGetActor:false
})
