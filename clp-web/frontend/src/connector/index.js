import {host} from "../store/auth";
import { PlugConnector } from "./plug";
import {IdentityConnector} from "./identity";
// import {StoicConnector} from "./stoic"
import {BitfinityConnector} from "./bitfinity";

export class WalletConnector {
    constructor() {
        this.connector = null;
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
            whitelist: [],
        };
        console.log(connector)
        switch (connector) {
            case "plug":
                return new PlugConnector(config);
            case "identity":
                return new IdentityConnector(config);
            // case "stoic":
            //     return new StoicConnector(config);
            case "bitfinity":
                return new BitfinityConnector(config);
            default:
                throw new Error(`Connector error ${connector}: Not support this connect for now`);
        }
    }

    async connect() {
        if (!this.connector) return false;
        const isConnectedSuccessfully = await this.connector.connect();
        window.icConnector = this.connector;
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

export  function getConnectorPrincipal() {
    return window.icConnector.getPrincipal;
}

export const connector = new WalletConnector();