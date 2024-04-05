//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

use std::{
    io::prelude::*,
    process::{Child, Command, Stdio},
};
use {futures::executor::block_on, futures_locks::RwLock};

#[allow(dead_code)]
pub struct Mu {
    process: RwLock<Child>,
}

impl Mu {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let process = match Command::new("mu-sys")
            .arg("-0")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(_) => panic!(),
            Ok(process) => process,
        };

        Mu {
            process: RwLock::new(process),
        }
    }

    #[allow(dead_code)]
    pub fn eval(&self, form: String) -> std::result::Result<String, String> {
        let mut process = block_on(self.process.write());

        let nl_form = format!("{}\n", form);

        if let Err(why) = process
            .stdin
            .as_ref()
            .unwrap()
            .write_all(nl_form.as_bytes())
        {
            return Err(format!("eval: couldn't write to stdin: {}", why));
        }

        let mut s = String::new();
        let mut buf = [0; 1];

        loop {
            let ch = match process.stdout.as_mut().unwrap().read(&mut buf) {
                Err(why) => return Err(format!("eval: couldn't read stdout: {}", why)),
                Ok(_) => buf[0] as char,
            };

            if ch == '\0' {
                break;
            } else {
                s.push(ch)
            }
        }

        Ok(s)
    }

    pub fn version() -> std::result::Result<String, String> {
        let process = match Command::new("mu-sys")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(why) => return Err(format!("mu-sys: couldn't spawn: {}", why)),
            Ok(process) => process,
        };

        if let Err(why) = process.stdin.unwrap().write_all("lib:version".as_bytes()) {
            return Err(format!("mu-sys: couldn't write to stdin: {}", why));
        }

        let mut s = String::new();
        match process.stdout.unwrap().read_to_string(&mut s) {
            Err(why) => Err(format!("mu:sys: couldn't read stdout: {}", why)),
            Ok(_) => Ok(s),
        }
    }
}
