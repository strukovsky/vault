import * as anchor from '@project-serum/anchor'
import { Program } from '@project-serum/anchor'
import { Vault } from '../target/types/vault'
import { expect } from 'chai'

describe('vault', () => {
  anchor.setProvider(anchor.AnchorProvider.env())
  const program = anchor.workspace.Vault as Program<Vault>
  const vault = anchor.web3.Keypair.generate()
  const depositEntry = anchor.web3.Keypair.generate()

  it('should be initialized!', async () => {
    await program.methods.initialize(new anchor.BN(100))
      .accounts({
        vault: vault.publicKey
      })
      .signers([vault])
      .rpc()
  })

  it('should perform a deposit', async () => {
    const amount = 5000
    await program.methods.deposit(new anchor.BN(amount))
      .accounts({
        vault: vault.publicKey,
        depositEntry: depositEntry.publicKey
      })
      .signers([depositEntry])
      .rpc()
    const depositAmount = (await program.account.depositEntry.fetch(depositEntry.publicKey)).amount
    expect(depositAmount.toNumber()).to.be.eq(amount)
  })

  it('should perform a supply from admin', async () => {
    await program.methods.supply(new anchor.BN(10000))
      .accounts({
        vault: vault.publicKey
      })
      .rpc()
  })

  it('should perform a withdrawal', async () => {
    const amount = new anchor.BN(100)
    const before = (await program.account.depositEntry.fetch(depositEntry.publicKey)).amount.toNumber()
    await program.methods.withdraw(amount)
      .accounts({
        vault: vault.publicKey,
        depositEntry: depositEntry.publicKey
      })
      .rpc()
    const after = (await program.account.depositEntry.fetch(depositEntry.publicKey)).amount.toNumber()
    expect(before - amount.toNumber()).to.be.eq(after)
  })

  it('should perform an admin withdrawal', async () => {
    const amount = new anchor.BN(500)
    const balanceBefore = await anchor.getProvider().connection.getBalance(vault.publicKey)
    await program.methods.adminWithdraw(amount).accounts(
      {
        vault: vault.publicKey
      }
    ).rpc()
    const balanceAfter = await anchor.getProvider().connection.getBalance(vault.publicKey)
    expect(balanceBefore - balanceAfter).to.be.eq(amount.toNumber())
  })
})
