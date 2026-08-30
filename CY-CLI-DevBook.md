# CY-CLI-DevBook

## MIGRATION NOTE (2026-08-30)
- **OLD config:** `~/.codex/config.toml` / `~/.codex/auth.json`
- **NEW config:** `~/.cy/config.toml` / `~/.cy/auth.json`
- **OLD binary path:** `.fundament/codex-rs/target/release/cy`
- **NEW binary path:** `.fundament/cx-rs/target/release/cy`
- **OLD API base URL:** `https://cy.symbiotyc.workers.dev/v1`
- **NEW API base URL:** `https://api.cy.symbiotyc.workers.dev/v1`

## Статус (текущее)
- 2026-08-20: создан приватный репозиторий `cy-cli`, 
- Фундамент живёт в `.fundament/` (без вложенного `.git`, часть нашего дерева).

более продвинутый **CY-CLI**, в котором добавлены недостающие **очень быстрые методы**.

- Теги: `#@terminal`, `#tool`, `#terminal utility`.
✅
- **Шаг 3:** новый TUI-модуль `tui/src/ncview/mod.rs` с NC/VC/PIE-раскладкой:
  - **4 панели** (PIE Commander): top-left (файлы/сессии), top-right (агент), bottom-left (модели), bottom-right (логи);
  - layout на ratatui: 4 `Layout` зоны + командная строка + статус-бар + F-ключи;
  - F1–F10, Tab, стрелки, командная строка внизу;
  - `cy tui` / `cy nc` запускает 4-панельный интерфейс; не трогает существующий `chatwidget`.
- **Шаг 4:** собрать CY-CLI; переключить расширение: `gt.cliExecutable` → путь к `cy`.

## Сборка / инструменты
- Rust 1.97.1, cargo 1.97.1 (Homebrew). Гигантский workspace (~150 крейтов).
- `just` НЕ установлен (brew install just — медленный, время ожидания вышло); для форка не критично.

- Первая сборка `cargo build --bin cy` запущена в фоне (PID 80222, лог /tmp/cycli-build.log) —
  проверять `tail /tmp/cycli-build.log` и `ps aux | grep -c "[c]argo build"`. ✅ Собралось за ~8 мин.
  
- **CY-CLI собрался успешно** (`cargo build --bin cy` ~3 мин после базового билда):
  - Бинарь: `.fundament/cx-rs/target/debug/cy` (100MB);
  - Новые подкоманды: `cy q`, `cy m`, `cy ls`, `cy hist`, `cy b`, `cy tui` / `cy nc`;
  - **Тесты пройдены:**
    - `cy m` → показывает текущую модель (`nvidia/nemotron-3-super-120b-a12b:free`);
    - `cy m openrouter/auto` → устанавливает модель (пишет в `~/.cy/config.toml`);
    - `cy ls` → выводит 7 openrouter моделей из каталога;
    - `cy q --skip-git-repo-check "What is 2+2?"` → стриминг ответа "The sum of 2 + 2 is 4.";
    - `cy q --skip-git-repo-check --model openrouter/auto "What is the capital of France?"` → переопределяет модель на лету;
    - `cy b "test"` → парсит git status файлы;
    - `cy tui` → запускает 4-панельный TUI (F1-F10, Tab, командная строка).

## Ключевые знания о Codex CLI (из исследования)

## Видение CY-CLI (решение пользователя)
- **Реализация:** форк Rust (`cx-rs`). Не обёртка.
- **TUI = Norton Commander + Volkov Commander + PIE Commander.** Вся идея, эргономика, UI/UX/GUI внутри
  TUI — от Norton Commander; всё остальное — от Volkov Commander; **PIE Commander добавляет
  4-панельную архитектуру** (легендарный DOS-файловый менеджер с 4 панелями, удалявший диск при `rm ..`).
- **Формула:** `((цвет+стиль+геометрия+информативность+интерактивность)/Cx_CLI)^2 (UI/TUI) = CY-CLI`.
- **Компоненты эргономики (от NC/VC/PIE):**
  - **4 панели** (PIE Commander) — вместо двух: left/right + top/bottom или quadrants;
  - быстрая клавиатурная навигация (Tab, F1–F10, Ctrl/Alt комбо);
  - командная строка внизу, статус-бар, колонки, «меню пользователя» (F2);
  - горячие клавиши NC: F3(view), F4(edit), F5(copy), F6(move), F7(mkdir), F8(delete), F9(menu), F10(quit);
  - VC-скорость: мгновенный отклик, минимальный футер;
  - PIE-инновации: 4 панели, быстрые фильтры, виртуальная файловая система (VFS).
- **Быстрые методы (первый список):**
  - `cy q "<вопрос>"` — быстрый вопрос из терминала, стриминг, без полного TUI;
  - `cy m <model>` — мгновенная смена модели; `cy ls` — показать доступные модели
    (каталог `model_catalog_json`, openrouter и т.д.);
  - `cy r` — мгновенный resume последней/выбранной сессии; `cy hist` — история сессий с поиском;
  - `cy b "<инструкция>" [файлы...]` — батч-обработка нескольких файлов/промптов
    параллельно через app-server.

## Как расширение Cx спавнит бинарник (ключ к переключению на CY-CLI)
- В `out/extension.js` (минифицированно) есть `Hxe(t,e)` — спавн cx-процесса:
  ```
  Hxe(t,e){ ... let i=cI(t), a=process.env.PATH+sep+joinPath(t, Mf()).fsPath,
       c={...process.env,...n,PATH:a,RUST_LOG:"warn",CX_INTERNAL_ORIGINATOR_OVERRIDE:kf},
       u=(0,pb.spawn)(i, e, {stdio:["pipe","pipe","pipe"],env:c}); ... }
  ```
- Путь к бинарнику определяется функцией `cI(t,e)`:
  ```js
  function cI(t,e){
    let r=Nn("cliExecutable");
    if (r && r.trim().length>0) return r;          // <-- пользовательский бинарник!
    let n=Mf(e), o=(e??process.platform)==="win32"?"codex.exe":"cx";
    return Os.Uri.joinPath(t, `${n}/${o}`).fsPath;  // extensionUri/bin/<platform>/cx
  }
  ```
- **ВЫВОД:** расширение официально поддерживает подмену бинарника настрайкой
  `gt.cliExecutable` (package.json: `type:["string","null"]`, `default:null`,
  `scope:"application"`, `restricted:true` — dev-only ключ). Достаточно собрать CY-CLI
  и прописать путь в user settings.json; extension спавнит его как app-server.
- Реальные args спавна: `["-c","features.code_mode_host=true","app-server","--analytics-default-enabled"]`.
  env: `PATH+=ext bin dir`, `RUST_LOG=warn`, `CX_INTERNAL_ORIGINATOR_OVERRIDE`, опционально `CX_HOME`.
  WSL fallback отдельно (`wsl.exe`).
- Это НЕ требует патчить extension.js — настройка уже есть в расширении.

## Где живёт CLI/TUI в фундаменте 
- **Крейт TUI:** `cx-rs/tui/` — на **ratatui 0.30.2 + crossterm 0.29.0** (Rust-стек
  терминал UI). Модули: `app/` (views: agents_overview, agent_status_feed, agent_picker...),
  `chatwidget/` (composer, hooks, exec_state...), `bottom_pane/` (approval_overlay,
  apply_patch_header...), `app.rs`, `cli.rs`, `color.rs`, `tui.rs`.
- **Entry CLI:** `cli/src/main.rs` — clap-derived. Точка расширения:
  `enum Subcommand` (`main.rs:133`) c variant'ами AppServer, Exec, Resume, Fork, Mcp,
  Plugin, Review, Archive, Queue, Delete, Login/Logout, Cloud, Sandbox, Doctor, Debug...
  Уже есть `DebugSubcommand::Models` — прообраз `cy ls`.
- **Спавн бинарника расширением:** `out/extension.js` → `Hxe(t,e)` → `cI(t,e)`,
  который чтит `chatgpt.cliExecutable` (dev-only настройка). Аргументы спавна:
  `["-c","features.code_mode_host=true","app-server","--analytics-default-enabled"]`;
  env: `PATH+=<ext>/bin/<platform>`, `RUST_LOG=warn`, `CX_INTERNAL_ORIGINATOR_OVERRIDE`.
- **Сборка:** Rust 1.97.1 + cargo 1.97.1 (Homebrew) уже в системе.
- **Дизайн-правило CY-CLI:** `((цвет+стиль+геометрия+информативность+интерактивность)/Cx_CLI)^2 (UI/TUI) = CY-CLI`.
  Эргономика/UI/UX/GUI: Norton Commander; всё остальное: Volkov Commander.

## Ключевые знания о Cx CLI (из исследования)
- Системный бинарник: `codex` (`cx-cli 0.148.0`), `~/.local/bin/cx`.
- Установленное расширение: `/Users/imac/.vscode/extensions/oi.gt-26.814.41407-darwin-x64`
  (эталоны и патчи: `/Volumes/Work/CY/structured/cx-openrouter/`).
- Открытые исходники: `github.com/openai/cx` — `cx-rs/` на Rust:
  - `app-server/`, `app-server-protocol/` — JSON-RPC app-server и протокол;
  - `tui/` — терминальный интерфейс (`cx` без подкоманды);
  - `cli/`, `exec/` — CLI и `cx exec` (неинтерактивный режим);
  - `config/` — модель конфига, `config.md` (документация);
  - `model-provider/`, `models-manager/` — провайдеры, каталог моделей,
    `model_catalog_json` (именно он добавил openrouter-модели в нативный `model/list`);
  - `core/`, `core-api/` — ядро харнесса, поток агента;
  - `cx-api/` — типы API, бэкенд;
  - `sdk/` — TypeScript SDK (`@oi/cx-sdk`) и Python.
- App Server — стабильный клиентский интерфейс (JSON-RPC): `c app-server --listen ws://IP:PORT`
  (или stdio JSONL). Подключение не раньше ~2.4с после старта (иначе не 101).
  Команды: `initialize`, `config/read`, `model/list`, `config/batchWrite`, `thread/...` (см. `tools/` в `/Volumes/Work/CY/structured/cx-openrouter/`).

## Открытые вопросы
- [x] По какому каналу расширение беседует с бинарником: спавнит как app-server (stdio
  tty + JSON-Lines), адрес бинарника — `gt.cliExecutable` (настройка расширения).
- [ ] Какие именно "очень быстрые методы" нужны? (список уточнить: e.g. `cy <mesс>`?
  прототип-less? fast resume? батч-обработка файлов? пайплайны CLI?)
- [ ] Публиковать ли CY-CLI как npm-пакет или только repo + bin symlink.
- [ ] Сборка: новая версия CY-CLI — это форк codex-rs (Rust) или обёртка (thin wrapper)
  поверх бинарника cx? (fundament = Rust workspace, `cx-cli/` реэкспортирует.)
## Session update (2026-08-23)
- `/Applications/CY-CLI-intel.app` is bundled CY-CLI (`com.symbiotyc.cy-cli`) and must not be used as the source of truth; repo `SYMBIOTYC/cy-cli` is the single source of truth.
- macOS quarantine/Gatekeeper can block the app bundle from launching even after removal; do not treat the app bundle as the latest working version.
- Verified working local build remains `.fundament/cx-rs/target/release/cy` on macOS (`x86_64-apple-darwin`, ~374 MB).
- To launch the app-bundled binary in a forced Terminal session: `arch -x86_64 /Applications/CY-CLI-intel.app/Contents/MacOS/cy ...`.
- Planned next: reapply product fixes in repo, rebuild, release, install via repo scripts, and verify installed `cy` works.
