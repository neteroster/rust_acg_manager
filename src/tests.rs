use blake3::Hash;
use std::{path::Path, str::FromStr};
use crate::music_manager::scanner::{self, parse_directory, DirectoryType, AudioQuality};

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
    let test_2 = "[Set]来自风平浪静的明天";

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
            assert_eq!(quality, AudioQuality::HiRES);
            assert_eq!(title.as_str(), "「見上げてごらん、夜空の星を Interstellar Focus」原声带");
            assert_eq!(id, None);
        },
        _ => panic!("tests assertion failed. [parse_directory_test()]"),
    };
    match res_2 {
        DirectoryType::AlbumSet { title } => {
            assert_eq!(title.as_str(), "来自风平浪静的明天");
        },
        _ => panic!("tests assertion failed. [parse_directory_test()]"),
    }

}