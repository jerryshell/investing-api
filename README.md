# Investing API

Unofficial [investing.com](https://www.investing.com/) API. Written in Rust.

🏗 **This project is still in the early stages of development.**

## Usage

```
> ./investing-api --help
investing-api 0.2.0
github.com/jerryshell/investing-api

USAGE:
    investing-api [OPTIONS] --name <NAME> --start-date <START_DATE> --end-date <END_DATE>

OPTIONS:
    -c, --sort-col <SORT_COL>        [default: date]
    -e, --end-date <END_DATE>        %m/%d/%Y
    -h, --help                       Print help information
    -i, --interval <INTERVAL>        [default: Daily]
    -n, --name <NAME>                
    -o, --sort-ord <SORT_ORD>        [default: DESC]
    -s, --start-date <START_DATE>    %m/%d/%Y
    -V, --version                    Print version information
```

### Example

Fetch [csi1000](https://www.investing.com/indices/csi1000) historical data from `2022/01/01` to `2022/01/07`

```
> ./investing-api --name csi1000 --start-date 01/01/2022 --end-date 01/07/2022
[
    DataItem {
        timestamp_sec: 1641513600,
        price: 7684.46,
        open: 7841.13,
        high: 7865.75,
        low: 7681.13,
        vol: 212063,
    },
    DataItem {
        timestamp_sec: 1641427200,
        price: 7844.65,
        open: 7743.86,
        high: 7863.55,
        low: 7721.78,
        vol: 190535,
    },
    DataItem {
        timestamp_sec: 1641340800,
        price: 7781.45,
        open: 7968.74,
        high: 7968.92,
        low: 7733.51,
        vol: 216695,
    },
    DataItem {
        timestamp_sec: 1641254400,
        price: 7982.35,
        open: 8045.1,
        high: 8060.99,
        low: 7948.91,
        vol: 223567,
    },
]
write to csv...

> ls *.csv
1171911.csv
```

## License

[GNU Affero General Public License v3.0](https://choosealicense.com/licenses/agpl-3.0/)
