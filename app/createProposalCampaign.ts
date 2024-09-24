import { TokenMetadata } from "@solana/spl-token-metadata";
import { createSolanaConnection, generateKeypair, uploadFilePinata, uploadJSONPinata } from "./helpers";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { createSignerFromKeypair, generateSigner, signerIdentity, TransactionBuilder } from "@metaplex-foundation/umi";
import { MPL_CORE_PROGRAM_ID, mplCore, createV1, createCollectionV1, fetchAsset } from '@metaplex-foundation/mpl-core'


console.log("Creating proposal campaign");

(async function() {

    const feepayerKeypair = generateKeypair("FEEPAYER");
    console.log("Feepayer Public Key:", feepayerKeypair.publicKey.toBase58());
    
    const haipTokenMintKeypair = generateKeypair("HAIP_TOKEN_MINT");
    console.log("HAIP Token Mint Public Key:", haipTokenMintKeypair.publicKey.toBase58());

    const haipTokenOwnerKeypair = generateKeypair("HAIP_TOKEN_OWNER");
    console.log("HAIP Token Owner Public Key:", haipTokenOwnerKeypair.publicKey.toBase58());
    
    const haipTokenAuthorityKeypair = generateKeypair("HAIP_TOKEN_AUTHORITY");
    console.log("HAIP Token Authority Public Key:", haipTokenAuthorityKeypair.publicKey.toBase58());

    const haipCampaignTestCollection1 = generateKeypair("HAIP_CAMPAIGN_TEST_COLLECTION_1");
    console.log("HAIP Campaign Test 1 Collection Public Key:", haipCampaignTestCollection1.publicKey.toBase58());
    
    const haipCampaignTestAsset1 = generateKeypair("HAIP_CAMPAIGN_TEST_ASSET_1");
    console.log("HAIP Campaign Test 1 Asset Public Key:", haipCampaignTestAsset1.publicKey.toBase58());

    // Upload Image campaign
    const haipFileData = await uploadFilePinata("haip_campaign_test_1", "./assets/campaign_test_1/haip_crownfund_adventure.png", "image/png");
    console.log("Haip logo File uploaded: ", haipFileData);

    const haipImageUrl = `https://ipfs.io/ipfs/${haipFileData.IpfsHash}`;

    console.log("Uploading metadata to pinata");

    // also we need to upload the token metadata to IPFS as json file.
    const haipMetadata = {
        "name": "Crownfunding Adventure by Haip",
        "symbol": "CAHAIP",
        "description": "NFT representing the Campaign of Crownfunding Adventure",
        "image": haipImageUrl,
    };

    // Upload offchain campaign metadata
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

    // Step 2 - Create a New Token
    const connection = createSolanaConnection();

    // Create Metaplex Core Mint
    // Metaplex Setup
    const umi = createUmi(connection.rpcEndpoint);
    let umiKeypair = umi.eddsa.createKeypairFromSecretKey(feepayerKeypair.secretKey);
    const signerKeypair = createSignerFromKeypair(umi, umiKeypair);
    umi.use(signerIdentity(signerKeypair)).use(mplCore());

    const confirm = async (signature: string): Promise<string> => {
        const block = await connection.getLatestBlockhash();
        await connection.confirmTransaction({
          signature,
          ...block
        })
        return signature
    }
    
    const log = async(signature: string): Promise<string> => {
        console.log(`Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`);
        return signature;
    }

    // const collection = generateSigner(umi);
    let collectionKeypair = umi.eddsa.createKeypairFromSecretKey(haipCampaignTestCollection1.secretKey);
    const collection = createSignerFromKeypair(umi, collectionKeypair);
    
    // const asset = generateSigner(umi);
    let assetKeypair = umi.eddsa.createKeypairFromSecretKey(haipCampaignTestAsset1.secretKey);
    const asset = createSignerFromKeypair(umi, assetKeypair);

    
    try {
        await new TransactionBuilder().add(
          createCollectionV1(umi, {
            collection,
            name: 'Crownfunding Adventure by Haip',
            uri: haipJSONUrl
          })
        ).add(
          createV1(umi, {
            asset: asset,
            name: 'Crownfunding Adventure by Haip',
            uri: haipJSONUrl,
            collection: collection.publicKey,
          })
        ).sendAndConfirm(umi)
      } catch(err) {
        console.log(err);
      }
    
    
// Add plugins
//  - attributes
//  - Freeze delegate

// Send to the client to sign it

// ( Created )

})();

