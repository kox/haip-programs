import 'dotenv/config';
import * as fs from "fs";
import { PinataSDK, PinResponse } from "pinata-web3";
import { clusterApiUrl, Connection, Keypair, sendAndConfirmTransaction, SystemProgram, Transaction } from '@solana/web3.js';
import { createAssociatedTokenAccountIdempotent, createInitializeMetadataPointerInstruction, createInitializeMintInstruction, ExtensionType, getMintLen, LENGTH_SIZE, mintTo, TOKEN_2022_PROGRAM_ID, TYPE_SIZE } from '@solana/spl-token';
import { createInitializeInstruction, pack, TokenMetadata } from '@solana/spl-token-metadata';

const pinata = new PinataSDK({
    pinataJwt: process.env.PINATA_API_KEY,
    pinataGateway: process.env.PINATA_GATEWAY
})

export function generateKeypair(name: string) {
    const filePath = process.env[name];

    if (!filePath) {
        throw new Error(`No key file path found for ${name}`);
    }

    // Read the JSON file containing the private key
    const keyContent = fs.readFileSync(filePath, { encoding: 'utf-8' });

    // Parse the JSON into an array of numbers (the secret key)
    const secretKey = Uint8Array.from(JSON.parse(keyContent));

    // Create a Keypair from the secret key
    const keypair = Keypair.fromSecretKey(secretKey);

    return keypair;
}

export function createSolanaConnection() {
    let rpc = process.env['RPC_DEVNET'] || '';

    if (!rpc.length) {
        rpc = clusterApiUrl("devnet")
    }

    const connection = new Connection(rpc, "confirmed");

    return connection;
}

export async function uploadFilePinata(name: string, filename: string, type: string): Promise<PinResponse> {
    const blob = new Blob([fs.readFileSync(filename)]);
    
    const file = new File([blob], name, { type: type})

    const upload = await pinata.upload.file(file);

    return upload;
}

export async function uploadJSONPinata(json: { [label: string]: string }): Promise<PinResponse> {
    const upload = await pinata.upload.json(json);

    return upload;
}

export async function createTokenAndMint(
    connection: Connection, 
    tokenMetadata: TokenMetadata, 
    payer: Keypair,
    owner: Keypair,
    mintKeypair: Keypair,
    authority: Keypair,
    decimals: number,
    mintAmount: bigint,
): Promise<[string, string]> {
    // Calculate the minimum balance for the mint account
    const mintLen = getMintLen([ExtensionType.MetadataPointer]);
    const metadataLen = TYPE_SIZE + LENGTH_SIZE + pack(tokenMetadata).length;
    const mintLamports = await connection.getMinimumBalanceForRentExemption(mintLen + metadataLen);

    const transaction = new Transaction().add(
        SystemProgram.createAccount({
            fromPubkey: payer.publicKey,
            newAccountPubkey: mintKeypair.publicKey,
            space: mintLen,
            lamports: mintLamports,
            programId: TOKEN_2022_PROGRAM_ID,
        }),
        createInitializeMetadataPointerInstruction(
            mintKeypair.publicKey,
            authority.publicKey,
            mintKeypair.publicKey,
            TOKEN_2022_PROGRAM_ID,
        ),
        createInitializeMintInstruction(
            mintKeypair.publicKey,
            decimals,
            authority.publicKey,
            authority.publicKey,
            TOKEN_2022_PROGRAM_ID,
        ),
        createInitializeInstruction({
            programId: TOKEN_2022_PROGRAM_ID, 
            metadata: mintKeypair.publicKey,
            updateAuthority: authority.publicKey,
            mint: mintKeypair.publicKey,
            mintAuthority: authority.publicKey, 
            name: tokenMetadata.name,
            symbol: tokenMetadata.symbol,
            uri: tokenMetadata.uri,
        })
    );

    // Initialize NFT with metadata
    const initSig = await sendAndConfirmTransaction(connection, transaction, [payer, mintKeypair, authority]);
    // Create associated token account
    const sourceAccount = await createAssociatedTokenAccountIdempotent(connection, payer, mintKeypair.publicKey, owner.publicKey, {}, TOKEN_2022_PROGRAM_ID);
    // Mint NFT to associated token account
    const mintSig = await mintTo(connection, payer, mintKeypair.publicKey, sourceAccount, authority, mintAmount, [], undefined, TOKEN_2022_PROGRAM_ID);

    return [initSig, mintSig];
}