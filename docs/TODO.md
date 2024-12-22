## Features to add

- More complex and complete game mechanics based on the Wikipedia spec
- More user features, robust implementations (modules)
- Better UI (perspective, animations), interactive options

## Later Improvements

- FE: search of a specific tile (show how many in board, how many in other melds)
- FE: Audio effects
- FE: Animations
- FE: Perspective of tiles
- FE: Display the last tile in board in different size
- FE: Display other player melds, and bonus tiles, in small size and on hover
- BE: leaderboard using redis
- BE: promote anonymous account to real account
- BE: decouple mahjong specific logic from server to a different package
- BE: Allow running without redis
- BE: Use event sourcing in the DB for games
- CORE: Other rules from https://en.wikipedia.org/wiki/Mahjong
- CORE: Replace bool returning functions with side effects
- CORE: Minimum points win support
- CORE: Support declaring concealed melds
- CORE: Support charleston in the drawing phase
- CORE: Average rounds are too high in the simulation
- CORE: Support the deciding of the dealer with dice
- CORE: Support three players: high effort
- FS: Refactor logic to support multiple types of games (e.g. listed in wikipedia)
    - Move most business logic to the core (rust/ts)
- FS: Support rhythym of play setting
- FS: Move more logic from web to the web_lib
- FS: Random starting position
- Move other projects bash scripts to the main scripts dir
- Convert DB operations into transactions (there is an example)
- Change player names when they are AI
- Full AI game
- Use the game version in more endpoints
- Improve scoring logic (explicitly list points sources)
    - Add unit tests
- Statistics for moves
- Impersonate player from admin view
- Record of games for each player
- Ranking of players
- Dark theme
