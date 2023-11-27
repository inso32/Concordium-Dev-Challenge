#![cfg_attr(not(feature = "std"), no_std)]

//! # A Concordium V1 smart contract
use concordium_std::*;
use core::fmt::Debug;

/// Your smart contract state.
#[derive(Serialize, SchemaType)]
pub struct State {
    // Your state
    counter: i8,
}

/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serialize, SchemaType)]
pub enum Error {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParams,
    /// Your error
    OwnerError,
    IncrementError,
    DecrementError,
}

/// Init function that creates a new smart contract.
#[init(contract = "counter")]
fn init(_ctx: &InitContext, _state_builder: &mut StateBuilder) -> InitResult<State> {
    // Your code

    Ok(State { counter: 0 })
}

pub type MyInputType = bool;


type IncrementVal = i8;
/// Receive function. The input parameter is the `IncrementVal`.
///  If the account owner does not match the contract owner, the receive function will throw [`Error::OwnerError`].
///  If the number to increment by is not positive or is zero, the receive function will throw [`Error::IncrementError`].
#[receive(
    contract = "counter",
    name = "increment",
    parameter = "IncrementVal",
    error = "Error",
    mutable
)]
fn increment(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
) -> Result<(), Error> {
    // Your code

    let param: IncrementVal = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::OwnerError
    );

    ensure!(param > 0, Error::IncrementError);
    state.counter += param;
    Ok(())
}

#[receive(
    contract = "counter",
    name = "decrement",
    parameter = "IncrementVal",
    error = "Error",
    mutable
)]
fn decrement(
    ctx: &ReceiveContext,
    host: &mut Host<State>,
) -> Result<(), Error> {
    // Your code

    let param: IncrementVal = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    ensure!(
        ctx.sender().matches_account(&ctx.owner()),
        Error::OwnerError
    );

    ensure!(param < 0, Error::DecrementError);
    state.counter += param;
    Ok(())
}


/// View function that returns the content of the state.
#[receive(contract = "counter", name = "view", return_value = "i8")]
fn view<'a, 'b>(
    _ctx: &'a ReceiveContext,
    host: &'b Host<State>,
) -> ReceiveResult<i8> {
    Ok(host.state().counter)
}
