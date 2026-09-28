> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Тестирование самостоятельно размещённых сред от начала до конца

> Проверьте образ самостоятельно размещённого runner из CI: отправьте сеанс через CLI, прочитайте ответы Claude через hook Stop и напишите скрипт для полного цикла.

<Note>
  Самостоятельно размещённые среды находятся в публичной бета-версии на планах Team и Enterprise; раздел [Доступность и ограничения](/docs/ru/self-hosted-environments#availability-and-limitations) охватывает путь включения. Эта страница — рецепт для CI-тестирования; см. [краткое руководство](/docs/ru/self-hosted-environments-quickstart) для настройки и [Развёртывание в production](/docs/ru/self-hosted-environments-deploy) для рецептов флота.
</Note>

В [самостоятельно размещённой среде](/docs/ru/self-hosted-environments) облачные [сеансы](/docs/ru/claude-code-on-the-web) Claude Code работают на образе runner, который вы создаёте и поддерживаете. Перед развёртыванием нового образа в вашу production-среду запустите полный сеанс против тестовой среды из скрипта: создайте сеанс, прочитайте ответ Claude, отправьте дополнительный вопрос и прочитайте этот ответ тоже. Это форма CI smoke-теста, который проверяет ваш образ runner, доступ к git и любые пользовательские инструменты перед тем, как вы продвинете изменение.

Этот рецепт предполагает, что вы уже [настроили среду и runner](/docs/ru/self-hosted-environments-quickstart#set-up-an-environment-and-runner), и что ваша CI-задача запускает процесс runner на том же хосте, что и тестовый скрипт — это естественная настройка для тестирования нового образа runner. Hook Stop, который вы устанавливаете на runner, записывает финальный ответ каждого хода в локальный файл, и скрипт читает его оттуда, поэтому единственные вызовы к API Anthropic — это две сами отправки. Если ваши тестовые runner находятся на отдельной инфраструктуре, см. [Удалённые тестовые runner](#remote-test-runners).

<h2 id="install-the-capture-hook-on-your-test-runner">
  Установите hook захвата на ваш тестовый runner
</h2>

Обратное чтение работает через Claude Code [hook Stop](/docs/ru/hooks#stop): когда Claude завершает ход, hook получает финальное сообщение ассистента как `last_assistant_message` в JSON stdin и добавляет его в `$E2E_REPLY_DIR/<session_id>.txt`. Установите его так же, как [hook Stop commit-nudge](/docs/ru/self-hosted-environments-configuration#prompt-sessions-to-push-their-work), в `~/.claude/` на хосте runner, который runner вносит в каждый сеанс.

<h3 id="save-the-hook-files">
  Сохраните файлы hook
</h3>

Сохраните два файла ниже на хосте runner:

* Блок настроек: объедините в `~/.claude/settings.json` на хосте runner
* Скрипт: сохраните как `~/.claude/hooks/e2e-stop-hook-capture.sh` на хосте runner и сделайте его исполняемым

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "timeout": 10,
            "command": "\"$CLAUDE_CONFIG_DIR/hooks/e2e-stop-hook-capture.sh\""
          }
        ]
      }
    ]
  }
}
```

```sh theme={null}
#!/bin/sh
# Stop hook for testing a self-hosted environment end to end: writes each
# turn's final assistant reply to $E2E_REPLY_DIR/<session_id>.txt so a
# co-located test driver can read it without calling the Anthropic API.
# Install on the TEST runner only. Requires jq.

# No-op unless the driver is listening. Never fail the turn.
[ -n "${E2E_REPLY_DIR:-}" ] && [ -d "$E2E_REPLY_DIR" ] || exit 0

# CLAUDE_CODE_REMOTE_SESSION_ID is exported in cse_... form; the session
# id the dispatch CLI prints is in session_... form. Same id, different
# prefix.
sid=$(printf '%s' "${CLAUDE_CODE_REMOTE_SESSION_ID:-}" | sed 's/^cse_/session_/')
[ -n "$sid" ] || exit 0

# last_assistant_message is absent when the final assistant turn had no
# text, such as a tool-use-only turn. The `// empty` filter makes that a
# zero-byte write rather than the literal string "null".
jq -r '.last_assistant_message // empty' >> "$E2E_REPLY_DIR/$sid.txt" 2>/dev/null
exit 0
```

<h3 id="before-you-start-the-runner">
  Перед запуском runner
</h3>

Hook имеет следующие требования:

* Установите его перед запуском runner. Runner создаёт снимок `~/.claude/` один раз при запуске, поэтому hook, добавленный к работающему runner, вступает в силу только после перезагрузки.
* Экспортируйте `E2E_REPLY_DIR` в процесс runner. Hook не выполняет никаких действий, когда переменная не установлена или каталог не существует, поэтому установите её везде, где вы запускаете runner, например в модуле systemd, спецификации pod или шаге CI. Тестовый скрипт ниже также требует её.

Установите этот hook только на runner, обслуживающих ваше тестовое окружение. Он записывает финальный ответ каждого сеанса на диск всякий раз, когда существует `E2E_REPLY_DIR`, что безвредно на одноразовом CI runner, но не то, что нужно переносить в образ runner production-окружения, где переменная может быть установлена случайно.

<h2 id="run-the-test-loop">
  Запустите тестовый цикл
</h2>

Флаги dispatch `--environment` и `--ref` требуют Claude Code v2.1.224 или позже на машине, которая запускает скрипт, то же минимальное требование, что и для самого runner. С установленным hook и запущенным runner на этом хосте тестовый скрипт:

1. Создаёт сеанс в тестовом окружении с помощью `claude -p "<prompt>" --environment <environment-id> --output-format json`, запущенной из git-репозитория, чтобы CLI мог автоматически обнаружить репозиторий из удалённого `origin`. Опциональный флаг `--ref <branch>` основывает checkout сеанса на именованном ref вместо локального HEAD. Команда создаёт сеанс, выводит одну строку JSON, содержащую `session_id`, и выходит без ожидания ответа Claude.
2. Ждёт, пока ответ появится в `$E2E_REPLY_DIR/<session_id>.txt`, записанный hook Stop на runner после завершения хода.
3. Отправляет дополнительный вопрос с помощью `claude -p "<message>" --cloud <session_id> --output-format json` (см. [Отправка дополнительного сообщения в работающий сеанс](/docs/ru/claude-code-on-the-web#send-follow-ups-from-the-cli)), который отправляет событие пользователя в существующий сеанс и выходит.
4. Ждёт ответа на дополнительный вопрос так же, как на шаге 2.

<h3 id="environment-dispatch-behavior">
  Поведение dispatch `--environment`
</h3>

Claude Code создаёт сеанс, выводит ID сеанса и ссылку на него, и выходит.

Флаг имеет приоритет над параметром [`remote.defaultEnvironmentId`](/docs/ru/settings-reference#remote-defaultenvironmentid). Он не поддерживает `--output-format stream-json` и не может быть объединён с флагами, которые возобновляют, присоединяются к или предварительно настраивают сеанс, такими как `--resume`, `--continue`, `--teleport`, `--session-id` или `--init-only`. `--cloud` отклоняется с ID сеанса или URL, и в неинтерактивных запусках, когда он содержит описание. Голый `--cloud` рассматривается как отсутствующий. Из терминала вы можете передать задачу как описание `--cloud` вместо позиционного prompt.

<h2 id="example-script">
  Пример скрипта
</h2>

Скрипт ниже запускает полный цикл против `$CLAUDE_TEST_ENVIRONMENT_ID`, ID `ccpool_...` вашего тестового окружения, показанный в диалоговом окне деталей окружения на странице администратора или возвращённый вызовом [create-environment](#create-a-dedicated-test-environment), и проверяет наличие фразы-маркера в каждом ответе. Запустите его из git-репозитория, который вы хотите, чтобы сеанс работал, после запуска runner на этом хосте с установленным hook захвата и экспортированным `E2E_REPLY_DIR`.

```bash theme={null}
#!/usr/bin/env bash
# End-to-end test against a self-hosted environment, using Stop-hook read-back.
# Prereqs: `claude auth login` has been run on this machine (see "Authenticate
# from CI" below); jq is installed; CLAUDE_TEST_ENVIRONMENT_ID names an
# environment whose runner is the one on this host, with the capture hook
# installed and E2E_REPLY_DIR in its environment.

set -euo pipefail

: "${CLAUDE_TEST_ENVIRONMENT_ID:=${CLAUDE_TEST_POOL_ID:-}}"  # CLAUDE_TEST_POOL_ID is the legacy spelling
: "${CLAUDE_TEST_ENVIRONMENT_ID:?set CLAUDE_TEST_ENVIRONMENT_ID to a ccpool_... id served by a runner on this host}"
: "${E2E_REPLY_DIR:?set E2E_REPLY_DIR to the directory the Stop hook on your test runner writes to, and export it to the runner process}"
: "${TEST_REPO_REF:=main}"

[ -d "$E2E_REPLY_DIR" ] || {
  echo "FAIL: E2E_REPLY_DIR ($E2E_REPLY_DIR) does not exist. The Stop hook on the runner needs it." >&2
  exit 1
}

# Waits until $E2E_REPLY_DIR/<session_id>.txt contains $2, or fails after
# 90 seconds. Tune the timeout to your environment's cold-start time. The
# file is written by the Stop hook on the runner.
await_reply() {
  local expect="$2" f="$E2E_REPLY_DIR/$1.txt"
  local deadline=$(($(date +%s) + 90))
  while :; do
    if [ -f "$f" ] && grep -qF -- "$expect" "$f"; then
      return
    fi
    [ "$(date +%s)" -lt "$deadline" ] || {
      echo "FAIL: '$expect' not in $f within 90s. The Stop hook on the runner did not write it." >&2
      echo "-- $E2E_REPLY_DIR contents --" >&2; ls -la "$E2E_REPLY_DIR" >&2
      [ -f "$f" ] && { echo "-- $f --" >&2; cat "$f" >&2; }
      exit 1
    }
    sleep 1
  done
}

# 1. Create the session on the test environment. Run from a git checkout
# so the CLI can auto-detect the repo. --ref pins the checkout to a named
# ref regardless of local HEAD.
TURN1="e2e-probe-$(date +%s)-$$: say exactly 'ok: custom tools are reachable' and nothing else"
EXPECT1="ok: custom tools are reachable"
create_json=$(claude -p "$TURN1" --environment "$CLAUDE_TEST_ENVIRONMENT_ID" \
  --ref "$TEST_REPO_REF" --output-format json)
echo "create: $create_json"
SESSION_ID=$(jq -er '.session_id' <<<"$create_json")

# 2. Wait for the turn-1 reply.
await_reply "$SESSION_ID" "$EXPECT1"
echo "turn-1 reply ok"

# 3. Post a follow-up via the CLI.
TURN2="e2e-probe-followup-$(date +%s): say exactly 'ok: follow-up delivered' and nothing else"
EXPECT2="ok: follow-up delivered"
followup_json=$(claude -p "$TURN2" --cloud "$SESSION_ID" --output-format json)
echo "followup: $followup_json"
jq -e '.ok == true' <<<"$followup_json" >/dev/null

# 4. Wait for the turn-2 reply.
await_reply "$SESSION_ID" "$EXPECT2"
echo "turn-2 reply ok"

echo "PASS: test-environment round-trip (session $SESSION_ID)"
```

Замените prompts `TURN1`/`TURN2` и маркеры `EXPECT1`/`EXPECT2` на всё, что проверяет вашу настройку, например попросите Claude запустить один из ваших пользовательских инструментов MCP и проверьте его вывод.

<h2 id="remote-test-runners">
  Удалённые тестовые runner
</h2>

Если ваши тестовые runner находятся на отдельной инфраструктуре, например на постоянном флоте Kubernetes, с которым ваша CI-задача не может поделиться файловой системой, замените запись файла в hook Stop на POST к конечной точке, которую слушает ваш драйвер:

```sh theme={null}
#!/bin/sh
# Variant of the capture hook for runners on separate infrastructure.
# Set E2E_REPLY_URL on the runner to an endpoint the driver controls.
[ -n "${E2E_REPLY_URL:-}" ] || exit 0
sid=$(printf '%s' "${CLAUDE_CODE_REMOTE_SESSION_ID:-}" | sed 's/^cse_/session_/')
[ -n "$sid" ] || exit 0
jq -r '.last_assistant_message // empty' | \
  curl -fsS -X POST --data-binary @- "$E2E_REPLY_URL/$sid" >/dev/null 2>&1
exit 0
```

На стороне драйвера запустите что-нибудь, что принимает POST и удерживает ответ до тех пор, пока тест его не запросит, например небольшой HTTP-слушатель внутри CI-задачи или приёмник webhook, который вы уже запускаете. Hook работает на вашей инфраструктуре, поэтому конечная точка должна быть доступна только из ваших runner.

<h2 id="authenticate-from-ci">
  Аутентификация из CI
</h2>

Как `claude -p ... --environment`, так и `claude -p ... --cloud` аутентифицируются с помощью токена OAuth claude.ai; ключи API, такие как `sk-ant-xxxxx`, не принимаются ни для одного из вызовов. Два подхода делают токен доступным в CI.

<h3 id="long-lived-ci-host">
  Долгоживущий хост CI
</h3>

Запустите `claude auth login` один раз интерактивно на машине, которая выполняет скрипт, используя выделенную учётную запись пользователя для автоматизации. Claude Code хранит токен в OS keychain на macOS или в `~/.claude/.credentials.json` на Linux и Windows. На хосте macOS, чей Keychain не может быть записан, как это типично в SSH-сеансе, где Keychain входа остаётся заблокированным, Claude Code также хранит токен в `~/.claude/.credentials.json`. См. [Управление учётными данными](/docs/ru/authentication#credential-management).

CLI автоматически обновляет краткосрочный токен доступа при каждом вызове, но базовый грант refresh-token ограничен 30 днями с момента первоначального входа, поэтому повторно запустите `claude auth login` интерактивно на этом хосте каждые 30 дней.

<h3 id="ephemeral-ci-runners">
  Эфемерные CI runner
</h3>

На сегодняшний день нет долгоживущего CI-токена для этого. Область, которая предоставляет управление удалённым сеансом, `user:sessions:claude_code`, ограничена на сервере 30 днями, поэтому `claude setup-token`, который выпускает токен только для вывода на один год, не охватывает это. [Секрет окружения](/docs/ru/self-hosted-environments-quickstart#set-up-an-environment-and-runner) также не принимается, так как он только авторизует runner на регистрацию в окружении, а не на создание сеансов.

Чтобы предоставить сохранённый вход на эфемерный runner, установите [`CLAUDE_CODE_OAUTH_REFRESH_TOKEN` и `CLAUDE_CODE_OAUTH_SCOPES`](/docs/ru/env-vars#variables), чтобы `claude auth login` обменял токен без браузера; то же ограничение 30 дней применяется к гранту refresh. Свяжитесь с вашей командой учётных записей Anthropic, если вам нужен путь идентификации машины, который не привязан к учётной записи человека.

<h2 id="create-a-dedicated-test-environment">
  Создайте выделенное тестовое окружение
</h2>

Создавайте и удаляйте окружения программно, чтобы каждый запуск CI получал чистое; runner, который запускает ваша CI-задача, регистрируется в свежем окружении. Вызовы create и delete ниже — это те же конечные точки, которые использует страница администратора **Cloud environments** на claude.ai, и они требуют заголовка `anthropic-beta: ccr-byoc-2025-07-29`.

<h3 id="mint-the-admin-token">
  Выпустите токен администратора
</h3>

`$ADMIN_TOKEN` — это токен доступа OAuth claude.ai для учётной записи, которая имеет роль Owner, выпущенный так же, как [Аутентификация из CI](#authenticate-from-ci):

* **Выпустите его**: запустите `claude auth login` с учётной записью, которая имеет роль Owner, затем прочитайте текущий токен доступа из того места, где [Долгоживущий хост CI](#long-lived-ci-host) говорит, что Claude Code его сохранил.
* **Прочитайте его свежим при каждом запуске**: CLI ротирует токен доступа, и то же ограничение 30 дней для гранта refresh применяется, поэтому не сохраняйте копию.
* **Передайте его через stdin**: как делает пример, чтобы токен никогда не попал в список аргументов curl или ваш журнал сборки.

<h3 id="create-the-environment">
  Создайте окружение
</h3>

Захватите ответ без его вывода: `pool_secret` — это долгоживущее учётное данные, которое может регистрировать runner в окружении, поэтому сохраните его как замаскированный секрет CI и выводите только ID окружения. Форма `-H @-`, которая держит токен вне списка процессов, требует curl 7.55 или позже; более старый curl рассматривает `@-` как буквальный заголовок и отправляет запрос без авторизации.

```bash theme={null}
create=$(curl -fsS -X POST -H @- \
  -H "anthropic-beta: ccr-byoc-2025-07-29" -H "anthropic-version: 2023-06-01" \
  -H "content-type: application/json" \
  -d '{"name":"ci-test-environment"}' \
  https://api.anthropic.com/v1/code/runners/self-hosted/pools \
  <<<"Authorization: Bearer $ADMIN_TOKEN")
ENVIRONMENT_ID=$(jq -er .pool.pool_id <<<"$create")
ENVIRONMENT_SECRET=$(jq -er .pool_secret <<<"$create")
```

Пока [Owner не включит **Allow self-hosted environments**](/docs/ru/self-hosted-environments#availability-and-limitations) для организации, вызов завершится с ошибкой `403` `permission_error`, читающей `self-hosted runners are disabled by your organization's policy`.

Запустите runner на этом хосте с `SELF_HOSTED_RUNNER_ENVIRONMENT_SECRET=$ENVIRONMENT_SECRET`, плюс hook захвата и `E2E_REPLY_DIR` согласно [Установите hook захвата](#install-the-capture-hook-on-your-test-runner), затем запустите тестовый скрипт.

<h3 id="delete-the-environment">
  Удалите окружение
</h3>

Удалите окружение после завершения запуска, чтобы каждый запуск CI начинался чистым:

```bash theme={null}
curl -fsS -X DELETE -H @- \
  -H "anthropic-beta: ccr-byoc-2025-07-29" -H "anthropic-version: 2023-06-01" \
  "https://api.anthropic.com/v1/code/runners/self-hosted/pools/$ENVIRONMENT_ID" \
  <<<"Authorization: Bearer $ADMIN_TOKEN"
```
