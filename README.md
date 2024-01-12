
# cytt

https://cytt.duckdns.org

Inspired by [Obito1903/CY-celcat](https://github.com/Obito1903/CY-celcat)

## Config

### Required
| Environment Variable           | Note 
| :----------------------------- | :-
| `CYTT_CELCAT_USERNAME`         | 
| `CYTT_CELCAT_PASSWORD`         | 
| `CYTT_GROUP_n_NAME`            | n in range (0-255) inclusive <br/> must match `[A-Za-z0-9-_]`
| `CYTT_GROUP_n_STUDENTID`       | n in range (0-255) inclusive

### Optional
| Environment Variable           | Default Value  | Note 
| :----------------------------- | :------------- | :-
| `CYTT_GROUP_n_DISPLAY_NAME`    |                | n in range (0-255) inclusive
| `CYTT_GROUP_n_GCALID`          |                | ^
| `CYTT_GROUP_n_GCALID_CM`       |                | ^
| `CYTT_GROUP_n_GCALID_TD`       |                | ^
| `CYTT_GROUP_n_GCALID_EXAMEN`   |                | ^
| `CYTT_GROUP_n_GCALID_AUTRE`    |                | ^
| `CYTT_HOST`                    | `127.0.0.1`    | 
| `CYTT_PORT`                    | `8000`         | 
| `CYTT_DATA_PATH`               | `./data`       | must have rw perms
| `CYTT_PUBLIC_PATH`             | `./public`     | ^
| `CYTT_CALENDAR_FETCH_INTERVAL` | `1800` (30min) | in seconds
| `CYTT_CALENDAR_FETCH_RANGE`    | `10`           | in weeks <br/> 0 = only current
