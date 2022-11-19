# taurifetch
neofetch but written using [Tauri](https://tauri.app/)

## How to run project
### Stuff required
1. Install Tauri, check [this](https://tauri.app/v1/guides/getting-started/prerequisites/) article.
2. Install `yarn`.
3. Navigate to root folder of project.
4. In terminal type: `yarn install`; this part can be optional sometimes, depends on your installation.

### Live server
1. Open live server in VS Code using extension or in any other IDE or independent live server program (if you want to change address or port, you have to change strings in `package.json` (just port) and in `tauri.conf.json` (addresss)).
2. And then run `yarn tauri dev`; running command first time might take some time.

### Without live server
1. Remove this line (`"localhost": "./node_modules/.bin/servor src index.html 5050 --reload",`) from `package.json`.
2. Change this line (`"devPath": "http://127.0.0.1:5500/src/",`) in `tauri.conf.json` into `"devPath": "../",`.
3. And then run `yarn tauri dev` or `yarn tauri build` where you get access to installers; running command first time might take some time.

## Resources used
- [Fireship's video](https://www.youtube.com/watch?v=-X8evddpu7M)
- [StackOverflow question 56300132](https://stackoverflow.com/questions/56300132/how-to-override-css-prefers-color-scheme-setting)
