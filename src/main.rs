use m3u8_downloader_rs::utils;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Arg{
    #[structopt(short, long)]
    url: String
}

#[tokio::main]
async fn main() {
    let opt = Arg::from_args();
    let url = opt.url.as_str();
    let url_list = utils::request::get_m3u8_list(url).await.unwrap();
    let url_list = utils::parse::parse_m3u8_list(url_list);
    utils::request::get_all_ts(&url_list, url).await;
    utils::merge::merge_ts(url_list.len());
}
