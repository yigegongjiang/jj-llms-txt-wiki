> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Komponenten zu einem Plugin hinzufügen

> Fügen Sie Skills, Hooks, MCP-Server und alle anderen Komponententypen zu einem Claude Code-Plugin hinzu, mit einem Beispiel, das für jeden validiert.

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

Ein Claude Code-Plugin wird aus Komponenten erstellt, wie Skills, Agents, Hooks und MCP-Servern. Jede Komponente hat einen Standard-Ordner im Plugin, einen optionalen Manifest-Schlüssel in `.claude-plugin/plugin.json`, der diesen Ordner ersetzt oder ergänzt, und einen Namen, den der Benutzer sieht. Für jede Schlüssels vollständige Feldtabelle siehe die [Manifest-Referenz](/docs/de/plugins/manifest-reference#fields).

Verwenden Sie diese Seite, um eine Komponente zu einem Plugin hinzuzufügen, das bereits geladen wird.

Nachdem Sie eine Komponente hinzugefügt haben, führen Sie `/reload-plugins` in einer laufenden Sitzung aus oder starten Sie eine neue, damit Claude Code sie lädt. Um die Datei der Komponente vor dem Laden zu überprüfen, führen Sie [`claude plugin validate .`](/docs/de/plugins/cli-reference#plugin-validate) in Ihrer Shell aus dem Plugin-Verzeichnis aus.

<Note>
  Diese Fälle werden auf anderen Seiten behandelt:

  * **Ihr erstes Plugin erstellen**: Beginnen Sie mit [Plugin erstellen](/docs/de/plugins/create)
  * **Plugin von jemand anderem installieren**: Siehe [Plugins installieren](/docs/de/plugins/install)
  * **Ihre Plugin-Benutzer sind auf claude.ai oder in Cowork**: Ein anderer Satz von Komponenten wird dort geladen. Siehe [Plugins auf claude.ai und in Cowork](https://claude.com/docs/plugins/overview)
</Note>

<h2 id="explore-the-plugin-directory">
  Plugin-Verzeichnis erkunden
</h2>

Der Explorer zeigt ein Beispiel-Plugin, `my-plugin`, das an seinem Standard-Speicherort eine von jeder Art von Komponente hat:

* Ein Review-Skill und einen `about`-Befehl
* Einen Security-Review-Subagenten
* Einen Hook, der Dateien nach Claude-Bearbeitungen formatiert, und den `scripts/`-Ordner, den er aufruft
* Einen Log-Monitor
* Einen Output-Stil und ein Farbschema
* Einen Route-Audit-Workflow
* Eine `hello-plugin`-Ausführungsdatei
* Standard-Einstellungen
* Einen lokalen MCP-Server und einen Go-Sprachserver

Jede Datei ist das kleinste gültige Beispiel ihres Formats, um die Form zu zeigen, nicht um nützlich zu sein: Ein echter Skill oder Agent trägt vollständige Anweisungen und oft unterstützende Dateien, und ein echter Hook oder Monitor führt echte Arbeit aus. Die Abschnitte nach dem Explorer verwenden die gleichen Dateien als ihre Beispiele und verlinken auf vollständigere. Wählen Sie eine Datei oder einen Ordner aus, um zu lesen, wofür sie gedacht ist, zu sehen, was darin geht, und den Abschnitt zu finden, der sie behandelt.

<PluginExplorer>
  <Piece id="manifest">
    Das [Manifest](/docs/de/plugins/manifest-reference) ist die `plugin.json`-Datei im `.claude-plugin/`-Verzeichnis eines Plugins. Sie enthält die Metadaten des Plugins und die `userConfig`-Werte, die Claude Code den Benutzer fragt. Nur `name` ist erforderlich. In diesem Fall ist `description` der Text, den Benutzer für das Plugin in `/plugin` sehen, und `version` hält Benutzer auf dieser Version, bis Sie sie ändern:

    ```json theme={null}
    {
      "name": "my-plugin",
      "version": "1.0.0",
      "description": "Review, formatting, and database tools for this team"
    }
    ```
  </Piece>

  <Piece id="skills">
    Ein [Skill](/docs/de/skills) ist eine `SKILL.md`-Datei. Speichern Sie jeden Skill in seinem eigenen Verzeichnis unter `skills/`. Claude liest die `description` jedes Skills, und wenn das, was der Benutzer fragt, damit übereinstimmt, wie zum Beispiel Claude zu bitten, einen Pull Request zu überprüfen, lädt Claude die Anweisungen des Skills und folgt ihnen. Der Benutzer kann ihn auch direkt als `/my-plugin:review` ausführen:

    ```markdown theme={null}
    ---
    description: Reviews a pull request for style and test coverage. Use when asked to review code.
    ---

    Review the changed files. Report style problems first, then missing tests.
    ```
  </Piece>

  <Piece id="commands">
    Ein Befehl ist eine einzelne Markdown-Datei, die der Benutzer nach Name ausführt. Befehle sind das ältere Format: Ein Skill wird auf die gleiche Weise nach Name ausgeführt und kann auch unterstützende Dateien in seinem eigenen Verzeichnis tragen, daher schreiben Sie neue als Skills und behalten Sie `commands/` für Dateien, die Sie bereits haben. Diese Datei wird zu `/my-plugin:about` und nimmt die gleiche Frontmatter wie ein Skill:

    ```markdown theme={null}
    ---
    description: Summarize the repository
    ---

    Summarize what this repository does in three sentences.
    ```
  </Piece>

  <Piece id="agents">
    Ein [Subagent](/docs/de/sub-agents) ist ein separater Assistent mit seinen eigenen Anweisungen und seinem eigenen Kontextfenster, dem Claude eine Aufgabe delegieren und ein Ergebnis zurückbekommen kann. Jede Markdown-Datei unter `agents/` definiert einen: Die Frontmatter benennt ihn und sagt, wann er zu verwenden ist, und der Text ist sein System-Prompt. Dieser wird `my-plugin:security-reviewer` genannt, und der Benutzer kann ihn mit `@agent-my-plugin:security-reviewer` aufrufen:

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
    Ein [Hook](/docs/de/hooks-guide) führt etwas automatisch an einem Punkt im Lebenszyklus von Claude Code aus, wie zum Beispiel nach jeder Dateibearbeitung: ein Shell-Befehl, eine HTTP-Anfrage, ein MCP-Tool-Aufruf, ein Prompt an ein Modell oder ein Subagent. Speichern Sie die Hooks des Plugins in `hooks/hooks.json` im Plugin-Root. Dieser führt das `scripts/format.sh` des Plugins nach jedem Write oder Edit aus:

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
    Ein Monitor ist ein Shell-Befehl, den Claude Code im Hintergrund startet, wenn die Sitzung startet, und der läuft, bis sie endet, unter Verwendung des [Monitor-Tools](/docs/de/tools-reference#monitor-tool). Was er ausgibt, erreicht Claude als Benachrichtigungen. Ein `when`-Feld kann ihn stattdessen starten, wenn ein benannter Skill zum ersten Mal ausgeführt wird. Dieser verfolgt ein Fehlerprotokoll:

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
    Ein Plugin kann [Output-Stile](/docs/de/output-styles) enthalten, die ändern, wie Claude seine Antworten formatiert und formuliert. Speichern Sie jeden Output-Stil als `output-styles/<name>.md`. Dieser erscheint in `/output-style` als `my-plugin:terse`:

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
    Ein Plugin kann [Farbschemas](/docs/de/terminal-config#create-a-custom-theme) für die Claude Code-Schnittstelle enthalten. Speichern Sie jedes Schema als `themes/<slug>.json`. Dieses erscheint in `/theme` als `Dracula`, markiert als von `my-plugin`:

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
    Der `workflows/`-Ordner enthält [Workflow](/docs/de/workflows) `.js`-Dateien: einen `meta`-Block, dann einen Script-Text, der mehrere Subagenten orchestriert. Dieser läuft als `/my-plugin:audit-routes`:

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
    `bin/` ist, wie ein Plugin ein Befehlszeilentool versendet. Während das Plugin aktiviert ist, setzt Claude Code diesen Ordner auf den `PATH` der Shell, in der es Befehle ausführt, damit Claude oder die Anweisungen eines Skills das Tool nach Name ausführen können, ohne dass der Benutzer etwas installieren muss. Mit dieser [ausführbaren Datei](#executables) an Ort und Stelle ist `hello-plugin` ein Befehl, den Claude ausführen kann:

    ```bash theme={null}
    #!/bin/bash
    echo "hello from my-plugin"
    ```
  </Piece>

  <Piece id="scripts">
    Der Hook in `hooks/hooks.json` führt ein Script aus, und dieser Ordner ist, wo das Beispiel es behält. Der Name `scripts/` ist eine Konvention, nicht etwas, das Claude Code sucht: Der Hook zeigt auf die Datei nach ihrem Pfad, `${CLAUDE_PLUGIN_ROOT}/scripts/format.sh`. Ein Formatter-Script könnte so aussehen:

    ```bash theme={null}
    #!/bin/bash
    npx prettier --write .
    ```
  </Piece>

  <Piece id="settings">
    Eine `settings.json` im Plugin-Root hält [Einstellungen](/docs/de/settings-reference), die gelten, während das Plugin aktiviert ist, damit ein Plugin ändern kann, wie sich die Sitzung verhält, und nicht nur Komponenten hinzufügt. Nur zwei Schlüssel wirken sich von einem Plugin aus, [`agent`](/docs/de/settings-reference#agent) und [`subagentStatusLine`](/docs/de/settings-reference#subagentstatusline); jeder andere Schlüssel wird gelöscht. Siehe [Standard-Einstellungen](#default-settings).

    Dieser setzt `agent`, der die Haupt-Thread-Sitzung als den eigenen `security-reviewer`-Agent des Plugins ausführt, damit der System-Prompt, die Tool-Einschränkungen und das Modell dieses Agenten auf die ganze Sitzung angewendet werden:

    ```json theme={null}
    {
      "agent": "security-reviewer"
    }
    ```
  </Piece>

  <Piece id="mcp">
    Ein [MCP-Server](/docs/de/mcp) gibt Claude Tools von einem externen System. Deklarieren Sie ihn in `.mcp.json` im Plugin-Root. Dieser startet einen lokalen Server aus einem Script im Plugin und erscheint in `/mcp` als `plugin:my-plugin:db`:

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
    Ein LSP-Server gibt Claude [Diagnostik und Code-Navigation](/docs/de/plugins/code-intelligence) für eine Sprache. Deklarieren Sie den Server in `.lsp.json` im Plugin-Root. Dieser verbindet den Go-Sprachserver für `.go`-Dateien:

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
  Fügen Sie jede Art von Komponente hinzu
</h2>

Jeder Abschnitt unten behandelt eine Art von Komponente: wo ihre Dateien im Plugin gehen, ein Beispiel, das validiert, was der Benutzer sieht, sobald das Plugin geladen wird, und der Manifest-Schlüssel, der den Standard-Speicherort ändert. Fügen Sie die hinzu, die Ihr Plugin benötigt; keine ist erforderlich.

<h3 id="skills">
  Skills
</h3>

Ein [Skill](/docs/de/skills) ist eine `SKILL.md`-Datei, die Claude laden kann, wenn ihre Beschreibung der Aufgabe entspricht. Der Benutzer kann ihn auch als Befehl ausführen. Speichern Sie jeden Skill in seinem eigenen Verzeichnis unter `skills/`:

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json
└── skills/
    └── review/
        └── SKILL.md
```

Geben Sie der `SKILL.md` eine `description`, damit Claude weiß, wann er sie verwenden soll:

```markdown skills/review/SKILL.md theme={null}
---
description: Reviews a pull request for style and test coverage. Use when asked to review code.
---

Review the changed files. Report style problems first, then missing tests.
```

Nachdem Sie das Plugin geladen haben, führt `/my-plugin:review` den Skill aus. Der Befehlsname und wer ihn aufrufen kann, folgen diesen Regeln:

* **Befehlsname**: `/<plugin>:<directory>`, also `skills/review/SKILL.md` in `my-plugin` ist `/my-plugin:review`. Wenn Sie `name` in der Frontmatter setzen, ersetzt es das letzte Segment und das Plugin-Präfix bleibt. Siehe [wie ein Skill seinen Befehlsnamen erhält](/docs/de/skills#how-a-skill-gets-its-command-name)
* **Wer ruft ihn auf**: Claude, der Benutzer oder beide, gesteuert durch Frontmatter. Siehe [Kontrollieren Sie, wer einen Skill aufruft](/docs/de/skills#control-who-invokes-a-skill)

Sie können auch Skills außerhalb des Standard-`skills/`-Verzeichnisses platzieren:

* **Zusätzliche Verzeichnisse**: Listen Sie sie im `skills`-Manifest-Schlüssel auf. Sie ergänzen den Standard-`skills/`-Scan, anstatt ihn zu ersetzen, anders als `commands` und `agents`
* **Ein einzelner Skill im Plugin-Root**: Ohne `skills/`-Verzeichnis und ohne `skills`-Manifest-Schlüssel lädt eine `SKILL.md` im Plugin-Root als ein Skill. Setzen Sie `name` in seiner Frontmatter, da sonst eine Marketplace-Installation den Skill nach seinem [Cache-Verzeichnis](/docs/de/plugins/loading#find-plugins-on-disk) benennt, anstatt nach Ihrem Plugin

Um Anweisungen in ein Plugin einzubeziehen, schreiben Sie sie als Skill. Claude Code lädt keine `CLAUDE.md` im Plugin-Root, und `claude plugin validate` warnt `CLAUDE.md at the plugin root is not loaded as project context`.

Für Frontmatter-Felder und unterstützende Dateien siehe [Skills](/docs/de/skills).

<h3 id="commands">
  Befehle
</h3>

Ein Befehl ist eine einzelne Markdown-Datei, die der Benutzer nach Name ausführt, wie `/my-plugin:about`.

<Note>
  Befehle sind das ältere Format, und [Skills](#skills) ersetzen sie für neue Arbeiten. Ein Skill wird auf die gleiche Weise nach Name ausgeführt, und er kann auch unterstützende Dateien in seinem Verzeichnis tragen. Behalten Sie `commands/` für Dateien, die Sie von `.claude/commands/` verschieben.
</Note>

Speichern Sie einen Befehl unter `commands/<file>.md` und er wird zu `/<plugin>:<file>`. Ein Unterverzeichnis fügt ein Segment hinzu, also ist `commands/db/migrate.md` `/my-plugin:db:migrate`.

Befehlsdateien nehmen die gleiche Frontmatter wie Skills.

<h4 id="define-commands-in-the-manifest">
  Definieren Sie Befehle im Manifest
</h4>

Sie brauchen dies nur, wenn Sie Befehlsdateien irgendwo anders als `commands/` behalten möchten, oder um einen kurzen Befehl in `plugin.json` ohne separate Markdown-Datei zu definieren. Setzen Sie den `commands`-Manifest-Schlüssel, und Claude Code liest ihn statt `commands/` zu scannen. Der Schlüssel nimmt einen Pfad, ein Array von Pfaden oder ein Objekt, das jeden Befehlsnamen entweder auf eine `source`-Datei oder inline `content` abbildet.

Dieses Manifest definiert `/my-plugin:about` inline, ohne Markdown-Datei:

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

Laden Sie das Plugin und führen Sie `/my-plugin:about` in der Sitzung aus, um zu bestätigen, dass es geladen wurde.

Für die vollständige Schlüsselsyntax siehe [`commands`](/docs/de/plugins/manifest-reference#commands).

<h3 id="agents">
  Agents
</h3>

Ein [Subagent](/docs/de/sub-agents) ist ein separater Assistent mit seinen eigenen Anweisungen und Kontextfenster, dem Claude eine Aufgabe delegieren kann. Jede Markdown-Datei unter `agents/` definiert einen:

```markdown agents/security-reviewer.md theme={null}
---
name: security-reviewer
description: Reviews code changes for security issues. Use after edits to authentication or input handling.
model: sonnet
---

You are a security reviewer. Read the changed files and report injection, authentication, and secrets-handling risks.
```

Dieser Agent wird `my-plugin:security-reviewer` genannt, und der Benutzer kann ihn [explizit aufrufen](/docs/de/sub-agents#invoke-subagents-explicitly) mit `@agent-my-plugin:security-reviewer`. Die Namensform ist `<plugin>:<name>`, wobei `<name>` aus der Frontmatter kommt, oder aus dem Dateinamen, wenn es keine gibt.

Der `agents`-Manifest-Schlüssel ersetzt den `agents/`-Scan.

<h4 id="organize-agents-in-subfolders">
  Organisieren Sie Agents in Unterordnern
</h4>

Sie können Plugin-Agent-Dateien in Unterordnern von `agents/` platzieren. Claude Code [lädt sie rekursiv](/docs/de/sub-agents#choose-the-subagent-scope) und verbindet den Plugin-Namen, jeden Unterordnernamen und den Dateinamen mit Doppelpunkten, um den scoped Namen des Agenten zu bilden. Zum Beispiel lädt `agents/review/security.md` in einem Plugin namens `my-plugin` als `my-plugin:review:security`. Zwei Einstellungen ändern diesen Namen:

* Frontmatter `name`: Sie ersetzt nur den Dateinamen, also `name: audit` in `agents/review/security.md` lädt als `my-plugin:review:audit`
* Manifest [`agents`](/docs/de/plugins/manifest-reference#fields)-Feld: Eine Datei, die Sie dort auflisten, lädt ohne Unterordnernamen, also `"agents": "./custom/review/security.md"` lädt als `my-plugin:security`

<h4 id="frontmatter-fields-in-plugin-agents">
  Frontmatter-Felder in Plugin-Agents
</h4>

Die Frontmatter eines Plugin-Agenten folgt diesen Regeln:

* **Unterstützte Felder**: `name`, `description`, `model`, `effort`, `maxTurns`, `tools`, `disallowedTools`, `skills`, `memory`, `background`, `omitClaudeMd`, `isolation`, `color` und der `cacheTtl`-Schlüssel von `experimental`. Der einzige gültige `isolation`-Wert ist `"worktree"`. Siehe [unterstützte Frontmatter-Felder](/docs/de/sub-agents#supported-frontmatter-fields) für das, was jedes tut
* **Ignorierte Felder**: `permissionMode`, `hooks`, `mcpServers` und `initialPrompt`. Eine Agent-Datei kann nicht auf eigene Faust Hooks oder MCP-Server hinzufügen, daher fügen Sie diese stattdessen als Plugin-[Hooks](#hooks) und [MCP-Server](#mcp-servers) hinzu
* **Frontmatter, die nicht analysiert wird**: Der Agent lädt immer noch mit jedem Feld ignoriert. Er wird nach der Datei benannt, und seine Beschreibung liest `Agent from my-plugin plugin`. Führen Sie [`claude plugin validate`](/docs/de/plugins/cli-reference#plugin-validate) in Ihrer Shell aus, um diese Dateien zu finden

Für das, was jedes Feld tut und die Vorrangregeln, siehe [Subagents](/docs/de/sub-agents#supported-frontmatter-fields).

<h3 id="hooks">
  Hooks
</h3>

Ein [Hook](/docs/de/hooks-guide) führt etwas automatisch an einem Punkt im Lebenszyklus von Claude Code aus, wie zum Beispiel nach jeder Dateibearbeitung: ein Shell-Befehl, eine HTTP-Anfrage, ein MCP-Tool-Aufruf, ein Prompt an ein Modell oder ein Subagent. Speichern Sie die Hooks des Plugins in `hooks/hooks.json` im Plugin-Root, unter einem Top-Level-`"hooks"`-Schlüssel, in der gleichen Form wie das `hooks`-Objekt in `settings.json`. Das ermöglicht es Ihnen, einen bestehenden Settings-Hook unverändert zu kopieren.

Dieser Hook führt ein gebündeltes Script nach jedem `Write` oder `Edit` aus:

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

Speichern Sie das Script unter `scripts/format.sh` und machen Sie es ausführbar.

Laden Sie das Plugin und bitten Sie Claude, eine Datei zu bearbeiten. Ein `PostToolUse`-Hook, der 0 beendet, zeigt nichts im Transkript, daher bestätigen Sie, dass er mit [Debug-Logging](/docs/de/hooks#debug-hooks) oder durch das, was das Script selbst ändert, gelaufen ist.

Hooks in `hooks/hooks.json` und im `hooks`-Manifest-Schlüssel laden beide. Für jedes Ereignis und seine Nutzlast siehe [Hook-Ereignisse](/docs/de/hooks#hook-events).

<h4 id="when-plugin-hooks-fire">
  Wenn Plugin-Hooks auslösen
</h4>

Die Hooks eines Plugins warten nicht darauf, dass einer der Skills oder Befehle des Plugins verwendet wird. Claude Code registriert sie, wenn eine Sitzung das Plugin lädt, und sie lösen auf ihren Ereignissen von da an aus. Um einzuschränken, wann ein Hook läuft, verengen Sie seinen `matcher`.

Wenn ein Hook nie auslöst, siehe [Hooks, die nicht auslösen](/docs/de/plugins/troubleshooting#failed-to-load-hooks-from-and-hooks-that-dont-fire).

<h4 id="environment-quoting-and-matching-mcp-tools">
  Umgebung, Anführungszeichen und Matching von MCP-Tools
</h4>

Die Umgebung des Hooks, die Anführungszeichen von `${CLAUDE_PLUGIN_ROOT}` und Matcher für die eigenen MCP-Tools des Plugins funktionieren wie folgt:

* **Umgebung**: Jeder Hook-Prozess erhält `CLAUDE_PLUGIN_ROOT` und `CLAUDE_PLUGIN_DATA` in seiner Umgebung, plus `CLAUDE_PLUGIN_OPTION_<KEY>` für jeden [Benutzerkonfiguration](#user-configuration)-Wert, damit Ihr Script sie von dort lesen kann
* **Anführungszeichen**: Wenn `command` keine `args` hat, läuft es durch eine Shell, daher wickeln Sie den `${CLAUDE_PLUGIN_ROOT}`-Pfad in doppelte Anführungszeichen, wie das Beispiel `hooks/hooks.json` unter [Hooks](#hooks) tut, um den erweiterten Pfad ein Shell-Wort zu halten. Wenn Sie stattdessen `args` übergeben, wird jedes Element als ein Argument ohne Shell übergeben und braucht keine Anführungszeichen. Siehe [Exec-Form und Shell-Form](/docs/de/hooks#exec-form-and-shell-form)
* **Matching der eigenen MCP-Tools des Plugins**: Ein Tool von einem [MCP-Server, den dieses Plugin deklariert](#mcp-servers), wird `mcp__plugin_<plugin>_<server>__<tool>` genannt, daher schreiben Sie diesen vollständigen Namen in den Matcher. Ein Matcher nur auf dem Servernamen löst nie aus. Siehe [Match MCP-Tools](/docs/de/hooks#match-mcp-tools)

<h3 id="mcp-servers">
  MCP-Server
</h3>

Ein MCP-Server gibt Claude Tools von einem externen System. Deklarieren Sie ihn in `.mcp.json` im Plugin-Root, in der gleichen Form wie ein [Projekt `.mcp.json`](/docs/de/mcp#project-scope). Diese `.mcp.json` deklariert einen Server namens `db`:

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

Sie können auch den `mcpServers`-Wrapper weglassen und `db` auf der Top-Level der Datei platzieren.

Laden Sie das Plugin und führen Sie `/mcp` aus, um zu bestätigen, dass der Server als `plugin:my-plugin:db` erscheint.

`claude plugin validate` überprüft `.mcp.json` und meldet einen Server-Eintrag, den Claude Code zur Ladezeit als Fehler ablegen würde. Erfordert Claude Code v2.1.281 oder später.

Für wo ein schlechter Eintrag zur Ladezeit angezeigt wird, siehe [MCP-Server, die nicht starten](/docs/de/plugins/troubleshooting#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start).

Der `mcpServers`-Manifest-Schlüssel nimmt eine inline Server-Map, einen Pfad zu einer JSON-Datei oder ein Array davon. Wenn ein Manifest-Server den gleichen Namen wie einer in `.mcp.json` hat, ersetzt der Manifest-Server ihn.

<h4 id="reach-users-on-claude-ai-and-cowork">
  Erreichen Sie Benutzer auf claude.ai und Cowork
</h4>

Ein lokaler Stdio-Server, wie der `db`-Server unter [MCP-Server](#mcp-servers), läuft in Claude Code und in einer Cowork-Sitzung, die auf Ihrem Computer in der Claude Desktop-App läuft, aber nicht auf claude.ai. Um Benutzer dort auch zu erreichen, referenzieren Sie einen Remote-Server durch seine `https://`-URL, die claude.ai und Cowork dem Benutzer als Connector anbieten.

<h4 id="server-names-tool-names-and-reloads">
  Server-Namen, Tool-Namen und Neuladen
</h4>

Die Namen des Servers, die Variable-Substitution und das Neuladen-Verhalten folgen diesen Regeln:

* **Server-Name**: `plugin:<plugin>:<server>`, also der `db`-Server in `my-plugin` ist `plugin:my-plugin:db` in `/mcp`. Verwenden Sie die gleiche Form, um den Server in einem [`mcp_tool`-Hook](/docs/de/hooks#mcp-tool-hook-fields) zu benennen
* **Tool-Namen**: `mcp__plugin_<plugin>_<server>__<tool>`, also ein `query`-Tool auf diesem `db`-Server ist `mcp__plugin_my-plugin_db__query`. Das ist der Name, der in [Berechtigungsregeln](/docs/de/permissions) und [Hook-Matchern](#hooks) verwendet wird
* **Substitution**: `${CLAUDE_PLUGIN_ROOT}` und die anderen [Pfad-Variablen](#path-variables-and-persistent-data) werden in `command`, `args` und `env` ersetzt. Keine Anführungszeichen sind in `args` erforderlich, da jedes Element als ein Argument übergeben wird
* **Neuladen**: Wenn der Benutzer `/reload-plugins` ausführt und [das Neuladen angewendet wird](/docs/de/plugins/cli-reference#reloads-that-change-mcp-tools), behält ein Server, dessen Konfiguration unverändert ist, seine Verbindung. Ein Server, dessen Konfiguration sich geändert hat, verbindet sich neu, und einer, den Sie entfernt haben, trennt sich

<h4 id="include-a-packaged-mcpb-server">
  Schließen Sie einen verpackten MCPB-Server ein
</h4>

Der `mcpServers`-Schlüssel akzeptiert auch einen verpackten Server als [MCPB-Datei](https://github.com/modelcontextprotocol/mcpb), deren Erweiterung `.mcpb` oder die ältere `.dxt` ist. Zeigen Sie den Schlüssel auf die Datei, als Pfad im Plugin oder eine `https://`-URL:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "my-plugin",
  "mcpServers": "./servers/db.mcpb"
}
```

Der Server nimmt seinen Namen aus dem `name` im Manifest des Bundles.

Für Transporte und Authentifizierung siehe [MCP](/docs/de/mcp#plugin-provided-mcp-servers).

<h3 id="lsp-servers">
  LSP-Server
</h3>

Ein LSP-Server gibt Claude Diagnostik und Code-Navigation für eine Sprache. Wenn ein [offizielles Code-Intelligence-Plugin](/docs/de/plugins/code-intelligence) Ihre Sprache bereits abdeckt, installieren Sie das statt einen zu schreiben. Andernfalls deklarieren Sie den Server in `.lsp.json` im Plugin-Root:

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

Die Datei bildet jeden Server-Namen direkt auf seine Konfiguration ab, ohne ein Wrapper-Objekt um die Map. `command` ist der Name des Binärs, mit seinen Argumenten in `args`. `extensionToLanguage` braucht mindestens eine Erweiterung, jede beginnend mit `.`.

`claude plugin validate` liest diese Datei nicht. Wenn ein Eintrag ungültig ist, wird die ganze Datei zur Ladezeit übersprungen und `Invalid LSP server config for ".lsp.json"` erscheint in der `/plugin`-Registerkarte **Errors**.

Ihr Plugin konfiguriert die Verbindung, installiert aber nicht das Server-Binär, und jede Dateierweiterung bekommt einen Server:

* **Fehlendes Binär**: Claude Code startet `command` nach Name aus dem `PATH` des Benutzers. Wenn das Binär nicht da ist, schlägt der Server fehl zu starten und `claude --debug` protokolliert `LSP server <name> failed to start`
* **Erweiterungs-Konflikte**: Wenn zwei aktivierte Server die gleiche Erweiterung beanspruchen, behandelt der erste registrierte diese Dateien und der andere wird nicht für sie verwendet, ob die Server von einem Plugin oder zwei kommen. Die `/plugin`-Registerkarte **Errors** zeigt die Warnung `LSP server "<name>" is not used for <ext> files`

Der `lspServers`-Manifest-Schlüssel nimmt die gleiche Map inline, einen Pfad zu einer JSON-Datei oder ein Array davon, und seine Server ergänzen die in `.lsp.json`. Wenn ein Manifest-Server den gleichen Namen wie einer in `.lsp.json` hat, ersetzt der Manifest-Server ihn.

Für `transport`, Timeouts, Neustarts und die anderen Felder siehe [`lspServers`](/docs/de/plugins/manifest-reference#lspservers).

Senden Sie Log-Ausgabe an stderr, nicht stdout. Claude Code liest den stdout eines Servers nur als Protokoll-Nachrichten und akzeptiert Nachrichten-Header bis zu 64 KiB und einen Nachrichten-Text bis zu 32 MiB.

Claude Code trennt einen Server, der eines der Limits überschreitet oder nicht-Protokoll-Ausgabe an stdout schreibt, und zählt die Trennung als Absturz für `restartOnCrash` und `maxRestarts`. Wenn Sie mit `--debug` laufen, schreibt Claude Code einen Fehler, der die Ursache benennt, in das Debug-Log.

<h3 id="executables">
  Ausführbare Dateien
</h3>

Dateien in `bin/` im Plugin-Root sind auf dem `PATH` der Shell des Bash-Tools, während das Plugin aktiviert ist, daher kann Claude sie als bloße Befehle ausführen. Fügen Sie ein ausführbares Script hinzu:

```bash bin/hello-plugin theme={null}
#!/bin/bash
echo "hello from my-plugin"
```

Machen Sie es mit `chmod +x bin/hello-plugin` ausführbar und laden Sie das Plugin. Wenn Sie Claude bitten, `hello-plugin` auszuführen, zeigt das Bash-Tool-Ergebnis die Ausgabe des Scripts.

Plugin-`bin/`-Verzeichnisse kommen nach den eigenen `PATH`-Einträgen des Benutzers, daher kann ein Plugin nicht `git`, `ls` oder einen anderen System-Befehl überschatten.

claude.ai und Cowork installieren kein Plugin, das ein Top-Level-`bin/`-Verzeichnis hat, einschließlich eines, das Sie [über claude.ai-Organisationseinstellungen verteilen](/docs/de/plugins/host-marketplace#distribute-through-organization-settings).

<h3 id="default-settings">
  Standard-Einstellungen
</h3>

Um Standard-Einstellungen zu setzen, die gelten, während das Plugin aktiviert ist, fügen Sie eine `settings.json` im Plugin-Root hinzu, oder setzen Sie das gleiche Objekt inline im `settings`-Manifest-Schlüssel. Zwei Schlüssel wirken sich aus, `agent` und `subagentStatusLine`, und jeder andere Schlüssel wird gelöscht.

Setzen Sie `agent`, um einen der eigenen Agents des Plugins als Haupt-Thread auszuführen:

```json settings.json theme={null}
{
  "agent": "security-reviewer"
}
```

Laden Sie das Plugin und starten Sie eine Sitzung. Claude antwortet dann in der Haupt-Konversation mit dem System-Prompt und Modell des `security-reviewer`-Agenten.

Für alles, das der Schlüssel kontrolliert, siehe die [`agent`-Einstellung](/docs/de/settings-reference#agent).

Wenn der gleiche Schlüssel an mehr als einem Ort gesetzt ist, entscheiden diese Regeln, welcher Wert angewendet wird:

* **Datei über Manifest**: Wenn beide existieren und `settings.json` mindestens einen unterstützten Schlüssel setzt, wendet `settings.json` an und das Manifest `settings` wird ignoriert
* **Benutzer-Einstellungen über Plugin-Standard**: Über Einstellungs-Quellen hinweg sind Plugin-Standard die niedrigste Schicht, daher überschreibt Ihr eigenes `agent` eines Benutzers in `~/.claude/settings.json` Ihres
* **Zwei Plugins setzen den gleichen Schlüssel**: Der Wert vom zuletzt geladenen Plugin wendet an, und `claude --debug` protokolliert `overrides setting`

Für die `subagentStatusLine`-Form siehe [Subagent-Statuszeilen](/docs/de/statusline#subagent-status-lines).

<h3 id="themes-and-output-styles">
  Themen und Output-Stile
</h3>

Ein Plugin kann Farbschemas und Output-Stile enthalten. Beide erscheinen in den gleichen Pickern wie die des Benutzers. Für jeden setzt der Manifest-Schlüssel den Ordner-Scan.

| Komponente  | Speichern unter           | Format                                                                                                                                | Erscheint in                           | Manifest-Schlüssel    |
| :---------- | :------------------------ | :------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------- | :-------------------- |
| Thema       | `themes/<slug>.json`      | Das [benutzerdefinierte Thema-Dateiformat](/docs/de/terminal-config#create-a-custom-theme), das Benutzer in `~/.claude/themes/` schreiben  | `/theme`, unter dem `name` der Datei   | `experimental.themes` |
| Output-Stil | `output-styles/<name>.md` | Das [benutzerdefinierte Output-Stil-Format](/docs/de/output-styles#create-a-custom-output-style), mit `name` und `description`-Frontmatter | `/output-style`, als `<plugin>:<name>` | `outputStyles`        |

Plugin-Themen sind schreibgeschützt, daher wenn ein Benutzer eines in `/theme` bearbeitet, wird die Bearbeitung als Kopie in seinem eigenen Themen-Verzeichnis gespeichert.

Dieses Thema färbt den Prompt-Akzent und Fehlertext auf der dunklen Voreinstellung um:

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
  Kanäle
</h3>

Ein [Kanal](/docs/de/channels) ermöglicht es einem externen System wie einer Chat-App, Nachrichten in eine Sitzung zu senden. In einem Plugin ist ein Kanal einer der MCP-Server plus ein `channels`-Eintrag, der sich daran bindet und seine eigene Konfiguration auffordern kann. Dieses Manifest bindet einen Kanal an einen `telegram`-Server und fragt nach einem Bot-Token:

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

`server` muss einem Schlüssel in `mcpServers` entsprechen. Die pro-Kanal `userConfig` nimmt die gleiche Form wie der [Top-Level-`userConfig`-Schlüssel](#user-configuration).

Für das, was der Server implementieren muss und wie Benutzer einen Kanal-Plugin aktivieren, siehe [Als Plugin verpacken](/docs/de/channels-reference#package-as-a-plugin) in der Kanäle-Referenz. Für die Feldtabelle siehe [`channels`](/docs/de/plugins/manifest-reference#channels).

<h3 id="monitors">
  Monitore
</h3>

Ein Monitor ist ein Shell-Befehl, der im Hintergrund für die ganze Sitzung läuft. Was er ausgibt, erreicht Claude als Benachrichtigungen, daher kann Claude auf ein Protokoll oder eine Statusänderung reagieren, ohne gebeten zu werden, es zu beobachten. Speichern Sie die Einträge in `monitors/monitors.json`:

```json monitors/monitors.json theme={null}
[
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Application error log"
  }
]
```

Der Befehl läuft in einer Shell, im Arbeitsverzeichnis, in dem die Sitzung gestartet wurde.

Der Befehl eines Monitors ist begrenzt, wo er startet und was er referenzieren kann:

* **Nur interaktive Sitzungen**: Plugin-Monitore starten in einer interaktiven Sitzung und nie im nicht-interaktiven Modus mit dem `-p`-Flag. Sie starten auch nur, wo das [Monitor-Tool](/docs/de/tools-reference#monitor-tool) verfügbar ist
* **Keine Benutzerkonfiguration**: `command` erhält die [Pfad-Variablen](#path-variables-and-persistent-data) und `${ENV_VAR}` aus der Umgebung, aber nie `${user_config.*}`. Ein Monitor, der einen referenziert, startet nicht, und Monitor-Prozesse erhalten auch nicht `CLAUDE_PLUGIN_OPTION_<KEY>`
* **Deaktivieren während der Sitzung**: Wenn Sie ein Plugin während der Sitzung deaktivieren, stoppt Claude Code nicht die Monitore, die bereits laufen. Sie stoppen, wenn die Sitzung endet

Der `experimental.monitors`-Manifest-Schlüssel nimmt das gleiche Array inline oder einen Pfad zu einer JSON-Datei und wird statt `monitors/monitors.json` gelesen.

Für den `when`-Trigger und die anderen Felder siehe [`monitors`](/docs/de/plugins/manifest-reference#monitors).

<h2 id="user-configuration">
  Fragen Sie den Benutzer nach Konfigurationswerten
</h2>

Deklarieren Sie die Werte, die Ihr Plugin vom Benutzer benötigt, im `userConfig`-Manifest-Schlüssel, damit Benutzer nicht `settings.json` selbst bearbeiten. Jede Option erscheint in einem Dialog mit seinem `title` als Label und seiner `description` darunter.

Setzen Sie `"sensitive": true` für einen Token oder ein Passwort. Der Dialog maskiert dann die Eingabe, und der Wert wird in sicherer Speicherung statt `settings.json` gespeichert.

Dieses Manifest fragt nach einem Endpunkt und einem Token:

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
  Wenn der Konfigurationsdialog erscheint
</h3>

Der Dialog erscheint nur in der interaktiven `/plugin`-Schnittstelle. Er öffnet sich für jede Option, die noch nicht gesetzt ist, wenn der Benutzer eines der folgenden tut:

* Installiert das Plugin in `/plugin`
* Führt `/plugin install <plugin>@<marketplace>` in einer Sitzung aus
* Aktiviert das Plugin aus der **Installed**-Registerkarte in `/plugin`

Um den gleichen Dialog jederzeit zu öffnen, führt der Benutzer `/plugin configure <plugin>@<marketplace>` aus.

Der `claude plugin install`-Shell-Befehl fordert nie `userConfig`-Werte auf. Um Werte aus der Shell zu setzen, übergeben Sie jeden als `--config KEY=VALUE`. Wenn Optionen ungesetzt bleiben, druckt der Befehl eine `userConfig options not yet set`-Zeile, die beide Wege benennt, um sie zu setzen. [Der `userConfig`-Dialog erscheint nie](/docs/de/plugins/troubleshooting#the-userconfig-dialog-never-appears) zitiert die Zeile.

Für die Optionsfelder, wo jeder Wert gespeichert wird, wie eine Komponente einen gespeicherten Wert referenziert und welche Felder `${user_config.*}` ablehnen, siehe [Benutzerkonfiguration](/docs/de/plugins/manifest-reference#user-configuration).

<h2 id="path-variables-and-persistent-data">
  Referenzieren Sie Plugin-Pfade und speichern Sie Daten
</h2>

Sie wissen nicht, wo Ihr Plugin installiert wird, daher referenzieren Sie seine Dateien und Daten durch diese Variablen statt fester Pfade. Sie werden in Skill-, Befehls- und Agent-Inhalten, in Hook- und Monitor-Befehlen und in MCP- und LSP-Server-Konfigurationen ersetzt. Sie werden auch an Hook-, MCP- und LSP-Prozesse exportiert:

* **`${CLAUDE_PLUGIN_ROOT}`**: Das Installationsverzeichnis des Plugins. Jede Version hat ihr eigenes [Cache-Verzeichnis](/docs/de/plugins/loading#find-plugins-on-disk), daher ändert sich der Pfad, wenn das Plugin aktualisiert wird. Schreiben Sie keinen Zustand dort
* **`${CLAUDE_PLUGIN_DATA}`**: Ein Verzeichnis, das Updates überlebt, für `node_modules`, virtuelle Umgebungen und Caches. Es wird zu `~/.claude/plugins/data/<id>/` aufgelöst und wird erstellt, wenn zuerst referenziert
* **`${CLAUDE_PROJECT_DIR}`**: Das Projekt-Root, der gleiche Wert, den Hooks erhalten

Im Pfad des Daten-Verzeichnisses ist `<id>` die Plugin-ID mit jedem Zeichen außer Buchstaben, Ziffern, `_` und `-` ersetzt durch `-`, daher wird `my-plugin@my-marketplace` zu `my-plugin-my-marketplace`.

Auf Windows verwenden die ersetzten Pfade Schrägstriche, daher liest eine Shell Backslashes nicht als Escapes.

<h3 id="install-dependencies-into-the-data-directory">
  Installieren Sie Abhängigkeiten in das Daten-Verzeichnis
</h3>

Für ein Marketplace-installiertes Plugin installiert Claude Code automatisch berechtigte [Node.js-Paket-Abhängigkeiten](/docs/de/plugins/loading#node-js-package-dependencies), wenn es das Plugin zwischenspeichert, daher müssen Sie sie möglicherweise nicht selbst installieren. Wenn Sie es tun, installiert dieser `SessionStart`-Hook `node_modules` in `${CLAUDE_PLUGIN_DATA}` beim ersten Lauf und erneut nach einer Aktualisierung, die `package.json` ändert:

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

Nach der ersten Sitzung existiert `~/.claude/plugins/data/<id>/node_modules`. Ein MCP-Server kann dann `NODE_PATH` auf `${CLAUDE_PLUGIN_DATA}/node_modules` in seinem `env` setzen. Für welche Felder welche Variable ersetzen, siehe [Umgebungsvariablen](/docs/de/plugins/manifest-reference#environment-variables).

<h2 id="next-steps">
  Nächste Schritte
</h2>

* [Plugin-Manifest-Referenz](/docs/de/plugins/manifest-reference): `plugin.json`-Felder, Pfad-Regeln und das Standard-Layout
* [Testen Sie Plugins mit Evals](/docs/de/plugin-evals): Überprüfen Sie, dass die Komponenten, die Sie hinzugefügt haben, Claudes Verhalten so ändern, wie Sie beabsichtigen
* [Veröffentlichen und verteilen Sie ein Plugin](/docs/de/plugins/publish): Versionieren Sie das Plugin und setzen Sie es in einen Marketplace
* [Beheben Sie Plugin-Probleme](/docs/de/plugins/troubleshooting): Was zu tun ist, wenn eine Komponente nicht lädt oder ein Hook nicht auslöst
