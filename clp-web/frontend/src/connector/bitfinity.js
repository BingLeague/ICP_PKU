import { iiPrincipal, isConnected } from "../store/wallet";
import {
    VAULT_CANISTER_ID,
    LEDGER_CANISTER_ID,
    CKBTC_CANISTER_ID,
    CKETH_CANISTER_ID,
    CUSD_CANISTER_ID,
    POOL_CANISTER_ID
} from "../store/auth";
const iiExpireTime = 1 * 24 * 3600;

export class BitfinityConnector {
    constructor(config) {
        this.config = {
            whitelist: config.whitelist,
            host: config.host,
            providerUrl: "",
            dev: false,
        };
        this.principal = undefined;
        // this.type = ConnectorType.BITFINITY;
    }

    get getPrincipal() {
        return this.principal;
    }

    async init() {
        if (!window.ic?.bitfinityWallet) {
            return false;
        }
        if (await this.isConnected()) {
            let principal = await window.ic.bitfinityWallet.getPrincipal();
            this.principal = principal.toText()
        }
        console.log(  this.principal)

        return true;
    }

    async createActor({ canisterId, interfaceFactory }) {
        return await window.ic.bitfinityWallet.createActor({
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

        if (window.ic && window.ic.bitfinityWallet) {
            return await window.ic.bitfinityWallet.isConnected();
        }

        return false;
    }

    async connect() {
        // Fix tracing message if Bitfinity is uninstalled but still connect to
        if (!window.ic?.bitfinityWallet) {
            return false;
        }
        const whitelist = [
            VAULT_CANISTER_ID,
            LEDGER_CANISTER_ID,
            CKBTC_CANISTER_ID,
            CKETH_CANISTER_ID,
            CUSD_CANISTER_ID,
            POOL_CANISTER_ID
        ];
        if (await this.isConnected()) {
            let principal = await window.ic.bitfinityWallet.getPrincipal();
            this.principal = principal.toText()
        } else {
            await window.ic.bitfinityWallet.requestConnect({
                whitelist: whitelist
            });
            let principal = await window.ic.bitfinityWallet.getPrincipal();
            this.principal = principal.toText()
        }
        window.localStorage.setItem("ii-expire-time", (new Date().getTime() + iiExpireTime * 1000).toString());


        return true;
    }

    async disconnect() {
        await window.ic.bitfinityWallet.disconnect();
    }

    async expired() {
        return false;
    }
}
