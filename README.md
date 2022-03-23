# RatioUp

Tool to fake upload on your torrents. It can be useful on private or semi-private bittorrent trackers especially if you have a slow internet connection. Furthermore,
there are often many seeders so it becomes hard to seed and increase your ratio.

It is a tool like [JOAL](https://github.com/anthonyraymond/joal) or [Ratio Master](http://ratiomaster.net/).

I'm making this tool in order to practice Rust programming, having something lighter than Joal (written in Java) and that runs on any OS (I want to install it on my ARM NAS with only 2GB RAM).

## Disclamer

RatioUp is not designed to help or encourage you downloading illegal materials ! You must respect the law applicable in your country. I couldn't be held responsible for illegal activities performed by your usage of RatioUp.

I am not responsible if you get banned using this tool. However, you can reduce risk by using popular torrents (with many seeders and leechers).

For now, I'm not planning add a security layer because I'll use it on my home lan network. If you want to secure it, use a reverse proxy with **nginx** (and any other web server you ar familiar with) and add a SSL layer and a basic authentication.

## Deployment

```shell
docker run -d --name RatioUp --restart unless-stopped -v PATH:/data slundi/ratioup
```

Change the **PATH** in order to keep your configuration.

## Command line interface (CLI)

```shell
RatioUp -d ~/torrents -c ~/RatioUp.json -p 8070
```

Arguments are:

| Argument        | Default value | Description                                                              |
|-----------------|---------------|--------------------------------------------------------------------------|
| `c` or `config` | `config.json` | Path to the JSON configuration file                                      |
| `d` or `dir`    | `./torrents`  | Path to the directory where torrents are saved (without trailing slash)  |
| `p` or `port`   | `7070`        | Web server port                                                          |
| `root`          | `/`           | Web root (ie: <http://127.0.0.1:8070/ROOT/>)                             |

## Configuration

Here is an example of the config.json:

```json
{
    "client":"qbittorrent-4.3.9",
    "min_upload_rate":8192,
    "max_upload_rate":104857600,
    "min_download_rate": 0,
    "max_download_rate": 0,
    "numwant": 100,
    "numwant_on_stop": 0,
    "seed_if_zero_leecher": false,
    "seed_public_torrent": false
}
```

Download and upload rates are in bytes (ie: 16MB = 16 x 1024 x 1024 = 16777216 bytes).
To disable downloads, set `min_download_rate` and `max_download_rate` to 0.

## Todo

- [ ] Announce at the correct interval or after 30 minutes if we are unable to parse the tracker response (sometimes there is an utf8 string, sometimes it is binary data)
- [ ] Fix response parsing: MalformedBencode("Input contains invalid UTF-8."), MalformedResponse("peers" does not map to a dict or a string of bytes.")
- [ ] Docker image
- [ ] Decrease torrent speeds from global download and upload speeds

## Roadmap

- [ ] Torrents with multiple trackers?
- [ ] Improve UI
- [ ] Retracker torrents
- [ ] Further testings (I use *rtorrent* and *qBittorrent*, other clients may not work properly)
