use blake3::Hash;
use std::{path::Path, str::FromStr};
use crate::{music_manager::{scanner::{self, parse_directory, DirectoryType, AudioQuality, scan, Album, Music}, interact::generate_line_album}, config::parser::AllConfig};

#[tokio::test]
async fn blake3_dir_digest_test() {
    let res = scanner::blake3_dir_digest(Path::new("D:/hash_test"))
    .await
    .unwrap();

    assert_eq!(res, Hash::from_str("cb51de251349b4b132f3328775ee30144f2fbc4d096031797ea24c821173ccdb").unwrap());
}

#[tokio::test]
async fn parse_directory_test() {
    let test_0 = "[KSLA-0162][CD-Res]Summer Pockets Orchestara Album 『Echoes of Summer』";
    let test_1 = "[N][Hi-Res]「見上げてごらん、夜空の星を Interstellar Focus」原声带";
    let test_2 = "[Set]Angel Beats!";

    let res_0 = parse_directory(test_0).await.unwrap();
    let res_1 = parse_directory(test_1).await.unwrap();
    let res_2 = parse_directory(test_2).await.unwrap();
    match res_0 {
        DirectoryType::Album { quality, title, id } => {
            assert_eq!(quality, AudioQuality::CdRes);
            assert_eq!(title.as_str(), "Summer Pockets Orchestara Album 『Echoes of Summer』");
            assert_eq!(id.unwrap().as_str(), "KSLA-0162");
        },
        _ => panic!("tests assertion failed. [parse_directory_test()]"),
    };
    match res_1 {
        DirectoryType::Album { quality, title, id } => {
            assert_eq!(quality, AudioQuality::HiRes);
            assert_eq!(title.as_str(), "「見上げてごらん、夜空の星を Interstellar Focus」原声带");
            assert_eq!(id, None);
        },
        _ => panic!("tests assertion failed. [parse_directory_test()]"),
    };
    match res_2 {
        DirectoryType::AlbumSet { title } => {
            assert_eq!(title.as_str(), "Angel Beats!");
        },
        _ => panic!("tests assertion failed. [parse_directory_test()]"),
    }

}

#[tokio::test]
async fn scan_test() {
    let res = scan(Path::new("D:/cd_test")).await.unwrap();
    let ab = res.single_album;
    assert_eq!(ab[0].quality, AudioQuality::CdRes);
    assert_eq!(ab[0].id.as_ref().unwrap().as_str(), "0");
    assert_eq!(ab[0].title.as_str(), "TitleA");

    assert_eq!(ab[1].quality, AudioQuality::HiRes);
    assert_eq!(ab[1].id, None);
    assert_eq!(ab[1].title.as_str(), "TitleB");

    let abls = res.album_set;
    assert_eq!(abls[0].title.as_str(), "TitleC");
    assert_eq!(abls[0].albums[1].quality, AudioQuality::NormalRes);
}

#[test]
fn generate_line_album_test() {
    let ab = Album {
        quality: AudioQuality::CdRes,
        title: String::from("TestT"),
        id: Some(String::from("A001")),
        checksum: scanner::CheckSum::Blake3(Hash::from_str("cb51de251349b4b132f3328775ee30144f2fbc4d096031797ea24c821173ccdb").unwrap())
    };
    let res = generate_line_album(&ab, 1);
    assert_eq!(res.as_str(), "    - [A001][CD-Res]TestT[checksum | BLAKE3 | cb51de251349b4b132f3328775ee30144f2fbc4d096031797ea24c821173ccdb]");
    let ab = Album {
        quality: AudioQuality::CdRes,
        title: String::from("TestT"),
        id: None,
        checksum: scanner::CheckSum::Blake3(Hash::from_str("cb51de251349b4b132f3328775ee30144f2fbc4d096031797ea24c821173ccdb").unwrap())
    };
    let res = generate_line_album(&ab, 1);
    assert_eq!(res.as_str(), "    - [N][CD-Res]TestT[checksum | BLAKE3 | cb51de251349b4b132f3328775ee30144f2fbc4d096031797ea24c821173ccdb]");
}

#[test]
fn config_read_from_file_test() {
    let p = Path::new("D:/cfs.json");
    let res = AllConfig::from_file(p).unwrap();
    assert_eq!(res.username, "neteroster");
    assert_eq!(res.music_config.enable, true);
}

#[tokio::test]
async fn music_serialize_test() {
    let music = scan(Path::new("D:/cd_test")).await.unwrap();
    let music_ser = music.serialize_to_json().unwrap();
    let music_des = Music::from_json(music_ser.as_str()).unwrap();
    let ab = music_des.single_album;
    assert_eq!(ab[0].quality, AudioQuality::CdRes);
    assert_eq!(ab[0].id.as_ref().unwrap().as_str(), "0");
    assert_eq!(ab[0].title.as_str(), "TitleA");

    assert_eq!(ab[1].quality, AudioQuality::HiRes);
    assert_eq!(ab[1].id, None);
    assert_eq!(ab[1].title.as_str(), "TitleB");

    let abls = music_des.album_set;
    assert_eq!(abls[0].title.as_str(), "TitleC");
    assert_eq!(abls[0].albums[1].quality, AudioQuality::NormalRes);
}