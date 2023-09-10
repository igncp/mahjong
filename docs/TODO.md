## Doing

- Change player names when they are AI
- Form to create AI game vs real players game
    - Link to invite players to game
- Setup code-push and webview components in the mobile app
    - https://github.com/Microsoft/react-native-code-push

## Improvements

- Random user position (especially with other players)
- Diesel updates (no remove + insert for sqlite)
- Refactor the socket server
- ESLint rule noshadow
- Move web and server logic which should be in core
    - Move logic in core to correct places
- Move useful utils (e.g. i18n, locale) to the TS SDK

## Backlog

- Full AI game
- Use the game version in more endpoints
- Improve scoring logic
- Consider connecting to postgres and setup container DB for moves
- Game hall state where it waits for other real players to join
- Statistics for moves
- Impersonate player from admin view
- Record of games for each player
- Ranking of players
- Dark theme
