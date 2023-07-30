//! Instructions shared by all Hyperlane Sealevel Token programs.

use account_utils::{DiscriminatorData, DiscriminatorEncode, PROGRAM_INSTRUCTION_DISCRIMINATOR};
use borsh::{BorshDeserialize, BorshSerialize};
use hyperlane_core::{H256, U256};
use hyperlane_sealevel_connection_client::router::RemoteRouterConfig;
use solana_program::{
    instruction::{AccountMeta, Instruction as SolanaInstruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use hyperlane_sealevel_mailbox::mailbox_message_dispatch_authority_pda_seeds;

use crate::hyperlane_token_pda_seeds;

/// Instructions shared by all Hyperlane Sealevel Token programs.
#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub enum Instruction {
    /// Initialize the program.
    Init(Init),
    /// Transfer tokens to a remote recipient.
    TransferRemote(TransferRemote),
    /// Enroll a remote router. Only owner.
    EnrollRemoteRouter(RemoteRouterConfig),
    /// Enroll multiple remote routers. Only owner.
    EnrollRemoteRouters(Vec<RemoteRouterConfig>),
    /// Set the interchain security module. Only owner.
    SetInterchainSecurityModule(Option<Pubkey>),
    /// Transfer ownership of the program. Only owner.
    TransferOwnership(Option<Pubkey>),
}

impl DiscriminatorData for Instruction {
    const DISCRIMINATOR: [u8; Self::DISCRIMINATOR_LENGTH] = PROGRAM_INSTRUCTION_DISCRIMINATOR;
}

/// Instruction data for initializing the program.
#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub struct Init {
    /// The address of the mailbox contract.
    pub mailbox: Pubkey,
    /// The interchain security module.
    pub interchain_security_module: Option<Pubkey>,
    /// The local decimals.
    pub decimals: u8,
    /// The remote decimals.
    pub remote_decimals: u8,
}

/// Instruction data for transferring `amount_or_id` token to `recipient` on `destination` domain.
#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub struct TransferRemote {
    /// The destination domain.
    pub destination_domain: u32,
    /// The remote recipient.
    pub recipient: H256,
    /// The amount or ID of the token to transfer.
    pub amount_or_id: U256,
}

/// Gets an instruction to initialize the program. This provides only the
/// account metas required by the library, and consuming programs are expected
/// to add the accounts for their own use.
pub fn init_instruction(
    program_id: Pubkey,
    payer: Pubkey,
    init: Init,
) -> Result<SolanaInstruction, ProgramError> {
    let (token_key, _token_bump) =
        Pubkey::try_find_program_address(hyperlane_token_pda_seeds!(), &program_id)
            .ok_or(ProgramError::InvalidSeeds)?;

    let (dispatch_authority_key, _dispatch_authority_bump) = Pubkey::try_find_program_address(
        mailbox_message_dispatch_authority_pda_seeds!(),
        &program_id,
    )
    .ok_or(ProgramError::InvalidSeeds)?;

    let ixn = Instruction::Init(init);

    // Accounts:
    // 0.   [executable] The system program.
    // 1.   [writable] The token PDA account.
    // 2.   [writable] The dispatch authority PDA account.
    // 3.   [signer] The payer and access control owner.
    // 4..N [??..??] Plugin-specific accounts.
    let accounts = vec![
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new(token_key, false),
        AccountMeta::new(dispatch_authority_key, false),
        AccountMeta::new(payer, true),
    ];

    let instruction = SolanaInstruction {
        program_id,
        data: ixn.encode()?,
        accounts,
    };

    Ok(instruction)
}

/// Enrolls remote routers.
pub fn enroll_remote_routers_instruction(
    program_id: Pubkey,
    owner_payer: Pubkey,
    configs: Vec<RemoteRouterConfig>,
) -> Result<SolanaInstruction, ProgramError> {
    let (token_key, _token_bump) =
        Pubkey::try_find_program_address(hyperlane_token_pda_seeds!(), &program_id)
            .ok_or(ProgramError::InvalidSeeds)?;

    let ixn = Instruction::EnrollRemoteRouters(configs);

    // Accounts:
    // 0. [writeable] The token PDA account.
    // 1. [signer] The owner.
    let accounts = vec![
        AccountMeta::new(token_key, false),
        AccountMeta::new(owner_payer, true),
    ];

    let instruction = SolanaInstruction {
        program_id,
        data: ixn.encode()?,
        accounts,
    };

    Ok(instruction)
}