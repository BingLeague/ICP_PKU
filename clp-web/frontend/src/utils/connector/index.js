import { IDL } from "@dfinity/candid";
// import { Connector } from "constants/wallet";
// import { host } from "constants/server";
// import { updateAuth } from "store/auth/hooks";
// import { getDelegationIds } from "constants/connector";
import { InternetIdentityConnector } from "./internet-identity";
import { StoicConnector } from "./stoic";
// import { NF_IDConnector } from "./NF_ID";
import { PlugConnector } from "./plug";
// import { ICPSwapConnector } from "./icpswap";
import { InfinityConnector } from "./infinity";
// import { MeConnector } from "./me";
// import { MetamaskConnector } from "./metamask";

// In JavaScript, we don't have TypeScript type definitions like ConnectorClass, ProviderOptions, Provider, ConnectConfig

export class WalletConnector {
    constructor() {
        this.connector = null;
        this.connectorType = Connector.ICPSwap;
    }

    // initial connect instance
    async init(connectorType) {
        const connector = await WalletConnector.create(connectorType);
        this.connectorType = connectorType;
        if (connector) {
            await connector.init();
            this.connector = connector;
        }
    }

    static async create(connector) {
        const config = {
            host,
            whitelist: await getDelegationIds(),
        };

        switch (connector) {
            case Connector.IC:
                return new InternetIdentityConnector(config);
            case Connector.STOIC:
                return new StoicConnector(config);
            case Connector.NFID:
                return new NF_IDConnector(config);
            case Connector.PLUG:
                return new PlugConnector(config);
            case Connector.ICPSwap:
                return new ICPSwapConnector(config);
            case Connector.INFINITY:
                return new InfinityConnector(config);
            case Connector.ME:
                return new MeConnector(config);
            case Connector.Metamask:
                return new MetamaskConnector(config);
            default:
                throw new Error(`Connector error ${connector}: Not support this connect for now`);
        }
    }

    async connect() {
        if (!this.connector) return false;

        const isConnectedSuccessfully = await this.connector.connect();

        window.icConnector = this.connector;

        updateAuth({ walletType: this.connectorType });

        return isConnectedSuccessfully;
    }

    async isConnected() {
        const isConnected = await this.connector?.isConnected();
        return isConnected;
    }

    async createActor(canisterId, interfaceFactory) {
        return await this.connector?.createActor({ canisterId, interfaceFactory });
    }
}

export async function getConnectorIsConnected() {
    return window.icConnector.isConnected();
}

export async function getConnectorPrincipal() {
    return window.icConnector.getPrincipal;
}

export const connector = new WalletConnector();