/*!
 * Contract that requires 3 different accounts signatures to authorize some CCD transfers.
 */

#![cfg_attr(not(feature = "std"), no_std)]
use concordium_std::{collections::*, *};
