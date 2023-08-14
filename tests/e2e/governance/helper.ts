import BN from 'bn.js'
import {Transaction, VoteType} from '../../../typechain-generated/types-arguments/my_governor'
import ContractGovernance from '../../../typechain-generated/contracts/my_governor'
import {KeyringPair} from '@polkadot/keyring/types'
import {expect} from 'chai'

export class GovernorHelper {
  private proposal: Transaction | undefined;
  private description: string | undefined;
  private governor: ContractGovernance | undefined;
  private proposalId: number[] | undefined;

  constructor(governor: ContractGovernance) {
    this.governor = governor
  }

  addProposal(callee: string, selector: number[], input: (string | number | BN)[], description: string) {
    this.proposal = {
      callee: callee,
      selector: selector,
      destination: callee,
      input: input,
      transferredValue: 0,
      gasLimit: 1000000000000
    }
    this.description = description
  }
  
  getProposalId(): number[] | undefined {
    return this.proposalId
  }

  async propose(proposer: KeyringPair) {
    if (this.proposal === undefined || this.description === undefined) {
      throw new Error('Proposal not set')
    }
    // todo: check how proposal Id is calculated
    this.proposalId = (await this.governor?.query.propose([this.proposal!], this.description!))?.value.unwrapRecursively().ok
    await this.governor?.withSigner(proposer).tx.propose([this.proposal!], this.description!)
  }

  async waitForSnapshot() {
    const proposalSnapshot = (await this.governor?.query.proposalSnapshot(this.proposalId as unknown as number[]))?.value.unwrapRecursively().ok
    await this.governor?.tx.setBlockTimestamp(proposalSnapshot as number)
  }

  async castVote(voter: KeyringPair, vote: VoteType) {
    if (this.proposalId === undefined) {
      throw new Error('Proposal Id not set')
    }
    await this.governor?.withSigner(voter).tx.castVote(this.proposalId, vote)
  }

  async waitForDeadline() {
    const proposalDeadline = (await this.governor?.query.proposalDeadline(this.proposalId as unknown as number[]))?.value.unwrapRecursively().ok
    await this.governor?.tx.setBlockTimestamp(proposalDeadline as number)
  }
}