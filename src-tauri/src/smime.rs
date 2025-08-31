use std::process::Stdio;
use tokio::io::{copy, AsyncRead, AsyncWrite, AsyncWriteExt};
use tokio::process::Command;

pub struct SMIME {
    pub my_key: String,
    pub my_cert: String,
    pub her_cert: String,
}

pub async fn encrypt_and_sign<'a, 'b, R: AsyncRead + Unpin + 'a, W: AsyncWrite + Unpin + 'b>(
    input: R,
    output: W,
    smime: &SMIME,
) -> Result<(), Box<dyn std::error::Error>> {
    both(input, output, smime, false).await
}

pub async fn sign_and_encrypt<'a, 'b, R: AsyncRead + Unpin + 'a, W: AsyncWrite + Unpin + 'b>(
    input: R,
    output: W,
    smime: &SMIME,
) -> Result<(), Box<dyn std::error::Error>> {
    both(input, output, smime, true).await
}

async fn both<'a, 'b, R: AsyncRead + Unpin + 'a, W: AsyncWrite + Unpin + 'b>(
    input: R,
    output: W,
    smime: &SMIME,
    sign_first: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut sign = Command::new("openssl")
        .arg("smime")
        .arg("-stream")
        .arg("-sign")
        .arg("-inkey")
        .arg(smime.my_key.as_str())
        .arg("-signer")
        .arg(smime.my_cert.as_str())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut encrypt = Command::new("openssl")
        .arg("smime")
        .arg("-stream")
        .arg("-encrypt")
        .arg(smime.her_cert.as_str())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let sign_in = sign.stdin.take().expect("sign input");
    let sign_out = sign.stdout.take().expect("sign output");
    let encrypt_in = encrypt.stdin.take().expect("encrypt input");
    let encrypt_out = encrypt.stdout.take().expect("encrypt output");

    let (input, cp1, cp2) = if sign_first {
        (
            copy_pipe(input, sign_in),
            copy_pipe(sign_out, encrypt_in),
            copy_pipe(encrypt_out, output),
        )
    } else {
        (
            copy_pipe(input, encrypt_in),
            copy_pipe(encrypt_out, sign_in),
            copy_pipe(sign_out, output),
        )
    };

    let (n1, n2, n3) = tokio::try_join!(input, cp1, cp2)?;
    log::debug!("Copied {n1}b to sign, {n2}b to encrypt, {n3}b to output");

    let status = sign.wait().await?;
    if !status.success() {
        return Err(format!("Sign process failed with status: {}", status).into());
    }

    let status = encrypt.wait().await?;
    if !status.success() {
        return Err(format!("Encrypt process failed with status: {}", status).into());
    }

    Ok(())
}

async fn copy_pipe<R, W>(mut reader: R, mut writer: W) -> tokio::io::Result<u64>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let n = copy(&mut reader, &mut writer).await?;
    writer.shutdown().await?;
    Ok(n)
}
