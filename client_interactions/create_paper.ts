import {
    submitPaper,
    author5Keypair,
    author6Keypair
} from "./setup";


const runPaperSubmission = async () => {
    try {
        await submitPaper(author6Keypair);
    } catch (err) {
        console.log("Error While Submitting Paper", err)
    }
}

runPaperSubmission();

