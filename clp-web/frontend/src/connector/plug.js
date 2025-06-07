const MAX_PLUG_WHITELIST_NUMBER = 200;
import {iiPrincipal, isConnected} from "../store/wallet";
import {
    VAULT_CANISTER_ID,
    LEDGER_CANISTER_ID,
    CKBTC_CANISTER_ID,
    CKETH_CANISTER_ID,
    CUSD_CANISTER_ID,
    POOL_CANISTER_ID
} from "../store/auth";
const iiExpireTime = 1 * 24 * 3600;
export class PlugConnector {
    constructor(config) {
        this.config = {
            whitelist: config.whitelist,
            host: config.host,
            providerUrl: "",
            dev: false,
        };
        this.principal = undefined;
        // this.type = ConnectorType.PLUG;
    }

    get getPrincipal() {
        return this.principal;
    }

    async init() {
        if (!window.ic?.plug) {
            return false;
        }
        if (await this.isConnected()) {
            this.principal = window.ic.plug.principalId;
        }
        return true;
    }

    async createActor({canisterId, interfaceFactory}) {
        return await window.ic.plug.createActor({
            canisterId,
            interfaceFactory,
        });
    }

    async isConnected() {
        let isUnLocked;

        isConnected.subscribe(value => {
            isUnLocked = value;
        });
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

        const whitelist = [
            VAULT_CANISTER_ID,
            LEDGER_CANISTER_ID,
            CKBTC_CANISTER_ID,
            CKETH_CANISTER_ID,
            CUSD_CANISTER_ID,
            POOL_CANISTER_ID
        ]
        if (await this.isConnected()) {
            this.principal = window.ic.plug.principalId;
        } else {
            await window.ic.plug.requestConnect({
                whitelist:whitelist
            });
            this.principal = window.ic.plug.principalId;
        }
        window.localStorage.setItem("ii-expire-time", (new Date().getTime() + iiExpireTime * 1000).toString());
        return true;
    }

    async disconnect() {
        await window.ic.plug.disconnect();
    }

    async expired() {
        return false;
    }
}
