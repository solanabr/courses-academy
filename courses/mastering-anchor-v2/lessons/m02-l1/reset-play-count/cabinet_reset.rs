// Reference solution for the m02-l1 challenge — the test half.
// Drop in at `programs/cabinet-counter/tests/cabinet_reset.rs`, beside the
// `cabinet.rs` you wrote in the lab.
//
// The interesting assertion is the second one. A `reset` that zeroes BOTH
// fields sails past a lazy test that only checks `play_count`; asserting the
// high score survived is what actually catches that bug. Cargo compiles every
// tests/*.rs as its own crate root, which is why the little `send` helper is
// repeated here instead of imported from cabinet.rs.

use {
    anchor_lang::{
        bytemuck, programs::System, solana_program::instruction::Instruction, Id,
        InstructionData, ToAccountMetas,
    },
    anchor_lang::prelude::Address,
    anchor_v2_testing::{Keypair, Message, Signer, VersionedMessage, VersionedTransaction},
    cabinet_counter::{accounts, instruction, Cabinet},
};

fn send(
    svm: &mut anchor_v2_testing::LiteSVM,
    payer: &Keypair,
    ix: Instruction,
) {
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer]).unwrap();
    svm.send_transaction(tx).unwrap();
}

#[test]
fn reset_zeroes_only_play_count() {
    let program_id = cabinet_counter::id();

    let mut svm = anchor_v2_testing::svm();
    let bytes = include_bytes!("../../../target/deploy/cabinet_counter.so");
    svm.add_program(program_id, bytes).unwrap();

    let player = Keypair::new();
    svm.airdrop(&player.pubkey(), 1_000_000_000).unwrap();

    let (cabinet, _bump) =
        Address::find_program_address(&[b"cabinet", player.pubkey().as_ref()], &program_id);

    let init_ix = Instruction::new_with_bytes(
        program_id,
        &instruction::Init {}.data(),
        accounts::Init {
            cabinet,
            player: player.pubkey(),
            system_program: System::id(),
        }
        .to_account_metas(None),
    );
    send(&mut svm, &player, init_ix);

    // A couple of plays, so the tally and the marquee both hold real numbers:
    // play_count = 2, high_score = 4200.
    for score in [1500u64, 4200] {
        let inc_ix = Instruction::new_with_bytes(
            program_id,
            &instruction::Increment { score }.data(),
            accounts::Increment {
                cabinet,
                player: player.pubkey(),
            }
            .to_account_metas(None),
        );
        send(&mut svm, &player, inc_ix);
    }

    // The operator clears the shift.
    let reset_ix = Instruction::new_with_bytes(
        program_id,
        &instruction::Reset {}.data(),
        accounts::Reset {
            cabinet,
            player: player.pubkey(),
        }
        .to_account_metas(None),
    );
    send(&mut svm, &player, reset_ix);

    let raw = svm.get_account(&cabinet).unwrap().data;
    let state: &Cabinet = bytemuck::from_bytes(&raw[8..8 + core::mem::size_of::<Cabinet>()]);

    // The tally is cleared...
    assert_eq!(state.play_count.get(), 0, "reset clears the play tally");
    // ...and the marquee survived. This line is the whole point: a reset that
    // wipes both fields fails HERE, not at the play_count assert above.
    assert_eq!(state.high_score.get(), 4200, "the all-time high must survive a reset");
}
