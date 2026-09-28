> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Fichiers de paramètres et précédence

> Modifiez les paramètres Claude Code, choisissez la portée à laquelle appartient une clé, vérifiez la modification, et apprenez quelle valeur Claude Code utilise quand une clé est définie à plusieurs endroits.

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

Les paramètres sont les clés JSON qui modifient le comportement de Claude Code : quel modèle il démarre, ce qu'il peut exécuter sans demander, quels fichiers il ne peut pas lire, comment il s'affiche dans votre terminal, et ce que votre organisation applique.

<Tip>
  Pour rechercher une clé spécifique, allez à [Tous les paramètres](/docs/fr/settings-reference), qui énumère chaque clé avec le fichier dans lequel vous la définissez, sa valeur par défaut, et un exemple.
</Tip>

Claude Code lit les paramètres à partir de fichiers de paramètres JSON tels que `~/.claude/settings.json`. Il les cherche à quelques emplacements, et [le fichier à partir duquel il lit un paramètre décide à qui le paramètre s'applique](#settings-files-and-who-they-affect). Cette page couvre ces fichiers : dans lequel mettre un paramètre, comment modifier un paramètre et confirmer qu'il s'est appliqué, et quelle valeur Claude Code utilise quand la même clé est définie dans plus d'un fichier. [Configurer les permissions](/docs/fr/permissions) couvre ce que Claude Code peut exécuter sans demander et comment écrire les règles `allow`, `ask`, et `deny`.

<Note>
  Cette page couvre Claude Code s'exécutant sur votre machine : le terminal, les extensions [VS Code](/docs/fr/vs-code) et [JetBrains](/docs/fr/jetbrains), et l'[application de bureau](/docs/fr/desktop), qui lisent tous les mêmes fichiers de paramètres. Une session cloud sur [Claude Code sur le web](/docs/fr/claude-code-on-the-web) s'exécute sur une machine différente et ne lit que certains d'entre eux ; voir [Paramètres dans les sessions cloud](#settings-in-cloud-sessions).
</Note>

<span id="settings-files" />

<span id="configuration-scopes" />

<span id="available-scopes" />

<span id="when-to-use-each-scope" />

<span id="what-uses-scopes" />

<span id="subagent-configuration" />

<span id="where-settings-live" />

<h2 id="settings-files-and-who-they-affect">
  Fichiers de paramètres et qui ils affectent
</h2>

Claude Code lit les paramètres à partir de quatre fichiers, et une organisation peut également fournir des paramètres gérés à partir de la console claude.ai. Chaque source a une portée : l'ensemble des personnes et des projets auxquels un paramètre enregistré s'applique, que ce soit juste vous, tout le monde dans un projet, ou tout le monde dans votre organisation.

| Portée         | Fichier                                                                                      | Qui est affecté                                                                                                                                                                                                  | Utilisez-le pour                                                                                    |
| :------------- | :------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------- |
| Utilisateur    | `~/.claude/settings.json`                                                                    | Vous, dans chaque projet sur cette machine                                                                                                                                                                       | Préférences personnelles : thème, mode éditeur, modèle par défaut, vos propres règles de permission |
| Projet partagé | `.claude/settings.json`                                                                      | Tout le monde travaillant dans le dossier qui le contient. Dans un référentiel git, validez-le pour que vos coéquipiers l'obtiennent                                                                             | Permissions d'équipe, hooks, plugins, et les variables d'environnement dont le projet a besoin      |
| Projet local   | `.claude/settings.local.json`                                                                | Vous, dans ce seul projet. Claude Code le garde hors de git quand il crée le fichier ; si vous le créez à la main, ajoutez-le à `.gitignore` vous-même                                                           | Remplacements personnels pour un projet, et test avant de partager                                  |
| Géré           | `managed-settings.json` et autres [sources gérées](/docs/fr/managed-settings#delivery-mechanisms) | Tout le monde dans votre organisation auquel elle est déployée ; rien de ce que vous définissez ne la remplace, à part quelques [exceptions sensibles à la sécurité](#exceptions-to-managed-settings-precedence) | Politique de sécurité et exigences de conformité                                                    |

Dans la colonne Fichier, `~/.claude` est le dossier `.claude` dans votre répertoire personnel, et un `.claude` nu est le dossier `.claude` à l'intérieur de votre projet.

<span id="where-each-file-applies" />

<span id="compare-what-each-file-reaches" />

<h3 id="compare-the-scope-of-each-settings-file">
  Comparez la portée de chaque fichier de paramètres
</h3>

Supposons que vous ayez trois projets sur votre machine, `website/`, `api/`, et `acme-app/`, qu'un coéquipier ait son propre clone de `acme-app/`, et que vous démarriez une [session cloud](#settings-in-cloud-sessions) sur `acme-app/`.

Le graphique ci-dessous montre dans quels dossiers un paramètre s'applique quand vous démarrez Claude Code à partir d'eux. Cliquez sur un fichier de paramètres pour voir les dossiers qu'il atteint.

<SettingsScope />

* **`~/.claude/settings.json`** : chaque projet sur votre machine, et rien sur celle de votre coéquipier ou dans la session cloud
* **`acme-app/.claude/settings.json`** : votre `acme-app/`. Il atteint le clone de votre coéquipier et la session cloud uniquement si vous validez le fichier dans le contrôle de version ; jusqu'à ce que vous le fassiez, c'est un fichier sur votre disque comme n'importe quel autre et personne d'autre ne l'a
* **`acme-app/.claude/settings.local.json`** : votre `acme-app/` uniquement. Claude Code l'ajoute à vos exclusions git globales la première fois qu'il écrit le fichier, donc il reste hors de vos commits ; si vous créez le fichier à la main, [ajoutez-le à `.gitignore` vous-même](#keep-personal-settings-out-of-a-repository)
* **Paramètres gérés**, qu'il s'agisse d'un fichier `managed-settings.json`, d'une politique MDM, ou de [paramètres gérés par le serveur](/docs/fr/server-managed-settings) à partir de la console claude.ai : chaque projet sur chaque machine auquel votre organisation la déploie, ou auquel vous vous connectez avec votre compte d'organisation. Seuls les paramètres gérés par le serveur atteignent la session cloud

<span id="which-files-you-have" />

<h3 id="find-or-create-your-settings-files">
  Trouvez ou créez vos fichiers de paramètres
</h3>

L'installation de Claude Code ne crée aucun fichier de paramètres. Si votre machine ou projet en a déjà un, il provient de l'une de ces sources :

* **Géré** : votre organisation la déploie. Vous ne la créez ni ne l'éditez.
* **Projet partagé** : un projet qui utilise déjà Claude Code peut en avoir un validé. Sinon, créez-le à `.claude/settings.json` dans le dossier du projet.
* **Utilisateur** et **Projet local** : créez-les vous-même, ou laissez Claude Code les créer. Il écrit `~/.claude/settings.json` la première fois que vous modifiez une option dans le menu `/config` qu'il stocke dans les paramètres utilisateur, comme le thème, et `.claude/settings.local.json` la première fois que vous donnez une approbation permanente sur une invite de permission, comme « Oui, et ne me demande plus » pour une commande Bash. Quelques options `/config`, y compris **Afficher les conseils**, s'enregistrent dans `.claude/settings.local.json` à la place du fichier utilisateur.

<Info>
  Sur Windows, `~/.claude` signifie `%USERPROFILE%\.claude`. Pour garder les fichiers du répertoire personnel ailleurs, définissez [`CLAUDE_CONFIG_DIR`](/docs/fr/env-vars) ; Claude Code stocke alors vos paramètres, l'historique de session, et les plugins à la place.
</Info>

Claude Code conserve également un cinquième fichier, [`~/.claude.json`](/docs/fr/claude-directory#ce-claude-json), qu'il écrit pour lui-même ; vous n'avez pas besoin de l'éditer. Il contient votre session de connexion, les configurations de [serveur MCP](/docs/fr/mcp), l'état par projet comme les décisions de confiance, et les [clés de configuration globale](/docs/fr/settings-reference#global-config-settings) que `/config` écrit pour vous.

<h3 id="share-settings-with-your-team">
  Partagez les paramètres avec votre équipe
</h3>

Validez `.claude/settings.json` pour que tout le monde qui clone le référentiel obtienne les mêmes permissions, hooks, et plugins. Chaque coéquipier peut toujours le remplacer pour lui-même dans son propre `.claude/settings.local.json`, donc les exceptions personnelles n'ont pas besoin d'une validation. Pour un fichier d'équipe complet, voir [les paramètres partagés d'une équipe](/docs/fr/settings-example#a-teams-shared-settings).

Certains de ce que vous validez attendent que chaque coéquipier [fasse confiance au dossier](/docs/fr/permissions#project-allow-rules-and-workspace-trust), et quelques clés ne prennent jamais effet à partir d'un fichier de référentiel ; [Dépannez un paramètre qui ne s'applique pas](#common-cases) couvre les deux.

<span id="local-settings-file" />

<span id="where-claude-code-saves-the-project-local-file" />

<span id="the-project-local-file" />

<span id="keep-personal-settings-out-of-the-repository" />

<h3 id="keep-personal-settings-out-of-a-repository">
  Gardez les paramètres personnels hors d'un référentiel
</h3>

Pour modifier un paramètre pour vous-même dans un projet sans le modifier pour vos coéquipiers, enregistrez-le dans `.claude/settings.local.json` à l'intérieur du projet. Claude Code applique ce fichier sur le `.claude/settings.json` validé, donc si le fichier de votre équipe définit `"model": "claude-sonnet-5"` et que vous voulez Opus, mettez `"model": "claude-opus-5-5"` dans votre fichier local et seules vos sessions changent.

Claude Code écrit également dans ce fichier, le garde hors de vos commits, et applique ses règles allow sans l'étape de confiance :

* **Claude Code l'écrit aussi.** Quand Claude demande la permission d'exécuter une commande Bash et que vous choisissez « Oui, et ne me demande plus », Claude Code enregistre cette [approbation de permission](/docs/fr/permissions#permission-system) ici comme une règle `allow`.
* **Vous n'avez pas besoin de le gitignorer vous-même, sauf si vous l'avez créé à la main.** La première fois que Claude Code écrit le fichier dans un référentiel git qui ne l'ignore pas déjà, il ajoute `**/.claude/settings.local.json` à votre fichier d'exclusions git global, donc le fichier reste hors de vos commits dans chaque référentiel. Ce fichier est `core.excludesFile` quand votre configuration git globale le définit à un chemin absolu ou préfixé par `~` ; sinon c'est `$XDG_CONFIG_HOME/git/ignore`, ou `~/.config/git/ignore` quand `XDG_CONFIG_HOME` n'est pas défini. Si vous avez créé le fichier à la main et que Claude Code ne l'a pas encore écrit, ajoutez-le à `.gitignore` vous-même.
* **Ses règles allow n'attendent pas la confiance tant que le fichier reste non suivi.** Parce que le fichier est le vôtre et non celui du référentiel, Claude Code applique ses règles `allow` sans l'étape de [confiance de l'espace de travail](/docs/fr/permissions#project-allow-rules-and-workspace-trust) qu'il exige pour le fichier validé. Si le fichier est suivi par git, l'étape de confiance s'applique aussi ; voir [Quand votre fichier de paramètres locaux a besoin de confiance](/docs/fr/permissions#when-your-local-settings-file-needs-trust).

<span id="where-claude-code-looks-for-each-file" />

<span id="how-claude-code-keeps-the-local-file-out-of-git" />

<span id="local-allow-rules-dont-wait-for-workspace-trust" />

<h4 id="where-claude-code-keeps-the-local-file-in-a-git-repository">
  Où Claude Code garde le fichier local dans un référentiel git
</h4>

Quand Claude demande la permission d'exécuter une commande Bash et que vous choisissez « Oui, et ne me demande plus », Claude Code enregistre cette approbation comme une règle `allow` dans `.claude/settings.local.json`. Si vous démarrez Claude Code dans un sous-répertoire d'un référentiel git, il lit et écrit ce fichier à la racine du référentiel et applique l'approbation dans tout le référentiel. Dans un [worktree](/docs/fr/worktrees), il utilise le fichier à la racine du checkout principal.

Deux règles qualifient l'emplacement racine :

* **Quand le fichier reste avec `.claude/settings.json` à la place** : en dehors d'un référentiel git, quand la racine du référentiel est votre répertoire personnel, sur Windows, ou quand la racine du référentiel ou son entrée `.git` ou `.claude` n'est pas possédée par votre utilisateur.
* **Les chemins dans le fichier ne s'ancrent pas à la racine du référentiel** : une règle de permission qui commence par `/` ou un chemin sandbox relatif [s'ancre au répertoire de travail principal de la session](/docs/fr/permissions#read-and-edit) à la place.

Avant v2.1.211, Claude Code gardait le fichier dans le répertoire de démarrage. Il lit toujours un fichier qu'une version antérieure a laissé là à côté du fichier racine ; où les deux définissent la même clé, la valeur de la racine s'applique, et les règles de permission des deux fichiers s'appliquent. L'assistant [`resolveSettings()`](/docs/fr/agent-sdk/typescript#resolvesettings) du SDK Agent lit toujours le fichier à partir du répertoire de démarrage.

Claude Code lit le `.claude/settings.json` partagé à partir du [répertoire de travail principal](/docs/fr/permissions#working-directories) de la session, donc pour utiliser un fichier validé à la racine du référentiel, démarrez Claude Code là. Après avoir [déplacé la session avec `/cd`](/docs/fr/permissions#move-the-session-to-another-directory), Claude Code lit les deux fichiers de projet à partir du nouveau répertoire à la place, plaçant le fichier local par les mêmes règles. Les lire à partir du répertoire vers lequel vous avez déplacé nécessite Claude Code v2.1.246 ou ultérieur.

<span id="managed-settings-delivery" />

<span id="precedence-within-the-managed-tier" />

<span id="parent-settings-from-embedding-hosts" />

<span id="enforce-settings-for-an-organization" />

<span id="settings-your-organization-manages" />

<h3 id="check-what-your-organization-enforces">
  Vérifiez ce que votre organisation applique
</h3>

Si votre organisation gère Claude Code, certains paramètres sont décidés pour vous et rien de ce que vous mettez dans vos propres fichiers ne les change. Pour voir lesquels, exécutez `/status` : la ligne `Setting sources` nomme la source gérée qui s'applique à vous. Les paramètres gérés s'appliquent partout où Claude Code s'exécute sur cette machine ; [Ce qu'un développeur peut modifier](/docs/fr/managed-settings#what-a-developer-can-change) couvre les droits d'administrateur local et les outils autres que Claude Code.

Les paramètres gérés vous atteignent via les [mécanismes de livraison](/docs/fr/managed-settings#delivery-mechanisms) sur la page des paramètres gérés, le plus souvent :

* [Paramètres gérés par le serveur](/docs/fr/server-managed-settings), que Claude Code récupère à partir de la console d'administration claude.ai ou d'une [passerelle d'applications Claude](/docs/fr/claude-apps-gateway) auto-hébergée
* Politiques MDM ou au niveau du système d'exploitation, et fichiers `managed-settings.json` dans un répertoire système
* Un hôte d'intégration tel que Claude Desktop, via l'option SDK `managedSettings` ; voir [Contrôler la politique à partir d'un hôte d'intégration](/docs/fr/managed-settings#parent-settings-from-embedding-hosts)

Dans une session [Cowork](https://claude.com/docs/cowork/overview) qui s'exécute sur votre machine dans l'application Claude Desktop, Claude Code ne récupère pas les paramètres gérés par le serveur à partir de la console d'administration claude.ai, et il lit la politique déployée sur votre appareil sauf si la configuration Claude Desktop de votre organisation définit `requireCoworkFullVmSandbox`. [Où et quand une politique s'applique](/docs/fr/managed-settings#where-and-when-a-policy-applies) couvre Cowork et les sessions cloud.

Si vous êtes l'administrateur, [Configurez Claude Code pour votre organisation](/docs/fr/admin-setup) vous guide dans le choix de ce qu'il faut appliquer, et [Déployez les paramètres gérés](/docs/fr/managed-settings) couvre la livraison et comment confirmer qu'une politique est en vigueur.

<h2 id="change-a-setting">
  Modifiez un paramètre
</h2>

Vous pouvez modifier un paramètre à partir du menu `/config`, en éditant un fichier de paramètres, ou pour une session à partir de la ligne de commande.

<span id="system-prompt" />

L'invite système de Claude Code n'est pas publiée. Pour donner à Claude des instructions permanentes, utilisez les fichiers [`CLAUDE.md`](/docs/fr/memory) ou l'indicateur `--append-system-prompt`.

<h3 id="use-the-/config-menu">
  Utilisez le menu /config
</h3>

Exécutez `/config` à l'intérieur de Claude Code et ouvrez l'onglet **Config**. Il énumère un petit ensemble d'options personnelles comme le thème, le mode éditeur, et la sortie détaillée, pas chaque clé de paramètres. Sélectionnez une option pour la modifier ; Claude Code l'enregistre pour vous :

* **La plupart des options** : `~/.claude/settings.json`
* **Quelques options, comme Afficher les conseils** : `.claude/settings.local.json`
* **Les [options de configuration globale](/docs/fr/settings-reference#global-config-settings)** : `~/.claude.json`

Pour définir une option sans le menu, passez `key=value`, comme `/config verbose=true`.

<Note>
  `/config` fait partie de l'interface du terminal. Le [panneau de chat VS Code](/docs/fr/vs-code) et l'[application de bureau](/docs/fr/desktop) ne l'ouvrent pas ; modifiez les paramètres là en éditant un fichier de paramètres ou via les paramètres propres de ces applications.
</Note>

<h3 id="edit-a-settings-file">
  Éditez un fichier de paramètres
</h3>

Ouvrez le fichier de paramètres pour la portée que vous voulez dans votre éditeur et ajoutez ou modifiez une clé. Les fichiers de paramètres sont du JSON strict : un commentaire `//` ou une virgule finale est une erreur de syntaxe, et Claude Code signale le fichier comme une [Erreur de paramètres](#fix-a-broken-settings-file) au prochain démarrage. Par exemple, pour laisser Claude Code exécuter vos commandes lint et test sans demander et l'empêcher de lire les fichiers `.env`, ajoutez ceci à `~/.claude/settings.json` :

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

Chaque entrée sous `permissions` est une règle qui nomme un outil et ce qu'il peut faire ; [Configurer les permissions](/docs/fr/permissions) explique la syntaxe. La ligne `$schema` pointe vers le [schéma JSON publié](https://json.schemastore.org/claude-code-settings.json) pour les paramètres Claude Code, qui vous donne l'autocomplétion et la validation en ligne dans VS Code, Cursor, et tout autre éditeur qui supporte le schéma JSON. Le schéma peut être en retard par rapport aux versions CLI les plus récentes, donc un avertissement de validation sur une clé récemment documentée ne signifie pas que votre configuration est invalide.

Après avoir enregistré, exécutez `/status` à l'intérieur de Claude Code pour confirmer que le fichier a été chargé ; [Confirmez ce qui a été chargé](#check-what-loaded) dit ce que la ligne `Setting sources` affiche et comment un fichier cassé est signalé.

Pour un fichier personnel complet, un fichier d'équipe, et un fichier d'organisation, chacun affiché avec un commentaire sur chaque clé qu'il définit, voir les [fichiers de paramètres d'exemple](/docs/fr/settings-example).

<span id="pass-settings-for-one-session" />

<h3 id="change-a-setting-for-one-session">
  Modifiez un paramètre pour une session
</h3>

Pour essayer une valeur sans l'enregistrer, définissez-la quand vous démarrez Claude Code. La valeur s'applique à cette session et vos fichiers de paramètres restent comme ils étaient. Vous avez trois façons de le faire :

* **`--settings`** : passez une clé en JSON, en ligne ou comme chemin vers un fichier. Claude Code l'applique au-dessus de vos fichiers utilisateur, projet et locaux et en dessous des paramètres gérés. Il peut définir n'importe quelle clé que votre fichier de paramètres utilisateur peut définir ; il ne peut pas définir les clés `Managed` ou `Global config`.
* **Un indicateur pour cette clé** : certaines clés ont leur propre indicateur, comme `--model` pour `model` et `--effort` pour `effortLevel` et `modelSettings`.
* **Une variable d'environnement** : exportez la variable appairée de la clé avant d'exécuter `claude`, comme `ANTHROPIC_MODEL` pour `model`.

L'entrée de chaque clé sur la [référence des paramètres](/docs/fr/settings-reference) énumère ses remplacements par session et lequel a la priorité, donc vérifiez l'entrée pour la clé que vous voulez modifier.

Les commandes que vous exécutez à l'intérieur d'une session enregistrent généralement votre choix : quand vous modifiez un paramètre dans `/config`, Claude Code l'écrit dans vos fichiers de paramètres, et `/model` enregistre la valeur comme votre défaut pour les nouvelles sessions.

Si vous appuyez sur `s` dans le sélecteur `/model`, Claude Code bascule le modèle sans l'enregistrer comme votre défaut utilisateur. [Ajustez le niveau d'effort](/docs/fr/model-config#adjust-effort-level) indique quels choix `/effort` Claude Code enregistre comme votre défaut pour le modèle que vous utilisez et lesquels s'appliquent à la session actuelle uniquement.

Par exemple, pour démarrer une session sur Opus sans modifier votre défaut :

```bash theme={null}
claude --settings '{"model": "claude-opus-5-5"}'
```

<h3 id="when-edits-take-effect">
  Quand les modifications prennent effet
</h3>

Claude Code surveille vos fichiers de paramètres et les recharge quand ils changent, donc il applique la plupart des modifications à la session en cours sans redémarrage, y compris les modifications à `permissions`, `hooks`, et les assistants d'identifiants comme `apiKeyHelper`. Claude Code charge aussi un fichier de paramètres que vous créez en session si son répertoire existait quand la session a commencé. Pour le dossier `.claude/` du projet, il charge le fichier même quand vous créez le dossier dans la même session.

Le rechargement couvre les paramètres utilisateur, projet, locaux et gérés, et Claude Code exécute le [hook `ConfigChange`](/docs/fr/hooks#configchange) pour chaque changement de fichier de paramètres qu'il détecte, pas pour les paramètres gérés qui arrivent de MDM ou de la console claude.ai. Les paramètres gérés qui arrivent via MDM ou de la console claude.ai atteignent une session en cours selon un calendrier plutôt que sur enregistrement ; le [tableau de livraison](/docs/fr/managed-settings#choose-a-delivery-mechanism) le donne par source.

Claude Code lit certaines clés une seule fois, au démarrage de la session, donc une modification à l'une d'elles n'atteint pas la session en cours. Les clés côté administrateur qui attendent aussi un redémarrage, comme `requiredMinimumVersion`, sont énumérées sous [où et quand une politique s'applique](/docs/fr/managed-settings#where-and-when-a-policy-applies). Celles que vous êtes le plus susceptible de modifier en session :

* [`model`](/docs/fr/settings-reference#model) : utilisez [`/model`](/docs/fr/model-config#setting-your-model) pour basculer en session. Chaque modèle a son propre cache d'invite, donc la première demande après un basculement relit la conversation entière sans cache ; voir [Basculer les modèles](/docs/fr/prompt-caching#switching-models)
* [`effortLevel`](/docs/fr/settings-reference#effortlevel) et [`modelSettings`](/docs/fr/settings-reference#modelsettings) : utilisez [`/effort`](/docs/fr/model-config#adjust-effort-level) pour modifier l'effort en session

<span id="verify-active-settings" />

<span id="check-what-loaded" />

<h3 id="confirm-what-loaded">
  Confirmez ce qui a été chargé
</h3>

Exécutez `/status` à l'intérieur de Claude Code pour voir quelles sources de paramètres sont actives. L'onglet **Status** inclut une ligne `Setting sources` qui énumère chaque fichier de paramètres que Claude Code a chargé pour la session actuelle, comme `User settings` ou `Project local settings`. Quand les [paramètres gérés](/docs/fr/admin-setup#decide-how-settings-reach-devices) sont en vigueur, l'entrée des paramètres gérés affiche entre parenthèses comment ils ont atteint votre machine.

La ligne confirme quels fichiers Claude Code a lus ; elle n'affiche pas quel fichier a fourni chaque clé. Pour énumérer les entrées que Claude Code a rejetées, exécutez [`claude doctor`](/docs/fr/debug-your-config) ; pour un modèle que les paramètres de projet ou gérés définissent, l'en-tête de démarrage nomme le fichier qui l'a défini. `/status` et `/config` ouvrent le même dialogue sur des onglets différents, et l'onglet **Config** n'est pas une vue de vos contenus `settings.json`.

<h3 id="fix-a-broken-settings-file">
  Réparez un fichier de paramètres cassé
</h3>

Si vous faites une erreur de frappe JSON ou définissez une clé à une valeur que Claude Code n'accepte pas, Claude Code vous le dit au début d'une session interactive. Ce qu'il affiche dépend de la quantité du fichier affectée :

* **Erreur de paramètres** : un fichier utilisateur, projet ou local a du JSON invalide ou une valeur que le schéma rejette. Au début d'une session interactive, Claude Code affiche un dialogue qui vous permet de corriger le fichier avec l'aide de Claude, de quitter, ou de continuer sans les paramètres cassés.
* **Avertissement de paramètres** : seules les entrées individuelles échouent, comme une règle de permission mal formée ou un nom d'événement hook inconnu. Claude Code ignore ces valeurs et garde le reste du fichier en vigueur.
* **Paramètres gérés** : Claude Code continue d'appliquer le reste du fichier. [Les entrées invalides dans les paramètres gérés](/docs/fr/managed-settings#invalid-entries-in-managed-settings) dit ce qu'il supprime et quelles clés reviennent à une valeur plus stricte jusqu'à ce que vous les corrigiez. Pour un document de paramètres gérés qui n'est pas du JSON valide, voir [Le document de paramètres gérés n'a pas pu être analysé](/docs/fr/errors#managed-settings-document-could-not-be-parsed).
* **Erreur de configuration** : `~/.claude.json` ne peut pas être analysé. Claude Code copie le fichier cassé à `~/.claude/backups/.claude.json.corrupted.<timestamp>` et demande si vous voulez quitter et le corriger à la main ou réinitialiser à la configuration par défaut ; une exécution `-p` imprime l'erreur et quitte. Pour récupérer votre état précédent, copiez l'un des cinq fichiers `.claude.json.backup.<timestamp>` les plus récents dans `~/.claude/backups/`, que Claude Code enregistre avant d'écrire le fichier.

Après avoir continué, exécutez `/status` pour voir les fichiers affectés et `claude doctor` pour les détails de chaque erreur.

Une exécution `-p` n'affiche aucun dialogue. Sauf si [un document de paramètres gérés ne peut pas être analysé](/docs/fr/errors#managed-settings-document-could-not-be-parsed), Claude Code ignore le fichier cassé ou les valeurs et continue avec le reste, donc après une exécution `-p` qui ignore un paramètre, exécutez `claude doctor` pour voir ce qu'il a supprimé.

<span id="how-scopes-interact" />

<span id="key-points-about-the-configuration-system" />

<span id="which-value-claude-code-uses" />

<span id="which-value-wins" />

<h2 id="settings-precedence">
  Précédence des paramètres
</h2>

Quand la même clé apparaît à plus d'un endroit, Claude Code utilise la valeur du niveau le plus élevé qui la définit. La pile ci-dessous montre les niveaux, le plus élevé en haut ; une clé à un niveau plus élevé remplace la même clé n'importe où en dessous.

<SettingsPrecedence />

En ordre, priorité la plus élevée en premier :

1. **Paramètres gérés** : paramètres que votre organisation déploie, par un fichier `managed-settings.json`, une politique MDM, ou [paramètres gérés par le serveur](/docs/fr/server-managed-settings) à partir de la console claude.ai. Rien de ce que vous définissez ne les remplace : une clé que vous passez avec `--settings` ne remplace pas la même clé gérée, et un indicateur comme `--model` choisit uniquement parmi les modèles que votre organisation autorise. Un `model` géré définit le modèle avec lequel chaque session démarre, et vous pouvez toujours basculer avec `/model` ; le verrou est [`availableModels`](/docs/fr/settings-reference#availablemodels), qui contraint `/model`, `--model`, et la clé `model` dans vos propres fichiers. Quand votre organisation fournit plus d'une source gérée, les règles pour [la précédence au sein du niveau géré](/docs/fr/managed-settings#precedence-within-the-managed-tier) disent ce que Claude Code lit à partir de chacune.
2. **Arguments de ligne de commande** : indicateurs que vous passez quand vous démarrez `claude` à partir d'un terminal, pour une session ; voir [Modifiez un paramètre pour une session](#change-a-setting-for-one-session). Claude Code fusionne le JSON que vous passez avec `--settings <file-or-json>` avec vos fichiers de paramètres par les mêmes règles que les autres niveaux : il prend une clé que vous définissez ici sur la même clé dans les paramètres locaux, projet ou utilisateur, et garde la valeur de niveau inférieur pour une clé que vous omettez.
3. **Paramètres de projet local** (`.claude/settings.local.json`) : vos paramètres personnels pour ce projet.
4. **Paramètres de projet partagés** (`.claude/settings.json`) : paramètres que votre équipe valide dans le contrôle de source.
5. **Paramètres utilisateur** (`~/.claude/settings.json`) : vos paramètres personnels pour chaque projet.

Les variables d'environnement ne sont pas un niveau dans cette pile. Quand un comportement a à la fois une variable shell et une clé de paramètres, lequel s'applique est décidé par paire, pas par niveau : `ANTHROPIC_MODEL` exportée dans votre shell s'applique sur la clé `model` à partir de n'importe quel fichier, tandis que `ANTHROPIC_DEFAULT_MODEL` s'applique uniquement quand aucun fichier ne définit `model`. La [référence des variables d'environnement](/docs/fr/env-vars#precedence) dit quelles clés ont une paire et lequel Claude Code lit en premier. Un bloc `env` à l'intérieur d'un fichier de paramètres est une clé ordinaire et suit les niveaux ci-dessus.

Pour quelques clés sensibles à la sécurité, Claude Code honore une valeur plus stricte d'un niveau inférieur sur une valeur gérée ; [Exceptions à la précédence des paramètres gérés](#exceptions-to-managed-settings-precedence) les énumère.

<h3 id="lists-merge-instead-of-overriding">
  Les listes fusionnent au lieu de remplacer
</h3>

Quand vous définissez la même clé de liste, comme `permissions.allow`, dans plus d'un fichier, Claude Code combine les listes au lieu de choisir une, donc chaque fichier peut ajouter des entrées sans supprimer celles d'un autre fichier. Quatre clés qui contiennent des listes de modèles ou des entrées par modèle suivent leurs propres règles :

* [`fallbackModel`](/docs/fr/settings-reference#fallbackmodel) est une chaîne ordonnée où la position porte du sens, donc Claude Code prend la valeur entière du fichier de priorité la plus élevée qui la définit.
* [`modelPicker`](/docs/fr/settings-reference#modelpicker) contient une liste ordonnée de lignes plus un indicateur de remplacement, donc Claude Code ne fusionne jamais les lignes de deux sources. Il prend la valeur entière du plus élevé des paramètres gérés, `--settings`, et des paramètres utilisateur qui la définit, et ignore la clé dans les paramètres de projet et locaux. Nécessite Claude Code v2.1.242 ou ultérieur.
* [`availableModels`](/docs/fr/settings-reference#availablemodels) : quand les paramètres gérés que Claude Code applique la définissent, Claude Code applique cette liste telle quelle et ignore les entrées que vous ajoutez dans les paramètres utilisateur, projet ou locaux, sauf si une application qui intègre Claude Code fournit sa propre liste de modèles ; voir [Exceptions à la précédence des paramètres gérés](#exceptions-to-managed-settings-precedence). Entre les sources gérées, la liste ne fusionne jamais non plus ; [comment Claude Code combine les sources gérées](/docs/fr/managed-settings#how-claude-code-combines-managed-sources) dit quelle liste de source s'applique. Entre les portées non gérées, Claude Code fusionne les tableaux comme d'habitude.
* [`modelSettings`](/docs/fr/settings-reference#modelsettings) : Claude Code la résout un modèle à la fois, avec [`effortLevel`](/docs/fr/settings-reference#effortlevel). L'entrée `modelSettings` indique quel fichier s'applique à un modèle.

<span id="examples" />

<h3 id="precedence-examples">
  Exemples de précédence
</h3>

Pendant que Claude travaille, Claude Code affiche un conseil d'une ligne sous le spinner, comme « Utilisez /config pour modifier votre mode de permission par défaut (y compris Plan Mode) ». Supposons que vous vouliez ces conseils désactivés, donc vous définissez [`spinnerTipsEnabled`](/docs/fr/settings-reference#spinnertipsenabled) à `false` dans `~/.claude/settings.json`. Chaque scénario ci-dessous est quelque chose qui peut les réactiver, et ce que vous pouvez faire à ce sujet.

<h4 id="team-settings-override-personal-settings">
  Les paramètres d'équipe remplacent les paramètres personnels
</h4>

Le `.claude/settings.json` de votre équipe le définit à `true`. Claude Code utilise la valeur du projet parce que le projet partagé se situe au-dessus de l'utilisateur, donc vous voyez des conseils dans ce projet et nulle part ailleurs.

Vous pouvez récupérer votre valeur : ajoutez `"spinnerTipsEnabled": false` à `.claude/settings.local.json` dans ce projet. Le projet local se situe au-dessus du projet partagé, donc vos sessions là-bas arrêtent d'afficher des conseils et les sessions de vos coéquipiers ne changent pas.

<h4 id="organization-settings-override-everything">
  Les paramètres de l'organisation remplacent tout
</h4>

Les paramètres gérés de votre organisation le définissent à `true`. Rien de ce que vous mettez dans les paramètres utilisateur, projet ou locaux n'éteint les conseils, et non plus `--settings`. Géré est le niveau le plus élevé.

Vous ne pouvez pas récupérer votre valeur. Exécutez `/status` pour voir quelle source gérée s'applique, et demandez à votre administrateur si la politique devrait changer.

<h4 id="the-command-line-overrides-your-files-for-one-session">
  La ligne de commande remplace vos fichiers pour une session
</h4>

Vous avez démarré la session avec `claude --settings '{"spinnerTipsEnabled": true}'`. La ligne de commande se situe au-dessus de chaque fichier sauf géré, donc cette session affiche des conseils même si vos fichiers disent `false`.

Vous récupérez votre valeur à la session suivante ; `--settings` dure une session et n'écrit dans aucun fichier.

<h4 id="a-flag-or-environment-variable-sets-the-same-thing">
  Un indicateur ou une variable d'environnement définit la même chose
</h4>

Certaines clés ont un indicateur de ligne de commande ou une variable d'environnement qui remplace la valeur des paramètres indépendamment du fichier qui l'a défini : `ANTHROPIC_MODEL` remplace le paramètre [`model`](/docs/fr/settings-reference#model), et `--model` remplace les deux pour une session.

Que vous puissiez récupérer votre valeur dépend de la clé : supprimez la variable ou retirez l'indicateur, et vérifiez l'entrée de la clé sur la [référence des paramètres](/docs/fr/settings-reference) et la ligne de la variable sur la [référence des variables d'environnement](/docs/fr/env-vars) pour lequel Claude Code utilise.

<span id="keys-ignored-in-a-repository-file" />

<span id="keys-only-you-or-your-organization-can-set" />

<span id="common-cases" />

<span id="which-value-applies-in-common-situations" />

<h3 id="troubleshoot-a-setting-that-doesn’t-apply">
  Dépannez un paramètre qui ne s'applique pas
</h3>

Quand vous définissez une clé et que Claude Code ne se comporte pas comme si vous l'aviez, commencez par `/status` pour voir quels fichiers il a chargés, puis trouvez votre symptôme ci-dessous. [Déboguer votre configuration](/docs/fr/debug-your-config) couvre les vérifications plus larges, y compris un test de configuration propre.

<h4 id="a-value-you-set-is-ignored">
  Une valeur que vous avez définie est ignorée
</h4>

Quelque chose d'autre définit la même clé, le fichier ne peut pas définir cette valeur, ou le fichier n'a pas été chargé :

* **Un niveau plus élevé la définit.** Un autre fichier de paramètres, un indicateur `--settings`, ou une source gérée définit la clé au-dessus de la vôtre ; la [pile](#settings-precedence) dit lequel. Un indicateur ou une variable d'environnement peut aussi remplacer la clé de son propre chef, décidé clé par clé ; l'entrée de la clé sur la [référence des paramètres](/docs/fr/settings-reference) dit lequel Claude Code utilise, et l'[entrée `env`](/docs/fr/settings-reference#env) couvre une valeur `env` gérée par rapport à une exportation shell.
* **Une clé de sécurité garde sa valeur stricte.** Pour quelques clés, Claude Code honore la valeur restrictive à partir de n'importe quel fichier, donc un `true` de projet pour [`disableClaudeAiConnectors`](/docs/fr/settings-reference#disableclaudeaiconnectors) reste activé ; voir [Exceptions à la précédence des paramètres gérés](#exceptions-to-managed-settings-precedence).
* **Le fichier ne peut pas définir cette valeur.** Les valeurs [`permissions.defaultMode`](/docs/fr/settings-reference#permissions-defaultmode) `auto` et `bypassPermissions` ne prennent pas effet à partir des paramètres de projet ou locaux ; définissez-les dans les paramètres utilisateur ou gérés à la place, ou passez `--permission-mode` pour une session. Avant v2.1.257, `bypassPermissions` prenait effet à partir de n'importe quel fichier.

  Une variable d'exportation de télémétrie dans un bloc [`env`](/docs/fr/settings-reference#env) ne prend pas effet à partir des paramètres de projet ou locaux non plus, à part quelques valeurs off. [Variables que Claude Code ignore dans `env`](/docs/fr/settings-reference#variables-claude-code-ignores-in-env) énumère les variables et ces valeurs.
* **Le fichier est cassé.** Du JSON invalide ou une valeur rejetée fait que Claude Code ignore le fichier ou l'entrée ; voir [Réparez un fichier de paramètres cassé](#fix-a-broken-settings-file).

<h4 id="a-change-you-made-in-claude-code-is-lost-in-new-sessions">
  Un changement que vous avez fait dans Claude Code est perdu dans les nouvelles sessions
</h4>

Quand vous enregistrez un choix pour les nouvelles sessions à partir de Claude Code, comme un modèle par défaut avec `/model`, Claude Code l'écrit dans votre fichier de paramètres utilisateur, `~/.claude/settings.json`. Si vous ne pouvez pas écrire dans ce fichier, par exemple parce qu'un autre outil le génère ou le lie à une copie en lecture seule, le changement s'applique à la session actuelle et est parti à la session suivante. Définissez la clé dans l'outil qui génère le fichier, ou remplacez le fichier par un que vous pouvez écrire.

Si vous pouvez écrire dans le fichier et le changement ne dure toujours pas, vérifiez si le changement était [pour une session uniquement](#change-a-setting-for-one-session) ou [un niveau plus élevé définit la même clé](#a-value-you-set-is-ignored). Pour la clé `model`, [Une nouvelle session démarre sur un modèle différent de celui que vous avez choisi](/docs/fr/model-config#a-new-session-starts-on-a-different-model-than-you-picked) énumère plus de causes.

<h4 id="a-managed-change-hasn’t-reached-you">
  Un changement géré ne vous a pas atteint
</h4>

Les sources gérées atteignent une session en cours selon le calendrier du [tableau de livraison](/docs/fr/managed-settings#choose-a-delivery-mechanism), donc redémarrez d'abord la session. Si `/status` nomme alors une source différente de celle que votre administrateur a modifiée, une source de priorité plus élevée s'applique ; [Comment Claude Code combine les sources gérées](/docs/fr/managed-settings#how-claude-code-combines-managed-sources) donne l'ordre.

<h4 id="a-committed-key-doesn’t-reach-teammates">
  Une clé validée n'atteint pas les coéquipiers
</h4>

Deux choses empêchent une clé dans `.claude/settings.json` de s'appliquer pour tout le monde qui la clone :

* **Claude Code ignore la clé dans un fichier de référentiel.** Cherchez `User, local, or managed`, `User or managed`, `Managed`, ou `Global config` dans la colonne Scope de l'[index des paramètres](/docs/fr/settings-reference#settings-index). Ces clés ne s'appliquent jamais à partir du fichier partagé, à part quelques-unes qu'un fichier de référentiel peut toujours désactiver. Chacune de ces entrées le dit sur sa ligne Scope. Les clés `Global config` s'appliquent uniquement à partir de `~/.claude.json`.

  À l'intérieur de la clé `env`, les variables d'exportation de télémétrie ne s'appliquent jamais à partir du fichier partagé non plus, à part quelques valeurs off ; voir [Variables que Claude Code ignore dans `env`](/docs/fr/settings-reference#variables-claude-code-ignores-in-env).
* **La clé attend la confiance.** Les règles `permissions.allow`, `permissions.additionalDirectories`, `extraKnownMarketplaces`, et la plupart des valeurs [`env`](/docs/fr/settings-reference#env) s'appliquent uniquement après que chaque coéquipier [fasse confiance au dossier](/docs/fr/permissions#project-allow-rules-and-workspace-trust). Jusqu'à ce qu'ils le fassent, ils voient toujours des invites et n'obtiennent pas les plugins d'une marketplace que le fichier déclare. Les règles `deny` et `ask` s'appliquent immédiatement.

<h4 id="permission-rules-combine-differently-than-you-expected">
  Les règles de permission se combinent différemment que vous ne l'attendiez
</h4>

* **Vous avez choisi « Oui, et ne me demande plus » sur une invite de permission mais vous êtes toujours invité pour le même outil.** Ce choix a enregistré une règle `allow` dans votre fichier local, et une règle `allow` là ne surclasse pas une règle `ask` à partir d'un fichier de projet ou géré ; [comment les règles de permission se combinent](/docs/fr/permissions#settings-precedence) explique l'ordre. Dans l'extension VS Code, la carte d'approbation vous permet de choisir le fichier de destination, y compris le fichier partagé du projet, ce qui change la règle pour tout le monde ; dans la CLI, Claude Code écrit uniquement dans votre fichier local.
* **Les règles allow de votre organisation s'appliquent toujours à côté des vôtres.** C'est attendu : Claude Code fusionne [`permissions.allow`](/docs/fr/settings-reference#permissions-allow) entre les portées, sauf si votre organisation définit [`allowManagedPermissionRulesOnly`](/docs/fr/settings-reference#allowmanagedpermissionrulesonly).

<span id="security-keys-where-the-stricter-value-applies" />

<h3 id="exceptions-to-managed-settings-precedence">
  Exceptions à la précédence des paramètres gérés
</h3>

Pour quelques clés dont les valeurs restreignent une session, Claude Code honore une valeur restrictive d'une portée qui ne pourrait autrement pas remplacer les paramètres gérés. Trouvez la clé dans ce tableau pour voir quelle valeur elle honore et d'où.

| Clé                                                                             | Valeur que Claude Code honore                                                                                                              | Notes                                                                                                                                                                               |
| :------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`disableClaudeAiConnectors`](/docs/fr/settings-reference#disableclaudeaiconnectors) | `true` à partir de n'importe quelle portée                                                                                                 | Honorée même quand une source gérée définit `false`                                                                                                                                 |
| [`enableArtifact`](/docs/fr/settings-reference#enableartifact)                       | `false` à partir de n'importe quelle portée, et `disableArtifact: true` à partir de n'importe quelle portée                                | Honorée même quand une source gérée définit `true` ; rien ne réactive l'[outil Artifact](/docs/fr/artifacts#disable-artifacts). Nécessite Claude Code v2.1.242 ou ultérieur              |
| [`isolatePeerMachines`](/docs/fr/settings-reference#isolatepeermachines)             | `true` à partir de n'importe quelle portée                                                                                                 | Honorée même quand une source gérée définit `false`                                                                                                                                 |
| [`remoteControlAtStartup`](/docs/fr/settings-reference#remotecontrolatstartup)       | `false` à partir de `.claude/settings.json` ou `.claude/settings.local.json`                                                               | Honorée même quand une source gérée définit `true` ; un `true` de projet ou local est ignoré                                                                                        |
| [`crossSessionInbound`](/docs/fr/settings-reference#crosssessioninbound)             | Une valeur plus stricte à partir de `.claude/settings.json` ou `.claude/settings.local.json`, sur l'échelle `accept` \< `hold` \< `refuse` | Honorée sur les valeurs gérées, `--settings`, et utilisateur ; une valeur de projet ou local qui n'est pas plus stricte est ignorée                                                 |
| [`useAutoModeDuringPlan`](/docs/fr/settings-reference#useautomodeduringplan)         | `false` à partir de n'importe quelle source gérée, `--settings`, `~/.claude/settings.json`, ou `.claude/settings.local.json`               | Honorée même quand la source gérée gagnante définit `true` ; un `false` dans `.claude/settings.json` est ignoré                                                                     |
| [`syncClaudeAiSkills`](/docs/fr/settings-reference#syncclaudeaiskills)               | `false` à partir de n'importe quelle source gérée, `--settings`, `~/.claude/settings.json`, ou `.claude/settings.local.json`               | Honorée même quand la source gérée gagnante définit `true` ; un `false` dans `.claude/settings.json` est ignoré                                                                     |
| [`syncClaudeAiPlugins`](/docs/fr/settings-reference#syncclaudeaiplugins)             | `false` à partir de n'importe quelle source gérée, `--settings`, `~/.claude/settings.json`, ou `.claude/settings.local.json`               | Honorée même quand la source gérée gagnante définit `true` ; un `false` dans `.claude/settings.json` est ignoré                                                                     |
| [`maxEffortLevel`](/docs/fr/settings-reference#maxeffortlevel)                       | Un plafond inférieur à partir de n'importe quelle portée, y compris `--settings`                                                           | Honorée même quand les paramètres gérés que Claude Code applique définissent un plafond plus élevé ; le plafond le plus bas s'applique. Nécessite Claude Code v2.1.267 ou ultérieur |

Une application qui exécute Claude Code à l'intérieur d'elle-même et définit [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/fr/env-vars) est aussi une exception. Claude Code prend la configuration de modèle de cette application sur les clés `model`, `fallbackModel`, `modelPicker`, et `modelOverrides` à partir de chaque source gérée, et sur les variables de sélection de modèle dans un bloc `env` géré, comme `ANTHROPIC_MODEL` et la famille `ANTHROPIC_DEFAULT_*_MODEL`. Claude Code garde une liste blanche [`availableModels`](/docs/fr/settings-reference#availablemodels) gérée en vigueur sauf si l'application fournit la sienne.

<h2 id="settings-in-cloud-sessions">
  Paramètres dans les sessions cloud
</h2>

Une [session cloud](/docs/fr/claude-code-on-the-web) s'exécute dans un [environnement cloud](/docs/fr/cloud-environments) sur un clone frais de votre référentiel, pas sur votre machine. Cela change quels paramètres l'atteignent :

* **Paramètres de projet partagés** (`.claude/settings.json`) : lus dans une session avec un référentiel, car le fichier fait partie du clone et la session démarre à l'intérieur. Validez un paramètre là pour l'appliquer dans ces sessions. Une session avec plusieurs référentiels démarre au-dessus des clones et lit uniquement les clés `enabledPlugins` et `extraKnownMarketplaces` du `.claude/settings.json` de chaque référentiel, pas les règles de permission, les hooks, `env`, ou d'autres clés. Les marketplaces et les plugins que ces deux clés déclarent ne [se chargent toujours pas dans une session cloud](/docs/fr/cloud-environments#what-carries-over-from-your-setup).
* **Paramètres utilisateur et projet local** (`~/.claude/settings.json` et `.claude/settings.local.json`) : non lus. Les deux restent sur votre machine, et le fichier local n'est pas dans le clone.
* **Paramètres gérés** : seuls les [paramètres gérés par le serveur](/docs/fr/server-managed-settings) atteignent une session cloud ; un fichier `managed-settings.json` ou un profil MDM sur votre appareil ne le font pas. Un [environnement auto-hébergé](/docs/fr/self-hosted-environments) lit aussi le fichier de paramètres gérés dans son image de runner. [Comment Claude Code combine les sources gérées](/docs/fr/managed-settings#how-claude-code-combines-managed-sources) dit quand ce fichier s'applique.
* **`/config`** : dans votre navigateur sur claude.ai/code, ouvre la section Claude Code de vos paramètres claude.ai à la place de modifier une valeur. Pour modifier un paramètre pour une session cloud, définissez une [variable d'environnement](/docs/fr/cloud-environments#set-environment-variables) sur l'environnement, ou dans une session avec un référentiel, validez la clé dans le `.claude/settings.json` de ce référentiel.

[Ce qui se transfère de votre configuration](/docs/fr/cloud-environments#what-carries-over-from-your-setup) énumère le reste : `CLAUDE.md`, skills, serveurs MCP, plugins, et identifiants.

<h2 id="what’s-next">
  Quoi de neuf
</h2>

* [Tous les paramètres](/docs/fr/settings-reference) : chaque clé, avec où vous la définissez et un exemple
* [Fichiers de paramètres d'exemple](/docs/fr/settings-example) : un fichier personnel, un fichier d'équipe, et un fichier géré d'une organisation
* [Configurer les permissions](/docs/fr/permissions) : règles allow, ask, et deny, et ce que Claude Code exécute sans demander
* [Variables d'environnement](/docs/fr/env-vars) : les variables que Claude Code lit et le bloc `env`
* [Déboguer votre configuration](/docs/fr/debug-your-config) : quand un paramètre ne s'applique pas
* [Référence du répertoire Claude](/docs/fr/claude-directory) : chaque fichier que Claude Code lit, y compris les subagents, serveurs MCP, plugins, et `CLAUDE.md`
