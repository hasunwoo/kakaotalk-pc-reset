#![windows_subsystem = "windows"]

use std::{
    error::Error,
    fs, io,
    path::PathBuf,
    process::{Command, Output},
};

use directories::{BaseDirs, UserDirs};

fn main() -> Result<(), Box<dyn Error>> {
    if let Some(kakao_download_dir) = get_kakaotalk_download_dir() {
        if fs::remove_dir_all(&kakao_download_dir).is_ok() {
            println!("카카오톡 다운로드 폴더를 성공적으로 삭제했습니다.")
        }
    }
    let kakao_dir = get_kakaotalk_user_dir().ok_or("카카오톡 데이터 경로를 찾을 수 없습니다.")?;
    if !kakao_dir.exists() {
        println!("삭제할 카카오톡 데이터가 없습니다.");
        return Ok(());
    }
    terminate_kakaotalk().map_err(|e| format!("카카오톡을 종료할 수 없습니다: {}", e))?;
    fs::remove_dir_all(&kakao_dir)
        .map_err(|e| format!("카카오톡 데이터를 삭제할 수 없습니다: {}", e))?;
    println!("카카오톡 데이터를 성공적으로 삭제했습니다.");
    Ok(())
}

fn terminate_kakaotalk() -> io::Result<Output> {
    Command::new("taskkill")
        .args(["/IM", "KakaoTalk.exe"])
        .output()
}

fn get_kakaotalk_download_dir() -> Option<PathBuf> {
    let user_dir = UserDirs::new()?;
    let documents_dir = user_dir.document_dir()?;
    let kakaodalk_download_dir = documents_dir.join("카카오톡 받은 파일");
    Some(kakaodalk_download_dir)
}

fn get_kakaotalk_user_dir() -> Option<PathBuf> {
    let base_dir = BaseDirs::new()?;
    let local_dir = base_dir.data_local_dir();
    let kakao_user_dir = local_dir.join("kakao").join("KakaoTalk").join("users");
    Some(kakao_user_dir)
}
