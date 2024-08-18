# Technical design documentation

- A game that can be played in terminal
- CLI shell binary program (with Rust language)
- Compiles to WASM and can be run in browser
- Browser may need a library to emulate terminal UI
- Browser interactions with JavaScript
- Game needs to work in a real terminal as well
- Separate headless game server (with Go language)
- Game server has HTTP API
- JWT authentication
- Token is stored in-memory
- Users and login
- Users can create save files and load them between sessions
- Persist user game data to a database
- Allow anonymous players to play the game (Login is not required)
- Maybe global leaderboard
- Realtime WebSocket communication to track players that are online
