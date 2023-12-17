use crate::server::provider::{Provider, Software};

pub const PAPERMC_ARRAY: [Software; 4] = [
    Software {
        id: "paper", provider: Provider::PaperMC
    }, Software {
        id: "folia", provider: Provider::PaperMC
    }, Software {
        id: "velocity", provider: Provider::PaperMC
    }, Software {
        id: "waterfall", provider: Provider::PaperMC
    }
];