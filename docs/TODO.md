## Features to add

- More complex and complete game mechanics
- More user features, robust implementations (modules)
- Better UI (using canvas, no antd), interactive options

## Later Improvements

- No TUI
- Refactor logic to support multiple types of games
    - Move most business logic to the core (rust/ts)
- Move other projects bash scripts to the main scripts dir
- Convert DB operations into transactions
- Random user position (especially with other players)
- ESLint rule noshadow
- Move useful utils (e.g. i18n, locale) to the TS SDK

## Backlog ideas

- Change player names when they are AI
- Form to create AI game vs real players game
    - Link to invite players to game (and qr code)
- Full AI game
- Use the game version in more endpoints
- Improve scoring logic (explicitly list each points)
    - Add unit tests
- Game hall state where it waits for other real players to join
- Statistics for moves
- Impersonate player from admin view
- Record of games for each player
- Ranking of players
- Dark theme
- Feature reduction: No wasm / no web lib
