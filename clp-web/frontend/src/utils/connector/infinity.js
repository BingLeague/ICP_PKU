import { getStoreWalletUnlocked } from "store/auth/hooks";
import { ConnectorType } from "./connectors";

export class InfinityConnector {
    constructor(config) {
        this.config = {
            whitelist: config.whitelist,
            host: config.host,
            providerUrl: "",
            dev: false,
        };
        this.principal = undefined;
        this.type = ConnectorType.INFINITY;
    }

    get getPrincipal() {
        return this.principal;
    }

    async init() {
        return true;
    }

    async createActor({ canisterId, interfaceFactory }) {
        return await window.ic.infinityWallet.createActor({
            canisterId,
            interfaceFactory,
        });
    }

    async isConnected() {
        const isUnLocked = getStoreWalletUnlocked();

        if (typeof isUnLocked === "boolean" && !isUnLocked) {
            return false;
        }

        if (window.ic.infinityWallet) {
            return await window.ic.infinityWallet.isConnected();
        }

        return false;
    }

    async connect() {
        if (await this.isConnected()) {
            this.principal = (await window.ic.infinityWallet.getPrincipal()).toString();
        } else {
            // disconnect first
            await window.ic.infinityWallet.disconnect();
            await window.ic.infinityWallet.requestConnect({ whitelist: this.config.whitelist });

            this.principal = (await window.ic.infinityWallet.getPrincipal()).toString();
        }

        return true;
    }

    async disconnect() {
        await window.ic.infinityWallet.disconnect();
    }

    async expired() {
        return false;
    }
}

export const InfinitySwapWallet = {
    connector: InfinityConnector,
    id: "infinity",
    type: ConnectorType.INFINITY,
};
