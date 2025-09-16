import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OpenQuanta } from "../target/types/open_quanta";
import { 
  Connection,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
 } from "@solana/web3.js";
 import { 
  createMint,
  createAssociatedTokenAccount,
  getAssociatedTokenAddressSync,
  mintTo,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAccount,
 } from "@solana/spl-token";
import { expect } from "chai";

import {
  AssetV1,
  fetchAssetV1,
  createCollectionV2,
  createV2,
  fetchCollectionV1,
  Key,
  MPL_CORE_PROGRAM_ID,
  mplCore,
} from "@metaplex-foundation/mpl-core";

import {
  Context, 
  DateTime,
  PublicKey as metaplexPublicKey,
  PublicKeyInput,
  Signer,
  TransactionSignature,
  createGenericFile,
  signerIdentity,
  sol,
  Umi,
  assertAccountExists,
  generateSigner,
  now,
  percentAmount,
  publicKey,
  some,
  transactionBuilder
} from "@metaplex-foundation/umi";

import {createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import { base58 } from "@metaplex-foundation/umi/serializers";

describe("Open Quanta", async () => {

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.open_quanta as Program<OpenQuanta>;
  const mplCoreProgram = new PublicKey(MPL_CORE_PROGRAM_ID);

  const deployer = provider.wallet;
  const newAdmin = anchor.web3.Keypair.generate();
  const admin2 = anchor.web3.Keypair.generate();
  const author1 = anchor.web3.Keypair.generate();
  const collectionMint = anchor.web3.Keypair.generate();

  const umi = createUmi("https://api.devnet.solana.com").use(irysUploader());

  async function airdropSol(provider, publicKey, solAmount) {
    const airdropSig = await provider.connection.requestAirdrop(
      publicKey,
      solAmount * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSig);
  }


  // Airdropping Before
  before( async () => {
    await airdropSol(provider, newAdmin.publicKey, 5);
    await airdropSol(provider, admin2.publicKey, 5);
    await airdropSol(provider, author1.publicKey, 5);
  })

  it("Admin Initialization Test", async () => {
    
    // Admin PDA
    const [administratorsPDA, administratorsBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("administrators"), Buffer.from("OpenQuanta")],
      program.programId
    );

    await program.methods
      .initializeAdmins([newAdmin.publicKey, admin2.publicKey])
      .accounts({
        deployer: deployer.publicKey,
        //@ts-ignore
        admins: administratorsPDA,
        systemProgram: SystemProgram.programId
      })
      .signers([deployer.payer])
      .rpc();

    // Make Assertions
    const administratorsData = await program.account.administrators.fetch(administratorsPDA);
    expect(administratorsData.adminsPubkey.length).to.eq(2);
  })

  it("PaperID Counter Initialization", async () => {
    // Get PDA
    const [administratorsPDA, administratorsBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("administrators"), Buffer.from("OpenQuanta")],
      program.programId
    );

    const [paperIdPDA, paperIdBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("paper_id_counter")],
      program.programId
    );

    const [nftMintAuthorityPDA, ] = PublicKey.findProgramAddressSync(
      [Buffer.from("OpenQuanta_Nft_Mint_Authority")],
      program.programId
    );

    await program.methods
      .initializePaperId()
      .accounts({
        admin: newAdmin.publicKey,
        //@ts-ignore
        admins: administratorsPDA,
        paperIdAssigner: paperIdPDA,
        oqNftMintAuthority: nftMintAuthorityPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([newAdmin])
      .rpc();

    // Make Assertions
    const paperCounterData = await program.account.paperIdCounter.fetch(paperIdPDA);
    expect(paperCounterData.currentId.toNumber()).to.eq(0);
  })

  it.skip("Collection Registry Initialization", async () => {

    // Get PDAs
    const [administratorsPDA, administratorsBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("administrators"), Buffer.from("OpenQuanta")],
      program.programId
    );

    const [collectionRegistryPDA, ] = PublicKey.findProgramAddressSync(
      [Buffer.from("collection_registry"), Buffer.from("OpenQuanta")],
      program.programId
    );

    const [nftMintAuthorityPDA, ] = PublicKey.findProgramAddressSync(
      [Buffer.from("OpenQuanta_Nft_Mint_Authority")],
      program.programId
    );

    const collectionMint = await createCollectionV2

    await program.methods
      .initializeCollectionRegistry()
      .accounts({})
      .signers([])
      .rpc();
  })

  it.skip("Create OpenQuanta Authorship NFT Collection", async () => {})


  it("Author Profile Creation", async () => {
    // Get PDAs
    const [author1PDA, ] = PublicKey.findProgramAddressSync(
      [Buffer.from("author_profile"), author1.publicKey.toBuffer()],
      program.programId
    );


    await program.methods
      .initializeAuthorProfile("www.open-quanta.com/profile/author1", "Biochemistry")
      .accounts({
        author: author1.publicKey,
        //@ts-ignore
        authorProfile: author1PDA,
        systemProgram: SystemProgram.programId
      })
      .signers([author1])
      .rpc();

    // Make Assertions
    const authorProfileData = await program.account.authorProfile.fetch(author1PDA);
    expect(authorProfileData.authorFieldOfStudy.toString()).to.eq("Biochemistry");
    expect(authorProfileData.authorProfileUri.toString()).to.eq("www.open-quanta.com/profile/author1");
    expect(authorProfileData.numberOfSubmittedPapers).to.eq(0);
  })

  it.skip("Author Submits Paper and Gets Minted Authorship NFT", async () => {})
})
