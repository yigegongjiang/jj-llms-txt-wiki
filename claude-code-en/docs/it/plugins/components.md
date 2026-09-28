> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Aggiungi componenti a un plugin

> Aggiungi skills, hooks, server MCP e ogni altro tipo di componente a un plugin Claude Code, con un esempio che convalida per ciascuno.

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

Un plugin Claude Code è costruito da componenti, come skills, agenti, hooks e server MCP. Ogni componente ha una cartella predefinita nel plugin, una chiave manifest opzionale in `.claude-plugin/plugin.json` che sostituisce o aggiunge a quella cartella, e un nome che l'utente vede. Per la tabella dei campi completa di ogni chiave, vedere il [riferimento manifest](/docs/it/plugins/manifest-reference#fields).

Utilizzare questa pagina per aggiungere un componente a un plugin che già carica.

Dopo aver aggiunto un componente, eseguire `/reload-plugins` in una sessione in esecuzione o avviarne una nuova in modo che Claude Code lo carichi. Per controllare il file del componente prima di caricarlo, eseguire [`claude plugin validate .`](/docs/it/plugins/cli-reference#plugin-validate) nella shell dalla directory del plugin.

<Note>
  Questi casi sono coperti su altre pagine:

  * **Costruire il primo plugin**: iniziare con [Crea un plugin](/docs/it/plugins/create)
  * **Installare il plugin di qualcun altro**: vedere [Installa plugin](/docs/it/plugins/install)
  * **Gli utenti del plugin sono su claude.ai o in Cowork**: un set diverso di componenti carica lì. Vedere [Plugin su claude.ai e in Cowork](https://claude.com/docs/plugins/overview)
</Note>

<h2 id="explore-the-plugin-directory">
  Esplora la directory dei plugin
</h2>

L'explorer mostra un plugin di esempio, `my-plugin`, che ha uno di ogni tipo di componente nella sua posizione predefinita:

* Una skill di revisione e un comando `about`
* Un subagent di security-review
* Un hook che formatta i file dopo che Claude li modifica, e la cartella `scripts/` che chiama
* Un monitor di log
* Uno stile di output e un tema di colore
* Un workflow route-audit
* Un eseguibile `hello-plugin`
* Impostazioni predefinite
* Un server MCP locale e un language server Go

Ogni file è l'esempio valido più piccolo del suo formato, presente per mostrare la struttura piuttosto che per essere utile: una skill o un agent reale contiene istruzioni complete e spesso file di supporto, e un hook o monitor reale svolge un lavoro reale. Le sezioni dopo l'explorer utilizzano gli stessi file come esempi e collegano a versioni più complete. Seleziona un file o una cartella per leggere a cosa serve, vedere cosa va dentro e trovare la sezione che la copre.

<PluginExplorer>
  <Piece id="manifest">
    Il [manifest](/docs/it/plugins/manifest-reference) è il file `plugin.json` nella directory `.claude-plugin/` di un plugin. Contiene i metadati del plugin e i valori `userConfig` che Claude Code richiede all'utente. Solo `name` è obbligatorio. In questo, `description` è il testo che gli utenti vedono per il plugin in `/plugin`, e `version` mantiene gli utenti su quella versione finché non la modifichi:

    ```json theme={null}
    {
      "name": "my-plugin",
      "version": "1.0.0",
      "description": "Review, formatting, and database tools for this team"
    }
    ```
  </Piece>

  <Piece id="skills">
    Una [skill](/docs/it/skills) è un file `SKILL.md`. Salva ogni skill nella sua directory sotto `skills/`. Claude legge la `description` di ogni skill, e quando quello che l'utente chiede corrisponde, come chiedere a Claude di revisionare una pull request qui, Claude carica le istruzioni della skill e le segue. L'utente può anche eseguirla direttamente come `/my-plugin:review`:

    ```markdown theme={null}
    ---
    description: Reviews a pull request for style and test coverage. Use when asked to review code.
    ---

    Review the changed files. Report style problems first, then missing tests.
    ```
  </Piece>

  <Piece id="commands">
    Un comando è un singolo file Markdown che l'utente esegue per nome. I comandi sono il formato più vecchio: una skill viene eseguita per nome allo stesso modo e può anche contenere file di supporto nella sua directory, quindi scrivi i nuovi come skill e mantieni `commands/` per i file che hai già. Questo file diventa `/my-plugin:about` e accetta lo stesso frontmatter di una skill:

    ```markdown theme={null}
    ---
    description: Summarize the repository
    ---

    Summarize what this repository does in three sentences.
    ```
  </Piece>

  <Piece id="agents">
    Un [subagent](/docs/it/sub-agents) è un assistente separato, con le sue istruzioni e la sua finestra di contesto, che Claude può delegare un compito e ottenere un risultato. Ogni file Markdown sotto `agents/` ne definisce uno: il frontmatter lo nomina e dice quando usarlo, e il corpo è il suo system prompt. Questo è denominato `my-plugin:security-reviewer`, e l'utente può invocarlo con `@agent-my-plugin:security-reviewer`:

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
    Un [hook](/docs/it/hooks-guide) esegue qualcosa automaticamente in un punto del ciclo di vita di Claude Code, come dopo ogni modifica di file: un comando shell, una richiesta HTTP, una chiamata a uno strumento MCP, un prompt a un modello, o un subagent. Salva gli hook del plugin in `hooks/hooks.json` alla radice del plugin. Questo esegue lo script `scripts/format.sh` del plugin dopo che Claude scrive o modifica un file:

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
    Un monitor è un comando shell che Claude Code avvia in background quando la sessione inizia e continua a eseguire fino a quando non termina, utilizzando lo [strumento Monitor](/docs/it/tools-reference#monitor-tool). Quello che stampa raggiunge Claude come notifiche. Un campo `when` può invece avviarlo la prima volta che una skill denominata viene eseguita. Questo monitora un log di errore:

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
    Un plugin può includere [stili di output](/docs/it/output-styles), che cambiano il modo in cui Claude formatta e formula le sue risposte. Salva ogni stile di output come `output-styles/<name>.md`. Questo appare in `/output-style` come `my-plugin:terse`:

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
    Un plugin può includere [temi di colore](/docs/it/terminal-config#create-a-custom-theme) per l'interfaccia di Claude Code. Salva ogni tema come `themes/<slug>.json`. Questo appare in `/theme` come `Dracula`, contrassegnato come da `my-plugin`:

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
    La cartella `workflows/` contiene file [workflow](/docs/it/workflows) `.js`: un blocco `meta`, quindi un corpo di script che orchestra diversi subagent. Questo viene eseguito come `/my-plugin:audit-routes`:

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
    `bin/` è il modo in cui un plugin fornisce uno strumento da riga di comando. Mentre il plugin è abilitato, Claude Code mette questa cartella sul `PATH` della shell in cui esegue i comandi, quindi Claude, o le istruzioni di una skill, possono eseguire lo strumento per nome senza che l'utente installi nulla. Con questo [eseguibile](#executables) in posizione, `hello-plugin` è un comando che Claude può eseguire:

    ```bash theme={null}
    #!/bin/bash
    echo "hello from my-plugin"
    ```
  </Piece>

  <Piece id="scripts">
    L'hook in `hooks/hooks.json` esegue uno script, e questa cartella è dove l'esempio lo mantiene. Il nome `scripts/` è una convenzione, non qualcosa che Claude Code cerca: l'hook punta al file dal suo percorso, `${CLAUDE_PLUGIN_ROOT}/scripts/format.sh`. Uno script di formattazione potrebbe assomigliare a questo:

    ```bash theme={null}
    #!/bin/bash
    npx prettier --write .
    ```
  </Piece>

  <Piece id="settings">
    Un `settings.json` alla radice del plugin contiene [impostazioni](/docs/it/settings-reference) che si applicano mentre il plugin è abilitato, quindi un plugin può cambiare il comportamento della sessione e non solo aggiungere componenti. Solo due chiavi hanno effetto da un plugin, [`agent`](/docs/it/settings-reference#agent) e [`subagentStatusLine`](/docs/it/settings-reference#subagentstatusline); ogni altra chiave viene scartata. Vedi [Impostazioni predefinite](#default-settings).

    Questo imposta `agent`, che esegue il thread principale della sessione come l'agent `security-reviewer` del plugin, quindi il system prompt di quell'agent, le restrizioni degli strumenti e il modello si applicano all'intera sessione:

    ```json theme={null}
    {
      "agent": "security-reviewer"
    }
    ```
  </Piece>

  <Piece id="mcp">
    Un [server MCP](/docs/it/mcp) fornisce a Claude strumenti da un sistema esterno. Dichiaralo in `.mcp.json` alla radice del plugin. Questo avvia un server locale da uno script all'interno del plugin e appare in `/mcp` come `plugin:my-plugin:db`:

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
    Un server LSP fornisce a Claude [diagnostica e navigazione del codice](/docs/it/plugins/code-intelligence) per un linguaggio. Dichiara il server in `.lsp.json` alla radice del plugin. Questo connette il language server Go per i file `.go`:

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
  Aggiungi ogni tipo di componente
</h2>

Ogni sezione di seguito copre un tipo di componente: dove i suoi file vanno nel plugin, un esempio che convalida, cosa vede l'utente una volta che il plugin carica, e la chiave manifest che cambia la posizione predefinita. Aggiungere quelli di cui il plugin ha bisogno; nessuno è obbligatorio.

<h3 id="skills">
  Skills
</h3>

Una [skill](/docs/it/skills) è un file `SKILL.md` che Claude può caricare quando la sua descrizione corrisponde al compito. L'utente può anche eseguirla come comando. Salvare ogni skill nella sua directory sotto `skills/`:

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json
└── skills/
    └── review/
        └── SKILL.md
```

Dare al `SKILL.md` una `description` in modo che Claude sappia quando usarla:

```markdown skills/review/SKILL.md theme={null}
---
description: Reviews a pull request for style and test coverage. Use when asked to review code.
---

Review the changed files. Report style problems first, then missing tests.
```

Dopo aver caricato il plugin, `/my-plugin:review` esegue la skill. Il nome del comando e chi può invocarlo seguono queste regole:

* **Nome del comando**: `/<plugin>:<directory>`, quindi `skills/review/SKILL.md` in `my-plugin` è `/my-plugin:review`. Se imposti `name` nel frontmatter, sostituisce l'ultimo segmento e il prefisso del plugin rimane. Vedere [come una skill ottiene il suo nome di comando](/docs/it/skills#how-a-skill-gets-its-command-name)
* **Chi la invoca**: Claude, l'utente o entrambi, controllato dal frontmatter. Vedere [Controlla chi invoca una skill](/docs/it/skills#control-who-invokes-a-skill)

Puoi anche posizionare le skills al di fuori della directory predefinita `skills/`:

* **Directory aggiuntive**: elencale nella chiave manifest `skills`. Aggiungono alla scansione predefinita `skills/` piuttosto che sostituirla, a differenza di `commands` e `agents`
* **Una singola skill alla radice del plugin**: senza directory `skills/` e senza chiave manifest `skills`, un `SKILL.md` alla radice del plugin carica come una skill. Imposta `name` nel suo frontmatter, perché altrimenti un'installazione del marketplace nomina la skill dopo la sua [directory cache](/docs/it/plugins/loading#find-plugins-on-disk) piuttosto che il tuo plugin

Per includere istruzioni in un plugin, scrivile come una skill. Claude Code non carica un `CLAUDE.md` alla radice del plugin, e `claude plugin validate` avverte `CLAUDE.md at the plugin root is not loaded as project context`.

Per i campi frontmatter e i file di supporto, vedere [Skills](/docs/it/skills).

<h3 id="commands">
  Comandi
</h3>

Un comando è un singolo file Markdown che l'utente esegue per nome, come `/my-plugin:about`.

<Note>
  I comandi sono il formato più vecchio, e le [skills](#skills) li superano per il nuovo lavoro. Una skill viene eseguita per nome allo stesso modo, e può anche portare file di supporto nella sua directory. Mantenere `commands/` per i file che stai spostando da `.claude/commands/`.
</Note>

Salvare un comando in `commands/<file>.md` e diventa `/<plugin>:<file>`. Una sottodirectory aggiunge un segmento, quindi `commands/db/migrate.md` è `/my-plugin:db:migrate`.

I file di comando accettano lo stesso frontmatter delle skills.

<h4 id="define-commands-in-the-manifest">
  Definisci comandi nel manifest
</h4>

Hai bisogno di questo solo se vuoi mantenere i file di comando da qualche parte diversa da `commands/`, o per definire un comando breve all'interno di `plugin.json` senza un file Markdown separato. Imposta la chiave manifest `commands`, e Claude Code la legge invece di scansionare `commands/`. La chiave accetta un percorso, un array di percorsi, o un oggetto che mappa ogni nome di comando a un file `source` o a `content` inline.

Questo manifest definisce `/my-plugin:about` inline, senza file Markdown:

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

Carica il plugin ed esegui `/my-plugin:about` nella sessione per confermare che ha caricato.

Per la sintassi completa della chiave, vedere [`commands`](/docs/it/plugins/manifest-reference#commands).

<h3 id="agents">
  Agenti
</h3>

Un [subagente](/docs/it/sub-agents) è un assistente separato, con le sue istruzioni e finestra di contesto, che Claude può delegare a un compito. Ogni file Markdown sotto `agents/` ne definisce uno:

```markdown agents/security-reviewer.md theme={null}
---
name: security-reviewer
description: Reviews code changes for security issues. Use after edits to authentication or input handling.
model: sonnet
---

You are a security reviewer. Read the changed files and report injection, authentication, and secrets-handling risks.
```

Questo agente è denominato `my-plugin:security-reviewer`, e l'utente può [invocarlo esplicitamente](/docs/it/sub-agents#invoke-subagents-explicitly) con `@agent-my-plugin:security-reviewer`. La forma del nome è `<plugin>:<name>`, dove `<name>` viene dal frontmatter, o dal nome del file quando non c'è.

La chiave manifest `agents` sostituisce la scansione `agents/`.

<h4 id="organize-agents-in-subfolders">
  Organizza agenti in sottocartelle
</h4>

Puoi mettere i file dell'agente del plugin in sottocartelle di `agents/`. Claude Code [li carica ricorsivamente](/docs/it/sub-agents#choose-the-subagent-scope) e unisce il nome del plugin, ogni nome di sottocartella e il nome del file con due punti per formare il nome con ambito dell'agente. Ad esempio, `agents/review/security.md` in un plugin denominato `my-plugin` carica come `my-plugin:review:security`. Due impostazioni cambiano quel nome:

* Frontmatter `name`: sostituisce solo il nome del file, quindi `name: audit` in `agents/review/security.md` carica come `my-plugin:review:audit`
* Campo manifest [`agents`](/docs/it/plugins/manifest-reference#fields): un file che elenchi lì carica senza nomi di sottocartella, quindi `"agents": "./custom/review/security.md"` carica come `my-plugin:security`

<h4 id="frontmatter-fields-in-plugin-agents">
  Campi frontmatter negli agenti del plugin
</h4>

Il frontmatter di un agente del plugin segue queste regole:

* **Campi supportati**: `name`, `description`, `model`, `effort`, `maxTurns`, `tools`, `disallowedTools`, `skills`, `memory`, `background`, `omitClaudeMd`, `isolation`, `color`, e la chiave `cacheTtl` di `experimental`. L'unico valore `isolation` valido è `"worktree"`. Vedere [campi frontmatter supportati](/docs/it/sub-agents#supported-frontmatter-fields) per quello che fa ciascuno
* **Campi ignorati**: `permissionMode`, `hooks`, `mcpServers`, e `initialPrompt`. Un file agente non può aggiungere hook o server MCP da solo, quindi aggiungili come plugin [hooks](#hooks) e [server MCP](#mcp-servers) invece
* **Frontmatter che non analizza**: l'agente carica comunque con ogni campo ignorato. È denominato dopo il file, e la sua descrizione legge `Agent from my-plugin plugin`. Esegui [`claude plugin validate`](/docs/it/plugins/cli-reference#plugin-validate) nella shell per trovare questi file

Per quello che fa ogni campo e le regole di precedenza, vedere [Subagenti](/docs/it/sub-agents#supported-frontmatter-fields).

<h3 id="hooks">
  Hooks
</h3>

Un [hook](/docs/it/hooks-guide) esegue qualcosa automaticamente in un punto del ciclo di vita di Claude Code, come dopo ogni modifica di file: un comando shell, una richiesta HTTP, una chiamata a uno strumento MCP, un prompt a un modello, o un subagente. Salvare gli hook del plugin in `hooks/hooks.json` alla radice del plugin, sotto una chiave `"hooks"` di livello superiore, nella stessa forma dell'oggetto `hooks` in `settings.json`. Questo ti permette di copiare un hook di impostazioni esistente senza modifiche.

Questo hook esegue uno script in bundle dopo ogni `Write` o `Edit`:

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

Salvare lo script in `scripts/format.sh` e renderlo eseguibile.

Carica il plugin e chiedi a Claude di modificare un file. Un hook `PostToolUse` che esce con 0 non mostra nulla nella trascrizione, quindi conferma che è stato eseguito con [debug logging](/docs/it/hooks#debug-hooks) o da quello che lo script stesso cambia.

Gli hook in `hooks/hooks.json` e nella chiave manifest `hooks` caricano entrambi. Per ogni evento e il suo payload, vedere [Hook events](/docs/it/hooks#hook-events).

<h4 id="when-plugin-hooks-fire">
  Quando gli hook del plugin si attivano
</h4>

Gli hook di un plugin non aspettano che una delle skill o dei comandi del plugin venga utilizzata. Claude Code li registra quando una sessione carica il plugin, e si attivano sui loro eventi da allora in poi. Per limitare quando un hook viene eseguito, restringere il suo `matcher`.

Se un hook non si attiva mai, vedere [hook che non si attivano](/docs/it/plugins/troubleshooting#failed-to-load-hooks-from-and-hooks-that-dont-fire).

<h4 id="environment-quoting-and-matching-mcp-tools">
  Ambiente, quoting e corrispondenza degli strumenti MCP
</h4>

L'ambiente dell'hook, il quoting di `${CLAUDE_PLUGIN_ROOT}`, e i matcher per gli strumenti MCP del plugin funzionano come segue:

* **Ambiente**: ogni processo hook riceve `CLAUDE_PLUGIN_ROOT` e `CLAUDE_PLUGIN_DATA` nel suo ambiente, più `CLAUDE_PLUGIN_OPTION_<KEY>` per ogni valore di [configurazione utente](#user-configuration), in modo che lo script possa leggerli da lì
* **Quoting**: quando `command` non ha `args`, viene eseguito attraverso una shell, quindi avvolgi il percorso `${CLAUDE_PLUGIN_ROOT}` tra virgolette doppie, come fa l'esempio `hooks/hooks.json` sotto [Hooks](#hooks), per mantenere il percorso espanso una parola shell. Quando passi `args` invece, ogni elemento viene passato come un argomento senza shell e non ha bisogno di quoting. Vedere [exec form e shell form](/docs/it/hooks#exec-form-and-shell-form)
* **Corrispondenza degli strumenti MCP del plugin**: uno strumento da un [server MCP che questo plugin dichiara](#mcp-servers) è denominato `mcp__plugin_<plugin>_<server>__<tool>`, quindi scrivi quel nome completo nel matcher. Un matcher sul solo nome del server non si attiva mai. Vedere [Match MCP tools](/docs/it/hooks#match-mcp-tools)

<h3 id="mcp-servers">
  Server MCP
</h3>

Un server MCP fornisce a Claude strumenti da un sistema esterno. Dichiararlo in `.mcp.json` alla radice del plugin, nella stessa forma di un [`.mcp.json` di progetto](/docs/it/mcp#project-scope). Questo `.mcp.json` dichiara un server denominato `db`:

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

Puoi anche omettere il wrapper `mcpServers` e mettere `db` al livello superiore del file.

Carica il plugin ed esegui `/mcp` per confermare che il server appare come `plugin:my-plugin:db`.

`claude plugin validate` controlla `.mcp.json` e segnala una voce di server che Claude Code eliminerebbe al momento del caricamento come errore. Richiede Claude Code v2.1.281 o successivo.

Per dove una voce errata appare al momento del caricamento, vedere [Server MCP che non si avviano](/docs/it/plugins/troubleshooting#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start).

La chiave manifest `mcpServers` accetta una mappa di server inline, un percorso a un file JSON, o un array di quelli. Quando un server manifest ha lo stesso nome di uno in `.mcp.json`, il server manifest lo sostituisce.

<h4 id="reach-users-on-claude-ai-and-cowork">
  Raggiungi gli utenti su claude.ai e Cowork
</h4>

Un server stdio locale, come il server `db` sotto [Server MCP](#mcp-servers), viene eseguito in Claude Code e in una sessione Cowork che viene eseguita sulla tua macchina nell'app Claude Desktop, ma non su claude.ai. Per raggiungerli anche lì, fai riferimento a un server remoto dal suo URL `https://`, che claude.ai e Cowork offrono all'utente come connettore.

<h4 id="server-names-tool-names-and-reloads">
  Nomi dei server, nomi degli strumenti e ricaricamenti
</h4>

I nomi del server, la sostituzione delle variabili e il comportamento di ricaricamento seguono queste regole:

* **Nome del server**: `plugin:<plugin>:<server>`, quindi il server `db` in `my-plugin` è `plugin:my-plugin:db` in `/mcp`. Usa la stessa forma per nominare il server in un [hook `mcp_tool`](/docs/it/hooks#mcp-tool-hook-fields)
* **Nomi degli strumenti**: `mcp__plugin_<plugin>_<server>__<tool>`, quindi uno strumento `query` su quel server `db` è `mcp__plugin_my-plugin_db__query`. Questo è il nome da usare in [regole di permesso](/docs/it/permissions) e [matcher di hook](#hooks)
* **Sostituzione**: `${CLAUDE_PLUGIN_ROOT}` e le altre [variabili di percorso](#path-variables-and-persistent-data) vengono sostituite in `command`, `args`, e `env`. Non è necessario quoting in `args`, perché ogni elemento viene passato come un argomento
* **Ricaricamento**: quando l'utente esegue `/reload-plugins` e [il ricaricamento si applica](/docs/it/plugins/cli-reference#reloads-that-change-mcp-tools), un server la cui configurazione è invariata mantiene la sua connessione. Un server la cui configurazione è cambiata si riconnette, e uno che hai rimosso si disconnette

<h4 id="include-a-packaged-mcpb-server">
  Includi un server MCPB in pacchetto
</h4>

La chiave `mcpServers` accetta anche un server in pacchetto come file [MCPB](https://github.com/modelcontextprotocol/mcpb), la cui estensione è `.mcpb` o la più vecchia `.dxt`. Punta la chiave al file, come percorso all'interno del plugin o un URL `https://`:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "my-plugin",
  "mcpServers": "./servers/db.mcpb"
}
```

Il server prende il suo nome da `name` nel manifest del bundle.

Per trasporti e autenticazione, vedere [MCP](/docs/it/mcp#plugin-provided-mcp-servers).

<h3 id="lsp-servers">
  Server LSP
</h3>

Un server LSP fornisce a Claude diagnostica e navigazione del codice per un linguaggio. Se un [plugin ufficiale di code intelligence](/docs/it/plugins/code-intelligence) copre già il tuo linguaggio, installa quello invece di scriverne uno. Altrimenti dichiara il server in `.lsp.json` alla radice del plugin:

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

Il file mappa ogni nome di server direttamente alla sua configurazione, senza oggetto wrapper attorno alla mappa. `command` è il nome del binario, con i suoi argomenti in `args`. `extensionToLanguage` ha bisogno di almeno un'estensione, ognuna che inizia con `.`.

`claude plugin validate` non legge questo file. Quando una voce è non valida, l'intero file viene saltato al caricamento e `Invalid LSP server config for ".lsp.json"` appare nella scheda **Errors** di `/plugin`.

Il tuo plugin configura la connessione ma non installa il binario del server, e ogni estensione di file ottiene un server:

* **Binario mancante**: Claude Code avvia `command` per nome dal `PATH` dell'utente. Quando il binario non è lì, il server non si avvia e `claude --debug` registra `LSP server <name> failed to start`
* **Conflitti di estensione**: quando due server abilitati rivendicano la stessa estensione, il primo registrato gestisce quei file e l'altro non viene utilizzato per loro, che i server provengano da un plugin o da due. La scheda **Errors** di `/plugin` mostra l'avviso `LSP server "<name>" is not used for <ext> files`

La chiave manifest `lspServers` accetta la stessa mappa inline, un percorso a un file JSON, o un array di quelli, e i suoi server si aggiungono a quelli in `.lsp.json`. Quando un server manifest ha lo stesso nome di uno in `.lsp.json`, il server manifest lo sostituisce.

Per `transport`, timeout, riavvii e gli altri campi, vedere [`lspServers`](/docs/it/plugins/manifest-reference#lspservers).

Invia l'output del log a stderr, non a stdout. Claude Code legge stdout di un server solo come messaggi di protocollo, e accetta intestazioni di messaggi fino a 64 KiB e un corpo di messaggio fino a 32 MiB.

Claude Code disconnette un server che supera uno dei due limiti o scrive output non-protocollo a stdout, e conta la disconnessione come un crash per `restartOnCrash` e `maxRestarts`. Quando esegui con `--debug`, Claude Code scrive un errore che nomina la causa al log di debug.

<h3 id="executables">
  Eseguibili
</h3>

I file in `bin/` alla radice del plugin sono su `PATH` della shell dello strumento Bash mentre il plugin è abilitato, quindi Claude può eseguirli come comandi nudi. Aggiungi uno script eseguibile:

```bash bin/hello-plugin theme={null}
#!/bin/bash
echo "hello from my-plugin"
```

Rendilo eseguibile con `chmod +x bin/hello-plugin` e carica il plugin. Quando chiedi a Claude di eseguire `hello-plugin`, il risultato dello strumento Bash mostra l'output dello script.

Le directory `bin/` del plugin vengono dopo le voci `PATH` dell'utente, quindi un plugin non può oscurare `git`, `ls`, o un altro comando di sistema.

claude.ai e Cowork non installano un plugin che ha una directory `bin/` di livello superiore, incluso uno che [distribuisci attraverso le impostazioni dell'organizzazione claude.ai](/docs/it/plugins/host-marketplace#distribute-through-organization-settings).

<h3 id="default-settings">
  Impostazioni predefinite
</h3>

Per impostare i valori predefiniti che si applicano mentre il plugin è abilitato, aggiungi un `settings.json` alla radice del plugin, o metti lo stesso oggetto inline nella chiave manifest `settings`. Due chiavi hanno effetto, `agent` e `subagentStatusLine`, e ogni altra chiave viene eliminata.

Imposta `agent` per eseguire uno dei propri agenti del plugin come thread principale:

```json settings.json theme={null}
{
  "agent": "security-reviewer"
}
```

Carica il plugin e avvia una sessione. Claude quindi risponde nella conversazione principale con il prompt di sistema e il modello dell'agente `security-reviewer`.

Per tutto quello che la chiave controlla, vedere l'[impostazione `agent`](/docs/it/settings-reference#agent).

Quando la stessa chiave è impostata in più di un posto, queste regole decidono quale valore si applica:

* **File su manifest**: quando entrambi esistono e `settings.json` imposta almeno una chiave supportata, `settings.json` si applica e il `settings` del manifest viene ignorato
* **Impostazioni utente su valori predefiniti del plugin**: tra le fonti di impostazioni, i valori predefiniti del plugin sono il livello più basso, quindi un `agent` proprio dell'utente in `~/.claude/settings.json` sostituisce il tuo
* **Due plugin impostano la stessa chiave**: il valore dal plugin caricato per ultimo si applica, e `claude --debug` registra `overrides setting`

Per la forma `subagentStatusLine`, vedere [linee di stato del subagente](/docs/it/statusline#subagent-status-lines).

<h3 id="themes-and-output-styles">
  Temi e stili di output
</h3>

Un plugin può includere temi di colore e stili di output. Entrambi appaiono negli stessi picker dei propri dell'utente. Per uno qualsiasi, impostare la chiave manifest sostituisce la scansione della cartella.

| Componente      | Salva come                | Formato                                                                                                                             | Appare in                               | Chiave manifest       |
| :-------------- | :------------------------ | :---------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------- | :-------------------- |
| Tema            | `themes/<slug>.json`      | Il formato [file tema personalizzato](/docs/it/terminal-config#create-a-custom-theme) che gli utenti scrivono in `~/.claude/themes/`     | `/theme`, sotto il `name` del file      | `experimental.themes` |
| Stile di output | `output-styles/<name>.md` | Il formato [stile di output personalizzato](/docs/it/output-styles#create-a-custom-output-style), con frontmatter `name` e `description` | `/output-style`, come `<plugin>:<name>` | `outputStyles`        |

I temi del plugin sono di sola lettura, quindi quando un utente ne modifica uno in `/theme`, la modifica viene salvata come copia nella sua directory di temi.

Questo tema ricolora il prompt di accento e il testo di errore sul preset scuro:

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
  Canali
</h3>

Un [canale](/docs/it/channels) consente a un sistema esterno come un'app di chat di inviare messaggi in una sessione. In un plugin, un canale è uno dei server MCP più una voce `channels` che si lega ad esso e può richiedere la sua configurazione. Questo manifest lega un canale a un server `telegram` e chiede un token bot:

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

`server` deve corrispondere a una chiave in `mcpServers`. Il `userConfig` per canale accetta la stessa forma della chiave [`userConfig`](#user-configuration) di livello superiore.

Per quello che il server deve implementare e come gli utenti abilitano un plugin di canale, vedere [Pacchetto come plugin](/docs/it/channels-reference#package-as-a-plugin) nel riferimento dei canali. Per la tabella dei campi, vedere [`channels`](/docs/it/plugins/manifest-reference#channels).

<h3 id="monitors">
  Monitor
</h3>

Un monitor è un comando shell che viene eseguito in background per l'intera sessione. Quello che stampa raggiunge Claude come notifiche, quindi Claude può reagire a un log o a un cambio di stato senza essere chiesto di guardarlo. Salvare le voci in `monitors/monitors.json`:

```json monitors/monitors.json theme={null}
[
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Application error log"
  }
]
```

Il comando viene eseguito in una shell, nella directory di lavoro in cui la sessione è stata avviata.

Il comando di un monitor è limitato in dove inizia e cosa può fare riferimento:

* **Solo sessioni interattive**: i monitor del plugin si avviano in una sessione interattiva e mai in modalità non interattiva con il flag `-p`. Si avviano anche solo dove lo [strumento Monitor](/docs/it/tools-reference#monitor-tool) è disponibile
* **Nessuna configurazione utente**: `command` ottiene le [variabili di percorso](#path-variables-and-persistent-data) e `${ENV_VAR}` dall'ambiente, ma mai `${user_config.*}`. Un monitor che fa riferimento a uno non si avvia, e i processi monitor non ricevono nemmeno `CLAUDE_PLUGIN_OPTION_<KEY>`
* **Disabilitazione a metà sessione**: se disabiliti un plugin a metà sessione, Claude Code non ferma i monitor che sono già in esecuzione. Si fermano quando la sessione finisce

La chiave manifest `experimental.monitors` accetta lo stesso array inline o un percorso a un file JSON, e viene letta invece di `monitors/monitors.json`.

Per il trigger `when` e gli altri campi, vedere [`monitors`](/docs/it/plugins/manifest-reference#monitors).

<h2 id="user-configuration">
  Chiedi all'utente i valori di configurazione
</h2>

Dichiara i valori di cui il tuo plugin ha bisogno dall'utente nella chiave manifest `userConfig`, in modo che gli utenti non modifichino `settings.json` da soli. Ogni opzione appare in una finestra di dialogo con il suo `title` come etichetta e la sua `description` sotto.

Imposta `"sensitive": true` per un token o una password. La finestra di dialogo quindi maschera l'input, e il valore viene archiviato in archiviazione sicura piuttosto che in `settings.json`.

Questo manifest chiede un endpoint e un token:

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
  Quando appare la finestra di dialogo di configurazione
</h3>

La finestra di dialogo appare solo nell'interfaccia interattiva `/plugin`. Si apre per qualsiasi opzione che non è ancora impostata quando l'utente fa uno dei seguenti:

* Installa il plugin in `/plugin`
* Esegui `/plugin install <plugin>@<marketplace>` all'interno di una sessione
* Abilita il plugin dalla scheda **Installed** in `/plugin`

Per aprire la stessa finestra di dialogo in qualsiasi momento, l'utente esegue `/plugin configure <plugin>@<marketplace>`.

Il comando shell `claude plugin install` non richiede mai i valori `userConfig`. Per impostare i valori dalla shell, passa ognuno come `--config KEY=VALUE`. Quando le opzioni rimangono non impostate, il comando stampa una riga `userConfig options not yet set` che nomina entrambi i modi per impostarli. [La finestra di dialogo `userConfig` non appare mai](/docs/it/plugins/troubleshooting#the-userconfig-dialog-never-appears) cita la riga.

Per i campi dell'opzione, dove ogni valore viene archiviato, come un componente fa riferimento a un valore salvato, e quali campi rifiutano `${user_config.*}`, vedere [Configurazione utente](/docs/it/plugins/manifest-reference#user-configuration).

<h2 id="path-variables-and-persistent-data">
  Fai riferimento ai percorsi del plugin e archivia i dati
</h2>

Non sai dove il tuo plugin verrà installato, quindi fai riferimento ai suoi file e dati attraverso queste variabili piuttosto che percorsi fissi. Vengono sostituite nel contenuto di skill, comando e agente, nei comandi di hook e monitor, e nelle configurazioni di server MCP e LSP. Vengono anche esportate ai processi hook, MCP e LSP:

* **`${CLAUDE_PLUGIN_ROOT}`**: la directory di installazione del plugin. Ogni versione ha la sua [directory cache](/docs/it/plugins/loading#find-plugins-on-disk), quindi il percorso cambia quando il plugin si aggiorna. Non scrivere stato lì
* **`${CLAUDE_PLUGIN_DATA}`**: una directory che sopravvive agli aggiornamenti, per `node_modules`, ambienti virtuali e cache. Si risolve in `~/.claude/plugins/data/<id>/` e viene creata quando viene referenziata per la prima volta
* **`${CLAUDE_PROJECT_DIR}`**: la radice del progetto, lo stesso valore che gli hook ricevono

Nel percorso della directory dei dati, `<id>` è l'identificatore del plugin con ogni carattere diverso da lettere, cifre, `_`, e `-` sostituito da `-`, quindi `my-plugin@my-marketplace` diventa `my-plugin-my-marketplace`.

Su Windows, i percorsi sostituiti utilizzano barre in avanti in modo che una shell non legga le barre rovesciate come escape.

<h3 id="install-dependencies-into-the-data-directory">
  Installa le dipendenze nella directory dei dati
</h3>

Per un plugin installato dal marketplace, Claude Code installa automaticamente le [dipendenze di pacchetti Node.js](/docs/it/plugins/loading#node-js-package-dependencies) idonee quando memorizza il plugin nella cache, quindi potresti non aver bisogno di installarle tu stesso. Quando lo fai, questo hook `SessionStart` installa `node_modules` in `${CLAUDE_PLUGIN_DATA}` alla prima esecuzione e di nuovo dopo che un aggiornamento cambia `package.json`:

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

Dopo la prima sessione, `~/.claude/plugins/data/<id>/node_modules` esiste. Un server MCP può quindi impostare `NODE_PATH` a `${CLAUDE_PLUGIN_DATA}/node_modules` nel suo `env`. Per quali campi sostituiscono quale variabile, vedere [Variabili di ambiente](/docs/it/plugins/manifest-reference#environment-variables).

<h2 id="next-steps">
  Passaggi successivi
</h2>

* [Riferimento manifest del plugin](/docs/it/plugins/manifest-reference): campi `plugin.json`, regole di percorso e layout standard
* [Testa i plugin con evals](/docs/it/plugin-evals): controlla che i componenti che hai aggiunto cambino il comportamento di Claude nel modo che intendi
* [Pubblica e distribuisci un plugin](/docs/it/plugins/publish): versiona il plugin e mettilo in un marketplace
* [Risolvi i problemi dei plugin](/docs/it/plugins/troubleshooting): cosa fare quando un componente non carica o un hook non si attiva
