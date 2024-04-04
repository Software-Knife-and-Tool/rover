//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

extern crate mu;

#[allow(unused_imports)]
use {
    mu::{Condition, Mu as Mu_, Result as Result_, Tag},
    std::{fs, io},
};

#[allow(dead_code)]
pub struct Mu {
    config: Option<String>,
    mu: Mu_,
}

#[allow(dead_code)]
impl Mu {
    pub fn new(config_: Option<String>) -> Self {
        let config = match config_ {
            None => Mu_::config(&String::new()).unwrap(),
            Some(ref config) => match Mu_::config(config) {
                None => {
                    eprintln!("mu: configuration error");
                    std::process::exit(-1)
                }
                Some(config) => config,
            },
        };

        Mu {
            config: config_,
            mu: Mu_::new(&config),
        }
    }

    pub fn eval(&self, expr: &str) -> io::Result<String> {
        match self.mu.eval_str(expr) {
            Err(err) => std::io::Result::Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                self.mu.exception_string(err),
            )),
            Ok(value) => Ok(self.mu.write_to_string(value, false).to_string()),
        }
    }
}
