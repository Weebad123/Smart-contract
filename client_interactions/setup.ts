import * as anchor from "@coral-xyz/anchor";
import {
    Program,
    AnchorProvider,
    utils,
    BN
} from "@coral-xyz/anchor";
import { 
    Keypair,
    PublicKey,
    clusterApiUrl,
    Connection,
    SystemProgram,
    sendAndConfirmTransaction
 } from "@solana/web3.js";
 import fs from "fs";
 import path from "path";
 import { bs58, utf8 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

 import type { OpenQuanta } from "../target/types/open_quanta";
import { Key, MPL_CORE_PROGRAM_ID, mplCore } from "@metaplex-foundation/mpl-core";
import {
    createGenericFile,
    createSignerFromKeypair,
    generateSigner,
    keypairIdentity,
    signerIdentity,
    sol,
} from "@metaplex-foundation/umi";
import {
    createUmi,
} from "@metaplex-foundation/umi-bundle-defaults";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import { error } from "console";
import {
    TurboFactory, ArweaveSigner,
    SOLToTokenAmount,
    HexSolanaSigner
} from "@ardrive/turbo-sdk";

 // LOAD IDL FILE
 const idlFile = fs.readFileSync(
    path.resolve(__dirname, "../target/idl/open_quanta.json"),
    "utf-8"
 );
 const idl = JSON.parse(idlFile);
 const openQuantaProgramId = new PublicKey(idl.address);

 // SET UP CONNECTION TO DEVNET
 const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

 // SET UP DEPLOYER
 const deployerKeypairFile = fs.readFileSync(
    path.resolve(__dirname, "/home/ernest/.config/solana/id.json"),
    "utf-8"
 );
 const deployerWalletKeypair = Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(deployerKeypairFile))
 );
 const deployerWallet = new anchor.Wallet(deployerWalletKeypair);

 // SET UP PROVIDER AND INITIALIZE PROGRAM
 const provider = new AnchorProvider(connection, deployerWallet, {
    commitment: "confirmed"
 });
 anchor.setProvider(provider);

 const program = new Program<OpenQuanta>(idl, provider);


 // SET SAMPLE WALLETS FOR TESTING
 const collectionKeypairFile = fs.readFileSync(
    path.resolve(__dirname, "./wallets/collection-wallet.json"),
    "utf-8"
 );
 const collectionKeypair = Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(collectionKeypairFile))
 );

 const nftAssetKeypairFile = fs.readFileSync(
    path.resolve(__dirname, "./wallets/nftAsset-wallet.json"),
    "utf-8"
 );
 const nftAssetKeypair = Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(nftAssetKeypairFile))
 );

 const nftAsset555KeypairFile = fs.readFileSync(
    path.resolve(__dirname, "./wallets/nftAsset555-wallet.json"),
    "utf-8"
 );
 const nftAsset555Keypair = Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(nftAsset555KeypairFile))
 );

 const author1KeypairFile = fs.readFileSync(
    path.resolve(__dirname, "./wallets/author1-wallet.json"),
    "utf-8"
 );
 export const author1Keypair = Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(author1KeypairFile))
 );

 const author3KeypairFile = fs.readFileSync(
    path.resolve(__dirname, "./wallets/author3-wallet.json"),
    "utf-8"
 );
 export const author3Keypair = Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(author3KeypairFile))
 );

 const author4KeypairFile = fs.readFileSync(
    path.resolve(__dirname, "./wallets/author4-wallet.json"),
    "utf-8"
 );
 export const author4Keypair = Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(author4KeypairFile))
 );

 const author5KeypairFile = fs.readFileSync(
    path.resolve(__dirname, "./wallets/author5-wallet.json"),
    "utf-8"
 );
 export const author5Keypair = Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(author5KeypairFile))
 );

 const author6KeypairFile = fs.readFileSync(
    path.resolve(__dirname, "./wallets/author6-wallet.json"),
    "utf-8"
 );
 export const author6Keypair = Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(author6KeypairFile))
 );
 

 // FUNCTION TO INITIALIZE ADMIN
 // Get Administrators PDA
export const [administratorsPDA, ] = PublicKey.findProgramAddressSync(
    [Buffer.from("administrators"), Buffer.from("openQuanta")],
    openQuantaProgramId
);

export const[paperIdPDA, ] = PublicKey.findProgramAddressSync(
    [Buffer.from("paper_id_counter"), Buffer.from("openQuanta")],
    openQuantaProgramId
);

export const [nftMintAuthorityPDA, ] = PublicKey.findProgramAddressSync(
    [Buffer.from("openQuanta_Nft_Mint_Authority")],
    openQuantaProgramId
);

export const [collectionRegistryPDA, ] = PublicKey.findProgramAddressSync(
    [Buffer.from("collections_registry"), Buffer.from("openQuanta")],
    openQuantaProgramId
);

export const [author1PDA, ] = PublicKey.findProgramAddressSync(
    [Buffer.from("author_profile"), author1Keypair.publicKey.toBuffer()],
    program.programId
);

export const [author3PDA, ] = PublicKey.findProgramAddressSync(
    [Buffer.from("author_profile"), author3Keypair.publicKey.toBuffer()],
    program.programId
);

export const [author4PDA, ] = PublicKey.findProgramAddressSync(
    [Buffer.from("author_profile"), author4Keypair.publicKey.toBuffer()],
    program.programId
);

export const [author5PDA, ] = PublicKey.findProgramAddressSync(
    [Buffer.from("author_profile"), author5Keypair.publicKey.toBuffer()],
    program.programId
);

export const [author6PDA, ] = PublicKey.findProgramAddressSync(
    [Buffer.from("author_profile"), author6Keypair.publicKey.toBuffer()],
    program.programId
);
// INITIALIZE ADMIN INSTRUCTION
 export const initializeAdmin = async (
    adminKeys: PublicKey[]
 ): Promise<string> => {

    for (const adminKey of adminKeys) {
        console.log("Initializing Admin With Address: ", adminKey);
    }

    try {
        const tx = await program.methods
            .initializeAdmins(adminKeys)
            .accounts({
                deployer: deployerWallet.publicKey,
                //@ts-ignore
                admins: administratorsPDA,
                systemProgram: SystemProgram.programId
            })
            .signers([])
            .rpc();

        console.log("Transaction submitted successfully: ", tx);
        console.log(
            "View Transaction On Solana Explorer:",
            `https://explorer.solana.com/tx/${tx}?cluster=devnet`
        );
        return tx;
    } catch (err) {
        console.log("There was an error initializing Admin(s)", err);
        throw err;
    }
 }

// INITIALIZE PAPER ID COUNTER INSTRUCTION
 export const initializePaperIdCounter = async (): Promise<void> => {
    // Call Instruction
    try {
        const tx = await program.methods
            .initializePaperId()
            .accounts({
                admin: deployerWallet.publicKey,
                //@ts-ignore
                admins: administratorsPDA,
                paperIdAssigner: paperIdPDA,
                oqNftMintAuthority: nftMintAuthorityPDA,
                systemProgram: SystemProgram.programId,
            })
            .signers([deployerWallet.payer])
            .rpc();

        console.log("Transaction Submitted Successfully: ", tx);
        console.log(
            "View Paper ID Counter Initialization Transaction On Solana Explorer Here:",
            `https://explorer.solana.com/tx/${tx}?cluster=devnet`
        );

    } catch (err) {
        console.log("There was an error Initializing Paper ID Counter: ", err);
    }
 }


 // INITIALIZE COLLECTION REGISTRY
 export const initializeCollectionRegistry = async (): Promise<void> => {

    try {
        const tx = await program.methods
            .initializeCollectionRegistry()
            .accounts({
                admin: deployerWallet.publicKey,
                //@ts-ignore
                admins: administratorsPDA,
                collectionMint: collectionKeypair.publicKey,
                collectionRegistry: collectionRegistryPDA,
                oqNftMintAuthority: nftMintAuthorityPDA,
                systemProgram: SystemProgram.programId,
            })
            .signers([deployerWallet.payer])
            .rpc();

        console.log("Collection Registry Initialization Transaction Submitted Successfully:", tx);
        console.log(
            "View Collection Registry Initialization Transaction on Solana Explorer Here: ", 
            `https://explorer.solana.com/tx/${tx}?cluster=devnet`
        );
    } catch (err) {
        console.log("There was a problem initializing the collection registry: ", err);
    }
 }


 // CREATE AUTHOR PROFILE
 export const initializeAuthorProfile = async(
    author_address: Keypair,
    profile_uri: string,
    field_of_study: string,
 ): Promise<void> => {
    const [authorPDA, ] = PublicKey.findProgramAddressSync(
        [Buffer.from("author_profile"), author_address.publicKey.toBuffer()],
        openQuantaProgramId
    );

    try {
        const tx = await program.methods
        .initializeAuthorProfile(profile_uri, field_of_study)
        .accounts({
            author: author_address.publicKey,
            //@ts-ignore
            authorProfile: authorPDA,
            systemProgram: SystemProgram.programId
        })
        .signers([author_address])
        .rpc();

        console.log("Author Creating A Profile On OpenQuanta: ", tx);
        console.log(
            "View Author Profile Creation On Solana Explorer Here: ",
            `https://explorer.solana.com/tx/${tx}?cluster=devnet`
        );
    } catch (err) {
        console.log("There was an error creating author profile: ", err);
    }
 }


 // CREATING OPENQUANTA NFT COLLECTION
 export const createOpenQuantaNFTCollection = async(): Promise<void> => {

    // Setting up Umi
    const umi = createUmi('https://api.devnet.solana.com', 'confirmed')
        .use(mplCore())
        .use(irysUploader(/*{address: 'https://devnet.irys.xyz'}*/));

    //console.log("The UMI instance is: ", umi);

    const keypath = path.resolve(process.env.HOME || ".", ".config/solana/id.json");
    const secret = JSON.parse(fs.readFileSync(keypath, "utf8"));
    const kp = Keypair.fromSecretKey(Uint8Array.from(secret));
    umi.use(keypairIdentity(kp as unknown as any));
    console.log("umi.identity:", String((umi as any).identity?.publicKey));

    //umi.use(signerIdentity(signer));

    //console.log("Airdropping 1 SOL to identity");
    //await umi.rpc.airdrop(umi.identity.publicKey, sol(1));

    //Upload NFT Image to Arweave
    const imageFile = fs.readFileSync(
        path.join(__dirname, '..','assets','openQuanta.jpeg')
    );
    console.log("Image File is: ", imageFile);

    const umiImageFile = createGenericFile(imageFile, 'openQuanta.jpeg', {
        tags: [{name: 'Content-Type', value: 'image/jpeg' }],
    });

    console.log("Image FILE received:");

    const imageUri = await umi.uploader.upload([umiImageFile]).catch((err) => {
        throw new Error(err)
    });

    console.log("NFT image has been uploaded successfully");
    console.log('imageUri: ', imageUri[0]);

    // Upload NFT metadata to Arweave

    const metadata = {
        name: "openQuanta Authorship NFT Collection",
        symbol: "QPT",
        description: "Authorship NFT Collection of OpenQuanta",
        image: imageUri[0],
        external_url: "https://openquanta.vercel.app/",
        properties: {
            files: [
                {
                    uri: imageUri[0],
                    type: 'image/jpeg'
                },
            ],
            category: 'image',
        },
    };

    console.log("Uploading metadata to Arweave .....");
    const metadatauri = await umi.uploader.uploadJson(metadata).catch((err) => {
        throw new Error(err)
    })

    console.log("Metadata URI is: ", metadatauri);

    // call instruction to Create Collection
    
    try {
        const tx = await program.methods
        .createCollection("openQuanta Authorship NFT Collection", metadatauri)
        .accounts({
            admin: deployerWallet.publicKey,
            //@ts-ignore
            admins: administratorsPDA,
            collection: collectionKeypair.publicKey,
            collectionRegistry: collectionRegistryPDA,
            payer: deployerWallet.publicKey,
            mplCore: MPL_CORE_PROGRAM_ID,
            systemProgram: SystemProgram.programId
        })
        .signers([deployerWallet.payer, collectionKeypair])
        .rpc();

    console.log("Creating OpenQuanta Authorship NFT Collection ...");
    console.log(
        "View openQuanta Collection Creation on Solana Explorer Here",
        `https://explorer.solana.com/tx/${tx}?cluster=devnet`
    );

    } catch (err) {
        console.log("There was an error Creating the OpenQuanta NFT Collection: ", err);
    }
 }


 // SUBMIT PAPER AND GETS MINTED QPT NFT
 export const submitPaper = async (
    author_address: Keypair
 ): Promise<void> => {
   
    
    // Upload To Arweave Via Turbo SDK
    //const fileData = fs.readFileSync("../assets/Receipt.PDF");
    const researchFile = fs.readFileSync(
        path.join(__dirname, '..','assets','Receipt.PDF')
    );
    console.log("Research File is: ", researchFile);
    const arweaveFileUri = await uploadFileToArweave(researchFile, author_address);

    // Upload File To Arweave via Umi instance

    // Build instruction arguments
    const author1PaperArgs = {
      titleOfPaper: "Encryption and Cryptography",
      paperArweaveHash: arweaveFileUri,
      fieldOfResearch: "Cryptography",
      paperVersion: 1,
    };

    const author3PaperArgs = {
      titleOfPaper: "Genetic proofs: the future of medical tech",
      //paperArweaveHash: arweaveFileUri,
      fieldOfResearch: "Genetics",
      paperVersion: new BN(1),
    };

    const coAuthors = [];// Empty for now

    const counterAccount = await program.account.paperIdCounter.fetch(paperIdPDA);
    const nextId = counterAccount.currentId.toNumber() + 1;
    const formattedId = `OQ-${String(nextId).padStart(7, '0')}`;
    console.log("The Current On-chain Counter ID is: ", counterAccount.currentId.toNumber());

    const [researchPaperPDA, ] = PublicKey.findProgramAddressSync(
        [Buffer.from("paper"), author_address.publicKey.toBuffer(), Buffer.from(formattedId)],
        program.programId
    );

    const [authorPDA, ] = PublicKey.findProgramAddressSync(
        [Buffer.from("author_profile"), author_address.publicKey.toBuffer()],
        program.programId
    );

    const [nftAssetPDA,] = PublicKey.findProgramAddressSync(
        [Buffer.from("asset"), author_address.publicKey.toBuffer(), Buffer.from(formattedId)],
        program.programId
    );
    // Call Actual instruction
    try {
        const tx = await program.methods
            .submitPaper(author1PaperArgs, coAuthors)
            .accounts({
                paperSubmitter: author_address.publicKey,
                //@ts-ignore
                paperIdAssigner: paperIdPDA,
                authorProfile: authorPDA,
                researchPaper: researchPaperPDA,
                collectionRegistry: collectionRegistryPDA,
                collection: collectionKeypair.publicKey,
                oqNftMintAuthority: nftMintAuthorityPDA,
                nftAsset: author_address.publicKey,
                mplCore: MPL_CORE_PROGRAM_ID,
                systemProgram: SystemProgram.programId
            })
            .signers([author_address, author_address])
            .rpc();

        console.log("Submitting Paper and Getting Minted QPT NFT ...");
        console.log(
            "View openQuanta Paper Submission on Solana Explorer Here",
            `https://explorer.solana.com/tx/${tx}?cluster=devnet`
        );
    } catch (err) {
        console.log("There was an error submitting paper and getting QPT NFT: ", err);
    }
 }

 const uploadFileToArweave = async(
    file: any,
    owner_of_paper: Keypair
 ): Promise<string> => {

    // Upload File To Arweave Via Turbo SDK
    const author5Wallet = JSON.parse(fs.readFileSync(
        path.resolve(__dirname, "./wallets/dennis-wallet.json"),
    "utf-8"));
    console.log("The Wallet is: ", author5Wallet);
    const secretKey = new Uint8Array(author5Wallet);

    const signer = new HexSolanaSigner(bs58.encode(secretKey));
    const turbo = TurboFactory.authenticated({
        privateKey: bs58.encode(secretKey),
        token: 'solana',
        signer: signer,
        gatewayUrl: 'https://api.devnet.solana.com',
        paymentServiceConfig: {
            url: 'https://payment.ardrive.dev',
        },
        uploadServiceConfig: {
            url: 'https://upload.ardrive.dev',
        }
    });

        // Get SOL Price For File To Be Uploaded
        const tokenPriceForFile = await turbo.getTokenPriceForBytes(file);

        try {
            // Pay For Upload Cost
        //! @Note Gotta Read More For The Uploading Logic
        const {winc, status, id, ...fundResult} = await turbo.topUpWithTokens({
            tokenAmount: SOLToTokenAmount(/*tokenPriceForFile.tokenPrice*/0.1),
            feeMultiplier: null
        });
        } catch (err) {
            console.log("Error While Paying The Upload Cost", err);
        }

        const titleOfPaper = "Genetic proofs: the future of medical tech"
        // Upload File
        const arweaveHash = await turbo.upload({
            data: file,
            dataItemOpts: {
                tags: [// @TODO miht add more tags, maybe or maybe not
                    { name: "Content-Type", value: file.type || "application/octet-stream" },
                    { name: "Title", value: titleOfPaper.toString() },
                    { name: "Owner of Paper", value: owner_of_paper.publicKey.toString()}
                ],
            },
        });

        // Logs
        console.log("Upload ID is: ", arweaveHash.id);
        console.log("Owner of Hash is: ", arweaveHash.owner);
        return arweaveHash.id
 }