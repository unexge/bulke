# bulke
Bulk edit files.

## Example (not working yet)
```bash
$ cat ./ap/jp.yaml
// ...
region: ap
country: jp

$ cat ./eu/de.yaml
// ...
region: eu
country: de
// ...

$ bulke '
region: {=REGION}
country: {=COUNTRY}
{+endpoint: $COUNTRY.$REGION.app.local}
' ./**/*.yaml

$ cat ./ap/jp.yaml
// ...
region: ap
country: jp
endpoint: jp.ap.app.local
// ...

$ cat ./eu/de.yaml
// ...
region: eu
country: de
endpoint: de.eu.app.local
// ...
```