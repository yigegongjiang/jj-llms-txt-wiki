> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agregar componentes a un plugin

> Agrega skills, hooks, servidores MCP y todos los demás tipos de componentes a un plugin de Claude Code, con un ejemplo que valida cada uno.

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

Un plugin de Claude Code se construye a partir de componentes, como skills, agentes, hooks y servidores MCP. Cada componente tiene una carpeta predeterminada en el plugin, una clave de manifiesto opcional en `.claude-plugin/plugin.json` que reemplaza o se suma a esa carpeta, y un nombre que ve el usuario. Para la tabla de campos completa de cada clave, consulte la [referencia de manifiesto](/docs/es/plugins/manifest-reference#fields).

Utilice esta página para agregar un componente a un plugin que ya se carga.

Después de agregar un componente, ejecute `/reload-plugins` en una sesión en ejecución o inicie una nueva para que Claude Code lo cargue. Para verificar el archivo del componente antes de cargarlo, ejecute [`claude plugin validate .`](/docs/es/plugins/cli-reference#plugin-validate) en su shell desde el directorio del plugin.

<Note>
  Estos casos se tratan en otras páginas:

  * **Construir su primer plugin**: comience con [Crear un plugin](/docs/es/plugins/create)
  * **Instalar el plugin de otra persona**: consulte [Instalar plugins](/docs/es/plugins/install)
  * **Los usuarios de su plugin están en claude.ai o en Cowork**: se carga un conjunto diferente de componentes allí. Consulte [Plugins en claude.ai y en Cowork](https://claude.com/docs/plugins/overview)
</Note>

<h2 id="explore-the-plugin-directory">
  Explorar el directorio del plugin
</h2>

El explorador muestra un plugin de ejemplo, `my-plugin`, que tiene uno de cada tipo de componente en su ubicación predeterminada:

* Una skill de revisión y un comando `about`
* Un subagente de revisión de seguridad
* Un hook que formatea archivos después de que Claude los edita, y la carpeta `scripts/` que llama
* Un monitor de registro
* Un estilo de salida y un tema de color
* Un flujo de trabajo de auditoría de rutas
* Un ejecutable `hello-plugin`
* Configuración predeterminada
* Un servidor MCP local y un servidor de lenguaje Go

Cada archivo es el ejemplo válido más pequeño de su formato, presente para mostrar la forma en lugar de ser útil: una skill o agente real lleva instrucciones completas y a menudo archivos de apoyo, y un hook o monitor real realiza trabajo real. Las secciones después del explorador utilizan los mismos archivos que sus ejemplos y enlazan a otros más completos. Seleccione un archivo o carpeta para leer para qué sirve, ver qué va en él y encontrar la sección que lo cubre.

<PluginExplorer>
  <Piece id="manifest">
    El [manifiesto](/docs/es/plugins/manifest-reference) es el archivo `plugin.json` en el directorio `.claude-plugin/` de un plugin. Contiene los metadatos del plugin y los valores de `userConfig` que Claude Code solicita al usuario. Solo `name` es obligatorio. En este, `description` es el texto que los usuarios ven para el plugin en `/plugin`, y `version` mantiene a los usuarios en esa versión hasta que la cambie:

    ```json theme={null}
    {
      "name": "my-plugin",
      "version": "1.0.0",
      "description": "Review, formatting, and database tools for this team"
    }
    ```
  </Piece>

  <Piece id="skills">
    Una [skill](/docs/es/skills) es un archivo `SKILL.md`. Guarde cada skill en su propio directorio bajo `skills/`. Claude lee la `description` de cada skill, y cuando lo que el usuario pregunta coincide con ella, como pedirle a Claude que revise una solicitud de extracción aquí, Claude carga las instrucciones de la skill y las sigue. El usuario también puede ejecutarla directamente como `/my-plugin:review`:

    ```markdown theme={null}
    ---
    description: Reviews a pull request for style and test coverage. Use when asked to review code.
    ---

    Review the changed files. Report style problems first, then missing tests.
    ```
  </Piece>

  <Piece id="commands">
    Un comando es un único archivo Markdown que el usuario ejecuta por nombre. Los comandos son el formato anterior: una skill se ejecuta por nombre de la misma manera y también puede llevar archivos de apoyo en su propio directorio, así que escriba los nuevos como skills y mantenga `commands/` para los archivos que ya tiene. Este archivo se convierte en `/my-plugin:about` y toma el mismo frontmatter que una skill:

    ```markdown theme={null}
    ---
    description: Summarize the repository
    ---

    Summarize what this repository does in three sentences.
    ```
  </Piece>

  <Piece id="agents">
    Un [subagente](/docs/es/sub-agents) es un asistente separado, con sus propias instrucciones y su propia ventana de contexto, al que Claude puede delegar una tarea y obtener un resultado. Cada archivo Markdown bajo `agents/` define uno: el frontmatter lo nombra y dice cuándo usarlo, y el cuerpo es su indicación del sistema. Este se llama `my-plugin:security-reviewer`, y el usuario puede invocarlo con `@agent-my-plugin:security-reviewer`:

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
    Un [hook](/docs/es/hooks-guide) ejecuta algo automáticamente en un punto del ciclo de vida de Claude Code, como después de cada edición de archivo: un comando de shell, una solicitud HTTP, una llamada a herramienta MCP, un indicador a un modelo o un subagente. Guarde los hooks del plugin en `hooks/hooks.json` en la raíz del plugin. Este ejecuta el script `scripts/format.sh` del plugin después de que Claude escribe o edita un archivo:

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
    Un monitor es un comando de shell que Claude Code inicia en segundo plano cuando la sesión comienza y mantiene en ejecución hasta que termina, utilizando la [herramienta Monitor](/docs/es/tools-reference#monitor-tool). Lo que imprime llega a Claude como notificaciones. Un campo `when` puede iniciarlo la primera vez que se ejecuta una skill nombrada. Este rastrea un registro de errores:

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
    Un plugin puede incluir [estilos de salida](/docs/es/output-styles), que cambian cómo Claude formatea y expresa sus respuestas. Guarde cada estilo de salida como `output-styles/<name>.md`. Este aparece en `/output-style` como `my-plugin:terse`:

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
    Un plugin puede incluir [temas de color](/docs/es/terminal-config#create-a-custom-theme) para la interfaz de Claude Code. Guarde cada tema como `themes/<slug>.json`. Este aparece en `/theme` como `Dracula`, marcado como de `my-plugin`:

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
    La carpeta `workflows/` contiene archivos [workflow](/docs/es/workflows) `.js`: un bloque `meta`, luego un cuerpo de script que orquesta varios subagentes. Este se ejecuta como `/my-plugin:audit-routes`:

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
    `bin/` es cómo un plugin envía una herramienta de línea de comandos. Mientras el plugin está habilitado, Claude Code coloca esta carpeta en el `PATH` del shell en el que ejecuta comandos, para que Claude, o las instrucciones de una skill, puedan ejecutar la herramienta por nombre sin que el usuario instale nada. Con este [ejecutable](#executables) en su lugar, `hello-plugin` es un comando que Claude puede ejecutar:

    ```bash theme={null}
    #!/bin/bash
    echo "hello from my-plugin"
    ```
  </Piece>

  <Piece id="scripts">
    El hook en `hooks/hooks.json` ejecuta un script, y esta carpeta es donde el ejemplo lo mantiene. El nombre `scripts/` es una convención, no algo que Claude Code busque: el hook apunta al archivo por su ruta, `${CLAUDE_PLUGIN_ROOT}/scripts/format.sh`. Un script de formateador podría verse así:

    ```bash theme={null}
    #!/bin/bash
    npx prettier --write .
    ```
  </Piece>

  <Piece id="settings">
    Un `settings.json` en la raíz del plugin contiene [configuración](/docs/es/settings-reference) que se aplica mientras el plugin está habilitado, para que un plugin pueda cambiar cómo se comporta la sesión y no solo agregar componentes. Solo dos claves tienen efecto desde un plugin, [`agent`](/docs/es/settings-reference#agent) y [`subagentStatusLine`](/docs/es/settings-reference#subagentstatusline); todas las demás claves se descartan. Consulte [Configuración predeterminada](#default-settings).

    Este establece `agent`, que ejecuta el hilo principal de la sesión como el agente `security-reviewer` del plugin, para que el indicador del sistema de ese agente, las restricciones de herramientas y el modelo se apliquen a toda la sesión:

    ```json theme={null}
    {
      "agent": "security-reviewer"
    }
    ```
  </Piece>

  <Piece id="mcp">
    Un [servidor MCP](/docs/es/mcp) proporciona a Claude herramientas de un sistema externo. Declárelo en `.mcp.json` en la raíz del plugin. Este inicia un servidor local desde un script dentro del plugin y aparece en `/mcp` como `plugin:my-plugin:db`:

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
    Un servidor LSP proporciona a Claude [diagnósticos y navegación de código](/docs/es/plugins/code-intelligence) para un lenguaje. Declare el servidor en `.lsp.json` en la raíz del plugin. Este conecta el servidor de lenguaje Go para archivos `.go`:

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
  Agregar cada tipo de componente
</h2>

Cada sección a continuación cubre un tipo de componente: dónde van sus archivos en el plugin, un ejemplo que valida, qué ve el usuario una vez que se carga el plugin, y la clave de manifiesto que cambia la ubicación predeterminada. Agregue los que su plugin necesite; ninguno es obligatorio.

<h3 id="skills">
  Skills
</h3>

Una [skill](/docs/es/skills) es un archivo `SKILL.md` que Claude puede cargar cuando su descripción coincide con la tarea. El usuario también puede ejecutarla como un comando. Guarde cada skill en su propio directorio bajo `skills/`:

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json
└── skills/
    └── review/
        └── SKILL.md
```

Dé al `SKILL.md` una `description` para que Claude sepa cuándo usarla:

```markdown skills/review/SKILL.md theme={null}
---
description: Reviews a pull request for style and test coverage. Use when asked to review code.
---

Review the changed files. Report style problems first, then missing tests.
```

Después de cargar el plugin, `/my-plugin:review` ejecuta la skill. El nombre del comando y quién puede invocarlo siguen estas reglas:

* **Nombre del comando**: `/<plugin>:<directory>`, así que `skills/review/SKILL.md` en `my-plugin` es `/my-plugin:review`. Si establece `name` en el frontmatter, reemplaza el último segmento y el prefijo del plugin permanece. Consulte [cómo una skill obtiene su nombre de comando](/docs/es/skills#how-a-skill-gets-its-command-name)
* **Quién la invoca**: Claude, el usuario, o ambos, controlado por frontmatter. Consulte [Controlar quién invoca una skill](/docs/es/skills#control-who-invokes-a-skill)

También puede colocar skills fuera del directorio predeterminado `skills/`:

* **Directorios adicionales**: enumérelos en la clave de manifiesto `skills`. Se suman al escaneo predeterminado de `skills/` en lugar de reemplazarlo, a diferencia de `commands` y `agents`
* **Una única skill en la raíz del plugin**: sin directorio `skills/` y sin clave de manifiesto `skills`, un `SKILL.md` en la raíz del plugin se carga como una skill. Establezca `name` en su frontmatter, porque de lo contrario una instalación de marketplace nombra la skill después de su [directorio de caché](/docs/es/plugins/loading#find-plugins-on-disk) en lugar de su plugin

Para incluir instrucciones en un plugin, escríbalas como una skill. Claude Code no carga un `CLAUDE.md` en la raíz del plugin, y `claude plugin validate` advierte `CLAUDE.md at the plugin root is not loaded as project context`.

Para campos de frontmatter y archivos de apoyo, consulte [Skills](/docs/es/skills).

<h3 id="commands">
  Comandos
</h3>

Un comando es un único archivo Markdown que el usuario ejecuta por nombre, como `/my-plugin:about`.

<Note>
  Los comandos son el formato anterior, y [skills](#skills) los reemplazan para trabajo nuevo. Una skill se ejecuta por nombre de la misma manera, y también puede llevar archivos de apoyo en su directorio. Mantenga `commands/` para archivos que está moviendo desde `.claude/commands/`.
</Note>

Guarde un comando en `commands/<file>.md` y se convierte en `/<plugin>:<file>`. Un subdirectorio agrega un segmento, así que `commands/db/migrate.md` es `/my-plugin:db:migrate`.

Los archivos de comando toman el mismo frontmatter que las skills.

<h4 id="define-commands-in-the-manifest">
  Definir comandos en el manifiesto
</h4>

Solo necesita esto si desea mantener archivos de comando en algún lugar que no sea `commands/`, o para definir un comando corto dentro de `plugin.json` sin un archivo Markdown separado. Establezca la clave de manifiesto `commands`, y Claude Code la lee en lugar de escanear `commands/`. La clave toma una ruta, una matriz de rutas, u un objeto que asigna cada nombre de comando a un archivo `source` o contenido `content` en línea.

Este manifiesto define `/my-plugin:about` en línea, sin archivo Markdown:

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

Cargue el plugin y ejecute `/my-plugin:about` en la sesión para confirmar que se cargó.

Para la sintaxis completa de la clave, consulte [`commands`](/docs/es/plugins/manifest-reference#commands).

<h3 id="agents">
  Agentes
</h3>

Un [subagente](/docs/es/sub-agents) es un asistente separado, con sus propias instrucciones y ventana de contexto, al que Claude puede delegar una tarea. Cada archivo Markdown bajo `agents/` define uno:

```markdown agents/security-reviewer.md theme={null}
---
name: security-reviewer
description: Reviews code changes for security issues. Use after edits to authentication or input handling.
model: sonnet
---

You are a security reviewer. Read the changed files and report injection, authentication, and secrets-handling risks.
```

Este agente se llama `my-plugin:security-reviewer`, y el usuario puede [invocarlo explícitamente](/docs/es/sub-agents#invoke-subagents-explicitly) con `@agent-my-plugin:security-reviewer`. La forma del nombre es `<plugin>:<name>`, donde `<name>` viene del frontmatter, o del nombre del archivo cuando no hay ninguno.

La clave de manifiesto `agents` reemplaza el escaneo de `agents/`.

<h4 id="organize-agents-in-subfolders">
  Organizar agentes en subcarpetas
</h4>

Puede colocar archivos de agente del plugin en subcarpetas de `agents/`. Claude Code [los carga recursivamente](/docs/es/sub-agents#choose-the-subagent-scope) y une el nombre del plugin, cada nombre de subcarpeta y el nombre del archivo con dos puntos para formar el nombre con alcance del agente. Por ejemplo, `agents/review/security.md` en un plugin llamado `my-plugin` se carga como `my-plugin:review:security`. Dos configuraciones cambian ese nombre:

* Frontmatter `name`: reemplaza solo el nombre del archivo, así que `name: audit` en `agents/review/security.md` se carga como `my-plugin:review:audit`
* Campo de manifiesto [`agents`](/docs/es/plugins/manifest-reference#fields): un archivo que enumera allí se carga sin nombres de subcarpeta, así que `"agents": "./custom/review/security.md"` se carga como `my-plugin:security`

<h4 id="frontmatter-fields-in-plugin-agents">
  Campos de frontmatter en agentes de plugin
</h4>

El frontmatter de un agente de plugin sigue estas reglas:

* **Campos admitidos**: `name`, `description`, `model`, `effort`, `maxTurns`, `tools`, `disallowedTools`, `skills`, `memory`, `background`, `omitClaudeMd`, `isolation`, `color`, y la clave `cacheTtl` de `experimental`. El único valor válido de `isolation` es `"worktree"`. Consulte [campos de frontmatter admitidos](/docs/es/sub-agents#supported-frontmatter-fields) para ver qué hace cada uno
* **Campos ignorados**: `permissionMode`, `hooks`, `mcpServers`, e `initialPrompt`. Un archivo de agente no puede agregar hooks o servidores MCP por su cuenta, así que agregue esos como plugin [hooks](#hooks) y [servidores MCP](#mcp-servers) en su lugar
* **Frontmatter que no se analiza**: el agente aún se carga con cada campo ignorado. Se nombra después del archivo, y su descripción dice `Agent from my-plugin plugin`. Ejecute [`claude plugin validate`](/docs/es/plugins/cli-reference#plugin-validate) en su shell para encontrar estos archivos

Para ver qué hace cada campo y las reglas de precedencia, consulte [Subagentes](/docs/es/sub-agents#supported-frontmatter-fields).

<h3 id="hooks">
  Hooks
</h3>

Un [hook](/docs/es/hooks-guide) ejecuta algo automáticamente en un punto del ciclo de vida de Claude Code, como después de cada edición de archivo: un comando de shell, una solicitud HTTP, una llamada a herramienta MCP, un indicador a un modelo o un subagente. Guarde los hooks del plugin en `hooks/hooks.json` en la raíz del plugin, bajo una clave `"hooks"` de nivel superior, en la misma forma que el objeto `hooks` en `settings.json`. Eso le permite copiar un hook de configuración existente sin cambios.

Este hook ejecuta un script incluido después de cada `Write` o `Edit`:

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

Guarde el script en `scripts/format.sh` y hágalo ejecutable.

Cargue el plugin y pida a Claude que edite un archivo. Un hook `PostToolUse` que sale con 0 no muestra nada en la transcripción, así que confirme que se ejecutó con [registro de depuración](/docs/es/hooks#debug-hooks) o por lo que el script mismo cambia.

Los hooks en `hooks/hooks.json` y en la clave de manifiesto `hooks` se cargan ambos. Para cada evento y su carga útil, consulte [Eventos de hook](/docs/es/hooks#hook-events).

<h4 id="when-plugin-hooks-fire">
  Cuándo se activan los hooks del plugin
</h4>

Los hooks de un plugin no esperan a que se use una de las skills o comandos del plugin. Claude Code los registra cuando una sesión carga el plugin, y se activan en sus eventos a partir de entonces. Para limitar cuándo se ejecuta un hook, reduzca su `matcher`.

Si un hook nunca se activa, consulte [hooks que no se activan](/docs/es/plugins/troubleshooting#failed-to-load-hooks-from-and-hooks-that-dont-fire).

<h4 id="environment-quoting-and-matching-mcp-tools">
  Entorno, entrecomillado y coincidencia de herramientas MCP
</h4>

El entorno del hook, el entrecomillado de `${CLAUDE_PLUGIN_ROOT}` y los matchers para las herramientas MCP propias del plugin funcionan de la siguiente manera:

* **Entorno**: cada proceso de hook recibe `CLAUDE_PLUGIN_ROOT` y `CLAUDE_PLUGIN_DATA` en su entorno, más `CLAUDE_PLUGIN_OPTION_<KEY>` para cada valor de [configuración del usuario](#user-configuration), para que su script pueda leerlos desde allí
* **Entrecomillado**: cuando `command` no tiene `args`, se ejecuta a través de un shell, así que envuelva la ruta `${CLAUDE_PLUGIN_ROOT}` entre comillas dobles, como hace el ejemplo `hooks/hooks.json` bajo [Hooks](#hooks), para mantener la ruta expandida como una palabra de shell. Cuando pasa `args` en su lugar, cada elemento se pasa como un argumento sin shell y no necesita entrecomillado. Consulte [forma exec y forma shell](/docs/es/hooks#exec-form-and-shell-form)
* **Coincidencia de las herramientas MCP propias del plugin**: una herramienta de un [servidor MCP que este plugin declara](#mcp-servers) se llama `mcp__plugin_<plugin>_<server>__<tool>`, así que escriba ese nombre completo en el matcher. Un matcher solo en el nombre del servidor nunca se activa. Consulte [Coincidir herramientas MCP](/docs/es/hooks#match-mcp-tools)

<h3 id="mcp-servers">
  Servidores MCP
</h3>

Un servidor MCP proporciona a Claude herramientas de un sistema externo. Declárelo en `.mcp.json` en la raíz del plugin, en la misma forma que un [`.mcp.json` de proyecto](/docs/es/mcp#project-scope). Este `.mcp.json` declara un servidor llamado `db`:

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

También puede omitir el contenedor `mcpServers` y poner `db` en el nivel superior del archivo.

Cargue el plugin y ejecute `/mcp` para confirmar que el servidor aparece como `plugin:my-plugin:db`.

`claude plugin validate` verifica `.mcp.json` e informa una entrada de servidor que Claude Code descartaría en el tiempo de carga como un error. Requiere Claude Code v2.1.281 o posterior.

Para ver dónde aparece una entrada incorrecta en el tiempo de carga, consulte [Servidores MCP que no se inician](/docs/es/plugins/troubleshooting#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start).

La clave de manifiesto `mcpServers` toma un mapa de servidor en línea, una ruta a un archivo JSON, o una matriz de esos. Cuando un servidor de manifiesto tiene el mismo nombre que uno en `.mcp.json`, el servidor de manifiesto lo reemplaza.

<h4 id="reach-users-on-claude-ai-and-cowork">
  Alcanzar usuarios en claude.ai y Cowork
</h4>

Un servidor stdio local, como el servidor `db` bajo [Servidores MCP](#mcp-servers), se ejecuta en Claude Code y en una sesión de Cowork que se ejecuta en su máquina en la aplicación Claude Desktop, pero no en claude.ai. Para alcanzar a los usuarios allí también, haga referencia a un servidor remoto por su URL `https://`, que claude.ai y Cowork ofrecen al usuario como un conector.

<h4 id="server-names-tool-names-and-reloads">
  Nombres de servidor, nombres de herramientas y recargas
</h4>

Los nombres del servidor, la sustitución de variables y el comportamiento de recarga siguen estas reglas:

* **Nombre del servidor**: `plugin:<plugin>:<server>`, así que el servidor `db` en `my-plugin` es `plugin:my-plugin:db` en `/mcp`. Use la misma forma para nombrar el servidor en un [hook `mcp_tool`](/docs/es/hooks#mcp-tool-hook-fields)
* **Nombres de herramientas**: `mcp__plugin_<plugin>_<server>__<tool>`, así que una herramienta `query` en ese servidor `db` es `mcp__plugin_my-plugin_db__query`. Ese es el nombre a usar en [reglas de permisos](/docs/es/permissions) y [matchers de hook](#hooks)
* **Sustitución**: `${CLAUDE_PLUGIN_ROOT}` y las otras [variables de ruta](#path-variables-and-persistent-data) se sustituyen en `command`, `args` y `env`. No se necesita entrecomillado en `args`, porque cada elemento se pasa como un argumento
* **Recarga**: cuando el usuario ejecuta `/reload-plugins` y [la recarga se aplica](/docs/es/plugins/cli-reference#reloads-that-change-mcp-tools), un servidor cuya configuración no ha cambiado mantiene su conexión. Un servidor cuya configuración cambió se reconecta, y uno que eliminó se desconecta

<h4 id="include-a-packaged-mcpb-server">
  Incluir un servidor MCPB empaquetado
</h4>

La clave `mcpServers` también acepta un servidor empaquetado como un [archivo MCPB](https://github.com/modelcontextprotocol/mcpb), cuya extensión es `.mcpb` o la anterior `.dxt`. Apunte la clave al archivo, como una ruta dentro del plugin o una URL `https://`:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "my-plugin",
  "mcpServers": "./servers/db.mcpb"
}
```

El servidor toma su nombre del `name` en el manifiesto del paquete.

Para transportes y autenticación, consulte [MCP](/docs/es/mcp#plugin-provided-mcp-servers).

<h3 id="lsp-servers">
  Servidores LSP
</h3>

Un servidor LSP proporciona a Claude diagnósticos y navegación de código para un lenguaje. Si un [plugin oficial de inteligencia de código](/docs/es/plugins/code-intelligence) ya cubre su lenguaje, instale ese en su lugar de escribir uno. De lo contrario, declare el servidor en `.lsp.json` en la raíz del plugin:

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

El archivo asigna cada nombre de servidor directamente a su configuración, sin un objeto contenedor alrededor del mapa. `command` es el nombre del binario, con sus argumentos en `args`. `extensionToLanguage` necesita al menos una extensión, cada una comenzando con `.`.

`claude plugin validate` no lee este archivo. Cuando cualquier entrada es inválida, todo el archivo se omite en la carga y `Invalid LSP server config for ".lsp.json"` aparece en la pestaña **Errors** de `/plugin`.

Su plugin configura la conexión pero no instala el binario del servidor, y cada extensión de archivo obtiene un servidor:

* **Binario faltante**: Claude Code inicia `command` por nombre desde el `PATH` del usuario. Cuando el binario no está allí, el servidor falla al iniciarse y `claude --debug` registra `LSP server <name> failed to start`
* **Conflictos de extensión**: cuando dos servidores habilitados reclaman la misma extensión, el primero registrado maneja esos archivos y el otro no se usa para ellos, ya sea que los servidores provengan de un plugin o dos. La pestaña **Errors** de `/plugin` muestra la advertencia `LSP server "<name>" is not used for <ext> files`

La clave de manifiesto `lspServers` toma el mismo mapa en línea, una ruta a un archivo JSON, o una matriz de esos, y sus servidores se suman a los de `.lsp.json`. Cuando un servidor de manifiesto tiene el mismo nombre que uno en `.lsp.json`, el servidor de manifiesto lo reemplaza.

Para `transport`, tiempos de espera, reinicios y los otros campos, consulte [`lspServers`](/docs/es/plugins/manifest-reference#lspservers).

Envíe la salida de registro a stderr, no a stdout. Claude Code lee el stdout de un servidor solo como mensajes de protocolo, y acepta encabezados de mensaje de hasta 64 KiB y un cuerpo de mensaje de hasta 32 MiB.

Claude Code desconecta un servidor que excede cualquiera de los límites o escribe salida que no es de protocolo a stdout, y cuenta la desconexión como un bloqueo para `restartOnCrash` y `maxRestarts`. Cuando ejecuta con `--debug`, Claude Code escribe un error que nombra la causa en el registro de depuración.

<h3 id="executables">
  Ejecutables
</h3>

Los archivos en `bin/` en la raíz del plugin están en el `PATH` del shell de la herramienta Bash mientras el plugin está habilitado, para que Claude pueda ejecutarlos como comandos simples. Agregue un script ejecutable:

```bash bin/hello-plugin theme={null}
#!/bin/bash
echo "hello from my-plugin"
```

Hágalo ejecutable con `chmod +x bin/hello-plugin` y cargue el plugin. Cuando pide a Claude que ejecute `hello-plugin`, el resultado de la herramienta Bash muestra la salida del script.

Los directorios `bin/` del plugin vienen después de las entradas `PATH` propias del usuario, así que un plugin no puede sombrear `git`, `ls` u otro comando del sistema.

claude.ai y Cowork no instalan un plugin que tenga un directorio `bin/` de nivel superior, incluido uno que [distribuya a través de la configuración de la organización de claude.ai](/docs/es/plugins/host-marketplace#distribute-through-organization-settings).

<h3 id="default-settings">
  Configuración predeterminada
</h3>

Para establecer valores predeterminados que se apliquen mientras el plugin está habilitado, agregue un `settings.json` en la raíz del plugin, o coloque el mismo objeto en línea en la clave de manifiesto `settings`. Dos claves tienen efecto, `agent` y `subagentStatusLine`, y todas las demás claves se descartan.

Establezca `agent` para ejecutar uno de los agentes propios del plugin como el hilo principal:

```json settings.json theme={null}
{
  "agent": "security-reviewer"
}
```

Cargue el plugin e inicie una sesión. Claude entonces responde en la conversación principal con el indicador del sistema del agente `security-reviewer` y el modelo.

Para todo lo que controla la clave, consulte la [configuración `agent`](/docs/es/settings-reference#agent).

Cuando la misma clave se establece en más de un lugar, estas reglas deciden qué valor se aplica:

* **Archivo sobre manifiesto**: cuando ambos existen y `settings.json` establece al menos una clave admitida, `settings.json` se aplica y el `settings` del manifiesto se ignora
* **Configuración del usuario sobre valores predeterminados del plugin**: en todas las fuentes de configuración, los valores predeterminados del plugin son la capa más baja, así que un `agent` propio del usuario en `~/.claude/settings.json` anula el suyo
* **Dos plugins establecen la misma clave**: el valor del plugin cargado último se aplica, y `claude --debug` registra `overrides setting`

Para la forma `subagentStatusLine`, consulte [líneas de estado de subagente](/docs/es/statusline#subagent-status-lines).

<h3 id="themes-and-output-styles">
  Temas y estilos de salida
</h3>

Un plugin puede incluir temas de color y estilos de salida. Ambos aparecen en los mismos selectores que los del usuario. Para cualquiera de los dos, establecer la clave de manifiesto reemplaza el escaneo de carpeta.

| Componente       | Guardar como              | Formato                                                                                                                                   | Aparece en                              | Clave de manifiesto   |
| :--------------- | :------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------- | :-------------------- |
| Tema             | `themes/<slug>.json`      | El formato de [archivo de tema personalizado](/docs/es/terminal-config#create-a-custom-theme) que los usuarios escriben en `~/.claude/themes/` | `/theme`, bajo el `name` del archivo    | `experimental.themes` |
| Estilo de salida | `output-styles/<name>.md` | El formato de [estilo de salida personalizado](/docs/es/output-styles#create-a-custom-output-style), con frontmatter `name` y `description`    | `/output-style`, como `<plugin>:<name>` | `outputStyles`        |

Los temas del plugin son de solo lectura, así que cuando un usuario edita uno en `/theme`, la edición se guarda como una copia en su propio directorio de temas.

Este tema recolora el acento del indicador y el texto de error en el preajuste oscuro:

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
  Canales
</h3>

Un [canal](/docs/es/channels) permite que un sistema externo como una aplicación de chat envíe mensajes a una sesión. En un plugin, un canal es uno de los servidores MCP más una entrada `channels` que se vincula a él y puede solicitar su propia configuración. Este manifiesto vincula un canal a un servidor `telegram` y solicita un token de bot:

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

`server` debe coincidir con una clave en `mcpServers`. El `userConfig` por canal toma la misma forma que la clave `userConfig` de [nivel superior](#user-configuration).

Para lo que el servidor debe implementar y cómo los usuarios habilitan un plugin de canal, consulte [Empaquetar como un plugin](/docs/es/channels-reference#package-as-a-plugin) en la referencia de canales. Para la tabla de campos, consulte [`channels`](/docs/es/plugins/manifest-reference#channels).

<h3 id="monitors">
  Monitores
</h3>

Un monitor es un comando de shell que se ejecuta en segundo plano durante toda la sesión. Lo que imprime llega a Claude como notificaciones, para que Claude pueda reaccionar a un registro o un cambio de estado sin que se le pida que lo observe. Guarde las entradas en `monitors/monitors.json`:

```json monitors/monitors.json theme={null}
[
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Application error log"
  }
]
```

El comando se ejecuta en un shell, en el directorio de trabajo en el que comenzó la sesión.

El comando de un monitor está limitado en dónde comienza y qué puede referenciar:

* **Solo sesiones interactivas**: los monitores del plugin comienzan en una sesión interactiva y nunca en modo no interactivo con la bandera `-p`. También comienzan solo donde la [herramienta Monitor](/docs/es/tools-reference#monitor-tool) está disponible
* **Sin configuración del usuario**: `command` obtiene las [variables de ruta](#path-variables-and-persistent-data) y `${ENV_VAR}` del entorno, pero nunca `${user_config.*}`. Un monitor que hace referencia a uno no comienza, y los procesos de monitor tampoco reciben `CLAUDE_PLUGIN_OPTION_<KEY>`
* **Deshabilitación a mitad de sesión**: si deshabilita un plugin a mitad de sesión, Claude Code no detiene los monitores que ya se están ejecutando. Se detienen cuando termina la sesión

La clave de manifiesto `experimental.monitors` toma la misma matriz en línea o una ruta a un archivo JSON, y se lee en lugar de `monitors/monitors.json`.

Para el disparador `when` y los otros campos, consulte [`monitors`](/docs/es/plugins/manifest-reference#monitors).

<h2 id="user-configuration">
  Pedir al usuario valores de configuración
</h2>

Declare los valores que su plugin necesita del usuario en la clave de manifiesto `userConfig`, para que los usuarios no editen `settings.json` ellos mismos. Cada opción aparece en un diálogo con su `title` como etiqueta y su `description` debajo.

Establezca `"sensitive": true` para un token o contraseña. El diálogo entonces enmascara la entrada, y el valor se almacena en almacenamiento seguro en lugar de `settings.json`.

Este manifiesto solicita un punto final y un token:

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
  Cuándo aparece el diálogo de configuración
</h3>

El diálogo aparece solo en la interfaz interactiva `/plugin`. Se abre para cualquier opción que aún no esté establecida cuando el usuario hace cualquiera de lo siguiente:

* Instala el plugin en `/plugin`
* Ejecuta `/plugin install <plugin>@<marketplace>` dentro de una sesión
* Habilita el plugin desde la pestaña **Installed** en `/plugin`

Para abrir el mismo diálogo en cualquier momento, el usuario ejecuta `/plugin configure <plugin>@<marketplace>`.

El comando de shell `claude plugin install` nunca solicita valores de `userConfig`. Para establecer valores desde el shell, pase cada uno como `--config KEY=VALUE`. Cuando las opciones permanecen sin establecer, el comando imprime una línea `userConfig options not yet set` que nombra ambas formas de establecerlas. [El diálogo `userConfig` nunca aparece](/docs/es/plugins/troubleshooting#the-userconfig-dialog-never-appears) cita la línea.

Para los campos de opción, dónde se almacena cada valor, cómo un componente hace referencia a un valor guardado, y qué campos rechazan `${user_config.*}`, consulte [Configuración del usuario](/docs/es/plugins/manifest-reference#user-configuration).

<h2 id="path-variables-and-persistent-data">
  Hacer referencia a rutas de plugin y almacenar datos
</h2>

No sabe dónde se instalará su plugin, así que haga referencia a sus archivos y datos a través de estas variables en lugar de rutas fijas. Se sustituyen en contenido de skill, comando y agente, en comandos de hook y monitor, y en configuraciones de servidor MCP y LSP. También se exportan a procesos de hook, MCP y LSP:

* **`${CLAUDE_PLUGIN_ROOT}`**: el directorio de instalación del plugin. Cada versión tiene su propio [directorio de caché](/docs/es/plugins/loading#find-plugins-on-disk), así que la ruta cambia cuando el plugin se actualiza. No escriba estado allí
* **`${CLAUDE_PLUGIN_DATA}`**: un directorio que sobrevive a las actualizaciones, para `node_modules`, entornos virtuales y cachés. Se resuelve a `~/.claude/plugins/data/<id>/` y se crea cuando se hace referencia por primera vez
* **`${CLAUDE_PROJECT_DIR}`**: la raíz del proyecto, el mismo valor que los hooks reciben

En la ruta del directorio de datos, `<id>` es el identificador del plugin con cada carácter que no sea letra, dígito, `_` y `-` reemplazado por `-`, así que `my-plugin@my-marketplace` se convierte en `my-plugin-my-marketplace`.

En Windows, las rutas sustituidas usan barras diagonales para que un shell no lea las barras invertidas como escapes.

<h3 id="install-dependencies-into-the-data-directory">
  Instalar dependencias en el directorio de datos
</h3>

Para un plugin instalado desde marketplace, Claude Code instala automáticamente [dependencias de paquetes Node.js](/docs/es/plugins/loading#node-js-package-dependencies) elegibles cuando almacena en caché el plugin, así que es posible que no necesite instalarlas usted mismo. Cuando lo hace, este hook `SessionStart` instala `node_modules` en `${CLAUDE_PLUGIN_DATA}` en la primera ejecución y nuevamente después de que una actualización cambie `package.json`:

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

Después de la primera sesión, `~/.claude/plugins/data/<id>/node_modules` existe. Un servidor MCP puede entonces establecer `NODE_PATH` a `${CLAUDE_PLUGIN_DATA}/node_modules` en su `env`. Para qué campos sustituyen qué variable, consulte [Variables de entorno](/docs/es/plugins/manifest-reference#environment-variables).

<h2 id="next-steps">
  Próximos pasos
</h2>

* [Referencia de manifiesto de plugin](/docs/es/plugins/manifest-reference): campos `plugin.json`, reglas de ruta y el diseño estándar
* [Probar plugins con evals](/docs/es/plugin-evals): verifique que los componentes que agregó cambien el comportamiento de Claude de la manera que pretende
* [Publicar y distribuir un plugin](/docs/es/plugins/publish): versione el plugin y colóquelo en un marketplace
* [Solucionar problemas de plugins](/docs/es/plugins/troubleshooting): qué hacer cuando un componente no se carga o un hook no se activa
