import { getStoreWalletUnlocked } from "store/auth/hooks";
import { ConnectorType } from "./connectors";

const MAX_PLUG_WHITELIST_NUMBER = 200;

export class PlugConnector {
    constructor(config) {
        this.config = {
            whitelist: config.whitelist,
            host: config.host,
            providerUrl: "",
            dev: false,
        };
        this.principal = undefined;
        this.type = ConnectorType.PLUG;
    }

    get getPrincipal() {
        return this.principal;
    }

    async init() {
        return true;
    }

    async createActor({ canisterId, interfaceFactory }) {
        return await window.ic.plug.createActor({
            canisterId,
            interfaceFactory,
        });
    }

    async isConnected() {
        const isUnLocked = getStoreWalletUnlocked();

        if (typeof isUnLocked === "boolean" && !isUnLocked) {
            return false;
        }

        if (window.ic && window.ic.plug) {
            return await window.ic.plug.isConnected();
        }

        return false;
    }

    async connect() {
        // Fix tracing message if plug is uninstalled but still connect to
        if (!window.ic?.plug) {
            return false;
        }

        if (await this.isConnected()) {
            this.principal = window.ic.plug.principalId;
        } else {
            await window.ic.plug.requestConnect({
                whitelist:
                    this.config.whitelist.length > MAX_PLUG_WHITELIST_NUMBER
                        ? this.config.whitelist.slice(0, MAX_PLUG_WHITELIST_NUMBER)
                        : this.config.whitelist,
            });
            this.principal = window.ic.plug.principalId;
        }

        return true;
    }

    async disconnect() {
        await window.ic.plug.disconnect();
    }

    async expired() {
        return false;
    }
}
