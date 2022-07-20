mod episode;
mod frame;
mod util;

use std::collections::HashMap;
use std::fs;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use warp::Filter;

use anyhow::{ensure, Context, Result};
use rand::seq::SliceRandom;
use crate::episode::EpisodeNumber;

const API_ADDRESS: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

#[tokio::main]
async fn main() -> Result<()> {
    let episodes_order = EpisodeNumber::generate_episodes_order();

    let index_page = Arc::new(read_index_page()?);
    let game_page = Arc::new(read_game_page()?);

    let index = warp::get().and(warp::path("index.html")).map(move || {
        let index_page = Arc::clone(&index_page);
        warp::reply::html(index_page.to_string())
    });

    let game = warp::get().and(warp::path("game.html")).map(move || {
        let game_page = Arc::clone(&game_page);
        warp::reply::html(game_page.to_string())
    });

    let status = warp::get().and(warp::path("status")).map(|| "ok");

    let frame = warp::get()
        .and(warp::path!("frame" / usize))
        .map(|frame: usize| "toudou");

    let routes = index.or(game).or(status).or(frame);

    let server = warp::serve(routes);

    server
        .run(SocketAddr::new(IpAddr::V4(API_ADDRESS), 8080))
        .await;

    Ok(())
}

fn read_index_page() -> Result<String> {
    fs::read_to_string("static/index.html").context("Error reading the index.html file")
}

fn read_game_page() -> Result<String> {
    fs::read_to_string("static/game.html").context("Error reading the game.html file")
}
