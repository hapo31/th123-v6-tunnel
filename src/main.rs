mod actor;
use actix::prelude::*;
use actor::{InitialTh123PortEvent, TH12TunnelActor};

// TODO:
// サーバー/クライアントモード選択(自動で出来ると嬉しいかもね？)
// ポート番号設定

// 実装方針
// 1. 受信用スレッドを2本立てる
// 2. actix のアクターを使いデータ受信後にイベントを送信する
// 3. Handler で UDPソケットを使ってデータを転送

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("preparing...");

    let actor = TH12TunnelActor::create(10800u16);
    let addr = actor.start();
    let res = addr.send(InitialTh123PortEvent(11451u16)).await;
    println!("{}", res.unwrap());
    Ok(())
}
