import {
    initializeAuthorProfile,
    author1Keypair
} from "./setup";

const author_uri = "https://profile.openQuanta.org/author1_profile_biochemistry/";
const fieldOfStudy = "Biochemistry";


const createProfile = async () => {
    
    try {
        await initializeAuthorProfile(author1Keypair, author_uri, fieldOfStudy);
    } catch (err) {}
}

createProfile();
