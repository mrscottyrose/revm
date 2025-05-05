use crate::{
    gas, interpreter::Interpreter,
    primitives::{Spec::*, B256, U256},
    Host,
    InstructionContext, InterpreterResult,
};

/// Get the address of the executing account.
#[inline]
pub fn address<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::BASE);
    let mut tmp = [0u8; 32];
    tmp[12..].copy_from_slice(&context.interp.contract.address.0); // left-pad with zeros
    context.interp.stack.push(U256::from_be_bytes(tmp))?;
    Ok(())
}

/// Get balance of the given account.
#[inline]
pub fn balance<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    check!(context.interp, Istanbul);
    gas!(context.interp, gas::balance_cost(context.interp.spec()));

    pop_address!(context.interp, address);
    let (balance, is_cold) = context.host.balance(address)?;
    gas!(context.interp, gas::cold_account_access_cost(context.interp.spec(), is_cold));
    context.interp.stack.push(balance)?;
    Ok(())
}

/// Get balance of the given account.
/// EIP-2929: BALANCE opcode reads warm storage
#[inline]
pub fn balance_pre_istanbul<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::BALANCE);
    pop_address!(context.interp, address);
    let (balance, is_cold) = context.host.balance(address)?;
    debug_assert!(!is_cold, "Balance pre-istanbul is always warm");
    context.interp.stack.push(balance)?;
    Ok(())
}

/// Get the originator of the transaction.
#[inline]
pub fn origin<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::BASE);
    let mut tmp = [0u8; 32];
    tmp[12..].copy_from_slice(&context.host.env().tx.caller.0);
    context.interp.stack.push(U256::from_be_bytes(tmp))?;
    Ok(())
}

/// Get the caller of the execution frame.
#[inline]
pub fn caller<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::BASE);
    let mut tmp = [0u8; 32];
    tmp[12..].copy_from_slice(&context.interp.contract.caller.0);
    context.interp.stack.push(U256::from_be_bytes(tmp))?;
    Ok(())
}

/// Get the value of the call.
#[inline]
pub fn callvalue<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::BASE);
    context.interp.stack.push(context.interp.contract.value)?;
    Ok(())
}

/// Get the gas price of the transaction.
#[inline]
pub fn gasprice<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::BASE);
    context.interp.stack.push(context.host.env().effective_gas_price())?;
    Ok(())
}

/// Get the base fee of the current block.
#[inline]
pub fn basefee<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    // EIP-3198: BASEFEE opcode
    check!(context.interp, London);
    gas!(context.interp, gas::BASE);
    context.interp.stack.push(context.host.env().block.basefee)?;
    Ok(())
}

/// Get the extcodesize of the given account.
#[inline]
pub fn extcodesize<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    check!(context.interp, Berlin);
    gas!(context.interp, gas::extcode_cost(context.interp.spec()));
    pop_address!(context.interp, address);
    let (codehash, is_cold) = context.host.code_hash(address)?;
    gas!(context.interp, gas::cold_account_access_cost(context.interp.spec(), is_cold));
    context.interp.stack.push(U256::from(
        context
            .host
            .bytecode_len(address, codehash)
            .unwrap_or_default(),
    ))?;
    Ok(())
}

/// Get the extcodesize of the given account.
/// Berlin spec reduced gas cost.
#[inline]
pub fn extcodesize_pre_berlin<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::EXTCODEHASH); //TODO check gas cost
    pop_address!(context.interp, address);
    let (codehash, is_cold) = context.host.code_hash(address)?;
    debug_assert!(!is_cold);
    context.interp.stack.push(U256::from(
        context
            .host
            .bytecode_len(address, codehash)
            .unwrap_or_default(),
    ))?;
    Ok(())
}

/// Get the extcodehash of the given account.
#[inline]
pub fn extcodehash<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    // EIP-1052: EXTCODEHASH opcode
    check!(context.interp, Constantinople);
    check!(context.interp, Berlin);
    gas!(context.interp, gas::extcode_cost(context.interp.spec()));

    pop_address!(context.interp, address);
    let (code_hash, is_cold) = context.host.code_hash(address)?;
    gas!(context.interp, gas::cold_account_access_cost(context.interp.spec(), is_cold));
    context.interp.stack.push(U256::from_be_bytes(code_hash.0))?;
    Ok(())
}

/// Get the extcodehash of the given account.
/// Berlin spec reduces gas cost.
#[inline]
pub fn extcodehash_pre_berlin<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    // EIP-1052: EXTCODEHASH opcode
    check!(context.interp, Constantinople);
    gas!(context.interp, gas::EXTCODEHASH);

    pop_address!(context.interp, address);
    let (code_hash, is_cold) = context.host.code_hash(address)?;
    debug_assert!(!is_cold);
    context.interp.stack.push(U256::from_be_bytes(code_hash.0))?;
    Ok(())
}

/// Get the code size of the executing account.
#[inline]
pub fn codesize<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::BASE);
    context
        .interp
        .stack
        .push(U256::from(context.interp.contract.bytecode.len()))?;
    Ok(())
}

/// Get the code hash of the executing account.
#[inline]
pub fn codehash<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::BASE);
    let hash = if context.interp.contract.bytecode.is_empty() {
        context.interp.contract.code_hash
    } else {
        context.interp.contract.bytecode.hash_slow()
    };
    context.interp.stack.push(U256::from_be_bytes(hash.0))?;
    Ok(())
}

/// Get the block hash of the given block number.
#[inline]
pub fn blockhash<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::BLOCKHASH);
    let number = context.interp.stack.pop_u256()?;
    context.interp.stack.push(context.host.block_hash(number)?)?;
    Ok(())
}

/// Get the block coinbase address.
#[inline]
pub fn coinbase<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::BASE);
    let mut tmp = [0u8; 32];
    tmp[12..].copy_from_slice(&context.host.env().block.coinbase.0);
    context.interp.stack.push(U256::from_be_bytes(tmp))?;
    Ok(())
}

/// Get the block timestamp.
#[inline]
pub fn timestamp<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::BASE);
    context.interp.stack.push(context.host.env().block.timestamp)?;
    Ok(())
}

/// Get the block number.
#[inline]
pub fn number<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    gas!(context.interp, gas::BASE);
    context.interp.stack.push(context.host.env().block.number)?;
    Ok(())
}

/// Get the block difficulty.
#[inline]
pub fn difficulty<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::BASE);
    context.interp.stack.push(context.host.env().block.difficulty)?;
    Ok(())
}

/// Get the block gas limit.
#[inline]
pub fn gaslimit<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::BASE);
    context.interp.stack.push(context.host.env().block.gas_limit)?;
    Ok(())
}

/// Get the chain ID.
#[inline]
pub fn chainid<ITy: InterpreterTypes, H: Host + ?Sized>(context: &mut InstructionContext<'_, ITy, H>) -> InterpreterResult {
    // EIP-1344: ChainID opcode
    check!(context.interp, Istanbul);
    gas!(context.interp, gas::BASE);
    context.interp.stack.push(context.host.env().cfg.chain_id)?;
    Ok(())
}

/// Get the chain ID.
#[inline]
pub fn chainid_pre_istanbul<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    // EIP-1344: ChainID opcode
    gas!(context.interp, gas::BASE);
    context.interp.stack.push(context.host.env().cfg.chain_id)?;
    Ok(())
}

/// Get the self balance.
#[inline]
pub fn selfbalance<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    // EIP-1884: Repricing for trie-size-dependent opcodes
    check!(context.interp, Istanbul);
    gas!(context.interp, gas::LOW);
    let (balance, _is_cold) = context.host.balance(context.interp.contract.address)?;
    context.interp.stack.push(balance)?;
    Ok(())
}

/// Get the self balance.
#[inline]
pub fn selfbalance_pre_istanbul<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    gas!(context.interp, gas::LOW);
    let (balance, _is_cold) = context.host.balance(context.interp.contract.address)?;
    context.interp.stack.push(balance)?;
    Ok(())
}

/// Get the blob hash.
#[inline]
pub fn blobhash<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    // EIP-4844: Shard Blob Transactions
    check!(context.interp, Cancun);
    gas!(context.interp, gas::BLOBHASH);
    let index = context.interp.stack.pop_u256()?;
    let blob_hash = context.host.env().blob_hashes.get(index.try_into().unwrap_or(usize::MAX))
        .map(|b| U256::from_be_bytes(b.0))
        .unwrap_or_default();
    context.interp.stack.push(blob_hash)?;
    Ok(())
}

/// Get the blob base fee.
#[inline]
pub fn blobbasefee<ITy: InterpreterTypes, H: Host + ?Sized>(
    context: &mut InstructionContext<'_, ITy, H>,
) -> InterpreterResult {
    // EIP-7516: BLOBBASEFEE opcode
    check!(context.interp, Cancun);
    gas!(context.interp, gas::BASE);
    // TODO: This is not spec compliant, fix host interface to be able to return Option
    context.interp.stack.push(context.host.env().block.get_blob_gasprice().map_or(U256::ZERO, U256::from))?;
    Ok(())
} 