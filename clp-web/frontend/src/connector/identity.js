import {AuthClient} from "@dfinity/auth-client";

const MAX_PLUG_WHITELIST_NUMBER = 200;
import {iiPrincipal, isConnected} from "../store/wallet";
import {
    auth,
    host,
} from "../store/auth";
import {createCanisterActor} from "../store/store";
import {HttpAgent} from "@dfinity/agent/lib/cjs/agent";

const iiExpireTime = 7 * 24 * 3600;

export class IdentityConnector {
    constructor(config) {
        this.config = {
            whitelist: config.whitelist, host: config.host, providerUrl: "", dev: false,
        };
        this.principal = undefined;
        this.client = undefined;
        this.agent = undefined
        // this.type = ConnectorType.PLUG;
    }

    get getPrincipal() {
        return this.principal;
    }

    async init() {
        this.client = await AuthClient.create();
        const isConnected = await this.isConnected();

        if (isConnected) {
            this.identity = this.client.getIdentity();
            this.principal = this.identity?.getPrincipal().toString();
        }
        return true;
    }

    async createActor({canisterId, interfaceFactory}) {
        this.agent = new HttpAgent({identity:this.identity, host});
        if (process.env.DFX_NETWORK !== "ic" ) {
            // Fetch root key for certificate validation during development
            this.agent.fetchRootKey().catch((err) => {
                console.warn("Unable to fetch root key. Check to ensure that your local replica is running");
                console.error(err);
            });
        }
        return await createCanisterActor(this.agent, interfaceFactory, canisterId);
    }

    async isConnected() {
        return !!(await this.client?.isAuthenticated());
    }

    handleAuth() {
        this.principal = this.client.getIdentity().getPrincipal().toText();
    }

    async connect() {
        let _this = this

        this.client = this.client? this.client:await AuthClient.create();
        await new Promise((resolve, reject) => {
            this.client?.login({
                identityProvider: process.env.DFX_NETWORK === "ic" ? "https://identity.ic0.app/#authorize" : `http://${process.env.INTERNET_IDENTITY_CANISTER_ID}.localhost:4944/#authorize`,
                onSuccess: () => {
                    _this.handleAuth()
                    resolve(true)
                },
                onError: reject,
                maxTimeToLive: BigInt(iiExpireTime * 1000 * 1000 * 1000),
            });
        });
        window.localStorage.setItem("ii-expire-time", (new Date().getTime() + iiExpireTime * 1000).toString());

        // Update Auth Store
        if (await this.client.isAuthenticated()) {
            await this.handleAuth();
            const identity = this.client?.getIdentity();
            const principal = identity?.getPrincipal().toString();
            this.identity = identity;
            this.principal = principal;
            return true;
        }



    }

    async disconnect() {
        await this.client.logout();
    }

    async expired() {
        const iiExpireTime = window.localStorage.getItem("ii-expire-time");
        if (!iiExpireTime) return true;
        return new Date().getTime() >= Number(iiExpireTime);
    }
}
