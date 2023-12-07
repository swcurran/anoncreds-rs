use once_cell::sync::Lazy;
use std::collections::HashSet;

use crate::data_types::w3c::credential::{Contexts, Types};
use crate::data_types::w3c::uri::URI;

// Contexts
pub const W3C_CONTEXT: &str = "https://www.w3.org/2018/credentials/v1";
pub const W3C_ANONCREDS_CONTEXT: &str = "https://raw.githubusercontent.com/hyperledger/anoncreds-spec/main/data/anoncreds-w3c-context.json";

// Types
pub const W3C_CREDENTIAL_TYPE: &str = "VerifiableCredential";
pub const W3C_PRESENTATION_TYPE: &str = "VerifiablePresentation";
pub const W3C_ANONCREDS_CREDENTIAL_TYPE: &str = "AnonCredsCredential";
pub const W3C_ANONCREDS_PRESENTATION_TYPE: &str = "AnonCredsPresentation";

pub(crate) static ANONCREDS_CONTEXTS: Lazy<Contexts> = Lazy::new(|| {
    Contexts(HashSet::from([
        URI::from(W3C_CONTEXT),
        URI::from(W3C_ANONCREDS_CONTEXT),
    ]))
});

pub(crate) static ANONCREDS_CREDENTIAL_TYPES: Lazy<Types> = Lazy::new(|| {
    Types(HashSet::from([
        String::from(W3C_CREDENTIAL_TYPE),
        String::from(W3C_ANONCREDS_CREDENTIAL_TYPE),
    ]))
});

pub(crate) static ANONCREDS_PRESENTATION_TYPES: Lazy<Types> = Lazy::new(|| {
    Types(HashSet::from([
        String::from(W3C_PRESENTATION_TYPE),
        String::from(W3C_ANONCREDS_PRESENTATION_TYPE),
    ]))
});
