> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Добавление компонентов в плагин

> Добавляйте skills, hooks, MCP серверы и все остальные типы компонентов в плагин Claude Code с примерами, которые проходят валидацию для каждого.

export const Piece = ({id, children}) => <div className="pe-piece" data-piece={id}>{children}</div>;

export const PluginExplorer = ({children}) => {
  const PIECES = [{
    id: 'manifest',
    name: 'Manifest',
    path: '.claude-plugin/plugin.json',
    required: "Required by Anthropic's directory",
    lines: [{
      depth: 0,
      kind: 'folder',
      text: '.claude-plugin/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'plugin.json'
    }],
    href: '/en/plugins/manifest-reference#manifest-file',
    linkText: 'Go to the manifest reference'
  }, {
    id: 'skills',
    name: 'Skills',
    path: 'skills/review/SKILL.md',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'skills/'
    }, {
      depth: 1,
      kind: 'folder',
      text: 'review/'
    }, {
      depth: 2,
      kind: 'file',
      text: 'SKILL.md'
    }],
    href: '/en/plugins/components#skills',
    linkText: 'Go to the Skills section'
  }, {
    id: 'commands',
    name: 'Commands',
    path: 'commands/about.md',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'commands/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'about.md'
    }],
    href: '/en/plugins/components#commands',
    linkText: 'Go to the Commands section'
  }, {
    id: 'agents',
    name: 'Agents',
    path: 'agents/security-reviewer.md',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'agents/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'security-reviewer.md'
    }],
    href: '/en/plugins/components#agents',
    linkText: 'Go to the Agents section'
  }, {
    id: 'hooks',
    name: 'Hooks',
    path: 'hooks/hooks.json',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'hooks/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'hooks.json'
    }],
    href: '/en/plugins/components#hooks',
    linkText: 'Go to the Hooks section'
  }, {
    id: 'monitors',
    name: 'Monitors',
    path: 'monitors/monitors.json',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'monitors/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'monitors.json'
    }],
    href: '/en/plugins/components#monitors',
    linkText: 'Go to the Monitors section'
  }, {
    id: 'output-styles',
    name: 'Output styles',
    path: 'output-styles/terse.md',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'output-styles/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'terse.md'
    }],
    href: '/en/plugins/components#themes-and-output-styles',
    linkText: 'Go to the Themes and output styles section'
  }, {
    id: 'themes',
    name: 'Themes',
    path: 'themes/dracula.json',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'themes/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'dracula.json'
    }],
    href: '/en/plugins/components#themes-and-output-styles',
    linkText: 'Go to the Themes and output styles section'
  }, {
    id: 'workflows',
    name: 'Workflows',
    path: 'workflows/audit-routes.js',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'workflows/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'audit-routes.js'
    }],
    href: '/en/workflows#distribute-a-workflow-in-a-plugin',
    linkText: 'Go to Distribute a workflow in a plugin'
  }, {
    id: 'bin',
    name: 'Executables',
    path: 'bin/hello-plugin',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'bin/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'hello-plugin'
    }],
    href: '/en/plugins/components#executables',
    linkText: 'Go to the Executables section'
  }, {
    id: 'scripts',
    name: 'Scripts',
    path: 'scripts/format.sh',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'scripts/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'format.sh'
    }],
    href: '/en/plugins/components#hooks',
    linkText: 'Go to the Hooks section'
  }, {
    id: 'settings',
    name: 'Default settings',
    path: 'settings.json',
    lines: [{
      depth: 0,
      kind: 'file',
      text: 'settings.json'
    }],
    href: '/en/plugins/components#default-settings',
    linkText: 'Go to the Default settings section'
  }, {
    id: 'mcp',
    name: 'MCP servers',
    path: '.mcp.json',
    lines: [{
      depth: 0,
      kind: 'file',
      text: '.mcp.json'
    }],
    href: '/en/plugins/components#mcp-servers',
    linkText: 'Go to the MCP servers section'
  }, {
    id: 'lsp',
    name: 'LSP servers',
    path: '.lsp.json',
    lines: [{
      depth: 0,
      kind: 'file',
      text: '.lsp.json'
    }],
    href: '/en/plugins/components#lsp-servers',
    linkText: 'Go to the LSP servers section'
  }];
  const [selectedId, setSelectedId] = useState('manifest');
  const [isFullscreen, setIsFullscreen] = useState(false);
  const rootRef = useRef(null);
  useEffect(() => {
    const onFsChange = () => setIsFullscreen(!!document.fullscreenElement);
    document.addEventListener('fullscreenchange', onFsChange);
    return () => document.removeEventListener('fullscreenchange', onFsChange);
  }, []);
  const toggleFullscreen = () => {
    if (!rootRef.current) return;
    if (document.fullscreenElement) document.exitFullscreen(); else rootRef.current.requestFullscreen().catch(() => {});
  };
  const selected = PIECES.find(p => p.id === selectedId) || PIECES[0];
  const onTreeKeyDown = e => {
    const keys = ['ArrowDown', 'ArrowUp', 'Home', 'End'];
    if (keys.indexOf(e.key) === -1) return;
    const i = PIECES.findIndex(p => p.id === selectedId);
    let next = i;
    if (e.key === 'ArrowDown') next = Math.min(PIECES.length - 1, i + 1);
    if (e.key === 'ArrowUp') next = Math.max(0, i - 1);
    if (e.key === 'Home') next = 0;
    if (e.key === 'End') next = PIECES.length - 1;
    e.preventDefault();
    if (next === i) return;
    const id = PIECES[next].id;
    setSelectedId(id);
    const el = document.getElementById('pe-node-' + id);
    if (el) el.focus();
  };
  const FolderIcon = () => <svg className="pe-icon" width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.3" strokeLinejoin="round" aria-hidden="true">
      <path d="M1.5 4.5a1 1 0 0 1 1-1h3.2l1.3 1.5h6a1 1 0 0 1 1 1V12a1 1 0 0 1-1 1h-10.5a1 1 0 0 1-1-1z" />
    </svg>;
  const FileIcon = () => <svg className="pe-icon" width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.3" strokeLinejoin="round" aria-hidden="true">
      <path d="M4 1.5h5.5L13 5v9.5H4z" />
      <path d="M9.5 1.5V5H13" />
    </svg>;
  return <div ref={rootRef} className={isFullscreen ? 'pe-root pe-fullscreen not-prose' : 'pe-root not-prose'} data-selected={selected.id}>
      <style>{`
        .pe-root {
          --pe-mono: var(--font-mono, ui-monospace, SFMono-Regular, Menlo, monospace);
          --pe-accent: #D97757;
          --pe-accent-text: #A8502F;
          --pe-accent-bg: rgba(217,119,87,0.10);
          --pe-bg: #FFFFFF;
          --pe-surface: #FAFAF7;
          --pe-hover: #F0EEE6;
          --pe-border: #E8E6DC;
          --pe-text: #141413;
          --pe-text-2: #3D3D3A;
          --pe-text-3: #5E5D59;
          font-family: inherit;
          background: var(--pe-bg);
          color: var(--pe-text);
          border: 1px solid var(--pe-border);
          border-radius: 12px;
          margin: 1.5rem 0;
          overflow: hidden;
          box-sizing: border-box;
        }
        .dark .pe-root {
          --pe-accent-text: #EBA98F;
          --pe-accent-bg: rgba(217,119,87,0.18);
          --pe-bg: #1A1918;
          --pe-surface: #232221;
          --pe-hover: #2E2D2B;
          --pe-border: #3A3936;
          --pe-text: #F1EFE9;
          --pe-text-2: #D6D4CA;
          --pe-text-3: #B8B5AD;
        }
        .pe-root *, .pe-root *::before, .pe-root *::after { box-sizing: border-box; }
        .pe-head { display: flex; align-items: flex-start; gap: 12px; padding: 18px 24px 16px; border-bottom: 1px solid var(--pe-border); }
        .pe-head-text { flex: 1; min-width: 0; }
        .pe-fs-btn { flex-shrink: 0; width: 32px; height: 32px; display: inline-flex; align-items: center; justify-content: center; border: 1px solid var(--pe-border); border-radius: 6px; background: var(--pe-surface); color: var(--pe-text-2); font-size: 15px; line-height: 1; cursor: pointer; }
        .pe-fs-btn:hover { background: var(--pe-hover); }
        .pe-fs-btn:focus-visible { outline: 2px solid var(--pe-accent); outline-offset: 2px; }
        .pe-fullscreen { border-radius: 0; height: 100vh; display: flex; flex-direction: column; overflow: auto; }
        .pe-fullscreen .pe-body { flex: 1; }
        .pe-title { font-size: 19px; font-weight: 600; line-height: 1.3; color: var(--pe-text); margin: 0; }
        .pe-sub { font-size: 15px; line-height: 1.5; color: var(--pe-text-3); margin: 4px 0 0; }
        .pe-sub code { font-family: var(--pe-mono); font-size: 0.88em; padding: 1px 5px; border-radius: 4px; background: var(--pe-surface); border: 1px solid var(--pe-border); }
        .pe-body { display: flex; align-items: stretch; }
        .pe-tree-pane { width: 270px; flex-shrink: 0; background: var(--pe-surface); border-right: 1px solid var(--pe-border); padding: 16px 0 12px; }
        .pe-panel { flex: 1; min-width: 0; padding: 16px 24px 24px; }
        .pe-caption { font-size: 13px; font-weight: 600; color: var(--pe-text-3); margin: 0 0 10px; }
        .pe-tree-pane .pe-caption { padding: 0 16px; }
        .pe-rootline { display: flex; align-items: center; gap: 7px; padding: 3px 16px; font-family: var(--pe-mono); font-size: 13.5px; color: var(--pe-text-3); }
        .pe-node {
          display: block; width: 100%; margin: 0; padding: 3px 16px 3px 30px; text-align: left; cursor: pointer;
          background: transparent; color: var(--pe-text-2);
          border: none; border-left: 3px solid transparent;
          font-family: var(--pe-mono); font-size: 13.5px; line-height: 1.4;
        }
        .pe-node:hover { background: var(--pe-hover); }
        .pe-node:focus-visible { outline: 2px solid var(--pe-accent); outline-offset: -2px; }
        .pe-node[aria-pressed="true"] { background: var(--pe-accent-bg); border-left-color: var(--pe-accent); color: var(--pe-accent-text); font-weight: 600; }
        .pe-line { display: flex; align-items: center; gap: 7px; padding: 2px 0; }
        .pe-line-tree { flex-wrap: wrap; }
        .pe-line-tree .pe-req { flex-basis: 100%; margin: 2px 0 0 22px; white-space: normal; width: fit-content; max-width: calc(100% - 22px); }
        .pe-line span { overflow-wrap: anywhere; }
        .pe-piece { display: none; font-size: 16px; line-height: 1.6; color: var(--pe-text-2); }
        .pe-root[data-selected="manifest"] .pe-piece[data-piece="manifest"],
        .pe-root[data-selected="skills"] .pe-piece[data-piece="skills"],
        .pe-root[data-selected="commands"] .pe-piece[data-piece="commands"],
        .pe-root[data-selected="agents"] .pe-piece[data-piece="agents"],
        .pe-root[data-selected="hooks"] .pe-piece[data-piece="hooks"],
        .pe-root[data-selected="monitors"] .pe-piece[data-piece="monitors"],
        .pe-root[data-selected="output-styles"] .pe-piece[data-piece="output-styles"],
        .pe-root[data-selected="themes"] .pe-piece[data-piece="themes"],
        .pe-root[data-selected="workflows"] .pe-piece[data-piece="workflows"],
        .pe-root[data-selected="bin"] .pe-piece[data-piece="bin"],
        .pe-root[data-selected="scripts"] .pe-piece[data-piece="scripts"],
        .pe-root[data-selected="settings"] .pe-piece[data-piece="settings"],
        .pe-root[data-selected="mcp"] .pe-piece[data-piece="mcp"],
        .pe-root[data-selected="lsp"] .pe-piece[data-piece="lsp"] { display: block; }
        .pe-piece p { margin: 0 0 10px; }
        .pe-piece p:last-child { margin-bottom: 0; }
        .pe-piece code { font-family: var(--pe-mono); font-size: 0.88em; padding: 1px 5px; border-radius: 4px; background: var(--pe-surface); border: 1px solid var(--pe-border); }
        .pe-piece .code-block { margin: 12px 0 0; }
        .pe-piece pre code { padding: 0; border: none; background: none; }
        .pe-piece a { color: var(--pe-accent-text); }
        .pe-line-compact { display: none; }
        .pe-icon { flex-shrink: 0; }
        .pe-req { margin-left: 8px; padding: 0 6px; border-radius: 999px; font-size: 11px; line-height: 18px; letter-spacing: .02em; color: var(--pe-accent-text); border: 1px solid var(--pe-border); background: var(--pe-surface); white-space: nowrap; font-weight: 500; vertical-align: middle; }
        .pe-name { font-size: 22px; font-weight: 600; line-height: 1.25; letter-spacing: -0.2px; color: var(--pe-text); margin: 0; }
        .pe-path { font-family: var(--pe-mono); font-size: 13.5px; color: var(--pe-accent-text); margin: 4px 0 0; overflow-wrap: anywhere; }
        .pe-block { margin: 20px 0 0; }
        .pe-link {
          display: inline-block; margin: 24px 0 0; padding: 8px 14px; border-radius: 8px;
          font-size: 14.5px; font-weight: 600; text-decoration: none;
          color: var(--pe-accent-text); background: var(--pe-accent-bg); border: 1px solid var(--pe-accent);
        }
        .pe-link:hover { filter: brightness(0.97); }
        .pe-link:focus-visible { outline: 2px solid var(--pe-accent); outline-offset: 2px; }
        @media (max-width: 700px) {
          .pe-head { padding: 16px 16px 14px; }
          .pe-body { flex-direction: column; }
          .pe-tree-pane { width: 100%; border-right: none; border-bottom: 1px solid var(--pe-border); }
          .pe-line-tree { display: none; }
          .pe-line-compact { display: flex; }
          .pe-panel { padding: 16px 16px 20px; }
        }
      `}</style>

      <div className="pe-head">
        <div className="pe-head-text">
          <div className="pe-title">What goes in a plugin</div>
          <div className="pe-sub">This example plugin, <code>my-plugin</code>, has one of every kind of component, each in its default location. Select a file or folder to read what it’s for and see what goes in it.</div>
        </div>
        <button type="button" className="pe-fs-btn" onClick={toggleFullscreen} aria-label={isFullscreen ? 'Exit fullscreen' : 'Fullscreen'} title={isFullscreen ? 'Exit fullscreen' : 'Fullscreen'}>
          {isFullscreen ? '⤡' : '⛶'}
        </button>
      </div>

      <div className="pe-body">
        <div className="pe-tree-pane">
          <div className="pe-caption" id="pe-tree-caption">Plugin directory</div>
          <div role="group" aria-labelledby="pe-tree-caption" onKeyDown={onTreeKeyDown}>
            <div className="pe-rootline"><FolderIcon /><span>my-plugin/</span></div>
            {PIECES.map(p => <button key={p.id} id={'pe-node-' + p.id} type="button" className="pe-node" aria-pressed={p.id === selected.id} aria-label={p.name + ', ' + p.path} onClick={() => setSelectedId(p.id)}>
                {p.lines.map((line, i) => <span key={i} className="pe-line pe-line-tree" style={{
    paddingLeft: line.depth * 18 + 'px'
  }}>
                    {line.kind === 'folder' ? <FolderIcon /> : <FileIcon />}
                    <span>{line.text}</span>
                    {p.required && i === p.lines.length - 1 ? <span className="pe-req">{p.required}</span> : null}
                  </span>)}
                <span className="pe-line pe-line-compact">
                  <FileIcon />
                  <span>{p.path}</span>
                  {p.required ? <span className="pe-req">{p.required}</span> : null}
                </span>
              </button>)}
          </div>
        </div>

        <div className="pe-panel" role="region" aria-labelledby="pe-panel-caption" aria-live="polite" aria-atomic="true">
          <div className="pe-caption" id="pe-panel-caption">Selected piece</div>
          <div className="pe-name">{selected.name}{selected.required ? <span className="pe-req">{selected.required}</span> : null}</div>
          <div className="pe-path">{selected.path}</div>

          <div className="pe-block">{children}</div>

          <a className="pe-link" href={selected.href}>{selected.linkText}</a>
        </div>
      </div>
    </div>;
};

Плагин Claude Code строится из компонентов, таких как skills, agents, hooks и MCP серверы. Каждый компонент имеет папку по умолчанию в плагине, необязательный ключ манифеста в `.claude-plugin/plugin.json`, который заменяет или добавляет к этой папке, и имя, которое видит пользователь. Для полной таблицы полей каждого ключа см. [справочник манифеста](/docs/ru/plugins/manifest-reference#fields).

Используйте эту страницу для добавления компонента в плагин, который уже загружается.

После добавления компонента запустите `/reload-plugins` в работающей сессии или начните новую, чтобы Claude Code загрузил его. Чтобы проверить файл компонента перед загрузкой, запустите [`claude plugin validate .`](/docs/ru/plugins/cli-reference#plugin-validate) в вашей оболочке из директории плагина.

<Note>
  Эти случаи рассматриваются на других страницах:

  * **Создание вашего первого плагина**: начните с [Create a plugin](/docs/ru/plugins/create)
  * **Установка плагина кого-то другого**: см. [Install plugins](/docs/ru/plugins/install)
  * **Ваши пользователи плагина находятся на claude.ai или в Cowork**: там загружается другой набор компонентов. См. [Plugins on claude.ai and in Cowork](https://claude.com/docs/plugins/overview)
</Note>

<h2 id="explore-the-plugin-directory">
  Изучите каталог плагинов
</h2>

Обозреватель показывает пример плагина `my-plugin`, который содержит по одному компоненту каждого вида в его расположении по умолчанию:

* Skill для проверки и команду `about`
* Subagent для проверки безопасности
* Hook, который форматирует файлы после редактирования Claude, и папку `scripts/`, которую он вызывает
* Монитор логов
* Стиль вывода и цветовую тему
* Workflow для аудита маршрутов
* Исполняемый файл `hello-plugin`
* Параметры по умолчанию
* Локальный MCP сервер и языковой сервер Go

Каждый файл — это наименьший допустимый пример своего формата, предназначенный для демонстрации структуры, а не для практического использования: реальный skill или agent содержит полные инструкции и часто вспомогательные файлы, а реальный hook или монитор выполняет реальную работу. Разделы после обозревателя используют те же файлы в качестве примеров и ссылаются на более полные версии. Выберите файл или папку, чтобы прочитать, для чего она нужна, увидеть, что в ней содержится, и найти раздел, который её описывает.

<PluginExplorer>
  <Piece id="manifest">
    [Манифест](/docs/ru/plugins/manifest-reference) — это файл `plugin.json` в директории `.claude-plugin/` плагина. Он содержит метаданные плагина и значения `userConfig`, которые Claude Code запрашивает у пользователя. Обязателен только `name`. В этом примере `description` — это текст, который пользователи видят для плагина в `/plugin`, а `version` удерживает пользователей на этой версии, пока вы её не измените:

    ```json theme={null}
    {
      "name": "my-plugin",
      "version": "1.0.0",
      "description": "Review, formatting, and database tools for this team"
    }
    ```
  </Piece>

  <Piece id="skills">
    [Skill](/docs/ru/skills) — это файл `SKILL.md`. Сохраняйте каждый skill в отдельной директории в папке `skills/`. Claude читает `description` каждого skill, и когда то, что просит пользователь, совпадает с ней, например, когда пользователь просит Claude проверить pull request, Claude загружает инструкции skill и следует им. Пользователь также может запустить его напрямую как `/my-plugin:review`:

    ```markdown theme={null}
    ---
    description: Reviews a pull request for style and test coverage. Use when asked to review code.
    ---

    Review the changed files. Report style problems first, then missing tests.
    ```
  </Piece>

  <Piece id="commands">
    Команда — это один файл Markdown, который пользователь запускает по имени. Команды — это более старый формат: skill запускается по имени таким же образом и может также содержать вспомогательные файлы в собственной директории, поэтому пишите новые как skills и сохраняйте `commands/` для файлов, которые у вас уже есть. Этот файл становится `/my-plugin:about` и принимает тот же frontmatter, что и skill:

    ```markdown theme={null}
    ---
    description: Summarize the repository
    ---

    Summarize what this repository does in three sentences.
    ```
  </Piece>

  <Piece id="agents">
    [Subagent](/docs/ru/sub-agents) — это отдельный помощник с собственными инструкциями и собственным контекстным окном, которому Claude может делегировать задачу и получить результат. Каждый файл Markdown в папке `agents/` определяет один: frontmatter называет его и говорит, когда его использовать, а тело — это его системный prompt. Этот назван `my-plugin:security-reviewer`, и пользователь может вызвать его с помощью `@agent-my-plugin:security-reviewer`:

    ```markdown theme={null}
    ---
    name: security-reviewer
    description: Reviews code changes for security issues. Use after edits to authentication or input handling.
    model: sonnet
    ---

    You are a security reviewer. Read the changed files and report injection, authentication, and secrets-handling risks.
    ```
  </Piece>

  <Piece id="hooks">
    [Hook](/docs/ru/hooks-guide) запускает что-то автоматически в точке жизненного цикла Claude Code, например, после каждого редактирования файла: команду shell, HTTP запрос, вызов инструмента MCP, prompt к модели или subagent. Сохраняйте hooks плагина в `hooks/hooks.json` в корне плагина. Этот запускает `scripts/format.sh` плагина после того, как Claude записывает или редактирует файл:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [
              {
                "type": "command",
                "command": "\"${CLAUDE_PLUGIN_ROOT}/scripts/format.sh\""
              }
            ]
          }
        ]
      }
    }
    ```
  </Piece>

  <Piece id="monitors">
    Монитор — это команда shell, которую Claude Code запускает в фоне при запуске сеанса и держит запущенной до его завершения, используя [инструмент Monitor](/docs/ru/tools-reference#monitor-tool). То, что он выводит, достигает Claude как уведомления. Поле `when` может вместо этого запустить его в первый раз, когда запустится названный skill. Этот отслеживает журнал ошибок:

    ```json theme={null}
    [
      {
        "name": "error-log",
        "command": "tail -F ./logs/error.log",
        "description": "Application error log"
      }
    ]
    ```
  </Piece>

  <Piece id="output-styles">
    Плагин может включать [стили вывода](/docs/ru/output-styles), которые изменяют, как Claude форматирует и формулирует свои ответы. Сохраняйте каждый стиль вывода как `output-styles/<name>.md`. Этот появляется в `/output-style` как `my-plugin:terse`:

    ```markdown theme={null}
    ---
    name: terse
    description: Answer in as few words as possible
    keep-coding-instructions: true
    ---

    Keep every reply short. Skip preambles and summaries.
    ```
  </Piece>

  <Piece id="themes">
    Плагин может включать [цветовые темы](/docs/ru/terminal-config#create-a-custom-theme) для интерфейса Claude Code. Сохраняйте каждую тему как `themes/<slug>.json`. Этот появляется в `/theme` как `Dracula`, отмеченный как из `my-plugin`:

    ```json theme={null}
    {
      "name": "Dracula",
      "base": "dark",
      "overrides": {
        "claude": "#bd93f9",
        "error": "#ff5555"
      }
    }
    ```
  </Piece>

  <Piece id="workflows">
    Папка `workflows/` содержит файлы [workflow](/docs/ru/workflows) `.js`: блок `meta`, затем тело скрипта, которое координирует несколько subagents. Этот запускается как `/my-plugin:audit-routes`:

    ```javascript theme={null}
    export const meta = {
      name: 'audit-routes',
      description: 'Audit every route handler for missing auth checks',
    }

    const found = await agent('List every .ts file under src/routes/.', {
      schema: { type: 'object', required: ['files'], properties: { files: { type: 'array', items: { type: 'string' } } } },
    })

    const audits = await pipeline(found.files, file =>
      agent(`Audit ${file} for missing authentication checks.`, { label: file }),
    )

    return audits.filter(Boolean)
    ```
  </Piece>

  <Piece id="bin">
    `bin/` — это способ, которым плагин поставляет инструмент командной строки. Пока плагин включен, Claude Code помещает эту папку в `PATH` shell, в котором он запускает команды, поэтому Claude или инструкции skill могут запустить инструмент по имени без установки пользователем. С этим [исполняемым файлом](#executables) на месте `hello-plugin` — это команда, которую Claude может запустить:

    ```bash theme={null}
    #!/bin/bash
    echo "hello from my-plugin"
    ```
  </Piece>

  <Piece id="scripts">
    Hook в `hooks/hooks.json` запускает скрипт, и эта папка — это место, где пример его хранит. Имя `scripts/` — это соглашение, а не что-то, что Claude Code ищет: hook указывает на файл по его пути, `${CLAUDE_PLUGIN_ROOT}/scripts/format.sh`. Скрипт форматирования может выглядеть так:

    ```bash theme={null}
    #!/bin/bash
    npx prettier --write .
    ```
  </Piece>

  <Piece id="settings">
    `settings.json` в корне плагина содержит [параметры](/docs/ru/settings-reference), которые применяются, пока плагин включен, поэтому плагин может изменить поведение сеанса и не только добавить компоненты. Только два ключа действуют из плагина, [`agent`](/docs/ru/settings-reference#agent) и [`subagentStatusLine`](/docs/ru/settings-reference#subagentstatusline); все остальные ключи отбрасываются. См. [Параметры по умолчанию](#default-settings).

    Этот устанавливает `agent`, который запускает основной поток сеанса как собственный agent `security-reviewer` плагина, поэтому системный prompt этого agent, ограничения инструментов и модель применяются ко всему сеансу:

    ```json theme={null}
    {
      "agent": "security-reviewer"
    }
    ```
  </Piece>

  <Piece id="mcp">
    [MCP сервер](/docs/ru/mcp) предоставляет Claude инструменты из внешней системы. Объявите его в `.mcp.json` в корне плагина. Этот запускает локальный сервер из скрипта внутри плагина и появляется в `/mcp` как `plugin:my-plugin:db`:

    ```json theme={null}
    {
      "mcpServers": {
        "db": {
          "command": "node",
          "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"]
        }
      }
    }
    ```
  </Piece>

  <Piece id="lsp">
    LSP сервер предоставляет Claude [диагностику и навигацию по коду](/docs/ru/plugins/code-intelligence) для языка. Объявите сервер в `.lsp.json` в корне плагина. Этот подключает языковой сервер Go для файлов `.go`:

    ```json theme={null}
    {
      "gopls": {
        "command": "gopls",
        "args": ["serve"],
        "extensionToLanguage": {
          ".go": "go"
        }
      }
    }
    ```
  </Piece>
</PluginExplorer>

<h2 id="add-each-kind-of-component">
  Добавление каждого вида компонента
</h2>

Каждый раздел ниже охватывает один вид компонента: где его файлы находятся в плагине, пример, который проходит валидацию, что видит пользователь после загрузки плагина, и ключ манифеста, который изменяет расположение по умолчанию. Добавляйте те, которые нужны вашему плагину; ни один не требуется.

<h3 id="skills">
  Skills
</h3>

[Skill](/docs/ru/skills) — это файл `SKILL.md`, который Claude может загрузить, когда его описание совпадает с задачей. Пользователь также может запустить его как команду. Сохраняйте каждый skill в его собственной директории под `skills/`:

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json
└── skills/
    └── review/
        └── SKILL.md
```

Дайте `SKILL.md` `description`, чтобы Claude знал, когда его использовать:

```markdown skills/review/SKILL.md theme={null}
---
description: Reviews a pull request for style and test coverage. Use when asked to review code.
---

Review the changed files. Report style problems first, then missing tests.
```

После загрузки плагина `/my-plugin:review` запускает skill. Имя команды и кто может её вызвать следуют этим правилам:

* **Имя команды**: `/<plugin>:<directory>`, поэтому `skills/review/SKILL.md` в `my-plugin` — это `/my-plugin:review`. Если вы установите `name` в frontmatter, он заменит последний сегмент, и префикс плагина остаётся. См. [как skill получает имя команды](/docs/ru/skills#how-a-skill-gets-its-command-name)
* **Кто вызывает**: Claude, пользователь или оба, контролируется frontmatter. См. [Control who invokes a skill](/docs/ru/skills#control-who-invokes-a-skill)

Вы также можете разместить skills вне директории по умолчанию `skills/`:

* **Дополнительные директории**: перечислите их в ключе манифеста `skills`. Они добавляются к сканированию по умолчанию `skills/`, а не заменяют его, в отличие от `commands` и `agents`
* **Один skill в корне плагина**: без директории `skills/` и без ключа манифеста `skills`, `SKILL.md` в корне плагина загружается как один skill. Установите `name` в его frontmatter, потому что иначе установка из marketplace назовёт skill по его [директории кэша](/docs/ru/plugins/loading#find-plugins-on-disk), а не по вашему плагину

Чтобы включить инструкции в плагин, напишите их как skill. Claude Code не загружает `CLAUDE.md` в корне плагина, и `claude plugin validate` предупреждает `CLAUDE.md at the plugin root is not loaded as project context`.

Для полей frontmatter и вспомогательных файлов см. [Skills](/docs/ru/skills).

<h3 id="commands">
  Команды
</h3>

Команда — это один файл Markdown, который пользователь запускает по имени, например `/my-plugin:about`.

<Note>
  Команды — это более старый формат, и [skills](#skills) их заменяют для новой работы. Skill запускается по имени таким же образом, и он также может содержать вспомогательные файлы в своей директории. Сохраняйте `commands/` для файлов, которые вы переносите из `.claude/commands/`.
</Note>

Сохраняйте команду в `commands/<file>.md` и она становится `/<plugin>:<file>`. Подпапка добавляет сегмент, поэтому `commands/db/migrate.md` — это `/my-plugin:db:migrate`.

Файлы команд принимают тот же frontmatter, что и skills.

<h4 id="define-commands-in-the-manifest">
  Определение команд в манифесте
</h4>

Это нужно только, если вы хотите сохранить файлы команд где-то в другом месте, чем `commands/`, или определить короткую команду внутри `plugin.json` без отдельного файла Markdown. Установите ключ манифеста `commands`, и Claude Code читает его вместо сканирования `commands/`. Ключ принимает путь, массив путей или объект, который отображает каждое имя команды либо на файл `source`, либо на встроенное `content`.

Этот манифест определяет `/my-plugin:about` встроенным образом, без файла Markdown:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "my-plugin",
  "commands": {
    "about": {
      "content": "Summarize what this repository does in three sentences.",
      "description": "Summarize the repository"
    }
  }
}
```

Загрузите плагин и запустите `/my-plugin:about` в сессии, чтобы подтвердить, что он загрузился.

Для полного синтаксиса ключа см. [`commands`](/docs/ru/plugins/manifest-reference#commands).

<h3 id="agents">
  Агенты
</h3>

[Подагент](/docs/ru/sub-agents) — это отдельный помощник с собственными инструкциями и окном контекста, которому Claude может делегировать задачу. Каждый файл Markdown под `agents/` определяет один:

```markdown agents/security-reviewer.md theme={null}
---
name: security-reviewer
description: Reviews code changes for security issues. Use after edits to authentication or input handling.
model: sonnet
---

You are a security reviewer. Read the changed files and report injection, authentication, and secrets-handling risks.
```

Этот агент назван `my-plugin:security-reviewer`, и пользователь может [вызвать его явно](/docs/ru/sub-agents#invoke-subagents-explicitly) с помощью `@agent-my-plugin:security-reviewer`. Форма имени — `<plugin>:<name>`, где `<name>` берётся из frontmatter или из имени файла, когда его нет.

Ключ `agents` манифеста заменяет сканирование `agents/`.

<h4 id="organize-agents-in-subfolders">
  Организация агентов в подпапках
</h4>

Вы можете поместить файлы агентов плагина в подпапки `agents/`. Claude Code [загружает их рекурсивно](/docs/ru/sub-agents#choose-the-subagent-scope) и объединяет имя плагина, каждое имя подпапки и имя файла с двоеточиями, чтобы сформировать имя агента с областью видимости. Например, `agents/review/security.md` в плагине с именем `my-plugin` загружается как `my-plugin:review:security`. Два параметра изменяют это имя:

* Frontmatter `name`: он заменяет только имя файла, поэтому `name: audit` в `agents/review/security.md` загружается как `my-plugin:review:audit`
* Манифест [`agents`](/docs/ru/plugins/manifest-reference#fields) поле: файл, который вы там перечислите, загружается без имён подпапок, поэтому `"agents": "./custom/review/security.md"` загружается как `my-plugin:security`

<h4 id="frontmatter-fields-in-plugin-agents">
  Поля frontmatter в агентах плагина
</h4>

Frontmatter агента плагина следует этим правилам:

* **Поддерживаемые поля**: `name`, `description`, `model`, `effort`, `maxTurns`, `tools`, `disallowedTools`, `skills`, `memory`, `background`, `omitClaudeMd`, `isolation`, `color` и ключ `cacheTtl` из `experimental`. Единственное допустимое значение `isolation` — это `"worktree"`. См. [поддерживаемые поля frontmatter](/docs/ru/sub-agents#supported-frontmatter-fields) для того, что делает каждое
* **Игнорируемые поля**: `permissionMode`, `hooks`, `mcpServers` и `initialPrompt`. Файл агента не может добавлять hooks или MCP серверы самостоятельно, поэтому добавляйте их как плагин [hooks](#hooks) и [MCP серверы](#mcp-servers) вместо этого
* **Frontmatter, который не парсится**: агент всё ещё загружается со всеми полями, игнорируемыми. Он назван по имени файла, и его описание читается как `Agent from my-plugin plugin`. Запустите [`claude plugin validate`](/docs/ru/plugins/cli-reference#plugin-validate) в вашей оболочке, чтобы найти эти файлы

Для того, что делает каждое поле и правила приоритета, см. [Subagents](/docs/ru/sub-agents#supported-frontmatter-fields).

<h3 id="hooks">
  Hooks
</h3>

[Hook](/docs/ru/hooks-guide) запускает что-то автоматически в точке жизненного цикла Claude Code, например, после каждого редактирования файла: команду оболочки, HTTP запрос, вызов инструмента MCP, prompt к модели или подагента. Сохраняйте hooks плагина в `hooks/hooks.json` в корне плагина, под верхним уровнем ключа `"hooks"`, в той же форме, что и объект `hooks` в `settings.json`. Это позволяет вам скопировать существующий hook параметров без изменений.

Этот hook запускает встроенный скрипт после каждого `Write` или `Edit`:

```json hooks/hooks.json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}/scripts/format.sh\""
          }
        ]
      }
    ]
  }
}
```

Сохраняйте скрипт в `scripts/format.sh` и сделайте его исполняемым.

Загрузите плагин и попросите Claude отредактировать файл. Hook `PostToolUse`, который выходит с кодом 0, ничего не показывает в транскрипте, поэтому подтвердите, что он запустился с помощью [debug logging](/docs/ru/hooks#debug-hooks) или по тому, что сам скрипт изменяет.

Hooks в `hooks/hooks.json` и в ключе манифеста `hooks` оба загружаются. Для каждого события и его payload см. [Hook events](/docs/ru/hooks#hook-events).

<h4 id="when-plugin-hooks-fire">
  Когда запускаются hooks плагина
</h4>

Hooks плагина не ждут использования одного из skills или команд плагина. Claude Code регистрирует их, когда сессия загружает плагин, и они запускаются на своих событиях с этого момента. Чтобы ограничить, когда запускается hook, сузьте его `matcher`.

Если hook никогда не запускается, см. [hooks that don't fire](/docs/ru/plugins/troubleshooting#failed-to-load-hooks-from-and-hooks-that-dont-fire).

<h4 id="environment-quoting-and-matching-mcp-tools">
  Окружение, кавычки и соответствие инструментам MCP
</h4>

Окружение hook, кавычки `${CLAUDE_PLUGIN_ROOT}` и matchers для собственных инструментов MCP плагина работают следующим образом:

* **Окружение**: каждый процесс hook получает `CLAUDE_PLUGIN_ROOT` и `CLAUDE_PLUGIN_DATA` в своём окружении, плюс `CLAUDE_PLUGIN_OPTION_<KEY>` для каждого значения [конфигурации пользователя](#user-configuration), поэтому ваш скрипт может читать их оттуда
* **Кавычки**: когда `command` не имеет `args`, он запускается через оболочку, поэтому оберните путь `${CLAUDE_PLUGIN_ROOT}` в двойные кавычки, как пример `hooks/hooks.json` под [Hooks](#hooks) делает, чтобы сохранить развёрнутый путь одним словом оболочки. Когда вы передаёте `args` вместо этого, каждый элемент передаётся как один аргумент без оболочки и не нуждается в кавычках. См. [exec form and shell form](/docs/ru/hooks#exec-form-and-shell-form)
* **Соответствие собственным инструментам MCP плагина**: инструмент из [MCP сервера, который объявляет этот плагин](#mcp-servers), назван `mcp__plugin_<plugin>_<server>__<tool>`, поэтому напишите это полное имя в matcher. Matcher только на имя сервера никогда не запускается. См. [Match MCP tools](/docs/ru/hooks#match-mcp-tools)

<h3 id="mcp-servers">
  MCP серверы
</h3>

MCP сервер предоставляет Claude инструменты из внешней системы. Объявите его в `.mcp.json` в корне плагина, в той же форме, что и [проект `.mcp.json`](/docs/ru/mcp#project-scope). Этот `.mcp.json` объявляет один сервер с именем `db`:

```json .mcp.json theme={null}
{
  "mcpServers": {
    "db": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"]
    }
  }
}
```

Вы также можете опустить обёртку `mcpServers` и поместить `db` на верхний уровень файла.

Загрузите плагин и запустите `/mcp`, чтобы подтвердить, что сервер появляется как `plugin:my-plugin:db`.

`claude plugin validate` проверяет `.mcp.json` и сообщает запись сервера, которую Claude Code отбросит во время загрузки, как ошибку. Требуется Claude Code v2.1.281 или позже.

Для того, где плохая запись появляется во время загрузки, см. [MCP servers that don't start](/docs/ru/plugins/troubleshooting#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start).

Ключ манифеста `mcpServers` принимает встроенную карту сервера, путь к файлу JSON или массив этих. Когда сервер манифеста имеет то же имя, что и один в `.mcp.json`, сервер манифеста заменяет его.

<h4 id="reach-users-on-claude-ai-and-cowork">
  Достижение пользователей на claude.ai и Cowork
</h4>

Локальный сервер stdio, такой как сервер `db` под [MCP servers](#mcp-servers), работает в Claude Code и в сессии Cowork, которая работает на вашей машине в приложении Claude Desktop, но не на claude.ai. Чтобы достичь пользователей там тоже, ссылайтесь на удалённый сервер по его URL `https://`, который claude.ai и Cowork предлагают пользователю как соединитель.

<h4 id="server-names-tool-names-and-reloads">
  Имена серверов, имена инструментов и перезагрузки
</h4>

Имена сервера, подстановка переменных и поведение перезагрузки следуют этим правилам:

* **Имя сервера**: `plugin:<plugin>:<server>`, поэтому сервер `db` в `my-plugin` — это `plugin:my-plugin:db` в `/mcp`. Используйте ту же форму для именования сервера в [hook `mcp_tool`](/docs/ru/hooks#mcp-tool-hook-fields)
* **Имена инструментов**: `mcp__plugin_<plugin>_<server>__<tool>`, поэтому инструмент `query` на том сервере `db` — это `mcp__plugin_my-plugin_db__query`. Это имя, которое нужно использовать в [правилах разрешений](/docs/ru/permissions) и [matchers hook](#hooks)
* **Подстановка**: `${CLAUDE_PLUGIN_ROOT}` и другие [переменные пути](#path-variables-and-persistent-data) подставляются в `command`, `args` и `env`. Кавычки не нужны в `args`, потому что каждый элемент передаётся как один аргумент
* **Перезагрузка**: когда пользователь запускает `/reload-plugins` и [перезагрузка применяется](/docs/ru/plugins/cli-reference#reloads-that-change-mcp-tools), сервер, конфигурация которого не изменилась, сохраняет своё соединение. Сервер, конфигурация которого изменилась, переподключается, и тот, который вы удалили, отключается

<h4 id="include-a-packaged-mcpb-server">
  Включение упакованного MCPB сервера
</h4>

Ключ `mcpServers` также принимает упакованный сервер как [файл MCPB](https://github.com/modelcontextprotocol/mcpb), расширение которого `.mcpb` или более старое `.dxt`. Укажите ключ на файл, как путь внутри плагина или URL `https://`:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "my-plugin",
  "mcpServers": "./servers/db.mcpb"
}
```

Сервер берёт своё имя из `name` в манифесте пакета.

Для транспортов и аутентификации см. [MCP](/docs/ru/mcp#plugin-provided-mcp-servers).

<h3 id="lsp-servers">
  LSP серверы
</h3>

LSP сервер предоставляет Claude диагностику и навигацию по коду для языка. Если [официальный плагин code intelligence](/docs/ru/plugins/code-intelligence) уже охватывает ваш язык, установите его вместо написания собственного. Иначе объявите сервер в `.lsp.json` в корне плагина:

```json .lsp.json theme={null}
{
  "gopls": {
    "command": "gopls",
    "args": ["serve"],
    "extensionToLanguage": {
      ".go": "go"
    }
  }
}
```

Файл отображает каждое имя сервера непосредственно на его конфигурацию, без объекта-обёртки вокруг карты. `command` — это имя двоичного файла, с его аргументами в `args`. `extensionToLanguage` нуждается по крайней мере в одном расширении, каждое начинается с `.`.

`claude plugin validate` не читает этот файл. Когда любая запись недействительна, весь файл пропускается при загрузке и `Invalid LSP server config for ".lsp.json"` появляется на вкладке **Errors** в `/plugin`.

Ваш плагин конфигурирует соединение, но не устанавливает двоичный файл сервера, и каждое расширение файла получает один сервер:

* **Отсутствующий двоичный файл**: Claude Code запускает `command` по имени из `PATH` пользователя. Когда двоичный файл там не находится, сервер не запускается и `claude --debug` логирует `LSP server <name> failed to start`
* **Конфликты расширений**: когда два включённых сервера претендуют на одно и то же расширение, первый зарегистрированный обрабатывает эти файлы, а другой не используется для них, независимо от того, поступают ли серверы из одного плагина или двух. Вкладка **Errors** в `/plugin` показывает предупреждение `LSP server "<name>" is not used for <ext> files`

Ключ манифеста `lspServers` принимает ту же карту встроенной, путь к файлу JSON или массив этих, и его серверы добавляются к тем, что в `.lsp.json`. Когда сервер манифеста имеет то же имя, что и один в `.lsp.json`, сервер манифеста заменяет его.

Для `transport`, timeouts, restarts и других полей см. [`lspServers`](/docs/ru/plugins/manifest-reference#lspservers).

Отправляйте вывод логов в stderr, а не stdout. Claude Code читает stdout сервера только как сообщения протокола и принимает заголовки сообщений до 64 КиБ и тело сообщения до 32 МиБ.

Claude Code отключает сервер, который превышает любой лимит или пишет вывод, не являющийся протоколом, в stdout, и считает отключение сбоем для `restartOnCrash` и `maxRestarts`. Когда вы запускаете с `--debug`, Claude Code пишет ошибку, называющую причину, в журнал отладки.

<h3 id="executables">
  Исполняемые файлы
</h3>

Файлы в `bin/` в корне плагина находятся на `PATH` оболочки инструмента Bash, пока плагин включен, поэтому Claude может запустить их как простые команды. Добавьте исполняемый скрипт:

```bash bin/hello-plugin theme={null}
#!/bin/bash
echo "hello from my-plugin"
```

Сделайте его исполняемым с помощью `chmod +x bin/hello-plugin` и загрузите плагин. Когда вы просите Claude запустить `hello-plugin`, результат инструмента Bash показывает вывод скрипта.

Директории `bin/` плагина идут после собственных записей `PATH` пользователя, поэтому плагин не может затенять `git`, `ls` или другую системную команду.

claude.ai и Cowork не устанавливают плагин, который имеет директорию `bin/` верхнего уровня, включая тот, который вы [распространяете через параметры организации claude.ai](/docs/ru/plugins/host-marketplace#distribute-through-organization-settings).

<h3 id="default-settings">
  Параметры по умолчанию
</h3>

Чтобы установить значения по умолчанию, которые применяются, пока плагин включен, добавьте `settings.json` в корень плагина или поместите тот же объект встроенным в ключ манифеста `settings`. Два ключа вступают в силу, `agent` и `subagentStatusLine`, и все остальные ключи отбрасываются.

Установите `agent` для запуска одного из собственных агентов плагина как основного потока:

```json settings.json theme={null}
{
  "agent": "security-reviewer"
}
```

Загрузите плагин и начните сессию. Claude затем отвечает в основном разговоре с системным prompt и моделью агента `security-reviewer`.

Для всего, что контролирует ключ, см. [параметр `agent`](/docs/ru/settings-reference#agent).

Когда один и тот же ключ установлен в более чем одном месте, эти правила решают, какое значение применяется:

* **Файл над манифестом**: когда оба существуют и `settings.json` устанавливает по крайней мере один поддерживаемый ключ, `settings.json` применяется и `settings` манифеста игнорируется
* **Параметры пользователя над значениями по умолчанию плагина**: во всех источниках параметров значения по умолчанию плагина — это самый низкий уровень, поэтому собственный `agent` пользователя в `~/.claude/settings.json` переопределяет ваш
* **Два плагина устанавливают один и тот же ключ**: значение из плагина, загруженного последним, применяется, и `claude --debug` логирует `overrides setting`

Для формы `subagentStatusLine` см. [subagent status lines](/docs/ru/statusline#subagent-status-lines).

<h3 id="themes-and-output-styles">
  Темы и стили вывода
</h3>

Плагин может включать цветовые темы и стили вывода. Оба появляются в тех же выборщиках, что и собственные пользователя. Для любого из них установка ключа манифеста заменяет сканирование папки.

| Компонент    | Сохраняйте как            | Формат                                                                                                                             | Появляется в                           | Ключ манифеста        |
| :----------- | :------------------------ | :--------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------- | :-------------------- |
| Тема         | `themes/<slug>.json`      | Формат [пользовательского файла темы](/docs/ru/terminal-config#create-a-custom-theme), который пользователи пишут в `~/.claude/themes/` | `/theme`, под `name` файла             | `experimental.themes` |
| Стиль вывода | `output-styles/<name>.md` | Формат [пользовательского стиля вывода](/docs/ru/output-styles#create-a-custom-output-style), с frontmatter `name` и `description`      | `/output-style`, как `<plugin>:<name>` | `outputStyles`        |

Темы плагина доступны только для чтения, поэтому когда пользователь редактирует одну в `/theme`, редактирование сохраняется как копия в их собственной директории тем.

Эта тема перекрашивает акцент prompt и текст ошибки на тёмном предустановке:

```json themes/dracula.json theme={null}
{
  "name": "Dracula",
  "base": "dark",
  "overrides": {
    "claude": "#bd93f9",
    "error": "#ff5555"
  }
}
```

<h3 id="channels">
  Каналы
</h3>

[Канал](/docs/ru/channels) позволяет внешней системе, такой как приложение чата, отправлять сообщения в сессию. В плагине канал — это один из MCP серверов плюс запись `channels`, которая привязывает к нему и может запросить собственную конфигурацию. Этот манифест привязывает канал к серверу `telegram` и запрашивает токен бота:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "my-plugin",
  "mcpServers": {
    "telegram": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"],
      "env": { "BOT_TOKEN": "${user_config.bot_token}" }
    }
  },
  "channels": [
    {
      "server": "telegram",
      "userConfig": {
        "bot_token": {
          "type": "string",
          "title": "Bot token",
          "description": "Telegram bot token",
          "sensitive": true
        }
      }
    }
  ]
}
```

`server` должен совпадать с ключом в `mcpServers`. Per-channel `userConfig` принимает ту же форму, что и [верхний уровень ключа `userConfig`](#user-configuration).

Для того, что должен реализовать сервер и как пользователи включают плагин канала, см. [Package as a plugin](/docs/ru/channels-reference#package-as-a-plugin) в справочнике каналов. Для таблицы полей см. [`channels`](/docs/ru/plugins/manifest-reference#channels).

<h3 id="monitors">
  Мониторы
</h3>

Монитор — это команда оболочки, которая работает в фоне для всей сессии. То, что она выводит, достигает Claude как уведомления, поэтому Claude может реагировать на журнал или изменение статуса без просьбы наблюдать за ним. Сохраняйте записи в `monitors/monitors.json`:

```json monitors/monitors.json theme={null}
[
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Application error log"
  }
]
```

Команда запускается в оболочке, в рабочей директории, в которой сессия началась.

Команда монитора ограничена в том, где она запускается и на что может ссылаться:

* **Только интерактивные сессии**: мониторы плагина запускаются в интерактивной сессии и никогда в неинтерактивном режиме с флагом `-p`. Они также запускаются только там, где доступен [инструмент Monitor](/docs/ru/tools-reference#monitor-tool)
* **Нет конфигурации пользователя**: `command` получает [переменные пути](#path-variables-and-persistent-data) и `${ENV_VAR}` из окружения, но никогда `${user_config.*}`. Монитор, который ссылается на один, не запускается, и процессы монитора также не получают `CLAUDE_PLUGIN_OPTION_<KEY>`
* **Отключение во время сессии**: если вы отключите плагин во время сессии, Claude Code не останавливает мониторы, которые уже работают. Они останавливаются, когда сессия заканчивается

Ключ манифеста `experimental.monitors` принимает тот же массив встроенным или путь к файлу JSON и читается вместо `monitors/monitors.json`.

Для триггера `when` и других полей см. [`monitors`](/docs/ru/plugins/manifest-reference#monitors).

<h2 id="user-configuration">
  Запрос значений конфигурации у пользователя
</h2>

Объявите значения, которые ваш плагин нужен от пользователя, в ключе манифеста `userConfig`, чтобы пользователи не редактировали `settings.json` сами. Каждый вариант появляется в диалоге с его `title` как метка и его `description` под ней.

Установите `"sensitive": true` для токена или пароля. Диалог затем маскирует ввод, и значение хранится в защищённом хранилище, а не в `settings.json`.

Этот манифест запрашивает конечную точку и токен:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "my-plugin",
  "userConfig": {
    "api_url": {
      "type": "string",
      "title": "API URL",
      "description": "Base URL of your team's API"
    },
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "Token for your team's API",
      "sensitive": true
    }
  }
}
```

<h3 id="when-the-configuration-dialog-appears">
  Когда появляется диалог конфигурации
</h3>

Диалог появляется только в интерактивном интерфейсе `/plugin`. Он открывается для любого варианта, который ещё не установлен, когда пользователь делает любое из следующего:

* Устанавливает плагин в `/plugin`
* Запускает `/plugin install <plugin>@<marketplace>` внутри сессии
* Включает плагин из вкладки **Installed** в `/plugin`

Чтобы открыть тот же диалог в любое время, пользователь запускает `/plugin configure <plugin>@<marketplace>`.

Команда оболочки `claude plugin install` никогда не запрашивает значения `userConfig`. Чтобы установить значения из оболочки, передайте каждое как `--config KEY=VALUE`. Когда варианты остаются неустановленными, команда выводит строку `userConfig options not yet set`, которая называет оба способа их установки. [The `userConfig` dialog never appears](/docs/ru/plugins/troubleshooting#the-userconfig-dialog-never-appears) цитирует строку.

Для полей варианта, где хранится каждое значение, как компонент ссылается на сохранённое значение и какие поля отклоняют `${user_config.*}`, см. [User configuration](/docs/ru/plugins/manifest-reference#user-configuration).

<h2 id="path-variables-and-persistent-data">
  Ссылка на пути плагина и хранение данных
</h2>

Вы не знаете, где будет установлен ваш плагин, поэтому ссылайтесь на его файлы и данные через эти переменные, а не через фиксированные пути. Они подставляются в содержимое skill, команды и агента, в команды hook и монитора, а также в конфигурации MCP и LSP сервера. Они также экспортируются в процессы hook, MCP и LSP:

* **`${CLAUDE_PLUGIN_ROOT}`**: директория установки плагина. Каждая версия имеет свою собственную [директорию кэша](/docs/ru/plugins/loading#find-plugins-on-disk), поэтому путь изменяется при обновлении плагина. Не пишите состояние там
* **`${CLAUDE_PLUGIN_DATA}`**: директория, которая выживает обновления, для `node_modules`, виртуальных окружений и кэшей. Она разрешается в `~/.claude/plugins/data/<id>/` и создаётся при первой ссылке
* **`${CLAUDE_PROJECT_DIR}`**: корень проекта, то же значение, которое получают hooks

В пути директории данных `<id>` — это идентификатор плагина со всеми символами, кроме букв, цифр, `_` и `-`, заменённых на `-`, поэтому `my-plugin@my-marketplace` становится `my-plugin-my-marketplace`.

На Windows подставленные пути используют прямые слэши, поэтому оболочка не читает обратные слэши как экранирование.

<h3 id="install-dependencies-into-the-data-directory">
  Установка зависимостей в директорию данных
</h3>

Для плагина, установленного из marketplace, Claude Code автоматически устанавливает подходящие [зависимости пакета Node.js](/docs/ru/plugins/loading#node-js-package-dependencies) при кэшировании плагина, поэтому вам может не потребоваться устанавливать их самостоятельно. Когда вам нужно, этот hook `SessionStart` устанавливает `node_modules` в `${CLAUDE_PLUGIN_DATA}` при первом запуске и снова после обновления, которое изменяет `package.json`:

```json hooks/hooks.json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "diff -q \"${CLAUDE_PLUGIN_ROOT}/package.json\" \"${CLAUDE_PLUGIN_DATA}/package.json\" >/dev/null 2>&1 || (cd \"${CLAUDE_PLUGIN_DATA}\" && cp \"${CLAUDE_PLUGIN_ROOT}/package.json\" . && npm install) || rm -f \"${CLAUDE_PLUGIN_DATA}/package.json\""
          }
        ]
      }
    ]
  }
}
```

После первой сессии `~/.claude/plugins/data/<id>/node_modules` существует. MCP сервер может затем установить `NODE_PATH` в `${CLAUDE_PLUGIN_DATA}/node_modules` в его `env`. Для того, какие поля подставляют какую переменную, см. [Environment variables](/docs/ru/plugins/manifest-reference#environment-variables).

<h2 id="next-steps">
  Следующие шаги
</h2>

* [Plugin manifest reference](/docs/ru/plugins/manifest-reference): поля `plugin.json`, правила пути и стандартная раскладка
* [Test plugins with evals](/docs/ru/plugin-evals): проверьте, что компоненты, которые вы добавили, изменяют поведение Claude так, как вы намеревались
* [Publish and distribute a plugin](/docs/ru/plugins/publish): версионируйте плагин и поместите его в marketplace
* [Troubleshoot plugins](/docs/ru/plugins/troubleshooting): что делать, когда компонент не загружается или hook не запускается
