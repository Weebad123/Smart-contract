import { PublicKey } from "@solana/web3.js"
import {
    initializeAdmin,
} from "./setup"


const adminKeys = [
    new PublicKey("EnhBRG71jQBpJE5yj7QMEYLaWaHsPGHVbs3do6dg6p9q"),
];

const runInitialize = async () => {

    try {
        await initializeAdmin(adminKeys);
    } catch (err){}
}

runInitialize();

