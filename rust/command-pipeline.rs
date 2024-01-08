


let gh = process::Command::new("gh")
    .args([
        "api",
        "-H",
        "Accept: application/vnd.github+json",
        "-H",
        "X-GitHub-Api-Version: 2022-11-28",
        "--cache",
        "24h",
        "--jq",
        ".body",
        "repos/YanniHu1996/rust-ops/issues/1",
    ])
    .stdout(Stdio::piped())
    .spawn()?;

let base64 = process::Command::new("base64")
    .args(["-d"])
    .stdin(Stdio::from(gh.stdout.unwrap()))
    .stdout(Stdio::piped())
    .spawn()?;

let output = base64.wait_with_output()?;
