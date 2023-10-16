
import {
  Connection,
  Keypair,
  Signer,
  PublicKey,
  Transaction,
  TransactionSignature,
  ConfirmOptions,
  sendAndConfirmRawTransaction,
  RpcResponseAndContext,
  SimulatedTransactionResponse,
  Commitment,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import * as splToken from '@solana/spl-token'
import fs from 'fs'
import * as anchor from '@project-serum/anchor'

const CLIENT_DATA_SIZE = 115

const sleep = (ms : number) => {
    return new Promise(resolve => setTimeout(resolve, ms));
};

let programId = new PublicKey('8dBdRbEoQz6bpiuf39kbegrbi6HVUDkkVxjvLC63i3mM')  // devnet address
// let programId = new PublicKey('AjcQtY8eL1MjpvfTFoz3oqr3o9njaHWJCk9DwD8hSx2i')  // testnet address
// let programId = new PublicKey('2LW95Az7So2D2XQevaTzs686qmc4RqCJsQ7b6bzGtFdd') // localnet

const idl=JSON.parse(fs.readFileSync('src/solana_anchor.json','utf8'))

export async function initializePresale(
    conn : Connection,
    authority : Keypair,
    presale : Keypair,
    presale_pot : PublicKey,
    token_for_sale : PublicKey,
    token_being_raised : PublicKey,
    min_allocation : number,
    max_allocation : number,
    hardcap : number,
    token_per_usd_numberator : number,
    token_per_usd_denominator : number,
    token_percentage_distributed : number,
    ){
    // console.log("initializing presale")
    let wallet = new anchor.Wallet(authority)
    let provider = new anchor.Provider(conn,wallet,anchor.Provider.defaultOptions())
    const program = new anchor.Program(idl, programId,provider)
    try {
        await  program.rpc.initializePresale(
            new anchor.BN(min_allocation),
            new anchor.BN(max_allocation),
            new anchor.BN(hardcap),
            new anchor.BN(token_per_usd_numberator),
            new anchor.BN(token_per_usd_denominator),
            new anchor.BN(token_percentage_distributed),
            {
                accounts:{
                    presale : presale.publicKey,
                    authority : authority.publicKey,
                    presalePot : presale_pot,
                    tokenForSale : token_for_sale,
                    tokenBeingRaised : token_being_raised,
                    systemProgram : anchor.web3.SystemProgram.programId,
                },
                signers: [authority,presale]
            }
        )
    } catch (err) {
        console.log(err)
    }
    await sleep(100)
    // const account = await program.account.presaleData.fetch(presale.publicKey)
    // console.log(account)
}

export async function initializeClient(
    conn : Connection,
    authority : Keypair,
    client : Keypair,
    client_pot : PublicKey,
    presale : PublicKey,
    ){
    // console.log("Init Client Data")
    let wallet = new anchor.Wallet(authority)
    let provider = new anchor.Provider(conn,wallet,anchor.Provider.defaultOptions())
    const program = new anchor.Program(idl, programId,provider)
    try {
        await program.rpc.initializeClient(
            {
                accounts:{
                    authority : authority.publicKey,
                    client : client.publicKey,
                    clientPot : client_pot,
                    presale : presale,
                    systemProgram : anchor.web3.SystemProgram.programId,
                },
                signers : [authority,client]
            }
        )
    } catch(err) {
        console.log(err)
    }
    await sleep(100)
    // console.log(await program.account.clientData.fetch(client.publicKey))    
    // console.log("End")
}

export async function addToWhitelist(
    conn : Connection,
    authority : Keypair,
    member : PublicKey,
    client : PublicKey,
    presale : PublicKey,
    ){
    // console.log("Add to whitelist")
    let wallet = new anchor.Wallet(authority)
    let provider = new anchor.Provider(conn,wallet,anchor.Provider.defaultOptions())
    const program = new anchor.Program(idl, programId,provider)
    try {
        await program.rpc.addToWhitelist(
            {
                accounts:{
                    authority : authority.publicKey,
                    member : member,
                    client : client,
                    presale : presale,
                },
                signers : [authority]
            }
        )
    } catch (err) {
        console.log(err)
    }
    await sleep(100)
    // console.log(await program.account.clientData.fetch(client))
    
    // console.log("End")
}

export async function startPresale(
    conn : Connection,
    authority : Keypair,
    presale : PublicKey,
    ){
    // console.log("Starting Presale...")
    let wallet = new anchor.Wallet(authority)
    let provider = new anchor.Provider(conn,wallet,anchor.Provider.defaultOptions())
    const program = new anchor.Program(idl, programId,provider)
    try{
        await program.rpc.startPresale(
            {
                accounts:{
                    authority : authority.publicKey,
                    presale : presale,
                },
                signers : [authority]
            }
        )
    } catch (err) {
        console.log(err)
    }
    await sleep(100)
    // console.log(await program.account.presaleData.fetch(presale))
    
    // console.log("Presale Start")    
}

export async function stopPresale(
    conn : Connection,
    authority : Keypair,
    presale : PublicKey,
    ){
    // console.log("Stopping Presale...")
    let wallet = new anchor.Wallet(authority)
    let provider = new anchor.Provider(conn,wallet,anchor.Provider.defaultOptions())
    const program = new anchor.Program(idl, programId,provider)
    try {
        await program.rpc.stopPresale(
            {
                accounts:{
                    authority : authority.publicKey,
                    presale : presale,
                },
                signers : [authority]
            }
        )
    } catch (err) {
        console.log(err)
    }
    await sleep(100)
    // console.log(await program.account.presaleData.fetch(presale))
    
    // console.log("Presale Stop")    
}

export async function stopWhitelist(
    conn : Connection,
    authority : Keypair,
    presale : PublicKey,
    ){
    // console.log("Stopping Whitelist...")
    let wallet = new anchor.Wallet(authority)
    let provider = new anchor.Provider(conn,wallet,anchor.Provider.defaultOptions())
    const program = new anchor.Program(idl, programId,provider)
    try {
        await program.rpc.stopWhitelist(
            {
                accounts:{
                    authority : authority.publicKey,
                    presale : presale,
                },
                signers : [authority]
            }
        )
    } catch (err) {
        console.log(err)
    }
    await sleep(100)
    // console.log(await program.account.presaleData.fetch(presale))
    
    // console.log("Whitelist Stop")    
}

export async function buy(
    conn : Connection,
    bidder : Keypair,
    bidder_token : PublicKey,
    mint : PublicKey,
    presale : PublicKey,
    client : PublicKey,
    amount : number,
    ){
    // console.log("Buying...")
    let wallet = new anchor.Wallet(bidder)
    let provider = new anchor.Provider(conn,wallet,anchor.Provider.defaultOptions())
    const program = new anchor.Program(idl, programId,provider)
    let presaleData = await program.account.presaleData.fetch(presale)
    try {
        await program.rpc.buy(
            new anchor.BN(amount),
            {
                accounts:{
                    bidder : bidder.publicKey,
                    bidderToken : bidder_token,
                    presalePot : presaleData.presalePot,
                    mint : mint,
                    presale : presale,
                    client : client,
                    tokenProgram : splToken.TOKEN_PROGRAM_ID,
                },
                signers : [bidder]
            }
        )
    } catch (err) {
        console.log(err)
    }
    await sleep(100)
    // console.log(await program.account.presaleData.fetch(presale))
    // console.log(await program.account.clientData.fetch(client))
    // console.log("End")
}

export async function distributeToken(
    conn : Connection,
    authority : Keypair,
    auth_token : PublicKey,
    mint : PublicKey,
    presale : PublicKey,   
    client : PublicKey,
    percentage_of_amount_owed : number
    ){
    // console.log("Distributing...")
    let wallet = new anchor.Wallet(authority)
    let provider = new anchor.Provider(conn,wallet,anchor.Provider.defaultOptions())
    const program = new anchor.Program(idl, programId,provider)
    let clientData = await program.account.clientData.fetch(client)

    if(clientData.amount == 0){
        // console.log("amount is 0")
        return;
    }

    if(clientData.alreadyPaid){
        // console.log("already paid")
        return;
    }
    try {
        await program.rpc.distributeToken(
            new anchor.BN(percentage_of_amount_owed),
            {
                accounts:{
                    authority : authority.publicKey,
                    authToken : auth_token,
                    clientPot : clientData.clientPot,
                    mint : mint,
                    presale : presale,
                    client : client,
                    tokenProgram : splToken.TOKEN_PROGRAM_ID,
                },
                signers : [authority]
            }
        )
    } catch(err) {
        console.log(err)
    } 
    await sleep(100)
    // console.log(await program.account.clientData.fetch(client))
    // console.log("End")   
}

export async function distributeTokens(
    conn : Connection,
    authority : Keypair,
    auth_token : PublicKey,
    mint : PublicKey,
    presale : PublicKey,
    percentage_of_amount_owed : number,
    ){
    let resp = await conn.getProgramAccounts(
        programId,
        {
            dataSlice : {
                length : 0,
                offset : 0
            },
            filters : [
                {
                    dataSize : CLIENT_DATA_SIZE
                },
                {
                    memcmp : {
                        offset : 8,
                        bytes : presale.toBase58()
                    }
                }
            ]
        }
    )

    for(let i in resp) {
        try {
            await distributeToken(conn,authority,auth_token,mint,presale,resp[i].pubkey,percentage_of_amount_owed)
        } catch(err) {
            // console.log(err)
        }       
    }
}

export async function setAuthority(
    conn : Connection,
    authority : Keypair,
    new_authority : PublicKey,
    presale : PublicKey,
    ){
    let wallet = new anchor.Wallet(authority)
    let provider = new anchor.Provider(conn,wallet,anchor.Provider.defaultOptions())
    const program = new anchor.Program(idl, programId,provider)
    let presaleData = await program.account.presaleData.fetch(presale)
    // try {
        await program.rpc.setAuthority(
            {
                accounts:{
                    authority : authority.publicKey,
                    newAuthority : new_authority,
                    presalePot : presaleData.presalePot,
                    presale : presale,
                    tokenProgram : splToken.TOKEN_PROGRAM_ID,
                },
                signers : [authority]
            }
        )
    // } catch (err) {
    //     console.log(err)
    // }
    await sleep(100)
}