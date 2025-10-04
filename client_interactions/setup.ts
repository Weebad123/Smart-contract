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
    SystemProgram
 } from "@solana/web3.js";
 import fs from "fs";
 import path from "path";
 import { utf8 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

 import type { OpenQuanta } from "../target/types/open_quanta";
import { Key } from "@metaplex-foundation/mpl-core";


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

 const author1KeypairFile = fs.readFileSync(
    path.resolve(__dirname, "./wallets/author1-wallet.json"),
    "utf-8"
 );
 export const author1Keypair = Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(author1KeypairFile))
 );
 

 // FUNCTION TO INITIALIZE ADMIN
 // Get Administrators PDA
export const [administratorsPDA, ] = PublicKey.findProgramAddressSync(
    [Buffer.from("administrators"), Buffer.from("OpenQuanta")],
    openQuantaProgramId
);

export const[paperIdPDA, ] = PublicKey.findProgramAddressSync(
    [Buffer.from("paper_id_counter")],
    openQuantaProgramId
);

export const [nftMintAuthorityPDA, ] = PublicKey.findProgramAddressSync(
    [Buffer.from("OpenQuanta_Nft_Mint_Authority")],
    openQuantaProgramId
);

export const [collectionRegistryPDA, ] = PublicKey.findProgramAddressSync(
    [Buffer.from("collection_registry"), Buffer.from("OpenQuanta")],
    openQuantaProgramId
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