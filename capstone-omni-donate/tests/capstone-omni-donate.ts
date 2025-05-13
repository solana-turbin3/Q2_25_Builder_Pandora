import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { CapstoneOmniDonate } from "../target/types/capstone_omni_donate";
import { 
  Account,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
  TOKEN_2022_PROGRAM_ID,
  getAccount
} from "@solana/spl-token";

import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
// import { BN } from "bn.js";
import { confirmTransaction } from "@solana-developers/helpers";
import assert from 'chai';



describe("capstone-omni-donate", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.getProvider();
  const connection = provider.connection;

  const program = anchor.workspace.capstoneOmniDonate as Program<CapstoneOmniDonate>;

  const programId = program.programId;
  const tokenProgram = TOKEN_PROGRAM_ID;

  let oracle = Keypair.generate()
  let creator = Keypair.generate();

  let donor = anchor.web3.Keypair.generate();
  let beneficiary = anchor.web3.Keypair.generate();

  let id = new BN(100);
  let campaign: anchor.web3.PublicKey, configAccount, weather: anchor.web3.PublicKey, vaultMint, vault, donorAta, configAta, beneficiaryAta;

  let protocol_address = Keypair.generate();
  let fee_collector = Keypair.generate();


  before("Create and airdrop to accounts", async () => {
    
    await airdrop(connection, creator.publicKey, 2);
    await airdrop(connection, donor.publicKey, 2);
    await airdrop(connection, beneficiary.publicKey, 2);
    await airdrop(connection, oracle.publicKey, 2);

    campaign = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("campaign"),
        creator.publicKey.toBuffer(),
        id.toArrayLike(Buffer, "le", 8)
      ],
    program.programId)[0];

    weather = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("weather"),
        creator.publicKey.toBuffer(),
        id.toArrayLike(Buffer, "le", 8)
      ],
    program.programId)[0];

    // Define Config Account for the Protocol
    configAccount = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("config"),
        creator.publicKey.toBuffer()
      ],
    program.programId)[0];
  

    vaultMint = await createMint(
      connection,
      creator,
      creator.publicKey,
      null,
      9
    );

    beneficiaryAta = await getOrCreateAssociatedTokenAccount (
      connection,
      beneficiary,
      vaultMint,
      beneficiary.publicKey,
      false
    );

    donorAta = await getOrCreateAssociatedTokenAccount(
      connection,
      donor,
      vaultMint,
      donor.publicKey,
      false,
    );

    
    configAta = await getOrCreateAssociatedTokenAccount(
      connection,
      donor,
      vaultMint,
      configAccount,
      true,
    );

    await mintTo(
      connection,
      donor,
      vaultMint,
      donorAta.address,
      creator,
      10000
    );
  
    vault = await getOrCreateAssociatedTokenAccount(
      connection,
      creator,
      vaultMint,
      campaign,
      true,
    );

  });

  it("Initialize the Config Account For Fee Collection", async() => {
    let fee_bps = new BN(100);

    let accounts = {
      signer: creator.publicKey,
      configAccount,
    };

    const tx = await program.methods.initConfig(
      protocol_address.publicKey,
      fee_bps,
      fee_collector.publicKey
    )
    .accounts({...accounts})
    .signers([creator])
    .rpc()

    console.log("Your transaction signature", tx);

  });
  

  it("Should Initialize a New Campaign", async () => {

    let name = "Kenya";
    let description = "for farmers";
    let condition_target = [1001, 0, 1003];

    let accounts = {
      creator: creator.publicKey,
      donor: donor.publicKey,
      donorAta: donorAta.address,
      campaign,
      vault: vault.address,
      vaultMint,
      tokenProgram
    };


    const tx = await program.methods.createCampaign(
      id,
      name,
      description,
      condition_target,
      oracle.publicKey,
      beneficiary.publicKey
    )
    .accounts({...accounts})
    .signers([creator])
    .rpc();



    console.log("Your transaction signature", tx);

  });

  it("Should Donate funds Successfully", async () => {

    let accounts = {
      creator: creator.publicKey,
      configAccount,
      configAta: configAta.address,
      donor: donor.publicKey,
      donorAta: donorAta.address,
      campaign,
      vault: vault.address,
      vaultMint,
      tokenProgram
    };

    let amount = new BN(1e2);

    const tx = await program.methods.donate(
      id,
      amount,
    )
    .accounts({...accounts})
    .signers([donor])
    .rpc();

    let vault_balance = (await getAccount(connection, vault.address)).amount;
    let protocol_balance = (await getAccount(connection, configAta.address)).amount;

    console.log("Vault Balance", vault_balance);
    console.log("Fees Collected", protocol_balance);
    console.log("Your transaction signature", tx);

  });

  it("Beneficiary Should Claim funds Successfully", async () => {

    let accounts = {
      creator: creator.publicKey,
      beneficiary: beneficiary.publicKey,
      beneficiaryAta: beneficiaryAta.address,
      donor: donor.publicKey,
      donorAta: donorAta.address,
      campaign,
      vault: vault.address,
      vaultMint,
      tokenProgram
    };


    const tx = await program.methods.claim(
      id,
    )
    .accounts({...accounts})
    .signers([beneficiary])
    .rpc();


    let beneficiary_balance = (await getAccount(connection, beneficiaryAta.address)).amount;

    console.log("Beneficiary Balance", beneficiary_balance);


    
    console.log("Your transaction signature", tx);

  });

  it("Should not allow Additional Claims", async() =>{

    let accounts = {
      creator: creator.publicKey,
      beneficiary: beneficiary.publicKey,
      beneficiaryAta: beneficiaryAta.address,
      donor: donor.publicKey,
      donorAta: donorAta.address,
      campaign,
      vault: vault.address,
      vaultMint,
      tokenProgram
    };

    try { 
      await program.methods.claim(
      id,
    )
    .accounts({...accounts})
    .signers([beneficiary])
    .rpc();

    assert.expect.fail("This Failed");
    }
    catch (err: any ){
      console.log("Should Not Allow Additional Claims");
    }

  });

  it("Oracle Should Not be able to Bypass Update CoolDown", async() => {

    let accounts = {
      oracle: oracle.publicKey,
      campaign,
      weather,
    };
    let condition_code = 1001;
    let codition = "Flooded";

    try { await program.methods.updateWeather(
      id,
      condition_code,
      codition
    ).accounts({...accounts})
    .signers([oracle])
    .rpc();
  } catch(err: any) {
    console.log("Time Not Passed for Weather Update");
  }

    
  });

  it("Should Update Fee Basis Point", async() => {


    let accounts = {
      signer: creator.publicKey,
      configAccount
    };
    let fee_bps = new BN(1e6);

    const tx = await program.methods.updateFeeBps(
      fee_bps
    ).accounts({...accounts})
    .signers([creator])
    .rpc();
  });

  it("Only Admin can Update Fee Basis Point", async() => {


    let accounts = {
      signer: creator.publicKey,
      configAccount
    };
    let fee_bps = new BN(100);

    try { await program.methods.updateFeeBps(
      fee_bps
    ).accounts({...accounts})
    .signers([oracle])
    .rpc();
    } catch(err: any){
    }

  });

  it("Update Fee Collection Address", async() => {

    let fee_address = (Keypair.generate()).publicKey;
    let is_protocol = false;

    let accounts = {
      signer: creator.publicKey,
      configAccount
    };
    let fee_bps = new BN(100);

    const tx = await program.methods.updateAddress(
      fee_address,
      is_protocol
    ).accounts({...accounts})
    .signers([creator])
    .rpc();
  });

  it("Update Protocol Address", async() => {

    let protocol_address = (Keypair.generate()).publicKey;
    let is_protocol = true;

    let accounts = {
      signer: creator.publicKey,
      configAccount
    };
    let fee_bps = new BN(100);

    const tx = await program.methods.updateAddress(
      protocol_address,
      is_protocol
    ).accounts({...accounts})
    .signers([creator])
    .rpc();
  });


});

async function airdrop(connection, address: PublicKey, amount: number) {
  let airdrop_signature = await connection.requestAirdrop(
    address,
    amount * LAMPORTS_PER_SOL
  );
  // console.log("✍🏾 Airdrop Signature: ", airdrop_signature);

  let confirmedAirdrop = await confirmTransaction(connection, airdrop_signature, "confirmed");

}
