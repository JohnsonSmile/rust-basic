use std::{env, error::Error, fs, path::Path, process};

use anyhow::{Ok, Result};
use git2::{
    BranchType, CertificateCheckStatus, Cred, Direction, FetchOptions, RemoteCallbacks, Repository,
    build,
};

// cargo run -- the poem.txt
// fn main() {
//     let args = env::args();
//     let config = Config::build(args).unwrap_or_else(|err| {
//         println!("err: {err}");
//         process::exit(-1)
//     });
//     if let Err(err) = run(config) {
//         println!("err: {err}");
//         process::exit(-1)
//     }
// }

fn main() -> Result<()> {
    let remote_url = "git@e.coding.net:fedtech/ares/erp_api.git";
    let local_path = "repos/erp_api";

    let repo = Repository::open(local_path)?;
    let mut remote = repo
        .find_remote(remote_url)
        .or_else(|_| repo.remote_anonymous(remote_url))?;

    // 创建克隆配置
    let mut cb = RemoteCallbacks::new();
    // 设置 SSH 凭据回调
    cb.credentials(|url, username_from_url, allowed_types| {
        // SSH 私钥路径（通常在 ~/.ssh/id_rsa）
        let private_key = Path::new("ssh/id_rsa");
        // 公钥路径（通常在 ~/.ssh/id_rsa.pub）
        let public_key = private_key.with_extension("pub");
        // 若密钥有密码，提供密码；否则设为 None
        let passphrase = None;

        // 创建 SSH 密钥凭据
        Cred::ssh_key(
            username_from_url.unwrap_or("git"),
            Some(&public_key),
            &private_key,
            passphrase,
        )
    });
    // Connect to the remote and call the printing function for each of the
    // remote references.
    // let mut connection = remote.connect_auth(Direction::Fetch, Some(cb), None)?;

    for branch in repo.branches(Some(BranchType::Remote))? {
        let (branch_name, _) = branch?;
        println!("  * {:?}", branch_name.name());
    }

    // TODO:xxx
    return Ok(());

    // 创建克隆配置
    let mut cb = RemoteCallbacks::new();
    // 设置 SSH 凭据回调
    cb.credentials(|url, username_from_url, allowed_types| {
        // SSH 私钥路径（通常在 ~/.ssh/id_rsa）
        let private_key = Path::new("ssh/id_rsa");
        // 公钥路径（通常在 ~/.ssh/id_rsa.pub）
        let public_key = private_key.with_extension("pub");
        // 若密钥有密码，提供密码；否则设为 None
        let passphrase = None;

        // 创建 SSH 密钥凭据
        Cred::ssh_key(
            username_from_url.unwrap_or("git"),
            Some(&public_key),
            &private_key,
            passphrase,
        )
    });

    // Prepare fetch options.
    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);
    // Prepare builder.
    let mut builder = build::RepoBuilder::new();
    builder.fetch_options(fo);
    let path = Path::new(local_path);
    builder.clone(remote_url, path)?;
    Ok(())
}
