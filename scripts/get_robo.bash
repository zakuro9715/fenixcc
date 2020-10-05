tag_regex='"tag_name": "(.+)",'
api_url=https://api.github.com/repos/tj/robo/releases

get_tag() {
  curl -s $api_url | grep tag_name | head -n 1 \
    | sed -E "s/${tag_regex}/\1/g" | tr -d ' '
}

download_url_base=https://github.com/tj/robo/releases/download/
tag="$(get_tag)"
curl -sSL "$download_url_base/$tag/robo_linux_amd64" -o robo
chmod +x robo
