import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { TokenMetadata } from "@solana/spl-token-metadata";
import { createSolanaConnection, createTokenAndMint, generateKeypair, uploadFilePinata, uploadJSONPinata } from "./helpers";

console.log("Starting to create HAIP token");

(async function() {
    const feepayerKeypair = generateKeypair("FEEPAYER");
    console.log("Feepayer Public Key:", feepayerKeypair.publicKey.toBase58());
    
    const haipTokenMintKeypair = generateKeypair("HAIP_TOKEN_MINT");
    console.log("HAIP Token Mint Public Key:", haipTokenMintKeypair.publicKey.toBase58());

    const haipTokenOwnerKeypair = generateKeypair("HAIP_TOKEN_OWNER");
    console.log("HAIP Token Owner Public Key:", haipTokenOwnerKeypair.publicKey.toBase58());
    
    const haipTokenAuthorityKeypair = generateKeypair("HAIP_TOKEN_AUTHORITY");
    console.log("HAIP Token Authority Public Key:", haipTokenAuthorityKeypair.publicKey.toBase58());

    const decimals = 9;
    const mintAmount = BigInt(1_000_000 * Math.pow(10, decimals)); // Mint 1,000,000 tokens
    
    console.log("Uploading image to pinata");
    
    /* 
    Already run this, so i only need to use the IPFS url
    // We need to update the image to IPFS. we use pinata to pin it and make it reachable
    const haipFileData = await uploadFilePinata("haip_logo", "./assets/haip.png", "image/png");
    console.log("Haip logo File uploaded: ", haipFileData);

    const haipImageUrl = `https://ipfs.io/ipfs/${haipFileData.IpfsHash}`;

    console.log("Uploading metadata to pinata");

    // also we need to upload the token metadata to IPFS as json file.
    const haipMetadata = {
        "name": "Haip",
        "symbol": "HAIP",
        "description": "Utility token to access and use the HAIP platform",
        "image": haipImageUrl,
    };

    const haipMetadataData = await uploadJSONPinata(haipMetadata);

    console.log("Haip metadata uploaded: ", haipMetadataData);

    const haipJSONUrl = `https://ipfs.io/ipfs/${haipMetadataData.IpfsHash}`;

    // We prepare the haip token metadata 
    const haipTokenMetadata: TokenMetadata = {
        updateAuthority: haipTokenAuthorityKeypair.publicKey,
        mint: haipTokenMintKeypair.publicKey,
        name: 'Haip',
        symbol: 'HAIP',
        uri: haipJSONUrl,
        additionalMetadata: []
    };

    console.log("Haip metadata generated: ", haipTokenMetadata); */

    const haipTokenMetadata: TokenMetadata = {
        updateAuthority: haipTokenAuthorityKeypair.publicKey,
        mint: haipTokenMintKeypair.publicKey,
        name: 'Haip',
        symbol: 'HAIP',
        uri: 'https://ipfs.io/ipfs/bafkreigo23kzepbmg3e75275ncqbl6oxwkp32jtloff3uhjnvzuzcfq4s4',
        additionalMetadata: []
    };
    
    // Step 2 - Create a New Token
    const connection = createSolanaConnection();

    /*  
    //I already ran this so i don't need more SOL
    await connection.requestAirdrop(feepayerKeypair.publicKey, LAMPORTS_PER_SOL); 
    */
    
    const results = await createTokenAndMint(
        connection,
        haipTokenMetadata,
        feepayerKeypair,
        haipTokenOwnerKeypair,
        haipTokenMintKeypair,
        haipTokenAuthorityKeypair,
        decimals,
        mintAmount
    );

    console.log('createTokenAndMint: ', results);
})();

