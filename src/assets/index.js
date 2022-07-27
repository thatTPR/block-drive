import { Actor, HttpAgent } from "@dfinity/agent";
import './login.js';
// Imports candid interface
import { idlFactory } from './hello.did.js.js';

import token from 'ic:cansiters/token';
import core from 'ic:cansisters/core';
// CANISTER_ID is replaced by webpack based on node enviroment





export const canisterId = await login(); // TODO

/**
 *
 * @param {string | Principal} canisterId Canister ID of Agent
 * @param {{agentOptions?: import("@dfinity/agent").HttpAgentOptions; actorOptions?: import("@dfinity/agent").ActorConfig}} [options]
 * @return {import("@dfinity/agent").ActorSubclass<import("./hello.did.js.js")._SERVICE>}
 */
export const createActor = (canisterId, options) => {
    const agent = new HttpAgent({ ...options?.agentOptions });

    // Fetch root key for certificate validation during development
    if (process.env.NODE_ENV !== "production") agent.fetchRootKey();

    // Creates an actor with using the candid interface and the HttpAgent
    return Actor.createActor(idlFactory, {
        agent,
        canisterId,
        ...options?.actorOptions,
    });
};

/**
 * A ready-to-use agent for the hello canister
 * @type {import("@dfinity/agent").ActorSubclass<import("./hello.did.js.js")._SERVICE>}
 */
export const hello = createActor(canisterId);