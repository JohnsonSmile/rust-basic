use std::fs;
use std::path::Path;

use anyhow::{Result, bail};
use chrono::DateTime;
use git2::{
    Cred, FetchOptions, RemoteCallbacks, Repository, Sort,
    build::{self},
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

struct Args {
    /// git 的地址
    remote_url: String,
    /// 本地的 git 项目保存地址
    local_path: String,
    /// rsa地址
    private_key_path: String,
    /// 作者的名字
    author_names: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args {
        remote_url: "git@e.coding.net:fedtech/ares/erp_api.git".to_string(),
        local_path: "repos".to_string(),
        private_key_path: "ssh/id_rsa".to_string(),
        author_names: vec![
            "mawanli".to_string(),
            "马万里".to_string(),
            "JohnsonSmile".to_string(),
        ],
    };

    let remote_url = &args.remote_url;
    let Some(project_name) = remote_url
        .split('/')
        .last()
        .and_then(|s| Some(s.replace(".git", "")))
    else {
        bail!("解析失败")
    };
    let local_path = Path::new(&args.local_path).join(project_name); // "repos/erp_api";
    
    // credential
    let mut cb = RemoteCallbacks::new();
    // 设置 SSH 凭据回调
   cb.credentials(|_url, username_from_url, _allowed_types| {
        // SSH 私钥路径（通常在 ~/.ssh/id_rsa）
        let private_key = Path::new(&args.private_key_path);
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

    // fetch option
    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);
    
    if !fs::exists(&local_path)? {
        // 不存在就clone
        // Prepare builder.
        let mut builder = build::RepoBuilder::new();
        builder.fetch_options(fo);
        builder.clone(remote_url, &local_path)?;
    }

    // credential
    let mut cb = RemoteCallbacks::new();
    // 设置 SSH 凭据回调
    cb.credentials(|_url, username_from_url, _allowed_types| {
        // SSH 私钥路径（通常在 ~/.ssh/id_rsa）
        let private_key = Path::new(&args.private_key_path);
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

    // fetch option
    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);
    
    let repo = Repository::open(local_path)?;
    let mut remote = repo
        .find_remote(remote_url)
        .or_else(|_| repo.remote_anonymous(remote_url))?;
    

    let references = repo.references()?;
    for reference in references {
        let reference = reference?;
        let Some(reference_name) = reference.name() else {
            continue;
        };
        println!("reference_name: {:?}", reference_name);
        repo.set_head(reference_name)?;
        remote.fetch(&[reference_name], Some(&mut fo), None)?;

        let mut revwalk = repo.revwalk()?;
        revwalk.set_sorting(Sort::TIME)?;
        revwalk.push_head()?;
        for commit in revwalk {
            let oid = commit?;
            let commit = repo.find_commit(oid)?;
            let author_name = commit.author().name().unwrap_or("unknown").to_string();
            if args.author_names.contains(&author_name) {
                println!("author_name: {:?}", author_name);
                println!("commit: {:?}", commit.id());
                println!(
                    "commit time: {:?}",
                    DateTime::from_timestamp(commit.time().seconds(), 0)
                        .unwrap()
                        .to_utc()
                );
                println!(
                    "commit message: {:?}",
                    commit.message().unwrap_or("unknown")
                );
            }
        }
    }
    
    Ok(())
}
