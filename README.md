# htcount

![Build](https://github.com/YGGverse/htcount/actions/workflows/build.yml/badge.svg)
[![Dependencies](https://deps.rs/repo/github/YGGverse/htcount/status.svg)](https://deps.rs/repo/github/YGGverse/htcount)
[![crates.io](https://img.shields.io/crates/v/htcount.svg)](https://crates.io/crates/htcount)

Simple CLI/daemon tool for counting visitors using `access.log` and exporting totals in multiple formats, such as JSON or SVG badge

## Features

### Log format support

* [x] Nginx
* [ ] Apache

### Export formats

* [x] JSON - for API usage
* [x] SVG - configurable badge button

## Install

1. `git clone https://github.com/YGGverse/htcount.git && cd htcount`
2. `cargo build --release`
3. `sudo install target/release/htcount /usr/local/bin/htcount`

## Usage

``` bash
htcount --source      /var/log/nginx/access.log\
        --export-json /path/to/totals.json\
        --export-svg  /path/to/totals.svg
```

### Options

``` bash
-d, --debug <DEBUG>
        Debug level

        * `i` - info * `d` - detailed

        [default: i]

-f, --format <FORMAT>
        Log format for given `source`

        * `nginx`

        [default: nginx]

    --export-json <EXPORT_JSON>
        Export results to JSON file (e.g. `/path/to/stats.json`)

    --export-svg <EXPORT_SVG>
        Export results to SVG file (e.g. `/path/to/badge.svg`)

        * use `{hits}` / `{hosts}` pattern to replace parsed values

    --template-svg <TEMPLATE_SVG>
        Use custom SVG file template with `{hits}` / `{hosts}` placeholders

        [default: default/counter.svg]

-c, --capacity <CAPACITY>
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