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
    fs,
    io::{Cursor, Read},
    path::PathBuf,
};

use anyhow::Result;
use zip::ZipArchive;

use crate::{
    download::download_file, jar_mf::parse_manifest, maven::coord::MavenCoordinate,
    piston::download::download_minecraft_jar,
};

use super::install::{InstallProfile, Processor};

pub fn resolve_jar(libs: &PathBuf, jar: String) -> String {
    let it = MavenCoordinate::from(jar);

    libs.join(it.path()).to_str().unwrap().to_string()
}

// tmp_dir should be where the installer's files are extracted to
pub async fn build_processor_command(
    profile: &InstallProfile,
    java: &PathBuf,
    lib_dir: &PathBuf,
    tmp_dir: &PathBuf,
    proc: Processor,
    maven: impl AsRef<str>,
) -> Result<Vec<String>> {
    let mut args = Vec::new();

    for arg in proc.args {
        if arg.starts_with('[') && arg.ends_with(']') {
            let path = arg.trim_start_matches('[').trim_end_matches(']').into();

            args.push(resolve_jar(&lib_dir, path));
        } else if arg.starts_with('{') && arg.ends_with('}') {
            let arg_trimmed = arg.trim_start_matches('{').trim_end_matches('}');

            if arg_trimmed == "SIDE" {
                args.push("client".into());
            } else if arg_trimmed == "MINECRAFT_JAR" {
                let jar_path = resolve_jar(
                    &lib_dir,
                    format!("net.minecraft:client:{}", profile.minecraft),
                );

                download_minecraft_jar(PathBuf::from(&jar_path), &profile.minecraft).await?;
                args.push(jar_path);
            } else if let Some(data) = profile.data.get(arg_trimmed) {
                let val = data.client.clone();

                if val.starts_with('[') && val.ends_with(']') {
                    let path = val.trim_start_matches('[').trim_end_matches(']').into();

                    args.push(resolve_jar(&lib_dir, path));
                } else if val.starts_with('/') {
                    args.push(
                        tmp_dir
                            .join(val.trim_start_matches('/'))
                            .to_str()
                            .unwrap()
                            .to_string(),
                    );
                } else {
                    args.push(val);
                }
            } else {
                args.push(arg);
            }
        } else {
            args.push(arg);
        }
    }

    let jarc = MavenCoordinate::from(&proc.jar);
    let jar = resolve_jar(&lib_dir, proc.jar);

    download_file(
        &lib_dir,
        jarc.url(maven.as_ref()),
        jarc.path(),
        Option::<String>::None,
        &None,
    )
    .await?;

    let mut cp = vec![jar.clone()];

    cp.extend(
        proc.classpath
            .iter()
            .map(|v| resolve_jar(&lib_dir, v.to_owned())),
    );

    let bytes = fs::read(jar)?;
    let cursor = Cursor::new(bytes);
    let mut zip = ZipArchive::new(cursor)?;
    let mut file = zip.by_name("META-INF/MANIFEST.MF")?;
    let mut buf = Vec::new();

    file.read_to_end(&mut buf)?;

    let data = String::from_utf8(buf)?;
    let manifest = parse_manifest(data);
    let main = manifest.get("Main-Class").unwrap();
    let mut cmd = Vec::new();

    cmd.push(java.to_str().unwrap().to_string());
    cmd.push("-cp".into());
    cmd.push(cp.join(":"));
    cmd.push(main.to_owned());
    cmd.extend(args);

    Ok(cmd)
}

pub fn should_run_processor(sides: Option<Vec<String>>) -> bool {
    if let Some(sides) = sides {
        sides.contains(&"client".into())
    } else {
        true
    }
}
