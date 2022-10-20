//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

pub struct ImplementedConversion {
    pub code: u32,
    pub param_codes: Vec<u32>,
    pub param_type: String,
    pub conversion_type: String
}
impl ImplementedConversion {
    pub fn new(code: u32, param_codes: &[u32], param_type: &str, conversion_type: &str) -> Self {
        Self {
            code,
            param_codes: param_codes.into(),
            param_type: param_type.into(),
            conversion_type: conversion_type.into()
        }
    }
}