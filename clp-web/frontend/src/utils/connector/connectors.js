import { Connector } from "@icpswap/actor";

// Since JavaScript does not support TypeScript's static typing and interfaces,
// we use a base class to define the structure that concrete implementations should follow.

export class IConnector {
    // Constructor to initialize properties
    constructor() {
        if (new.target === IConnector) {
            throw new Error("IConnector is an abstract class and cannot be instantiated directly.");
        }
    }

    // Methods are defined as stubs to be implemented by subclasses
    async init() {
        throw new Error("init method must be implemented");
    }

    async isConnected() {
        throw new Error("isConnected method must be implemented");
    }

    async createActor({ canisterId, interfaceFactory }) {
        throw new Error("createActor method must be implemented");
    }

    async connect() {
        throw new Error("connect method must be implemented");
    }

    async disconnect() {
        throw new Error("disconnect method must be implemented");
    }

    get getPrincipal() {
        throw new Error("getPrincipal getter must be implemented");
    }

    async expired() {
        throw new Error("expired method must be implemented");
    }
}

// Exporting Connector as ConnectorType for consistency
export const ConnectorType = Connector;
