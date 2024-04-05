//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

use std::{
    io::prelude::*,
    process::{Child, Command, Stdio},
};

#[allow(dead_code)]
pub struct Mu {
    process: Child,
}

impl Mu {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut process = match Command::new("mu-sys")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(_) => panic!(),
            Ok(process) => process,
        };

        if let Err(why) = process
            .stdin
            .as_ref()
            .unwrap()
            .write_all("lib:version".as_bytes())
        {
            panic!("mu-sys: couldn't write to stdin: {}", why)
        }

        let mut s = String::new();
        match process.stdout.as_mut().unwrap().read_to_string(&mut s) {
            Err(why) => panic!("mu:sys: couldn't read stdout: {}", why),
            Ok(_) => Mu { process },
        }
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
