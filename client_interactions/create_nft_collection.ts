import {
    createOpenQuantaNFTCollection
} from "./setup";


const runCreating = async () => {

    try {
        await createOpenQuantaNFTCollection();
    } catch (err) {}
}

runCreating();