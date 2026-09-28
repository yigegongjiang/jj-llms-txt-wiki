> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Adicionar componentes a um plugin

> Adicione skills, hooks, servidores MCP e todos os outros tipos de componentes a um plugin Claude Code, com um exemplo que valida cada um.

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

Um plugin Claude Code é construído a partir de componentes, como skills, agentes, hooks e servidores MCP. Cada componente tem uma pasta padrão no plugin, uma chave de manifesto opcional em `.claude-plugin/plugin.json` que substitui ou adiciona àquela pasta, e um nome que o usuário vê. Para cada tabela de campos completa da chave, consulte a [referência de manifesto](/docs/pt/plugins/manifest-reference#fields).

Use esta página para adicionar um componente a um plugin que já carrega.

Depois de adicionar um componente, execute `/reload-plugins` em uma sessão em execução ou inicie uma nova para que Claude Code o carregue. Para verificar o arquivo do componente antes de carregá-lo, execute [`claude plugin validate .`](/docs/pt/plugins/cli-reference#plugin-validate) no seu shell a partir do diretório do plugin.

<Note>
  Estes casos são cobertos em outras páginas:

  * **Construindo seu primeiro plugin**: comece com [Criar um plugin](/docs/pt/plugins/create)
  * **Instalando o plugin de outra pessoa**: consulte [Instalar plugins](/docs/pt/plugins/install)
  * **Os usuários do seu plugin estão em claude.ai ou em Cowork**: um conjunto diferente de componentes carrega lá. Consulte [Plugins em claude.ai e em Cowork](https://claude.com/docs/plugins/overview)
</Note>

<h2 id="explore-the-plugin-directory">
  Explorar o diretório do plugin
</h2>

O explorador mostra um plugin de exemplo, `my-plugin`, que tem um de cada tipo de componente em sua localização padrão:

* Uma skill de revisão e um comando `about`
* Um subagente de revisão de segurança
* Um hook que formata arquivos após Claude editá-los, e a pasta `scripts/` que ele chama
* Um monitor de log
* Um estilo de saída e um tema de cor
* Um workflow de auditoria de rotas
* Um executável `hello-plugin`
* Configurações padrão
* Um servidor MCP local e um servidor de linguagem Go

Cada arquivo é o menor exemplo válido de seu formato, lá para mostrar a forma em vez de ser útil: uma skill ou agente real carrega instruções completas e frequentemente arquivos de suporte, e um hook ou monitor real faz trabalho real. As seções após o explorador usam os mesmos arquivos que seus exemplos e vinculam a versões mais completas. Selecione um arquivo ou pasta para ler para que serve, veja o que entra nele e encontre a seção que o cobre.

<PluginExplorer>
  <Piece id="manifest">
    O [manifesto](/docs/pt/plugins/manifest-reference) é o arquivo `plugin.json` no diretório `.claude-plugin/` de um plugin. Ele contém os metadados do plugin e os valores `userConfig` que Claude Code solicita ao usuário. Apenas `name` é obrigatório. Neste, `description` é o texto que os usuários veem para o plugin em `/plugin`, e `version` mantém os usuários nessa versão até você alterá-la:

    ```json theme={null}
    {
      "name": "my-plugin",
      "version": "1.0.0",
      "description": "Review, formatting, and database tools for this team"
    }
    ```
  </Piece>

  <Piece id="skills">
    Uma [skill](/docs/pt/skills) é um arquivo `SKILL.md`. Salve cada skill em seu próprio diretório em `skills/`. Claude lê a `description` de cada skill, e quando o que o usuário pede corresponde a ela, como pedir a Claude para revisar um pull request aqui, Claude carrega as instruções da skill e as segue. O usuário também pode executá-la diretamente como `/my-plugin:review`:

    ```markdown theme={null}
    ---
    description: Reviews a pull request for style and test coverage. Use when asked to review code.
    ---

    Review the changed files. Report style problems first, then missing tests.
    ```
  </Piece>

  <Piece id="commands">
    Um comando é um único arquivo Markdown que o usuário executa por nome. Comandos são o formato mais antigo: uma skill é executada por nome da mesma forma e também pode carregar arquivos de suporte em seu próprio diretório, então escreva novos como skills e mantenha `commands/` para arquivos que você já tem. Este arquivo se torna `/my-plugin:about` e usa o mesmo frontmatter que uma skill:

    ```markdown theme={null}
    ---
    description: Summarize the repository
    ---

    Summarize what this repository does in three sentences.
    ```
  </Piece>

  <Piece id="agents">
    Um [subagente](/docs/pt/sub-agents) é um assistente separado, com suas próprias instruções e sua própria janela de contexto, que Claude pode delegar uma tarefa e obter um resultado. Cada arquivo Markdown em `agents/` define um: o frontmatter o nomeia e diz quando usá-lo, e o corpo é seu prompt do sistema. Este é nomeado `my-plugin:security-reviewer`, e o usuário pode invocá-lo com `@agent-my-plugin:security-reviewer`:

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
    Um [hook](/docs/pt/hooks-guide) executa algo automaticamente em um ponto do ciclo de vida do Claude Code, como após cada edição de arquivo: um comando shell, uma solicitação HTTP, uma chamada de ferramenta MCP, um prompt para um modelo ou um subagente. Salve os hooks do plugin em `hooks/hooks.json` na raiz do plugin. Este executa o `scripts/format.sh` do plugin após Claude escrever ou editar um arquivo:

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
    Um monitor é um comando shell que Claude Code inicia em segundo plano quando a sessão inicia e mantém em execução até que termine, usando a [ferramenta Monitor](/docs/pt/tools-reference#monitor-tool). O que ele imprime chega a Claude como notificações. Um campo `when` pode, em vez disso, iniciá-lo na primeira vez que uma skill nomeada é executada. Este monitora um log de erros:

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
    Um plugin pode incluir [estilos de saída](/docs/pt/output-styles), que alteram como Claude formata e expressa suas respostas. Salve cada estilo de saída como `output-styles/<name>.md`. Este aparece em `/output-style` como `my-plugin:terse`:

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
    Um plugin pode incluir [temas de cor](/docs/pt/terminal-config#create-a-custom-theme) para a interface Claude Code. Salve cada tema como `themes/<slug>.json`. Este aparece em `/theme` como `Dracula`, marcado como de `my-plugin`:

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
    A pasta `workflows/` contém arquivos `.js` de [workflow](/docs/pt/workflows): um bloco `meta`, depois um corpo de script que orquestra vários subagentes. Este é executado como `/my-plugin:audit-routes`:

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
    `bin/` é como um plugin envia uma ferramenta de linha de comando. Enquanto o plugin está habilitado, Claude Code coloca esta pasta no `PATH` do shell em que executa comandos, para que Claude, ou as instruções de uma skill, possam executar a ferramenta por nome sem o usuário instalar nada. Com este [executável](#executables) em vigor, `hello-plugin` é um comando que Claude pode executar:

    ```bash theme={null}
    #!/bin/bash
    echo "hello from my-plugin"
    ```
  </Piece>

  <Piece id="scripts">
    O hook em `hooks/hooks.json` executa um script, e esta pasta é onde o exemplo o mantém. O nome `scripts/` é uma convenção, não algo que Claude Code procure: o hook aponta para o arquivo por seu caminho, `${CLAUDE_PLUGIN_ROOT}/scripts/format.sh`. Um script de formatação pode parecer assim:

    ```bash theme={null}
    #!/bin/bash
    npx prettier --write .
    ```
  </Piece>

  <Piece id="settings">
    Um `settings.json` na raiz do plugin contém [configurações](/docs/pt/settings-reference) que se aplicam enquanto o plugin está habilitado, para que um plugin possa alterar como a sessão se comporta e não apenas adicionar componentes. Apenas duas chaves têm efeito de um plugin, [`agent`](/docs/pt/settings-reference#agent) e [`subagentStatusLine`](/docs/pt/settings-reference#subagentstatusline); todas as outras chaves são descartadas. Consulte [Configurações padrão](#default-settings).

    Este define `agent`, que executa o thread principal da sessão como o agente `security-reviewer` do plugin, para que o prompt do sistema, restrições de ferramentas e modelo desse agente se apliquem a toda a sessão:

    ```json theme={null}
    {
      "agent": "security-reviewer"
    }
    ```
  </Piece>

  <Piece id="mcp">
    Um [servidor MCP](/docs/pt/mcp) fornece a Claude ferramentas de um sistema externo. Declare-o em `.mcp.json` na raiz do plugin. Este inicia um servidor local a partir de um script dentro do plugin e aparece em `/mcp` como `plugin:my-plugin:db`:

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
    Um servidor LSP fornece a Claude [diagnósticos e navegação de código](/docs/pt/plugins/code-intelligence) para uma linguagem. Declare o servidor em `.lsp.json` na raiz do plugin. Este conecta o servidor de linguagem Go para arquivos `.go`:

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
  Adicionar cada tipo de componente
</h2>

Cada seção abaixo cobre um tipo de componente: onde seus arquivos vão no plugin, um exemplo que valida, o que o usuário vê uma vez que o plugin carrega, e a chave de manifesto que altera a localização padrão. Adicione os que seu plugin precisa; nenhum é obrigatório.

<h3 id="skills">
  Skills
</h3>

Uma [skill](/docs/pt/skills) é um arquivo `SKILL.md` que Claude pode carregar quando sua descrição corresponde à tarefa. O usuário também pode executá-la como um comando. Salve cada skill em seu próprio diretório em `skills/`:

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json
└── skills/
    └── review/
        └── SKILL.md
```

Dê ao `SKILL.md` uma `description` para que Claude saiba quando usá-la:

```markdown skills/review/SKILL.md theme={null}
---
description: Reviews a pull request for style and test coverage. Use when asked to review code.
---

Review the changed files. Report style problems first, then missing tests.
```

Depois de carregar o plugin, `/my-plugin:review` executa a skill. O nome do comando e quem pode invocá-lo seguem estas regras:

* **Nome do comando**: `/<plugin>:<directory>`, então `skills/review/SKILL.md` em `my-plugin` é `/my-plugin:review`. Se você definir `name` no frontmatter, ele substitui o último segmento e o prefixo do plugin permanece. Consulte [como uma skill obtém seu nome de comando](/docs/pt/skills#how-a-skill-gets-its-command-name)
* **Quem a invoca**: Claude, o usuário ou ambos, controlado pelo frontmatter. Consulte [Controlar quem invoca uma skill](/docs/pt/skills#control-who-invokes-a-skill)

Você também pode colocar skills fora do diretório padrão `skills/`:

* **Diretórios adicionais**: liste-os na chave de manifesto `skills`. Eles adicionam à varredura padrão `skills/` em vez de substituí-la, diferentemente de `commands` e `agents`
* **Uma única skill na raiz do plugin**: sem diretório `skills/` e sem chave de manifesto `skills`, um `SKILL.md` na raiz do plugin carrega como uma skill. Defina `name` em seu frontmatter, porque caso contrário uma instalação de marketplace nomeia a skill após seu [diretório de cache](/docs/pt/plugins/loading#find-plugins-on-disk) em vez de seu plugin

Para incluir instruções em um plugin, escreva-as como uma skill. Claude Code não carrega um `CLAUDE.md` na raiz do plugin, e `claude plugin validate` avisa `CLAUDE.md at the plugin root is not loaded as project context`.

Para campos de frontmatter e arquivos de suporte, consulte [Skills](/docs/pt/skills).

<h3 id="commands">
  Comandos
</h3>

Um comando é um único arquivo Markdown que o usuário executa por nome, como `/my-plugin:about`.

<Note>
  Comandos são o formato mais antigo, e [skills](#skills) os superam para novo trabalho. Uma skill é executada por nome da mesma forma, e também pode carregar arquivos de suporte em seu diretório. Mantenha `commands/` para arquivos que você está movendo de `.claude/commands/`.
</Note>

Salve um comando em `commands/<file>.md` e ele se torna `/<plugin>:<file>`. Um subdiretório adiciona um segmento, então `commands/db/migrate.md` é `/my-plugin:db:migrate`.

Arquivos de comando usam o mesmo frontmatter que skills.

<h4 id="define-commands-in-the-manifest">
  Definir comandos no manifesto
</h4>

Você só precisa disso se quiser manter arquivos de comando em algum lugar diferente de `commands/`, ou para definir um comando curto dentro de `plugin.json` sem um arquivo Markdown separado. Defina a chave de manifesto `commands`, e Claude Code a lê em vez de varrer `commands/`. A chave usa um caminho, uma matriz de caminhos ou um objeto que mapeia cada nome de comando para um arquivo `source` ou `content` inline.

Este manifesto define `/my-plugin:about` inline, sem arquivo Markdown:

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

Carregue o plugin e execute `/my-plugin:about` na sessão para confirmar que carregou.

Para a sintaxe completa da chave, consulte [`commands`](/docs/pt/plugins/manifest-reference#commands).

<h3 id="agents">
  Agentes
</h3>

Um [subagente](/docs/pt/sub-agents) é um assistente separado, com suas próprias instruções e janela de contexto, que Claude pode delegar uma tarefa. Cada arquivo Markdown em `agents/` define um:

```markdown agents/security-reviewer.md theme={null}
---
name: security-reviewer
description: Reviews code changes for security issues. Use after edits to authentication or input handling.
model: sonnet
---

You are a security reviewer. Read the changed files and report injection, authentication, and secrets-handling risks.
```

Este agente é nomeado `my-plugin:security-reviewer`, e o usuário pode [invocá-lo explicitamente](/docs/pt/sub-agents#invoke-subagents-explicitly) com `@agent-my-plugin:security-reviewer`. A forma do nome é `<plugin>:<name>`, onde `<name>` vem do frontmatter, ou do nome do arquivo quando não há.

A chave `agents` substitui a varredura `agents/`.

<h4 id="organize-agents-in-subfolders">
  Organizar agentes em subpastas
</h4>

Você pode colocar arquivos de agente do plugin em subpastas de `agents/`. Claude Code [os carrega recursivamente](/docs/pt/sub-agents#choose-the-subagent-scope) e une o nome do plugin, cada nome de subpasta e o nome do arquivo com dois-pontos para formar o nome com escopo do agente. Por exemplo, `agents/review/security.md` em um plugin nomeado `my-plugin` carrega como `my-plugin:review:security`. Duas configurações alteram esse nome:

* Frontmatter `name`: ele substitui apenas o nome do arquivo, então `name: audit` em `agents/review/security.md` carrega como `my-plugin:review:audit`
* Campo de manifesto [`agents`](/docs/pt/plugins/manifest-reference#fields): um arquivo que você lista lá carrega sem nomes de subpasta, então `"agents": "./custom/review/security.md"` carrega como `my-plugin:security`

<h4 id="frontmatter-fields-in-plugin-agents">
  Campos de frontmatter em agentes de plugin
</h4>

O frontmatter de um agente de plugin segue estas regras:

* **Campos suportados**: `name`, `description`, `model`, `effort`, `maxTurns`, `tools`, `disallowedTools`, `skills`, `memory`, `background`, `omitClaudeMd`, `isolation`, `color` e a chave `cacheTtl` de `experimental`. O único valor `isolation` válido é `"worktree"`. Consulte [campos de frontmatter suportados](/docs/pt/sub-agents#supported-frontmatter-fields) para saber o que cada um faz
* **Campos ignorados**: `permissionMode`, `hooks`, `mcpServers` e `initialPrompt`. Um arquivo de agente não pode adicionar hooks ou servidores MCP por conta própria, então adicione-os como plugin [hooks](#hooks) e [servidores MCP](#mcp-servers) em vez disso
* **Frontmatter que não analisa**: o agente ainda carrega com cada campo ignorado. É nomeado após o arquivo, e sua descrição lê `Agent from my-plugin plugin`. Execute [`claude plugin validate`](/docs/pt/plugins/cli-reference#plugin-validate) no seu shell para encontrar esses arquivos

Para saber o que cada campo faz e as regras de precedência, consulte [Subagentes](/docs/pt/sub-agents#supported-frontmatter-fields).

<h3 id="hooks">
  Hooks
</h3>

Um [hook](/docs/pt/hooks-guide) executa algo automaticamente em um ponto do ciclo de vida do Claude Code, como após cada edição de arquivo: um comando shell, uma solicitação HTTP, uma chamada de ferramenta MCP, um prompt para um modelo ou um subagente. Salve os hooks do plugin em `hooks/hooks.json` na raiz do plugin, sob uma chave `"hooks"` de nível superior, na mesma forma que o objeto `hooks` em `settings.json`. Isso permite copiar um hook de configurações existente sem alterações.

Este hook executa um script agrupado após cada `Write` ou `Edit`:

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

Salve o script em `scripts/format.sh` e torne-o executável.

Carregue o plugin e peça a Claude para editar um arquivo. Um hook `PostToolUse` que sai com 0 não mostra nada na transcrição, então confirme que foi executado com [log de depuração](/docs/pt/hooks#debug-hooks) ou pelo que o script em si altera.

Hooks em `hooks/hooks.json` e na chave de manifesto `hooks` ambos carregam. Para cada evento e sua carga útil, consulte [Eventos de hook](/docs/pt/hooks#hook-events).

<h4 id="when-plugin-hooks-fire">
  Quando os hooks do plugin disparam
</h4>

Os hooks de um plugin não esperam que uma das skills ou comandos do plugin seja usada. Claude Code os registra quando uma sessão carrega o plugin, e eles disparam em seus eventos a partir de então. Para limitar quando um hook é executado, restrinja seu `matcher`.

Se um hook nunca dispara, consulte [hooks que não disparam](/docs/pt/plugins/troubleshooting#failed-to-load-hooks-from-and-hooks-that-dont-fire).

<h4 id="environment-quoting-and-matching-mcp-tools">
  Ambiente, citação e correspondência de ferramentas MCP
</h4>

O ambiente do hook, a citação de `${CLAUDE_PLUGIN_ROOT}` e os matchers para as próprias ferramentas MCP do plugin funcionam da seguinte forma:

* **Ambiente**: cada processo de hook recebe `CLAUDE_PLUGIN_ROOT` e `CLAUDE_PLUGIN_DATA` em seu ambiente, mais `CLAUDE_PLUGIN_OPTION_<KEY>` para cada valor de [configuração do usuário](#user-configuration), para que seu script possa lê-los de lá
* **Citação**: quando `command` não tem `args`, ele é executado através de um shell, então envolva o caminho `${CLAUDE_PLUGIN_ROOT}` em aspas duplas, como o exemplo `hooks/hooks.json` em [Hooks](#hooks) faz, para manter o caminho expandido como uma palavra de shell. Quando você passa `args` em vez disso, cada elemento é passado como um argumento sem shell e não precisa de citação. Consulte [forma exec e forma shell](/docs/pt/hooks#exec-form-and-shell-form)
* **Correspondência das próprias ferramentas MCP do plugin**: uma ferramenta de um [servidor MCP que este plugin declara](#mcp-servers) é nomeada `mcp__plugin_<plugin>_<server>__<tool>`, então escreva esse nome completo no matcher. Um matcher apenas no nome do servidor nunca dispara. Consulte [Corresponder ferramentas MCP](/docs/pt/hooks#match-mcp-tools)

<h3 id="mcp-servers">
  Servidores MCP
</h3>

Um servidor MCP fornece a Claude ferramentas de um sistema externo. Declare-o em `.mcp.json` na raiz do plugin, na mesma forma que um [`.mcp.json` de projeto](/docs/pt/mcp#project-scope). Este `.mcp.json` declara um servidor nomeado `db`:

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

Você também pode omitir o wrapper `mcpServers` e colocar `db` no nível superior do arquivo.

Carregue o plugin e execute `/mcp` para confirmar que o servidor aparece como `plugin:my-plugin:db`.

`claude plugin validate` verifica `.mcp.json` e relata uma entrada de servidor que Claude Code descartaria no tempo de carregamento como um erro. Requer Claude Code v2.1.281 ou posterior.

Para onde uma entrada ruim aparece no tempo de carregamento, consulte [Servidores MCP que não iniciam](/docs/pt/plugins/troubleshooting#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start).

A chave de manifesto `mcpServers` usa um mapa de servidor inline, um caminho para um arquivo JSON ou uma matriz daqueles. Quando um servidor de manifesto tem o mesmo nome que um em `.mcp.json`, o servidor de manifesto o substitui.

<h4 id="reach-users-on-claude-ai-and-cowork">
  Alcançar usuários em claude.ai e Cowork
</h4>

Um servidor stdio local, como o servidor `db` em [Servidores MCP](#mcp-servers), é executado em Claude Code e em uma sessão Cowork que é executada em sua máquina no aplicativo Claude Desktop, mas não em claude.ai. Para alcançar usuários lá também, referencie um servidor remoto por sua URL `https://`, que claude.ai e Cowork oferecem ao usuário como um conector.

<h4 id="server-names-tool-names-and-reloads">
  Nomes de servidor, nomes de ferramentas e recarregamentos
</h4>

Os nomes do servidor, substituição de variáveis e comportamento de recarga seguem estas regras:

* **Nome do servidor**: `plugin:<plugin>:<server>`, então o servidor `db` em `my-plugin` é `plugin:my-plugin:db` em `/mcp`. Use a mesma forma para nomear o servidor em um [hook `mcp_tool`](/docs/pt/hooks#mcp-tool-hook-fields)
* **Nomes de ferramentas**: `mcp__plugin_<plugin>_<server>__<tool>`, então uma ferramenta `query` naquele servidor `db` é `mcp__plugin_my-plugin_db__query`. Esse é o nome a usar em [regras de permissão](/docs/pt/permissions) e [matchers de hook](#hooks)
* **Substituição**: `${CLAUDE_PLUGIN_ROOT}` e as outras [variáveis de caminho](#path-variables-and-persistent-data) são substituídas em `command`, `args` e `env`. Nenhuma citação é necessária em `args`, porque cada elemento é passado como um argumento
* **Recarga**: quando o usuário executa `/reload-plugins` e [o recarga se aplica](/docs/pt/plugins/cli-reference#reloads-that-change-mcp-tools), um servidor cuja configuração não foi alterada mantém sua conexão. Um servidor cuja configuração mudou se reconecta, e um que você removeu se desconecta

<h4 id="include-a-packaged-mcpb-server">
  Incluir um servidor MCPB empacotado
</h4>

A chave `mcpServers` também aceita um servidor empacotado como um [arquivo MCPB](https://github.com/modelcontextprotocol/mcpb), cuja extensão é `.mcpb` ou a mais antiga `.dxt`. Aponte a chave para o arquivo, como um caminho dentro do plugin ou uma URL `https://`:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "my-plugin",
  "mcpServers": "./servers/db.mcpb"
}
```

O servidor usa seu nome do `name` no manifesto do pacote.

Para transportes e autenticação, consulte [MCP](/docs/pt/mcp#plugin-provided-mcp-servers).

<h3 id="lsp-servers">
  Servidores LSP
</h3>

Um servidor LSP fornece a Claude diagnósticos e navegação de código para uma linguagem. Se um [plugin oficial de inteligência de código](/docs/pt/plugins/code-intelligence) já cobre sua linguagem, instale esse em vez de escrever um. Caso contrário, declare o servidor em `.lsp.json` na raiz do plugin:

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

O arquivo mapeia cada nome de servidor diretamente para sua configuração, sem objeto wrapper ao redor do mapa. `command` é o nome do binário, com seus argumentos em `args`. `extensionToLanguage` precisa de pelo menos uma extensão, cada uma começando com `.`.

`claude plugin validate` não lê este arquivo. Quando qualquer entrada é inválida, o arquivo inteiro é ignorado no carregamento e `Invalid LSP server config for ".lsp.json"` aparece na aba **Errors** de `/plugin`.

Seu plugin configura a conexão mas não instala o binário do servidor, e cada extensão de arquivo obtém um servidor:

* **Binário ausente**: Claude Code inicia `command` por nome do `PATH` do usuário. Quando o binário não está lá, o servidor falha ao iniciar e `claude --debug` registra `LSP server <name> failed to start`
* **Conflitos de extensão**: quando dois servidores habilitados reivindicam a mesma extensão, o primeiro registrado manipula esses arquivos e o outro não é usado para eles, se os servidores vêm de um plugin ou dois. A aba **Errors** de `/plugin` mostra o aviso `LSP server "<name>" is not used for <ext> files`

A chave de manifesto `lspServers` usa o mesmo mapa inline, um caminho para um arquivo JSON ou uma matriz daqueles, e seus servidores adicionam aos em `.lsp.json`. Quando um servidor de manifesto tem o mesmo nome que um em `.lsp.json`, o servidor de manifesto o substitui.

Para `transport`, timeouts, reinicializações e os outros campos, consulte [`lspServers`](/docs/pt/plugins/manifest-reference#lspservers).

Envie a saída de log para stderr, não stdout. Claude Code lê stdout de um servidor apenas como mensagens de protocolo e aceita cabeçalhos de mensagem até 64 KiB e um corpo de mensagem até 32 MiB.

Claude Code desconecta um servidor que excede qualquer limite ou escreve saída não-protocolo para stdout, e conta a desconexão como uma falha para `restartOnCrash` e `maxRestarts`. Quando você executa com `--debug`, Claude Code escreve um erro nomeando a causa para o log de depuração.

<h3 id="executables">
  Executáveis
</h3>

Arquivos em `bin/` na raiz do plugin estão no `PATH` do shell da ferramenta Bash enquanto o plugin está habilitado, para que Claude possa executá-los como comandos simples. Adicione um script executável:

```bash bin/hello-plugin theme={null}
#!/bin/bash
echo "hello from my-plugin"
```

Torne-o executável com `chmod +x bin/hello-plugin` e carregue o plugin. Quando você pede a Claude para executar `hello-plugin`, o resultado da ferramenta Bash mostra a saída do script.

Diretórios `bin/` de plugin vêm após as entradas `PATH` do próprio usuário, então um plugin não pode sombrear `git`, `ls` ou outro comando do sistema.

claude.ai e Cowork não instalam um plugin que tem um diretório `bin/` de nível superior, incluindo um que você [distribui através das configurações da organização claude.ai](/docs/pt/plugins/host-marketplace#distribute-through-organization-settings).

<h3 id="default-settings">
  Configurações padrão
</h3>

Para definir padrões que se aplicam enquanto o plugin está habilitado, adicione um `settings.json` na raiz do plugin, ou coloque o mesmo objeto inline na chave de manifesto `settings`. Duas chaves têm efeito, `agent` e `subagentStatusLine`, e todas as outras chaves são descartadas.

Defina `agent` para executar um dos próprios agentes do plugin como o thread principal:

```json settings.json theme={null}
{
  "agent": "security-reviewer"
}
```

Carregue o plugin e inicie uma sessão. Claude então responde na conversa principal com o prompt do sistema e modelo do agente `security-reviewer`.

Para tudo que a chave controla, consulte a [configuração `agent`](/docs/pt/settings-reference#agent).

Quando a mesma chave é definida em mais de um lugar, estas regras decidem qual valor se aplica:

* **Arquivo sobre manifesto**: quando ambos existem e `settings.json` define pelo menos uma chave suportada, `settings.json` se aplica e o `settings` do manifesto é ignorado
* **Configurações do usuário sobre padrões do plugin**: entre fontes de configurações, padrões de plugin são a camada mais baixa, então um `agent` próprio do usuário em `~/.claude/settings.json` substitui o seu
* **Dois plugins definem a mesma chave**: o valor do plugin carregado por último se aplica, e `claude --debug` registra `overrides setting`

Para a forma `subagentStatusLine`, consulte [linhas de status de subagente](/docs/pt/statusline#subagent-status-lines).

<h3 id="themes-and-output-styles">
  Temas e estilos de saída
</h3>

Um plugin pode incluir temas de cor e estilos de saída. Ambos aparecem nos mesmos seletores que os do usuário. Para qualquer um, definir a chave de manifesto substitui a varredura de pasta.

| Componente      | Salvar como               | Formato                                                                                                                                 | Aparece em                              | Chave de manifesto    |
| :-------------- | :------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------- | :-------------------- |
| Tema            | `themes/<slug>.json`      | O formato de [arquivo de tema personalizado](/docs/pt/terminal-config#create-a-custom-theme) que os usuários escrevem em `~/.claude/themes/` | `/theme`, sob o `name` do arquivo       | `experimental.themes` |
| Estilo de saída | `output-styles/<name>.md` | O formato de [estilo de saída personalizado](/docs/pt/output-styles#create-a-custom-output-style), com frontmatter `name` e `description`    | `/output-style`, como `<plugin>:<name>` | `outputStyles`        |

Temas de plugin são somente leitura, então quando um usuário edita um em `/theme`, a edição é salva como uma cópia no diretório de temas próprio.

Este tema recolore o prompt de acento e texto de erro na predefinição escura:

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
  Canais
</h3>

Um [canal](/docs/pt/channels) permite que um sistema externo, como um aplicativo de chat, envie mensagens para uma sessão. Em um plugin, um canal é um dos servidores MCP mais uma entrada `channels` que se vincula a ele e pode solicitar sua própria configuração. Este manifesto vincula um canal a um servidor `telegram` e solicita um token de bot:

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

`server` deve corresponder a uma chave em `mcpServers`. O `userConfig` por canal usa a mesma forma que a chave [`userConfig` de nível superior](#user-configuration).

Para o que o servidor deve implementar e como os usuários habilitam um plugin de canal, consulte [Empacotar como um plugin](/docs/pt/channels-reference#package-as-a-plugin) na referência de canais. Para a tabela de campos, consulte [`channels`](/docs/pt/plugins/manifest-reference#channels).

<h3 id="monitors">
  Monitores
</h3>

Um monitor é um comando shell que é executado em segundo plano para toda a sessão. O que ele imprime chega a Claude como notificações, para que Claude possa reagir a um log ou mudança de status sem ser solicitado a observá-lo. Salve as entradas em `monitors/monitors.json`:

```json monitors/monitors.json theme={null}
[
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Application error log"
  }
]
```

O comando é executado em um shell, no diretório de trabalho em que a sessão foi iniciada.

O comando de um monitor é limitado em onde inicia e o que pode referenciar:

* **Apenas sessões interativas**: monitores de plugin iniciam em uma sessão interativa e nunca em modo não-interativo com a flag `-p`. Eles também iniciam apenas onde a [ferramenta Monitor](/docs/pt/tools-reference#monitor-tool) está disponível
* **Sem configuração do usuário**: `command` obtém as [variáveis de caminho](#path-variables-and-persistent-data) e `${ENV_VAR}` do ambiente, mas nunca `${user_config.*}`. Um monitor que referencia um não inicia, e processos de monitor não recebem `CLAUDE_PLUGIN_OPTION_<KEY>` também
* **Desabilitação no meio da sessão**: se você desabilitar um plugin no meio da sessão, Claude Code não para monitores que já estão em execução. Eles param quando a sessão termina

A chave de manifesto `experimental.monitors` usa a mesma matriz inline ou um caminho para um arquivo JSON, e é lida em vez de `monitors/monitors.json`.

Para o gatilho `when` e os outros campos, consulte [`monitors`](/docs/pt/plugins/manifest-reference#monitors).

<h2 id="user-configuration">
  Solicitar ao usuário valores de configuração
</h2>

Declare os valores que seu plugin precisa do usuário na chave de manifesto `userConfig`, para que os usuários não editem `settings.json` eles mesmos. Cada opção aparece em um diálogo com seu `title` como o rótulo e sua `description` abaixo.

Defina `"sensitive": true` para um token ou senha. O diálogo então mascara a entrada, e o valor é armazenado em armazenamento seguro em vez de `settings.json`.

Este manifesto solicita um endpoint e um token:

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
  Quando o diálogo de configuração aparece
</h3>

O diálogo aparece apenas na interface interativa `/plugin`. Ele abre para qualquer opção que ainda não está definida quando o usuário faz qualquer um dos seguintes:

* Instala o plugin em `/plugin`
* Executa `/plugin install <plugin>@<marketplace>` dentro de uma sessão
* Habilita o plugin da aba **Installed** em `/plugin`

Para abrir o mesmo diálogo a qualquer momento, o usuário executa `/plugin configure <plugin>@<marketplace>`.

O comando shell `claude plugin install` nunca solicita valores `userConfig`. Para definir valores do shell, passe cada um como `--config KEY=VALUE`. Quando opções permanecem indefinidas, o comando imprime uma linha `userConfig options not yet set` que nomeia ambas as formas de defini-las. [O diálogo `userConfig` nunca aparece](/docs/pt/plugins/troubleshooting#the-userconfig-dialog-never-appears) cita a linha.

Para os campos de opção, onde cada valor é armazenado, como um componente referencia um valor salvo e quais campos rejeitam `${user_config.*}`, consulte [Configuração do usuário](/docs/pt/plugins/manifest-reference#user-configuration).

<h2 id="path-variables-and-persistent-data">
  Referenciar caminhos de plugin e armazenar dados
</h2>

Você não sabe onde seu plugin será instalado, então refira-se a seus arquivos e dados através destas variáveis em vez de caminhos fixos. Elas são substituídas em conteúdo de skill, comando e agente, em comandos de hook e monitor, e em configurações de servidor MCP e LSP. Elas também são exportadas para processos de hook, MCP e LSP:

* **`${CLAUDE_PLUGIN_ROOT}`**: o diretório de instalação do plugin. Cada versão tem seu próprio [diretório de cache](/docs/pt/plugins/loading#find-plugins-on-disk), então o caminho muda quando o plugin é atualizado. Não escreva estado lá
* **`${CLAUDE_PLUGIN_DATA}`**: um diretório que sobrevive a atualizações, para `node_modules`, ambientes virtuais e caches. Ele se resolve para `~/.claude/plugins/data/<id>/` e é criado quando primeiro referenciado
* **`${CLAUDE_PROJECT_DIR}`**: a raiz do projeto, o mesmo valor que hooks recebem

No caminho do diretório de dados, `<id>` é o identificador do plugin com cada caractere diferente de letras, dígitos, `_` e `-` substituído por `-`, então `my-plugin@my-marketplace` se torna `my-plugin-my-marketplace`.

No Windows, os caminhos substituídos usam barras para frente para que um shell não leia barras invertidas como escapes.

<h3 id="install-dependencies-into-the-data-directory">
  Instalar dependências no diretório de dados
</h3>

Para um plugin instalado no marketplace, Claude Code instala [dependências de pacote Node.js](/docs/pt/plugins/loading#node-js-package-dependencies) elegíveis automaticamente quando armazena em cache o plugin, então você pode não precisar instalá-las você mesmo. Quando você faz, este hook `SessionStart` instala `node_modules` em `${CLAUDE_PLUGIN_DATA}` na primeira execução e novamente após uma atualização alterar `package.json`:

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

Após a primeira sessão, `~/.claude/plugins/data/<id>/node_modules` existe. Um servidor MCP pode então definir `NODE_PATH` para `${CLAUDE_PLUGIN_DATA}/node_modules` em seu `env`. Para quais campos substituem qual variável, consulte [Variáveis de ambiente](/docs/pt/plugins/manifest-reference#environment-variables).

<h2 id="next-steps">
  Próximos passos
</h2>

* [Referência de manifesto de plugin](/docs/pt/plugins/manifest-reference): campos `plugin.json`, regras de caminho e o layout padrão
* [Testar plugins com evals](/docs/pt/plugin-evals): verifique se os componentes que você adicionou alteram o comportamento de Claude da forma que você pretende
* [Publicar e distribuir um plugin](/docs/pt/plugins/publish): versione o plugin e coloque-o em um marketplace
* [Solucionar problemas de plugins](/docs/pt/plugins/troubleshooting): o que fazer quando um componente não carrega ou um hook não dispara
