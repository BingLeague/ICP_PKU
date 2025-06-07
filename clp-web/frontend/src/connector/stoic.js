// Import necessary modules
import { StoicIdentity } from "ic-stoic-identity";
import { Actor, HttpAgent } from "@dfinity/agent";
import {
    CKBTC_CANISTER_ID,
    CKETH_CANISTER_ID,
    CUSD_CANISTER_ID,
    LEDGER_CANISTER_ID, POOL_CANISTER_ID,
    VAULT_CANISTER_ID
} from "../store/auth";
const whitelist = [
    VAULT_CANISTER_ID,
    LEDGER_CANISTER_ID,
    CKBTC_CANISTER_ID,
    CKETH_CANISTER_ID,
    CUSD_CANISTER_ID,
    POOL_CANISTER_ID
]
export class StoicConnector {
    constructor(config) {
        this.config = {
            whitelist,
            host: config.host,
            providerUrl: "https://www.stoicwallet.com",
            dev: false,
        };
        this.identity = undefined;
        this.principal = undefined;
        this.type = undefined
    }

    get getPrincipal() {
        return this.principal;
    }

    async init() {
        const identity = await StoicIdentity.load(this.config.providerUrl);
        console.log(identity)
        if (identity) {
            this.identity = identity;
            this.principal = identity.getPrincipal().toText();
        }

        return true;
    }

    async createActor({ canisterId, interfaceFactory }) {
        const agent = new HttpAgent({
            ...this.config,
            identity: this.identity,
        });

        // Fetch root key for certificate validation during development
        if (this.config.dev) {
            agent.fetchRootKey().catch((err) => {
                console.warn("Unable to fetch root key. Check to ensure that your local replica is running");
                console.error(err);
            });
        }

        return Actor.createActor(interfaceFactory, {
            agent,
            canisterId,
        });
    }

    async isConnected() {
        const identity = await StoicIdentity.load();
        return !!identity;
    }

    async connect() {
        this.identity = await StoicIdentity.connect();
        this.principal = this.identity.getPrincipal().toText();
        return true;
    }

    async disconnect() {
        await StoicIdentity.disconnect();
    }

    async expired() {
        return false;
    }
}

export const StoicWallet = {
    connector: StoicConnector,
    id: "stoic",
};
