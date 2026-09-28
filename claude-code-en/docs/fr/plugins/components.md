> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ajouter des composants à un plugin

> Ajoutez des skills, des hooks, des serveurs MCP et tous les autres types de composants à un plugin Claude Code, avec un exemple qui valide chacun.

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

Un plugin Claude Code est construit à partir de composants, tels que des skills, des agents, des hooks et des serveurs MCP. Chaque composant a un dossier par défaut dans le plugin, une clé de manifeste optionnelle dans `.claude-plugin/plugin.json` qui remplace ou ajoute à ce dossier, et un nom que l'utilisateur voit. Pour chaque tableau complet des champs de clé, consultez la [référence du manifeste](/docs/fr/plugins/manifest-reference#fields).

Utilisez cette page pour ajouter un composant à un plugin qui charge déjà.

Après avoir ajouté un composant, exécutez `/reload-plugins` dans une session en cours ou démarrez une nouvelle session pour que Claude Code le charge. Pour vérifier le fichier du composant avant de le charger, exécutez [`claude plugin validate .`](/docs/fr/plugins/cli-reference#plugin-validate) dans votre shell à partir du répertoire du plugin.

<Note>
  Ces cas sont couverts sur d'autres pages :

  * **Construire votre premier plugin** : commencez par [Créer un plugin](/docs/fr/plugins/create)
  * **Installer le plugin de quelqu'un d'autre** : consultez [Installer des plugins](/docs/fr/plugins/install)
  * **Les utilisateurs de votre plugin sont sur claude.ai ou dans Cowork** : un ensemble différent de composants se charge là. Consultez [Plugins sur claude.ai et dans Cowork](https://claude.com/docs/plugins/overview)
</Note>

<h2 id="explore-the-plugin-directory">
  Explorez le répertoire du plugin
</h2>

L'explorateur montre un exemple de plugin, `my-plugin`, qui a un de chaque type de composant à son emplacement par défaut :

* Une skill de révision et une commande `about`
* Un sous-agent de révision de sécurité
* Un hook qui formate les fichiers après que Claude les édite, et le dossier `scripts/` qu'il appelle
* Un moniteur de journal
* Un style de sortie et un thème de couleur
* Un workflow d'audit de route
* Un exécutable `hello-plugin`
* Les paramètres par défaut
* Un serveur MCP local et un serveur de langage Go

Chaque fichier est le plus petit exemple valide de son format, là pour montrer la forme plutôt que d'être utile : une skill ou un agent réel porte des instructions complètes et souvent des fichiers de support, et un hook ou un moniteur réel fait un vrai travail. Les sections après l'explorateur utilisent les mêmes fichiers que leurs exemples et renvoient à des versions plus complètes. Sélectionnez un fichier ou un dossier pour lire à quoi il sert, voir ce qu'il contient, et trouver la section qui le couvre.

<PluginExplorer>
  <Piece id="manifest">
    Le [manifeste](/docs/fr/plugins/manifest-reference) est le fichier `plugin.json` dans le répertoire `.claude-plugin/` d'un plugin. Il contient les métadonnées du plugin et les valeurs `userConfig` que Claude Code demande à l'utilisateur. Seul `name` est requis. Dans celui-ci, `description` est le texte que les utilisateurs voient pour le plugin dans `/plugin`, et `version` garde les utilisateurs sur cette version jusqu'à ce que vous la changiez :

    ```json theme={null}
    {
      "name": "my-plugin",
      "version": "1.0.0",
      "description": "Review, formatting, and database tools for this team"
    }
    ```
  </Piece>

  <Piece id="skills">
    Une [skill](/docs/fr/skills) est un fichier `SKILL.md`. Enregistrez chaque skill dans son propre répertoire sous `skills/`. Claude lit la `description` de chaque skill, et quand ce que l'utilisateur demande correspond, comme demander à Claude de réviser une pull request ici, Claude charge les instructions de la skill et les suit. L'utilisateur peut aussi l'exécuter directement comme `/my-plugin:review` :

    ```markdown theme={null}
    ---
    description: Reviews a pull request for style and test coverage. Use when asked to review code.
    ---

    Review the changed files. Report style problems first, then missing tests.
    ```
  </Piece>

  <Piece id="commands">
    Une commande est un seul fichier Markdown que l'utilisateur exécute par nom. Les commandes sont le format plus ancien : une skill s'exécute par nom de la même manière et peut aussi porter des fichiers de support dans son propre répertoire, donc écrivez les nouvelles comme des skills et gardez `commands/` pour les fichiers que vous avez déjà. Ce fichier devient `/my-plugin:about` et prend le même frontmatter qu'une skill :

    ```markdown theme={null}
    ---
    description: Summarize the repository
    ---

    Summarize what this repository does in three sentences.
    ```
  </Piece>

  <Piece id="agents">
    Un [sous-agent](/docs/fr/sub-agents) est un assistant séparé, avec ses propres instructions et sa propre fenêtre de contexte, que Claude peut déléguer une tâche et obtenir un résultat. Chaque fichier Markdown sous `agents/` en définit un : le frontmatter le nomme et dit quand l'utiliser, et le corps est son invite système. Celui-ci est nommé `my-plugin:security-reviewer`, et l'utilisateur peut l'invoquer avec `@agent-my-plugin:security-reviewer` :

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
    Un [hook](/docs/fr/hooks-guide) exécute quelque chose automatiquement à un point du cycle de vie de Claude Code, comme après chaque édition de fichier : une commande shell, une requête HTTP, un appel d'outil MCP, une invite à un modèle, ou un sous-agent. Enregistrez les hooks du plugin dans `hooks/hooks.json` à la racine du plugin. Celui-ci exécute le `scripts/format.sh` du plugin après que Claude écrit ou édite un fichier :

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
    Un moniteur est une commande shell que Claude Code démarre en arrière-plan quand la session démarre et continue à exécuter jusqu'à ce qu'elle se termine, en utilisant l'[outil Monitor](/docs/fr/tools-reference#monitor-tool). Ce qu'il imprime atteint Claude comme des notifications. Un champ `when` peut à la place le démarrer la première fois qu'une skill nommée s'exécute. Celui-ci suit un journal d'erreurs :

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
    Un plugin peut inclure des [styles de sortie](/docs/fr/output-styles), qui changent la façon dont Claude formate et formule ses réponses. Enregistrez chaque style de sortie comme `output-styles/<name>.md`. Celui-ci apparaît dans `/output-style` comme `my-plugin:terse` :

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
    Un plugin peut inclure des [thèmes de couleur](/docs/fr/terminal-config#create-a-custom-theme) pour l'interface Claude Code. Enregistrez chaque thème comme `themes/<slug>.json`. Celui-ci apparaît dans `/theme` comme `Dracula`, marqué comme provenant de `my-plugin` :

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
    Le dossier `workflows/` contient des fichiers [workflow](/docs/fr/workflows) `.js` : un bloc `meta`, puis un corps de script qui orchestre plusieurs sous-agents. Celui-ci s'exécute comme `/my-plugin:audit-routes` :

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
    `bin/` est la façon dont un plugin expédie un outil en ligne de commande. Tant que le plugin est activé, Claude Code met ce dossier sur le `PATH` du shell dans lequel il exécute les commandes, donc Claude, ou les instructions d'une skill, peuvent exécuter l'outil par nom sans que l'utilisateur n'installe rien. Avec cet [exécutable](#executables) en place, `hello-plugin` est une commande que Claude peut exécuter :

    ```bash theme={null}
    #!/bin/bash
    echo "hello from my-plugin"
    ```
  </Piece>

  <Piece id="scripts">
    Le hook dans `hooks/hooks.json` exécute un script, et ce dossier est l'endroit où l'exemple le garde. Le nom `scripts/` est une convention, pas quelque chose que Claude Code recherche : le hook pointe vers le fichier par son chemin, `${CLAUDE_PLUGIN_ROOT}/scripts/format.sh`. Un script de formatage pourrait ressembler à ceci :

    ```bash theme={null}
    #!/bin/bash
    npx prettier --write .
    ```
  </Piece>

  <Piece id="settings">
    Un `settings.json` à la racine du plugin contient des [paramètres](/docs/fr/settings-reference) qui s'appliquent tant que le plugin est activé, donc un plugin peut changer le comportement de la session et non seulement ajouter des composants. Seules deux clés prennent effet à partir d'un plugin, [`agent`](/docs/fr/settings-reference#agent) et [`subagentStatusLine`](/docs/fr/settings-reference#subagentstatusline) ; toute autre clé est supprimée. Consultez [Paramètres par défaut](#default-settings).

    Celui-ci définit `agent`, qui exécute le fil principal de la session comme le propre agent `security-reviewer` du plugin, donc l'invite système, les restrictions d'outils et le modèle de cet agent s'appliquent à toute la session :

    ```json theme={null}
    {
      "agent": "security-reviewer"
    }
    ```
  </Piece>

  <Piece id="mcp">
    Un [serveur MCP](/docs/fr/mcp) donne à Claude des outils d'un système externe. Déclarez-le dans `.mcp.json` à la racine du plugin. Celui-ci démarre un serveur local à partir d'un script à l'intérieur du plugin, et apparaît dans `/mcp` comme `plugin:my-plugin:db` :

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
    Un serveur LSP donne à Claude des [diagnostics et une navigation de code](/docs/fr/plugins/code-intelligence) pour une langue. Déclarez le serveur dans `.lsp.json` à la racine du plugin. Celui-ci connecte le serveur de langage Go pour les fichiers `.go` :

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
  Ajouter chaque type de composant
</h2>

Chaque section ci-dessous couvre un type de composant : où ses fichiers vont dans le plugin, un exemple qui valide, ce que l'utilisateur voit une fois que le plugin charge, et la clé de manifeste qui change l'emplacement par défaut. Ajoutez ceux dont votre plugin a besoin ; aucun n'est requis.

<h3 id="skills">
  Skills
</h3>

Une [skill](/docs/fr/skills) est un fichier `SKILL.md` que Claude peut charger quand sa description correspond à la tâche. L'utilisateur peut aussi l'exécuter comme une commande. Enregistrez chaque skill dans son propre répertoire sous `skills/` :

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json
└── skills/
    └── review/
        └── SKILL.md
```

Donnez au `SKILL.md` une `description` pour que Claude sache quand l'utiliser :

```markdown skills/review/SKILL.md theme={null}
---
description: Reviews a pull request for style and test coverage. Use when asked to review code.
---

Review the changed files. Report style problems first, then missing tests.
```

Après avoir chargé le plugin, `/my-plugin:review` exécute la skill. Le nom de la commande et qui peut l'invoquer suivent ces règles :

* **Nom de la commande** : `/<plugin>:<directory>`, donc `skills/review/SKILL.md` dans `my-plugin` est `/my-plugin:review`. Si vous définissez `name` dans le frontmatter, il remplace le dernier segment et le préfixe du plugin reste. Consultez [comment une skill obtient son nom de commande](/docs/fr/skills#how-a-skill-gets-its-command-name)
* **Qui l'invoque** : Claude, l'utilisateur, ou les deux, contrôlé par le frontmatter. Consultez [Contrôler qui invoque une skill](/docs/fr/skills#control-who-invokes-a-skill)

Vous pouvez aussi placer des skills en dehors du répertoire par défaut `skills/` :

* **Répertoires supplémentaires** : listez-les dans la clé de manifeste `skills`. Ils s'ajoutent au scan `skills/` par défaut plutôt que de le remplacer, contrairement à `commands` et `agents`
* **Une seule skill à la racine du plugin** : sans répertoire `skills/` et sans clé de manifeste `skills`, un `SKILL.md` à la racine du plugin charge comme une skill. Définissez `name` dans son frontmatter, car sinon une installation marketplace nomme la skill d'après son [répertoire de cache](/docs/fr/plugins/loading#find-plugins-on-disk) plutôt que votre plugin

Pour inclure des instructions dans un plugin, écrivez-les comme une skill. Claude Code ne charge pas un `CLAUDE.md` à la racine du plugin, et `claude plugin validate` avertit `CLAUDE.md at the plugin root is not loaded as project context`.

Pour les champs de frontmatter et les fichiers de support, consultez [Skills](/docs/fr/skills).

<h3 id="commands">
  Commandes
</h3>

Une commande est un seul fichier Markdown que l'utilisateur exécute par nom, comme `/my-plugin:about`.

<Note>
  Les commandes sont le format plus ancien, et les [skills](#skills) les remplacent pour les nouveaux travaux. Une skill s'exécute par nom de la même manière, et elle peut aussi porter des fichiers de support dans son répertoire. Gardez `commands/` pour les fichiers que vous migrez depuis `.claude/commands/`.
</Note>

Enregistrez une commande à `commands/<file>.md` et elle devient `/<plugin>:<file>`. Un sous-répertoire ajoute un segment, donc `commands/db/migrate.md` est `/my-plugin:db:migrate`.

Les fichiers de commande prennent le même frontmatter que les skills.

<h4 id="define-commands-in-the-manifest">
  Définir les commandes dans le manifeste
</h4>

Vous n'en avez besoin que si vous voulez garder les fichiers de commande quelque part d'autre que `commands/`, ou pour définir une commande courte dans `plugin.json` sans fichier Markdown séparé. Définissez la clé de manifeste `commands`, et Claude Code la lit à la place de scanner `commands/`. La clé prend un chemin, un tableau de chemins, ou un objet qui mappe chaque nom de commande à soit un fichier `source` soit un `content` en ligne.

Ce manifeste définit `/my-plugin:about` en ligne, sans fichier Markdown :

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

Chargez le plugin et exécutez `/my-plugin:about` dans la session pour confirmer qu'il a chargé.

Pour la syntaxe complète de la clé, consultez [`commands`](/docs/fr/plugins/manifest-reference#commands).

<h3 id="agents">
  Agents
</h3>

Un [sous-agent](/docs/fr/sub-agents) est un assistant séparé, avec ses propres instructions et fenêtre de contexte, que Claude peut déléguer une tâche. Chaque fichier Markdown sous `agents/` en définit un :

```markdown agents/security-reviewer.md theme={null}
---
name: security-reviewer
description: Reviews code changes for security issues. Use after edits to authentication or input handling.
model: sonnet
---

You are a security reviewer. Read the changed files and report injection, authentication, and secrets-handling risks.
```

Cet agent est nommé `my-plugin:security-reviewer`, et l'utilisateur peut l'[invoquer explicitement](/docs/fr/sub-agents#invoke-subagents-explicitly) avec `@agent-my-plugin:security-reviewer`. La forme du nom est `<plugin>:<name>`, où `<name>` provient du frontmatter, ou du nom du fichier quand il n'y en a pas.

La clé de manifeste `agents` remplace le scan `agents/`.

<h4 id="organize-agents-in-subfolders">
  Organiser les agents dans des sous-dossiers
</h4>

Vous pouvez mettre les fichiers d'agent du plugin dans des sous-dossiers de `agents/`. Claude Code les [charge récursivement](/docs/fr/sub-agents#choose-the-subagent-scope) et joint le nom du plugin, chaque nom de sous-dossier, et le nom du fichier avec des deux-points pour former le nom d'agent scopé. Par exemple, `agents/review/security.md` dans un plugin nommé `my-plugin` charge comme `my-plugin:review:security`. Deux paramètres changent ce nom :

* Frontmatter `name` : il remplace seulement le nom du fichier, donc `name: audit` dans `agents/review/security.md` charge comme `my-plugin:review:audit`
* Champ de manifeste [`agents`](/docs/fr/plugins/manifest-reference#fields) : un fichier que vous listez là charge sans noms de sous-dossier, donc `"agents": "./custom/review/security.md"` charge comme `my-plugin:security`

<h4 id="frontmatter-fields-in-plugin-agents">
  Champs de frontmatter dans les agents du plugin
</h4>

Le frontmatter d'un agent du plugin suit ces règles :

* **Champs supportés** : `name`, `description`, `model`, `effort`, `maxTurns`, `tools`, `disallowedTools`, `skills`, `memory`, `background`, `omitClaudeMd`, `isolation`, `color`, et la clé `cacheTtl` de `experimental`. La seule valeur `isolation` valide est `"worktree"`. Consultez [champs de frontmatter supportés](/docs/fr/sub-agents#supported-frontmatter-fields) pour ce que chacun fait
* **Champs ignorés** : `permissionMode`, `hooks`, `mcpServers`, et `initialPrompt`. Un fichier d'agent ne peut pas ajouter des hooks ou des serveurs MCP par lui-même, donc ajoutez-les comme plugin [hooks](#hooks) et [serveurs MCP](#mcp-servers) à la place
* **Frontmatter qui ne s'analyse pas** : l'agent charge quand même avec chaque champ ignoré. Il est nommé d'après le fichier, et sa description lit `Agent from my-plugin plugin`. Exécutez [`claude plugin validate`](/docs/fr/plugins/cli-reference#plugin-validate) dans votre shell pour trouver ces fichiers

Pour ce que chaque champ fait et les règles de précédence, consultez [Sous-agents](/docs/fr/sub-agents#supported-frontmatter-fields).

<h3 id="hooks">
  Hooks
</h3>

Un [hook](/docs/fr/hooks-guide) exécute quelque chose automatiquement à un point du cycle de vie de Claude Code, comme après chaque édition de fichier : une commande shell, une requête HTTP, un appel d'outil MCP, une invite à un modèle, ou un sous-agent. Enregistrez les hooks du plugin dans `hooks/hooks.json` à la racine du plugin, sous une clé `"hooks"` de niveau supérieur, dans la même forme que l'objet `hooks` dans `settings.json`. Cela vous permet de copier un hook de paramètres existant inchangé.

Ce hook exécute un script groupé après chaque `Write` ou `Edit` :

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

Enregistrez le script à `scripts/format.sh` et rendez-le exécutable.

Chargez le plugin et demandez à Claude d'éditer un fichier. Un hook `PostToolUse` qui sort 0 ne montre rien dans la transcription, donc confirmez qu'il a exécuté avec [journalisation de débogage](/docs/fr/hooks#debug-hooks) ou par ce que le script lui-même change.

Les hooks dans `hooks/hooks.json` et dans la clé de manifeste `hooks` chargent tous les deux. Pour chaque événement et sa charge utile, consultez [Événements de hook](/docs/fr/hooks#hook-events).

<h4 id="when-plugin-hooks-fire">
  Quand les hooks du plugin se déclenchent
</h4>

Les hooks d'un plugin n'attendent pas qu'une des skills ou commandes du plugin soit utilisée. Claude Code les enregistre quand une session charge le plugin, et ils se déclenchent sur leurs événements à partir de là. Pour limiter quand un hook s'exécute, réduisez son `matcher`.

Si un hook ne se déclenche jamais, consultez [hooks qui ne se déclenchent pas](/docs/fr/plugins/troubleshooting#failed-to-load-hooks-from-and-hooks-that-dont-fire).

<h4 id="environment-quoting-and-matching-mcp-tools">
  Environnement, guillemets et correspondance des outils MCP
</h4>

L'environnement du hook, les guillemets de `${CLAUDE_PLUGIN_ROOT}`, et les matchers pour les outils MCP du plugin fonctionnent comme suit :

* **Environnement** : chaque processus de hook reçoit `CLAUDE_PLUGIN_ROOT` et `CLAUDE_PLUGIN_DATA` dans son environnement, plus `CLAUDE_PLUGIN_OPTION_<KEY>` pour chaque valeur de [configuration utilisateur](#user-configuration), donc votre script peut les lire de là
* **Guillemets** : quand `command` n'a pas `args`, il s'exécute via un shell, donc enveloppez le chemin `${CLAUDE_PLUGIN_ROOT}` entre guillemets doubles, comme l'exemple `hooks/hooks.json` sous [Hooks](#hooks) le fait, pour garder le chemin développé un mot shell. Quand vous passez `args` à la place, chaque élément est passé comme un argument sans shell et n'a besoin d'aucun guillemet. Consultez [forme exec et forme shell](/docs/fr/hooks#exec-form-and-shell-form)
* **Correspondance des outils MCP du plugin** : un outil d'un [serveur MCP que ce plugin déclare](#mcp-servers) est nommé `mcp__plugin_<plugin>_<server>__<tool>`, donc écrivez ce nom complet dans le matcher. Un matcher sur le nom du serveur seul ne se déclenche jamais. Consultez [Correspondance des outils MCP](/docs/fr/hooks#match-mcp-tools)

<h3 id="mcp-servers">
  Serveurs MCP
</h3>

Un serveur MCP donne à Claude des outils d'un système externe. Déclarez-le dans `.mcp.json` à la racine du plugin, dans la même forme qu'un [`.mcp.json` de projet](/docs/fr/mcp#project-scope). Ce `.mcp.json` déclare un serveur nommé `db` :

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

Vous pouvez aussi omettre le wrapper `mcpServers` et mettre `db` au niveau supérieur du fichier.

Chargez le plugin et exécutez `/mcp` pour confirmer que le serveur apparaît comme `plugin:my-plugin:db`.

`claude plugin validate` vérifie `.mcp.json` et signale une entrée de serveur que Claude Code supprimerait au moment du chargement comme une erreur. Nécessite Claude Code v2.1.281 ou ultérieur.

Pour où une mauvaise entrée s'affiche au moment du chargement, consultez [Serveurs MCP qui ne démarrent pas](/docs/fr/plugins/troubleshooting#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start).

La clé de manifeste `mcpServers` prend une carte de serveur en ligne, un chemin vers un fichier JSON, ou un tableau de ceux-ci. Quand un serveur de manifeste a le même nom qu'un dans `.mcp.json`, le serveur de manifeste le remplace.

<h4 id="reach-users-on-claude-ai-and-cowork">
  Atteindre les utilisateurs sur claude.ai et Cowork
</h4>

Un serveur stdio local, comme le serveur `db` sous [Serveurs MCP](#mcp-servers), s'exécute dans Claude Code et dans une session Cowork qui s'exécute sur votre machine dans l'application Claude Desktop, mais pas sur claude.ai. Pour atteindre les utilisateurs là aussi, référencez un serveur distant par son URL `https://`, que claude.ai et Cowork offrent à l'utilisateur comme connecteur.

<h4 id="server-names-tool-names-and-reloads">
  Noms de serveur, noms d'outils et rechargements
</h4>

Les noms du serveur, la substitution de variables, et le comportement de rechargement suivent ces règles :

* **Nom du serveur** : `plugin:<plugin>:<server>`, donc le serveur `db` dans `my-plugin` est `plugin:my-plugin:db` dans `/mcp`. Utilisez la même forme pour nommer le serveur dans un hook [`mcp_tool`](/docs/fr/hooks#mcp-tool-hook-fields)
* **Noms d'outils** : `mcp__plugin_<plugin>_<server>__<tool>`, donc un outil `query` sur ce serveur `db` est `mcp__plugin_my-plugin_db__query`. C'est le nom à utiliser dans les [règles de permission](/docs/fr/permissions) et les [matchers de hook](#hooks)
* **Substitution** : `${CLAUDE_PLUGIN_ROOT}` et les autres [variables de chemin](#path-variables-and-persistent-data) sont substituées dans `command`, `args`, et `env`. Aucun guillemet n'est nécessaire dans `args`, car chaque élément est passé comme un argument
* **Rechargement** : quand l'utilisateur exécute `/reload-plugins` et que [le rechargement s'applique](/docs/fr/plugins/cli-reference#reloads-that-change-mcp-tools), un serveur dont la configuration est inchangée garde sa connexion. Un serveur dont la configuration a changé se reconnecte, et un que vous avez supprimé se déconnecte

<h4 id="include-a-packaged-mcpb-server">
  Inclure un serveur MCPB emballé
</h4>

La clé `mcpServers` accepte aussi un serveur emballé comme un [fichier MCPB](https://github.com/modelcontextprotocol/mcpb), dont l'extension est `.mcpb` ou l'ancienne `.dxt`. Pointez la clé vers le fichier, comme un chemin à l'intérieur du plugin ou une URL `https://` :

```json .claude-plugin/plugin.json theme={null}
{
  "name": "my-plugin",
  "mcpServers": "./servers/db.mcpb"
}
```

Le serveur prend son nom du `name` dans le manifeste du bundle.

Pour les transports et l'authentification, consultez [MCP](/docs/fr/mcp#plugin-provided-mcp-servers).

<h3 id="lsp-servers">
  Serveurs LSP
</h3>

Un serveur LSP donne à Claude des diagnostics et une navigation de code pour une langue. Si un [plugin officiel de code intelligence](/docs/fr/plugins/code-intelligence) couvre déjà votre langue, installez celui-ci à la place d'en écrire un. Sinon, déclarez le serveur dans `.lsp.json` à la racine du plugin :

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

Le fichier mappe chaque nom de serveur directement à sa configuration, sans objet wrapper autour de la carte. `command` est le nom du binaire, avec ses arguments dans `args`. `extensionToLanguage` a besoin d'au moins une extension, chacune commençant par `.`.

`claude plugin validate` ne lit pas ce fichier. Quand une entrée est invalide, le fichier entier est ignoré au chargement et `Invalid LSP server config for ".lsp.json"` apparaît dans l'onglet **Errors** de `/plugin`.

Votre plugin configure la connexion mais n'installe pas le binaire du serveur, et chaque extension de fichier obtient un serveur :

* **Binaire manquant** : Claude Code démarre `command` par nom depuis le `PATH` de l'utilisateur. Quand le binaire n'est pas là, le serveur échoue à démarrer et `claude --debug` enregistre `LSP server <name> failed to start`
* **Conflits d'extension** : quand deux serveurs activés revendiquent la même extension, le premier enregistré gère ces fichiers et l'autre n'est pas utilisé pour eux, que les serveurs proviennent d'un plugin ou de deux. L'onglet **Errors** de `/plugin` montre l'avertissement `LSP server "<name>" is not used for <ext> files`

La clé de manifeste `lspServers` prend la même carte en ligne, un chemin vers un fichier JSON, ou un tableau de ceux-ci, et ses serveurs s'ajoutent à ceux dans `.lsp.json`. Quand un serveur de manifeste a le même nom qu'un dans `.lsp.json`, le serveur de manifeste le remplace.

Pour `transport`, les délais d'attente, les redémarrages, et les autres champs, consultez [`lspServers`](/docs/fr/plugins/manifest-reference#lspservers).

Envoyez la sortie du journal à stderr, pas stdout. Claude Code lit le stdout d'un serveur comme des messages de protocole seulement, et accepte les en-têtes de message jusqu'à 64 KiB et un corps de message jusqu'à 32 MiB.

Claude Code déconnecte un serveur qui dépasse l'une ou l'autre limite ou écrit une sortie non-protocole à stdout, et compte la déconnexion comme un crash pour `restartOnCrash` et `maxRestarts`. Quand vous exécutez avec `--debug`, Claude Code écrit une erreur nommant la cause au journal de débogage.

<h3 id="executables">
  Exécutables
</h3>

Les fichiers dans `bin/` à la racine du plugin sont sur le `PATH` du shell de l'outil Bash tant que le plugin est activé, donc Claude peut les exécuter comme des commandes nues. Ajoutez un script exécutable :

```bash bin/hello-plugin theme={null}
#!/bin/bash
echo "hello from my-plugin"
```

Rendez-le exécutable avec `chmod +x bin/hello-plugin` et chargez le plugin. Quand vous demandez à Claude d'exécuter `hello-plugin`, le résultat de l'outil Bash montre la sortie du script.

Les répertoires `bin/` du plugin viennent après les entrées `PATH` de l'utilisateur, donc un plugin ne peut pas masquer `git`, `ls`, ou une autre commande système.

claude.ai et Cowork n'installent pas un plugin qui a un répertoire `bin/` de niveau supérieur, y compris un que vous [distribuez via les paramètres d'organisation claude.ai](/docs/fr/plugins/host-marketplace#distribute-through-organization-settings).

<h3 id="default-settings">
  Paramètres par défaut
</h3>

Pour définir les paramètres par défaut qui s'appliquent tant que le plugin est activé, ajoutez un `settings.json` à la racine du plugin, ou mettez le même objet en ligne dans la clé de manifeste `settings`. Deux clés prennent effet, `agent` et `subagentStatusLine`, et toute autre clé est supprimée.

Définissez `agent` pour exécuter l'un des propres agents du plugin comme le fil principal :

```json settings.json theme={null}
{
  "agent": "security-reviewer"
}
```

Chargez le plugin et démarrez une session. Claude répond alors dans la conversation principale avec l'invite système et le modèle de l'agent `security-reviewer`.

Pour tout ce que la clé contrôle, consultez le [paramètre `agent`](/docs/fr/settings-reference#agent).

Quand la même clé est définie à plus d'un endroit, ces règles décident quelle valeur s'applique :

* **Fichier sur manifeste** : quand les deux existent et `settings.json` définit au moins une clé supportée, `settings.json` s'applique et le `settings` du manifeste est ignoré
* **Paramètres utilisateur sur paramètres par défaut du plugin** : dans les sources de paramètres, les paramètres par défaut du plugin sont la couche la plus basse, donc un `agent` personnel de l'utilisateur dans `~/.claude/settings.json` remplace le vôtre
* **Deux plugins définissent la même clé** : la valeur du plugin chargé en dernier s'applique, et `claude --debug` enregistre `overrides setting`

Pour la forme `subagentStatusLine`, consultez [lignes d'état du sous-agent](/docs/fr/statusline#subagent-status-lines).

<h3 id="themes-and-output-styles">
  Thèmes et styles de sortie
</h3>

Un plugin peut inclure des thèmes de couleur et des styles de sortie. Les deux apparaissent dans les mêmes sélecteurs que ceux de l'utilisateur. Pour l'un ou l'autre, définir la clé de manifeste remplace le scan de dossier.

| Composant       | Enregistrer comme         | Format                                                                                                                                         | Apparaît dans                            | Clé de manifeste      |
| :-------------- | :------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------- | :-------------------- |
| Thème           | `themes/<slug>.json`      | Le format de [fichier de thème personnalisé](/docs/fr/terminal-config#create-a-custom-theme) que les utilisateurs écrivent dans `~/.claude/themes/` | `/theme`, sous le `name` du fichier      | `experimental.themes` |
| Style de sortie | `output-styles/<name>.md` | Le format de [style de sortie personnalisé](/docs/fr/output-styles#create-a-custom-output-style), avec le frontmatter `name` et `description`       | `/output-style`, comme `<plugin>:<name>` | `outputStyles`        |

Les thèmes du plugin sont en lecture seule, donc quand un utilisateur en édite un dans `/theme`, l'édition est enregistrée comme une copie dans son propre répertoire de thèmes.

Ce thème recolore l'accent d'invite et le texte d'erreur sur le préréglage sombre :

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
  Canaux
</h3>

Un [canal](/docs/fr/channels) permet à un système externe tel qu'une application de chat d'envoyer des messages dans une session. Dans un plugin, un canal est l'un des serveurs MCP plus une entrée `channels` qui se lie à lui et peut demander sa propre configuration. Ce manifeste lie un canal à un serveur `telegram` et demande un jeton de bot :

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

`server` doit correspondre à une clé dans `mcpServers`. Le `userConfig` par canal prend la même forme que la clé [`userConfig`](#user-configuration) de niveau supérieur.

Pour ce que le serveur doit implémenter et comment les utilisateurs activent un plugin de canal, consultez [Empaqueter comme un plugin](/docs/fr/channels-reference#package-as-a-plugin) dans la référence des canaux. Pour le tableau des champs, consultez [`channels`](/docs/fr/plugins/manifest-reference#channels).

<h3 id="monitors">
  Moniteurs
</h3>

Un moniteur est une commande shell qui s'exécute en arrière-plan pour toute la session. Ce qu'il imprime atteint Claude comme des notifications, donc Claude peut réagir à un journal ou à un changement d'état sans être demandé de le surveiller. Enregistrez les entrées dans `monitors/monitors.json` :

```json monitors/monitors.json theme={null}
[
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Application error log"
  }
]
```

La commande s'exécute dans un shell, dans le répertoire de travail dans lequel la session a démarré.

La commande d'un moniteur est limitée dans où elle démarre et ce qu'elle peut référencer :

* **Sessions interactives seulement** : les moniteurs du plugin démarrent dans une session interactive et jamais en mode non-interactif avec le drapeau `-p`. Ils démarrent aussi seulement où l'[outil Monitor](/docs/fr/tools-reference#monitor-tool) est disponible
* **Pas de configuration utilisateur** : `command` obtient les [variables de chemin](#path-variables-and-persistent-data) et `${ENV_VAR}` de l'environnement, mais jamais `${user_config.*}`. Un moniteur qui en référence un ne démarre pas, et les processus de moniteur ne reçoivent pas non plus `CLAUDE_PLUGIN_OPTION_<KEY>`
* **Désactivation en cours de session** : si vous désactivez un plugin en cours de session, Claude Code n'arrête pas les moniteurs qui s'exécutent déjà. Ils s'arrêtent quand la session se termine

La clé de manifeste `experimental.monitors` prend le même tableau en ligne ou un chemin vers un fichier JSON, et est lue à la place de `monitors/monitors.json`.

Pour le déclencheur `when` et les autres champs, consultez [`monitors`](/docs/fr/plugins/manifest-reference#monitors).

<h2 id="user-configuration">
  Demander à l'utilisateur des valeurs de configuration
</h2>

Déclarez les valeurs dont votre plugin a besoin de l'utilisateur dans la clé de manifeste `userConfig`, pour que les utilisateurs ne modifient pas `settings.json` eux-mêmes. Chaque option apparaît dans une boîte de dialogue avec son `title` comme étiquette et sa `description` en dessous.

Définissez `"sensitive": true` pour un jeton ou un mot de passe. La boîte de dialogue masque alors l'entrée, et la valeur est stockée dans un stockage sécurisé plutôt que dans `settings.json`.

Ce manifeste demande un point de terminaison et un jeton :

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
  Quand la boîte de dialogue de configuration apparaît
</h3>

La boîte de dialogue n'apparaît que dans l'interface interactive `/plugin`. Elle s'ouvre pour toute option qui n'est pas encore définie quand l'utilisateur fait l'une des choses suivantes :

* Installe le plugin dans `/plugin`
* Exécute `/plugin install <plugin>@<marketplace>` à l'intérieur d'une session
* Active le plugin à partir de l'onglet **Installed** dans `/plugin`

Pour ouvrir la même boîte de dialogue à tout moment, l'utilisateur exécute `/plugin configure <plugin>@<marketplace>`.

La commande shell `claude plugin install` ne demande jamais les valeurs `userConfig`. Pour définir les valeurs à partir du shell, passez chacune comme `--config KEY=VALUE`. Quand les options restent non définies, la commande imprime une ligne `userConfig options not yet set` qui nomme les deux façons de les définir. [La boîte de dialogue `userConfig` ne s'affiche jamais](/docs/fr/plugins/troubleshooting#the-userconfig-dialog-never-appears) cite la ligne.

Pour les champs d'option, où chaque valeur est stockée, comment un composant référence une valeur enregistrée, et quels champs rejettent `${user_config.*}`, consultez [Configuration utilisateur](/docs/fr/plugins/manifest-reference#user-configuration).

<h2 id="path-variables-and-persistent-data">
  Référencer les chemins du plugin et stocker les données
</h2>

Vous ne savez pas où votre plugin sera installé, donc référencez ses fichiers et données via ces variables plutôt que des chemins fixes. Ils sont substitués dans le contenu des skills, commandes et agents, dans les commandes des hooks et moniteurs, et dans les configurations des serveurs MCP et LSP. Ils sont aussi exportés aux processus des hooks, MCP et LSP :

* **`${CLAUDE_PLUGIN_ROOT}`** : le répertoire d'installation du plugin. Chaque version a son propre [répertoire de cache](/docs/fr/plugins/loading#find-plugins-on-disk), donc le chemin change quand le plugin se met à jour. N'écrivez pas d'état là
* **`${CLAUDE_PLUGIN_DATA}`** : un répertoire qui survit aux mises à jour, pour `node_modules`, les environnements virtuels, et les caches. Il se résout en `~/.claude/plugins/data/<id>/` et est créé quand d'abord référencé
* **`${CLAUDE_PROJECT_DIR}`** : la racine du projet, la même valeur que les hooks reçoivent

Dans le chemin du répertoire de données, `<id>` est l'identifiant du plugin avec chaque caractère autre que les lettres, les chiffres, `_`, et `-` remplacé par `-`, donc `my-plugin@my-marketplace` devient `my-plugin-my-marketplace`.

Sur Windows, les chemins substitués utilisent des barres obliques avant pour qu'un shell ne lise pas les barres obliques arrière comme des échappements.

<h3 id="install-dependencies-into-the-data-directory">
  Installer les dépendances dans le répertoire de données
</h3>

Pour un plugin installé depuis la marketplace, Claude Code installe automatiquement les [dépendances de package Node.js](/docs/fr/plugins/loading#node-js-package-dependencies) éligibles quand il met en cache le plugin, donc vous n'aurez peut-être pas besoin de les installer vous-même. Quand vous le faites, ce hook `SessionStart` installe `node_modules` dans `${CLAUDE_PLUGIN_DATA}` à la première exécution et à nouveau après une mise à jour qui change `package.json` :

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

Après la première session, `~/.claude/plugins/data/<id>/node_modules` existe. Un serveur MCP peut alors définir `NODE_PATH` à `${CLAUDE_PLUGIN_DATA}/node_modules` dans son `env`. Pour quels champs substituent quelle variable, consultez [Variables d'environnement](/docs/fr/plugins/manifest-reference#environment-variables).

<h2 id="next-steps">
  Étapes suivantes
</h2>

* [Référence du manifeste du plugin](/docs/fr/plugins/manifest-reference) : champs `plugin.json`, règles de chemin, et la disposition standard
* [Tester les plugins avec des evals](/docs/fr/plugin-evals) : vérifiez que les composants que vous avez ajoutés changent le comportement de Claude de la façon que vous avez l'intention
* [Publier et distribuer un plugin](/docs/fr/plugins/publish) : versionnez le plugin et mettez-le dans une marketplace
* [Dépanner les plugins](/docs/fr/plugins/troubleshooting) : quoi faire quand un composant ne charge pas ou qu'un hook ne se déclenche pas
