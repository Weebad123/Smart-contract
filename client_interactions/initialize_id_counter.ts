import {
    initializePaperIdCounter
} from "./setup"


const initializeId = async () => {

    try {
        await initializePaperIdCounter();
    } catch (err) {}
}

initializeId();
