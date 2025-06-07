// Import necessary modules
import { StoicIdentity } from "ic-stoic-identity";
import { Actor, HttpAgent } from "@dfinity/agent";
import { Connector } from "@icpswap/actor";
import { ConnectorType } from "./connectors";

export class StoicConnector {
    constructor(config) {
        this.config = {
            whitelist: config.whitelist,
            host: config.host,
            providerUrl: "https://www.stoicwallet.com",
            dev: false,
        };
        this.identity = undefined;
        this.principal = undefined;
        this.type = ConnectorType.STOIC;
    }

    get getPrincipal() {
        return this.principal;
    }

    async init() {
        const identity = await StoicIdentity.load(this.config.providerUrl);

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
    type: Connector.STOIC,
};
