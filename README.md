# sloop
Non-interactive stateless irc message dispatcher. Great for scripts.

Here is an example run in a bash script:

```
#!/bin/bash

now=$(date +"%M")

cargo run -- -C "#sloop" -m "The minute of the hour is: $now" -n 'sloop'$now -p 6667 -s "irc.freenode.org"
```
