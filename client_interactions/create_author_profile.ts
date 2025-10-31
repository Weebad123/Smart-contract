import {
    initializeAuthorProfile,
    author1Keypair,
    author3Keypair,
    author4Keypair,
    author5Keypair,
    author6Keypair,
} from "./setup";

const author_uri = "https://profile.openQuanta.org/author1_profile_biochemistry/";
const fieldOfStudy = "Biochemistry";

const author4_uri = "https://profile.openQuanta.vercel.app/author3_profile_blockchain/";
const author4FieldOfStudy = "Cryptography";

const createProfile = async () => {
    
    try {
        await initializeAuthorProfile(author6Keypair, author4_uri, author4FieldOfStudy);
    } catch (err) {}
}

createProfile();
