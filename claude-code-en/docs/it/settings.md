> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# File di impostazioni e precedenza

> Modifica le impostazioni di Claude Code, scegli l'ambito a cui appartiene una chiave, verifica la modifica e scopri quale valore Claude Code utilizza quando una chiave è impostata in più posizioni.

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

Le impostazioni sono le chiavi JSON che modificano il comportamento di Claude Code: quale modello avvia, cosa può eseguire senza chiedere, quali file non può leggere, come appare nel vostro terminale e cosa la vostra organizzazione applica.

<Tip>
  Per cercare una chiave specifica, andate a [Tutte le impostazioni](/docs/it/settings-reference), che elenca ogni chiave con il file in cui la impostate, il suo valore predefinito e un esempio.
</Tip>

Claude Code legge le impostazioni da file di impostazioni JSON come `~/.claude/settings.json`. Le cerca in pochi percorsi, e [il file da cui legge un'impostazione decide a chi si applica](#settings-files-and-who-they-affect). Questa pagina copre quei file: quale usare per un'impostazione, come modificare un'impostazione e confermare che è stata applicata, e quale valore Claude Code utilizza quando la stessa chiave è impostata in più di un file. [Configura i permessi](/docs/it/permissions) copre cosa Claude Code può eseguire senza chiedere e come scrivere regole `allow`, `ask` e `deny`.

<Note>
  Questa pagina copre Claude Code in esecuzione sulla vostra macchina: il terminale, le estensioni [VS Code](/docs/it/vs-code) e [JetBrains](/docs/it/jetbrains), e l'[app desktop](/docs/it/desktop), che leggono tutti gli stessi file di impostazioni. Una sessione cloud su [Claude Code sul web](/docs/it/claude-code-on-the-web) viene eseguita su una macchina diversa e legge solo alcuni di essi; vedere [Impostazioni nelle sessioni cloud](#settings-in-cloud-sessions).
</Note>

<span id="settings-files" />

<span id="configuration-scopes" />

<span id="available-scopes" />

<span id="when-to-use-each-scope" />

<span id="what-uses-scopes" />

<span id="subagent-configuration" />

<span id="where-settings-live" />

<h2 id="settings-files-and-who-they-affect">
  File di impostazioni e chi interessano
</h2>

Claude Code legge le impostazioni da quattro file, e un'organizzazione può anche fornire impostazioni gestite dalla console claude.ai. Ogni fonte ha un ambito: l'insieme di persone e progetti a cui si applica un'impostazione salvata in essa, che sia solo voi, tutti in un progetto, o tutti nella vostra organizzazione.

| Ambito             | File                                                                                        | Chi interessa                                                                                                                                                                                             | Usarlo per                                                                                                  |
| :----------------- | :------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------- |
| Utente             | `~/.claude/settings.json`                                                                   | Voi, in ogni progetto su questa macchina                                                                                                                                                                  | Preferenze personali: tema, modalità editor, modello predefinito, vostre regole di autorizzazione personali |
| Progetto condiviso | `.claude/settings.json`                                                                     | Tutti coloro che lavorano nella cartella che lo contiene. In un repository git, eseguite il commit in modo che i vostri colleghi lo ottengano                                                             | Autorizzazioni del team, hooks, plugins, e le variabili di ambiente di cui il progetto ha bisogno           |
| Progetto locale    | `.claude/settings.local.json`                                                               | Voi, in questo solo progetto. Claude Code lo mantiene fuori da git quando crea il file; se lo create manualmente, aggiungetelo a `.gitignore` voi stessi                                                  | Override personali per un progetto, e test prima di condividere                                             |
| Gestito            | `managed-settings.json` e altri [managed sources](/docs/it/managed-settings#delivery-mechanisms) | Tutti coloro a cui la vostra organizzazione lo distribuisce; nulla di quello che impostate lo sostituisce, a parte poche [eccezioni sensibili alla sicurezza](#exceptions-to-managed-settings-precedence) | Requisiti di politica di sicurezza e conformità                                                             |

Nella colonna File, `~/.claude` è la cartella `.claude` nella vostra home directory, e un `.claude` semplice è la cartella `.claude` all'interno del vostro progetto.

<span id="where-each-file-applies" />

<span id="compare-what-each-file-reaches" />

<h3 id="compare-the-scope-of-each-settings-file">
  Confrontare l'ambito di ogni file di impostazioni
</h3>

Supponiamo che abbiate tre progetti sulla vostra macchina, `website/`, `api/`, e `acme-app/`, un vostro collega ha il suo clone di `acme-app/`, e avviate una [sessione cloud](#settings-in-cloud-sessions) su `acme-app/`.

Il grafico sottostante mostra in quali di quelle cartelle si applica un'impostazione quando avviate Claude Code da esse. Fate clic su un file di impostazioni per vedere le cartelle che raggiunge.

<SettingsScope />

* **`~/.claude/settings.json`**: ogni progetto sulla vostra macchina, e nulla su quella del vostro collega o nella sessione cloud
* **`acme-app/.claude/settings.json`**: il vostro `acme-app/`. Raggiunge il clone del vostro collega e la sessione cloud solo se eseguite il commit del file nel controllo di versione; fino a quando non lo fate, è un file sul vostro disco come qualsiasi altro e nessun altro lo ha
* **`acme-app/.claude/settings.local.json`**: il vostro `acme-app/` solo. Claude Code lo aggiunge alle vostre esclusioni git globali la prima volta che scrive il file, quindi rimane fuori dai vostri commit; se create il file manualmente, [aggiungetelo a `.gitignore` voi stessi](#keep-personal-settings-out-of-a-repository)
* **Impostazioni gestite**, che sia un file `managed-settings.json`, una politica MDM, o [impostazioni gestite dal server](/docs/it/server-managed-settings) dalla console claude.ai: ogni progetto su ogni macchina a cui la vostra organizzazione lo distribuisce, o a cui accedete con il vostro account organizzativo. Solo le impostazioni gestite dal server raggiungono la sessione cloud

<span id="which-files-you-have" />

<h3 id="find-or-create-your-settings-files">
  Trovare o creare i vostri file di impostazioni
</h3>

L'installazione di Claude Code non crea alcun file di impostazioni. Se la vostra macchina o il vostro progetto ne ha già uno, è venuto da una di queste fonti:

* **Gestito**: la vostra organizzazione lo distribuisce. Non lo create o modificate.
* **Progetto condiviso**: un progetto che già utilizza Claude Code potrebbe averne uno sottoposto a commit. Se no, createlo in `.claude/settings.json` nella cartella del progetto.
* **Utente** e **Progetto locale**: createli voi stessi, o lasciate che Claude Code li crei. Scrive `~/.claude/settings.json` la prima volta che cambiate un'opzione nel menu `/config` che memorizza nelle impostazioni utente, come il tema, e `.claude/settings.local.json` la prima volta che date un'approvazione permanente su un prompt di autorizzazione, come "Sì, e non chiedere di nuovo" per un comando Bash. Poche opzioni `/config`, incluso **Show tips**, vengono salvate in `.claude/settings.local.json` invece che nel file utente.

<Info>
  Su Windows, `~/.claude` significa `%USERPROFILE%\.claude`. Per mantenere i file della home directory altrove, impostate [`CLAUDE_CONFIG_DIR`](/docs/it/env-vars); Claude Code memorizza quindi le vostre impostazioni, la cronologia delle sessioni, e i plugins lì invece.
</Info>

Claude Code mantiene anche un quinto file, [`~/.claude.json`](/docs/it/claude-directory#ce-claude-json), che scrive per se stesso; non dovete modificarlo. Contiene la vostra sessione di accesso, configurazioni [MCP server](/docs/it/mcp), stato per progetto come decisioni di fiducia, e le [chiavi di configurazione globale](/docs/it/settings-reference#global-config-settings) che `/config` scrive per voi.

<h3 id="share-settings-with-your-team">
  Condividere le impostazioni con il vostro team
</h3>

Eseguite il commit di `.claude/settings.json` in modo che tutti coloro che clonano il repository ottengano le stesse autorizzazioni, hooks, e plugins. Ogni collega può comunque sostituirlo per se stesso nel proprio `.claude/settings.local.json`, quindi le eccezioni personali non hanno bisogno di un commit. Per un file team completo, vedete [le impostazioni condivise di un team](/docs/it/settings-example#a-teams-shared-settings).

Parte di quello che eseguite il commit attende fino a quando ogni collega [non si fida della cartella](/docs/it/permissions#project-allow-rules-and-workspace-trust), e poche chiavi non hanno mai effetto da un file di repository; [Troubleshoot a setting that doesn't apply](#common-cases) copre entrambi.

<span id="local-settings-file" />

<span id="where-claude-code-saves-the-project-local-file" />

<span id="the-project-local-file" />

<span id="keep-personal-settings-out-of-the-repository" />

<h3 id="keep-personal-settings-out-of-a-repository">
  Mantenere le impostazioni personali fuori da un repository
</h3>

Per cambiare un'impostazione per voi stessi in un progetto senza cambiarla per i vostri colleghi, salvatela in `.claude/settings.local.json` all'interno del progetto. Claude Code applica quel file sopra il `.claude/settings.json` sottoposto a commit, quindi se il file del vostro team imposta `"model": "claude-sonnet-5"` e volete Opus, mettete `"model": "claude-opus-5-5"` nel vostro file locale e solo le vostre sessioni cambiano.

Claude Code scrive anche in questo file, lo mantiene fuori dai vostri commit, e applica le sue regole di autorizzazione senza il passaggio di fiducia:

* **Claude Code lo scrive anche.** Quando Claude chiede il permesso di eseguire un comando Bash e scegliete "Sì, e non chiedere di nuovo", Claude Code salva quella [approvazione di autorizzazione](/docs/it/permissions#permission-system) qui come una regola `allow`.
* **Non dovete gitignore voi stessi, a meno che non l'abbiate creato manualmente.** La prima volta che Claude Code scrive il file in un repository git che non lo ignora già, aggiunge `**/.claude/settings.local.json` al vostro file di esclusioni git globale, quindi il file rimane fuori dai vostri commit in ogni repository. Quel file è `core.excludesFile` quando la vostra configurazione git globale lo imposta su un percorso assoluto o con prefisso `~`; altrimenti è `$XDG_CONFIG_HOME/git/ignore`, o `~/.config/git/ignore` quando `XDG_CONFIG_HOME` non è impostato. Se avete creato il file manualmente e Claude Code non ha ancora scritto in esso, aggiungetelo a `.gitignore` voi stessi.
* **Le sue regole di autorizzazione non attendono la fiducia mentre il file rimane non tracciato.** Poiché il file è vostro e non del repository, Claude Code applica le sue regole `allow` senza il passaggio [workspace trust](/docs/it/permissions#project-allow-rules-and-workspace-trust) che richiede per il file sottoposto a commit. Se il file è tracciato da git, il passaggio di fiducia si applica anche ad esso; vedete [When your local settings file needs trust](/docs/it/permissions#when-your-local-settings-file-needs-trust).

<span id="where-claude-code-looks-for-each-file" />

<span id="how-claude-code-keeps-the-local-file-out-of-git" />

<span id="local-allow-rules-dont-wait-for-workspace-trust" />

<h4 id="where-claude-code-keeps-the-local-file-in-a-git-repository">
  Dove Claude Code mantiene il file locale in un repository git
</h4>

Quando Claude chiede il permesso di eseguire un comando Bash e scegliete "Sì, e non chiedere di nuovo", Claude Code salva quella approvazione come una regola `allow` in `.claude/settings.local.json`. Se avviate Claude Code in una sottodirectory di un repository git, legge e scrive quel file alla radice del repository e applica l'approvazione in tutto il repository. In un [worktree](/docs/it/worktrees), utilizza il file alla radice del checkout principale.

Due regole qualificano la posizione della radice:

* **Quando il file rimane con `.claude/settings.json` invece**: fuori da un repository git, quando la radice del repository è la vostra home directory, su Windows, o quando la radice del repository o la sua voce `.git` o `.claude` non è di proprietà del vostro utente.
* **I percorsi nel file non si ancorano alla radice del repository**: una regola di autorizzazione che inizia con `/` o un percorso sandbox relativo [si ancora alla directory di lavoro primaria della sessione](/docs/it/permissions#read-and-edit) invece.

Prima della v2.1.211, Claude Code manteneva il file nella directory di avvio. Legge ancora un file che una versione precedente ha lasciato lì accanto al file radice; dove entrambi impostano la stessa chiave, il valore della radice si applica, e le regole di autorizzazione da entrambi i file si applicano. L'helper [`resolveSettings()`](/docs/it/agent-sdk/typescript#resolvesettings) dell'Agent SDK legge sempre il file dalla directory di avvio.

Claude Code legge il `.claude/settings.json` condiviso dalla [directory di lavoro primaria](/docs/it/permissions#working-directories) della sessione, quindi per utilizzare un file sottoposto a commit alla radice del repository, avviate Claude Code lì. Dopo aver [spostato la sessione con `/cd`](/docs/it/permissions#move-the-session-to-another-directory), Claude Code legge entrambi i file del progetto dalla nuova directory invece, posizionando il file locale secondo le stesse regole. Leggerli dalla directory in cui vi siete spostati richiede Claude Code v2.1.246 o successivo.

<span id="managed-settings-delivery" />

<span id="precedence-within-the-managed-tier" />

<span id="parent-settings-from-embedding-hosts" />

<span id="enforce-settings-for-an-organization" />

<span id="settings-your-organization-manages" />

<h3 id="check-what-your-organization-enforces">
  Verificare cosa la vostra organizzazione applica
</h3>

Se la vostra organizzazione gestisce Claude Code, alcune impostazioni sono decise per voi e nulla di quello che mettete nei vostri file cambia loro. Per vedere quali, eseguite `/status`: la riga `Setting sources` nomina la fonte gestita che si applica a voi. Le impostazioni gestite si applicano ovunque Claude Code funzioni su questa macchina; [What a developer can change](/docs/it/managed-settings#what-a-developer-can-change) copre i diritti di amministratore locale e gli strumenti diversi da Claude Code.

Le impostazioni gestite vi raggiungono attraverso i [delivery mechanisms](/docs/it/managed-settings#delivery-mechanisms) sulla pagina delle impostazioni gestite, più comunemente:

* [Server-managed settings](/docs/it/server-managed-settings), che Claude Code recupera dalla console di amministrazione claude.ai o da un [Claude apps gateway](/docs/it/claude-apps-gateway) auto-ospitato
* Politiche MDM o a livello di sistema operativo, e file `managed-settings.json` in una directory di sistema
* Un host di incorporamento come Claude Desktop, attraverso l'opzione SDK `managedSettings`; vedete [Control policy from an embedding host](/docs/it/managed-settings#parent-settings-from-embedding-hosts)

In una sessione [Cowork](https://claude.com/docs/cowork/overview) che funziona sulla vostra macchina nell'app Claude Desktop, Claude Code non recupera le impostazioni gestite dal server dalla console di amministrazione claude.ai, e legge la politica distribuita al vostro dispositivo a meno che la configurazione Claude Desktop della vostra organizzazione non imposti `requireCoworkFullVmSandbox`. [Where and when a policy applies](/docs/it/managed-settings#where-and-when-a-policy-applies) copre Cowork e le sessioni cloud.

Se siete l'amministratore, [Set up Claude Code for your organization](/docs/it/admin-setup) vi guida attraverso la scelta di cosa applicare, e [Deploy managed settings](/docs/it/managed-settings) copre la distribuzione e come confermare che una politica è in vigore.

<h2 id="change-a-setting">
  Modificate un'impostazione
</h2>

Potete modificare un'impostazione dal menu `/config`, modificando un file di impostazioni, o per una sessione dalla riga di comando.

<span id="system-prompt" />

Il prompt di sistema di Claude Code non è pubblicato. Per dare a Claude istruzioni permanenti, usate i file [`CLAUDE.md`](/docs/it/memory) o il flag `--append-system-prompt`.

<h3 id="use-the-/config-menu">
  Usate il menu /config
</h3>

Eseguite `/config` dentro Claude Code e aprite la scheda **Config**. Elenca un breve insieme di opzioni personali come tema, modalità editor e output dettagliato, non ogni chiave di impostazioni. Selezionate un'opzione per modificarla; Claude Code la salva per voi:

* **La maggior parte delle opzioni**: `~/.claude/settings.json`
* **Poche opzioni, come Mostra suggerimenti**: `.claude/settings.local.json`
* **Le [opzioni di configurazione globale](/docs/it/settings-reference#global-config-settings)**: `~/.claude.json`

Per impostare un'opzione senza il menu, passate `key=value`, come `/config verbose=true`.

<Note>
  `/config` fa parte dell'interfaccia del terminale. Il [pannello chat VS Code](/docs/it/vs-code) e l'[app desktop](/docs/it/desktop) non lo aprono; modificate le impostazioni lì modificando un file di impostazioni o attraverso le impostazioni di quelle app.
</Note>

<h3 id="edit-a-settings-file">
  Modificate un file di impostazioni
</h3>

Aprite il file di impostazioni per l'ambito che volete nel vostro editor e aggiungete o modificate una chiave. I file di impostazioni sono JSON rigoroso: un commento `//` o una virgola finale è un errore di sintassi, e Claude Code segnala il file come un [Errore di impostazioni](#fix-a-broken-settings-file) al prossimo avvio. Ad esempio, per permettere a Claude Code di eseguire i vostri comandi lint e test senza chiedere e impedirgli di leggere file `.env`, aggiungete questo a `~/.claude/settings.json`:

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

Ogni voce sotto `permissions` è una regola che nomina uno strumento e cosa può fare; [Configura i permessi](/docs/it/permissions) spiega la sintassi. La riga `$schema` punta allo [schema JSON pubblicato](https://json.schemastore.org/claude-code-settings.json) per le impostazioni di Claude Code, che vi dà l'autocompletamento e la convalida inline in VS Code, Cursor e qualsiasi altro editor che supporta lo schema JSON. Lo schema può rimanere indietro rispetto ai rilasci CLI più recenti, quindi un avviso di convalida su una chiave documentata di recente non significa che la vostra configurazione non sia valida.

Dopo aver salvato, eseguite `/status` dentro Claude Code per confermare che il file è stato caricato; [Conferma cosa è stato caricato](#check-what-loaded) dice cosa mostra la riga `Setting sources` e come un file rotto è segnalato.

Per un file personale completo, file team e file organizzativo, ognuno mostrato con un commento su ogni chiave che imposta, vedere i [file di impostazioni di esempio](/docs/it/settings-example).

<span id="pass-settings-for-one-session" />

<h3 id="change-a-setting-for-one-session">
  Modificate un'impostazione per una sessione
</h3>

Per provare un valore senza salvarlo, impostatelo quando avviate Claude Code. Il valore si applica a quella sessione e i vostri file di impostazioni rimangono come erano. Avete tre modi per farlo:

* **`--settings`**: passate una chiave come JSON, inline o come percorso a un file. Claude Code la applica sopra i vostri file utente, progetto e locale e sotto le impostazioni gestite. Può impostare qualsiasi chiave che il vostro file di impostazioni utente può impostare; non può impostare chiavi `Managed` o `Global config`.
* **Un flag per quella chiave**: alcune chiavi hanno il loro flag, come `--model` per `model` e `--effort` per `effortLevel` e `modelSettings`.
* **Una variabile di ambiente**: esportate la variabile accoppiata della chiave prima di eseguire `claude`, come `ANTHROPIC_MODEL` per `model`.

Ogni voce della chiave sulla [referenza delle impostazioni](/docs/it/settings-reference) elenca i suoi override per-sessione e quale ha la precedenza, quindi controllate la voce per la chiave che volete modificare.

I comandi che eseguite dentro una sessione per lo più salvano la vostra scelta: quando modificate un'impostazione in `/config`, Claude Code la scrive nei vostri file di impostazioni, e `/model` salva il valore come vostro predefinito per le nuove sessioni.

Se premete `s` nel selettore `/model`, Claude Code cambia il modello senza salvarlo come vostro predefinito utente. [Regola il livello di sforzo](/docs/it/model-config#adjust-effort-level) dice quali `/effort` Claude Code salva come vostro predefinito per il modello che state usando e quali si applicano solo alla sessione corrente.

Ad esempio, per avviare una sessione su Opus senza modificare il vostro predefinito:

```bash theme={null}
claude --settings '{"model": "claude-opus-5-5"}'
```

<h3 id="when-edits-take-effect">
  Quando gli edits hanno effetto
</h3>

Claude Code osserva i vostri file di impostazioni e li ricarica quando cambiano, quindi applica la maggior parte degli edits alla sessione in esecuzione senza un riavvio, inclusi gli edits a `permissions`, `hooks` e helper di credenziali come `apiKeyHelper`. Claude Code anche carica un file di impostazioni che create a metà sessione se la sua cartella esisteva quando la sessione è iniziata. Per la cartella `.claude/` del progetto, carica il file anche quando create la cartella nella stessa sessione.

Il ricaricamento copre le impostazioni utente, progetto, locale e gestite, e Claude Code esegue l'[hook `ConfigChange`](/docs/it/hooks#configchange) per ogni modifica di file di impostazioni che rileva, non per le impostazioni gestite che arrivano da MDM o dalla console claude.ai. Le impostazioni gestite che arrivano attraverso MDM o dalla console claude.ai raggiungono una sessione in esecuzione secondo una pianificazione piuttosto che al salvataggio; la [tabella di consegna](/docs/it/managed-settings#choose-a-delivery-mechanism) la fornisce per fonte.

Claude Code legge alcune chiavi solo una volta, all'avvio della sessione, quindi un edit a una di esse non raggiunge la sessione in esecuzione. Le chiavi lato amministratore che attendono anche un riavvio, come `requiredMinimumVersion`, sono elencate sotto [dove e quando si applica una politica](/docs/it/managed-settings#where-and-when-a-policy-applies). Quelle che è più probabile che modifichiate a metà sessione:

* [`model`](/docs/it/settings-reference#model): usate [`/model`](/docs/it/model-config#setting-your-model) per cambiare a metà sessione. Ogni modello ha la sua cache di prompt, quindi la prima richiesta dopo un cambio rilegge l'intera conversazione senza cache; vedere [Cambio di modelli](/docs/it/prompt-caching#switching-models)
* [`effortLevel`](/docs/it/settings-reference#effortlevel) e [`modelSettings`](/docs/it/settings-reference#modelsettings): usate [`/effort`](/docs/it/model-config#adjust-effort-level) per modificare lo sforzo a metà sessione

<span id="verify-active-settings" />

<span id="check-what-loaded" />

<h3 id="confirm-what-loaded">
  Confermate cosa è stato caricato
</h3>

Eseguite `/status` dentro Claude Code per vedere quali fonti di impostazioni sono attive. La scheda **Status** include una riga `Setting sources` che elenca ogni file di impostazioni che Claude Code ha caricato per la sessione corrente, come `User settings` o `Project local settings`. Quando le [impostazioni gestite](/docs/it/admin-setup#decide-how-settings-reach-devices) sono in vigore, la voce delle impostazioni gestite mostra tra parentesi come hanno raggiunto la vostra macchina.

La riga conferma quali file Claude Code ha letto; non mostra quale file ha fornito ogni chiave. Per elencare le voci che Claude Code ha rifiutato, eseguite [`claude doctor`](/docs/it/debug-your-config); per un modello che le impostazioni di progetto o gestite impostano, l'intestazione di avvio nomina il file che lo ha impostato. `/status` e `/config` aprono lo stesso dialogo su schede diverse, e la scheda **Config** non è una visualizzazione dei contenuti del vostro `settings.json`.

<h3 id="fix-a-broken-settings-file">
  Riparate un file di impostazioni rotto
</h3>

Se digitate male JSON o impostate una chiave a un valore che Claude Code non accetta, Claude Code ve lo dice all'inizio di una sessione interattiva. Quello che mostra dipende da quanto del file è interessato:

* **Errore di impostazioni**: un file utente, progetto o locale ha JSON non valido o un valore che lo schema rifiuta. All'inizio di una sessione interattiva Claude Code mostra un dialogo che vi permette di riparare il file con l'aiuto di Claude, uscire, o continuare senza le impostazioni rotte.
* **Avviso di impostazioni**: solo le voci individuali falliscono, come una regola di permesso malformata o un nome di evento hook sconosciuto. Claude Code salta quei valori e mantiene il resto del file in vigore.
* **Impostazioni gestite**: Claude Code continua a applicare il resto del file. [Voci non valide nelle impostazioni gestite](/docs/it/managed-settings#invalid-entries-in-managed-settings) dice cosa scarta e quali chiavi ricadono a un valore più rigoroso fino a quando non le riparate. Per un documento di impostazioni gestite che non è JSON valido, vedere [Il documento delle impostazioni gestite non potrebbe essere analizzato](/docs/it/errors#managed-settings-document-could-not-be-parsed).
* **Errore di configurazione**: `~/.claude.json` non può essere analizzato. Claude Code copia il file rotto a `~/.claude/backups/.claude.json.corrupted.<timestamp>` e chiede se uscire e ripararlo manualmente o ripristinare la configurazione predefinita; un'esecuzione `-p` stampa l'errore e esce. Per recuperare il vostro stato precedente, copiate indietro uno dei cinque file `.claude.json.backup.<timestamp>` più recenti in `~/.claude/backups/`, che Claude Code salva prima di scrivere il file.

Dopo aver continuato, eseguite `/status` per vedere i file interessati e `claude doctor` per i dettagli di ogni errore.

Un'esecuzione `-p` non mostra dialogo. A meno che [un documento di impostazioni gestite non possa essere analizzato](/docs/it/errors#managed-settings-document-could-not-be-parsed), Claude Code salta il file rotto o i valori e continua con il resto, quindi dopo un'esecuzione `-p` che ignora un'impostazione, eseguite `claude doctor` per vedere cosa ha scartato.

<span id="how-scopes-interact" />

<span id="key-points-about-the-configuration-system" />

<span id="which-value-claude-code-uses" />

<span id="which-value-wins" />

<h2 id="settings-precedence">
  Precedenza delle impostazioni
</h2>

Quando la stessa chiave appare in più di un posto, Claude Code utilizza il valore dal livello più alto che la imposta. Lo stack sottostante mostra i livelli, il più alto in alto; una chiave a un livello più alto sostituisce la stessa chiave ovunque al di sotto.

<SettingsPrecedence />

In ordine, dalla precedenza più alta in primo luogo:

1. **Impostazioni gestite**: impostazioni che la tua organizzazione distribuisce, tramite un file `managed-settings.json`, una policy MDM, o [impostazioni gestite dal server](/docs/it/server-managed-settings) dalla console claude.ai. Nulla di quello che imposti le sostituisce: una chiave che passi con `--settings` non sostituisce la stessa chiave gestita, e un flag come `--model` sceglie solo dai modelli che la tua organizzazione consente. Un `model` gestito imposta il modello con cui inizia ogni sessione, e puoi comunque passare a `/model`; il blocco è [`availableModels`](/docs/it/settings-reference#availablemodels), che vincola `/model`, `--model`, e la chiave `model` nei tuoi file. Quando la tua organizzazione fornisce più di una fonte gestita, le regole per la [precedenza all'interno del livello gestito](/docs/it/managed-settings#precedence-within-the-managed-tier) dicono cosa Claude Code legge da ciascuna.
2. **Argomenti della riga di comando**: flag che passi quando avvii `claude` da un terminale, per una sessione; vedi [Cambia un'impostazione per una sessione](#change-a-setting-for-one-session). Claude Code unisce il JSON che passi con `--settings <file-or-json>` con i tuoi file di impostazioni secondo le stesse regole degli altri livelli: prende una chiave che imposti qui rispetto alla stessa chiave nelle impostazioni locali, di progetto o utente, e mantiene il valore di livello inferiore per una chiave che ometti.
3. **Impostazioni locali del progetto** (`.claude/settings.local.json`): le tue impostazioni personali per questo progetto.
4. **Impostazioni di progetto condivise** (`.claude/settings.json`): impostazioni che il tuo team inserisce nel controllo del codice sorgente.
5. **Impostazioni utente** (`~/.claude/settings.json`): le tue impostazioni personali per ogni progetto.

Le variabili di ambiente non sono un livello in questo stack. Quando un comportamento ha sia una variabile di shell che una chiave di impostazione, quale si applica è deciso per coppia, non per livello: `ANTHROPIC_MODEL` esportato nella tua shell si applica rispetto alla chiave `model` da qualsiasi file, mentre `ANTHROPIC_DEFAULT_MODEL` si applica solo quando nessun file imposta `model`. Il [riferimento delle variabili di ambiente](/docs/it/env-vars#precedence) dice quali chiavi hanno una coppia e quale Claude Code legge per primo. Un blocco `env` all'interno di un file di impostazioni è una chiave ordinaria e segue i livelli sopra.

Per alcune chiavi sensibili alla sicurezza, Claude Code onora un valore più restrittivo da un livello inferiore rispetto a un valore gestito; [Eccezioni alla precedenza delle impostazioni gestite](#exceptions-to-managed-settings-precedence) le elenca.

<h3 id="lists-merge-instead-of-overriding">
  Gli elenchi si uniscono invece di sostituirsi
</h3>

Quando imposti la stessa chiave di elenco, come `permissions.allow`, in più di un file, Claude Code combina gli elenchi invece di sceglierne uno, così ogni file può aggiungere voci senza rimuovere quelle di un altro file. Quattro chiavi che contengono elenchi di modelli o voci per modello seguono le loro proprie regole:

* [`fallbackModel`](/docs/it/settings-reference#fallbackmodel) è una catena ordinata in cui la posizione ha significato, quindi Claude Code prende l'intero valore dal file con la precedenza più alta che lo definisce.
* [`modelPicker`](/docs/it/settings-reference#modelpicker) contiene un elenco ordinato di righe più un flag di sostituzione, quindi Claude Code non unisce mai righe da due fonti. Prende l'intero valore dal più alto tra impostazioni gestite, `--settings`, e impostazioni utente che lo definisce, e ignora la chiave nelle impostazioni di progetto e locali. Richiede Claude Code v2.1.242 o successivo.
* [`availableModels`](/docs/it/settings-reference#availablemodels): quando le impostazioni gestite che Claude Code applica lo definiscono, Claude Code applica quell'elenco così com'è e ignora le voci che aggiungi nelle impostazioni utente, di progetto o locali, a meno che un'app che incorpora Claude Code non fornisca il suo elenco di modelli; vedi [Eccezioni alla precedenza delle impostazioni gestite](#exceptions-to-managed-settings-precedence). Tra le fonti gestite l'elenco non si unisce mai; [come Claude Code combina le fonti gestite](/docs/it/managed-settings#how-claude-code-combines-managed-sources) dice quale elenco della fonte si applica. Tra gli ambiti non gestiti Claude Code unisce gli array come al solito.
* [`modelSettings`](/docs/it/settings-reference#modelsettings): Claude Code lo risolve un modello alla volta, insieme a [`effortLevel`](/docs/it/settings-reference#effortlevel). La voce `modelSettings` indica quale valore del file si applica a un modello.

<span id="examples" />

<h3 id="precedence-examples">
  Esempi di precedenza
</h3>

Mentre Claude lavora, Claude Code mostra un suggerimento di una riga sotto lo spinner, come "Usa /config per cambiare la tua modalità di autorizzazione predefinita (inclusa Plan Mode)". Supponiamo che tu voglia disattivare questi suggerimenti, quindi imposti [`spinnerTipsEnabled`](/docs/it/settings-reference#spinnertipsenabled) a `false` in `~/.claude/settings.json`. Ogni scenario sottostante è qualcosa che può riattivarli, e cosa puoi fare al riguardo.

<h4 id="team-settings-override-personal-settings">
  Le impostazioni del team sostituiscono le impostazioni personali
</h4>

Il `.claude/settings.json` del tuo team lo imposta a `true`. Claude Code utilizza il valore del progetto perché il progetto condiviso si trova sopra l'utente, quindi vedi i suggerimenti in quel progetto e da nessun'altra parte.

Puoi recuperare il tuo valore: aggiungi `"spinnerTipsEnabled": false` a `.claude/settings.local.json` in quel progetto. Il progetto locale si trova sopra il progetto condiviso, quindi le tue sessioni lì smettono di mostrare suggerimenti e le sessioni dei tuoi compagni di squadra non cambiano.

<h4 id="organization-settings-override-everything">
  Le impostazioni dell'organizzazione sostituiscono tutto
</h4>

Le impostazioni gestite della tua organizzazione lo impostano a `true`. Nulla di quello che metti nelle impostazioni utente, di progetto o locali disattiva i suggerimenti, e nemmeno `--settings`. Gestito è il livello più alto.

Non puoi recuperare il tuo valore. Esegui `/status` per vedere quale fonte gestita si applica, e chiedi al tuo amministratore se la policy dovrebbe cambiare.

<h4 id="the-command-line-overrides-your-files-for-one-session">
  La riga di comando sostituisce i tuoi file per una sessione
</h4>

Hai avviato la sessione con `claude --settings '{"spinnerTipsEnabled": true}'`. La riga di comando si trova sopra ogni file tranne gestito, quindi quella sessione mostra suggerimenti anche se i tuoi file dicono `false`.

Recuperi il tuo valore nella sessione successiva; `--settings` dura una sessione e non scrive in nessun file.

<h4 id="a-flag-or-environment-variable-sets-the-same-thing">
  Un flag o una variabile di ambiente imposta la stessa cosa
</h4>

Alcune chiavi hanno un flag della riga di comando o una variabile di ambiente che sostituisce il valore delle impostazioni indipendentemente da quale file lo ha impostato: `ANTHROPIC_MODEL` sostituisce l'impostazione [`model`](/docs/it/settings-reference#model), e `--model` sostituisce entrambi per una sessione.

Se puoi recuperare il tuo valore dipende dalla chiave: annulla l'impostazione della variabile o elimina il flag, e controlla la voce della chiave nel [riferimento delle impostazioni](/docs/it/settings-reference) e la riga della variabile nel [riferimento delle variabili di ambiente](/docs/it/env-vars) per sapere quale Claude Code utilizza.

<span id="keys-ignored-in-a-repository-file" />

<span id="keys-only-you-or-your-organization-can-set" />

<span id="common-cases" />

<span id="which-value-applies-in-common-situations" />

<h3 id="troubleshoot-a-setting-that-doesn’t-apply">
  Risolvi i problemi di un'impostazione che non si applica
</h3>

Quando imposti una chiave e Claude Code non si comporta come se l'avessi fatto, inizia con `/status` per vedere quali file ha caricato, quindi trova il tuo sintomo di seguito. [Debug della tua configurazione](/docs/it/debug-your-config) copre i controlli più ampi, incluso un test di configurazione pulita.

<h4 id="a-value-you-set-is-ignored">
  Un valore che hai impostato viene ignorato
</h4>

Qualcos'altro sta impostando la stessa chiave, il file non può impostare quel valore, o il file non è stato caricato:

* **Un livello più alto lo imposta.** Un altro file di impostazioni, un flag `--settings`, o una fonte gestita imposta la chiave sopra la tua; lo [stack](#settings-precedence) dice quale. Un flag o una variabile di ambiente può anche sostituire la chiave di per sé, deciso chiave per chiave; la voce della chiave nel [riferimento delle impostazioni](/docs/it/settings-reference) dice quale Claude Code utilizza, e la [voce `env`](/docs/it/settings-reference#env) copre un valore `env` gestito rispetto a un'esportazione di shell.
* **Una chiave di sicurezza mantiene il suo valore restrittivo.** Per alcune chiavi Claude Code onora il valore restrittivo da qualsiasi file, quindi un `true` di progetto per [`disableClaudeAiConnectors`](/docs/it/settings-reference#disableclaudeaiconnectors) rimane attivo; vedi [Eccezioni alla precedenza delle impostazioni gestite](#exceptions-to-managed-settings-precedence).
* **Il file non può impostare quel valore.** I valori [`permissions.defaultMode`](/docs/it/settings-reference#permissions-defaultmode) `auto` e `bypassPermissions` non hanno effetto dalle impostazioni di progetto o locali; impostali invece nelle impostazioni utente o gestite, o passa `--permission-mode` per una sessione. Prima della v2.1.257, `bypassPermissions` aveva effetto da qualsiasi file.

  Una variabile di esportazione telemetria in un blocco [`env`](/docs/it/settings-reference#env) non ha effetto dalle impostazioni di progetto o locali nemmeno, a parte alcuni valori off. [Variabili che Claude Code ignora in `env`](/docs/it/settings-reference#variables-claude-code-ignores-in-env) elenca le variabili e quei valori.
* **Il file è rotto.** JSON non valido o un valore rifiutato fa sì che Claude Code salti il file o la voce; vedi [Ripara un file di impostazioni rotto](#fix-a-broken-settings-file).

<h4 id="a-change-you-made-in-claude-code-is-lost-in-new-sessions">
  Una modifica che hai fatto in Claude Code viene persa nelle nuove sessioni
</h4>

Quando salvi una scelta per le nuove sessioni dall'interno di Claude Code, come un modello predefinito con `/model`, Claude Code la scrive nel tuo file di impostazioni utente, `~/.claude/settings.json`. Se non puoi scrivere in quel file, ad esempio perché un altro strumento lo genera o lo collega a una copia di sola lettura, la modifica si applica alla sessione corrente e scompare nella successiva. Imposta la chiave nello strumento che genera il file, o sostituisci il file con uno in cui puoi scrivere.

Se puoi scrivere nel file e la modifica comunque non dura, controlla se la modifica era [solo per una sessione](#change-a-setting-for-one-session) o [un livello più alto imposta la stessa chiave](#a-value-you-set-is-ignored). Per la chiave `model`, [Una nuova sessione inizia su un modello diverso da quello che hai scelto](/docs/it/model-config#a-new-session-starts-on-a-different-model-than-you-picked) elenca più cause.

<h4 id="a-managed-change-hasn’t-reached-you">
  Una modifica gestita non ti ha raggiunto
</h4>

Le fonti gestite raggiungono una sessione in esecuzione secondo la pianificazione nella [tabella di consegna](/docs/it/managed-settings#choose-a-delivery-mechanism), quindi riavvia prima la sessione. Se `/status` quindi nomina una fonte diversa da quella che il tuo amministratore ha modificato, una fonte con priorità più alta si applica; [Come Claude Code combina le fonti gestite](/docs/it/managed-settings#how-claude-code-combines-managed-sources) fornisce l'ordine.

<h4 id="a-committed-key-doesn’t-reach-teammates">
  Una chiave impegnata non raggiunge i compagni di squadra
</h4>

Due cose impediscono a una chiave in `.claude/settings.json` di applicarsi per tutti coloro che la clonano:

* **Claude Code ignora la chiave in un file di repository.** Cerca `User, local, or managed`, `User or managed`, `Managed`, o `Global config` nella colonna Scope dell'[indice delle impostazioni](/docs/it/settings-reference#settings-index). Quelle chiavi non si applicano mai dal file condiviso, a parte alcuni che un file di repository può comunque disattivare. Ciascuna di quelle voci lo dice sulla sua riga Scope. Le chiavi `Global config` si applicano solo da `~/.claude.json`.

  All'interno della chiave `env`, le variabili di esportazione telemetria non si applicano mai dal file condiviso nemmeno, a parte alcuni valori off; vedi [Variabili che Claude Code ignora in `env`](/docs/it/settings-reference#variables-claude-code-ignores-in-env).
* **La chiave attende la fiducia.** Le regole `permissions.allow`, `permissions.additionalDirectories`, `extraKnownMarketplaces`, e la maggior parte dei valori [`env`](/docs/it/settings-reference#env) si applicano solo dopo che ogni compagno di squadra [affida la cartella](/docs/it/permissions#project-allow-rules-and-workspace-trust). Fino ad allora vedono ancora i prompt e non ottengono plugin da un marketplace che il file dichiara. Le regole `deny` e `ask` si applicano subito.

<h4 id="permission-rules-combine-differently-than-you-expected">
  Le regole di autorizzazione si combinano diversamente da quanto ti aspettavi
</h4>

* **Hai scelto "Sì, e non chiedere di nuovo" su un prompt di autorizzazione ma ricevi comunque un prompt per lo stesso strumento.** Quella scelta ha salvato una regola `allow` nel tuo file locale, e una regola `allow` lì non supera una regola `ask` da un file di progetto o gestito; [come si combinano le regole di autorizzazione](/docs/it/permissions#settings-precedence) spiega l'ordine. Nell'estensione VS Code la scheda di approvazione ti permette di scegliere il file di destinazione, incluso il file condiviso del progetto, che cambia la regola per tutti; nella CLI, Claude Code scrive solo nel tuo file locale.
* **Le regole di autorizzazione della tua organizzazione si applicano ancora insieme alle tue.** È previsto: Claude Code unisce [`permissions.allow`](/docs/it/settings-reference#permissions-allow) tra gli ambiti, a meno che la tua organizzazione non imposti [`allowManagedPermissionRulesOnly`](/docs/it/settings-reference#allowmanagedpermissionrulesonly).

<span id="security-keys-where-the-stricter-value-applies" />

<h3 id="exceptions-to-managed-settings-precedence">
  Eccezioni alla precedenza delle impostazioni gestite
</h3>

Per alcune chiavi i cui valori limitano una sessione, Claude Code onora un valore restrittivo da un ambito che altrimenti non potrebbe sostituire le impostazioni gestite. Trova la chiave in questa tabella per vedere quale valore onora e da dove.

| Chiave                                                                          | Valore che Claude Code onora                                                                                                     | Note                                                                                                                                                                           |
| :------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`disableClaudeAiConnectors`](/docs/it/settings-reference#disableclaudeaiconnectors) | `true` da qualsiasi ambito                                                                                                       | Onorato anche quando una fonte gestita imposta `false`                                                                                                                         |
| [`enableArtifact`](/docs/it/settings-reference#enableartifact)                       | `false` da qualsiasi ambito, e `disableArtifact: true` da qualsiasi ambito                                                       | Onorato anche quando una fonte gestita imposta `true`; nulla riattiva lo [strumento Artifact](/docs/it/artifacts#disable-artifacts). Richiede Claude Code v2.1.242 o successivo     |
| [`isolatePeerMachines`](/docs/it/settings-reference#isolatepeermachines)             | `true` da qualsiasi ambito                                                                                                       | Onorato anche quando una fonte gestita imposta `false`                                                                                                                         |
| [`remoteControlAtStartup`](/docs/it/settings-reference#remotecontrolatstartup)       | `false` da `.claude/settings.json` o `.claude/settings.local.json`                                                               | Onorato anche quando una fonte gestita imposta `true`; un `true` di progetto o locale viene ignorato                                                                           |
| [`crossSessionInbound`](/docs/it/settings-reference#crosssessioninbound)             | Un valore più restrittivo da `.claude/settings.json` o `.claude/settings.local.json`, sulla scala `accept` \< `hold` \< `refuse` | Onorato rispetto ai valori gestiti, `--settings`, e utente; un valore di progetto o locale che non è più restrittivo viene ignorato                                            |
| [`useAutoModeDuringPlan`](/docs/it/settings-reference#useautomodeduringplan)         | `false` da qualsiasi fonte gestita, `--settings`, `~/.claude/settings.json`, o `.claude/settings.local.json`                     | Onorato anche quando la fonte gestita vincente imposta `true`; un `false` in `.claude/settings.json` viene ignorato                                                            |
| [`syncClaudeAiSkills`](/docs/it/settings-reference#syncclaudeaiskills)               | `false` da qualsiasi fonte gestita, `--settings`, `~/.claude/settings.json`, o `.claude/settings.local.json`                     | Onorato anche quando la fonte gestita vincente imposta `true`; un `false` in `.claude/settings.json` viene ignorato                                                            |
| [`syncClaudeAiPlugins`](/docs/it/settings-reference#syncclaudeaiplugins)             | `false` da qualsiasi fonte gestita, `--settings`, `~/.claude/settings.json`, o `.claude/settings.local.json`                     | Onorato anche quando la fonte gestita vincente imposta `true`; un `false` in `.claude/settings.json` viene ignorato                                                            |
| [`maxEffortLevel`](/docs/it/settings-reference#maxeffortlevel)                       | Un limite inferiore da qualsiasi ambito, incluso `--settings`                                                                    | Onorato anche quando le impostazioni gestite che Claude Code applica impostano un limite superiore; si applica il limite più basso. Richiede Claude Code v2.1.267 o successivo |

Un'app che esegue Claude Code al suo interno e imposta [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/it/env-vars) è anche un'eccezione. Claude Code prende la configurazione del modello di quell'app rispetto alle chiavi `model`, `fallbackModel`, `modelPicker`, e `modelOverrides` da ogni fonte gestita, e rispetto alle variabili di selezione del modello in un blocco `env` gestito, come `ANTHROPIC_MODEL` e la famiglia `ANTHROPIC_DEFAULT_*_MODEL`. Claude Code mantiene un elenco di autorizzazione [`availableModels`](/docs/it/settings-reference#availablemodels) gestito in vigore a meno che l'app non fornisca il suo.

<h2 id="settings-in-cloud-sessions">
  Impostazioni nelle sessioni cloud
</h2>

Una [sessione cloud](/docs/it/claude-code-on-the-web) viene eseguita in un [ambiente cloud](/docs/it/cloud-environments) su un clone fresco del vostro repository, non sulla vostra macchina. Questo cambia quali impostazioni la raggiungono:

* **Impostazioni di progetto condivise** (`.claude/settings.json`): lette in una sessione con un repository, perché il file fa parte del clone e la sessione inizia al suo interno. Committate un'impostazione lì per applicarla in quelle sessioni. Una sessione con più repository inizia sopra i clone e legge solo i tasti `enabledPlugins` e `extraKnownMarketplaces` da ogni `.claude/settings.json` del repository, non le regole di permesso, gli hook, `env` o altre chiavi. I marketplace e i plugin che questi due tasti dichiarano ancora [non si caricano in una sessione cloud](/docs/it/cloud-environments#what-carries-over-from-your-setup).
* **Impostazioni utente e di progetto locale** (`~/.claude/settings.json` e `.claude/settings.local.json`): non lette. Entrambe rimangono sulla vostra macchina, e il file locale non è nel clone.
* **Impostazioni gestite**: solo le [impostazioni gestite dal server](/docs/it/server-managed-settings) raggiungono una sessione cloud; un file `managed-settings.json` o un profilo MDM sul vostro dispositivo no. Un [ambiente auto-ospitato](/docs/it/self-hosted-environments) legge anche il file di impostazioni gestite nella sua immagine di runner. [Come Claude Code combina le fonti gestite](/docs/it/managed-settings#how-claude-code-combines-managed-sources) dice quando quel file si applica.
* **`/config`**: nel vostro browser su claude.ai/code, apre la sezione Claude Code delle vostre impostazioni claude.ai invece di modificare un valore. Per modificare un'impostazione per una sessione cloud, impostate una [variabile di ambiente](/docs/it/cloud-environments#set-environment-variables) sull'ambiente, o in una sessione con un repository, committate la chiave al `.claude/settings.json` di quel repository.

[Cosa si trasporta dalla vostra configurazione](/docs/it/cloud-environments#what-carries-over-from-your-setup) elenca il resto: `CLAUDE.md`, skills, MCP server, plugin e credenziali.

<h2 id="what’s-next">
  Cosa c'è dopo
</h2>

* [Tutte le impostazioni](/docs/it/settings-reference): ogni chiave, con dove la impostate e un esempio
* [File di impostazioni di esempio](/docs/it/settings-example): un file personale, un file team e un file gestito di un'organizzazione
* [Configura i permessi](/docs/it/permissions): regole allow, ask e deny, e cosa Claude Code esegue senza chiedere
* [Variabili di ambiente](/docs/it/env-vars): le variabili che Claude Code legge e il blocco `env`
* [Debug della vostra configurazione](/docs/it/debug-your-config): quando un'impostazione non si applica
* [Referenza della directory Claude](/docs/it/claude-directory): ogni file che Claude Code legge, inclusi subagent, MCP server, plugin e `CLAUDE.md`
