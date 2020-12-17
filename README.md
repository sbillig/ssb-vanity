# ssb-vanity

Generate vanity keys for secure scuttlebutt.

```
cargo install ssb-vanity
ssb-vanity --prefix foo --threads 4
```

sample output (clipped):
```
@fOokzWbsbMfFu0gOc3rqBNm2EapbmZ3nksxjW66xSYA=.ed25519 HqV5k25Ev...
@fOOyl9GHCY0JDscjkEfHrHYk1ivZcydlEYgyjB119RE=.ed25519 nwvawV1SP...
@FOotFB/2BoE1Jp/sE1S6Z+GlDhQtQW/2RskngxxyVEg=.ed25519 uiVmiENCv...
@foOTMKSe2sl7+pA45ytr4hFeLEC0evYWp5u7N8SaxsU=.ed25519 KZfTeMXah...
@FoOfA52JVfu7VWrYFeWhh/9G6eCzIXR4cyqyJ6ZeeKY=.ed25519 CJsZtQ8rO...
@foo7VavatiBSpeyo32iSX2gTq9r9Dc8JvVFvaGotKaA=.ed25519 /aPuC8WI6...
```

This will print all case-insensitive matches, and will stop if/when it finds a case-sensitive match.
Matching a prefix of length 3 or 4 is quick enough; 5 or more will take some time.
