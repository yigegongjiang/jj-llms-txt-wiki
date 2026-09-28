> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Arquivos de configurações e precedência

> Altere as configurações do Claude Code, escolha o escopo ao qual uma chave pertence, verifique a alteração e aprenda qual valor o Claude Code usa quando uma chave é definida em vários locais.

export const SettingsPrecedence = () => {
  const LEVELS = [{
    n: 1,
    name: 'Managed settings',
    file: 'managed-settings.json, MDM, or the claude.ai console',
    who: 'Your organization',
    w: 390
  }, {
    n: 2,
    name: 'Command line',
    file: 'claude --settings',
    who: 'You, this session',
    w: 420
  }, {
    n: 3,
    name: 'Project local',
    file: '.claude/settings.local.json',
    who: 'You, this project',
    w: 480
  }, {
    n: 4,
    name: 'Shared project',
    file: '.claude/settings.json',
    who: 'Everyone in the project',
    w: 540
  }, {
    n: 5,
    name: 'User',
    file: '~/.claude/settings.json',
    who: 'You, every project',
    w: 600
  }];
  const W = 760;
  const ROW = 58;
  const GAP = 8;
  const TOP = 34;
  const H = TOP + LEVELS.length * (ROW + GAP) + 30;
  const cx = W / 2;
  const mono = 'var(--font-mono, ui-monospace, SFMono-Regular, Menlo, monospace)';
  const sans = 'var(--font-sans, system-ui, -apple-system, sans-serif)';
  return <div className="sp-root not-prose" role="img" aria-label="Settings precedence, highest first: managed settings, command line, project local, shared project, user. A key set at a higher level overrides the same key set lower down.">
      <style>{`
        .sp-root { --sp-text: #1A1918; --sp-sub: #5E5D59; --sp-faint: #8A8880; --sp-fill: #F5F4EF; --sp-stroke: rgba(0,0,0,0.12); --sp-top: #D97757; --sp-top-fill: rgba(217,119,87,0.14); --sp-arrow: #8A8880; margin: 1.25rem 0; }
        .dark .sp-root { --sp-text: #F1EFE9; --sp-sub: #B8B5AD; --sp-faint: #8A8880; --sp-fill: #24231F; --sp-stroke: rgba(255,255,255,0.12); --sp-top-fill: rgba(217,119,87,0.22); --sp-arrow: #8A8880; }
        .sp-root svg { width: 100%; height: auto; display: block; max-width: ${W}px; margin: 0 auto; }
      `}</style>
      <svg viewBox={`0 0 ${W} ${H}`} xmlns="http://www.w3.org/2000/svg">
        <text x={cx} y={18} textAnchor="middle" fontFamily={sans} fontSize="12.5" fontWeight="600" fill="var(--sp-sub)">Highest precedence</text>
        {LEVELS.map((l, i) => {
    const y = TOP + i * (ROW + GAP);
    const x = cx - l.w / 2;
    const top = i === 0;
    return <g key={l.n}>
              <rect x={x} y={y} width={l.w} height={ROW} rx={10} fill={top ? 'var(--sp-top-fill)' : 'var(--sp-fill)'} stroke={top ? 'var(--sp-top)' : 'var(--sp-stroke)'} strokeWidth={top ? 1.5 : 1} />
              <text x={x + 14} y={y + 24} fontFamily={sans} fontSize="14" fontWeight="600" fill="var(--sp-text)">{l.n}. {l.name}</text>
              <text x={x + 14} y={y + 43} fontFamily={mono} fontSize="11.5" fill="var(--sp-sub)">{l.file}</text>
              <text x={x + l.w - 14} y={y + 24} textAnchor="end" fontFamily={sans} fontSize="12" fill="var(--sp-faint)">{l.who}</text>
            </g>;
  })}
        <text x={cx} y={H - 10} textAnchor="middle" fontFamily={sans} fontSize="12.5" fontWeight="600" fill="var(--sp-sub)">Lowest precedence</text>
        <g stroke="var(--sp-arrow)" strokeWidth="1.5" fill="none">
          <line x1={W - 40} y1={TOP + 10} x2={W - 40} y2={H - 38} />
          <path d={`M ${W - 46} ${TOP + 18} L ${W - 40} ${TOP + 10} L ${W - 34} ${TOP + 18}`} />
        </g>
        <text x={W - 40} y={H - 22} textAnchor="middle" fontFamily={sans} fontSize="10.5" fill="var(--sp-faint)">overrides</text>
      </svg>
    </div>;
};

export const SettingsScope = ({defaultSelected = 'project'}) => {
  const FILES = [{
    id: 'user',
    path: '~/.claude/settings.json'
  }, {
    id: 'project',
    path: 'acme-app/.claude/settings.json'
  }, {
    id: 'local',
    path: 'acme-app/.claude/settings.local.json'
  }, {
    id: 'managed',
    path: 'Managed settings',
    ring: 'managed-settings.json, MDM, or the claude.ai console'
  }];
  const SHORT = {
    user: '~/.claude/settings.json',
    project: 'acme-app/.claude/settings.json',
    local: 'acme-app/.claude/settings.local.json',
    managed: 'managed-settings.json, MDM, or the claude.ai console'
  };
  const TILE_MARK = {
    project: 'settings.json',
    local: 'settings.local.json'
  };
  const initial = FILES.some(f => f.id === defaultSelected) ? defaultSelected : 'project';
  const [sel, setSel] = useState(initial);
  const [scale, setScale] = useState(1);
  const [isFullscreen, setIsFullscreen] = useState(false);
  const rootRef = useRef(null);
  const frameRef = useRef(null);
  const CANVAS_W = 862;
  const CANVAS_H = 240;
  useEffect(() => {
    const el = frameRef.current;
    if (!el) return;
    const measure = () => setScale(Math.min(1, el.clientWidth / CANVAS_W));
    measure();
    if (typeof ResizeObserver === 'undefined') {
      window.addEventListener('resize', measure);
      return () => window.removeEventListener('resize', measure);
    }
    const ro = new ResizeObserver(measure);
    ro.observe(el);
    return () => ro.disconnect();
  }, []);
  useEffect(() => {
    const onFsChange = () => setIsFullscreen(!!document.fullscreenElement);
    document.addEventListener('fullscreenchange', onFsChange);
    return () => document.removeEventListener('fullscreenchange', onFsChange);
  }, []);
  const toggleFullscreen = () => {
    if (!rootRef.current) return;
    if (document.fullscreenElement) document.exitFullscreen(); else rootRef.current.requestFullscreen().catch(() => {});
  };
  const COVERAGE = {
    user: ['website', 'api', 'yacme'],
    project: ['yacme', 'tacme', 'cacme'],
    local: ['yacme'],
    managed: ['website', 'api', 'yacme', 'tacme', 'cacme']
  };
  const RINGS = {
    local: {
      l: 282,
      t: 50,
      w: 142,
      h: 124
    },
    project: {
      l: 282,
      t: 50,
      w: 560,
      h: 124
    },
    user: {
      l: 2,
      t: 34,
      w: 446,
      h: 198
    },
    managed: {
      l: 0,
      t: 32,
      w: 862,
      h: 204
    }
  };
  const TILES = [{
    id: 'website',
    name: 'website/',
    left: 30,
    caption: ''
  }, {
    id: 'api',
    name: 'api/',
    left: 160,
    caption: ''
  }, {
    id: 'yacme',
    name: 'acme-app/',
    left: 290,
    caption: ''
  }, {
    id: 'tacme',
    name: 'acme-app/',
    left: 497,
    caption: sel === 'project' ? 'their clone, once you commit the file' : 'their clone'
  }, {
    id: 'cacme',
    name: 'acme-app/',
    left: 704,
    caption: sel === 'project' ? 'fresh clone, once you commit the file' : sel === 'managed' ? 'server-managed only' : 'fresh clone'
  }];
  const FILE_AT = {
    user: {
      machine: 'you',
      tiles: []
    },
    project: {
      machine: null,
      tiles: ['yacme', 'tacme', 'cacme']
    },
    local: {
      machine: null,
      tiles: ['yacme']
    },
    managed: {
      machine: null,
      tiles: []
    }
  };
  const fileAt = FILE_AT[sel];
  const coverage = COVERAGE[sel];
  const ring = RINGS[sel];
  const selFile = FILES.find(f => f.id === sel);
  const FolderIcon = ({open}) => <svg width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.3" strokeLinejoin="round" aria-hidden="true">
      <path d="M1.5 4.5a1 1 0 0 1 1-1h3.2l1.3 1.5h6a1 1 0 0 1 1 1V12a1 1 0 0 1-1 1h-10.5a1 1 0 0 1-1-1z" />
      {open && <path d="M1.5 7.5h13" />}
    </svg>;
  const FileIcon = () => <svg width="10" height="10" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.3" strokeLinejoin="round" aria-hidden="true">
      <path d="M4 1.5h5.5L13 5v9.5H4z" />
      <path d="M9.5 1.5V5H13" />
    </svg>;
  const CloudIcon = () => <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.3" strokeLinejoin="round" aria-hidden="true">
      <path d="M4.5 12.5h7a2.5 2.5 0 0 0 .4-4.97A3.5 3.5 0 0 0 5.2 6.6 3 3 0 0 0 4.5 12.5z" />
    </svg>;
  const LaptopIcon = () => <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.3" strokeLinejoin="round" aria-hidden="true">
      <rect x="2.5" y="3" width="11" height="7.5" rx="1" />
      <path d="M1 12.5h14" />
    </svg>;
  return <div ref={rootRef} className={'ssc-root not-prose' + (isFullscreen ? ' ssc-fs' : '')}>
      <style>{`
        .ssc-root {
          --ssc-bg: #FFFFFF;
          --ssc-text: #1A1918;
          --ssc-sub: #5E5D59;
          --ssc-faint: #8A8880;
          --ssc-border: rgba(0,0,0,0.12);
          --ssc-panel: #F5F4EF;
          --ssc-tile: #FAFAF8;
          --ssc-clay: #D97757;
          --ssc-clay-bg: rgba(217,119,87,0.14);
          --ssc-label: #B0562F;
          --ssc-hover: rgba(115,114,108,0.10);
          font-family: var(--font-sans, system-ui, -apple-system, sans-serif);
          background: var(--ssc-bg);
          color: var(--ssc-text);
          border: 1px solid var(--ssc-border);
          border-radius: 16px;
          padding: 20px 24px 24px;
          margin: 1.5rem 0;
          box-sizing: border-box;
        }
        .dark .ssc-root {
          --ssc-bg: #1B1A18;
          --ssc-text: #F1EFE9;
          --ssc-sub: #B8B5AD;
          --ssc-faint: #8A8880;
          --ssc-border: rgba(255,255,255,0.12);
          --ssc-panel: #24231F;
          --ssc-tile: #2A2925;
          --ssc-clay-bg: rgba(217,119,87,0.20);
          --ssc-label: #EBC9B7;
        }
        .ssc-fs { display: flex; flex-direction: column; justify-content: center; align-items: center; margin: 0; border-radius: 0; height: 100vh; }
        .ssc-fs .ssc-head { width: 100%; max-width: ${CANVAS_W}px; }
        .ssc-fs .ssc-frame { width: 100%; }
        .ssc-mono { font-family: var(--font-mono, ui-monospace, SFMono-Regular, Menlo, monospace); }
        .ssc-head { display: flex; align-items: flex-start; justify-content: space-between; gap: 12px; margin-bottom: 20px; }
        .ssc-files { display: flex; gap: 8px; flex-wrap: wrap; }
        .ssc-file {
          font-size: 12.5px; font-weight: 430; padding: 8px 13px; border-radius: 10px; cursor: pointer;
          border: 0.5px solid var(--ssc-border); background: var(--ssc-tile); color: var(--ssc-text);
          white-space: nowrap; transition: background 0.2s, border-color 0.2s;
        }
        .ssc-file:hover { filter: brightness(0.97); }
        .ssc-file[aria-pressed="true"] { font-weight: 600; border: 1.5px solid var(--ssc-clay); background: var(--ssc-clay-bg); }
        .ssc-fsbtn {
          display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; flex-shrink: 0;
          border: none; background: none; border-radius: 6px; cursor: pointer; color: var(--ssc-faint); font-size: 15px;
        }
        .ssc-fsbtn:hover { background: var(--ssc-hover); }
        .ssc-frame { width: 100%; max-width: ${CANVAS_W}px; margin: 0 auto; }
        .ssc-canvas { position: relative; width: ${CANVAS_W}px; height: ${CANVAS_H}px; transform-origin: top left; }
        .ssc-machine { position: absolute; top: 42px; height: 182px; background: var(--ssc-panel); border-radius: 16px; }
        .ssc-machine-label { position: absolute; top: 192px; display: flex; align-items: center; gap: 8px; font-size: 13.5px; font-weight: 600; }
        .ssc-tile {
          position: absolute; top: 58px; width: 126px; height: 108px; border-radius: 12px; padding: 11px 12px; box-sizing: border-box;
          background: var(--ssc-tile); border: 0.5px solid var(--ssc-border); opacity: 0.6;
          transition: background 0.25s, border-color 0.25s, opacity 0.25s;
        }
        .ssc-tile.ssc-on { background: var(--ssc-clay-bg); border: 1px solid var(--ssc-clay); opacity: 1; }
        .ssc-tile-name { display: flex; align-items: center; gap: 6px; color: var(--ssc-faint); }
        .ssc-tile.ssc-on .ssc-tile-name { color: var(--ssc-clay); }
        .ssc-tile-name span { font-size: 12px; font-weight: 430; white-space: nowrap; color: var(--ssc-text); }
        .ssc-tile.ssc-on .ssc-tile-name span { font-weight: 600; }
        .ssc-tile-caption { font-size: 10.5px; color: var(--ssc-sub); margin-top: 5px; line-height: 1.35; }
        .ssc-filemark {
          position: absolute; left: 5px; right: 5px; bottom: 8px; display: inline-flex; align-items: center; justify-content: center; gap: 2px;
          font-size: 8.5px; color: var(--ssc-label); background: var(--ssc-bg); border: 1px solid var(--ssc-clay);
          border-radius: 6px; padding: 2px 3px; white-space: nowrap; overflow: hidden;
        }
        .ssc-filemark svg { flex-shrink: 0; }
        .ssc-machine-filemark { display: inline-flex; align-items: center; gap: 4px; margin-left: 10px; font-size: 10.5px; font-weight: 500; color: var(--ssc-label); }
        .ssc-ring {
          position: absolute; border: 2px solid var(--ssc-clay); border-radius: 18px; pointer-events: none;
          transition: left 0.35s ease, top 0.35s ease, width 0.35s ease, height 0.35s ease;
        }
        .ssc-ring-label {
          position: absolute; font-size: 12px; font-weight: 600; color: var(--ssc-label); white-space: nowrap; pointer-events: none;
          transition: left 0.35s ease, top 0.35s ease;
        }
      `}</style>

      <div className="ssc-head">
        <div className="ssc-files" role="group" aria-label="Settings file">
          {FILES.map(f => <button key={f.id} type="button" className="ssc-file ssc-mono" aria-pressed={f.id === sel} onClick={() => setSel(f.id)}>{f.path}</button>)}
        </div>
        <button type="button" className="ssc-fsbtn" onClick={toggleFullscreen} aria-label={isFullscreen ? 'Exit fullscreen' : 'Enter fullscreen'} title={isFullscreen ? 'Exit fullscreen' : 'Fullscreen'}>{isFullscreen ? '⤡' : '⛶'}</button>
      </div>

      <div ref={frameRef} className="ssc-frame" style={{
    height: CANVAS_H * scale + 'px'
  }}>
        <div className="ssc-canvas" style={{
    transform: 'scale(' + scale + ')'
  }}>
          <div className="ssc-machine" style={{
    left: '10px',
    width: '430px'
  }} />
          <span className="ssc-machine-label" style={{
    left: '30px'
  }}><LaptopIcon />Your machine{fileAt.machine === 'you' && <span className="ssc-machine-filemark ssc-mono"><FileIcon />{selFile.path}</span>}</span>
          <div className="ssc-machine" style={{
    left: '460px',
    width: '200px'
  }} />
          <span className="ssc-machine-label" style={{
    left: '470px'
  }}><LaptopIcon />A teammate’s machine</span>
          <div className="ssc-machine" style={{
    left: '682px',
    width: '170px'
  }} />
          <span className="ssc-machine-label" style={{
    left: '692px'
  }}><CloudIcon />A cloud session</span>

          {TILES.map(t => {
    const on = coverage.includes(t.id);
    return <div key={t.id} className={'ssc-tile' + (on ? ' ssc-on' : '')} style={{
      left: t.left + 'px'
    }}>
                <div className="ssc-tile-name"><FolderIcon open={on} /><span className="ssc-mono">{t.name}</span></div>
                {t.caption && <div className="ssc-tile-caption">{t.caption}</div>}
                {fileAt.tiles.includes(t.id) && <span className="ssc-filemark ssc-mono" title={SHORT[sel]}><FileIcon />{TILE_MARK[sel]}</span>}
              </div>;
  })}

          <div className="ssc-ring" style={{
    left: ring.l + 'px',
    top: ring.t + 'px',
    width: ring.w + 'px',
    height: ring.h + 'px'
  }} />
          <span className="ssc-ring-label ssc-mono" style={{
    left: ring.l + 14 + 'px',
    top: ring.t - 26 + 'px'
  }}>{selFile.ring || selFile.path}</span>
        </div>
      </div>
    </div>;
};

As configurações são as chaves JSON que alteram como o Claude Code se comporta: qual modelo ele inicia, o que pode executar sem perguntar, quais arquivos não pode ler, como aparece no seu terminal e o que sua organização impõe.

<Tip>
  Para procurar uma chave específica, vá para [Todas as configurações](/docs/pt/settings-reference), que lista cada chave com o arquivo em que você a define, seu padrão e um exemplo.
</Tip>

O Claude Code lê configurações de arquivos de configurações JSON como `~/.claude/settings.json`. Ele procura por eles em alguns locais, e [o arquivo do qual ele lê uma configuração decide a quem a configuração se aplica](#settings-files-and-who-they-affect). Esta página cobre esses arquivos: em qual colocar uma configuração, como alterar uma configuração e confirmar que foi aplicada, e qual valor o Claude Code usa quando a mesma chave é definida em mais de um arquivo. [Configurar permissões](/docs/pt/permissions) cobre o que o Claude Code pode executar sem perguntar e como escrever regras `allow`, `ask` e `deny`.

<Note>
  Esta página cobre o Claude Code em execução na sua máquina: o terminal, as extensões [VS Code](/docs/pt/vs-code) e [JetBrains](/docs/pt/jetbrains), e o [aplicativo desktop](/docs/pt/desktop), que todos leem os mesmos arquivos de configurações. Uma sessão em nuvem no [Claude Code na web](/docs/pt/claude-code-on-the-web) é executada em uma máquina diferente e lê apenas alguns deles; veja [Configurações em sessões em nuvem](#settings-in-cloud-sessions).
</Note>

<span id="settings-files" />

<span id="configuration-scopes" />

<span id="available-scopes" />

<span id="when-to-use-each-scope" />

<span id="what-uses-scopes" />

<span id="subagent-configuration" />

<span id="where-settings-live" />

<h2 id="settings-files-and-who-they-affect">
  Arquivos de configurações e quem eles afetam
</h2>

O Claude Code lê configurações de quatro arquivos, e uma organização também pode entregar configurações gerenciadas do console claude.ai. Cada fonte tem um escopo: o conjunto de pessoas e projetos aos quais uma configuração salva nela se aplica, seja apenas você, todos em um projeto ou todos em sua organização.

| Escopo                | Arquivo                                                                                            | Quem afeta                                                                                                                                                                         | Use para                                                                                      |
| :-------------------- | :------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------- |
| Usuário               | `~/.claude/settings.json`                                                                          | Você, em cada projeto nesta máquina                                                                                                                                                | Preferências pessoais: tema, modo de editor, modelo padrão, suas próprias regras de permissão |
| Projeto compartilhado | `.claude/settings.json`                                                                            | Todos trabalhando na pasta que o contém. Em um repositório git, confirme-o para que os colegas de equipe o obtenham                                                                | Permissões de equipe, hooks, plugins e as variáveis de ambiente que o projeto precisa         |
| Projeto local         | `.claude/settings.local.json`                                                                      | Você, apenas neste projeto. O Claude Code o mantém fora do git quando cria o arquivo; se você o criar manualmente, adicione-o ao `.gitignore` você mesmo                           | Substituições pessoais para um projeto e testes antes de compartilhar                         |
| Gerenciado            | `managed-settings.json` e outros [mecanismos de entrega](/docs/pt/managed-settings#delivery-mechanisms) | Todos em sua organização para os quais é implantado; nada que você defina o substitui, exceto algumas [exceções sensíveis à segurança](#exceptions-to-managed-settings-precedence) | Política de segurança e requisitos de conformidade                                            |

Na coluna Arquivo, `~/.claude` é a pasta `.claude` no seu diretório home, e um `.claude` simples é a pasta `.claude` dentro do seu projeto.

<span id="where-each-file-applies" />

<span id="compare-what-each-file-reaches" />

<h3 id="compare-the-scope-of-each-settings-file">
  Compare o escopo de cada arquivo de configurações
</h3>

Suponha que você tenha três projetos em sua máquina, `website/`, `api/` e `acme-app/`, um colega de equipe tenha seu próprio clone de `acme-app/` e você inicie uma [sessão em nuvem](#settings-in-cloud-sessions) em `acme-app/`.

O gráfico abaixo mostra em quais dessas pastas uma configuração se aplica quando você inicia o Claude Code a partir delas. Clique em um arquivo de configurações para ver as pastas que ele alcança.

<SettingsScope />

* **`~/.claude/settings.json`**: cada projeto em sua máquina, e nada no de seu colega de equipe ou na sessão em nuvem
* **`acme-app/.claude/settings.json`**: seu `acme-app/`. Alcança o clone do seu colega de equipe e a sessão em nuvem apenas se você confirmar o arquivo no controle de versão; até então, é um arquivo no seu disco como qualquer outro e ninguém mais o tem
* **`acme-app/.claude/settings.local.json`**: seu `acme-app/` apenas. O Claude Code o adiciona às suas exclusões git globais na primeira vez que escreve o arquivo, para que fique fora de seus commits; se você criar o arquivo manualmente, [adicione-o ao `.gitignore` você mesmo](#keep-personal-settings-out-of-a-repository)
* **Configurações gerenciadas**, seja um arquivo `managed-settings.json`, uma política MDM ou [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) do console claude.ai: cada projeto em cada máquina para a qual sua organização as implanta, ou que você entra com sua conta organizacional. Apenas configurações gerenciadas pelo servidor alcançam a sessão em nuvem

<span id="which-files-you-have" />

<h3 id="find-or-create-your-settings-files">
  Encontre ou crie seus arquivos de configurações
</h3>

Instalar o Claude Code não cria nenhum arquivo de configurações. Se sua máquina ou projeto já tiver um, ele veio de uma dessas fontes:

* **Gerenciado**: sua organização o implanta. Você não o cria ou edita.
* **Projeto compartilhado**: um projeto que já usa Claude Code pode ter um confirmado. Se não, crie-o em `.claude/settings.json` na pasta do projeto.
* **Usuário** e **Projeto local**: crie-os você mesmo, ou deixe o Claude Code criá-los. Ele escreve `~/.claude/settings.json` na primeira vez que você altera uma opção no menu `/config` que ele armazena em configurações de usuário, como o tema, e `.claude/settings.local.json` na primeira vez que você dá uma aprovação permanente em um prompt de permissão, como "Sim, e não pergunte novamente" para um comando Bash. Algumas opções `/config`, incluindo **Mostrar dicas**, são salvas em `.claude/settings.local.json` em vez do arquivo de usuário.

<Info>
  No Windows, `~/.claude` significa `%USERPROFILE%\.claude`. Para manter os arquivos do diretório home em outro lugar, defina [`CLAUDE_CONFIG_DIR`](/docs/pt/env-vars); o Claude Code então armazena suas configurações, histórico de sessão e plugins lá em vez disso.
</Info>

O Claude Code também mantém um quinto arquivo, [`~/.claude.json`](/docs/pt/claude-directory#ce-claude-json), que ele escreve para si mesmo; você não precisa editá-lo. Ele contém sua sessão de entrada, configurações de [servidor MCP](/docs/pt/mcp), estado por projeto como decisões de confiança, e as [chaves de config globais](/docs/pt/settings-reference#global-config-settings) que `/config` escreve para você.

<h3 id="share-settings-with-your-team">
  Compartilhe configurações com sua equipe
</h3>

Confirme `.claude/settings.json` para que todos que clonem o repositório obtenham as mesmas permissões, hooks e plugins. Cada colega de equipe ainda pode substituí-lo para si mesmo em seu próprio `.claude/settings.local.json`, para que exceções pessoais não precisem de um commit. Para um arquivo de equipe completo, veja [configurações compartilhadas de uma equipe](/docs/pt/settings-example#a-teams-shared-settings).

Parte do que você confirma espera até que cada colega de equipe [confie na pasta](/docs/pt/permissions#project-allow-rules-and-workspace-trust), e algumas chaves nunca entram em vigor de um arquivo de repositório; [Solucione problemas de uma configuração que não se aplica](#common-cases) cobre ambos.

<span id="local-settings-file" />

<span id="where-claude-code-saves-the-project-local-file" />

<span id="the-project-local-file" />

<span id="keep-personal-settings-out-of-the-repository" />

<h3 id="keep-personal-settings-out-of-a-repository">
  Mantenha configurações pessoais fora de um repositório
</h3>

Para alterar uma configuração para você em um projeto sem alterá-la para seus colegas de equipe, salve-a em `.claude/settings.local.json` dentro do projeto. O Claude Code aplica esse arquivo sobre o `.claude/settings.json` confirmado, então se o arquivo da sua equipe define `"model": "claude-sonnet-5"` e você quer Opus, coloque `"model": "claude-opus-5-5"` no seu arquivo local e apenas suas sessões mudam.

O Claude Code também escreve neste arquivo, o mantém fora de seus commits e aplica suas regras de permissão sem a etapa de confiança:

* **O Claude Code também o escreve.** Quando Claude pede permissão para executar um comando Bash e você escolhe "Sim, e não pergunte novamente", o Claude Code salva essa [aprovação de permissão](/docs/pt/permissions#permission-system) aqui como uma regra `allow`.
* **Você não precisa gitignore você mesmo, a menos que o tenha criado manualmente.** Na primeira vez que o Claude Code escreve o arquivo em um repositório git que ainda não o ignora, ele adiciona `**/.claude/settings.local.json` ao seu arquivo de exclusões git globais, para que o arquivo fique fora de seus commits em cada repositório. Esse arquivo é `core.excludesFile` quando sua config git global o define para um caminho absoluto ou com prefixo `~`; caso contrário é `$XDG_CONFIG_HOME/git/ignore`, ou `~/.config/git/ignore` quando `XDG_CONFIG_HOME` não está definido. Se você criou o arquivo manualmente e o Claude Code ainda não escreveu nele, adicione-o ao `.gitignore` você mesmo.
* **Suas regras allow não esperam por confiança enquanto o arquivo permanece não rastreado.** Como o arquivo é seu e não do repositório, o Claude Code aplica suas regras `allow` sem a etapa de [confiança do workspace](/docs/pt/permissions#project-allow-rules-and-workspace-trust) que exige para o arquivo confirmado. Se o arquivo for rastreado pelo git, a etapa de confiança também se aplica a ele; veja [Quando seu arquivo de configurações local precisa de confiança](/docs/pt/permissions#when-your-local-settings-file-needs-trust).

<span id="where-claude-code-looks-for-each-file" />

<span id="how-claude-code-keeps-the-local-file-out-of-git" />

<span id="local-allow-rules-dont-wait-for-workspace-trust" />

<h4 id="where-claude-code-keeps-the-local-file-in-a-git-repository">
  Onde o Claude Code mantém o arquivo local em um repositório git
</h4>

Quando Claude pede permissão para executar um comando Bash e você escolhe "Sim, e não pergunte novamente", o Claude Code salva essa aprovação como uma regra `allow` em `.claude/settings.local.json`. Se você iniciar o Claude Code em um subdiretório de um repositório git, ele lê e escreve esse arquivo na raiz do repositório e aplica a aprovação em todo o repositório. Em uma [worktree](/docs/pt/worktrees), ele usa o arquivo na raiz do checkout principal.

Duas regras qualificam a localização da raiz:

* **Quando o arquivo fica com `.claude/settings.json` em vez disso**: fora de um repositório git, quando a raiz do repositório é seu diretório home, no Windows ou quando a raiz do repositório ou sua entrada `.git` ou `.claude` não é de propriedade do seu usuário.
* **Caminhos no arquivo não ancoram na raiz do repositório**: uma regra de permissão que começa com `/` ou um caminho de sandbox relativo [ancora no diretório de trabalho primário da sessão](/docs/pt/permissions#read-and-edit) em vez disso.

Antes da v2.1.211, o Claude Code mantinha o arquivo no diretório inicial. Ele ainda lê um arquivo que uma versão anterior deixou lá ao lado do arquivo raiz; onde ambos definem a mesma chave, o valor da raiz se aplica, e regras de permissão de ambos os arquivos se aplicam. O helper [`resolveSettings()`](/docs/pt/agent-sdk/typescript#resolvesettings) do Agent SDK sempre lê o arquivo do diretório inicial.

O Claude Code lê o `.claude/settings.json` compartilhado do [diretório de trabalho primário](/docs/pt/permissions#working-directories) da sessão, então para usar um arquivo confirmado na raiz do repositório, inicie o Claude Code lá. Depois que você [mover a sessão com `/cd`](/docs/pt/permissions#move-the-session-to-another-directory), o Claude Code lê ambos os arquivos do projeto do novo diretório em vez disso, colocando o arquivo local pelas mesmas regras. Lê-los do diretório para o qual você se moveu requer Claude Code v2.1.246 ou posterior.

<span id="managed-settings-delivery" />

<span id="precedence-within-the-managed-tier" />

<span id="parent-settings-from-embedding-hosts" />

<span id="enforce-settings-for-an-organization" />

<span id="settings-your-organization-manages" />

<h3 id="check-what-your-organization-enforces">
  Verifique o que sua organização impõe
</h3>

Se sua organização gerencia o Claude Code, algumas configurações são decididas para você e nada que você coloque em seus próprios arquivos as altera. Para ver quais, execute `/status`: a linha `Setting sources` nomeia a fonte gerenciada que se aplica a você. As configurações gerenciadas se aplicam onde quer que o Claude Code seja executado nesta máquina; [O que um desenvolvedor pode alterar](/docs/pt/managed-settings#what-a-developer-can-change) cobre direitos de administrador local e ferramentas diferentes do Claude Code.

As configurações gerenciadas chegam até você através dos [mecanismos de entrega](/docs/pt/managed-settings#delivery-mechanisms) na página de configurações gerenciadas, mais comumente:

* [Configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings), que o Claude Code busca do console de administração claude.ai ou de um [gateway de aplicativos Claude](/docs/pt/claude-apps-gateway) auto-hospedado
* Políticas de nível MDM ou SO, e arquivos `managed-settings.json` em um diretório do sistema
* Um host de incorporação como Claude Desktop, através da opção SDK `managedSettings`; veja [Controlar política de um host de incorporação](/docs/pt/managed-settings#parent-settings-from-embedding-hosts)

Em uma sessão [Cowork](https://claude.com/docs/cowork/overview) que é executada em sua máquina no aplicativo Claude Desktop, o Claude Code não busca configurações gerenciadas pelo servidor do console de administração claude.ai, e lê a política implantada em seu dispositivo a menos que a configuração Claude Desktop da sua organização defina `requireCoworkFullVmSandbox`. [Onde e quando uma política se aplica](/docs/pt/managed-settings#where-and-when-a-policy-applies) cobre Cowork e sessões em nuvem.

Se você é o administrador, [Configure o Claude Code para sua organização](/docs/pt/admin-setup) o guia através da escolha do que impor, e [Implante configurações gerenciadas](/docs/pt/managed-settings) cobre entrega e como confirmar que uma política está em vigor.

<h2 id="change-a-setting">
  Altere uma configuração
</h2>

Você pode alterar uma configuração no menu `/config`, editando um arquivo de configurações ou para uma sessão a partir da linha de comando.

<span id="system-prompt" />

O prompt do sistema do Claude Code não é publicado. Para dar ao Claude instruções permanentes, use arquivos [`CLAUDE.md`](/docs/pt/memory) ou a flag `--append-system-prompt`.

<h3 id="use-the-/config-menu">
  Use o menu /config
</h3>

Execute `/config` dentro do Claude Code e abra a aba **Config**. Ela lista um pequeno conjunto de opções pessoais como tema, modo de editor e saída verbose, não cada chave de configurações. Selecione uma opção para alterá-la; o Claude Code a salva para você:

* **Maioria das opções**: `~/.claude/settings.json`
* **Algumas opções, como Mostrar dicas**: `.claude/settings.local.json`
* **As [opções de config globais](/docs/pt/settings-reference#global-config-settings)**: `~/.claude.json`

Para definir uma opção sem o menu, passe `key=value`, como `/config verbose=true`.

<Note>
  `/config` faz parte da interface do terminal. O [painel de chat VS Code](/docs/pt/vs-code) e o [aplicativo desktop](/docs/pt/desktop) não o abrem; altere as configurações lá editando um arquivo de configurações ou através das configurações desses aplicativos.
</Note>

<h3 id="edit-a-settings-file">
  Edite um arquivo de configurações
</h3>

Abra o arquivo de configurações para o escopo que você quer no seu editor e adicione ou altere uma chave. Os arquivos de configurações são JSON rigoroso: um comentário `//` ou uma vírgula à direita é um erro de sintaxe, e o Claude Code relata o arquivo como um [Erro de Configurações](#fix-a-broken-settings-file) na próxima inicialização. Por exemplo, para deixar o Claude Code executar seus comandos lint e test sem perguntar e impedi-lo de ler arquivos `.env`, adicione isto a `~/.claude/settings.json`:

```json ~/.claude/settings.json theme={null}
{
  "$schema": "https://json.schemastore.org/claude-code-settings.json",
  "permissions": {
    "allow": [
      "Bash(npm run lint)",
      "Bash(npm run test *)"
    ],
    "deny": [
      "Read(./.env)",
      "Read(./.env.*)"
    ]
  }
}
```

Cada entrada sob `permissions` é uma regra que nomeia uma ferramenta e o que ela pode fazer; [Configurar permissões](/docs/pt/permissions) explica a sintaxe. A linha `$schema` aponta para o [esquema JSON publicado](https://json.schemastore.org/claude-code-settings.json) para configurações do Claude Code, que oferece a você preenchimento automático e validação inline no VS Code, Cursor e qualquer outro editor que suporte esquema JSON. O esquema pode ficar atrás dos lançamentos CLI mais recentes, então um aviso de validação em uma chave documentada recentemente não significa que sua configuração é inválida.

Depois que você salvar, execute `/status` dentro do Claude Code para confirmar que o arquivo foi carregado; [Confirme o que foi carregado](#check-what-loaded) diz o que a linha `Setting sources` mostra e como um arquivo quebrado é relatado.

Para um arquivo pessoal completo, arquivo de equipe e arquivo de organização, cada um mostrado com um comentário em cada chave que define, veja os [arquivos de configurações de exemplo](/docs/pt/settings-example).

<span id="pass-settings-for-one-session" />

<h3 id="change-a-setting-for-one-session">
  Altere uma configuração para uma sessão
</h3>

Para tentar um valor sem salvá-lo, defina-o quando você inicia o Claude Code. O valor se aplica a essa sessão e seus arquivos de configurações permanecem como estavam. Você tem três maneiras de fazer isto:

* **`--settings`**: passe uma chave como JSON, inline ou como um caminho para um arquivo. O Claude Code a aplica acima de seus arquivos de usuário, projeto e local e abaixo de configurações gerenciadas. Pode definir qualquer chave que seu arquivo de configurações de usuário pode definir; não pode definir chaves `Managed` ou `Global config`.
* **Uma flag para essa chave**: algumas chaves têm sua própria flag, como `--model` para `model` e `--effort` para `effortLevel` e `modelSettings`.
* **Uma variável de ambiente**: exporte a variável emparelhada da chave antes de executar `claude`, como `ANTHROPIC_MODEL` para `model`.

Cada entrada da chave na [referência de configurações](/docs/pt/settings-reference) lista suas substituições por sessão e qual tem precedência, então verifique a entrada para a chave que você quer alterar.

Os comandos que você executa dentro de uma sessão principalmente salvam sua escolha: quando você altera uma configuração em `/config`, o Claude Code a escreve em seus arquivos de configurações, e `/model` salva o valor como seu padrão para novas sessões.

Se você pressionar `s` no seletor `/model`, o Claude Code muda o modelo sem salvá-lo como seu padrão de usuário. [Ajuste o nível de esforço](/docs/pt/model-config#adjust-effort-level) diz quais picks `/effort` o Claude Code salva como seu padrão para o modelo que você está usando e quais se aplicam apenas à sessão atual.

Por exemplo, para iniciar uma sessão em Opus sem alterar seu padrão:

```bash theme={null}
claude --settings '{"model": "claude-opus-5-5"}'
```

<h3 id="when-edits-take-effect">
  Quando as edições entram em vigor
</h3>

O Claude Code observa seus arquivos de configurações e os recarrega quando mudam, para que aplique a maioria das edições à sessão em execução sem uma reinicialização, incluindo edições em `permissions`, `hooks` e auxiliares de credenciais como `apiKeyHelper`. O Claude Code também carrega um arquivo de configurações que você cria no meio da sessão se sua pasta existia quando a sessão começou. Para a pasta `.claude/` do projeto, ele carrega o arquivo mesmo quando você cria a pasta na mesma sessão.

O recarregamento cobre configurações de usuário, projeto, local e gerenciadas, e o Claude Code executa o [hook `ConfigChange`](/docs/pt/hooks#configchange) para cada mudança de arquivo de configurações que detecta, não para configurações gerenciadas que chegam de MDM ou do console claude.ai. As configurações gerenciadas que chegam através de MDM ou do console claude.ai alcançam uma sessão em execução em um cronograma em vez de ao salvar; a [tabela de entrega](/docs/pt/managed-settings#choose-a-delivery-mechanism) fornece por fonte.

O Claude Code lê algumas chaves apenas uma vez, na inicialização da sessão, então uma edição em uma delas não alcança a sessão em execução. As chaves do lado do administrador que também esperam por uma reinicialização, como `requiredMinimumVersion`, são listadas em [onde e quando uma política se aplica](/docs/pt/managed-settings#where-and-when-a-policy-applies). As que você provavelmente editará no meio da sessão:

* [`model`](/docs/pt/settings-reference#model): use [`/model`](/docs/pt/model-config#setting-your-model) para mudar no meio da sessão. Cada modelo tem seu próprio cache de prompt, então a primeira solicitação após uma mudança relê toda a conversa sem cache; veja [Mudando modelos](/docs/pt/prompt-caching#switching-models)
* [`effortLevel`](/docs/pt/settings-reference#effortlevel) e [`modelSettings`](/docs/pt/settings-reference#modelsettings): use [`/effort`](/docs/pt/model-config#adjust-effort-level) para alterar esforço no meio da sessão

<span id="verify-active-settings" />

<span id="check-what-loaded" />

<h3 id="confirm-what-loaded">
  Confirme o que foi carregado
</h3>

Execute `/status` dentro do Claude Code para ver quais fontes de configurações estão ativas. A aba **Status** inclui uma linha `Setting sources` que lista cada arquivo de configurações que o Claude Code carregou para a sessão atual, como `User settings` ou `Project local settings`. Quando [configurações gerenciadas](/docs/pt/admin-setup#decide-how-settings-reach-devices) estão em vigor, a entrada de configurações gerenciadas mostra entre parênteses como chegaram à sua máquina.

A linha confirma quais arquivos o Claude Code leu; ela não mostra qual arquivo forneceu cada chave. Para listar entradas que o Claude Code rejeitou, execute [`claude doctor`](/docs/pt/debug-your-config); para um modelo que configurações de projeto ou gerenciadas definem, o cabeçalho de inicialização nomeia o arquivo que o definiu. `/status` e `/config` abrem o mesmo diálogo em abas diferentes, e a aba **Config** não é uma visualização do conteúdo do seu `settings.json`.

<h3 id="fix-a-broken-settings-file">
  Corrija um arquivo de configurações quebrado
</h3>

Se você digitar JSON incorretamente ou definir uma chave para um valor que o Claude Code não aceita, o Claude Code o informa no início de uma sessão interativa. O que ele mostra depende de quanto do arquivo é afetado:

* **Erro de Configurações**: um arquivo de usuário, projeto ou local tem JSON inválido ou um valor que o esquema rejeita. No início de uma sessão interativa, o Claude Code mostra um diálogo que permite você corrigir o arquivo com a ajuda do Claude, sair ou continuar sem as configurações quebradas.
* **Aviso de Configurações**: apenas entradas individuais falham, como uma regra de permissão malformada ou um nome de evento de hook desconhecido. O Claude Code pula esses valores e mantém o resto do arquivo em vigor.
* **Configurações gerenciadas**: o Claude Code continua impondo o resto do arquivo. [Entradas inválidas em configurações gerenciadas](/docs/pt/managed-settings#invalid-entries-in-managed-settings) diz o que ele descarta e quais chaves voltam para um valor mais rigoroso até você corrigi-las. Para um documento de configurações gerenciadas que não é JSON válido, veja [Documento de configurações gerenciadas não pôde ser analisado](/docs/pt/errors#managed-settings-document-could-not-be-parsed).
* **Erro de configuração**: `~/.claude.json` não pode ser analisado. O Claude Code copia o arquivo quebrado para `~/.claude/backups/.claude.json.corrupted.<timestamp>` e pergunta se você quer sair e corrigi-lo manualmente ou redefinir para a configuração padrão; uma execução `-p` imprime o erro e sai. Para recuperar seu estado anterior, copie de volta um dos cinco arquivos `.claude.json.backup.<timestamp>` mais recentes em `~/.claude/backups/`, que o Claude Code salva antes de escrever o arquivo.

Depois que você continuar, execute `/status` para ver os arquivos afetados e `claude doctor` para os detalhes de cada erro.

Uma execução `-p` não mostra diálogo. A menos que [um documento de configurações gerenciadas não possa ser analisado](/docs/pt/errors#managed-settings-document-could-not-be-parsed), o Claude Code pula o arquivo ou valores quebrados e continua com o resto, então após uma execução `-p` que ignora uma configuração, execute `claude doctor` para ver o que ele descartou.

<span id="how-scopes-interact" />

<span id="key-points-about-the-configuration-system" />

<span id="which-value-claude-code-uses" />

<span id="which-value-wins" />

<h2 id="settings-precedence">
  Precedência de configurações
</h2>

Quando a mesma chave aparece em mais de um lugar, o Claude Code usa o valor do nível mais alto que a define. A pilha abaixo mostra os níveis, mais alto no topo; uma chave em um nível mais alto substitui a mesma chave em qualquer lugar abaixo.

<SettingsPrecedence />

Em ordem, precedência mais alta primeiro:

1. **Configurações gerenciadas**: configurações que sua organização implanta, por um arquivo `managed-settings.json`, uma política MDM ou [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) do console claude.ai. Nada que você defina as substitui: uma chave que você passa com `--settings` não substitui a mesma chave gerenciada, e uma flag como `--model` escolhe apenas entre os modelos que sua organização permite. Uma `model` gerenciada define o modelo com o qual cada sessão inicia, e você ainda pode mudar com `/model`; o bloqueio é [`availableModels`](/docs/pt/settings-reference#availablemodels), que restringe `/model`, `--model` e a chave `model` em seus próprios arquivos. Quando sua organização entrega mais de uma fonte gerenciada, as regras para [precedência dentro do nível gerenciado](/docs/pt/managed-settings#precedence-within-the-managed-tier) dizem o que o Claude Code lê de cada uma.
2. **Argumentos de linha de comando**: flags que você passa quando inicia `claude` de um terminal, para uma sessão; veja [Altere uma configuração para uma sessão](#change-a-setting-for-one-session). O Claude Code mescla JSON que você passa com `--settings <file-or-json>` com seus arquivos de configurações pelas mesmas regras que os outros níveis: ele toma uma chave que você define aqui sobre a mesma chave em configurações local, projeto ou usuário, e mantém o valor de nível inferior para uma chave que você omite.
3. **Configurações locais de projeto** (`.claude/settings.local.json`): suas configurações pessoais para este projeto.
4. **Configurações compartilhadas de projeto** (`.claude/settings.json`): configurações que sua equipe verifica no controle de origem.
5. **Configurações de usuário** (`~/.claude/settings.json`): suas configurações pessoais para cada projeto.

As variáveis de ambiente não são um nível nesta pilha. Quando um comportamento tem tanto uma variável de shell quanto uma chave de configurações, qual se aplica é decidido por par, não por nível: `ANTHROPIC_MODEL` exportada em seu shell se aplica sobre a chave `model` de qualquer arquivo, enquanto `ANTHROPIC_DEFAULT_MODEL` se aplica apenas quando nenhum arquivo define `model`. A [referência de variáveis de ambiente](/docs/pt/env-vars#precedence) diz quais chaves têm um par e qual o Claude Code lê primeiro. Um bloco `env` dentro de um arquivo de configurações é uma chave ordinária e segue os níveis acima.

Para algumas chaves sensíveis à segurança, o Claude Code honra um valor mais rigoroso de um nível inferior sobre um valor gerenciado; [Exceções à precedência de configurações gerenciadas](#exceptions-to-managed-settings-precedence) as lista.

<h3 id="lists-merge-instead-of-overriding">
  As listas se mesclam em vez de substituir
</h3>

Quando você define a mesma chave de lista, como `permissions.allow`, em mais de um arquivo, o Claude Code combina as listas em vez de escolher uma, para que cada arquivo possa adicionar entradas sem remover as de outro arquivo. Quatro chaves que contêm listas de modelos ou entradas por modelo seguem suas próprias regras:

* [`fallbackModel`](/docs/pt/settings-reference#fallbackmodel) é uma cadeia ordenada onde a posição carrega significado, então o Claude Code toma o valor inteiro do arquivo de precedência mais alta que o define.
* [`modelPicker`](/docs/pt/settings-reference#modelpicker) contém uma lista ordenada de linhas mais uma flag de substituição, então o Claude Code nunca mescla linhas de duas fontes. Ele toma o valor inteiro do mais alto de configurações gerenciadas, `--settings` e configurações de usuário que o define, e ignora a chave em configurações de projeto e local. Requer Claude Code v2.1.242 ou posterior.
* [`availableModels`](/docs/pt/settings-reference#availablemodels): quando as configurações gerenciadas que o Claude Code aplica a definem, o Claude Code aplica essa lista como está e ignora entradas que você adiciona em configurações de usuário, projeto ou local, a menos que um aplicativo que incorpora o Claude Code forneça sua própria lista de modelos; veja [Exceções à precedência de configurações gerenciadas](#exceptions-to-managed-settings-precedence). Entre fontes gerenciadas a lista nunca se mescla também; [como o Claude Code combina fontes gerenciadas](/docs/pt/managed-settings#how-claude-code-combines-managed-sources) diz qual lista de fonte se aplica. Entre escopos não gerenciados o Claude Code mescla os arrays como usual.
* [`modelSettings`](/docs/pt/settings-reference#modelsettings): o Claude Code o resolve um modelo por vez, junto com [`effortLevel`](/docs/pt/settings-reference#effortlevel). A entrada `modelSettings` afirma qual arquivo de valor se aplica a um modelo.

<span id="examples" />

<h3 id="precedence-examples">
  Exemplos de precedência
</h3>

Enquanto Claude trabalha, o Claude Code mostra uma dica de uma linha sob o spinner, como "Use /config para alterar seu modo de permissão padrão (incluindo Plan Mode)". Suponha que você queira essas dicas desligadas, então você define [`spinnerTipsEnabled`](/docs/pt/settings-reference#spinnertipsenabled) como `false` em `~/.claude/settings.json`. Cada cenário abaixo é algo que pode ligá-las novamente, e o que você pode fazer sobre isso.

<h4 id="team-settings-override-personal-settings">
  Configurações de equipe substituem configurações pessoais
</h4>

O `.claude/settings.json` da sua equipe o define como `true`. O Claude Code usa o valor do projeto porque o projeto compartilhado fica acima do usuário, então você vê dicas naquele projeto e em nenhum outro lugar.

Você pode recuperar seu valor: adicione `"spinnerTipsEnabled": false` a `.claude/settings.local.json` naquele projeto. O projeto local fica acima do projeto compartilhado, então suas sessões lá param de mostrar dicas e as sessões de seus colegas de equipe não mudam.

<h4 id="organization-settings-override-everything">
  Configurações da organização substituem tudo
</h4>

As configurações gerenciadas da sua organização o definem como `true`. Nada que você coloque em configurações de usuário, projeto ou local desliga as dicas, e nem `--settings`. Gerenciado é o nível superior.

Você não pode recuperar seu valor. Execute `/status` para ver qual fonte gerenciada se aplica e pergunte ao seu administrador se a política deve mudar.

<h4 id="the-command-line-overrides-your-files-for-one-session">
  A linha de comando substitui seus arquivos para uma sessão
</h4>

Você iniciou a sessão com `claude --settings '{"spinnerTipsEnabled": true}'`. A linha de comando fica acima de cada arquivo exceto gerenciado, então essa sessão mostra dicas mesmo que seus arquivos digam `false`.

Você recupera seu valor na próxima sessão; `--settings` dura uma sessão e não escreve em nenhum arquivo.

<h4 id="a-flag-or-environment-variable-sets-the-same-thing">
  Uma flag ou variável de ambiente define a mesma coisa
</h4>

Algumas chaves têm uma flag de linha de comando ou uma variável de ambiente que substitui o valor de configurações independentemente de qual arquivo o definiu: `ANTHROPIC_MODEL` substitui a configuração [`model`](/docs/pt/settings-reference#model), e `--model` substitui ambas para uma sessão.

Se você pode recuperar seu valor depende da chave: desdefina a variável ou solte a flag, e verifique a entrada da chave na [referência de configurações](/docs/pt/settings-reference) e a linha da variável na [referência de variáveis de ambiente](/docs/pt/env-vars) para qual o Claude Code usa.

<span id="keys-ignored-in-a-repository-file" />

<span id="keys-only-you-or-your-organization-can-set" />

<span id="common-cases" />

<span id="which-value-applies-in-common-situations" />

<h3 id="troubleshoot-a-setting-that-doesn’t-apply">
  Solucione problemas de uma configuração que não se aplica
</h3>

Quando você define uma chave e o Claude Code não se comporta como se você tivesse, comece com `/status` para ver quais arquivos ele carregou, então encontre seu sintoma abaixo. [Depure sua configuração](/docs/pt/debug-your-config) cobre as verificações mais amplas, incluindo um teste de configuração limpa.

<h4 id="a-value-you-set-is-ignored">
  Um valor que você define é ignorado
</h4>

Algo mais está definindo a mesma chave, o arquivo não pode definir esse valor ou o arquivo não foi carregado:

* **Um nível mais alto o define.** Outro arquivo de configurações, uma flag `--settings` ou uma fonte gerenciada define a chave acima da sua; a [pilha](#settings-precedence) diz qual. Uma flag ou variável de ambiente também pode substituir a chave por conta própria, decidido chave por chave; a entrada da chave na [referência de configurações](/docs/pt/settings-reference) diz qual o Claude Code usa, e a [entrada `env`](/docs/pt/settings-reference#env) cobre um valor `env` gerenciado versus uma exportação de shell.
* **Uma chave de segurança mantém seu valor rigoroso.** Para algumas chaves o Claude Code honra o valor restritivo de qualquer arquivo, então um projeto `true` para [`disableClaudeAiConnectors`](/docs/pt/settings-reference#disableclaudeaiconnectors) permanece ligado; veja [Exceções à precedência de configurações gerenciadas](#exceptions-to-managed-settings-precedence).
* **O arquivo não pode definir esse valor.** Os valores [`permissions.defaultMode`](/docs/pt/settings-reference#permissions-defaultmode) `auto` e `bypassPermissions` não entram em vigor de configurações de projeto ou local; defina-os em configurações de usuário ou gerenciadas em vez disso, ou passe `--permission-mode` para uma sessão. Antes da v2.1.257, `bypassPermissions` entrava em vigor de qualquer arquivo.

  Uma variável de exportação de telemetria em um bloco [`env`](/docs/pt/settings-reference#env) também não entra em vigor de configurações de projeto ou local, exceto por alguns valores desligados. [Variáveis que o Claude Code ignora em `env`](/docs/pt/settings-reference#variables-claude-code-ignores-in-env) lista as variáveis e esses valores.
* **O arquivo está quebrado.** JSON inválido ou um valor rejeitado faz o Claude Code pular o arquivo ou a entrada; veja [Corrija um arquivo de configurações quebrado](#fix-a-broken-settings-file).

<h4 id="a-change-you-made-in-claude-code-is-lost-in-new-sessions">
  Uma mudança que você fez no Claude Code é perdida em novas sessões
</h4>

Quando você salva uma escolha para novas sessões dentro do Claude Code, como um modelo padrão com `/model`, o Claude Code a escreve em seu arquivo de configurações de usuário, `~/.claude/settings.json`. Se você não pode escrever naquele arquivo, por exemplo porque outra ferramenta o gera ou o vincula a uma cópia somente leitura, a mudança se aplica à sessão atual e é perdida na próxima. Defina a chave na ferramenta que gera o arquivo, ou substitua o arquivo por um que você possa escrever.

Se você pode escrever no arquivo e a mudança ainda não dura, verifique se a mudança foi [apenas para uma sessão](#change-a-setting-for-one-session) ou [um nível mais alto define a mesma chave](#a-value-you-set-is-ignored). Para a chave `model`, [Uma nova sessão inicia em um modelo diferente do que você escolheu](/docs/pt/model-config#a-new-session-starts-on-a-different-model-than-you-picked) lista mais causas.

<h4 id="a-managed-change-hasn’t-reached-you">
  Uma mudança gerenciada não chegou até você
</h4>

As fontes gerenciadas alcançam uma sessão em execução no cronograma na [tabela de entrega](/docs/pt/managed-settings#choose-a-delivery-mechanism), então reinicie a sessão primeiro. Se `/status` então nomeia uma fonte diferente da que seu administrador alterou, uma fonte de prioridade mais alta se aplica; [Como o Claude Code combina fontes gerenciadas](/docs/pt/managed-settings#how-claude-code-combines-managed-sources) fornece a ordem.

<h4 id="a-committed-key-doesn’t-reach-teammates">
  Uma chave confirmada não alcança colegas de equipe
</h4>

Duas coisas mantêm uma chave em `.claude/settings.json` de se aplicar para todos que a clonam:

* **O Claude Code ignora a chave em um arquivo de repositório.** Procure por `User, local, or managed`, `User or managed`, `Managed` ou `Global config` na coluna Scope do [índice de configurações](/docs/pt/settings-reference#settings-index). Essas chaves nunca se aplicam do arquivo compartilhado, exceto por algumas que um arquivo de repositório ainda pode desligar. Cada uma dessas entradas diz assim em sua linha de Scope. As chaves `Global config` se aplicam apenas de `~/.claude.json`.

  Dentro da chave `env`, as variáveis de exportação de telemetria nunca se aplicam do arquivo compartilhado também, exceto por alguns valores desligados; veja [Variáveis que o Claude Code ignora em `env`](/docs/pt/settings-reference#variables-claude-code-ignores-in-env).
* **A chave espera por confiança.** As regras `permissions.allow`, `permissions.additionalDirectories`, `extraKnownMarketplaces` e a maioria dos valores [`env`](/docs/pt/settings-reference#env) se aplicam apenas depois que cada colega de equipe [confia na pasta](/docs/pt/permissions#project-allow-rules-and-workspace-trust). Até então eles ainda veem prompts e não obtêm plugins de um marketplace que o arquivo declara. As regras `deny` e `ask` se aplicam imediatamente.

<h4 id="permission-rules-combine-differently-than-you-expected">
  As regras de permissão se combinam diferentemente do que você esperava
</h4>

* **Você escolheu "Sim, e não pergunte novamente" em um prompt de permissão mas ainda recebe prompts para a mesma ferramenta.** Essa escolha salvou uma regra `allow` em seu arquivo local, e uma regra `allow` lá não supera uma regra `ask` de um arquivo de projeto ou gerenciado; [como as regras de permissão se combinam](/docs/pt/permissions#settings-precedence) explica a ordem. Na extensão VS Code o cartão de aprovação permite você escolher o arquivo de destino, incluindo o arquivo compartilhado do projeto, que muda a regra para todos; no CLI, o Claude Code escreve apenas em seu arquivo local.
* **As regras allow da sua organização ainda se aplicam ao lado das suas.** Isso é esperado: o Claude Code mescla [`permissions.allow`](/docs/pt/settings-reference#permissions-allow) entre escopos, a menos que sua organização defina [`allowManagedPermissionRulesOnly`](/docs/pt/settings-reference#allowmanagedpermissionrulesonly).

<span id="security-keys-where-the-stricter-value-applies" />

<h3 id="exceptions-to-managed-settings-precedence">
  Exceções à precedência de configurações gerenciadas
</h3>

Para algumas chaves cujos valores restringem uma sessão, o Claude Code honra um valor restritivo de um escopo que de outra forma não poderia substituir configurações gerenciadas. Encontre a chave nesta tabela para ver qual valor ele honra e de onde.

| Chave                                                                           | Valor que o Claude Code honra                                                                                                | Notas                                                                                                                                                                          |
| :------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`disableClaudeAiConnectors`](/docs/pt/settings-reference#disableclaudeaiconnectors) | `true` de qualquer escopo                                                                                                    | Honrado mesmo quando uma fonte gerenciada define `false`                                                                                                                       |
| [`enableArtifact`](/docs/pt/settings-reference#enableartifact)                       | `false` de qualquer escopo, e `disableArtifact: true` de qualquer escopo                                                     | Honrado mesmo quando uma fonte gerenciada define `true`; nada liga a [ferramenta Artifact](/docs/pt/artifacts#disable-artifacts) de volta. Requer Claude Code v2.1.242 ou posterior |
| [`isolatePeerMachines`](/docs/pt/settings-reference#isolatepeermachines)             | `true` de qualquer escopo                                                                                                    | Honrado mesmo quando uma fonte gerenciada define `false`                                                                                                                       |
| [`remoteControlAtStartup`](/docs/pt/settings-reference#remotecontrolatstartup)       | `false` de `.claude/settings.json` ou `.claude/settings.local.json`                                                          | Honrado mesmo quando uma fonte gerenciada define `true`; um `true` de projeto ou local é ignorado                                                                              |
| [`crossSessionInbound`](/docs/pt/settings-reference#crosssessioninbound)             | Um valor mais rigoroso de `.claude/settings.json` ou `.claude/settings.local.json`, na escada `accept` \< `hold` \< `refuse` | Honrado sobre valores gerenciados, `--settings` e de usuário; um valor de projeto ou local que não é mais rigoroso é ignorado                                                  |
| [`useAutoModeDuringPlan`](/docs/pt/settings-reference#useautomodeduringplan)         | `false` de qualquer fonte gerenciada, `--settings`, `~/.claude/settings.json` ou `.claude/settings.local.json`               | Honrado mesmo quando a fonte gerenciada vencedora define `true`; um `false` em `.claude/settings.json` é ignorado                                                              |
| [`syncClaudeAiSkills`](/docs/pt/settings-reference#syncclaudeaiskills)               | `false` de qualquer fonte gerenciada, `--settings`, `~/.claude/settings.json` ou `.claude/settings.local.json`               | Honrado mesmo quando a fonte gerenciada vencedora define `true`; um `false` em `.claude/settings.json` é ignorado                                                              |
| [`syncClaudeAiPlugins`](/docs/pt/settings-reference#syncclaudeaiplugins)             | `false` de qualquer fonte gerenciada, `--settings`, `~/.claude/settings.json` ou `.claude/settings.local.json`               | Honrado mesmo quando a fonte gerenciada vencedora define `true`; um `false` em `.claude/settings.json` é ignorado                                                              |
| [`maxEffortLevel`](/docs/pt/settings-reference#maxeffortlevel)                       | Um limite inferior de qualquer escopo, incluindo `--settings`                                                                | Honrado mesmo quando as configurações gerenciadas que o Claude Code aplica definem um limite superior; o limite inferior se aplica. Requer Claude Code v2.1.267 ou posterior   |

Um aplicativo que executa o Claude Code dentro de si e define [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/pt/env-vars) também é uma exceção. O Claude Code toma a configuração de modelo daquele aplicativo sobre as chaves `model`, `fallbackModel`, `modelPicker` e `modelOverrides` de cada fonte gerenciada, e sobre as variáveis de seleção de modelo em um bloco `env` gerenciado, como `ANTHROPIC_MODEL` e a família `ANTHROPIC_DEFAULT_*_MODEL`. O Claude Code mantém uma [`availableModels`](/docs/pt/settings-reference#availablemodels) gerenciada em vigor a menos que o aplicativo forneça a sua própria.

<h2 id="settings-in-cloud-sessions">
  Configurações em sessões em nuvem
</h2>

Uma [sessão em nuvem](/docs/pt/claude-code-on-the-web) é executada em um [ambiente em nuvem](/docs/pt/cloud-environments) em um clone fresco do seu repositório, não em sua máquina. Isso muda quais configurações a alcançam:

* **Configurações compartilhadas de projeto** (`.claude/settings.json`): lidas em uma sessão com um repositório, porque o arquivo faz parte do clone e a sessão começa dentro dele. Confirme uma configuração lá para aplicá-la nessas sessões. Uma sessão com vários repositórios começa acima dos clones e lê apenas as chaves `enabledPlugins` e `extraKnownMarketplaces` do `.claude/settings.json` de cada repositório, não regras de permissão, hooks, `env` ou outras chaves. Os marketplaces e plugins que essas duas chaves declaram ainda [não carregam em uma sessão em nuvem](/docs/pt/cloud-environments#what-carries-over-from-your-setup).
* **Configurações de usuário e projeto local** (`~/.claude/settings.json` e `.claude/settings.local.json`): não lidas. Ambas permanecem em sua máquina, e o arquivo local não está no clone.
* **Configurações gerenciadas**: apenas [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) alcançam uma sessão em nuvem; um arquivo `managed-settings.json` ou perfil MDM em seu dispositivo não. Um [ambiente auto-hospedado](/docs/pt/self-hosted-environments) também lê o arquivo de configurações gerenciadas em sua imagem de runner. [Como o Claude Code combina fontes gerenciadas](/docs/pt/managed-settings#how-claude-code-combines-managed-sources) diz quando esse arquivo se aplica.
* **`/config`**: no seu navegador em claude.ai/code, abre a seção Claude Code de suas configurações claude.ai em vez de alterar um valor. Para alterar uma configuração para uma sessão em nuvem, defina uma [variável de ambiente](/docs/pt/cloud-environments#set-environment-variables) no ambiente, ou em uma sessão com um repositório, confirme a chave no `.claude/settings.json` desse repositório.

[O que é transferido de sua configuração](/docs/pt/cloud-environments#what-carries-over-from-your-setup) lista o resto: `CLAUDE.md`, skills, servidores MCP, plugins e credenciais.

<h2 id="what’s-next">
  Próximos passos
</h2>

* [Todas as configurações](/docs/pt/settings-reference): cada chave, com onde você a define e um exemplo
* [Arquivos de configurações de exemplo](/docs/pt/settings-example): um arquivo pessoal, um arquivo de equipe e um arquivo gerenciado de uma organização
* [Configurar permissões](/docs/pt/permissions): regras allow, ask e deny, e o que o Claude Code executa sem perguntar
* [Variáveis de ambiente](/docs/pt/env-vars): as variáveis que o Claude Code lê e o bloco `env`
* [Depure sua configuração](/docs/pt/debug-your-config): quando uma configuração não se aplica
* [Referência do diretório Claude](/docs/pt/claude-directory): cada arquivo que o Claude Code lê, incluindo subagents, servidores MCP, plugins e `CLAUDE.md`
