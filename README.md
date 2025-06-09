# htcount

![Build](https://github.com/YGGverse/htcount/actions/workflows/build.yml/badge.svg)
[![Dependencies](https://deps.rs/repo/github/YGGverse/htcount/status.svg)](https://deps.rs/repo/github/YGGverse/htcount)
[![crates.io](https://img.shields.io/crates/v/htcount.svg)](https://crates.io/crates/htcount)

Simple CLI/daemon tool for counting visitors using `access.log` in the [Common Log Format](https://en.wikipedia.org/wiki/Common_Log_Format)\
Export totals in multiple formats, such as JSON or SVG badge!

## Features

### Export formats

* [x] JSON - for API usage
* [x] SVG - configurable or [default](https://github.com/YGGverse/htcount/tree/main/default) badge button

## Install

1. `git clone https://github.com/YGGverse/htcount.git && cd htcount`
2. `cargo build --release`
3. `sudo install target/release/htcount /usr/local/bin/htcount`

## Usage

``` bash
htcount --source       /var/log/nginx/access.log\
        --target-svg   /path/to/badge.svg\
        --template-svg /path/to/counter/template.svg
```
* see `default/counter.svg`

### Options

``` bash
-d, --debug <DEBUG>
        Debug level

        * `i` - info * `d` - detailed

        [default: i]

--target-json <TARGET_JSON>
        Export results to JSON file (e.g. `/path/to/stats.json`)

--target-svg <TARGET_SVG>
        Export results to SVG file (e.g. `/path/to/badge.svg`)

        * use `{hits}` / `{hosts}` pattern to replace parsed values

--template-svg <TEMPLATE_SVG>
        Use custom SVG file template with `{hits}` / `{hosts}` placeholders

        [default: default/counter.svg]

-m, --match-time <MATCH_TIME>
        Filter records match time pattern (e.g. `%d/%b/%Y`)

-c, --index-capacity <CAPACITY>
        Expected memory index capacity

        [default: 100]

-i, --ignore-host <IGNORE_HOST>
        Exclude host(s) from index

-s, --source <SOURCE>
        Access log source (e.g. `/var/nginx/access.log`)

-u, --update <UPDATE>
        Update delay in seconds

        [default: 300]

-h, --help
        Print help (see a summary with '-h')

-V, --version
        Print version
```

### systemd

``` /etc/systemd/system/htcount.service
#/etc/systemd/system/htcount.service

[Unit]
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
ExecStart=/usr/local/bin/htcount --source /var/log/nginx/access.log\
                                 --target-svg /var/www/htcount/visitors.svg\
                                 --template-svg /path/to/default/template.svg\
                                 --ignore-host 127.0.0.1\
                                 --ignore-host 127.0.0.2\
                                 --match-time %%d/%%b/%%Y\
                                 --update 3600\
                                 --debug n
StandardOutput=null
StandardError=null

[Install]
WantedBy=multi-user.target
```
* make sure `/var/www/htcount` directory exists
* replace `/path/to/default/template.svg` with your custom template path
* use `ignore-host` to skip local host requests
* to filter today-only records, use `match-time` argument `%d/%b/%Y`
    * to filter this month-only records use `%b/%Y`
    * make sure the time pattern in the example above corresponds to your `access.log` format
    * make sure the log time pattern `%` is escaped to `%%`

### Service management tips

* `systemctl daemon-reload` - update configuration
* `systemctl enable` - launch on system startup
* `systemctl restart htcount` - start systemd service
* `systemctl status htcount` - check if service is running

## Live examples

* `http://[302:68d0:f0d5:b88d::fed]` - [Snac](https://codeberg.org/grunfink/snac2) instance for the [Yggdrasil](https://yggdrasil-network.github.io/) network