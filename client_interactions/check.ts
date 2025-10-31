// diagnostic-upload.ts
import fs from "fs";
import path from "path";
import util from "util";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { mplCore } from "@metaplex-foundation/mpl-core";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import { keypairIdentity } from "@metaplex-foundation/umi";
import { Keypair } from "@solana/web3.js";
import { createGenericFile } from "@metaplex-foundation/umi";

process.on("unhandledRejection", (r) => {
  console.error("UNHANDLED_REJECTION:", util.inspect(r, { depth: 5 }));
});
process.on("uncaughtException", (err) => {
  console.error("UNCAUGHT_EXCEPTION:", err && (err.stack || err.message || util.inspect(err, { depth: 5 })));
});

// small helper that forces a timeout
function timeboxed<T>(p: Promise<T>, ms = 30000) {
  return Promise.race([
    p,
    new Promise<T>((_, rej) => setTimeout(() => rej(new Error(`TIMEOUT_${ms}ms`)), ms)),
  ]);
}

(async () => {
  console.log("START DIAGNOSTIC");

  // Create UMI and register uploader (same as your code)
  const umi = createUmi("https://api.devnet.solana.com")
    .use(mplCore())
    .use(irysUploader({ address: "https://devnet.irys.xyz" }));
  console.log("UMI created");

  // print uploader presence and methods
  const uploader = (umi as any).uploader;
  console.log("uploader present?", !!uploader);
  if (uploader) console.log("uploader methods:", Object.keys(uploader));

  // set identity from local keypair to ensure uploader has signer if needed
  const keypath = path.resolve(process.env.HOME || ".", ".config/solana/id.json");
  const secret = JSON.parse(fs.readFileSync(keypath, "utf8"));
  const kp = Keypair.fromSecretKey(Uint8Array.from(secret));
  umi.use(keypairIdentity(kp as unknown as any));
  console.log("umi.identity:", String((umi as any).identity?.publicKey));

  try {
    const balance = await timeboxed(umi.rpc.getBalance(kp.publicKey.toBase58() as unknown as any), 10000);
    console.log("balance:", balance);
  } catch (e) {
    console.warn("balance check failed or timed out:", e && (e.message || util.inspect(e)));
  }

  // Prepare file
  const imagePath = path.join(__dirname, "..", "assets", "openQuanta.jpeg"); // adjust if needed
  console.log("imagePath:", imagePath, "exists:", fs.existsSync(imagePath));
  const buf = fs.readFileSync(imagePath);
  const umiImageFile = createGenericFile(buf, "openQuanta.jpeg", { contentType: "image/jpeg" });
  console.log("createGenericFile ok, bytes:", buf.length);

  // If uploader missing -> bail with instruction
  if (!uploader || typeof uploader.upload !== "function") {
    console.error("NO uploader.upload() found. Either plugin failed to register or is incompatible.");
    console.error("Consider switching to nft.storage uploader: `yarn add @metaplex-foundation/umi-uploader-nft-storage nft.storage` and use `.use(nftStorageUploader(...))`");
    process.exit(1);
  }

  // Now attempt timeboxed upload and print everything raw
  console.log("Calling upload() — this will time out after 30s if it hangs");
  try {
    const rawPromise = uploader.upload([umiImageFile]);
    console.log("upload() returned a value? =>", !!rawPromise, "typeof:", typeof rawPromise, "is Promise?", rawPromise && typeof rawPromise.then === "function");
    const res = await timeboxed(rawPromise as Promise<any>, 30000);
    console.log("UPLOAD RESULT:", util.inspect(res, { depth: 5 }));
  } catch (err) {
    console.error("UPLOAD ERROR / TIMEOUT (raw):", err && (err.stack || util.inspect(err, { depth: 5 })));
  }

  console.log("END DIAGNOSTIC");
  process.exit(0);
})();
