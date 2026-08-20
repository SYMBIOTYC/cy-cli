# CY-CLI-DevBook

Рабочие записи и рассуждения по проекту **CY-CLI**. Ведётся параллельно с разработкой.
Все мысли из чатов про CY-CLI сюда переносятся и дополняются по мере прогресса.

## Статус (текущее)
- 2026-08-20: создан приватный репозиторий `cy-cli`, склонирован фундамент
  openai/codex (shallow, ~92M, коммит 312b62a; в системе `codex-cli 0.148.0`).
- Фундамент живёт в `.fundament/` (без вложенного `.git`, часть нашего дерева).

## Миссия
Взять за фундамент открытый **Codex CLI** (копия исходников), сделать на его базе
более продвинутый **CY-CLI**, в котором добавлены недостающие **очень быстрые методы**.
Расширение Codex (VS Code, `openai.chatgpt`) перевести на использование нового CY-CLI.

## Что такое "oneline" / уточнение требований
- В списке идей прошлого чата я упомянул "(интеграция с oneline и т.п.)".
- Инструмента под именем `oneline` в системе/репозиториях НЕ нашлось — ни бинарника,
  ни папок, ни упоминаний в agent.MD / DevBook.
- Спросил у пользователя. Ответ: речь о **CY-CLI** — отдельном новом репозитории,
  фундамент = Codex CLI, сверху добавить недостающие быстрые методы, расширение Codex
  перевести на CY-CLI. Все рассуждения записывать в `CY-CLI-DevBook.md`.
- Теги задачи: `#@terminal`, `#tool`, `#terminal utility`.

## План работ
1. [x] Создать репозиторий cy-cli (локально) + private GitHub-репозиторий.
2. [x] Скопировать исходники Codex CLI как фундамент (`git clone --shallow`).
3. [x] Документировать архитектуру codex (найден `chatgpt.cliExecutable`-хук).
4. [x] Исследовать TUI-структуру codex-rs (ratatui 0.30 + crossterm 0.29).
5. [ ] Первый спринт: собрать форк, переименовать бинарь codex → cy (`codex-cli` crate → `cy`),
   добавить subcommands `cy q / m / ls / r / hist / b` в `enum Subcommand` (cli/src/main.rs).
6. [ ] Новый NC/VC TUI: двухпанельный layout (левый/правый), командная строка снизу,
   F1–F10 / Tab горячие клавиши, статус-бар, «меню пользователя». Переписать корневой
   рендер в `tui/src/app.rs` под этот layout; стили — свои (NC/VC) НЕ из `tui/styles.md`
   (там конвенции codex: bold/dim/cyan/green/red/magenta; для NC-стиля понадобится своя палитра).
7. [ ] Собрать CY-CLI; переключить расширение: `chatgpt.cliExecutable` → путь к `cy`.

## План первого спринта (детально)
- **Шаг 0 (текущий):** `cargo build --bin codex` — проверка, что фундамент собирается локально.
- **Шаг 1:** переименовать бинарь: в `codex-rs/cli/Cargo.toml` → `[[bin]] name="cy"`, `default-run="cy"`;
  либо сделать тонкий новый crate-бинарь поверх `codex_cli` lib. Прогнать `cargo build --bin cy`.
- **Шаг 2:** новые подкоманды в `cli/src/main.rs` `enum Subcommand`:
  - `cy q "<промпт>"` — быстрый вопрос (однократный проход агента, стриминг вывода);
  - `cy m <model>` / `cy ls [query]` — модели (читает `model_catalog_json`, config.toml);
  - `cy r [id]` / `cy hist [query]` — resume/история сессий;
  - `cy b "<инструкция>" [файлы...]` — батч-обработка (параллельные проходы, вывод сводки).
- **Шаг 3:** начать новый TUI-модуль (например `tui/src/ncview/`) с NC/VC-раскладкой, не трогая
  существующий chatwidget — чтобы фундамент оставался рабочим до полной замены.

## Сборка / инструменты
- Rust 1.97.1, cargo 1.97.1 (Homebrew). Гигантский workspace (~150 крейтов).
- `just` НЕ установлен (brew install just — медленный, время ожидания вышло); для форка не критично.
- Первая сборка `cargo build --bin codex` запущена в фоне (PID 80222, лог /tmp/cycli-build.log) —
  проверять `tail /tmp/cycli-build.log` и `ps aux | grep -c "[c]argo build"`.

## Ключевые знания о Codex CLI (из исследования)

## Видение CY-CLI (решение пользователя)
- **Реализация:** форк Rust (`codex-rs`). Не обёртка.
- **TUI = Norton Commander + Volkov Commander.** Вся идея, эргономика, UI/UX/GUI внутри
  TUI — от Norton Commander; всё остальное — от Volkov Commander.
- **Формула:** `((цвет+стиль+геометрия+информативность+интерактивность)/Codex_CLI)^2 (UI/TUI) = CY-CLI`.
- **Компоненты эргономики (от NC/VC):**
  - две панели (левая/правая), альтернативные панели;
  - быстрое переключение между панелями (Tab);
  - командная строка внизу, управление через горячие клавиши (F1–F10, Ctrl/Alt комбо);
  - статус-бар, инфо о файле / текущей модели, колонки;
  - «меню пользователя» (панель с горячими клавишами) — быстрые команды;
  - быстрая клавиатурная навигация без мыши.
- **Быстрые методы (первый список):**
  - `cy q "<вопрос>"` — быстрый вопрос из терминала, стриминг, без полного TUI;
  - `cy m <model>` — мгновенная смена модели; `cy ls` — показать доступные модели
    (каталог `model_catalog_json`, openrouter и т.д.);
  - `cy r` — мгновенный resume последней/выбранной сессии; `cy hist` — история сессий с поиском;
  - `cy b "<инструкция>" [файлы...]` — батч-обработка нескольких файлов/промптов
    параллельно через app-server.

## Как расширение Codex спавнит бинарник (ключ к переключению на CY-CLI)
- В `out/extension.js` (минифицированно) есть `Hxe(t,e)` — спавн codex-процесса:
  ```
  Hxe(t,e){ ... let i=cI(t), a=process.env.PATH+sep+joinPath(t, Mf()).fsPath,
       c={...process.env,...n,PATH:a,RUST_LOG:"warn",CODEX_INTERNAL_ORIGINATOR_OVERRIDE:kf},
       u=(0,pb.spawn)(i, e, {stdio:["pipe","pipe","pipe"],env:c}); ... }
  ```
- Путь к бинарнику определяется функцией `cI(t,e)`:
  ```js
  function cI(t,e){
    let r=Nn("cliExecutable");
    if (r && r.trim().length>0) return r;          // <-- пользовательский бинарник!
    let n=Mf(e), o=(e??process.platform)==="win32"?"codex.exe":"codex";
    return Os.Uri.joinPath(t, `${n}/${o}`).fsPath;  // extensionUri/bin/<platform>/codex
  }
  ```
- **ВЫВОД:** расширение официально поддерживает подмену бинарника настрайкой
  `chatgpt.cliExecutable` (package.json: `type:["string","null"]`, `default:null`,
  `scope:"application"`, `restricted:true` — dev-only ключ). Достаточно собрать CY-CLI
  и прописать путь в user settings.json; extension спавнит его как app-server.
- Реальные args спавна: `["-c","features.code_mode_host=true","app-server","--analytics-default-enabled"]`.
  env: `PATH+=ext bin dir`, `RUST_LOG=warn`, `CODEX_INTERNAL_ORIGINATOR_OVERRIDE`, опционально `CODEX_HOME`.
  WSL fallback отдельно (`wsl.exe`).
- Это НЕ требует патчить extension.js — настройка уже есть в расширении.

## Где живёт CLI/TUI в фундаменте (для форка)
- **Крейт TUI:** `codex-rs/tui/` — на **ratatui 0.30.2 + crossterm 0.29.0** (Rust-стек
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
  env: `PATH+=<ext>/bin/<platform>`, `RUST_LOG=warn`, `CODEX_INTERNAL_ORIGINATOR_OVERRIDE`.
- **Сборка:** Rust 1.97.1 + cargo 1.97.1 (Homebrew) уже в системе.
- **Дизайн-правило CY-CLI:** `((цвет+стиль+геометрия+информативность+интерактивность)/Codex_CLI)^2 (UI/TUI) = CY-CLI`.
  Эргономика/UI/UX/GUI: Norton Commander; всё остальное: Volkov Commander.

## Ключевые знания о Codex CLI (из исследования)
- Системный бинарник: `codex` (`codex-cli 0.148.0`), `~/.local/bin/codex`.
- Установленное расширение: `/Users/imac/.vscode/extensions/openai.chatgpt-26.814.41407-darwin-x64`
  (эталоны и патчи: `/Volumes/Work/CY/structured/codex-openrouter/`).
- Открытые исходники: `github.com/openai/codex` — `codex-rs/` на Rust:
  - `app-server/`, `app-server-protocol/` — JSON-RPC app-server и протокол;
  - `tui/` — терминальный интерфейс (`codex` без подкоманды);
  - `cli/`, `exec/` — CLI и `codex exec` (неинтерактивный режим);
  - `config/` — модель конфига, `config.md` (документация);
  - `model-provider/`, `models-manager/` — провайдеры, каталог моделей,
    `model_catalog_json` (именно он добавил openrouter-модели в нативный `model/list`);
  - `core/`, `core-api/` — ядро харнесса, поток агента;
  - `codex-api/` — типы API, бэкенд;
  - `sdk/` — TypeScript SDK (`@openai/codex-sdk`) и Python.
- App Server — стабильный клиентский интерфейс (JSON-RPC): `codex app-server --listen ws://IP:PORT`
  (или stdio JSONL). Подключение не раньше ~2.4с после старта (иначе не 101).
  Команды: `initialize`, `config/read`, `model/list`, `config/batchWrite`, `thread/...` (см. `tools/` в `/Volumes/Work/CY/structured/codex-openrouter/`).

## Открытые вопросы
- [x] По какому каналу расширение беседует с бинарником: спавнит как app-server (stdio
  tty + JSON-Lines), адрес бинарника — `chatgpt.cliExecutable` (настройка расширения).
- [ ] Какие именно "очень быстрые методы" нужны? (список уточнить: e.g. `cy <mesс>`?
  прототип-less? fast resume? батч-обработка файлов? пайплайны CLI?)
- [ ] Публиковать ли CY-CLI как npm-пакет или только repo + bin symlink.
- [ ] Сборка: новая версия CY-CLI — это форк codex-rs (Rust) или обёртка (thin wrapper)
  поверх бинарника codex? (fundament = Rust workspace, `codex-cli/` реэкспортирует.)