# Gitea Timings

A command line tool that fetches time tracking data from the Gitea API and
pretty-prints a tabular summary.

## Example

```
You spent 7 hours and 59 minutes on 9 issues in the last 24 hours:
+----------+---------------------------------------------------------------+
| 02:30:02 | Example big feature                                           |
| 02:13:17 | Gitea Timings                                                 |
| 00:38:07 | A normal issue except this one has a relatively long name     |
| 00:33:58 |   ____ _ _               _____ _           _                  |
| 00:30:28 |  / ___(_) |_ ___  __ _  |_   _(_)_ __ ___ (_)_ __   __ _ ___  |
| 00:30:08 | | |  _| | __/ _ \/ _` |   | | | | '_ ` _ \| | '_ \ / _` / __| |
| 00:23:22 | | |_| | | ||  __/ (_| |   | | | | | | | | | | | | | (_| \__ \ |
| 00:21:04 |  \____|_|\__\___|\__,_|   |_| |_|_| |_| |_|_|_| |_|\__, |___/ |
| 00:18:40 |                                                    |___/      |
+----------+---------------------------------------------------------------+
```

## License

This project is licensed under the [MIT license](LICENSE-MIT).
