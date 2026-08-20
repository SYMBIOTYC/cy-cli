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
3. [ ] Документировать архитектуру codex (бинарник codex-cli, app-server, TUI, RPC).
4. [ ] Определить "очень быстрые методы", которых не хватает для ежедневной работы.
5. [ ] Выяснить, как расширение Codex привязывается к бинарнику и как переключить его
   на CY-CLI (патч `out/extension.js`, путь к бинарнику).
6. [ ] Реализация первых быстрых методов.

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