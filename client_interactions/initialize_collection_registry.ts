import {
    initializeCollectionRegistry
} from "./setup"


const initializeCollectionReg = async () => {

    try {
        await initializeCollectionRegistry();
    } catch (err) {}
}

initializeCollectionReg();