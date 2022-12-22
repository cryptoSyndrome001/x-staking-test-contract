import * as anchor from '@project-serum/anchor';
import { XStaking } from '../target/types/x_staking';
import { TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";
import {use as chaiUse, assert as assert_true} from 'chai'
import {assert_eq} from 'mocha-as-assert'
import chaiAsPromised from 'chai-as-promised'
chaiUse(chaiAsPromised)

describe('x-staking', () => {

  // Constants
  const TREASURY_TAG = Buffer.from("treasury");
  const TREASURY_VAULT_TAG = Buffer.from("treasury-vault");
  const POS_MINT_TAG = Buffer.from("pos-mint");
  const USER_POS_VAULT_TAG = Buffer.from("user-pos-vault");

  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.XStaking as anchor.Program<XStaking>;
  const programId = program.programId
  
  const treasuryAdminKeypair = anchor.web3.Keypair.fromSecretKey(new Uint8Array([77,208,114,7,139,212,103,94,237,253,48,38,14,224,165,191,66,46,169,145,18,112,97,175,233,240,161,98,25,242,7,252,208,249,117,252,63,51,225,57,170,88,58,227,162,23,52,244,27,77,68,53,139,68,161,54,205,226,129,29,14,84,206,123]));
  const treasuryAdmin = treasuryAdminKeypair.publicKey;
  const userKeypair = anchor.web3.Keypair.fromSecretKey(new Uint8Array([193,175,203,110,116,69,233,189,129,146,244,26,38,246,84,86,129,192,248,25,62,249,10,3,152,68,88,16,13,27,182,10,47,249,117,244,173,148,158,132,48,71,199,138,145,178,194,132,56,56,174,35,108,239,223,54,150,232,194,12,224,56,171,92]));
  const user = userKeypair.publicKey;
  let treasuryTokenMint: Token = null;
  const mintAmount = 10_000_000_000_000; // 10000 POS

  let userTreasuryVault = null;

  it('Is Initialize!', async () => {
    console.log("treasuryAdmin", treasuryAdmin.toBase58())
    console.log("user", user.toBase58())

    await safeAirdrop(program.provider.connection, treasuryAdmin, 1000000000)
    await safeAirdrop(program.provider.connection, user, 1000000000)

    treasuryTokenMint = await Token.createMint(
      program.provider.connection,
      treasuryAdminKeypair,
      treasuryAdmin,
      null,
      9,
      TOKEN_PROGRAM_ID
    );
    userTreasuryVault = await treasuryTokenMint.createAccount(user);
    await treasuryTokenMint.mintTo(
      userTreasuryVault,
      treasuryAdmin,
      [],
      mintAmount
    );
  })
  let posToken: Token = null;
  it('CreateTreasury !', async () => {
    const treasury = await pda([TREASURY_TAG, treasuryTokenMint.publicKey.toBuffer(), treasuryAdmin.toBuffer()], programId)
    const treasuryVault = await pda([TREASURY_VAULT_TAG, treasury.toBuffer()], programId)
    const posMint = await pda([POS_MINT_TAG, treasury.toBuffer()], programId)
    posToken = new Token(program.provider.connection, posMint, TOKEN_PROGRAM_ID, treasuryAdminKeypair)
    
    const tx = await program.rpc.createTreasury(
      {
        accounts: {
          treasury,
          treasuryMint: treasuryTokenMint.publicKey,
          posMint,
          treasuryVault,
          authority: treasuryAdmin,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY
        },
        signers: [treasuryAdminKeypair]
      });
    console.log("tx = ", tx);
      
    const treasuryData = await program.account.treasury.fetch(treasury);
    assert_eq(treasuryData.authority, treasuryAdmin)
    assert_true(treasuryData.treasuryMint.equals(treasuryTokenMint.publicKey), "treasuryMint")
    assert_true(treasuryData.treasuryVault.equals(treasuryVault), "treasuryVault")
    assert_true(treasuryData.posMint.equals(posMint), "posMint")
  });

  const stakeAmount = 100_000_000_000; //100 POS
  it('Stake !', async () => {
    const treasury = await pda([TREASURY_TAG, treasuryTokenMint.publicKey.toBuffer(), treasuryAdmin.toBuffer()], programId)
    const treasuryVault = await pda([TREASURY_VAULT_TAG, treasury.toBuffer()], programId)
    const posMint = await pda([POS_MINT_TAG, treasury.toBuffer()], programId)
    const userPosVault = await pda([USER_POS_VAULT_TAG, posMint.toBuffer(), user.toBuffer()], programId)
    let treasuryAmountBefore = ((await treasuryTokenMint.getAccountInfo(treasuryVault)).amount as anchor.BN).toNumber()
    let userPosAmountBefore = 0;
    try{
      userPosAmountBefore = ((await posToken.getAccountInfo(userPosVault)).amount as anchor.BN).toNumber()
    }catch(e){}

    const tx = await program.rpc.stake(
      new anchor.BN(stakeAmount),
      {
        accounts: {
          treasury,
          posMint,
          treasuryVault,
          userVault: userTreasuryVault,
          userPosVault,
          authority: user,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        },
        signers: [userKeypair]
      });
    console.log("tx = ", tx);

    let treasuryAmountAfter = ((await treasuryTokenMint.getAccountInfo(treasuryVault)).amount as anchor.BN).toNumber();
    let userPosAmountAfter = ((await posToken.getAccountInfo(userPosVault)).amount as anchor.BN).toNumber();
    assert_true(treasuryAmountAfter - treasuryAmountBefore === stakeAmount, "stakeAmount")
    assert_true(userPosAmountAfter - userPosAmountBefore === stakeAmount, "stakeAmount")
  });

  const redeemAmount = 10_000_000_000; //10 POS
  it('Redeem !', async () => {
    const treasury = await pda([TREASURY_TAG, treasuryTokenMint.publicKey.toBuffer(), treasuryAdmin.toBuffer()], programId)
    const treasuryVault = await pda([TREASURY_VAULT_TAG, treasury.toBuffer()], programId)
    const posMint = await pda([POS_MINT_TAG, treasury.toBuffer()], programId)
    const userPosVault = await pda([USER_POS_VAULT_TAG, posMint.toBuffer(), user.toBuffer()], programId)
    let treasuryAmountBefore = ((await treasuryTokenMint.getAccountInfo(treasuryVault)).amount as anchor.BN).toNumber()
    let userPosAmountBefore = ((await posToken.getAccountInfo(userPosVault)).amount as anchor.BN).toNumber()
    const tx = await program.rpc.redeem(
      new anchor.BN(redeemAmount),
      {
        accounts: {
          treasury,
          posMint,
          treasuryVault,
          userVault: userTreasuryVault,
          userPosVault,
          authority: user,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        signers: [userKeypair]
      });
    console.log("tx = ", tx);

    let treasuryAmountAfter = ((await treasuryTokenMint.getAccountInfo(treasuryVault)).amount as anchor.BN).toNumber()
    let userPosAmountAfter = ((await posToken.getAccountInfo(userPosVault)).amount as anchor.BN).toNumber()
    assert_true(treasuryAmountBefore - treasuryAmountAfter === redeemAmount, "redeemAmount")
    assert_true(userPosAmountBefore - userPosAmountAfter === redeemAmount, "redeemAmount")
  });
});

async function safeAirdrop(connection: anchor.web3.Connection, destination: anchor.web3.PublicKey, amount = 100000000) {
  while (await connection.getBalance(destination) < amount){
    try{
      // Request Airdrop for user
      await connection.confirmTransaction(
        await connection.requestAirdrop(destination, 100000000),
        "confirmed"
      );
    }catch{}
    
  };
}

async function pda(seeds: (Buffer | Uint8Array)[], programId: anchor.web3.PublicKey) {
  const [pdaKey] = 
      await anchor.web3.PublicKey.findProgramAddress(
        seeds,
        programId,
      );
  return pdaKey
}

async function delay(ms: number) {
  return new Promise( resolve => setTimeout(resolve, ms) );
}
