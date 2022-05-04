# bulke
Bulk edit files.

## Example
```bash
$ cat ./ap/jp.yaml
port: 8080
region: ap
country: jp
concurrency: 2

$ cat ./eu/de.yaml
port: 8080
region: eu
country: de
lifetime: 3600

$ bulke '
region: {=REGION}
country: {=COUNTRY}
{+endpoint: $COUNTRY.$REGION.app.local}
' './**/*.yaml'
ap/jp.yaml
eu/de.yaml

$ cat ./ap/jp.yaml
port: 8080
region: ap
country: jp
endpoint: jp.ap.app.local
concurrency: 2

$ cat ./eu/de.yaml
port: 8080
region: eu
country: de
endpoint: de.eu.app.local
lifetime: 3600
```