// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use std::{
    ffi::OsStr,
    process::{ExitStatus, Stdio},
};

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, Command},
};

use crate::framework::Framework;

pub static mut CLIENT_READY: bool = false;

pub fn read_lines(child: &mut Child, framework: Framework) {
    let stdout = child
        .stdout
        .take()
        .expect("Child did not have a handle to stdout!");

    let stderr = child
        .stderr
        .take()
        .expect("Child did not have a handle to stderr!");

    let mut stdout = BufReader::new(stdout).lines();
    let mut stderr = BufReader::new(stderr).lines();

    tokio::spawn(async move {
        while let Ok(Some(line)) = stdout.next_line().await {
            unsafe {
                if CLIENT_READY && !line.trim().is_empty() {
                    framework.process_message(&line);
                }
            }

            if line.trim().contains(framework.get_ready_str()) {
                info!("Started client!");

                unsafe {
                    CLIENT_READY = true;
                }
            }
        }
    });

    tokio::spawn(async move {
        while let Ok(Some(line)) = stderr.next_line().await {
            if !line.trim().is_empty() {
                framework.process_message(&line);
            }
        }
    });
}

pub async fn start_client<T>(dir: T, mut cmd: Vec<T>, framework: Framework) -> ExitStatus
where
    T: AsRef<OsStr>,
{
    cmd.reverse();

    let exec = cmd.pop().unwrap();

    cmd.reverse();

    let mut cmd = Command::new(exec.as_ref())
        .args(cmd)
        .current_dir(dir.as_ref())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    read_lines(&mut cmd, framework);

    cmd.wait().await.unwrap()
}
