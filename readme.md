## search-search

This is a small cmd tool to search twitch for streamers that are under the `game and development` tag.
it will use a `client_id` and a `client_secret` to fetch a twitch `client_credential` token and then use 
this token to perform a search of the streams currently online on twitch.

To use this you must set the `TWITCH_CLIENT_ID` and `TWITCH_CLIENT_SECRET` as env variables before searching.

You can as the first argument provide a search `String` to filter the results.

#### Example:
```
$ twitch-search rust

🔍 Searching twitch streams for: rust

en | https://twitch.tv/BrandontDev      | 26 viewers | 2022-01-02 21:06:33 UTC | Learning Rust - D&D Character Sheet TUI
en | https://twitch.tv/Xithrius         | 1 viewers | 2022-01-02 22:13:33 UTC | Python/Rust Open Source Programming.

Done (2/119) ❤
```

Results are printed as demonstrated in the above example.

### Blacklist
in the code there is a hard coded blacklist of streamer names that you can update if you want the excluded from the search results.

#### Possible feature upgrades
- change game id through cli arguments (hard coded now to `game development`)
- update blacklist through cli arguments
- add potential config file
- set client id and secret through cli or through external config file


