pub fn parse_m3u8_list(list_string: String) -> Vec<String>{
    let lines = list_string.split('\n');
    let mut url_list = Vec::<String>::new();
    let mut flag = false;
    for line in lines{
        if line.contains("#EXTINF"){
            flag = true;
        }else if flag{
            flag = false;
            url_list.push(line.to_string());
        }
    }
    url_list
}