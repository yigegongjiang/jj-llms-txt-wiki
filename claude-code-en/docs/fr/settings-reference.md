> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Tous les paramètres

> Référence complète pour chaque clé settings.json de Claude Code : où chacune se trouve, son type et sa valeur par défaut, et un exemple prêt à coller, avec un index de chaque clé.

export const BackToIndex = ({href = '#all-settings', label = 'Back to index'}) => {
  const [show, setShow] = useState(false);
  useEffect(() => {
    const onScroll = () => setShow(window.scrollY > window.innerHeight);
    onScroll();
    window.addEventListener('scroll', onScroll, {
      passive: true
    });
    return () => window.removeEventListener('scroll', onScroll);
  }, []);
  return <div className="not-prose">
      <style>{`
        .bti-btn {
          position: fixed; right: 20px; bottom: 20px; z-index: 40;
          display: inline-flex; align-items: center; gap: 6px;
          padding: 8px 12px; border-radius: 999px;
          font-size: 13px; font-weight: 500; line-height: 1; text-decoration: none;
          color: #1f1f1f; background: #ffffff; border: 1px solid #d9d9d9;
          box-shadow: 0 2px 8px rgba(0,0,0,0.12);
          opacity: 0; pointer-events: none; transform: translateY(6px);
          transition: opacity 160ms ease, transform 160ms ease;
        }
        .bti-btn.bti-show { opacity: 1; pointer-events: auto; transform: translateY(0); }
        .bti-btn:hover { border-color: #b3b3b3; }
        .dark .bti-btn { color: #ececec; background: #1e1e1e; border-color: #3a3a3a; box-shadow: 0 2px 8px rgba(0,0,0,0.5); }
        .dark .bti-btn:hover { border-color: #5a5a5a; }
        @media (max-width: 1279px) { .bti-btn { bottom: 112px; } }
        @media print { .bti-btn { display: none; } }
      `}</style>
      <a className={'bti-btn' + (show ? ' bti-show' : '')} href={href} aria-hidden={!show} tabIndex={show ? 0 : -1}>
        <svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.6" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true"><path d="M8 13V3M3.5 7.5 8 3l4.5 4.5" /></svg>
        {label}
      </a>
    </div>;
};

export const ReferenceFilter = ({placeholder, noun, facets, facetOrder, columnHelp, children}) => {
  const useLive = init => {
    const [v, setV] = useState(init);
    const ref = useRef(init);
    return [v, ref, x => {
      ref.current = x;
      setV(x);
    }];
  };
  const cap = s => s.charAt(0).toUpperCase() + s.slice(1);
  const plural = s => s.endsWith('y') ? s.slice(0, -1) + 'ies' : s + 's';
  const facetNames = facets || ['category', 'topic', 'scope', 'where'];
  const orderOf = {};
  Object.keys(facetOrder || ({})).forEach(k => {
    orderOf[k] = facetOrder[k].map(x => String(x).toLowerCase());
  });
  const rankIn = (col, v) => {
    const list = orderOf[col];
    if (!list) return -1;
    const i = list.indexOf(String(v).toLowerCase());
    return i < 0 ? list.length : i;
  };
  const cmpValues = col => (a, b) => {
    const ra = rankIn(col, a);
    const rb = rankIn(col, b);
    if (ra !== rb) return ra - rb;
    return a < b ? -1 : a > b ? 1 : 0;
  };
  const help = columnHelp || ({});
  const FIRST_COL_HELP = 'Click an entry to open it.';
  const optionLabel = (f, c) => c === 'All' ? 'All ' + plural(f.label.toLowerCase()) : c;
  const nounText = noun || 'entries';
  const placeholderText = placeholder || 'Filter this reference';
  const rootRef = useRef(null);
  const tablesRef = useRef(null);
  const searchRef = useRef(null);
  const menuRef = useRef({});
  const [q, qRef, setQ] = useLive('');
  const [sel, selRef, setSel] = useLive({});
  const [sortBy, sortRef, setSortBy] = useLive(null);
  const [menuOpen, menuOpenRef, setMenu] = useLive(null);
  const [facetList, setFacetList] = useState([]);
  const [firstHead, setFirstHead] = useState('');
  const [counts, setCounts] = useState({
    shown: 0,
    total: 0
  });
  const [disabled, setDisabled] = useState(false);
  const menuBtn = name => menuRef.current[name] ? menuRef.current[name].querySelector(':scope > button') : null;
  const menuList = name => menuRef.current[name] ? menuRef.current[name].querySelector('[role="listbox"]') : null;
  const closeMenu = name => {
    setMenu(null);
    const btn = menuBtn(name);
    if (btn) btn.focus();
  };
  const focusSelected = name => {
    const list = menuList(name);
    if (!list) return;
    const btn = list.querySelector('button[aria-selected="true"]') || list.querySelector('button');
    if (btn) btn.focus();
  };
  const setFacet = (name, value) => {
    setSel(Object.assign({}, selRef.current, {
      [name]: value
    }));
    apply(qRef.current);
    closeMenu(name);
  };
  const sortTables = by => {
    (tablesRef.current || []).forEach(tab => {
      const t = tab.el;
      const idx = tab.heads.indexOf(by);
      const body = t.querySelector('tbody');
      if (idx < 0 || !body) return;
      const rows = [...body.querySelectorAll('tr')];
      const keyOf = r => r.children[idx] ? r.children[idx].textContent.trim().toLowerCase() : '';
      const cmp = cmpValues(by);
      rows.map((r, i) => ({
        r,
        i: Number(r.dataset.sfIndex !== undefined ? r.dataset.sfIndex : i),
        k: keyOf(r)
      })).sort((a, b) => cmp(a.k, b.k) || a.i - b.i).forEach(x => body.appendChild(x.r));
      [...t.querySelectorAll('thead th')].forEach((h, i) => {
        const sortable = tab.heads[i] === tab.heads[0] || facetNames.indexOf(tab.heads[i]) > -1;
        if (sortable) h.setAttribute('aria-sort', i === idx ? 'ascending' : 'none'); else h.removeAttribute('aria-sort');
      });
    });
  };
  const scan = () => {
    const tables = [];
    let el = rootRef.current ? rootRef.current.nextElementSibling : null;
    while (el) {
      if (el.tagName === 'H2' || el.querySelector(':scope > h2')) break;
      const found = el.tagName === 'TABLE' ? [el] : [...el.querySelectorAll('table')];
      found.forEach(t => {
        const headCells = [...t.querySelectorAll('thead th, thead td')];
        const heads = headCells.map(h => h.textContent.trim().toLowerCase());
        if (heads.length === 0) return;
        const facetIdx = {};
        heads.forEach((h, i) => {
          if (facetNames.indexOf(h) > -1) facetIdx[h] = i;
        });
        if (!t.dataset.sfDecorated) {
          t.dataset.sfDecorated = '1';
          headCells.forEach((h, i) => {
            const text = i === 0 ? help[heads[0]] || FIRST_COL_HELP : help[heads[i]];
            if (text) h.title = text;
          });
        }
        const rows = [...t.querySelectorAll('tbody tr')].map((r, i) => {
          if (r.dataset.sfIndex === undefined) r.dataset.sfIndex = String(i);
          const cells = r.querySelectorAll('td');
          const fv = {};
          Object.keys(facetIdx).forEach(h => {
            fv[h] = cells[facetIdx[h]] ? cells[facetIdx[h]].textContent.trim() : '';
          });
          return {
            el: r,
            text: [...cells].map(c => c.textContent).join(' ').toLowerCase(),
            facets: fv,
            anchors: [...r.querySelectorAll('a[href^="#"]')].map(a => a.getAttribute('href').slice(1)),
            ids: [...r.querySelectorAll('[id]')].map(n => n.id)
          };
        });
        tables.push({
          el: t,
          box: t.closest('[data-table-wrapper]') || t,
          rows,
          heads
        });
      });
      el = el.nextElementSibling;
    }
    tablesRef.current = tables;
    if (sortRef.current) sortTables(sortRef.current);
    return tables;
  };
  const apply = query => {
    let tables = tablesRef.current || scan();
    if (tables.some(t => !t.el.isConnected)) tables = scan();
    const needle = query.trim().toLowerCase();
    const sel = selRef.current;
    const activeFacets = Object.keys(sel).filter(h => sel[h] && sel[h] !== 'All');
    const show = (el, on) => {
      const want = on ? '' : 'none';
      if (el.style.display !== want) el.style.display = want;
    };
    let total = 0;
    let shown = 0;
    const visibleTargets = new Set();
    tables.forEach(t => {
      let tableVisible = 0;
      t.rows.forEach(row => {
        total += 1;
        const catOk = activeFacets.every(h => {
          const v = row.facets[h];
          return v === sel[h] || v === '' || v === undefined;
        });
        const match = catOk && (needle === '' || row.text.includes(needle));
        show(row.el, match);
        if (match) {
          tableVisible += 1;
          row.anchors.forEach(a => visibleTargets.add(a));
        }
      });
      show(t.box, !(t.rows.length > 0 && tableVisible === 0));
      shown += tableVisible;
    });
    if (shown < total && visibleTargets.size > 0) {
      tables.forEach(t => {
        t.rows.forEach(row => {
          if (row.el.style.display === 'none' && row.ids.some(id => visibleTargets.has(id))) {
            show(row.el, true);
            show(t.box, true);
            shown += 1;
          }
        });
      });
    }
    setCounts({
      shown,
      total
    });
    return total;
  };
  const deriveFacets = tables => {
    const seen = {};
    tables.forEach(t => t.rows.forEach(r => {
      Object.keys(r.facets).forEach(h => {
        if (!seen[h]) seen[h] = [];
        if (r.facets[h] && seen[h].indexOf(r.facets[h]) === -1) seen[h].push(r.facets[h]);
      });
    }));
    const list = facetNames.filter(h => seen[h] && seen[h].length > 0).map(h => ({
      name: h,
      label: cap(h),
      values: seen[h].sort(cmpValues(h))
    }));
    setFacetList(list);
    const first = tables[0] ? tables[0].heads[0] : '';
    setFirstHead(first);
    if (!sortRef.current && first) {
      setSortBy(first);
      sortTables(first);
    }
    const init = {};
    list.forEach(f => {
      init[f.name] = selRef.current[f.name] || 'All';
    });
    setSel(init);
  };
  const onChange = value => {
    setQ(value);
    apply(value);
  };
  const clearAll = () => {
    const next = {};
    Object.keys(selRef.current).forEach(k => {
      next[k] = 'All';
    });
    setSel(next);
    setQ('');
    apply('');
    if (searchRef.current) searchRef.current.focus();
  };
  useEffect(() => {
    const tables = scan();
    deriveFacets(tables);
    const total = apply('');
    let retryTimer;
    if (total === 0) {
      retryTimer = setTimeout(() => {
        tablesRef.current = null;
        if (apply(qRef.current) > 0) deriveFacets(tablesRef.current); else setDisabled(true);
      }, 500);
    }
    const onKey = e => {
      if (e.key === 'Escape' && menuOpenRef.current !== null) closeMenu(menuOpenRef.current);
      if (!searchRef.current) return;
      if (e.metaKey || e.ctrlKey || e.altKey) return;
      const active = document.activeElement;
      const tag = active && active.tagName;
      const editable = active && active.isContentEditable;
      const interactive = tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT' || tag === 'BUTTON' || tag === 'A' || editable || active && active.getAttribute && active.getAttribute('role');
      if (e.key === '/' && !interactive) {
        const r = rootRef.current ? rootRef.current.getBoundingClientRect() : null;
        if (r && r.bottom > 0 && r.top < (window.innerHeight || 0)) {
          e.preventDefault();
          setMenu(null);
          searchRef.current.focus();
        }
      }
      if (e.key === 'Escape' && menuOpenRef.current === null && active === searchRef.current) {
        onChange('');
        searchRef.current.blur();
      }
    };
    const onDocClick = e => {
      const open = menuOpenRef.current;
      if (open !== null && menuRef.current[open] && !menuRef.current[open].contains(e.target)) setMenu(null);
    };
    window.addEventListener('keydown', onKey);
    document.addEventListener('mousedown', onDocClick);
    return () => {
      if (retryTimer) clearTimeout(retryTimer);
      window.removeEventListener('keydown', onKey);
      document.removeEventListener('mousedown', onDocClick);
      (tablesRef.current || []).forEach(t => {
        t.box.style.display = '';
        t.rows.forEach(row => {
          row.el.style.display = '';
        });
      });
    };
  }, []);
  useEffect(() => {
    if (menuOpen !== null) focusSelected(menuOpen);
  }, [menuOpen]);
  if (disabled) return null;
  const facetActive = Object.keys(sel).some(h => sel[h] && sel[h] !== 'All');
  const sortOptions = [firstHead].concat(facetList.map(f => f.name)).filter((h, i, a) => h && a.indexOf(h) === i);
  return <>
      <style>{`
        .sf-root {
          --sf-accent: #D97757;
          --sf-bg: #fff;
          --sf-border: #E8E6DC;
          --sf-text: #141413;
          --sf-text-3: #73726C;
          --sf-text-4: #9C9A92;
        }
        .dark .sf-root {
          --sf-bg: #1a1918;
          --sf-border: #3a3936;
          --sf-text: #e8e6dc;
          --sf-text-3: #9c9a92;
          --sf-text-4: #73726c;
        }
        .sf-root .sf-end {
          position: absolute;
          right: 10px;
          top: 50%;
          transform: translateY(-50%);
        }
        .sf-root .sf-x {
          background: none;
          border: none;
          cursor: pointer;
          color: var(--sf-text-3);
          font-size: 14px;
          padding: 2px 4px;
          line-height: 1;
        }
      `}</style>
      <div ref={rootRef} className="sf-root" style={{
    margin: '16px 0 8px'
  }}>
        <div style={{
    display: 'flex',
    gap: '8px',
    flexWrap: 'wrap',
    alignItems: 'center'
  }}>
        <div style={{
    position: 'relative',
    flex: '1 1 260px',
    maxWidth: '480px'
  }}>
          <input ref={searchRef} value={q} onChange={e => onChange(e.target.value)} placeholder={placeholderText} aria-label={placeholderText} style={{
    width: '100%',
    padding: '8px 56px 8px 12px',
    borderRadius: '8px',
    border: '1px solid var(--sf-border)',
    background: 'var(--sf-bg)',
    color: 'var(--sf-text)',
    fontSize: '14px',
    outline: 'none',
    boxSizing: 'border-box'
  }} />
          {q ? <button type="button" onClick={() => {
    onChange('');
    if (searchRef.current) searchRef.current.focus();
  }} aria-label="Clear text" className="sf-end sf-x">
              ×
            </button> : <span className="sf-end" style={{
    fontFamily: 'var(--font-mono, ui-monospace, monospace)',
    fontSize: '11px',
    color: 'var(--sf-text-4)',
    border: '1px solid var(--sf-border)',
    borderRadius: '3px',
    padding: '0 5px',
    pointerEvents: 'none'
  }}>
              /
            </span>}
        </div>
        {facetList.map(f => {
    const cur = sel[f.name] || 'All';
    const isOpen = menuOpen === f.name;
    return <div key={f.name} ref={el => {
      menuRef.current[f.name] = el;
    }} style={{
      position: 'relative'
    }}>
            <button type="button" onClick={() => setMenu(isOpen ? null : f.name)} onKeyDown={e => {
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        if (!isOpen) setMenu(f.name); else focusSelected(f.name);
      }
    }} aria-haspopup="listbox" aria-expanded={isOpen} style={{
      display: 'flex',
      alignItems: 'center',
      gap: '8px',
      padding: cur !== 'All' ? '8px 30px 8px 12px' : '8px 12px',
      borderRadius: '8px',
      border: '1px solid ' + (cur !== 'All' ? 'var(--sf-accent)' : 'var(--sf-border)'),
      background: 'var(--sf-bg)',
      color: cur === 'All' ? 'var(--sf-text-3)' : 'var(--sf-text)',
      fontSize: '13.5px',
      cursor: 'pointer',
      whiteSpace: 'nowrap',
      maxWidth: '260px'
    }}>
              <span style={{
      overflow: 'hidden',
      textOverflow: 'ellipsis'
    }}>
                {f.label + ': ' + optionLabel(f, cur)}
              </span>
              <span aria-hidden="true" style={{
      fontSize: '9px',
      color: 'var(--sf-text-4)',
      transform: isOpen ? 'rotate(180deg)' : 'none',
      transition: 'transform 120ms'
    }}>
                ▼
              </span>
            </button>
            {cur !== 'All' && <button type="button" onClick={() => setFacet(f.name, 'All')} aria-label={'Clear ' + f.label + ' filter'} title={'Clear ' + f.label + ' filter'} className="sf-end sf-x">
                ×
              </button>}
            {isOpen && <div role="listbox" aria-label={f.label} onKeyDown={e => {
      const items = [...e.currentTarget.querySelectorAll('button')];
      const idx = items.indexOf(document.activeElement);
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        (items[idx + 1] || items[0]).focus();
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        (items[idx - 1] || items[items.length - 1]).focus();
      } else if (e.key === 'Home') {
        e.preventDefault();
        if (items[0]) items[0].focus();
      } else if (e.key === 'End') {
        e.preventDefault();
        if (items[items.length - 1]) items[items.length - 1].focus();
      } else if (e.key === 'Tab') {
        closeMenu(f.name);
      }
    }} style={{
      position: 'absolute',
      top: 'calc(100% + 6px)',
      left: 0,
      zIndex: 1000,
      minWidth: '260px',
      maxHeight: '340px',
      overflowY: 'auto',
      background: 'var(--sf-bg)',
      border: '1px solid var(--sf-border)',
      borderRadius: '10px',
      boxShadow: '0 8px 24px rgba(0,0,0,0.12)',
      padding: '5px'
    }}>
                {['All'].concat(f.values).map(c => {
      const selected = cur === c;
      return <button key={c} role="option" aria-selected={selected} tabIndex={-1} onClick={() => setFacet(f.name, c)} style={{
        display: 'flex',
        alignItems: 'center',
        gap: '8px',
        width: '100%',
        textAlign: 'left',
        padding: '7px 10px',
        borderRadius: '6px',
        border: 'none',
        background: 'transparent',
        color: selected ? 'var(--sf-accent)' : 'var(--sf-text)',
        fontWeight: selected ? 600 : 400,
        fontSize: '13.5px',
        cursor: 'pointer'
      }}>
                      <span aria-hidden="true" style={{
        width: '14px',
        color: 'var(--sf-accent)',
        flexShrink: 0
      }}>
                        {selected ? '✓' : ''}
                      </span>
                      {optionLabel(f, c)}
                    </button>;
    })}
              </div>}
          </div>;
  })}
        {sortOptions.length > 1 && <div role="group" aria-label="Sort by" style={{
    display: 'flex',
    alignItems: 'center',
    gap: '4px',
    fontSize: '13px',
    color: 'var(--sf-text-3)',
    whiteSpace: 'nowrap'
  }}>
            <span style={{
    marginRight: '4px'
  }}>Sort by</span>
            {sortOptions.map(o => {
    const on = sortBy === o;
    return <button key={o} type="button" aria-pressed={on} onClick={() => {
      setSortBy(o);
      sortTables(o);
    }} style={{
      padding: '6px 10px',
      borderRadius: '8px',
      border: '1px solid ' + (on ? 'var(--sf-accent)' : 'var(--sf-border)'),
      background: 'var(--sf-bg)',
      color: on ? 'var(--sf-text)' : 'var(--sf-text-3)',
      fontSize: '13px',
      cursor: 'pointer'
    }}>
                  {o.charAt(0).toUpperCase() + o.slice(1)}
                </button>;
  })}
          </div>}
        </div>
        <div aria-live="polite" style={{
    margin: '8px 0 0',
    fontSize: '13px',
    color: 'var(--sf-text-3)',
    minHeight: '1px'
  }}>
          {q.trim() === '' && !facetActive ? <>{counts.total} {nounText}</> : counts.shown === 0 ? <>
                {q.trim() === '' ? 'No ' + nounText + ' match the selected filters.' : facetActive ? 'No ' + nounText + ' match \u201c' + q + '\u201d with the selected filters.' : 'No ' + nounText + ' match \u201c' + q + '\u201d.'}{' '}
                <button type="button" onClick={clearAll} style={{
    background: 'none',
    border: 'none',
    padding: 0,
    color: 'var(--sf-accent)',
    cursor: 'pointer',
    font: 'inherit',
    textDecoration: 'underline'
  }}>
                  Clear filters
                </button>
                {children ? <> {children}</> : null}
              </> : <>
                Showing {counts.shown} of {counts.total} {nounText}
              </>}
        </div>
      </div>
    </>;
};

<BackToIndex href="#all-settings" label="Retour à l'index" />

Cette page de référence répertorie chaque clé que Claude Code lit à partir d'un fichier de paramètres, plus le [court groupe de clés](#global-config-settings) qu'il conserve dans `~/.claude.json` à la place. Pour choisir un fichier ou vérifier la précédence, commencez par [Fichiers de paramètres et précédence](/docs/fr/settings).

<span id="available-settings" />

<span id="scopes" />

<span id="all-settings" />

<h2 id="settings-index">
  Index des paramètres
</h2>

Chaque clé ci-dessous renvoie à son entrée. La portée liste les [fichiers](/docs/fr/settings#settings-files-and-who-they-affect) dans lesquels elle peut aller : `User` est `~/.claude/settings.json`, `Project` est `.claude/settings.json`, `Local` est `.claude/settings.local.json`, et `Managed` est [ce que votre organisation déploie](/docs/fr/managed-settings). `Any file` signifie les quatre, et `Global config` signifie [`~/.claude.json`](#global-config-settings).

<ReferenceFilter
  noun="settings"
  placeholder="Filter settings by key or purpose"
  facetOrder={{ scope: ["Any file", "User, local, or managed", "User or managed", "Managed", "Global config"] }}
  columnHelp={{
topic: "The section of this page that holds the entry. Use Sort by to group the table by topic.",
scope: "Which settings files can set the key: user (~/.claude/settings.json), project (.claude/settings.json), local (.claude/settings.local.json), or managed (deployed by your organization). Global config keys are in ~/.claude.json instead.",
}}
/>

| Key                                                                                                   | Description                                                                                                                                                                                                                                                              | Topic                              | Scope                   |
| :---------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------- | :---------------------- |
| [`advisorModel`](#advisormodel)                                                                       | Choisissez quel modèle répond quand Claude demande l'[outil conseiller](/docs/fr/advisor)                                                                                                                                                                                     | Model and responses                | Any file                |
| [`agent`](#agent)                                                                                     | Commencez chaque session en tant que [sous-agent](/docs/fr/sub-agents) nommé avec son invite, ses outils et son modèle                                                                                                                                                        | Agents, sessions, and worktrees    | Any file                |
| [`agentPushNotifEnabled`](#agentpushnotifenabled)                                                     | Laissez Claude envoyer une [notification push à votre téléphone](/docs/fr/remote-control#mobile-push-notifications) quand il le décide                                                                                                                                        | Remote, desktop, and notifications | Any file                |
| [`allowAllClaudeAiMcps`](#allowallclaudeaimcps)                                                       | Chargez les [connecteurs claude.ai](/docs/fr/mcp) que Claude Code récupère lui-même aux côtés d'un [`managed-mcp.json`](/docs/fr/managed-mcp#exclusive-control-with-managed-mcp-json) déployé                                                                                      | MCP                                | Managed                 |
| [`allowedChannelPlugins`](#allowedchannelplugins)                                                     | Remplacez la liste d'autorisation par défaut des [plugins de canal](/docs/fr/channels#restrict-which-channel-plugins-can-run) qui peuvent envoyer des messages                                                                                                                | Plugins and skills                 | Managed                 |
| [`allowedHttpHookUrls`](#allowedhttphookurls)                                                         | Limitez les URL que les [hooks HTTP](/docs/fr/hooks) peuvent cibler                                                                                                                                                                                                           | Hooks and automation               | Any file                |
| [`allowedMcpServers`](#allowedmcpservers)                                                             | Liste d'autorisation des [serveurs MCP](/docs/fr/mcp) que les utilisateurs peuvent ajouter                                                                                                                                                                                    | MCP                                | Any file                |
| [`allowManagedHooksOnly`](#allowmanagedhooksonly)                                                     | Exécutez uniquement les [hooks](/docs/fr/hooks) que votre organisation déploie                                                                                                                                                                                                | Hooks and automation               | Managed                 |
| [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly)                                           | Rendez la liste d'autorisation [MCP](/docs/fr/mcp) gérée la seule qui s'applique                                                                                                                                                                                              | MCP                                | Managed                 |
| [`allowManagedPermissionRulesOnly`](#allowmanagedpermissionrulesonly)                                 | Rendez les [paramètres gérés](/docs/fr/managed-settings) la seule source de paramètres des [règles de permission](/docs/fr/permissions#managed-settings)                                                                                                                           | Permission settings                | Managed                 |
| [`alwaysThinkingEnabled`](#alwaysthinkingenabled)                                                     | Désactivez la [réflexion étendue](/docs/fr/model-config#extended-thinking) pour chaque session                                                                                                                                                                                | Model and responses                | Any file                |
| [`apiKeyHelper`](#apikeyhelper)                                                                       | Générez les [identifiants API](/docs/fr/authentication#credential-management) avec votre propre commande                                                                                                                                                                      | Authentication and providers       | Any file                |
| [`askUserQuestionTimeout`](#askuserquestiontimeout)                                                   | Laissez une question sans réponse [continuer automatiquement](/docs/fr/tools-reference#question-auto-continue-timeout) après un temps d'inactivité                                                                                                                            | Interface and terminal             | User or managed         |
| [`attribution`](#attribution)                                                                         | Personnalisez l'attribution que Claude Code ajoute aux commits et aux demandes de tirage                                                                                                                                                                                 | Git and attribution                | Any file                |
| [`attribution.commit`](#attribution-commit)                                                           | Modifiez ou masquez la bande-annonce que Claude Code ajoute aux commits                                                                                                                                                                                                  | Git and attribution                | Any file                |
| [`attribution.pr`](#attribution-pr)                                                                   | Modifiez ou masquez la ligne d'attribution dans les descriptions des demandes de tirage                                                                                                                                                                                  | Git and attribution                | Any file                |
| [`attribution.sessionUrl`](#attribution-sessionurl)                                                   | Omettez le lien de session claude.ai des commits [cloud](/docs/fr/claude-code-on-the-web) et [Remote Control](/docs/fr/remote-control)                                                                                                                                             | Git and attribution                | Any file                |
| [`autoCompactEnabled`](#autocompactenabled)                                                           | Désactivez ou activez la [compaction automatique](/docs/fr/context-window)                                                                                                                                                                                                    | Memory and context                 | Any file                |
| [`autoCompactWindow`](#autocompactwindow)                                                             | Définissez le remplissage du contexte avant que Claude Code [compacte](/docs/fr/context-window)                                                                                                                                                                               | Memory and context                 | Any file                |
| [`autoConnectIde`](#autoconnectide)                                                                   | Connectez-vous automatiquement à un IDE [VS Code](/docs/fr/vs-code) ou [JetBrains](/docs/fr/jetbrains#from-external-terminals) en cours d'exécution à partir d'un terminal externe                                                                                                 | Global config settings             | Global config           |
| [`autoContinueAtUsageLimit`](#autocontinueatusagelimit)                                               | Attendez dans la session ouverte et [continuez la tâche automatiquement](/docs/fr/interactive-mode#wait-for-a-usage-limit-to-reset) après la réinitialisation d'une limite d'utilisation claude.ai                                                                            | Interface and terminal             | User or managed         |
| [`autoInstallIdeExtension`](#autoinstallideextension)                                                 | Désactivez l'installation automatique de l'[extension IDE](/docs/fr/vs-code#install-the-extension) à partir d'un terminal VS Code                                                                                                                                             | Global config settings             | Global config           |
| [`autoMemoryDirectory`](#automemorydirectory)                                                         | Stockez la [mémoire automatique](/docs/fr/memory#auto-memory) dans un répertoire de votre choix                                                                                                                                                                               | Memory and context                 | Any file                |
| [`autoMemoryEnabled`](#automemoryenabled)                                                             | Désactivez ou activez la [mémoire automatique](/docs/fr/memory#auto-memory)                                                                                                                                                                                                   | Memory and context                 | Any file                |
| [`autoMode`](#automode)                                                                               | Ajoutez vos propres règles d'autorisation et de refus au classificateur du [mode automatique](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode)                                                                                                                     | Permission settings                | User or managed         |
| [`autoMode.classifyAllShell`](#automode-classifyallshell)                                             | Envoyez chaque commande shell via le [classificateur du mode automatique](/docs/fr/permission-modes#what-the-classifier-blocks-by-default), même celles qu'une règle d'autorisation étroite correspond                                                                        | Permission settings                | User or managed         |
| [`autoScrollEnabled`](#autoscrollenabled)                                                             | [Suivez la nouvelle sortie](/docs/fr/fullscreen#auto-follow) jusqu'au bas du rendu en plein écran                                                                                                                                                                             | Interface and terminal             | Any file                |
| [`autoUpdatesChannel`](#autoupdateschannel)                                                           | Suivez le [canal de version](/docs/fr/setup#configure-release-channel) stable au lieu du dernier                                                                                                                                                                              | Updates and versioning             | Any file                |
| [`availableModels`](#availablemodels)                                                                 | [Limitez les modèles](/docs/fr/model-config#restrict-model-selection) que les gens peuvent choisir                                                                                                                                                                            | Model and responses                | Any file                |
| [`awaySummaryEnabled`](#awaysummaryenabled)                                                           | Désactivez le [récapitulatif de session](/docs/fr/interactive-mode#session-recap) affiché quand vous revenez au terminal                                                                                                                                                      | Remote, desktop, and notifications | Any file                |
| [`awsAuthRefresh`](#awsauthrefresh)                                                                   | Actualisez les [identifiants Bedrock](/docs/fr/amazon-bedrock#advanced-credential-configuration) expirés dans `.aws` avec votre propre commande                                                                                                                               | Authentication and providers       | Any file                |
| [`awsCredentialExport`](#awscredentialexport)                                                         | Fournissez les [identifiants Bedrock](/docs/fr/amazon-bedrock#advanced-credential-configuration) en JSON à partir de votre propre commande                                                                                                                                    | Authentication and providers       | Any file                |
| [`axScreenReader`](#axscreenreader)                                                                   | Rendez la [sortie accessible aux lecteurs d'écran](/docs/fr/accessibility)                                                                                                                                                                                                    | Interface and terminal             | Any file                |
| [`bashEditDiffEnabled`](#basheditdiffenabled)                                                         | Enregistrez les [fichiers qui ont changé pendant l'exécution d'une commande Bash](/docs/fr/hooks#bash) dans chaque mode de permission                                                                                                                                         | Interface and terminal             | User or managed         |
| [`bashOutputMaxChars`](#bashoutputmaxchars)                                                           | Définissez la quantité de [sortie](/docs/fr/tools-reference#output-limits) d'une commande réussie que Claude reçoit en ligne                                                                                                                                                  | Memory and context                 | Any file                |
| [`blockedMarketplaces`](#blockedmarketplaces)                                                         | Bloquez les sources du [marché de plugins](/docs/fr/plugins/overview) pour votre organisation                                                                                                                                                                                 | Plugins and skills                 | Managed                 |
| [`browserExternalPageTools`](#browserexternalpagetools)                                               | Gardez les outils de Claude hors des pages externes dans le volet [Bureau](/docs/fr/desktop) Browser                                                                                                                                                                          | Tools                              | Managed                 |
| [`channelsEnabled`](#channelsenabled)                                                                 | Autorisez les [canaux](/docs/fr/channels#enable-channels-for-your-organization) pour votre organisation                                                                                                                                                                       | Plugins and skills                 | Managed                 |
| [`claudeMd`](#claudemd)                                                                               | Injectez les instructions [CLAUDE.md](/docs/fr/memory#deploy-organization-wide-claude-md) à l'échelle de l'organisation à partir des paramètres gérés                                                                                                                         | Memory and context                 | Managed                 |
| [`claudeMdExcludes`](#claudemdexcludes)                                                               | Ignorez les fichiers [CLAUDE.md](/docs/fr/memory#exclude-specific-claude-md-files) spécifiques lors du chargement de la mémoire                                                                                                                                               | Memory and context                 | Any file                |
| [`cleanupPeriodDays`](#cleanupperioddays)                                                             | Choisissez le nombre de jours que Claude Code conserve les [transcriptions](/docs/fr/data-usage#data-retention) avant de les supprimer                                                                                                                                        | Privacy and telemetry              | Any file                |
| [`companyAnnouncements`](#companyannouncements)                                                       | Affichez les annonces de votre organisation au démarrage                                                                                                                                                                                                                 | Interface and terminal             | Any file                |
| [`copyOnSelect`](#copyonselect)                                                                       | Désactivez la copie automatique du texte que vous sélectionnez avec la souris dans le [rendu en plein écran](/docs/fr/fullscreen#use-the-mouse) et la vue agent                                                                                                               | Global config settings             | Global config           |
| [`crossSessionInbound`](#crosssessioninbound)                                                         | Choisissez si Claude Code livre les [messages de vos autres sessions](/docs/fr/cross-session-messaging#control-inbound-messages), affiche un avis sans les livrer, ou les refuse                                                                                              | Agents, sessions, and worktrees    | Any file                |
| [`defaultShell`](#defaultshell)                                                                       | Choisissez si Bash ou PowerShell exécute les commandes shell que vous tapez avec le préfixe [`!`](/docs/fr/interactive-mode#shell-mode-with-prefix)                                                                                                                           | Interface and terminal             | Any file                |
| [`deniedMcpServers`](#deniedmcpservers)                                                               | Bloquez les [serveurs MCP](/docs/fr/mcp) spécifiques par URL, commande ou nom                                                                                                                                                                                                 | MCP                                | Any file                |
| [`desktopSessionCleanupPeriodDays`](#desktopsessioncleanupperioddays)                                 | Définissez une limite d'âge en jours pour les [transcriptions Claude Desktop et Cowork](/docs/fr/claude-directory#cleaned-up-automatically)                                                                                                                                   | Privacy and telemetry              | User or managed         |
| [`dialogExpiry`](#dialogexpiry)                                                                       | Définissez le temps que Claude Code attend pour que [Remote Control](/docs/fr/remote-control) ou un hôte SDK réponde à une boîte de dialogue transférée avant de l'annuler                                                                                                    | Interface and terminal             | User or managed         |
| [`diffTool`](#difftool)                                                                               | Choisissez si les modifications de fichiers proposées par Claude s'ouvrent dans le visualiseur de diff [VS Code](/docs/fr/vs-code) ou [JetBrains](/docs/fr/jetbrains#features) ou restent dans le terminal                                                                         | Global config settings             | Global config           |
| [`disableAgentView`](#disableagentview)                                                               | Désactivez les agents d'arrière-plan et la [vue agent](/docs/fr/agent-view)                                                                                                                                                                                                   | Agents, sessions, and worktrees    | Any file                |
| [`disableAllHooks`](#disableallhooks)                                                                 | Désactivez les [hooks](/docs/fr/hooks), une [ligne d'état](/docs/fr/statusline) personnalisée, et une commande [`@`](/docs/fr/interactive-mode#quick-commands) de suggestion de fichier personnalisée à la fois                                                                         | Hooks and automation               | Any file                |
| [`disableArtifact`](#disableartifact)                                                                 | Obsolète ; utilisez `enableArtifact` pour désactiver l'[outil Artifact](/docs/fr/artifacts)                                                                                                                                                                                   | Remote, desktop, and notifications | Any file                |
| [`disableAutoMode`](#disableautomode)                                                                 | Supprimez le [mode automatique](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) du cycle du mode de permission                                                                                                                                                    | Permission settings                | Any file                |
| [`disableBrowserExternalNavigation`](#disablebrowserexternalnavigation)                               | Limitez le volet [Bureau](/docs/fr/desktop) Browser à localhost pour les personnes et Claude                                                                                                                                                                                  | Tools                              | Managed                 |
| [`disableBundledSkills`](#disablebundledskills)                                                       | Désactivez les [compétences](/docs/fr/skills#bundled-skills) et les [flux de travail](/docs/fr/workflows) inclus avec Claude Code                                                                                                                                                  | Plugins and skills                 | Any file                |
| [`disableClaudeAiConnectors`](#disableclaudeaiconnectors)                                             | Désactivez les [connecteurs claude.ai](/docs/fr/mcp#disable-claude-ai-connectors) pour que Claude Code ne les récupère pas                                                                                                                                                    | MCP                                | Any file                |
| [`disableCommandPluginSources`](#disablecommandpluginsources)                                         | Bloquez les [plugins](/docs/fr/plugins/overview) qui s'installent en exécutant une commande déclarée par le marché                                                                                                                                                            | Plugins and skills                 | Managed                 |
| [`disableDeepLinkRegistration`](#disabledeeplinkregistration)                                         | Empêchez Claude Code d'enregistrer le gestionnaire [`claude-cli://`](/docs/fr/deep-links)                                                                                                                                                                                     | Remote, desktop, and notifications | Any file                |
| [`disableDesktopLocalSessions`](#disabledesktoplocalsessions)                                         | Désactivez les [sessions Desktop Code](/docs/fr/desktop#local-sessions-on-managed-devices) qui s'exécutent sur l'appareil, en laissant SSH à d'autres hôtes et au cloud                                                                                                       | Remote, desktop, and notifications | Managed                 |
| [`disabledMcpjsonServers`](#disabledmcpjsonservers)                                                   | Rejetez les serveurs spécifiques du [`.mcp.json`](/docs/fr/mcp#project-scope) d'un projet                                                                                                                                                                                     | MCP                                | Any file                |
| [`disableMobileSimulatorTools`](#disablemobilesimulatortools)                                         | Bloquez les outils de Claude dans le volet [Bureau](/docs/fr/desktop) iOS Simulator                                                                                                                                                                                           | Tools                              | Managed                 |
| [`disableRemoteControl`](#disableremotecontrol)                                                       | Désactivez [Remote Control](/docs/fr/remote-control) partout où il peut démarrer                                                                                                                                                                                              | Remote, desktop, and notifications | Any file                |
| [`disableSideloadFlags`](#disablesideloadflags)                                                       | Rejetez les drapeaux CLI qui chargent les [plugins](/docs/fr/plugins/overview), les [sous-agents](/docs/fr/sub-agents), et les [serveurs MCP](/docs/fr/mcp)                                                                                                                             | Enterprise and managed settings    | Managed                 |
| [`disableSkillShellExecution`](#disableskillshellexecution)                                           | Empêchez les [compétences](/docs/fr/skills) et les commandes personnalisées d'exécuter le shell en ligne                                                                                                                                                                      | Plugins and skills                 | Any file                |
| [`disableWorkflows`](#disableworkflows)                                                               | Désactivez les [flux de travail dynamiques](/docs/fr/workflows) pour tout le monde ; utilisez `enableWorkflows` pour vous-même                                                                                                                                                | Hooks and automation               | Any file                |
| [`editorMode`](#editormode)                                                                           | Utilisez les [liaisons de touches vim](/docs/fr/interactive-mode#vim-editor-mode) dans l'invite d'entrée                                                                                                                                                                      | Interface and terminal             | Any file                |
| [`effortLevel`](#effortlevel)                                                                         | Définissez un [niveau d'effort](/docs/fr/model-config#adjust-effort-level) par défaut pour les modèles sans niveau enregistré                                                                                                                                                 | Model and responses                | Any file                |
| [`emojiCompletionEnabled`](#emojicompletionenabled)                                                   | Désactivez les [suggestions et remplacements d'emoji `:shortcode:`](/docs/fr/interactive-mode#emoji-shortcodes) dans l'entrée d'invite                                                                                                                                        | Interface and terminal             | Any file                |
| [`enableAllProjectMcpServers`](#enableallprojectmcpservers)                                           | Approuvez chaque serveur dans les fichiers [`.mcp.json`](/docs/fr/mcp#project-server-approvals-and-workspace-trust) du projet sans invite                                                                                                                                     | MCP                                | Any file                |
| [`enableArtifact`](#enableartifact)                                                                   | Désactivez l'[outil Artifact](/docs/fr/artifacts) avec un `false` dans n'importe quel fichier ; aucun fichier ne peut le réactiver                                                                                                                                            | Remote, desktop, and notifications | Any file                |
| [`enabledMcpjsonServers`](#enabledmcpjsonservers)                                                     | Approuvez les serveurs spécifiques du [`.mcp.json`](/docs/fr/mcp#project-server-approvals-and-workspace-trust) d'un projet                                                                                                                                                    | MCP                                | Any file                |
| [`enabledPlugins`](#enabledplugins)                                                                   | Activez ou désactivez les [plugins](/docs/fr/plugins/overview) individuels par portée                                                                                                                                                                                         | Plugins and skills                 | Any file                |
| [`enableWorkflows`](#enableworkflows)                                                                 | Activez ou désactivez les [flux de travail dynamiques](/docs/fr/workflows) par rapport à la valeur par défaut de votre plan                                                                                                                                                   | Hooks and automation               | Any file                |
| [`enforceAvailableModels`](#enforceavailablemodels)                                                   | Gardez le [choix par défaut `/model`](/docs/fr/model-config#enforce-the-allowlist-for-the-default-model) dans votre liste d'autorisation `availableModels`                                                                                                                    | Model and responses                | Any file                |
| [`env`](#env)                                                                                         | Définissez les [variables d'environnement](/docs/fr/env-vars#in-settings-files) pour chaque session et ses sous-processus                                                                                                                                                     | Memory and context                 | Any file                |
| [`externalEditorContext`](#externaleditorcontext)                                                     | Affichez la dernière réponse de Claude en tant que commentaires quand vous appuyez sur [Ctrl+G](/docs/fr/interactive-mode#general-controls) pour éditer                                                                                                                       | Global config settings             | Global config           |
| [`extraKnownMarketplaces`](#extraknownmarketplaces)                                                   | Enregistrez les [marchés](/docs/fr/plugins/overview) pour un référentiel ou une organisation                                                                                                                                                                                  | Plugins and skills                 | Any file                |
| [`fallbackModel`](#fallbackmodel)                                                                     | Nommez les [modèles de secours](/docs/fr/model-config#fallback-model-chains) pour quand le principal est surchargé                                                                                                                                                            | Model and responses                | Any file                |
| [`fastMode`](#fastmode)                                                                               | Activez le [mode rapide](/docs/fr/fast-mode) pour les sessions où il est disponible                                                                                                                                                                                           | Model and responses                | Any file                |
| [`fastModePerSessionOptIn`](#fastmodepersessionoptin)                                                 | Exigez que les gens activent le [mode rapide](/docs/fr/fast-mode) à chaque session                                                                                                                                                                                            | Model and responses                | Any file                |
| [`feedbackDrafts`](#feedbackdrafts)                                                                   | Contrôlez si Claude met en file d'attente les [brouillons de commentaires](/docs/fr/tools-reference#sendfeedback-tool-behavior) pour que vous les examiniez                                                                                                                   | Privacy and telemetry              | User or managed         |
| [`feedbackSurveyRate`](#feedbacksurveyrate)                                                           | Modifiez la fréquence d'apparition de l'[enquête de qualité de session](/docs/fr/data-usage#session-quality-surveys)                                                                                                                                                          | Privacy and telemetry              | Any file                |
| [`fileCheckpointingEnabled`](#filecheckpointingenabled)                                               | Désactivez ou activez les instantanés de fichiers que [`/rewind`](/docs/fr/checkpointing) restaure                                                                                                                                                                            | Memory and context                 | Any file                |
| [`fileSuggestion`](#filesuggestion)                                                                   | Fournissez l'[autocomplétion de fichier `@`](/docs/fr/interactive-mode#quick-commands) à partir de votre propre commande                                                                                                                                                      | Interface and terminal             | Any file                |
| [`footerLinksRegexes`](#footerlinksregexes)                                                           | Transformez les ID de problème ou d'examen en sortie en [liens cliquables](/docs/fr/statusline#clickable-links) sous la zone d'entrée                                                                                                                                         | Interface and terminal             | User or managed         |
| [`forceLoginGatewayUrl`](#forcelogingatewayurl)                                                       | Définissez l'[URL de passerelle](/docs/fr/claude-apps-gateway#set-the-gateway-url) à laquelle l'écran de connexion se connecte                                                                                                                                                | Authentication and providers       | Managed                 |
| [`forceLoginMethod`](#forceloginmethod)                                                               | [Limitez la connexion](/docs/fr/authentication#restrict-login-to-your-organization) à claude.ai, Claude Console, ou une [passerelle cloud](/docs/fr/claude-apps-gateway)                                                                                                           | Authentication and providers       | Any file                |
| [`forceLoginOrgUUID`](#forceloginorguuid)                                                             | [Épinglez les connexions claude.ai à votre organisation](/docs/fr/authentication#restrict-login-to-your-organization) ; seule une source gérée l'applique                                                                                                                     | Authentication and providers       | Any file                |
| [`forceRemoteSettingsRefresh`](#forceremotesettingsrefresh)                                           | Bloquez le démarrage jusqu'à ce que les [paramètres gérés par serveur](/docs/fr/server-managed-settings) soient fraîchement récupérés                                                                                                                                         | Enterprise and managed settings    | Managed                 |
| [`gatewayInternalNetworks`](#gatewayinternalnetworks)                                                 | Laissez `/login` atteindre une [passerelle cloud](/docs/fr/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) sur l'espace IPv4 public que votre organisation utilise en interne                                                                            | Authentication and providers       | Managed                 |
| [`gcpAuthRefresh`](#gcpauthrefresh)                                                                   | Actualisez les [identifiants Google Cloud](/docs/fr/google-vertex-ai#advanced-credential-configuration) avec votre propre commande                                                                                                                                            | Authentication and providers       | Any file                |
| [`hooks`](#hooks)                                                                                     | Exécutez vos propres commandes en tant que [hooks](/docs/fr/hooks) à des points du cycle de vie de Claude Code                                                                                                                                                                | Hooks and automation               | Any file                |
| [`httpHookAllowedEnvVars`](#httphookallowedenvvars)                                                   | Limitez les variables d'environnement que les [hooks HTTP](/docs/fr/hooks) peuvent mettre dans les en-têtes                                                                                                                                                                   | Hooks and automation               | Any file                |
| [`includeCoAuthoredBy`](#includecoauthoredby)                                                         | Obsolète ; utilisez `attribution` pour masquer ou modifier l'attribution de commit et de PR                                                                                                                                                                              | Git and attribution                | Any file                |
| [`includeGitInstructions`](#includegitinstructions)                                                   | Supprimez les instructions de commit et de PR intégrées du contexte de Claude                                                                                                                                                                                            | Git and attribution                | Any file                |
| [`inputNeededNotifEnabled`](#inputneedednotifenabled)                                                 | Recevez une [notification push](/docs/fr/remote-control#mobile-push-notifications) quand Claude vous attend                                                                                                                                                                   | Remote, desktop, and notifications | Any file                |
| [`isolatePeerMachines`](#isolatepeermachines)                                                         | Demandez-vous avant que Claude [envoie un message à l'une de vos sessions sur une autre machine](/docs/fr/cross-session-messaging#require-approval-for-cross-machine-messages)                                                                                                | Agents, sessions, and worktrees    | Any file                |
| [`keybindingFlavor`](#keybindingflavor)                                                               | Obsolète et sans effet ; les raccourcis d'édition de mots suivent toujours les [conventions readline](/docs/fr/interactive-mode#make-ctrl-w-delete-back-to-whitespace)                                                                                                        | Interface and terminal             | Any file                |
| [`language`](#language)                                                                               | Faites répondre Claude dans une langue autre que l'anglais                                                                                                                                                                                                               | Model and responses                | Any file                |
| [`managedMcpServers`](#managedmcpservers)                                                             | Fournissez les [serveurs MCP](/docs/fr/managed-mcp#provide-servers-through-managed-settings) distants à chaque utilisateur aux côtés de ceux qu'ils ajoutent                                                                                                                  | MCP                                | Managed                 |
| [`managedSourcesBehavior`](#managedsourcesbehavior)                                                   | Composez chaque [source gérée](/docs/fr/managed-settings#how-claude-code-combines-managed-sources) que vous déployez au lieu d'utiliser la plus prioritaire seule                                                                                                             | Enterprise and managed settings    | Managed                 |
| [`maxEffortLevel`](#maxeffortlevel)                                                                   | Limitez le [niveau d'effort](/docs/fr/model-config#adjust-effort-level) pour chaque modèle ou par modèle, sur chaque fournisseur                                                                                                                                              | Model and responses                | Any file                |
| [`minimumVersion`](#minimumversion)                                                                   | Empêchez les [mises à jour automatiques](/docs/fr/setup#pin-a-minimum-version) d'installer quoi que ce soit en dessous d'une version                                                                                                                                          | Updates and versioning             | Any file                |
| [`model`](#model)                                                                                     | Modifiez le [modèle](/docs/fr/model-config#set-a-default-model-for-new-sessions) avec lequel Claude Code démarre                                                                                                                                                              | Model and responses                | Any file                |
| [`modelOverrides`](#modeloverrides)                                                                   | [Mappez les ID de modèle](/docs/fr/model-config#override-model-ids-per-version) aux ID de votre fournisseur, comme les ARN Bedrock                                                                                                                                            | Model and responses                | Any file                |
| [`modelPicker`](#modelpicker)                                                                         | Choisissez les modèles que le sélecteur [`/model`](/docs/fr/model-config#available-models) liste, dans votre propre ordre et avec vos propres étiquettes                                                                                                                      | Model and responses                | User or managed         |
| [`modelPricing`](#modelpricing)                                                                       | Signalez les dépenses aux tarifs contractés de votre organisation au lieu du prix catalogue                                                                                                                                                                              | Model and responses                | Managed                 |
| [`modelSettings`](#modelsettings)                                                                     | Conservez un [niveau d'effort](/docs/fr/model-config#adjust-effort-level) enregistré par modèle, ou limitez l'effort d'un modèle                                                                                                                                              | Model and responses                | Any file                |
| [`otelHeadersHelper`](#otelheadershelper)                                                             | Générez les en-têtes [OpenTelemetry](/docs/fr/monitoring-usage#dynamic-headers) rotatifs avec votre propre commande                                                                                                                                                           | Authentication and providers       | Any file                |
| [`outputStyle`](#outputstyle)                                                                         | Modifiez le rôle, le ton et le format de sortie de Claude avec un [style de sortie](/docs/fr/output-styles)                                                                                                                                                                   | Model and responses                | Any file                |
| [`parentSettingsBehavior`](#parentsettingsbehavior)                                                   | Appliquez ou supprimez les restrictions qu'un [hôte SDK ou IDE](/docs/fr/managed-settings#let-an-embedding-host-add-policy) transmet quand vous déployez les [paramètres gérés](/docs/fr/managed-settings)                                                                         | Enterprise and managed settings    | Managed                 |
| [`permissionExplainerEnabled`](#permissionexplainerenabled)                                           | Supprimé dans v2.1.257, ainsi que l'explication de la commande `Ctrl+E` sur les invites de permission shell                                                                                                                                                              | Global config settings             | Global config           |
| [`permissions`](#permissions)                                                                         | Définissez les règles d'autorisation, de demande et de refus et le [mode de permission](/docs/fr/permission-modes) de démarrage                                                                                                                                               | Permission settings                | Any file                |
| [`permissions.additionalDirectories`](#permissions-additionaldirectories)                             | Donnez à Claude l'accès aux fichiers des [répertoires en dehors du répertoire actuel](/docs/fr/permissions#working-directories)                                                                                                                                               | Permission settings                | Any file                |
| [`permissions.allow`](#permissions-allow)                                                             | Approuvez les [utilisations d'outils](/docs/fr/permissions#permission-rule-syntax) listées sans invite                                                                                                                                                                        | Permission settings                | Any file                |
| [`permissions.ask`](#permissions-ask)                                                                 | Invitez toujours avant les [utilisations d'outils](/docs/fr/permissions#permission-rule-syntax) listées                                                                                                                                                                       | Permission settings                | Any file                |
| [`permissions.blockReadsOutsideWorkingDirectories`](#permissions-blockreadsoutsideworkingdirectories) | Faites refuser les lectures des outils de fichier en dehors des [répertoires de travail](/docs/fr/permissions#working-directories) dans chaque mode de permission                                                                                                             | Permission settings                | Any file                |
| [`permissions.defaultMode`](#permissions-defaultmode)                                                 | Définissez le [mode de permission](/docs/fr/permission-modes#which-mode-a-session-starts-in) dans lequel les nouvelles sessions commencent                                                                                                                                    | Permission settings                | Any file                |
| [`permissions.deny`](#permissions-deny)                                                               | Bloquez les [utilisations d'outils](/docs/fr/permissions#permission-rule-syntax) listées, y compris les lectures de fichiers qui contiennent des secrets                                                                                                                      | Permission settings                | Any file                |
| [`permissions.disableBypassPermissionsMode`](#permissions-disablebypasspermissionsmode)               | Empêchez quiconque d'entrer dans le [mode bypassPermissions](/docs/fr/permission-modes#skip-all-checks-with-bypasspermissions-mode)                                                                                                                                           | Permission settings                | Any file                |
| [`plansDirectory`](#plansdirectory)                                                                   | Choisissez où le [mode plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode) écrit les fichiers de plan                                                                                                                                                     | Memory and context                 | Any file                |
| [`pluginConfigs`](#pluginconfigs)                                                                     | Stockez les réponses que vous avez données à la boîte de dialogue de configuration d'un [plugin](/docs/fr/plugins/overview)                                                                                                                                                   | Plugins and skills                 | User or managed         |
| [`pluginSuggestionMarketplaces`](#pluginsuggestionmarketplaces)                                       | Choisissez les [marchés](/docs/fr/plugins/org#restrict-what-users-can-install) qui peuvent afficher les suggestions d'installation de plugins dans `/plugin`                                                                                                                  | Plugins and skills                 | Managed                 |
| [`pluginTrustMessage`](#plugintrustmessage)                                                           | Ajoutez votre propre texte à l'avertissement de confiance du [plugin](/docs/fr/plugins/overview)                                                                                                                                                                              | Plugins and skills                 | Managed                 |
| [`policyHelper`](#policyhelper)                                                                       | Exécutez un exécutable qui calcule les [paramètres gérés](/docs/fr/managed-settings#compute-the-policy-with-a-helper-program) au démarrage                                                                                                                                    | Enterprise and managed settings    | Managed                 |
| [`policyHelper.path`](#policyhelper-path)                                                             | Nommez l'[exécutable d'aide](/docs/fr/managed-settings#compute-the-policy-with-a-helper-program) que Claude Code exécute                                                                                                                                                      | Enterprise and managed settings    | Managed                 |
| [`policyHelper.refreshIntervalMs`](#policyhelper-refreshintervalms)                                   | Réexécutez l'[aide](/docs/fr/managed-settings#compute-the-policy-with-a-helper-program) en arrière-plan à un intervalle                                                                                                                                                       | Enterprise and managed settings    | Managed                 |
| [`policyHelper.timeoutMs`](#policyhelper-timeoutms)                                                   | Définissez le temps que Claude Code attend pour l'[aide](/docs/fr/managed-settings#compute-the-policy-with-a-helper-program)                                                                                                                                                  | Enterprise and managed settings    | Managed                 |
| [`preferredNotifChannel`](#preferrednotifchannel)                                                     | Choisissez une [sonnerie de terminal ou une notification de bureau](/docs/fr/terminal-config#get-a-terminal-bell-or-notification) pour l'achèvement des tâches                                                                                                                | Remote, desktop, and notifications | Any file                |
| [`prefersReducedMotion`](#prefersreducedmotion)                                                       | [Réduisez ou désactivez](/docs/fr/accessibility#accessibility-settings) les animations de spinner, shimmer et flash                                                                                                                                                           | Interface and terminal             | Any file                |
| [`processWrapper`](#processwrapper)                                                                   | Exécutez les processus d'arrière-plan de Claude Code via un [lanceur d'entreprise](/docs/fr/corporate-launcher) sur macOS et Linux                                                                                                                                            | Agents, sessions, and worktrees    | User or managed         |
| [`promptCacheTtl`](#promptcachettl)                                                                   | Choisissez la [durée de vie du cache d'invite](/docs/fr/prompt-caching#cache-lifetime) pour la conversation principale                                                                                                                                                        | Model and responses                | Any file                |
| [`promptSuggestionEnabled`](#promptsuggestionenabled)                                                 | Masquez les [suggestions d'invite](/docs/fr/interactive-mode#prompt-suggestions) grisées dans la zone d'entrée                                                                                                                                                                | Interface and terminal             | Any file                |
| [`prUrlTemplate`](#prurltemplate)                                                                     | Pointez les liens PR vers un outil d'examen de code interne au lieu de github.com                                                                                                                                                                                        | Git and attribution                | Any file                |
| [`remote.defaultEnvironmentId`](#remote-defaultenvironmentid)                                         | Choisissez l'[environnement cloud](/docs/fr/cloud-environments) par défaut pour `claude --cloud` ; un ID `ccpool_` auto-hébergé est en lecture seule à partir des paramètres utilisateur et gérés et `--settings`                                                             | Remote, desktop, and notifications | Any file                |
| [`remoteControlAtStartup`](#remotecontrolatstartup)                                                   | Connectez [Remote Control](/docs/fr/remote-control#enable-remote-control-for-all-sessions) automatiquement quand une session démarre                                                                                                                                          | Remote, desktop, and notifications | Any file                |
| [`requiredMaximumVersion`](#requiredmaximumversion)                                                   | [Refusez de démarrer](/docs/fr/setup#pin-a-minimum-version) sur une version plus récente que celle que votre organisation autorise                                                                                                                                            | Updates and versioning             | Managed                 |
| [`requiredMinimumVersion`](#requiredminimumversion)                                                   | [Refusez de démarrer](/docs/fr/setup#pin-a-minimum-version) sur une version plus ancienne que celle que votre organisation exige                                                                                                                                              | Updates and versioning             | Managed                 |
| [`respectGitignore`](#respectgitignore)                                                               | Gardez les fichiers ignorés par git hors du [sélecteur de fichier `@`](/docs/fr/interactive-mode#quick-commands)                                                                                                                                                              | Interface and terminal             | Any file                |
| [`respondToBashCommands`](#respondtobashcommands)                                                     | Arrêtez Claude de répondre après l'exécution d'une [commande shell `!`](/docs/fr/interactive-mode#shell-mode-with-prefix)                                                                                                                                                     | Interface and terminal             | Any file                |
| [`sandbox`](#sandbox)                                                                                 | [Isolez les commandes Bash](/docs/fr/sandboxing) de votre système de fichiers et réseau sur macOS, Linux et WSL2                                                                                                                                                              | Sandbox settings                   | Any file                |
| [`sandbox.allowAppleEvents`](#sandbox-allowappleevents)                                               | Laissez les [commandes en sandbox](/docs/fr/sandboxing) envoyer des Apple Events sur macOS                                                                                                                                                                                    | Sandbox settings                   | User or managed         |
| [`sandbox.allowUnsandboxedCommands`](#sandbox-allowunsandboxedcommands)                               | Laissez Claude réessayer une commande bloquée en dehors du [sandbox](/docs/fr/sandboxing#the-unsandboxed-retry-escape-hatch), ou l'interdire                                                                                                                                  | Sandbox settings                   | Any file                |
| [`sandbox.autoAllowBashIfSandboxed`](#sandbox-autoallowbashifsandboxed)                               | Exécutez les [commandes en sandbox](/docs/fr/sandboxing#auto-allow-mode) sans invite de permission                                                                                                                                                                            | Sandbox settings                   | Any file                |
| [`sandbox.bwrapPath`](#sandbox-bwrappath)                                                             | Pointez le [sandbox](/docs/fr/sandboxing) vers un binaire bubblewrap en dehors de `PATH`                                                                                                                                                                                      | Sandbox settings                   | Managed                 |
| [`sandbox.credentials`](#sandbox-credentials)                                                         | Masquez ou masquez les fichiers et variables d'identifiants à l'intérieur du [sandbox](/docs/fr/sandboxing#protect-credentials)                                                                                                                                               | Sandbox settings                   | Any file                |
| [`sandbox.credentials.allowPlaintextInject`](#sandbox-credentials-allowplaintextinject)               | Laissez les [identifiants masqués](/docs/fr/sandboxing#mask-credentials) atteindre les services HTTP simples sur les réseaux de test de confiance                                                                                                                             | Sandbox settings                   | User or managed         |
| [`sandbox.credentials.awsPairs`](#sandbox-credentials-awspairs)                                       | Liez les variables de clé AWS nommées personnalisées en un identifiant pour la [re-signature](/docs/fr/sandboxing#re-sign-aws-requests)                                                                                                                                       | Sandbox settings                   | User or managed         |
| [`sandbox.credentials.envVars`](#sandbox-credentials-envvars)                                         | Désactivez ou masquez une variable d'environnement à l'intérieur du [sandbox](/docs/fr/sandboxing#mask-environment-variables)                                                                                                                                                 | Sandbox settings                   | Any file                |
| [`sandbox.credentials.files`](#sandbox-credentials-files)                                             | Bloquez ou masquez les lectures d'un fichier d'identifiants à l'intérieur du [sandbox](/docs/fr/sandboxing#mask-credential-files)                                                                                                                                             | Sandbox settings                   | Any file                |
| [`sandbox.credentials.sigv4`](#sandbox-credentials-sigv4)                                             | Choisissez si le streaming, les demandes présignées ou [les demandes AWS SigV4A](/docs/fr/sandboxing#re-sign-aws-requests) échouent ou passent                                                                                                                                | Sandbox settings                   | User or managed         |
| [`sandbox.enabled`](#sandbox-enabled)                                                                 | Activez le [sandboxing Bash](/docs/fr/sandboxing#get-started) sur macOS, Linux et WSL2                                                                                                                                                                                        | Sandbox settings                   | Any file                |
| [`sandbox.enableWeakerNestedSandbox`](#sandbox-enableweakernestedsandbox)                             | Exécutez le [sandbox](/docs/fr/sandboxing) Linux à l'intérieur d'un conteneur sans privilèges                                                                                                                                                                                 | Sandbox settings                   | Any file                |
| [`sandbox.enableWeakerNetworkIsolation`](#sandbox-enableweakernetworkisolation)                       | Laissez `gh`, `gcloud` et `terraform` vérifier TLS derrière un proxy MITM à l'intérieur du [sandbox](/docs/fr/sandboxing#troubleshooting) sur macOS                                                                                                                           | Sandbox settings                   | Any file                |
| [`sandbox.excludedCommands`](#sandbox-excludedcommands)                                               | Nommez les commandes que Claude Code peut exécuter en dehors du [sandbox](/docs/fr/sandboxing)                                                                                                                                                                                | Sandbox settings                   | Any file                |
| [`sandbox.failIfUnavailable`](#sandbox-failifunavailable)                                             | Refusez de démarrer quand le [sandbox](/docs/fr/sandboxing) ne peut pas, au lieu de s'exécuter sans sandbox                                                                                                                                                                   | Sandbox settings                   | Any file                |
| [`sandbox.filesystem`](#sandbox-filesystem)                                                           | Contrôlez les chemins que les [commandes en sandbox](/docs/fr/sandboxing#filesystem-isolation) peuvent lire et écrire                                                                                                                                                         | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly)       | Empêchez les développeurs de rouvrir les [chemins de lecture que votre organisation a bloqués](/docs/fr/sandboxing#keep-developers-from-widening-the-policy)                                                                                                                  | Sandbox settings                   | Managed                 |
| [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread)                                       | Rouvrez la lecture à l'intérieur d'une région que [`denyRead`](#sandbox-filesystem-denyread) bloque                                                                                                                                                                      | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.allowWrite`](#sandbox-filesystem-allowwrite)                                     | Ajoutez les chemins que les [commandes en sandbox](/docs/fr/sandboxing) peuvent écrire                                                                                                                                                                                        | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.denyRead`](#sandbox-filesystem-denyread)                                         | Bloquez les [commandes en sandbox](/docs/fr/sandboxing) de lire des chemins spécifiques                                                                                                                                                                                       | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.denyWrite`](#sandbox-filesystem-denywrite)                                       | Bloquez les [commandes en sandbox](/docs/fr/sandboxing) d'écrire dans des chemins spécifiques                                                                                                                                                                                 | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.disabled`](#sandbox-filesystem-disabled)                                         | [Désactivez l'isolation du système de fichiers](/docs/fr/sandboxing#disable-filesystem-isolation) tout en conservant l'isolation du réseau                                                                                                                                    | Sandbox settings                   | User or managed         |
| [`sandbox.ignoreViolations`](#sandbox-ignoreviolations)                                               | Silence les rapports de violation pour les chemins qu'une commande est censée sonder                                                                                                                                                                                     | Sandbox settings                   | Any file                |
| [`sandbox.network`](#sandbox-network)                                                                 | Contrôlez les hôtes, ports et sockets que les [commandes en sandbox](/docs/fr/sandboxing#network-isolation) atteignent                                                                                                                                                        | Sandbox settings                   | Any file                |
| [`sandbox.network.allowAllUnixSockets`](#sandbox-network-allowallunixsockets)                         | Laissez les [commandes en sandbox](/docs/fr/sandboxing) se connecter à chaque socket Unix                                                                                                                                                                                     | Sandbox settings                   | Any file                |
| [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains)                                   | Pré-autorisez les domaines pour que les [commandes en sandbox](/docs/fr/sandboxing) ne les demandent pas                                                                                                                                                                      | Sandbox settings                   | Any file                |
| [`sandbox.network.allowLocalBinding`](#sandbox-network-allowlocalbinding)                             | Laissez les [commandes en sandbox](/docs/fr/sandboxing) se lier aux ports localhost sur macOS                                                                                                                                                                                 | Sandbox settings                   | Any file                |
| [`sandbox.network.allowMachLookup`](#sandbox-network-allowmachlookup)                                 | Laissez les outils macOS [en sandbox](/docs/fr/sandboxing) comme le simulateur iOS ou Playwright atteindre leurs services XPC                                                                                                                                                 | Sandbox settings                   | Any file                |
| [`sandbox.network.allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly)                 | Verrouillez la liste d'autorisation du réseau aux [paramètres gérés](/docs/fr/sandboxing#keep-developers-from-widening-the-policy)                                                                                                                                            | Sandbox settings                   | Managed                 |
| [`sandbox.network.allowUnixSockets`](#sandbox-network-allowunixsockets)                               | Listez les chemins de socket Unix que les [commandes en sandbox](/docs/fr/sandboxing) peuvent utiliser sur macOS                                                                                                                                                              | Sandbox settings                   | Any file                |
| [`sandbox.network.deniedDomains`](#sandbox-network-denieddomains)                                     | Bloquez les domaines pour les [commandes en sandbox](/docs/fr/sandboxing), même à l'intérieur d'un caractère générique autorisé                                                                                                                                               | Sandbox settings                   | Any file                |
| [`sandbox.network.httpProxyPort`](#sandbox-network-httpproxyport)                                     | Routez le trafic HTTP du [sandbox](/docs/fr/sandboxing#custom-proxy-configuration) via votre propre proxy                                                                                                                                                                     | Sandbox settings                   | Any file                |
| [`sandbox.network.socksProxyPort`](#sandbox-network-socksproxyport)                                   | Routez le trafic SOCKS du [sandbox](/docs/fr/sandboxing#custom-proxy-configuration) via votre propre proxy                                                                                                                                                                    | Sandbox settings                   | Any file                |
| [`sandbox.network.strictAllowlist`](#sandbox-network-strictallowlist)                                 | Refusez les hôtes en dehors de la [liste d'autorisation](/docs/fr/sandboxing#network-isolation) au lieu de demander                                                                                                                                                           | Sandbox settings                   | User or managed         |
| [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate)                                       | Faites en sorte que le [sandbox](/docs/fr/sandboxing#network-isolation) proxy termine TLS pour qu'il puisse lire les demandes HTTPS                                                                                                                                           | Sandbox settings                   | User or managed         |
| [`sandbox.ripgrep`](#sandbox-ripgrep)                                                                 | Utilisez votre propre binaire ripgrep à l'intérieur du [sandbox](/docs/fr/sandboxing)                                                                                                                                                                                         | Sandbox settings                   | User or managed         |
| [`sandbox.socatPath`](#sandbox-socatpath)                                                             | Pointez le proxy du [sandbox](/docs/fr/sandboxing) vers un binaire `socat` en dehors de `PATH`                                                                                                                                                                                | Sandbox settings                   | Managed                 |
| [`showClearContextOnPlanAccept`](#showclearcontextonplanaccept)                                       | Affichez une option « effacer le contexte » sur l'[écran d'acceptation du plan](/docs/fr/permission-modes#review-and-approve-a-plan)                                                                                                                                          | Interface and terminal             | Any file                |
| [`showThinkingSummaries`](#showthinkingsummaries)                                                     | Consultez les résumés de la [réflexion](/docs/fr/model-config#extended-thinking) de Claude au lieu d'un stub réduit                                                                                                                                                           | Model and responses                | Any file                |
| [`showTurnDuration`](#showturnduration)                                                               | Masquez la durée « Cooked for » après chaque réponse                                                                                                                                                                                                                     | Interface and terminal             | Any file                |
| [`skillListingBudgetFraction`](#skilllistingbudgetfraction)                                           | Réservez plus ou moins de contexte pour l'[énumération des compétences](/docs/fr/skills#skill-descriptions-are-cut-short)                                                                                                                                                     | Memory and context                 | Any file                |
| [`skillListingMaxDescChars`](#skilllistingmaxdescchars)                                               | Limitez la longueur de la description de chaque compétence dans l'[énumération des compétences](/docs/fr/skills#skill-descriptions-are-cut-short)                                                                                                                             | Memory and context                 | Any file                |
| [`skillOverrides`](#skilloverrides)                                                                   | [Masquez ou réduisez une compétence](/docs/fr/skills#override-skill-visibility-from-settings) sans éditer son SKILL.md                                                                                                                                                        | Plugins and skills                 | Any file                |
| [`skipAutoPermissionPrompt`](#skipautopermissionprompt)                                               | Ignorez l'avis unique que Claude Code affiche quand vous entrez d'abord dans le [mode automatique](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) vous-même plutôt que via la valeur par défaut intégrée                                                         | Permission settings                | User or managed         |
| [`skipDangerousModePermissionPrompt`](#skipdangerousmodepermissionprompt)                             | Ignorez la boîte de dialogue de confirmation avant le [mode bypassPermissions](/docs/fr/permission-modes#skip-all-checks-with-bypasspermissions-mode)                                                                                                                         | Permission settings                | User, local, or managed |
| [`skipWebFetchPreflight`](#skipwebfetchpreflight)                                                     | Ignorez la [vérification du nom d'hôte WebFetch](/docs/fr/tools-reference#webfetch-tool-behavior) quand Anthropic est inaccessible                                                                                                                                            | Privacy and telemetry              | Any file                |
| [`spellcheck`](#spellcheck)                                                                           | Soulignez les mots mal orthographiés dans l'entrée d'invite avec un [vérificateur d'orthographe](/docs/fr/interactive-mode#check-spelling-as-you-type) que vous installez                                                                                                     | Interface and terminal             | User or managed         |
| [`spinnerTipsEnabled`](#spinnertipsenabled)                                                           | Masquez les conseils dans le spinner pendant que Claude travaille                                                                                                                                                                                                        | Interface and terminal             | Any file                |
| [`spinnerTipsOverride`](#spinnertipsoverride)                                                         | Ajoutez vos propres conseils à la rotation du spinner, ou remplacez les conseils intégrés                                                                                                                                                                                | Interface and terminal             | Any file                |
| [`spinnerVerbs`](#spinnerverbs)                                                                       | Ajoutez ou remplacez les verbes affichés pendant l'exécution d'un tour                                                                                                                                                                                                   | Interface and terminal             | Any file                |
| [`sshConfigs`](#sshconfigs)                                                                           | Ajoutez les [connexions SSH](/docs/fr/desktop#pre-configure-ssh-connections-for-your-team) à la liste déroulante de l'environnement Bureau                                                                                                                                    | Remote, desktop, and notifications | User or managed         |
| [`sshHostAllowlist`](#sshhostallowlist)                                                               | Limitez les hôtes que les [sessions SSH Bureau](/docs/fr/desktop#restrict-which-ssh-hosts-users-can-connect-to) peuvent atteindre                                                                                                                                             | Remote, desktop, and notifications | Managed                 |
| [`statusLine`](#statusline)                                                                           | Exécutez votre propre commande pour rendre une [ligne d'état](/docs/fr/statusline) sous l'invite                                                                                                                                                                              | Interface and terminal             | Any file                |
| [`strictKnownMarketplaces`](#strictknownmarketplaces)                                                 | Liste d'autorisation des sources du [marché](/docs/fr/plugins/overview) que les utilisateurs peuvent ajouter et installer                                                                                                                                                     | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)                                     | Bloquez les [compétences](/docs/fr/skills), les [agents](/docs/fr/sub-agents), les [hooks](/docs/fr/hooks) et les [serveurs MCP](/docs/fr/mcp) des sources utilisateur et projet                                                                                                             | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization.agents`](#strictpluginonlycustomization-agents)                       | Verrouillez les [agents](/docs/fr/sub-agents) aux sources de plugin et gérées                                                                                                                                                                                                 | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization.hooks`](#strictpluginonlycustomization-hooks)                         | Verrouillez les [hooks](/docs/fr/hooks) aux sources de plugin et gérées                                                                                                                                                                                                       | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization.mcp`](#strictpluginonlycustomization-mcp)                             | Verrouillez les [serveurs MCP](/docs/fr/mcp) aux sources de plugin et gérées                                                                                                                                                                                                  | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization.skills`](#strictpluginonlycustomization-skills)                       | Verrouillez les [compétences](/docs/fr/skills) aux sources de plugin et gérées                                                                                                                                                                                                | Plugins and skills                 | Managed                 |
| [`subagentPromptCacheTtl`](#subagentpromptcachettl)                                                   | Choisissez la [durée de vie du cache d'invite](/docs/fr/prompt-caching#cache-lifetime) pour les sous-agents et autres demandes en dehors de la conversation principale                                                                                                        | Model and responses                | Any file                |
| [`subagentStatusLine`](#subagentstatusline)                                                           | Réécrivez les lignes dans l'[affichage des tâches du sous-agent](/docs/fr/sub-agents) avec votre propre commande                                                                                                                                                              | Interface and terminal             | Any file                |
| [`switchModelsOnFlag`](#switchmodelsonflag)                                                           | Changez les modèles automatiquement ou mettez en pause quand un [classificateur de sécurité](/docs/fr/model-config#ask-before-switching) signale une demande                                                                                                                  | Model and responses                | Any file                |
| [`syncClaudeAiPlugins`](#syncclaudeaiplugins)                                                         | Arrêtez de charger les [plugins activés sur votre compte claude.ai](/docs/fr/plugins/loading#synced-plugins) et arrêtez de télécharger les nouveaux                                                                                                                           | Plugins and skills                 | User, local, or managed |
| [`syncClaudeAiSkills`](#syncclaudeaiskills)                                                           | Arrêtez de charger les [compétences activées sur votre compte claude.ai](/docs/fr/skills#how-synced-skills-behave) et arrêtez de télécharger les nouvelles                                                                                                                    | Plugins and skills                 | User, local, or managed |
| [`syntaxHighlightingDisabled`](#syntaxhighlightingdisabled)                                           | Désactivez la coloration syntaxique dans les diffs et les blocs de code                                                                                                                                                                                                  | Interface and terminal             | Any file                |
| [`taskOutputMaxChars`](#taskoutputmaxchars)                                                           | Supprimé dans v2.1.277, ainsi que l'outil `TaskOutput` qu'il dimensionnait                                                                                                                                                                                               | Memory and context                 | Any file                |
| [`teammateDefaultModel`](#teammatedefaultmodel)                                                       | Supprimé dans v2.1.234 ; voir [Spécifier les coéquipiers et les modèles](/docs/fr/agent-teams#specify-teammates-and-models) pour savoir comment Claude Code choisit le modèle d'un coéquipier                                                                                 | Global config settings             | Global config           |
| [`teammateMode`](#teammatemode)                                                                       | Choisissez comment les [coéquipiers de l'équipe agent affichent](/docs/fr/agent-teams#choose-a-display-mode)                                                                                                                                                                  | Agents, sessions, and worktrees    | Any file                |
| [`terminalProgressBarEnabled`](#terminalprogressbarenabled)                                           | Masquez la barre de progression du terminal dans les terminaux qui la supportent                                                                                                                                                                                         | Interface and terminal             | Any file                |
| [`terminalTitleFromRename`](#terminaltitlefromrename)                                                 | Arrêtez [`/rename`](/docs/fr/sessions#name-your-sessions) et `--name` de modifier le titre de l'onglet du terminal                                                                                                                                                            | Interface and terminal             | Any file                |
| [`theme`](#theme)                                                                                     | Choisissez le [thème de couleur](/docs/fr/terminal-config#match-the-color-theme) de l'interface, intégré ou personnalisé                                                                                                                                                      | Interface and terminal             | Any file                |
| [`timeFormat`](#timeformat)                                                                           | Affichez les heures dans l'interface sur une horloge 12 heures ou 24 heures, en UTC, ou avec un motif strftime                                                                                                                                                           | Interface and terminal             | Any file                |
| [`timeZone`](#timezone)                                                                               | Affichez les heures dans l'interface dans un fuseau horaire autre que celui de votre système                                                                                                                                                                             | Interface and terminal             | Any file                |
| [`tui`](#tui)                                                                                         | Choisissez le rendu [plein écran](/docs/fr/fullscreen) ou terminal classique                                                                                                                                                                                                  | Interface and terminal             | Any file                |
| [`ultracode`](#ultracode)                                                                             | Faites en sorte que Claude planifie un [flux de travail](/docs/fr/workflows#let-claude-decide-with-ultracode) pour chaque tâche substantielle sans être demandé                                                                                                               | Model and responses                | Any file                |
| [`useAutoModeDuringPlan`](#useautomodeduringplan)                                                     | Laissez le classificateur du [mode automatique](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) examiner les commandes shell en [mode plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode) ; définissez `false` pour obtenir des invites à la place | Permission settings                | User, local, or managed |
| [`verbose`](#verbose)                                                                                 | Affichez la [sortie complète de l'outil](/docs/fr/cli-reference#cli-flags) au lieu des résumés tronqués ; `viewMode` prend la priorité quand les deux sont définis                                                                                                            | Interface and terminal             | Any file                |
| [`viewMode`](#viewmode)                                                                               | Commencez chaque session dans la [vue par défaut, détaillée ou focus](/docs/fr/cli-reference#cli-flags)                                                                                                                                                                       | Interface and terminal             | Any file                |
| [`vimInsertModeRemaps`](#viminsertmoderemaps)                                                         | Mappez une [séquence en mode INSERT](/docs/fr/interactive-mode#remap-insert-mode-key-sequences) à deux touches comme `jj` à Échap                                                                                                                                             | Interface and terminal             | User or managed         |
| [`voice`](#voice)                                                                                     | Activez la [dictée vocale](/docs/fr/voice-dictation) et choisissez le mode maintien ou appui                                                                                                                                                                                  | Interface and terminal             | Any file                |
| [`voiceEnabled`](#voiceenabled)                                                                       | Activez la [dictée vocale](/docs/fr/voice-dictation) avec la forme à clé unique plus ancienne                                                                                                                                                                                 | Interface and terminal             | Any file                |
| [`wheelScrollAccelerationEnabled`](#wheelscrollaccelerationenabled)                                   | Désactivez l'[accélération de la molette de la souris](/docs/fr/fullscreen#mouse-wheel-scrolling) dans le rendu en plein écran                                                                                                                                                | Interface and terminal             | Any file                |
| [`workflowKeywordTriggerEnabled`](#workflowkeywordtriggerenabled)                                     | Laissez le mot `ultracode` dans une invite démarrer un [flux de travail](/docs/fr/workflows) ; définissez `false` pour le taper sans en démarrer un                                                                                                                           | Hooks and automation               | Any file                |
| [`workflowSizeGuideline`](#workflowsizeguideline)                                                     | Définissez le nombre d'agents que Claude vise dans les [flux de travail dynamiques](/docs/fr/workflows)                                                                                                                                                                       | Hooks and automation               | Any file                |
| [`worktree`](#worktree)                                                                               | Configurez la façon dont Claude Code crée les [worktrees](/docs/fr/worktrees) git                                                                                                                                                                                             | Agents, sessions, and worktrees    | Any file                |
| [`worktree.baseRef`](#worktree-baseref)                                                               | Branchement des nouveaux [worktrees](/docs/fr/worktrees) à partir de la branche par défaut distante ou de votre HEAD local                                                                                                                                                    | Agents, sessions, and worktrees    | Any file                |
| [`worktree.bgIsolation`](#worktree-bgisolation)                                                       | Laissez les sessions d'arrière-plan éditer la copie de travail sans [worktree](/docs/fr/worktrees)                                                                                                                                                                            | Agents, sessions, and worktrees    | Any file                |
| [`worktree.sparsePaths`](#worktree-sparsepaths)                                                       | Vérifiez uniquement les répertoires dont vous avez besoin dans chaque [worktree](/docs/fr/worktrees)                                                                                                                                                                          | Agents, sessions, and worktrees    | Any file                |
| [`worktree.symlinkDirectories`](#worktree-symlinkdirectories)                                         | Créez des liens symboliques vers les grands répertoires dans chaque [worktree](/docs/fr/worktrees) au lieu de les dupliquer                                                                                                                                                   | Agents, sessions, and worktrees    | Any file                |
| [`wslInheritsWindowsSettings`](#wslinheritswindowssettings)                                           | Faites en sorte que WSL lise les [paramètres gérés](/docs/fr/managed-settings) à partir de la chaîne de politique Windows                                                                                                                                                     | Enterprise and managed settings    | Managed                 |

<h2 id="model-and-responses">
  Modèle et réponses
</h2>

Choisissez les modèles que Claude Code utilise et comment il répond. Pour savoir comment ces paramètres interagissent avec la commande `/model` et les variables d'environnement, consultez [Configuration du modèle](/docs/fr/model-config).

<h3 id="advisormodel">
  `advisorModel`
</h3>

Choisissez quel modèle répond quand Claude appelle l'[outil advisor](/docs/fr/advisor) côté serveur. Laissez-le non défini pour désactiver l'advisor. L'advisor doit être au moins aussi capable que votre modèle principal. Consultez [Choisir un modèle advisor](/docs/fr/advisor#choose-an-advisor-model) pour connaître les appairements acceptés et ce qui se passe quand vous en choisissez un qui n'est pas accepté.

Vous n'éditez généralement pas cette clé à la main. Exécutez `/advisor` pour ouvrir un sélecteur qui affiche le choix actuel, les modèles qui peuvent conseiller, et **Pas d'advisor**. Claude Code enregistre votre choix dans cette clé dans `~/.claude/settings.json`. Si vous choisissez à partir d'un client [Remote Control](/docs/fr/remote-control) ou dans une session attachée à un worker distant, le choix s'applique à cette session uniquement et ne modifie pas cette clé.

Si votre compte nécessite le [consentement usage-credits](/docs/fr/advisor#fable-advisor-and-usage-credits), acceptez-le d'abord en exécutant `/model fable`. Jusqu'à ce que vous le fassiez, choisir Fable dans `/advisor` n'enregistre rien et Claude Code vous dit d'exécuter `/model fable` d'abord.

* **Portée** : [`Tout fichier`](#scopes)
* **Type** : chaîne, l'un des alias `"fable"`, `"opus"`, ou `"sonnet"`, qui se résolvent à la version par défaut actuelle de Claude Code pour cette famille de modèles, ou un ID de modèle complet tel que `"claude-opus-5-5"`
* **Par défaut** : non défini, donc l'advisor est désactivé
* **Remplacements par session** : `--advisor` a la priorité sur cette clé pour une session. [`CLAUDE_CODE_DISABLE_ADVISOR_TOOL`](/docs/fr/env-vars) désactive l'advisor, et cette clé ne peut pas le réactiver

```json settings.json theme={null}
{
  "advisorModel": "opus"
}
```

La clé n'a aucun effet sur les fournisseurs où l'advisor [n'est pas disponible](/docs/fr/advisor#requirements), comme Amazon Bedrock et Claude Platform sur AWS. `"fable"` nécessite l'[accès à Fable](/docs/fr/advisor#choose-an-advisor-model).

<h3 id="alwaysthinkingenabled">
  `alwaysThinkingEnabled`
</h3>

Désactivez la [réflexion étendue](/docs/fr/model-config#extended-thinking) pour chaque session en définissant ceci à `false`. La réflexion est activée par défaut, donc `true` ne change rien. La plupart des gens définissent ceci via `/config` plutôt qu'en éditant le fichier.

Sur les modèles qui réfléchissent toujours, comme Opus 5.5 et les modèles Fable, `false` n'a aucun effet. Sur les [fournisseurs tiers](/docs/fr/third-party-integrations), Claude Code omet le paramètre `thinking` au lieu de désactiver la réflexion, donc les modèles de raisonnement adaptatif peuvent toujours réfléchir. Avec la réflexion désactivée sur l'API Anthropic, Claude Code envoie l'effort `high` au lieu d'un niveau supérieur aux modèles qu'il sait [ne pas accepter cette combinaison](/docs/fr/errors#effort-isnt-available-with-thinking-turned-off), comme Opus 5.

* **Portée** : [`Tout fichier`](#scopes)
* **Type** : Booléen
  * `true` : aucun effet ; la réflexion est déjà activée
  * `false` : Claude Code désactive la réflexion étendue pour chaque session
* **Par défaut** : non défini, donc la réflexion est activée pour les modèles qui la supportent
* **Remplacements par session** : [`MAX_THINKING_TOKENS`](/docs/fr/env-vars) a la priorité sur cette clé pour une session : `0` désactive la réflexion, sous les mêmes limites de modèle et de fournisseur que `false`, et une valeur positive active la réflexion même quand cette clé est `false`. Sur les modèles de raisonnement adaptatif, le nombre lui-même est ignoré

```json settings.json theme={null}
{
  "alwaysThinkingEnabled": false
}
```

<h3 id="availablemodels">
  `availableModels`
</h3>

Limitez les modèles que les gens peuvent sélectionner pour la session principale, les [sous-agents](/docs/fr/sub-agents), les [compétences](/docs/fr/skills), et l'[advisor](/docs/fr/advisor). Une liste gérée contraint `/model`, `--model`, et la clé `model` dans les fichiers propres d'un développeur ; un modèle en dehors ne peut pas être sélectionné. En soi, cela ne touche pas l'option Par défaut ; associez-le à [`enforceAvailableModels`](#enforceavailablemodels) pour cela.

* **Portée** : [`Tout fichier`](#scopes). Déployez-le dans les paramètres gérés pour l'appliquer à une organisation.
* **Type** : tableau d'alias de modèles ou d'ID
* **Par défaut** : non défini, donc chaque modèle est disponible

Cet exemple permet aux gens de sélectionner uniquement les modèles Sonnet et Haiku :

```json settings.json theme={null}
{
  "availableModels": ["sonnet", "haiku"]
}
```

Consultez [Restreindre la sélection du modèle](/docs/fr/model-config#restrict-model-selection).

<h3 id="effortlevel">
  `effortLevel`
</h3>

Définissez un [niveau d'effort](/docs/fr/model-config#adjust-effort-level) par défaut pour les modèles pour lesquels vous n'avez pas enregistré de niveau. Les niveaux inférieurs sont plus rapides et moins chers sur les tâches simples, et les niveaux supérieurs raisonnent plus profondément sur les problèmes complexes.

Quand vous exécutez `/effort low`, `medium`, `high`, ou `xhigh` dans une session interactive sur votre machine, Claude Code enregistre le niveau pour le modèle actif sous [`modelSettings`](#modelsettings) plutôt que d'écrire cette clé. Avant la v2.1.251, `/effort` écrivait cette clé.

Dans le même fichier de paramètres, Claude Code utilise le niveau enregistré d'un modèle plutôt que cette clé. [`modelSettings`](#modelsettings) indique la précédence entre fichiers.

Dans une session attachée à un worker distant, dans une exécution `-p`, et dans l'Agent SDK, `/effort` s'applique à cette session uniquement. [Ajuster le niveau d'effort](/docs/fr/model-config#adjust-effort-level) énumère les choix interactifs qui s'appliquent également à cette session uniquement. Le message que `/effort` affiche indique ce qui s'est passé.

* **Portée** : [`Tout fichier`](#scopes)
* **Type** : chaîne, l'une de :
  * `"low"` : le moins de raisonnement, pour les tâches courtes, délimitées, sensibles à la latence qui ne sont pas sensibles à l'intelligence
  * `"medium"` : réduit l'utilisation des tokens pour le travail sensible aux coûts qui peut faire des compromis sur l'intelligence
  * `"high"` : équilibre l'utilisation des tokens et l'intelligence
  * `"xhigh"` : raisonnement plus profond avec une dépense de tokens plus élevée
* **Par défaut** : non défini
* **Remplacements par session** : `--effort` a la priorité sur cette clé pour une session, et [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/fr/env-vars) a la priorité sur les deux

```json settings.json theme={null}
{
  "effortLevel": "xhigh"
}
```

Dans votre fichier de paramètres utilisateur, `~/.claude/settings.json`, cette clé est la forme plus ancienne que `/effort` écrivait avant d'enregistrer les niveaux par modèle, et elle continue de s'appliquer où elle s'appliquait avant, sur Opus 5, Fable 5.1, et les modèles antérieurs. Opus 5.5 et les modèles publiés après l'ignorent et commencent à leur propre défaut jusqu'à ce que vous enregistriez un niveau pour eux, que `/effort` écrit sous [`modelSettings`](#modelsettings). Dans les paramètres de projet, locaux et gérés, et avec `--settings`, cette clé s'applique à chaque modèle.

<h3 id="enforceavailablemodels">
  `enforceAvailableModels`
</h3>

Le sélecteur `/model` a une option **Par défaut** qui se résout à votre [modèle par défaut de l'organisation](/docs/fr/model-config#organization-default-model) quand l'une s'applique, et sinon au défaut de votre type de compte. Une liste d'autorisation [`availableModels`](#availablemodels) limite les modèles que vous pouvez nommer, mais en soi elle laisse **Par défaut** seul, donc **Par défaut** peut toujours se résoudre à un modèle en dehors de la liste. Cette clé comble cette lacune. Nécessite Claude Code v2.1.175 ou ultérieur.

Quand votre organisation déploie des paramètres gérés, Claude Code lit cette clé à partir de la source gérée uniquement et l'ignore dans vos autres fichiers.

* **Portée** : [`Tout fichier`](#scopes)
* **Type** : Booléen
  * `true` : quand **Par défaut** se résoudrait à un modèle en dehors de `availableModels`, Claude Code le résout au premier modèle disponible de la liste
  * `false` : **Par défaut** se résout comme d'habitude, même à un modèle en dehors de `availableModels`
* **Par défaut** : `false`

Cet exemple restreint les sélections nommées aux modèles Sonnet et Haiku et fait que **Par défaut** se résout au premier d'entre eux qui est disponible :

```json settings.json theme={null}
{
  "availableModels": ["sonnet", "haiku"],
  "enforceAvailableModels": true
}
```

Cette clé n'a aucun effet quand `availableModels` est non défini ou vide. Consultez [Appliquer la liste d'autorisation au modèle Par défaut](/docs/fr/model-config#enforce-the-allowlist-for-the-default-model). Nécessite Claude Code v2.1.175 ou ultérieur.

<h3 id="fallbackmodel">
  `fallbackModel`
</h3>

Nommez les modèles de secours que Claude Code doit essayer, dans l'ordre, quand votre modèle principal est surchargé ou indisponible. Claude Code bascule vers le modèle disponible suivant dans la chaîne pour le reste du tour et affiche un avis. Sans chaîne, Claude Code réessaie le même modèle puis affiche l'erreur du serveur, et vous réessayez ou changez de modèles vous-même.

Un changement signifie un tour avec un [cache de prompt](/docs/fr/prompt-caching#switching-models) froid sur le modèle de secours ; votre message suivant réessaie d'abord le modèle principal.

* **Portée** : [`Tout fichier`](#scopes)
* **Type** : tableau d'alias de modèles ou d'ID ; `"default"` se développe au modèle par défaut
* **Par défaut** : non défini, donc une demande échouée n'est pas réessayée sur un autre modèle
* **Remplacements par session** : `--fallback-model` a la priorité sur cette clé pour une session

Cet exemple essaie d'abord Sonnet 5, puis Haiku 4.5, quand votre modèle principal échoue :

```json settings.json theme={null}
{
  "fallbackModel": ["claude-sonnet-5", "claude-haiku-4-5"]
}
```

Contrairement à la plupart des paramètres de tableau, cette clé ne fusionne pas entre les fichiers de paramètres : le fichier de plus haute priorité qui la définit fournit la chaîne entière. Si votre fichier de projet définit `["claude-sonnet-5"]` et votre fichier utilisateur définit `["claude-haiku-4-5"]`, la chaîne est `["claude-sonnet-5"]` uniquement. Claude Code conserve au maximum trois modèles distincts autorisés de la liste et ignore le reste. Consultez [Chaînes de modèles de secours](/docs/fr/model-config#fallback-model-chains).

<h3 id="fastmode">
  `fastMode`
</h3>

Activez le [mode rapide](/docs/fr/fast-mode) pour les sessions où il est disponible, pour le travail interactif comme l'itération rapide ou le débogage en direct où vous voulez la vitesse à un coût plus élevé par token. Vous n'éditez généralement pas cette clé à la main : exécuter `/fast` écrit `fastMode: true` dans `~/.claude/settings.json`, et l'exécuter à nouveau pour désactiver le mode rapide supprime la clé. Le mode rapide s'exécute uniquement sur Opus 5.5, Opus 5, et Opus 4.8 : l'activer à partir d'un autre modèle vous bascule vers Opus, et basculer vers un modèle non supporté le désactive. Consultez [Changer de modèle pendant que le mode rapide est activé](/docs/fr/fast-mode#switch-models-while-fast-mode-is-on).

* **Portée** : [`Tout fichier`](#scopes)
* **Type** : Booléen
  * `true` : Claude Code active le mode rapide pour les sessions où il est disponible
  * `false` : le mode rapide reste désactivé
* **Par défaut** : non défini, donc le mode rapide est désactivé
* **Remplacements par session** : [`CLAUDE_CODE_DISABLE_FAST_MODE`](/docs/fr/env-vars) désactive le mode rapide pour une session, et cette clé ne peut pas l'activer

```json settings.json theme={null}
{
  "fastMode": true
}
```

<h3 id="fastmodepersessionoptin">
  `fastModePerSessionOptIn`
</h3>

Normalement, exécuter `/fast` enregistre [`fastMode`](#fastmode) dans les paramètres utilisateur d'une personne, donc le mode rapide est activé au début de chaque session ultérieure. Définissez cette clé à `true` pour arrêter cela : un `fastMode: true` enregistré n'active plus le mode rapide au démarrage de la session, et chaque personne doit exécuter `/fast` dans chaque session où elle le souhaite. Claude Code laisse la clé `fastMode` dans son fichier, donc désactiver cette clé restaure l'ancien comportement.

Les propriétaires sur les plans Team ou Enterprise peuvent le déployer à l'échelle de l'organisation via les [paramètres gérés par le serveur](/docs/fr/server-managed-settings). Quand les paramètres gérés définissent la clé, `/fast on` est refusé en dehors des sessions de terminal interactif et rapporte que votre organisation a désactivé le mode rapide. Cela couvre le [mode non interactif](/docs/fr/headless), l'[extension VS Code](/docs/fr/vs-code), et les [sessions cloud](/docs/fr/claude-code-on-the-web).

* **Portée** : [`Tout fichier`](#scopes)
* **Type** : Booléen
  * `true` : un `fastMode: true` enregistré n'active plus le mode rapide au démarrage de la session, donc chaque personne exécute `/fast` dans chaque session où elle le souhaite ; un `fastMode: true` passé avec `--settings` compte toujours pour cette session sauf si les paramètres gérés définissent cette clé
  * `false` : un `fastMode: true` enregistré active le mode rapide au début de chaque session ultérieure
* **Par défaut** : `false`

```json settings.json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

Consultez [Exiger l'opt-in par session](/docs/fr/fast-mode#require-per-session-opt-in).

<h3 id="language">
  `language`
</h3>

Faites répondre Claude dans une langue autre que l'anglais par défaut. Il n'y a pas de liste fixe pour les réponses : Claude Code transmet la valeur verbatim à Claude comme instruction pour toujours répondre dans cette langue, donc tout nom de langue que Claude peut lire fonctionne. Claude Code ne vérifie pas la valeur, donc un nom mal orthographié atteint Claude tel qu'écrit plutôt que de produire une erreur. La même valeur définit la langue pour la [dictée vocale](/docs/fr/voice-dictation#change-the-dictation-language), qui a une liste fixe de [langues de dictée supportées](/docs/fr/voice-dictation#change-the-dictation-language), et pour les titres de session générés automatiquement.

* **Portée** : [`Tout fichier`](#scopes)
* **Type** : chaîne, tout nom de langue, comme `"japanese"`, `"spanish"`, ou `"french"` ; Claude Code ne le valide pas
* **Par défaut** : non défini ; les titres de session correspondent alors à la langue de votre conversation

```json settings.json theme={null}
{
  "language": "japanese"
}
```

<h3 id="maxeffortlevel">
  `maxEffortLevel`
</h3>

Limitez le [niveau d'effort](/docs/fr/model-config#adjust-effort-level) qu'une session peut utiliser, en laissant les niveaux inférieurs disponibles. Tout niveau supérieur s'exécute à la limite à la place, y compris celui de `/effort`, du sélecteur `/model`, de `--effort`, de [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/fr/env-vars), du frontmatter `effort` d'une compétence ou d'un sous-agent, ou du défaut du modèle lui-même. Claude Code applique la limite lui-même avant chaque demande, donc elle s'applique sur chaque fournisseur, y compris Amazon Bedrock, la plateforme Agent de Google Cloud, et Microsoft Foundry. Nécessite Claude Code v2.1.267 ou ultérieur.

* **Portée** : [`Tout fichier`](#scopes). Déployez-le dans les paramètres gérés pour l'appliquer à une organisation. Quand plusieurs portées définissent une limite, la plus basse s'applique, donc une limite définie dans une portée ne peut pas être augmentée à partir d'une autre
* **Type** : chaîne, l'une de `"low"`, `"medium"`, `"high"`, `"xhigh"`, ou `"max"`. Une valeur `"max"` ne définit aucune limite
* **Par défaut** : non défini, donc aucune limite ne s'applique
* **Effet sur ultracode** : une limite inférieure à `xhigh` rend [ultracode](#ultracode) indisponible sur les modèles auxquels la limite s'applique
* **Limites par modèle** : ajoutez `maxEffortLevel` à l'entrée [`modelSettings`](#modelsettings) d'un modèle. Cette entrée remplace cette clé pour le modèle uniquement dans la source de paramètres qui définit les deux, comme vos paramètres utilisateur ou une [source gérée](/docs/fr/managed-settings#how-claude-code-combines-managed-sources). Définissez `"max"` là pour exempter le modèle de la limite de cette source ; Claude Code applique toujours les limites d'autres sources

Cet exemple limite chaque modèle à `medium` et exempte Sonnet 4.6 :

```json settings.json theme={null}
{
  "maxEffortLevel": "medium",
  "modelSettings": {
    "claude-sonnet-4-6": {
      "maxEffortLevel": "max"
    }
  }
}
```

Quand votre organisation définit également une [limite d'effort](/docs/fr/model-config#organization-effort-limits) pour un modèle, la limite inférieure des deux s'applique.

<h3 id="model">
  `model`
</h3>

Définissez le modèle que chaque nouvelle session utilise, donc vous n'avez pas à en choisir un avec `/model` à chaque fois. Le définir ici ne vous empêche pas de basculer en milieu de session. Si votre administrateur a défini un [modèle par défaut de l'organisation](/docs/fr/model-config#organization-default-model) pour remplacer la sélection de l'utilisateur, vous obtenez ce modèle même quand vous définissez cette clé dans les paramètres utilisateur, de projet ou locaux.

* **Portée** : [`Tout fichier`](#scopes)
* **Type** : chaîne, un alias de modèle ou un ID de modèle complet
* **Par défaut** : non défini, donc Claude Code utilise le modèle par défaut de votre compte
* **Remplacements par session** : `--model` a la priorité sur [`ANTHROPIC_MODEL`](/docs/fr/env-vars), et les deux ont la priorité sur cette clé pour une session, y compris sur un `model` géré ; une liste [`availableModels`](#availablemodels) s'applique toujours au choix

```json settings.json theme={null}
{
  "model": "claude-sonnet-5"
}
```

Une valeur ici surclasse [`ANTHROPIC_DEFAULT_MODEL`](/docs/fr/model-config#set-a-default-model-for-new-sessions), que Claude Code utilise uniquement quand rien d'autre ne sélectionne un modèle.

<h3 id="modeloverrides">
  `modelOverrides`
</h3>

Mappez les ID de modèles Anthropic aux ID de modèles spécifiques au fournisseur, comme les ARN de profil d'inférence Amazon Bedrock. Chaque entrée du sélecteur de modèles utilise alors sa valeur mappée lors de l'appel de l'API du fournisseur. Les administrateurs utilisent ceci sur [Amazon Bedrock, la plateforme Agent de Google Cloud, et Microsoft Foundry](/docs/fr/model-config#override-model-ids-per-version) pour router chaque version de modèle vers un profil d'inférence spécifique, un nom de version, ou un déploiement pour la gouvernance, l'allocation des coûts, ou le routage régional.

* **Portée** : [`Tout fichier`](#scopes)
* **Type** : objet mappant l'ID du modèle à l'ID du modèle du fournisseur
* **Par défaut** : non défini

Cet exemple route chaque appel pour Opus 4.6 vers le profil d'inférence Bedrock nommé :

```json settings.json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-6": "arn:aws:bedrock:us-east-1:123456789012:inference-profile/example"
  }
}
```

Consultez [Remplacer les ID de modèles par version](/docs/fr/model-config#override-model-ids-per-version).

<h3 id="modelpicker">
  `modelPicker`
</h3>

Énumérez les modèles que le sélecteur `/model` offre, dans l'ordre que vous écrivez et sous les étiquettes que vous choisissez, donc le sélecteur énumère les modèles que votre organisation exécute, après la gamme intégrée ou à la place. Le `model` de chaque ligne est pris verbatim, donc il accepte tout ce que `--model` accepte : un alias comme `opus`, un ID de modèle Anthropic, ou un ID au format du fournisseur pour Amazon Bedrock, la plateforme Agent de Google Cloud, Microsoft Foundry, ou une passerelle LLM. Nécessite Claude Code v2.1.242 ou ultérieur.

* **Portée** : [`Utilisateur ou géré`](#scopes). Claude Code lit la clé à partir des paramètres gérés, `--settings`, et des paramètres utilisateur, et l'ignore dans les paramètres de projet et locaux donc un référentiel que vous clonez ne peut pas réétiqueter le sélecteur. Le plus élevé de ces trois qui définit la clé fournit la gamme entière, et Claude Code ne combine jamais les gammes de deux sources.
* **Type** : objet avec un tableau `options` de lignes et un Booléen `replaceBuiltInOptions` optionnel
* **Par défaut** : non défini, donc le sélecteur affiche la gamme intégrée

Cet exemple ajoute deux déploiements Bedrock après la gamme intégrée, sous des noms que votre équipe reconnaît :

```json managed-settings.json theme={null}
{
  "modelPicker": {
    "options": [
      { "model": "us.anthropic.claude-opus-4-8", "label": "Opus (production)" },
      {
        "model": "us.anthropic.claude-sonnet-4-6",
        "label": "Sonnet (production)",
        "description": "Day-to-day work"
      }
    ]
  }
}
```

<span id="modelpicker-options" />

<span id="modelpicker-replacebuiltinoptions" />

<h4 id="fields-for-modelpicker">
  Champs pour `modelPicker`
</h4>

La clé prend deux champs, l'un pour les lignes elles-mêmes et l'un pour savoir si elles remplacent la gamme intégrée ou l'ajoutent.

| Champ                   | Type                                                                                            | Ce qu'il fait                                                                                                                                                                                                                                                                                |
| :---------------------- | :---------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options`               | tableau de lignes, chacune avec un `model` requis et un `label` et une `description` optionnels | Les lignes que le sélecteur affiche, dans cet ordre, sauf qu'une ligne grisée se déplace vers le bas. Sans un `label`, Claude Code titre la ligne avec le nom intégré pour un modèle qu'il connaît, ou l'ID du modèle sinon, et sans une `description` il écrit une deuxième ligne générique |
| `replaceBuiltInOptions` | Booléen, par défaut `false`                                                                     | Définissez-le à `true` pour afficher uniquement ces lignes, **Par défaut**, et une ligne pour le modèle que la session utilise déjà. Laissez-le non défini pour ajouter ces lignes après la gamme intégrée                                                                                   |

Avec `replaceBuiltInOptions` activé, Claude Code masque chaque autre ligne : la gamme intégrée, les lignes qu'il ajoute pour les entrées [`availableModels`](#availablemodels), les modèles que la [découverte de passerelle](/docs/fr/llm-gateway-protocol#model-discovery) a trouvés, et [`ANTHROPIC_CUSTOM_MODEL_OPTION`](/docs/fr/model-config#add-a-custom-model-option). Avec lui désactivé, Claude Code saute un modèle énuméré que la gamme intégrée couvre déjà. Un label change ce que le sélecteur affiche, pas quel modèle Claude Code exécute.

Une liste d'autorisation [`availableModels`](#availablemodels) s'applique toujours à ces lignes. Avant d'ajouter un modèle énuméré à la liste d'autorisation, lisez [Comportement de fusion](/docs/fr/model-config#merge-behavior) : un ID de modèle spécifique restreint l'entrée générique de sa famille. Claude Code vérifie également chaque ligne par rapport à la session avant d'afficher le sélecteur :

* **Supprimée** : une ligne que Claude Code ne peut pas servir, comme un modèle retiré ou un modèle auquel votre organisation n'a pas accès
* **Grisée** : une ligne que vous ne pouvez pas sélectionner encore, affichée avec la raison
* **Aucune ligne ne survit** : Claude Code conserve la gamme intégrée, filtrée par la liste d'autorisation comme d'habitude

Claude Code supprime une ligne qu'il ne peut pas analyser et conserve le reste. Consultez [Corriger un fichier de paramètres cassé](/docs/fr/settings#fix-a-broken-settings-file).

<h3 id="modelpricing">
  `modelPricing`
</h3>

Rapportez les dépenses aux tarifs que votre organisation paie au lieu du prix catalogue. Définissez-le quand votre organisation a des tarifs contractuels, donc les chiffres en dollars que les développeurs voient correspondent à votre facture. Claude Code applique les tarifs dans `/usage`, la [ligne d'état](/docs/fr/statusline), le `total_cost_usd` de l'Agent SDK, la limite [`--max-budget-usd`](/docs/fr/cli-reference), et la métrique de coût [OpenTelemetry](/docs/fr/monitoring-usage) et les événements. Vous fournissez les tarifs : Claude Code ne les lit pas à partir de votre contrat ou de la Claude Console. Nécessite Claude Code v2.1.242 ou ultérieur.

* **Portée** : [`Géré`](#scopes). Déployez la clé via les paramètres gérés par le serveur, une politique MDM, un fichier `managed-settings.json`, ou un [assistant de politique](/docs/fr/managed-settings#compute-the-policy-with-a-helper-program). Claude Code l'ignore dans les paramètres utilisateur, de projet et locaux, dans `--settings`, et sur Windows dans le [registre HKCU](/docs/fr/managed-settings#where-each-mechanism-stores-the-policy) inscriptible par l'utilisateur. Avec les paramètres gérés par le serveur, chaque session rapporte les coûts au prix catalogue jusqu'à ce que la [récupération des paramètres](/docs/fr/server-managed-settings#fetch-and-caching-behavior) de cette session ait confirmé le paramètre. Une application hôte qui intègre Claude Code et définit [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/fr/env-vars) peut fournir sa propre table via l'option SDK [`managedSettings`](/docs/fr/agent-sdk/typescript#options), que Claude Code utilise uniquement quand aucune source gérée ne définit la clé et uniquement dans Claude Code v2.1.246 ou ultérieur.
* **Type** : objet avec un `multiplier` optionnel et une carte `overrides` optionnelle
* **Par défaut** : non défini, donc Claude Code rapporte le prix catalogue sauf si une application hôte fournit une table

Définissez `multiplier` seul pour une remise ou une majoration forfaitaire, `overrides` seul pour des tarifs par modèle, ou les deux.

Cet exemple définit les tarifs contractuels pour Sonnet 4.6 puis réduit chaque chiffre, la ligne Sonnet incluse, de 15 % :

```json managed-settings.json theme={null}
{
  "modelPricing": {
    "multiplier": 0.85,
    "overrides": {
      "claude-sonnet-4-6": {
        "input": 2.4,
        "output": 12,
        "cacheRead": 0.24,
        "cacheWrite": 3
      }
    }
  }
}
```

Définissez `multiplier` au-dessus de 1, jusqu'à 10, pour majorer chaque chiffre. Une majoration nécessite Claude Code v2.1.271 ou ultérieur. Les versions antérieures ignorent un `multiplier` au-dessus de 1 avec un avertissement et conservent le reste du paramètre.

Pour les étapes, y compris comment confirmer que les tarifs sont en vigueur, consultez [Rapporter les dépenses à vos tarifs contractuels](/docs/fr/costs#report-spend-at-your-contracted-rates).

<span id="modelpricing-multiplier" />

<span id="modelpricing-overrides" />

<h4 id="fields-for-modelpricing">
  Champs pour `modelPricing`
</h4>

| Champ        | Type                                                                                                               | Ce qu'il fait                                                                                                                                                                                                                                              |
| :----------- | :----------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `multiplier` | nombre supérieur à 0 et au maximum 10                                                                              | Redimensionne chaque coût que Claude Code calcule, qu'une ligne `overrides` le couvre ou non. Inférieur à 1 est une remise, supérieur à 1 une majoration                                                                                                   |
| `overrides`  | carte d'ID de modèle à un objet de tarif avec `input`, `output`, `cacheRead`, et `cacheWrite`, chacun de 0 à 10000 | Les tarifs USD par million de tokens pour ce modèle, les quatre requis. `cacheWrite` couvre à la fois les écritures de cache de cinq minutes et d'une heure. Consultez [Quels modèles une ligne s'applique à](#which-models-a-modelpricing-row-applies-to) |

Claude Code utilise les tarifs d'une ligne exactement comme vous les avez écrits, sans ajouter la surcharge du mode rapide ou le [tarif d'inférence réservé aux États-Unis](https://platform.claude.com/docs/en/about-claude/pricing). Si vous définissez également `multiplier`, Claude Code l'applique en plus des tarifs de la ligne. Claude Code supprime une ligne avec un tarif qu'il ne peut pas analyser, ou un `multiplier` qu'il ne peut pas analyser, et conserve le reste ; consultez [Corriger un fichier de paramètres cassé](/docs/fr/settings#fix-a-broken-settings-file).

<h4 id="which-models-a-modelpricing-row-applies-to">
  Quels modèles une ligne `modelPricing` s'applique à
</h4>

Claude Code décide quels modèles une ligne s'applique à à partir de la clé de la ligne :

* **L'ID d'un modèle intégré** : une clé que Claude Code lui-même utilise pour un modèle intégré, que cette clé soit l'ID du modèle lui-même, comme `claude-sonnet-4-6`, ou son ID Bedrock, Agent Platform, ou Foundry. Claude Code applique la ligne à chaque ID de snapshot daté et ID spécifique au fournisseur de ce modèle.
* **Toute autre clé** : une clé qui n'est pas l'ID d'un modèle intégré, comme un alias de modèle de passerelle. Claude Code applique la ligne à cet ID uniquement. Quand un ID de modèle correspond exactement à l'une de vos clés et tombe également sous une ligne clé par l'ID d'un modèle intégré, Claude Code utilise la correspondance exacte.
* **Un profil d'inférence d'application Bedrock** : une fois que Claude Code a résolu le profil au modèle vers lequel il route, via votre carte [`modelOverrides`](#modeloverrides) ou la [recherche `bedrock:GetInferenceProfile`](/docs/fr/amazon-bedrock#iam-configuration), Claude Code applique la ligne de ce modèle au profil.

<h3 id="modelsettings">
  `modelSettings`
</h3>

Enregistrez un [niveau d'effort](/docs/fr/model-config#adjust-effort-level) pour chaque modèle que vous utilisez. Nécessite Claude Code v2.1.251 ou ultérieur.

Dans une session interactive sur votre machine, quand vous enregistrez `low`, `medium`, `high`, ou `xhigh` comme votre défaut avec `/effort` ou le curseur d'effort du sélecteur `/model`, Claude Code écrit ce niveau ici sous le modèle que vous utilisez, donc vous éditez rarement cette clé vous-même. Quand vous choisissez l'un de ces niveaux dans le [sélecteur de modèles de l'extension VS Code](/docs/fr/vs-code#use-the-prompt-box), Claude Code l'enregistre ici de la même manière. L'entrée [`effortLevel`](#effortlevel) énumère les sessions où `/effort` s'applique à cette session uniquement.

Éditez la clé à la main pour modifier ou supprimer un niveau que vous avez enregistré.

Un `effortLevel` d'un modèle ici a la priorité sur le [`effortLevel`](#effortlevel) de niveau supérieur dans le même fichier de paramètres. Entre les fichiers, Claude Code résout chaque modèle séparément : le [fichier de paramètres](/docs/fr/settings#settings-precedence) de plus haute priorité qui définit soit un `effortLevel` pour ce modèle soit un `effortLevel` de niveau supérieur qui [s'applique à ce modèle](#effortlevel) décide, donc un `effortLevel` dans les paramètres gérés surclasse un niveau que vous avez enregistré dans les paramètres utilisateur. [Ajuster le niveau d'effort](/docs/fr/model-config#adjust-effort-level) énumère ce qui d'autre peut remplacer un niveau enregistré, comme `--effort` au lancement.

Pour limiter l'effort d'un modèle plutôt que de définir son niveau, ajoutez un champ [`maxEffortLevel`](#maxeffortlevel) à l'entrée de ce modèle. Le champ nécessite Claude Code v2.1.267 ou ultérieur.

* **Portée** : [`Tout fichier`](#scopes)
* **Type** : objet mappant un nom de modèle à un objet avec un champ `effortLevel`, l'un de `"low"`, `"medium"`, `"high"`, ou `"xhigh"`, un champ [`maxEffortLevel`](#maxeffortlevel), ou les deux
* **Par défaut** : non défini

Claude Code écrit chaque entrée sous le nom canonique du modèle, comme `claude-opus-5-5`, et correspond à l'alias de ce modèle, daté, `[1m]`, et les ID spécifiques au fournisseur reconnus à la même entrée.

Cet exemple garde Opus 5.5 à `high` tandis que les autres modèles utilisent leurs propres niveaux enregistrés ou par défaut :

```json settings.json theme={null}
{
  "modelSettings": {
    "claude-opus-5-5": {
      "effortLevel": "high"
    }
  }
}
```

Exécutez `/effort auto` pour effacer votre niveau enregistré pour le modèle que vous utilisez. Claude Code laisse les autres entrées et tout `effortLevel` de niveau supérieur en place.

<h3 id="outputstyle">
  `outputStyle`
</h3>

Sélectionnez un [style de sortie](/docs/fr/output-styles) par nom. Un style de sortie est un ensemble d'instructions enregistrées qui change le rôle, le ton et le format de sortie de Claude, comme les styles Explanatory et Learning intégrés ou l'un que vous avez écrit vous-même.

Si vous modifiez cette clé pendant une session, Claude utilise le nouveau style à partir de votre message suivant. Pour ce que ce message coûte en cache de prompt, consultez [Changer le style de sortie](/docs/fr/prompt-caching#changing-output-style). Avant la v2.1.251, l'édition s'appliquait uniquement après que vous ayez exécuté `/clear` ou démarré une nouvelle session.

* **Portée** : [`Tout fichier`](#scopes)
* **Type** : chaîne, le nom d'un style de sortie [intégré](/docs/fr/output-styles#built-in-output-styles) ou [personnalisé](/docs/fr/output-styles#create-a-custom-output-style)
* **Par défaut** : non défini, donc Claude Code utilise le style par défaut

Cet exemple sélectionne le style Explanatory intégré, qui ajoute des perspectives éducatives entre les tâches :

```json settings.json theme={null}
{
  "outputStyle": "Explanatory"
}
```

<h3 id="promptcachettl">
  `promptCacheTtl`
</h3>

Choisissez combien de temps le [cache de prompt](/docs/fr/prompt-caching) conserve la conversation principale. Cette clé s'applique à vos tours interactifs, `-p`, et Agent SDK, ainsi que les assistants que Claude Code exécute en ligne avec eux. La durée de vie d'une heure garde le cache chaud entre les pauses plus longues, et l'API [facture chaque écriture de cache à un tarif plus élevé](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing) qu'à la durée de vie de cinq minutes. Nécessite Claude Code v2.1.242 ou ultérieur.

* **Portée** : [`Tout fichier`](#scopes)
* **Type** : chaîne, l'une de :
  * `"5m"` : le cache se conserve pendant cinq minutes
  * `"1h"` : le cache se conserve pendant une heure
* **Par défaut** : non défini, donc chaque demande de conversation principale obtient sa [durée de vie par défaut](/docs/fr/prompt-caching#which-ttl-each-request-gets)
* **Remplacements par session** : [`FORCE_PROMPT_CACHING_5M`](/docs/fr/env-vars) a la priorité sur tout le reste, puis [`CLAUDE_CODE_PROMPT_CACHE_TTL`](/docs/fr/env-vars), puis cette clé, et enfin [`ENABLE_PROMPT_CACHING_1H`](/docs/fr/env-vars)

Cet exemple garde la conversation principale sur la durée de vie d'une heure et laisse les sous-agents sur cinq minutes :

```json settings.json theme={null}
{
  "promptCacheTtl": "1h",
  "subagentPromptCacheTtl": "5m"
}
```

Pour ce que chaque durée de vie coûte, consultez [Durée de vie du cache](/docs/fr/prompt-caching#cache-lifetime).

<h3 id="showthinkingsummaries">
  `showThinkingSummaries`
</h3>

Voyez les résumés de la [réflexion étendue](/docs/fr/model-config#extended-thinking) de Claude dans les sessions interactives. Définissez-le si vous voulez les résumés complets quand vous développez la réflexion avec `Ctrl+O`. Quand non défini ou `false`, l'API Anthropic rédige les blocs de réflexion et Claude Code affiche un stub réduit ; les fournisseurs tiers ne rédactent pas.

* **Portée** : [`Tout fichier`](#scopes)
* **Type** : Booléen
  * `true` : vous voyez les résumés complets de la réflexion quand vous développez la réflexion avec `Ctrl+O`
  * `false` : l'API Anthropic rédige les blocs de réflexion et Claude Code affiche un stub réduit
* **Par défaut** : `false`

```json settings.json theme={null}
{
  "showThinkingSummaries": true
}
```

La rédaction change uniquement ce que vous voyez, pas ce que le modèle génère. Pour réduire la dépense de réflexion, [réduisez le budget ou désactivez la réflexion](/docs/fr/model-config#extended-thinking) à la place.

<h3 id="subagentpromptcachettl">
  `subagentPromptCacheTtl`
</h3>

Choisissez combien de temps le [cache de prompt](/docs/fr/prompt-caching) conserve les demandes que Claude Code fait en dehors de la conversation principale. Cette clé s'applique aux [sous-agents](/docs/fr/sub-agents), aux [workflows](/docs/fr/workflows), et aux demandes propres de Claude Code en arrière-plan et d'assistance, comme la compaction et les titres de session. La durée de vie d'une heure garde le cache chaud entre les pauses plus longues, et l'API [facture chaque écriture de cache à un tarif plus élevé](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing) qu'à la durée de vie de cinq minutes. Nécessite Claude Code v2.1.242 ou ultérieur.

* **Portée** : [`Tout fichier`](#scopes)
* **Type** : chaîne, l'une de :
  * `"5m"` : le cache se conserve pendant cinq minutes
  * `"1h"` : le cache se conserve pendant une heure
* **Par défaut** : non défini, donc chacune de ces demandes obtient sa [durée de vie par défaut](/docs/fr/prompt-caching#which-ttl-each-request-gets)
* **Remplacements par session** : [`FORCE_PROMPT_CACHING_5M`](/docs/fr/env-vars) a la priorité sur tout le reste, puis [`CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`](/docs/fr/env-vars), puis cette clé, puis [`ENABLE_PROMPT_CACHING_1H`](/docs/fr/env-vars), qui demande la durée de vie d'une heure sur chaque demande. Pour où la valeur du frontmatter propre d'un sous-agent se classe, consultez [Choisir la TTL vous-même](/docs/fr/prompt-caching#choose-the-ttl-yourself)

Cet exemple donne aux sous-agents et aux autres demandes en dehors de la conversation principale la durée de vie d'une heure :

```json settings.json theme={null}
{
  "subagentPromptCacheTtl": "1h"
}
```

Cette clé couvre les demandes que [`promptCacheTtl`](#promptcachettl) ne couvre pas, donc définissez les deux pour choisir une durée de vie pour chaque demande que Claude Code fait. Pour comment le cache d'un sous-agent diffère de celui de la conversation principale, consultez [Sous-agents et le cache](/docs/fr/prompt-caching#subagents-and-the-cache).

<h3 id="switchmodelsonflag">
  `switchModelsOnFlag`
</h3>

Choisissez ce qui se passe quand un [classificateur de sécurité signale une demande](/docs/fr/model-config#automatic-model-fallback) : basculer vers le modèle de secours et continuer, ou faire une pause pour que vous puissiez choisir entre basculer et éditer l'invite.

* **Portée** : [`Tout fichier`](#scopes). Apparaît dans `/config` comme **Changer de modèles quand un message est signalé**.
* **Type** : Booléen
  * `true` : Claude Code bascule vers le modèle de secours et continue
  * `false` : dans une session interactive Claude Code fait une pause pour que vous puissiez choisir entre basculer et éditer l'invite ; où aucune boîte de dialogue ne peut s'afficher, comme une exécution `-p`, la demande signalée se termine comme une erreur
* **Par défaut** : `true`, basculer automatiquement

```json settings.json theme={null}
{
  "switchModelsOnFlag": false
}
```

Consultez [Demander avant de basculer](/docs/fr/model-config#ask-before-switching).

<h3 id="ultracode">
  `ultracode`
</h3>

Démarrez les sessions avec [ultracode](/docs/fr/workflows#let-claude-decide-with-ultracode) activé. Avec lui activé, Claude planifie un workflow pour chaque tâche substantielle au lieu d'attendre que vous le demandiez. Claude planifie les workflows uniquement quand les [workflows dynamiques](/docs/fr/workflows) sont activés pour vous, votre modèle supporte l'effort `xhigh`, et aucune [limite d'effort](/docs/fr/model-config#organization-effort-limits) inférieure à `xhigh` ne s'applique. De toute façon, `ultracode: true` exécute la session à l'effort `xhigh`, ou à la limite quand une limite d'effort est inférieure. Claude Code lit cette clé mais ne l'écrit jamais : `/effort ultracode` active ultracode pour la session actuelle uniquement.

* **Portée** : [`Tout fichier`](#scopes)
* **Type** : Booléen
  * `true` : les sessions commencent à l'effort `xhigh`, avec ultracode activé quand les workflows dynamiques sont activés pour vous, votre modèle supporte `xhigh`, et aucune limite d'effort n'est inférieure à `xhigh`
  * `false` : les sessions commencent avec ultracode désactivé
* **Par défaut** : non défini, donc ultracode est désactivé
* **Remplacements par session** : `/effort ultracode` active ultracode pour une session sans cette clé. L'indicateur `--effort ultracode` l'active également pour une session et nécessite Claude Code v2.1.203 ou ultérieur

```json settings.json theme={null}
{
  "ultracode": true
}
```

Ultracode exécute la session à l'effort `xhigh` et a la priorité sur `effortLevel` et les entrées [`modelSettings`](#modelsettings). Si une [limite d'effort](/docs/fr/model-config#organization-effort-limits) inférieure à `xhigh` s'applique au modèle, comme un paramètre [`maxEffortLevel`](#maxeffortlevel), la session s'exécute à la limite à la place et ultracode reste désactivé. Claude ne planifie alors pas les workflows de son propre chef, et `/effort` n'offre pas `ultracode`. Une demande de contrôle `apply_flag_settings` de l'Agent SDK accepte également la clé.

<h2 id="permission-settings">
  Paramètres de permission
</h2>

Décidez ce que Claude peut faire sans demander, quel mode de permission une session démarre, et ce que le classificateur du mode auto autorise. Pour la syntaxe des règles et le modèle de permission, consultez [Configurer les permissions](/docs/fr/permissions).

<h3 id="allowmanagedpermissionrulesonly">
  `allowManagedPermissionRulesOnly`
</h3>

Rendez les paramètres gérés la seule source de règles de permission. Claude Code ignore alors les règles `allow`, `ask` et `deny` dans les fichiers utilisateur, projet, local et `--settings`, ignore `--allowedTools`, masque les choix d'autorisation permanente dans les invites de permission, et arrête d'enregistrer les nouvelles règles.

Quand [les paramètres parents d'un hôte d'intégration](/docs/fr/managed-settings#let-an-embedding-host-add-policy) s'appliquent, Claude Code les traite comme faisant partie du niveau géré. Il supprime leurs règles `allow` et `additionalDirectories`, et conserve leurs règles `deny` et `ask` sauf les règles `Read` et `Edit` dont le motif commence par `!`. Un hôte ne peut pas découper les chemins des règles gérées avec une règle `!`, que vous définissiez cette clé ou non.

Les règles `--disallowedTools` et les règles `deny` et `ask` de la session actuelle s'appliquent toujours, y compris après que Claude Code recharge les paramètres en cours de session. Elles ne font que restreindre, donc elles ne peuvent pas élargir ce que les règles gérées accordent. Avant la v2.1.257, Claude Code supprimait ces règles de ligne de commande et de session au premier rechargement des paramètres.

Pour ce qu'un motif `!` dans une règle `--disallowedTools` ou de session peut découper, consultez [Règles Read et Edit](/docs/fr/permissions#read-and-edit).

* **Portée** : [`Managed`](#scopes)
* **Type** : Booléen
  * `true` : les paramètres gérés deviennent la seule source de règles de permission
  * `false` : Claude Code applique les règles de permission des fichiers utilisateur, projet, local et `--settings` en plus des règles gérées
* **Défaut** : non défini, donc Claude Code applique les règles de permission des paramètres utilisateur, projet et local et de `--settings`, en plus des règles gérées

```json managed-settings.json theme={null}
{
  "allowManagedPermissionRulesOnly": true
}
```

Cette clé ne verrouille pas la liste d'autorisation du serveur MCP ; pour cela, définissez [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly). Consultez [Paramètres gérés uniquement](/docs/fr/managed-settings#managed-only-settings).

<h3 id="automode">
  `autoMode`
</h3>

Ajoutez vos propres règles à ce que le classificateur du [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) bloque et autorise. Utilisez-le pour indiquer au classificateur quels dépôts, buckets et domaines votre organisation approuve, afin qu'il arrête de bloquer les opérations internes courantes. Le classificateur est livré avec [des règles d'autorisation et de blocage intégrées](/docs/fr/auto-mode-config#inspect-the-defaults-and-your-effective-config). Incluez la chaîne littérale `"$defaults"` dans un tableau pour conserver ces règles intégrées à cette position et ajouter les vôtres autour ; omettez-la pour les remplacer par les vôtres.

* **Portée** : [`User or managed`](#scopes)
* **Type** : objet avec des tableaux `environment`, `allow`, `soft_deny` et `hard_deny` de règles en prose, plus le Booléen [`classifyAllShell`](#automode-classifyallshell)
* **Défaut** : non défini, donc le classificateur utilise uniquement ses [règles intégrées](/docs/fr/auto-mode-config#inspect-the-defaults-and-your-effective-config)

Cet exemple conserve les règles `soft_deny` intégrées, via `"$defaults"`, et en ajoute une qui bloque `terraform apply` :

```json settings.json theme={null}
{
  "autoMode": {
    "soft_deny": ["$defaults", "Never run terraform apply"]
  }
}
```

Quand plus d'un de ces fichiers définit le même tableau, Claude Code concatène les entrées. Pour le format de règle et la façon dont chaque tableau est appliqué, consultez [Configurer le mode auto](/docs/fr/auto-mode-config).

<h3 id="automode-classifyallshell">
  `autoMode.classifyAllShell`
</h3>

Envoyez chaque commande Bash et PowerShell via le classificateur du mode auto pendant que le mode auto est actif. Par défaut, le mode auto suspend uniquement les règles d'autorisation qui pourraient exécuter du code arbitraire : les règles à l'échelle de l'outil et les règles avec caractères génériques telles que `Bash(*)`, et les préfixes d'interpréteur ou de shell-wrapper tels que `Bash(python *)`. Une commande qui correspond à toute autre règle d'autorisation, telle que `Bash(npm test)`, ignore le classificateur sauf si elle porte [des domaines autorisés par commande](/docs/fr/sandboxing#per-command-allowed-domains-in-auto-mode). Quand elle ignore, un argument destructeur que le préfixe de la règle n'a pas anticipé peut passer inaperçu. La définition de cette clé suspend chaque règle d'autorisation shell pour la session afin que le classificateur voie chaque commande. Nécessite Claude Code v2.1.193 ou ultérieur.

* **Portée** : [`User or managed`](#scopes). Lue partout où [`autoMode`](#automode) est lu.
* **Type** : Booléen
  * `true` : pendant que le mode auto est actif, Claude Code envoie chaque commande Bash et PowerShell via le classificateur et suspend vos règles d'autorisation shell ; en dehors du mode auto, les règles s'appliquent toujours
  * `false` : le mode auto suspend uniquement les règles d'autorisation qui pourraient exécuter du code arbitraire, telles que `Bash(*)` et `Bash(python *)` ; une commande qui correspond à toute autre règle d'autorisation ignore le classificateur sauf si elle porte [des domaines autorisés par commande](/docs/fr/sandboxing#per-command-allowed-domains-in-auto-mode), et chaque autre commande shell la traverse
* **Défaut** : `false`

```json settings.json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

Consultez [Router toutes les commandes shell via le classificateur](/docs/fr/auto-mode-config#route-all-shell-commands-through-the-classifier). Nécessite Claude Code v2.1.193 ou ultérieur.

<h3 id="disableautomode">
  `disableAutoMode`
</h3>

Supprimez le [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) du cycle `Shift+Tab`. Toute session qui démarrerait autrement [en mode auto](/docs/fr/permission-modes#which-mode-a-session-starts-in), que ce soit à partir de `--permission-mode auto`, d'un fichier de paramètres ou de la valeur par défaut intégrée, démarre plutôt en `default`. Les administrateurs le définissent dans les paramètres gérés pour empêcher les développeurs de leur organisation d'utiliser le mode auto.

* **Portée** : [`Any file`](#scopes). Plus utile dans les [paramètres gérés](/docs/fr/managed-settings), où les utilisateurs ne peuvent pas le remplacer. Également accepté sous `permissions` comme `permissions.disableAutoMode`.
* **Type** : la chaîne `"disable"`
* **Défaut** : non défini

```json settings.json theme={null}
{
  "disableAutoMode": "disable"
}
```

<h3 id="permissions">
  `permissions`
</h3>

Contrôlez quels outils Claude peut utiliser sans demander, lesquels invitent toujours, et lesquels sont bloqués, et définissez le [mode de permission](/docs/fr/permission-modes) qu'une session démarre. Chaque clé `permissions.*` ci-dessous s'imbrique sous cet objet.

* **Portée** : [`Any file`](#scopes)
* **Type** : objet avec `allow`, `ask`, `deny`, `additionalDirectories`, `blockReadsOutsideWorkingDirectories`, `defaultMode`, `disableBypassPermissionsMode` et `disableAutoMode`
* **Défaut** : non défini

Cet exemple approuve les commandes `npm run` sans demander, invite avant `git push`, bloque les lectures de `.env`, et démarre les sessions en `acceptEdits` :

```json settings.json theme={null}
{
  "permissions": {
    "allow": ["Bash(npm run *)"],
    "ask": ["Bash(git push *)"],
    "deny": ["Read(./.env)"],
    "defaultMode": "acceptEdits"
  }
}
```

Les trois tableaux de règles partagent une syntaxe ; consultez [Syntaxe des règles de permission](#permission-rule-syntax) sous `permissions.allow`. Pour savoir comment les règles de permission de différents fichiers se combinent, consultez [comment les règles de permission fusionnent entre les portées](/docs/fr/permissions#settings-precedence) ; pour savoir comment les clés de paramètres en général se combinent, consultez [Précédence des paramètres](/docs/fr/settings#settings-precedence) dans le guide des paramètres.

<h3 id="useautomodeduringplan">
  `useAutoModeDuringPlan`
</h3>

Choisissez si Claude Code utilise le classificateur du mode auto pour examiner les commandes shell en mode plan. Avec la valeur par défaut `true`, le classificateur examine chaque commande pendant la planification quand le mode auto est disponible et que vous ne voyez aucune invite. Définissez `false` pour obtenir une invite de permission pour chaque commande en dehors de l'ensemble intégré en lecture seule. Apparaît dans `/config` comme **Use auto mode during plan**.

* **Portée** : [`User, local, or managed`](#scopes). Un dépôt ne peut pas l'éteindre pour vous.
* **Type** : Booléen
  * `true` : identique à non défini ; quand le mode auto est disponible, le classificateur examine chaque commande shell pendant la planification au lieu de vous la demander. Un `false` dans l'un de ces fichiers l'éteint toujours
  * `false` : vous obtenez une invite de permission pour chaque commande en dehors de l'ensemble intégré en lecture seule
* **Défaut** : `true`

```json settings.json theme={null}
{
  "useAutoModeDuringPlan": false
}
```

<h3 id="permissions-allow">
  `permissions.allow`
</h3>

Listez les utilisations d'outils que Claude Code approuve sans vous demander. Dans une règle MCP, `*` ne peut apparaître que dans le nom de l'outil après le préfixe `mcp__<server>__`, tel que `mcp__github__get_*` ; il ne peut pas apparaître dans le nom du serveur.

* **Portée** : [`Any file`](#scopes)
* **Type** : tableau de chaînes de règles de permission
* **Défaut** : non défini
* **Remplacements par session** : `--allowedTools` ajoute des règles d'autorisation pour une session, et une règle de blocage de n'importe quel fichier de paramètres bloque toujours un outil qu'elle nomme

Cet exemple approuve `git diff` et permet à Claude Code de lire votre `.zshrc` sans demander :

```json settings.json theme={null}
{
  "permissions": {
    "allow": ["Bash(git diff *)", "Read(~/.zshrc)"]
  }
}
```

Claude Code applique les règles `allow` du `.claude/settings.json` d'un projet uniquement après que vous acceptiez la [boîte de dialogue de confiance de l'espace de travail](/docs/fr/permissions#project-allow-rules-and-workspace-trust) pour ce dossier.

<h4 id="permission-rule-syntax">
  Syntaxe des règles de permission
</h4>

Les règles de permission suivent le format `Tool` ou `Tool(specifier)`. Claude Code évalue d'abord les règles `deny`, puis `ask`, puis `allow`, et la première correspondance décide indépendamment de la spécificité de chaque règle ; consultez l'[ordre d'évaluation des règles de permission](/docs/fr/permissions#manage-permissions).

Chaque ligne montre une forme de règle et ce qu'elle correspond.

| Règle                          | Ce qu'elle correspond                     |
| :----------------------------- | :---------------------------------------- |
| `Bash`                         | Chaque commande Bash                      |
| `Bash(npm run *)`              | Commandes commençant par `npm run`        |
| `Read(./.env)`                 | Lectures du fichier `.env`                |
| `WebFetch(domain:example.com)` | Demandes de récupération vers example.com |

Pour la syntaxe complète des règles, y compris le comportement des caractères génériques, les motifs spécifiques aux outils pour Read, Edit, WebFetch, MCP et Agent, et les limitations de sécurité des motifs Bash, consultez [Syntaxe des règles de permission](/docs/fr/permissions#permission-rule-syntax).

<h3 id="permissions-ask">
  `permissions.ask`
</h3>

Listez les utilisations d'outils qui vous invitent à confirmer même dans un mode de permission qui les approuverait autrement, tel que `acceptEdits` ou `bypassPermissions`. En mode `dontAsk`, Claude Code refuse une utilisation d'outil correspondante au lieu d'inviter.

* **Portée** : [`Any file`](#scopes)
* **Type** : tableau de chaînes de règles de permission
* **Défaut** : non défini

```json settings.json theme={null}
{
  "permissions": {
    "ask": ["Bash(git push *)"]
  }
}
```

<span id="exclude-sensitive-files" />

<h3 id="permissions-deny">
  `permissions.deny`
</h3>

Listez les utilisations d'outils que Claude Code bloque. Utilisez-le pour les fichiers qui contiennent des clés API, des secrets ou des valeurs d'environnement : Claude Code exclut les fichiers correspondants de la découverte de fichiers et des résultats de recherche, refuse les lectures de ceux-ci, et bloque les outils [Edit et Write](/docs/fr/permissions#read-and-edit) sur les chemins correspondants.

Les règles de blocage Read et Edit s'appliquent aux outils de fichiers intégrés de Claude, aux commandes de fichiers que Claude Code reconnaît dans Bash, telles que `cat`, `head`, `tail`, `sed` et `tee`, et aux cibles des [redirections](/docs/fr/permissions#redirections) Bash telles que `> file` et `< file` ; elles ne s'appliquent pas à une commande qui lit des fichiers sans les nommer, telle que `grep -r pattern .`, ou à des sous-processus arbitraires, donc pour l'application au niveau du système d'exploitation, [activez le sandbox](/docs/fr/sandboxing).

* **Portée** : [`Any file`](#scopes)
* **Type** : tableau de chaînes de règles de permission
* **Défaut** : non défini
* **Remplacements par session** : `--disallowedTools` ajoute des règles de blocage pour une session à côté de cette clé

Cet exemple refuse les lectures des fichiers `.env`, du répertoire `secrets` et d'un fichier de credentials, et bloque les commandes `curl` :

```json settings.json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./.env)",
      "Read(./.env.*)",
      "Read(./secrets/**)",
      "Read(./config/credentials.json)",
      "Bash(curl *)"
    ]
  }
}
```

Les noms d'outils acceptent les motifs glob, donc `"*"` bloque chaque outil et `"mcp__*"` bloque chaque outil MCP. Claude Code ignore une règle de blocage pour l'outil [`EndConversation`](/docs/fr/tools-reference#endconversation-tool-behavior) tant que tout autre outil est toujours disponible pour Claude. Une règle de blocage `Bash` correspond à la commande telle que Claude l'écrit, donc `Bash(curl *)` n'arrête pas `/usr/bin/curl` ou `sh -c 'curl …'` ; consultez [ce qu'une règle Bash ne correspond pas](/docs/fr/permissions#bash-rule-limits). Cette clé remplace la configuration `ignorePatterns` dépréciée.

<h3 id="permissions-additionaldirectories">
  `permissions.additionalDirectories`
</h3>

Donnez à Claude l'accès aux fichiers des répertoires en dehors de celui dans lequel vous avez commencé, comme [répertoires de travail](/docs/fr/permissions#working-directories) supplémentaires. La plupart de la configuration `.claude/` n'est [pas découverte](/docs/fr/permissions#additional-directories-grant-file-access-not-configuration) à partir de ces répertoires.

* **Portée** : [`Any file`](#scopes)
* **Type** : tableau de chemins de répertoires
* **Défaut** : non défini
* **Remplacements par session** : `--add-dir` et `/add-dir` ajoutent des répertoires pour une session à côté de cette clé

```json settings.json theme={null}
{
  "permissions": {
    "additionalDirectories": ["../docs/"]
  }
}
```

Comme les règles `allow`, les entrées du `.claude/settings.json` d'un projet ne prennent effet qu'après que vous acceptiez la [boîte de dialogue de confiance de l'espace de travail](/docs/fr/permissions#project-allow-rules-and-workspace-trust) pour ce dossier.

<h3 id="permissions-blockreadsoutsideworkingdirectories">
  `permissions.blockReadsOutsideWorkingDirectories`
</h3>

Empêchez Claude de lire les chemins en dehors des [répertoires de travail](/docs/fr/permissions#working-directories) de la session avec les outils Read, Grep, Glob et LSP, dans chaque mode de permission y compris `bypassPermissions`. Une commande Bash qui lit un chemin correspondant via une commande de fichier que Claude Code reconnaît, telle que `cat`, vous invite même en mode auto et en mode `bypassPermissions`. Nécessite Claude Code v2.1.257 ou ultérieur.

Une commande Bash que l'analyseur shell ne peut pas tracer, telle qu'une qui change de répertoire plus d'une fois ou exécute un sous-shell, vous invite même en mode auto et en mode `bypassPermissions`. L'invite apparaît même quand la commande ne nomme aucun chemin en dehors des répertoires de travail. Cette invite ne s'applique pas quand la commande s'exécute dans le [sandbox](/docs/fr/sandboxing) et le sandbox applique le blocage.

Claude Code écrit également `true` ici quand vous choisissez de bloquer ces lectures sur [l'invite du mode auto avant la première lecture en dehors des répertoires de travail](/docs/fr/permission-modes#first-read-outside-the-working-directories).

* **Portée** : [`Any file`](#scopes). Si une source de paramètres définit `true`, le blocage s'applique, donc un fichier enregistré d'un dépôt peut activer le blocage pour un projet mais ne peut pas lever un blocage que vous avez défini.
* **Type** : Booléen
  * `true` : les lectures de fichiers en dehors des répertoires de travail sont bloquées
  * `false` : identique à non défini ; un `true` dans n'importe quel autre fichier de paramètres bloque toujours
* **Défaut** : non défini, donc les lectures en dehors des répertoires de travail suivent votre mode de permission et vos règles

```json settings.json theme={null}
{
  "permissions": {
    "blockReadsOutsideWorkingDirectories": true
  }
}
```

Si seul le fichier de paramètres enregistré d'un dépôt ajoute un répertoire, le blocage s'applique toujours aux lectures là-bas. Quand [`autoMemoryDirectory`](#automemorydirectory) provient du `.claude/settings.json` du projet, ou d'un `.claude/settings.local.json` [traité comme fourni par le dépôt](/docs/fr/permissions#when-your-local-settings-file-needs-trust), Claude Code ne charge aucune [mémoire auto](/docs/fr/memory#storage-location) à partir de ce répertoire et n'en enregistre aucune. Les fichiers dont Claude Code lui-même a besoin restent lisibles, tels que vos skills, plugins, règles, agents, commandes et le fichier de mémoire `CLAUDE.md` sous `~/.claude/`.

Quand le [sandbox](/docs/fr/sandboxing) est activé, le blocage refuse également aux commandes sandboxées l'accès en lecture aux répertoires home et aux racines de volumes montés en dehors des répertoires de travail. Une nouvelle tentative qui a besoin d'approbation pour [s'exécuter en dehors du sandbox](/docs/fr/sandboxing#the-unsandboxed-retry-escape-hatch) vous invite même en mode `bypassPermissions`. Les fichiers qu'un outil lit à partir de votre répertoire home, tels que `~/.gitconfig`, sont refusés avec le reste ; rouvrez un chemin spécifique avec [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread) quand un outil en a besoin.

Quand le répertoire de travail de la session est un [git worktree](/docs/fr/worktrees) lié, y compris un que Claude Code a entré en cours de session, le répertoire `.git` commun du dépôt reste lisible et inscriptible pour les commandes sandboxées, afin que git continue de fonctionner là-bas.

<h3 id="permissions-defaultmode">
  `permissions.defaultMode`
</h3>

Définissez le [mode de permission](/docs/fr/permission-modes) que les nouvelles sessions démarrent. Quand vous le laissez non défini, les sessions démarrent dans la [valeur par défaut intégrée](/docs/fr/permission-modes#which-mode-a-session-starts-in) pour votre plan et votre surface.

* **Portée** : [`Any file`](#scopes). `auto` et `bypassPermissions` ne prennent pas effet à partir des paramètres du projet ou local, donc définissez-les plutôt dans `~/.claude/settings.json`. Avant la v2.1.257, `bypassPermissions` prenait effet à partir de n'importe quel fichier. Pour les conversations que l'extension VS Code démarre, Claude Code lit uniquement les valeurs utilisateur, gérées et `--settings`.
* **Type** : chaîne, l'une des :
  * `"default"` : Claude Code exécute uniquement les lectures sans demander
  * `"acceptEdits"` : Claude Code exécute également les éditions de fichiers et les commandes courantes du système de fichiers telles que `mkdir` et `mv` sans demander
  * `"plan"` : Claude Code lit et planifie mais bloque les éditions jusqu'à ce que vous approuviez un plan
  * `"auto"` : Claude Code exécute tout, avec des vérifications de sécurité en arrière-plan
  * `"dontAsk"` : Claude Code refuse automatiquement chaque appel qui inviterait autrement ; les lectures, les autres actions qui ne nécessitent pas d'approbation, et les outils pré-approuvés s'exécutent toujours
  * `"bypassPermissions"` : Claude Code exécute tout sans demander
  * `"manual"` : un alias pour `"default"`, dans Claude Code v2.1.200 ou ultérieur
* **Défaut** : non défini
* **Remplacements par session** : `--permission-mode`, et son équivalent `--dangerously-skip-permissions` pour `bypassPermissions`, prennent la priorité sur cette clé pour une session

```json settings.json theme={null}
{
  "permissions": {
    "defaultMode": "acceptEdits"
  }
}
```

Les règles de permission se superposent à chaque mode : les règles `deny` bloquent dans chaque mode, y compris `bypassPermissions`. Consultez [Modes de permission](/docs/fr/permission-modes). `manual` nomme le mode de permission étiqueté Manual dans la CLI et l'extension VS Code ; l'alias nécessite Claude Code v2.1.200 ou ultérieur. Dans les sessions cloud, Claude Code honore uniquement `acceptEdits`, `plan`, `default` et `auto` à partir de cette clé. Pour les conversations que l'extension VS Code démarre, consultez [quel paramètre l'extension lit pour le mode de permission de démarrage](/docs/fr/permission-modes#switch-permission-modes).

<h3 id="permissions-disablebypasspermissionsmode">
  `permissions.disableBypassPermissionsMode`
</h3>

Empêchez quiconque d'entrer en mode `bypassPermissions`. Claude Code rejette alors l'indicateur `--dangerously-skip-permissions`, et ignore la [définition d'un agent](/docs/fr/sub-agents#permission-modes) `permissionMode: bypassPermissions`, donc le sous-agent s'exécute avec le mode de permission de la session parent.

* **Portée** : [`Any file`](#scopes). Généralement défini dans les [paramètres gérés](/docs/fr/managed-settings) pour appliquer la politique organisationnelle.
* **Type** : la chaîne `"disable"`
* **Défaut** : non défini
* **Remplacements par session** : cette clé prend la priorité sur `--dangerously-skip-permissions`, que Claude Code rejette pendant que la clé est définie

```json settings.json theme={null}
{
  "permissions": {
    "disableBypassPermissionsMode": "disable"
  }
}
```

Avant la v2.1.223, Claude Code appliquait le mode de permission du frontmatter même avec le contournement désactivé.

<h3 id="skipautopermissionprompt">
  `skipAutoPermissionPrompt`
</h3>

Ignorez l'avis unique décrivant le [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) que Claude Code affiche quand vous entrez vous-même en mode auto pour la première fois, par exemple via vos propres paramètres ou le sélecteur de mode, plutôt que quand la valeur par défaut intégrée démarre une session dedans. Claude Code affiche cet avis une fois puis enregistre qu'il a été affiché, donc cette clé ne compte que là où l'avis n'a pas encore apparu.

* **Portée** : [`User or managed`](#scopes). Un dépôt ne peut pas le définir pour vous.
* **Type** : Booléen
  * `true` : Claude Code ignore l'avis
  * `false` : identique à non défini ; l'avis apparaît une fois sauf si un autre de ces fichiers définit `true`
* **Défaut** : non défini, donc l'avis apparaît une fois

```json settings.json theme={null}
{
  "skipAutoPermissionPrompt": true
}
```

<h3 id="skipdangerousmodepermissionprompt">
  `skipDangerousModePermissionPrompt`
</h3>

Ignorez la boîte de dialogue de confirmation que Claude Code affiche avant qu'une session entre en mode `bypassPermissions`, que ce soit à partir de `--dangerously-skip-permissions` ou de `defaultMode: "bypassPermissions"`. Claude Code écrit `true` ici dans vos paramètres utilisateur quand vous acceptez cette boîte de dialogue une fois.

* **Portée** : [`User, local, or managed`](#scopes). Un dépôt non approuvé ne peut pas ignorer la boîte de dialogue pour vous.
* **Type** : Booléen
  * `true` : Claude Code ignore la boîte de dialogue de confirmation avant qu'une session entre en mode `bypassPermissions`
  * `false` : identique à non défini ; la boîte de dialogue apparaît sauf si un autre de ces fichiers définit `true`
* **Défaut** : non défini, donc la boîte de dialogue apparaît

```json settings.json theme={null}
{
  "skipDangerousModePermissionPrompt": true
}
```

<h2 id="sandbox-settings">
  Paramètres du sandbox
</h2>

Isolez les commandes que Claude exécute de votre système de fichiers, de votre réseau et de vos identifiants. Pour savoir comment fonctionne le sandboxing et connaître les exigences de plateforme, consultez [Sandboxing](/docs/fr/sandboxing).

<h3 id="sandbox">
  `sandbox`
</h3>

Isolez les commandes Bash que Claude exécute de votre système de fichiers et de votre réseau avec [sandboxing](/docs/fr/sandboxing). Activez le sandbox avec `enabled`, puis réduisez ou élargissez ce que les commandes en sandbox peuvent toucher avec les sous-objets `filesystem`, `network` et `credentials`. Le sandbox s'exécute sur macOS, Linux et WSL2.

* **Scope** : [`Any file`](#scopes)
* **Type** : objet avec `enabled`, `failIfUnavailable`, `autoAllowBashIfSandboxed`, `excludedCommands`, `allowUnsandboxedCommands`, `enableWeakerNestedSandbox`, `enableWeakerNetworkIsolation`, `allowAppleEvents`, `bwrapPath`, `socatPath`, `ignoreViolations` et `ripgrep`, plus les objets `filesystem`, `network` et `credentials`
* **Default** : non défini, donc Claude Code exécute les commandes sans sandbox

Ceci active le sandbox, ignore les invites de permission pour les commandes en sandbox, exécute `docker` en dehors du sandbox, ouvre deux chemins d'écriture supplémentaires, masque votre fichier d'identifiants AWS et pré-autorise GitHub et npm :

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "autoAllowBashIfSandboxed": true,
    "excludedCommands": ["docker *"],
    "filesystem": {
      "allowWrite": ["/tmp/build", "~/.kube"],
      "denyRead": ["~/.aws/credentials"]
    },
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org"]
    }
  }
}
```

Claude Code prend la valeur d'une clé booléenne à partir de la portée de paramètres la plus prioritaire qui la définit, donc un `enabled` ou `failIfUnavailable` géré remplace tout ce qu'un développeur définit. Il fusionne les clés de tableau dans chaque portée de paramètres que la session charge, donc un développeur peut ajouter des entrées ; consultez [Keep developers from widening the policy](/docs/fr/sandboxing#keep-developers-from-widening-the-policy) pour les verrous réservés aux paramètres gérés. Pour exiger le sandbox pour une organisation, consultez [Enforce sandboxing with managed settings](/docs/fr/sandboxing#enforce-sandboxing-with-managed-settings).

<h3 id="sandbox-enabled">
  `sandbox.enabled`
</h3>

Activez [sandboxing](/docs/fr/sandboxing) pour les commandes Bash. Lorsque vous choisissez un mode dans le panneau `/sandbox`, Claude Code écrit cette clé dans `.claude/settings.local.json` pour le projet actuel ; définissez-la dans `~/.claude/settings.json` pour mettre en sandbox chaque projet.

* **Scope** : [`Any file`](#scopes)
* **Type** : Booléen
  * `true` : Claude Code met en sandbox les commandes Bash
  * `false` : les commandes Bash s'exécutent sans sandbox
* **Default** : `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true
  }
}
```

Sur Linux et WSL2, le sandbox a besoin de `bubblewrap` et `socat` ; consultez [Set up Linux and WSL2](/docs/fr/sandboxing#set-up-linux-and-wsl2). Lorsque le sandbox ne peut pas démarrer, Claude Code affiche un avertissement et exécute les commandes sans sandbox sauf si vous définissez également [`failIfUnavailable`](#sandbox-failifunavailable).

<h3 id="sandbox-failifunavailable">
  `sandbox.failIfUnavailable`
</h3>

Faites en sorte que Claude Code se termine avec une erreur au démarrage lorsque `sandbox.enabled` est `true` mais que le sandbox ne peut pas démarrer, car une dépendance est manquante ou la plateforme n'est pas supportée. Sans cela, Claude Code affiche un avertissement et exécute les commandes sans sandbox. Utilisez-le dans les paramètres gérés lorsque votre organisation exige le sandboxing comme une barrière stricte.

* **Scope** : [`Any file`](#scopes)
* **Type** : Booléen
  * `true` : Claude Code se termine avec une erreur au démarrage lorsque `sandbox.enabled` est `true` mais que le sandbox ne peut pas démarrer
  * `false` : Claude Code affiche un avertissement et exécute les commandes sans sandbox
* **Default** : `false`

Ceci fait en sorte que chaque machine gérée mette en sandbox les commandes ou refuse de démarrer :

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "failIfUnavailable": true
  }
}
```

Consultez [Enforce sandboxing with managed settings](/docs/fr/sandboxing#enforce-sandboxing-with-managed-settings).

<h3 id="sandbox-autoallowbashifsandboxed">
  `sandbox.autoAllowBashIfSandboxed`
</h3>

Laissez Claude Code exécuter les commandes Bash en sandbox sans invite de permission. Les commandes qui ne peuvent pas s'exécuter dans le sandbox passent toujours par le flux de permission régulier, et les règles `deny` et les règles `ask` limitées au contenu telles que `Bash(git push *)` s'appliquent toujours ; une règle `ask` Bash simple est ignorée pour les commandes en sandbox. Définissez-la sur `false` pour envoyer également les commandes en sandbox par le flux de permission régulier, que l'onglet **Mode** de `/sandbox` appelle mode de permissions régulier.

* **Scope** : [`Any file`](#scopes)
* **Type** : Booléen
  * `true` : Claude Code exécute les commandes Bash en sandbox sans invite de permission, sous réserve des règles `deny` et des règles `ask` limitées au contenu ; `CLAUDE_CODE_SUBPROCESS_ENV_SCRUB` désactive l'auto-allow
  * `false` : les commandes en sandbox passent par le flux de permission régulier, donc vos règles d'autorisation et le mode de permission décident. L'onglet **Mode** de `/sandbox` appelle ceci mode de permissions régulier
* **Default** : `true`

Ceci garde le sandbox activé et envoie les commandes en sandbox par le flux de permission régulier :

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "autoAllowBashIfSandboxed": false
  }
}
```

Consultez [Sandbox modes](/docs/fr/sandboxing#sandbox-modes) pour savoir sur quoi l'auto-allow demande toujours et comment il se comporte en mode plan.

<h3 id="sandbox-excludedcommands">
  `sandbox.excludedCommands`
</h3>

Nommez les commandes que Claude Code exécute en dehors du sandbox, comme les outils qui ne fonctionnent pas sous celui-ci. Chaque entrée utilise la même syntaxe que le contenu d'une [règle de permission](/docs/fr/permissions#permission-rule-syntax) `Bash(...)` : une commande exacte, un préfixe tel que `docker *`, ou un motif avec caractères génériques.

Vos entrées sortent un appel Bash du sandbox uniquement lorsqu'elles couvrent chaque commande dedans, et certaines formes d'appel restent en sandbox même dans ce cas. Une entrée `docker *` seule ne sort pas `npm ci && docker build .` du sandbox.

* **Scope** : [`Any file`](#scopes)
* **Type** : tableau de motifs de commande
* **Default** : non défini, donc aucune commande n'est exclue

```json settings.json theme={null}
{
  "sandbox": {
    "excludedCommands": ["docker *"]
  }
}
```

Claude Code garde un appel Bash en sandbox lorsqu'il a l'une de ces formes, parmi d'autres :

* Une commande commençant par `sudo`, `eval` ou `xargs`
* Un `cd`, `pushd` ou `popd`, où qu'il apparaisse dans l'appel
* Une substitution de commande, un sous-shell ou un bloc de contrôle de flux tel que `if` ou `for`
* Une redirection, comme `docker build . > build.log`, autre qu'une qui duplique uniquement un descripteur de fichier, comme `2>&1` le fait
* Un nom de commande qui provient d'une variable

Par exemple, `cd build && docker compose up` reste en sandbox sous une entrée `docker *`, et ajouter une entrée `cd` ne change pas cela.

Les commandes exclues passent toujours par le flux de permission régulier. L'exclusion est une commodité, pas une limite de sécurité : préférez [`filesystem.allowWrite`](#sandbox-filesystem-allowwrite) lorsqu'un outil ne doit écrire que quelque part de spécifique. Claude Code fusionne les entrées dans chaque portée de paramètres que la session charge, et il n'y a pas de verrou réservé aux paramètres gérés pour cette liste, donc gardez une liste gérée étroite.

<h3 id="sandbox-allowunsandboxedcommands">
  `sandbox.allowUnsandboxedCommands`
</h3>

Laissez Claude réessayer une commande en dehors du sandbox avec le paramètre `dangerouslyDisableSandbox` après que le sandbox l'ait bloquée. Définissez-la sur `false` pour que Claude Code ignore complètement ce paramètre et que chaque commande que Claude exécute soit en sandbox ou apparaisse dans [`excludedCommands`](#sandbox-excludedcommands). L'onglet **Overrides** de `/sandbox` affiche cet état comme **Strict sandbox mode**. Utilisez `false` dans les paramètres gérés pour les politiques qui exigent un sandboxing strict.

* **Scope** : [`Any file`](#scopes)
* **Type** : Booléen
  * `true` : Claude peut réessayer une commande en dehors du sandbox avec le paramètre `dangerouslyDisableSandbox` après que le sandbox l'ait bloquée
  * `false` : Claude Code ignore ce paramètre, donc chaque commande que Claude exécute est en sandbox ou apparaît dans `excludedCommands`
* **Default** : `true`

Ceci applique le mode sandbox strict pour tous les utilisateurs couverts par les paramètres gérés :

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "allowUnsandboxedCommands": false
  }
}
```

Une tentative sans sandbox passe par le flux de permission régulier, avec une invite en mode Manuel. Consultez [The unsandboxed retry escape hatch](/docs/fr/sandboxing#the-unsandboxed-retry-escape-hatch).

Pour voir quand les commandes que vous tapez vous-même à l'[invite shell-mode avec préfixe `!`](/docs/fr/interactive-mode#shell-mode-with-prefix) s'exécutent en sandbox, consultez [strict sandbox mode](/docs/fr/sandboxing#the-unsandboxed-retry-escape-hatch).

<h3 id="sandbox-filesystem">
  `sandbox.filesystem`
</h3>

Contrôlez les chemins que les commandes en sandbox peuvent lire et écrire. Par défaut, elles peuvent écrire dans le répertoire de travail, le répertoire temporaire de la session et les répertoires que vous ajoutez avec `--add-dir`, `/add-dir` ou `permissions.additionalDirectories`, et peuvent lire le reste du système de fichiers, y compris les fichiers d'identifiants. Élargissez ou réduisez cela avec les quatre listes de chemins, ou désactivez la couche du système de fichiers avec `disabled`. Consultez [Filesystem isolation](/docs/fr/sandboxing#filesystem-isolation) pour les limites par défaut.

* **Scope** : [`Any file`](#scopes)
* **Type** : objet avec les tableaux `allowWrite`, `denyWrite`, `denyRead` et `allowRead`, plus les booléens `allowManagedReadPathsOnly` et `disabled`
* **Default** : non défini, donc les limites de lecture et d'écriture par défaut s'appliquent

Ceci permet aux commandes en sandbox d'écrire dans un répertoire de build et votre kubeconfig, et masque votre fichier d'identifiants AWS :

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "allowWrite": ["/tmp/build", "~/.kube"],
      "denyRead": ["~/.aws/credentials"]
    }
  }
}
```

Claude Code applique ces listes à la limite du sandbox du système d'exploitation, donc elles s'appliquent à chaque sous-processus qu'une commande en sandbox démarre, comme `kubectl`, `terraform` ou `npm`. Claude Code ajoute vos [règles de permission](/docs/fr/sandboxing#permission-rules) aux mêmes listes : les règles `Edit` allow et deny à `allowWrite` et `denyWrite`, les règles `Read` deny à `denyRead`, et les règles `WebFetch(domain:...)` allow et deny aux listes de domaines [`network`](#sandbox-network).

Sauf si un verrou réservé aux paramètres gérés est défini, Claude Code fusionne chaque liste dans les fichiers de paramètres que la session charge. [`allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly) limite `allowRead` aux entrées des paramètres gérés, et [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) fait la même chose pour les domaines autorisés.

[Configure sandboxing](/docs/fr/sandboxing#configure-sandboxing) couvre les sources que vous excluez avec `--setting-sources`. Lorsque vous modifiez une liste pendant une session, Claude Code [applique la modification à la session en cours d'exécution](/docs/fr/settings#when-edits-take-effect).

<h4 id="sandbox-path-prefixes">
  Préfixes de chemin du sandbox
</h4>

Les chemins dans `allowWrite`, `denyWrite`, `denyRead`, `allowRead` et [`credentials.files`](#sandbox-credentials-files) se résolvent par leur préfixe :

| Préfixe                | Signification                                                                                                 | Exemple                                                                      |
| :--------------------- | :------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------- |
| `/`                    | Chemin absolu à partir de la racine du système de fichiers                                                    | `/tmp/build` reste `/tmp/build`                                              |
| `~/`                   | Relatif au répertoire personnel                                                                               | `~/.kube` devient `$HOME/.kube`                                              |
| `./` ou pas de préfixe | Relatif à la racine du projet pour les paramètres du projet, ou à `~/.claude` pour les paramètres utilisateur | `./output` dans `.claude/settings.json` se résout en `<project-root>/output` |

Le préfixe `//path` pour les chemins absolus fonctionne également. Si vous utilisez `/path` simple en attendant une résolution relative au projet, passez à `./path`. Cette syntaxe diffère des [règles de permission Read et Edit](/docs/fr/permissions#read-and-edit), qui utilisent `//path` pour absolu et `/path` pour relatif au projet : les chemins du système de fichiers du sandbox utilisent des conventions standard, donc `/tmp/build` est un chemin absolu.

Claude Code supprime une barre oblique finale d'un chemin de répertoire, donc `~/.aws` et `~/.aws/` correspondent au même répertoire. Avant v2.1.224, Claude Code transmettait la barre oblique finale au sandbox, et Claude pouvait toujours lire ou écrire des chemins sous une entrée `denyRead` ou `denyWrite` écrite avec une.

Claude Code supprime également un `/**` final, donc `~/build/**` et `~/build` couvrent le même répertoire. Le fait qu'un caractère générique tel que `*` fonctionne dépend de la liste dans laquelle se trouve l'entrée et de la plateforme :

* **`allowWrite` et `denyWrite`** : sur macOS, les caractères génériques fonctionnent. Sur Linux et WSL2, le sandbox monte des chemins concrets, donc Claude Code ignore une entrée qui contient `*`, `?` ou `[` une fois que le `/**` final est supprimé, et cette entrée n'a aucun effet. Claude Code ajoute les chemins de vos règles de permission `Edit` à ces listes, donc la même limite s'applique à elles, et l'onglet **Config** de `/sandbox` avertit des règles de permission `Edit` et `Read` qui contiennent des caractères génériques.
* **`denyRead` et `allowRead`** : les caractères génériques fonctionnent sur chaque plateforme. Sur Linux et WSL2, Claude Code développe une entrée de lecture aux chemins concrets qu'elle correspond, ce qu'il ne fait pas pour les listes d'écriture.

<h3 id="sandbox-filesystem-allowwrite">
  `sandbox.filesystem.allowWrite`
</h3>

Ajoutez des chemins où les commandes en sandbox peuvent écrire, au-delà du répertoire de travail, du répertoire temporaire de la session et des répertoires que vous avez ajoutés avec `--add-dir`, `/add-dir` ou `permissions.additionalDirectories`. Utilisez-le lorsqu'un sous-processus tel que `kubectl` ou un outil de build doit écrire en dehors du projet.

* **Scope** : [`Any file`](#scopes)
* **Type** : tableau de chaînes de chemin, utilisant les [préfixes de chemin du sandbox](#sandbox-path-prefixes)
* **Default** : non défini, donc les commandes en sandbox peuvent écrire dans le répertoire de travail, le répertoire temporaire de la session, les répertoires que vous avez ajoutés avec `--add-dir` ou `/add-dir`, et les répertoires dans [`permissions.additionalDirectories`](#permissions-additionaldirectories)

Ceci permet à un build d'écrire sous `/tmp/build` et permet à `kubectl` de mettre à jour votre kubeconfig :

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "allowWrite": ["/tmp/build", "~/.kube"]
    }
  }
}
```

Claude Code fusionne les entrées dans chaque portée de paramètres que la session charge : les chemins utilisateur, projet, local et gérés se combinent plutôt que de se remplacer, et Claude Code ajoute les chemins de vos règles de permission `Edit(...)` allow. Une entrée `allowWrite` ne peut pas lever un [chemin protégé](/docs/fr/sandboxing#protected-paths).

<h3 id="sandbox-filesystem-denywrite">
  `sandbox.filesystem.denyWrite`
</h3>

Bloquez les commandes en sandbox d'écrire dans des chemins spécifiques, y compris les chemins à l'intérieur d'un répertoire qui est autrement accessible en écriture.

* **Scope** : [`Any file`](#scopes)
* **Type** : tableau de chaînes de chemin, utilisant les [préfixes de chemin du sandbox](#sandbox-path-prefixes)
* **Default** : non défini

Ceci empêche les commandes en sandbox de modifier la configuration système ou d'installer des binaires :

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyWrite": ["/etc", "/usr/local/bin"]
    }
  }
}
```

Claude Code fusionne les entrées dans chaque portée de paramètres que la session charge, et ajoute les chemins de vos règles de permission `Edit(...)` deny.

<h3 id="sandbox-filesystem-denyread">
  `sandbox.filesystem.denyRead`
</h3>

Bloquez les commandes en sandbox de lire des chemins spécifiques, comme les fichiers d'identifiants que la politique de lecture par défaut exposerait autrement. Pour protéger un fichier d'identifiants et le garder utilisable via le proxy du sandbox, consultez [`sandbox.credentials`](#sandbox-credentials) à la place.

* **Scope** : [`Any file`](#scopes)
* **Type** : tableau de chaînes de chemin, utilisant les [préfixes de chemin du sandbox](#sandbox-path-prefixes)
* **Default** : non défini, donc les commandes en sandbox conservent l'[accès en lecture par défaut](/docs/fr/sandboxing#filesystem-isolation), qui inclut les fichiers d'identifiants tels que `~/.aws/credentials`

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyRead": ["~/.aws/credentials"]
    }
  }
}
```

Claude Code fusionne les entrées dans chaque portée de paramètres que la session charge, et ajoute les chemins de vos règles de permission `Read(...)` deny. Lorsque [`filesystem.disabled`](#sandbox-filesystem-disabled) est `true`, Claude Code n'applique pas ces entrées.

<h3 id="sandbox-filesystem-allowread">
  `sandbox.filesystem.allowRead`
</h3>

Rouvrez la lecture pour des chemins spécifiques à l'intérieur d'une région que [`denyRead`](#sandbox-filesystem-denyread) bloque, pour construire un accès en lecture réservé à l'espace de travail. Une entrée `denyRead` exacte ou avec caractères génériques reste bloquée à l'intérieur d'une `allowRead` plus large, comme le montre le [tableau de chevauchement](/docs/fr/sandboxing#configure-sandboxing). Lorsqu'une entrée `denyRead` avec caractères génériques telle que `~/**/.env` correspond à un répertoire, Claude Code bloque également les lectures de son contenu. Avant v2.1.236 sur macOS, Claude Code rouvrait les chemins qu'une entrée `denyRead` avec caractères génériques correspondait partout où une entrée `allowRead` plus large les couvrait, et laissait le contenu d'un répertoire correspondant lisible.

* **Scope** : [`Any file`](#scopes)
* **Type** : tableau de chaînes de chemin, utilisant les [préfixes de chemin du sandbox](#sandbox-path-prefixes)
* **Default** : non défini

Ceci bloque les lectures de votre répertoire personnel sauf le projet lui-même :

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyRead": ["~/"],
      "allowRead": ["."]
    }
  }
}
```

Claude Code résout une entrée `.` à la racine du projet dans les paramètres du projet et à `~/.claude` dans les paramètres utilisateur. Claude Code fusionne les entrées dans chaque fichier de paramètres que la session charge sauf si [`allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly) est défini.

<h3 id="sandbox-filesystem-allowmanagedreadpathsonly">
  `sandbox.filesystem.allowManagedReadPathsOnly`
</h3>

Honorez uniquement les entrées [`allowRead`](#sandbox-filesystem-allowread) qui proviennent des paramètres gérés, afin que les développeurs ne puissent pas rouvrir l'accès en lecture aux chemins que votre organisation a bloqués. Claude Code fusionne toujours les entrées `denyRead` de chaque portée de paramètres que la session charge.

* **Scope** : [`Managed`](#scopes)
* **Type** : Booléen
  * `true` : Claude Code honore uniquement les entrées `allowRead` des paramètres gérés
  * `false` : les entrées `allowRead` fusionnent de chaque portée de paramètres que la session charge
* **Default** : `false`

Ceci bloque les lectures du répertoire personnel, rouvre `~/work`, et empêche les développeurs de rouvrir autre chose :

```json managed-settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyRead": ["~/"],
      "allowRead": ["~/work"],
      "allowManagedReadPathsOnly": true
    }
  }
}
```

Consultez [Keep developers from widening the policy](/docs/fr/sandboxing#keep-developers-from-widening-the-policy).

<h3 id="sandbox-filesystem-disabled">
  `sandbox.filesystem.disabled`
</h3>

Ignorez l'isolation du système de fichiers tout en conservant l'isolation du réseau. Les commandes en sandbox obtiennent un accès en lecture et écriture sans restriction au système de fichiers hôte, et leur sortie réseau reste confinée à [`network.allowedDomains`](#sandbox-network-alloweddomains). Utilisez-le lorsque vous mettez en sandbox pour contrôler où les commandes se connectent plutôt que ce qu'elles écrivent. Nécessite Claude Code v2.1.216 ou ultérieur.

* **Scope** : [`User or managed`](#scopes). Lorsque les paramètres gérés configurent `sandbox.filesystem` du tout, ou listent une entrée `sandbox.credentials.files` avec `"mode": "deny"`, seuls les paramètres gérés peuvent la définir.
* **Type** : Booléen
  * `true` : Claude Code ignore l'isolation du système de fichiers et conserve l'isolation du réseau
  * `false` : l'isolation du système de fichiers reste activée
* **Default** : `false`, donc l'isolation du système de fichiers reste activée

Ceci laisse le système de fichiers ouvert et confine la sortie réseau à GitHub et npm :

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "disabled": true
    },
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org"]
    }
  }
}
```

Avec la couche désactivée, Claude Code n'applique pas les entrées `denyRead` ou `credentials.files` `deny`, tandis que les entrées `credentials.envVars` et les entrées `mask` appliquées continuent de fonctionner. [`autoAllowBashIfSandboxed`](#sandbox-autoallowbashifsandboxed) utilise toujours `true` par défaut, donc définissez-le sur `false` pour continuer à demander. Consultez [Disable filesystem isolation](/docs/fr/sandboxing#disable-filesystem-isolation) pour la liste complète des sources qui peuvent la définir et ce qui change lorsque l'isolation est désactivée. Nécessite Claude Code v2.1.216 ou ultérieur.

<h3 id="sandbox-ignoreviolations">
  `sandbox.ignoreViolations`
</h3>

Silence les rapports de violation du sandbox pour les chemins qu'une commande est censée sonder et se voir refuser, comme un outil qui vérifie `/etc/hosts` au démarrage, afin que ces refus ne s'affichent pas comme des violations ou dans ce que Claude voit. Le sandbox bloque toujours l'accès ; seul le rapport est supprimé. Les clés sont des sous-chaînes à faire correspondre avec la commande, avec `*` correspondant à chaque commande, et les valeurs sont des sous-chaînes de la violation à ignorer pour cette commande, comme un chemin du système de fichiers.

* **Scope** : [`Any file`](#scopes)
* **Type** : objet mappant une sous-chaîne de commande à un tableau de sous-chaînes de violation, généralement des chemins
* **Default** : non défini, donc chaque violation est signalée

```json settings.json theme={null}
{
  "sandbox": {
    "ignoreViolations": {
      "*": ["/etc/hosts"]
    }
  }
}
```

<h3 id="sandbox-enableweakernestedsandbox">
  `sandbox.enableWeakerNestedSandbox`
</h3>

Exécutez le sandbox Linux à l'intérieur d'un conteneur Docker sans privilèges, où bubblewrap ne peut pas monter un `/proc` frais. À la place, le sandbox interne lie-monte le `/proc` existant du conteneur, ce qui expose les informations de processus qu'un montage frais cacherait. Ceci réduit la sécurité ; utilisez-le uniquement lorsque le conteneur externe fournit déjà l'isolation dont vous avez besoin.

* **Scope** : [`Any file`](#scopes)
* **Type** : Booléen
  * `true` : le sandbox interne lie-monte le `/proc` existant du conteneur au lieu de monter un frais
  * `false` : le sandbox monte un `/proc` frais, ce qui ne fonctionne pas dans un conteneur Docker sans privilèges
* **Default** : `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "enableWeakerNestedSandbox": true
  }
}
```

Linux et WSL2 uniquement. Consultez [Bubblewrap fails to start inside a container](/docs/fr/sandboxing#troubleshooting).

<h3 id="sandbox-enableweakernetworkisolation">
  `sandbox.enableWeakerNetworkIsolation`
</h3>

Laissez les commandes en sandbox sur macOS atteindre le service de confiance TLS système, `com.apple.trustd.agent`. Les outils basés sur Go tels que `gh`, `gcloud` et `terraform` en ont besoin pour vérifier les certificats TLS lorsque vous utilisez [`network.httpProxyPort`](#sandbox-network-httpproxyport) avec un proxy MITM et une CA personnalisée. Ceci réduit la sécurité en ouvrant un chemin potentiel d'exfiltration de données via le service de confiance.

* **Scope** : [`Any file`](#scopes)
* **Type** : Booléen
  * `true` : les commandes en sandbox sur macOS peuvent atteindre `com.apple.trustd.agent`
  * `false` : les commandes en sandbox sur macOS ne peuvent pas atteindre le service de confiance TLS système
* **Default** : `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "enableWeakerNetworkIsolation": true
  }
}
```

Si vous n'utilisez pas de proxy MITM, listez plutôt les outils défaillants dans [`excludedCommands`](#sandbox-excludedcommands) ; consultez [Go-based CLIs fail TLS verification on macOS](/docs/fr/sandboxing#troubleshooting).

<h3 id="sandbox-allowappleevents">
  `sandbox.allowAppleEvents`
</h3>

Laissez les commandes en sandbox sur macOS envoyer des Apple Events, que `open`, `osascript` et les outils qui ouvrent des URL dans un navigateur nécessitent ; sans cela, ils échouent avec l'erreur `-600`. Ceci supprime l'isolation de l'exécution du code : les commandes en sandbox peuvent lancer d'autres applications sans sandbox sans invite utilisateur, et peuvent envoyer des commandes AppleScript aux applications en cours d'exécution telles que Terminal, sous réserve de l'invite de consentement d'automatisation macOS par application (TCC).

* **Scope** : [`User or managed`](#scopes)
* **Type** : Booléen
  * `true` : les commandes en sandbox sur macOS peuvent envoyer des Apple Events
  * `false` : les commandes en sandbox sur macOS ne peuvent pas envoyer des Apple Events, donc `open` et `osascript` échouent avec l'erreur `-600`
* **Default** : `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "allowAppleEvents": true
  }
}
```

Pour conserver l'isolation et exécuter quand même un tel outil, ajoutez-le à [`excludedCommands`](#sandbox-excludedcommands) à la place. Consultez [Apple Events on macOS](/docs/fr/sandboxing#security-limitations).

<h3 id="sandbox-ripgrep">
  `sandbox.ripgrep`
</h3>

Pointez le sandbox vers un binaire ripgrep de votre choix au lieu de celui que Claude Code utilise, par exemple lorsque votre plateforme a besoin d'un `rg` construit différemment.

* **Scope** : [`User or managed`](#scopes)
* **Type** : objet avec `command`, le chemin vers le binaire ripgrep, et optionnellement `args`, un tableau d'arguments à ajouter en préfixe
* **Default** : non défini, donc le sandbox utilise le même binaire ripgrep que Claude Code. C'est le binaire fourni sauf si vous définissez [`USE_BUILTIN_RIPGREP`](/docs/fr/env-vars) sur `0`

```json settings.json theme={null}
{
  "sandbox": {
    "ripgrep": {
      "command": "/usr/local/bin/rg"
    }
  }
}
```

<h3 id="sandbox-bwrappath">
  `sandbox.bwrapPath`
</h3>

Pointez le sandbox vers un binaire bubblewrap installé en dehors de `PATH`, comme une copie vendorisée sur un hôte isolé. Claude Code utilise le chemin à la fois pour la vérification de dépendance au démarrage et lorsqu'il enveloppe chaque commande en sandbox.

* **Scope** : [`Managed`](#scopes). Claude Code le lit uniquement à partir des paramètres gérés afin qu'un fichier utilisateur, projet ou local ne puisse pas pointer le sandbox vers un binaire différent.
* **Type** : chaîne, un chemin absolu ; Claude Code abandonne un chemin relatif et revient à la recherche `PATH`
* **Default** : non défini, donc Claude Code trouve `bwrap` sur `PATH`

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "bwrapPath": "/opt/admin/bwrap"
  }
}
```

Linux et WSL2 uniquement.

<h3 id="sandbox-socatpath">
  `sandbox.socatPath`
</h3>

Pointez le proxy réseau du sandbox vers un binaire `socat` installé en dehors de `PATH`.

* **Scope** : [`Managed`](#scopes)
* **Type** : chaîne, un chemin absolu ; Claude Code abandonne un chemin relatif et revient à la recherche `PATH`
* **Default** : non défini, donc Claude Code trouve `socat` sur `PATH`

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "socatPath": "/opt/admin/socat"
  }
}
```

Linux et WSL2 uniquement.

<h3 id="sandbox-credentials">
  `sandbox.credentials`
</h3>

Déclarez les fichiers d'identifiants et les variables d'environnement à [protéger des commandes en sandbox](/docs/fr/sandboxing#protect-credentials). Chaque entrée nomme un fichier `path` ou une variable `name` et un `mode` : `deny` masque l'identifiant à l'intérieur du sandbox, et `mask` affiche aux commandes en sandbox un espace réservé tandis que le [proxy du sandbox](/docs/fr/sandboxing#mask-credentials) substitue la valeur réelle sur les demandes sortantes. Claude Code protège uniquement les entrées que vous listez ; il n'y a pas de liste de refus d'identifiants intégrée.

* **Scope** : [`Any file`](#scopes). Claude Code honore les entrées `mask`, `allowPlaintextInject`, `awsPairs` et `sigv4` uniquement à partir des paramètres utilisateur, gérés et de l'indicateur `--settings`.
* **Type** : objet avec `files`, `envVars`, `allowPlaintextInject`, `awsPairs` et `sigv4`
* **Default** : non défini, donc aucun identifiant n'est protégé

Ceci masque votre fichier d'identifiants AWS et supprime `GITHUB_TOKEN` des commandes en sandbox :

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "files": [{ "path": "~/.aws/credentials", "mode": "deny" }],
      "envVars": [{ "name": "GITHUB_TOKEN", "mode": "deny" }]
    }
  }
}
```

La protection de fichier `deny` fait partie de la couche du système de fichiers, donc elle ne s'applique pas lorsque vous [désactivez l'isolation du système de fichiers](/docs/fr/sandboxing#disable-filesystem-isolation) ; la protection de variable d'environnement s'applique toujours.

<h4 id="invalid-credential-entries-in-managed-settings">
  Entrées d'identifiants invalides dans les paramètres gérés
</h4>

Lorsqu'une entrée `sandbox.credentials` gérée échoue la validation, Claude Code continue de protéger l'identifiant où il peut :

* Une entrée dans `files` ou `envVars` qui a toujours un `path` ou `name` valide et un `mode` de `mask` ou `deny`, comme une dont le motif `extract` n'a pas de groupe de capture, est dégradée à `mode: "deny"` avec un avertissement, donc l'identifiant reste bloqué, pas masqué, jusqu'à ce que vous corrigiez l'entrée. Une entrée `files` dégradée épingle [`filesystem.disabled`](/docs/fr/sandboxing#disable-filesystem-isolation) comme une entrée `deny` explicite, et l'avertissement note que son bloc de lecture n'est pas appliqué si les paramètres gérés désactivent l'isolation du système de fichiers.
* Une entrée avec un `mode` inconnu ou un `path` ou `name` invalide est supprimée.
* Chaque cas avertit ; qu'une entrée soit dégradée ou supprimée, les entrées valides restantes sont toujours appliquées, et une valeur `credentials` entièrement invalide est abandonnée tandis que le reste de `sandbox` s'applique toujours.

S'applique en v2.1.191 et ultérieur ; avant v2.1.221, chaque entrée invalide était supprimée. Pour les autres clés gérées avec gestion par champ, consultez [Invalid entries in managed settings](/docs/fr/managed-settings#invalid-entries-in-managed-settings).

<h3 id="sandbox-credentials-files">
  `sandbox.credentials.files`
</h3>

Protégez les fichiers ou répertoires d'identifiants des commandes en sandbox. Avec `"mode": "deny"`, Claude Code bloque les lectures du chemin à l'intérieur du sandbox, le même bloc de lecture que [`sandbox.filesystem.denyRead`](#sandbox-filesystem-denyread). Avec `"mode": "mask"`, les commandes en sandbox sur Linux et WSL2 lisent une copie sentinelle du fichier, et le proxy du sandbox substitue la valeur réelle sur les demandes sortantes à `injectHosts` de cette entrée ; sur macOS le fichier est illisible à l'intérieur du sandbox à la place. `"mode": "mask"` nécessite Claude Code v2.1.221 ou ultérieur.

* **Scope** : [`Any file`](#scopes). Claude Code abandonne les entrées `mask` de `.claude/settings.json` du projet et de `.claude/settings.local.json` local.
* **Type** : tableau d'objets, chacun avec `path` et un `mode` de `"deny"` ou `"mask"`, plus les [champs mask optionnels pour les fichiers](#mask-fields-for-files)
* **Default** : non défini, donc aucun fichier d'identifiants n'est protégé

Ceci masque votre fichier d'identifiants AWS et masque le fichier hosts `gh`, en substituant la valeur réelle uniquement sur les demandes à `api.github.com` :

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "files": [
        { "path": "~/.aws/credentials", "mode": "deny" },
        { "path": "~/.config/gh/hosts.yml", "mode": "mask", "injectHosts": ["api.github.com"] }
      ]
    }
  }
}
```

Les chemins utilisent les mêmes [préfixes](#sandbox-path-prefixes) que les paramètres `sandbox.filesystem.*`, et Claude Code fusionne les tableaux de chaque portée de paramètres que la session charge. [Protect credentials](/docs/fr/sandboxing#protect-credentials) couvre ce qui s'applique toujours à partir des sources que vous excluez avec `--setting-sources`. `mask` entrées nécessitent Claude Code v2.1.221 ou ultérieur.

La substitution `mask` s'exécute uniquement via le proxy du sandbox, donc définissez [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate), ou [`allowPlaintextInject`](#sandbox-credentials-allowplaintextinject) pour les réseaux de test HTTP simple. `mask` s'applique à un seul fichier, donc listez chaque fichier d'identifiants individuellement. Claude Code accepte mais ignore les champs `mask` sur une entrée `deny`. [Mask credential files](/docs/fr/sandboxing#mask-credential-files) couvre quelles sources de paramètres sont honorées et quand une entrée revient à `deny`.

<span id="sandbox-credentials-files-extract" />

<span id="sandbox-credentials-files-onextractnomatch" />

<span id="sandbox-credentials-files-decode" />

<span id="sandbox-credentials-files-maskclaims" />

<span id="sandbox-credentials-files-maskduplicates" />

<span id="sandbox-credentials-files-injecthosts" />

<h4 id="mask-fields-for-files">
  Champs mask pour les fichiers
</h4>

Une entrée `mask` accepte ces champs optionnels. Sans `extract` ou `decode`, Claude Code remplace le contenu du fichier entier par un sentinelle. Sur macOS avec l'isolation du système de fichiers activée, Claude Code applique une entrée `mask` comme `deny` avant que `extract` ou `decode` s'exécute ; consultez [Mask credential files](/docs/fr/sandboxing#mask-credential-files).

| Champ              | Type                                                                                                                       | Ce qu'il fait                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :----------------- | :------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `extract`          | chaîne, une expression régulière avec au moins un groupe de capture                                                        | Masquez uniquement le texte capturé par le groupe 1 de chaque correspondance, afin que le reste du fichier reste analysable. Avec `decode` également défini, Claude Code vérifie chaque capture comme un JWT possible au lieu de la remplacer directement. Nécessite v2.1.221 ou ultérieur                                                                                                                                                                                                                                                                                                                                                             |
| `onExtractNoMatch` | `"warn"`, `"deny"` ou `"error"` ; par défaut `"warn"`                                                                      | Ce qui se passe lorsque `extract` ou `decode` ne trouve rien à masquer. `warn` laisse le fichier lisible tel quel à l'intérieur du sandbox, `deny` le rend illisible, et `error` arrête la configuration du sandbox jusqu'à ce que vous corrigiez la configuration. Claude Code traite `deny` comme `error` lorsque le bloc de lecture ne serait pas appliqué, car vous [désactivez l'isolation du système de fichiers](/docs/fr/sandboxing#disable-filesystem-isolation) ou une entrée [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread) rouvre le chemin. Nécessite v2.1.221 ou ultérieur ; le cas `decode` nécessite v2.1.224 ou ultérieur |
| `decode`           | la chaîne `"jwt"`                                                                                                          | Trouvez les JSON Web Tokens (JWTs) dans le fichier, avec un motif intégré ou avec `extract` lorsqu'il est défini, vérifiez chaque candidat, et remplacez-le par un faux token structurellement valide, afin que le code à l'intérieur du sandbox qui décode le token continue de fonctionner. Lorsqu'aucun candidat ne se vérifie, `onExtractNoMatch` gouverne le résultat. Nécessite v2.1.224 ou ultérieur                                                                                                                                                                                                                                            |
| `maskClaims`       | tableau de chaînes, au moins un nom de claim ; nécessite `decode`                                                          | Masquez uniquement les claims de charge utile de niveau supérieur nommés à l'intérieur de chaque JWT vérifié et reconstruisez le token autour de la charge utile modifiée, afin que les autres claims restent lisibles. Lorsqu'aucun claim nommé ne correspond, `onExtractNoMatch` gouverne le résultat. Nécessite v2.1.224 ou ultérieur                                                                                                                                                                                                                                                                                                               |
| `maskDuplicates`   | Booléen, par défaut `false`                                                                                                | Remplacez également les copies verbatim de chaque valeur masquée ailleurs dans le fichier, comme un secret collé dans un commentaire. Claude Code correspond aux sous-chaînes brutes, donc réservez-le aux secrets longs et à haute entropie. Consulté uniquement lorsque `extract` ou `decode` est défini. Nécessite v2.1.221 ou ultérieur                                                                                                                                                                                                                                                                                                            |
| `injectHosts`      | tableau de chaînes, chacun un hôte que [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) admet également | Réduisez les hôtes où le proxy du sandbox substitue la valeur réelle. Lorsqu'il n'est pas défini, le proxy la substitue sur les demandes à chaque hôte dans `sandbox.network.allowedDomains`. Nécessite v2.1.221 ou ultérieur                                                                                                                                                                                                                                                                                                                                                                                                                          |

Ceci masque uniquement la valeur `oauth_token` dans le fichier hosts `gh`, remplace chaque autre copie de celle-ci dans le fichier, rend le fichier illisible si le motif ne correspond à rien, et substitue le token réel uniquement sur les demandes à `api.github.com` :

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "files": [
        {
          "path": "~/.config/gh/hosts.yml",
          "mode": "mask",
          "extract": "oauth_token:\\s*(\\S+)",
          "maskDuplicates": true,
          "onExtractNoMatch": "deny",
          "injectHosts": ["api.github.com"]
        }
      ]
    }
  }
}
```

<h3 id="sandbox-credentials-envvars">
  `sandbox.credentials.envVars`
</h3>

Protégez les variables d'environnement des commandes en sandbox. Avec `"mode": "deny"`, Claude Code supprime la variable de l'environnement des commandes en sandbox. Avec `"mode": "mask"`, les commandes en sandbox voient une valeur sentinelle par session, et le proxy du sandbox substitue la valeur réelle sur les demandes sortantes à `injectHosts` de cette entrée, afin que les outils tels que `gh` et `npm` continuent de s'authentifier sans jamais tenir l'identifiant réel. `"mode": "mask"` nécessite Claude Code v2.1.199 ou ultérieur.

* **Scope** : [`Any file`](#scopes). Claude Code abandonne les entrées `mask` de `.claude/settings.json` du projet et de `.claude/settings.local.json` local.
* **Type** : tableau d'objets, chacun avec `name` et un `mode` de `"deny"` ou `"mask"`, plus les [champs mask optionnels pour les variables d'environnement](#mask-fields-for-environment-variables)
* **Default** : non défini, donc aucune variable d'environnement n'est protégée

Ceci supprime `NPM_TOKEN` des commandes en sandbox et masque `GITHUB_TOKEN`, en substituant la valeur réelle uniquement sur les demandes à `api.github.com` :

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "envVars": [
        { "name": "NPM_TOKEN", "mode": "deny" },
        { "name": "GITHUB_TOKEN", "mode": "mask", "injectHosts": ["api.github.com"] }
      ]
    }
  }
}
```

Le `name` doit commencer par une lettre ou un trait de soulignement et contenir uniquement des lettres, des chiffres et des traits de soulignement. Claude Code fusionne les tableaux de chaque portée de paramètres que la session charge, et applique `deny` lorsque la même variable apparaît avec les deux modes. [Protect credentials](/docs/fr/sandboxing#protect-credentials) couvre ce qui s'applique toujours à partir des sources que vous excluez avec `--setting-sources`. `mask` entrées nécessitent Claude Code v2.1.199 ou ultérieur.

La substitution `mask` s'exécute uniquement via le proxy du sandbox, donc définissez [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate), ou [`allowPlaintextInject`](#sandbox-credentials-allowplaintextinject) pour les réseaux de test HTTP simple ; consultez [Mask environment variables](/docs/fr/sandboxing#mask-environment-variables). Claude Code accepte mais ignore les champs `mask` sur une entrée `deny`.

<span id="sandbox-credentials-envvars-extract" />

<span id="sandbox-credentials-envvars-onextractnomatch" />

<span id="sandbox-credentials-envvars-decode" />

<span id="sandbox-credentials-envvars-maskclaims" />

<span id="sandbox-credentials-envvars-injecthosts" />

<h4 id="mask-fields-for-environment-variables">
  Champs mask pour les variables d'environnement
</h4>

Une entrée `mask` accepte ces champs optionnels. Sans `extract` ou `decode`, Claude Code remplace la valeur entière par un sentinelle. `extract` et `decode` ne peuvent pas être combinés sur la même entrée.

| Champ              | Type                                                                                                                       | Ce qu'il fait                                                                                                                                                                                                                                                                                                                                                                                                                            |
| :----------------- | :------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `extract`          | chaîne, une expression régulière avec au moins un groupe de capture                                                        | Masquez uniquement le texte capturé par le groupe 1 de chaque correspondance, comme le mot de passe à l'intérieur d'une chaîne de connexion `DATABASE_URL`, afin que le reste de la valeur reste analysable. Nécessite v2.1.224 ou ultérieur                                                                                                                                                                                             |
| `onExtractNoMatch` | `"warn"`, `"deny"` ou `"error"` ; par défaut `"warn"`. Sur une entrée avec `decode`, seul `"warn"` est accepté             | Ce qui se passe lorsque `extract` ne correspond à rien. `warn` transmet la variable sans masque, `deny` la désactive à l'intérieur du sandbox, et `error` arrête la configuration du sandbox jusqu'à ce que vous corrigiez la configuration. Nécessite v2.1.224 ou ultérieur                                                                                                                                                             |
| `decode`           | la chaîne `"jwt"`                                                                                                          | Vérifiez que la valeur entière est un JWT et remplacez-la par un faux token structurellement valide, afin que le code à l'intérieur du sandbox qui décode le token continue de fonctionner ; le proxy substitue le token réel entier à la sortie. Une valeur qui ne se vérifie pas passe sans masque avec un avertissement. Nécessite v2.1.224 ou ultérieur                                                                              |
| `maskClaims`       | tableau de chaînes, au moins un nom de claim ; nécessite `decode`                                                          | Masquez uniquement les claims de charge utile de niveau supérieur nommés à l'intérieur du JWT décodé et reconstruisez le token autour de la charge utile modifiée, afin que les autres claims restent lisibles. Lorsqu'aucun claim nommé ne correspond, la variable passe sans masque avec un avertissement. Nécessite v2.1.224 ou ultérieur                                                                                             |
| `injectHosts`      | tableau de chaînes, chacun un hôte que [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) admet également | Réduisez les hôtes où le proxy du sandbox substitue la valeur réelle. Lorsqu'il n'est pas défini, le proxy la substitue sur les demandes à chaque hôte dans `sandbox.network.allowedDomains`. Écrivez une destination IPv6 comme l'adresse compressée nue, comme `"::1"`, pas la forme entre crochets ; consultez [IPv6 destinations in `injectHosts`](/docs/fr/sandboxing#ipv6-destinations-in-injecthosts). Nécessite v2.1.199 ou ultérieur |

Ceci masque uniquement le mot de passe à l'intérieur de `DATABASE_URL`, désactive la variable si le motif ne correspond à rien, et masque un JWT dans `SERVICE_JWT` tout en laissant chaque claim sauf `api_key` lisible :

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "envVars": [
        {
          "name": "DATABASE_URL",
          "mode": "mask",
          "extract": "://[^:]+:([^@]+)@",
          "onExtractNoMatch": "deny"
        },
        {
          "name": "SERVICE_JWT",
          "mode": "mask",
          "decode": "jwt",
          "maskClaims": ["api_key"]
        }
      ]
    }
  }
}
```

<h3 id="sandbox-credentials-allowplaintextinject">
  `sandbox.credentials.allowPlaintextInject`
</h3>

Autorisez la substitution `mask` sur les demandes HTTP simples ainsi que sur HTTPS terminé par TLS. Sur HTTP simple, l'identité en amont n'est pas vérifiée et l'identifiant voyage en clair, donc laissez ceci désactivé en dehors des réseaux de test de confiance. Nécessite Claude Code v2.1.199 ou ultérieur.

* **Scope** : [`User or managed`](#scopes)
* **Type** : Booléen
  * `true` : Claude Code autorise la substitution `mask` sur les demandes HTTP simples ainsi que sur HTTPS terminé par TLS
  * `false` : Claude Code autorise la substitution `mask` uniquement sur HTTPS terminé par TLS
* **Default** : `false`

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "allowPlaintextInject": true
    }
  }
}
```

Nécessite Claude Code v2.1.199 ou ultérieur.

<h3 id="sandbox-credentials-awspairs">
  `sandbox.credentials.awsPairs`
</h3>

Groupez les variables d'environnement masquées qui forment un identifiant AWS pour [re-signing SigV4](/docs/fr/sandboxing#re-sign-aws-requests) lorsque votre identifiant vit dans des variables avec des noms non standard. Claude Code lie automatiquement le trio conventionnel `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` et `AWS_SESSION_TOKEN` lorsque vous masquez leurs valeurs entières, donc vous n'avez besoin de cette clé que pour d'autres noms. Nécessite Claude Code v2.1.224 ou ultérieur.

* **Scope** : [`User or managed`](#scopes)
* **Type** : tableau d'objets, chacun avec `accessKeyIdVar`, `secretAccessKeyVar` et optionnellement `sessionTokenVar`, nommant les entrées [`sandbox.credentials.envVars`](#sandbox-credentials-envvars)
* **Default** : non défini, donc seul le trio conventionnel est appairé

Ceci lie trois variables nommées personnalisées en un identifiant AWS pour re-signing :

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "awsPairs": [
        {
          "accessKeyIdVar": "MY_KEY_ID",
          "secretAccessKeyVar": "MY_SECRET_KEY",
          "sessionTokenVar": "MY_SESSION_TOKEN"
        }
      ]
    }
  }
}
```

Chaque variable nommée doit être une entrée `mask` de valeur entière dans [`sandbox.credentials.envVars`](#sandbox-credentials-envvars), sans `extract` ou `decode`, et ne peut remplir qu'un seul rôle dans toutes les paires.

<h3 id="sandbox-credentials-sigv4">
  `sandbox.credentials.sigv4`
</h3>

Choisissez ce que le proxy du sandbox fait avec les formes de demande AWS qu'il [ne peut pas re-signer](/docs/fr/sandboxing#re-sign-aws-requests) : `streaming` pour les téléchargements de streaming aws-chunked, `presigned` pour les URLs présignées, et `sigv4a` pour les signatures asymétriques SigV4A. Ceci s'applique uniquement aux demandes signées avec l'ID de clé d'accès espace réservé d'une paire masquée. Nécessite Claude Code v2.1.224 ou ultérieur.

* **Scope** : [`User or managed`](#scopes)
* **Type** : objet avec `streaming`, `presigned` et `sigv4a`, chacun l'un de :
  * `"deny"` : le proxy échoue la demande
  * `"passthrough"` : le proxy transmet la demande signée avec l'espace réservé masqué, afin que l'outil reçoive le propre rejet d'AWS
* **Default** : non défini, donc chaque forme est `"deny"`

Ceci transmet les téléchargements de streaming au lieu de les échouer au proxy :

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "sigv4": {
        "streaming": "passthrough"
      }
    }
  }
}
```

Avec `deny`, le proxy échoue la demande. Avec `passthrough`, le proxy transmet la demande avec sa signature calculée à partir de l'espace réservé masqué, afin qu'AWS la rejette et que l'outil appelant reçoive la propre réponse d'AWS au lieu d'une erreur de proxy.

<h3 id="sandbox-network">
  `sandbox.network`
</h3>

Contrôlez les hôtes, ports et sockets que les commandes en sandbox peuvent atteindre. Le sandbox achemine le trafic sortant via un proxy qui applique ces listes ; consultez [Network isolation](/docs/fr/sandboxing#network-isolation) pour savoir comment le proxy décide et quand il demande.

* **Scope** : [`Any file`](#scopes). `strictAllowlist`, `allowManagedDomainsOnly` et `tlsTerminate` sont lus à partir de moins de sources, comme leurs entrées le disent.
* **Type** : objet avec les sous-clés ci-dessous
* **Default** : non défini, donc aucun domaine n'est pré-autorisé et le sandbox demande pour chaque nouvel hôte

Ceci pré-autorise GitHub et npm, bloque `uploads.github.com`, et permet aux commandes de se lier à localhost :

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org"],
      "deniedDomains": ["uploads.github.com"],
      "allowLocalBinding": true
    }
  }
}
```

Claude Code fusionne les sous-clés de tableau dans les portées de paramètres et les déduplique, donc un projet peut ajouter des domaines à votre liste utilisateur. Les règles de permission `WebFetch(domain:...)` allow et deny alimentent les mêmes listes allow et deny.

<h3 id="sandbox-network-allowunixsockets">
  `sandbox.network.allowUnixSockets`
</h3>

Listez les chemins de socket Unix que les commandes en sandbox peuvent se connecter sur macOS. Claude Code ignore cette liste sur Linux et WSL2, où le filtre seccomp ne peut pas inspecter les chemins de socket ; utilisez [`allowAllUnixSockets`](#sandbox-network-allowallunixsockets) à la place.

* **Scope** : [`Any file`](#scopes)
* **Type** : tableau de chaînes, chacun un chemin de socket
* **Default** : non défini, donc le sandbox macOS bloque chaque socket Unix

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowUnixSockets": ["~/.ssh/agent-socket"]
    }
  }
}
```

Un chemin de socket peut accorder un accès large : autoriser `/var/run/docker.sock`, par exemple, permet à une commande en sandbox de contrôler le démon Docker. Consultez [Security limitations](/docs/fr/sandboxing#security-limitations).

<h3 id="sandbox-network-allowallunixsockets">
  `sandbox.network.allowAllUnixSockets`
</h3>

Laissez les commandes en sandbox se connecter à chaque socket Unix. Sur Linux et WSL2, le [filtre seccomp](/docs/fr/sandboxing#set-up-linux-and-wsl2) du sandbox bloque les appels `socket(AF_UNIX, ...)`, donc c'est le seul moyen de permettre les sockets Unix là. Lorsque le filtre est manquant, que `/sandbox` rapporte sur son onglet Dependencies, le sandbox ne bloque pas les appels de socket Unix. Consultez [Set up Linux and WSL2](/docs/fr/sandboxing#set-up-linux-and-wsl2) pour savoir d'où vient le filtre.

* **Scope** : [`Any file`](#scopes)
* **Type** : Booléen
  * `true` : les commandes en sandbox peuvent se connecter à chaque socket Unix
  * `false` : le sandbox bloque les connexions de socket Unix : sur macOS sauf les chemins dans `allowUnixSockets`, et sur Linux et WSL2 via le filtre seccomp lorsqu'il est présent
* **Default** : `false`

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowAllUnixSockets": true
    }
  }
}
```

Sur WSL2, `true` rouvre également le socket interop qui lance les binaires Windows tels que `cmd.exe` et `powershell.exe`.

<h3 id="sandbox-network-allowlocalbinding">
  `sandbox.network.allowLocalBinding`
</h3>

Laissez les commandes en sandbox se lier aux ports localhost sur macOS, par exemple pour démarrer un serveur de développement.

* **Scope** : [`Any file`](#scopes)
* **Type** : Booléen
  * `true` : les commandes en sandbox peuvent se lier aux ports localhost sur macOS
  * `false` : les commandes en sandbox sur macOS ne peuvent pas se lier aux ports localhost
* **Default** : `false`

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowLocalBinding": true
    }
  }
}
```

<h3 id="sandbox-network-allowmachlookup">
  `sandbox.network.allowMachLookup`
</h3>

Listez les noms de service XPC et Mach supplémentaires que le sandbox macOS peut rechercher. Les outils qui communiquent via XPC, comme le simulateur iOS ou Playwright, ont besoin de leurs services listés ici.

* **Scope** : [`Any file`](#scopes)
* **Type** : tableau de chaînes, chacun un nom de service ; un seul `*` final correspond à un préfixe, et `"*"` seul correspond à chaque service
* **Default** : non défini

Ceci permet chaque service sous le préfixe `com.apple.coresimulator.` :

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowMachLookup": ["com.apple.coresimulator.*"]
    }
  }
}
```

<h3 id="sandbox-network-alloweddomains">
  `sandbox.network.allowedDomains`
</h3>

Pré-autorisez les domaines pour le trafic sortant des commandes en sandbox, afin que le sandbox ne demande pas pour eux. Les caractères génériques tels que `*.example.com` correspondent aux sous-domaines, et un suffixe `:port` optionnel limite une entrée à un port ; une entrée sans port correspond à chaque port.

* **Scope** : [`Any file`](#scopes). Uniquement les paramètres gérés lorsque [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) est défini.
* **Type** : tableau de chaînes, chacun un domaine, motif avec caractères génériques ou littéral IP, avec un suffixe `:port` optionnel
* **Default** : non défini, donc le sandbox demande la première fois qu'une commande atteint un nouvel hôte

Ceci pré-autorise GitHub sur chaque port, chaque sous-domaine npm, et un hôte API sur le port 443 uniquement :

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org", "api.example.com:443"]
    }
  }
}
```

Écrivez les littéraux IPv6 entre crochets, avec un port optionnel : `"[::1]"` autorise chaque port et `"[::1]:443"` un port. La forme entre crochets nécessite Claude Code v2.1.229 ou ultérieur. Consultez [IPv6 addresses in domain lists](/docs/fr/sandboxing#ipv6-addresses-in-domain-lists).

<h3 id="sandbox-network-denieddomains">
  `sandbox.network.deniedDomains`
</h3>

Bloquez les domaines pour le trafic sortant des commandes en sandbox, en utilisant la même syntaxe de caractères génériques, port et IPv6 que [`allowedDomains`](#sandbox-network-alloweddomains). Un domaine refusé reste bloqué même lorsqu'une entrée `allowedDomains` le correspond également.

* **Scope** : [`Any file`](#scopes)
* **Type** : tableau de chaînes, chacun un domaine, motif avec caractères génériques ou littéral IP, avec un suffixe `:port` optionnel
* **Default** : non défini

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "deniedDomains": ["sensitive.cloud.example.com"]
    }
  }
}
```

Claude Code fusionne cette liste de chaque source de paramètres que la session charge même lorsque `allowManagedDomainsOnly` est défini, donc un développeur peut toujours resserrer la liste de refus. Pour les littéraux IPv6, consultez [IPv6 addresses in domain lists](/docs/fr/sandboxing#ipv6-addresses-in-domain-lists).

Une entrée écrite avec le point final qui marque un nom de domaine pleinement qualifié, comme `example.com.`, bloque les mêmes connexions que `example.com`.

<h3 id="sandbox-network-strictallowlist">
  `sandbox.network.strictAllowlist`
</h3>

Refusez aux commandes en sandbox l'accès aux hôtes en dehors de la liste d'autorisation au lieu de demander l'approbation. La liste d'autorisation est [`allowedDomains`](#sandbox-network-alloweddomains) plus les domaines des règles allow `WebFetch(domain:...)`, ou uniquement les entrées des paramètres gérés lorsque [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) est défini. Nécessite Claude Code v2.1.219 ou ultérieur.

* **Scope** : [`User or managed`](#scopes). Un référentiel ne peut pas l'activer ou le désactiver.
* **Type** : Booléen
  * `true` : Claude Code refuse aux commandes en sandbox l'accès aux hôtes en dehors de la liste d'autorisation
  * `false` : sauf si un autre fichier de paramètres de confiance définit `true`, Claude Code décide un hôte en dehors de la liste d'autorisation par mode de permission au lieu de le refuser directement : en mode auto il vérifie l'hôte par rapport aux [domaines autorisés par commande](/docs/fr/sandboxing#per-command-allowed-domains-in-auto-mode), en mode `dontAsk` il refuse, en mode `bypassPermissions` et en sessions plan-mode de terminal interactif où le bypass est disponible il autorise, et sinon il vous demande
* **Default** : `false`

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "strictAllowlist": true
    }
  }
}
```

Claude Code applique ceci uniquement pour les commandes en sandbox ; les outils en processus tels que `WebFetch` suivent toujours leurs [règles de permission](/docs/fr/sandboxing#permission-rules). Lorsque l'une des sources honorées la définit sur `true`, elle reste activée. Consultez [Network isolation](/docs/fr/sandboxing#network-isolation). Nécessite Claude Code v2.1.219 ou ultérieur.

<h3 id="sandbox-network-allowmanageddomainsonly">
  `sandbox.network.allowManagedDomainsOnly`
</h3>

Verrouillez la liste d'autorisation du réseau à ce que les paramètres gérés définissent. Claude Code honore alors uniquement `allowedDomains` et les règles allow `WebFetch(domain:...)` des paramètres gérés, ignore les domaines des paramètres utilisateur, projet, local et `--settings`, et bloque automatiquement un domaine non autorisé au lieu de demander.

* **Scope** : [`Managed`](#scopes)
* **Type** : Booléen
  * `true` : Claude Code honore uniquement `allowedDomains` et les règles allow `WebFetch(domain:...)` des paramètres gérés et bloque un domaine non autorisé au lieu de demander
  * `false` : les domaines des paramètres utilisateur, projet, local et `--settings` fusionnent dans la liste d'autorisation
* **Default** : `false`

Ceci verrouille la liste d'autorisation à GitHub et npm et ignore tous les domaines que les développeurs ajoutent :

```json managed-settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowManagedDomainsOnly": true,
      "allowedDomains": ["github.com", "*.npmjs.org"]
    }
  }
}
```

Les domaines refusés fusionnent toujours de chaque source que la session charge. Consultez [Keep developers from widening the policy](/docs/fr/sandboxing#keep-developers-from-widening-the-policy).

<h3 id="sandbox-network-httpproxyport">
  `sandbox.network.httpProxyPort`
</h3>

Pointez le sandbox vers votre propre proxy HTTP au lieu de celui que Claude Code exécute. Les organisations font cela pour inspecter le trafic HTTPS, appliquer leurs propres règles de filtrage ou enregistrer chaque demande. Lorsqu'il n'est pas défini, Claude Code démarre son propre proxy pour le trafic HTTP.

* **Scope** : [`Any file`](#scopes)
* **Type** : nombre, un port TCP local
* **Default** : non défini, donc Claude Code exécute son propre proxy

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "httpProxyPort": 8080
    }
  }
}
```

Définissez également [`socksProxyPort`](#sandbox-network-socksproxyport) si votre proxy doit également transporter le trafic SOCKS ; avec un seul des deux défini, Claude Code exécute toujours son propre proxy pour l'autre protocole. Consultez [Custom proxy configuration](/docs/fr/sandboxing#custom-proxy-configuration).

<h3 id="sandbox-network-socksproxyport">
  `sandbox.network.socksProxyPort`
</h3>

Pointez le sandbox vers votre propre proxy SOCKS5 au lieu de celui que Claude Code exécute. Lorsqu'il n'est pas défini, Claude Code démarre son propre proxy pour le trafic SOCKS.

* **Scope** : [`Any file`](#scopes)
* **Type** : nombre, un port TCP local
* **Default** : non défini, donc Claude Code exécute son propre proxy

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "socksProxyPort": 8081
    }
  }
}
```

Consultez [Custom proxy configuration](/docs/fr/sandboxing#custom-proxy-configuration).

<h3 id="sandbox-network-tlsterminate">
  `sandbox.network.tlsTerminate`
</h3>

Faites en sorte que le proxy du sandbox termine TLS afin qu'il puisse lire le contenu des demandes HTTPS. Ceci est expérimental, et la [substitution d'identifiants](/docs/fr/sandboxing#mask-credentials) `mask` l'exige. Définissez `{}` pour générer une autorité de certification éphémère pour la session, ou définissez `caCertPath` et `caKeyPath` pour utiliser la vôtre.

* **Scope** : [`User or managed`](#scopes). Un référentiel ne peut pas l'activer ou fournir une autorité de certification.
* **Type** : objet avec les chaînes optionnelles `caCertPath` et `caKeyPath`, chacun un chemin de fichier
* **Default** : non défini, donc le proxy ne termine pas ou n'inspecte pas TLS

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "tlsTerminate": {}
    }
  }
}
```

Lorsque plus d'une source honorée la définit, Claude Code utilise la valeur de la source la plus prioritaire : paramètres gérés, puis l'indicateur `--settings`, puis paramètres utilisateur. Nécessite Claude Code v2.1.199 ou ultérieur.

<span id="context-and-memory" />

<h2 id="memory-and-context">
  Mémoire et contexte
</h2>

Contrôlez ce que Claude Code charge dans le contexte, comment il compacte et où il conserve la mémoire et les plans. Voir [Gérer le contexte](/docs/fr/context-window) et [Mémoire](/docs/fr/memory).

<h3 id="autocompactenabled">
  `autoCompactEnabled`
</h3>

Faites en sorte que Claude Code [compacte la conversation automatiquement](/docs/fr/context-window#when-your-context-fills-up) lorsque le contexte approche de la limite. Apparaît dans `/config` sous **Auto-compact**, et le basculer là-bas écrit cette clé dans vos paramètres utilisateur.

* **Scope** : [`Any file`](#scopes)
* **Type** : Boolean
  * `true` : Claude Code compacte la conversation automatiquement lorsque le contexte approche de la limite
  * `false` : Claude Code ne compacte pas automatiquement
* **Default** : `true`
* **Per-session overrides** : [`DISABLE_AUTO_COMPACT`](/docs/fr/env-vars) désactive le compactage automatique pour une session ; celui des deux qui le désactive, l'autre ne peut pas le réactiver

```json settings.json theme={null}
{
  "autoCompactEnabled": false
}
```

La commande manuelle `/compact` continue de fonctionner tandis que le compactage automatique est désactivé.

<h3 id="autocompactwindow">
  `autoCompactWindow`
</h3>

Définissez le remplissage du contexte avant que Claude Code [compacte automatiquement](/docs/fr/context-window#when-your-context-fills-up).

* **Scope** : [`Any file`](#scopes)
* **Type** : nombre de tokens, de `100000` à `1000000`. Claude Code plafonne la valeur à la fenêtre de contexte de votre modèle ; l'[aperçu des modèles](https://platform.claude.com/docs/en/about-claude/models/overview) répertorie la fenêtre de chaque modèle
* **Default** : non défini, donc Claude Code choisit une fenêtre adaptée à votre modèle
* **Per-session overrides** : [`--autocompact`](/docs/fr/cli-reference#cli-flags) a la priorité sur cette clé pour une session, et [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/fr/env-vars) a la priorité sur les deux

```json settings.json theme={null}
{
  "autoCompactWindow": 500000
}
```

Définissez-le avec la commande [`/autocompact`](/docs/fr/commands#all-commands), qui écrit cette clé dans vos paramètres utilisateur. [Définir la fenêtre de compactage automatique](/docs/fr/model-config#set-the-auto-compact-window) explique comment la commande, l'indicateur, la variable et le paramètre interagissent.

<h3 id="automemorydirectory">
  `autoMemoryDirectory`
</h3>

Stockez la [mémoire automatique](/docs/fr/memory#storage-location) dans un répertoire de votre choix au lieu de la valeur par défaut par projet.

* **Scope** : [`Any file`](#scopes)
* **Type** : string, un chemin de répertoire absolu ou préfixé par `~/`
* **Default** : non défini, donc Claude Code utilise `~/.claude/projects/<project>/memory/`

```json settings.json theme={null}
{
  "autoMemoryDirectory": "~/my-memory-dir"
}
```

À partir des paramètres de projet ou locaux, Claude Code honore cette clé selon la même [règle de confiance d'espace de travail que les hooks](/docs/fr/permissions#what-runs-before-you-trust-a-folder), car un référentiel cloné peut fournir ces fichiers.

<h3 id="automemoryenabled">
  `autoMemoryEnabled`
</h3>

Activez ou désactivez la [mémoire automatique](/docs/fr/memory#enable-or-disable-auto-memory). Lorsque `false`, Claude ne lit pas et n'écrit pas dans le répertoire de mémoire automatique. Vous pouvez également le basculer avec `/memory` pendant une session, ce qui écrit cette clé dans vos paramètres utilisateur.

* **Scope** : [`Any file`](#scopes)
* **Type** : Boolean
  * `true` : identique à non défini ; la mémoire automatique reste activée sauf si quelque chose qui surclasse cette clé la désactive pour la session, comme `--bare`, le mode sécurisé ou `CLAUDE_CODE_DISABLE_AUTO_MEMORY`
  * `false` : Claude ne lit pas et n'écrit pas dans le répertoire de mémoire automatique
* **Default** : `true`
* **Per-session overrides** : [`CLAUDE_CODE_DISABLE_AUTO_MEMORY`](/docs/fr/env-vars) a la priorité sur cette clé pour une session, dans les deux sens

```json settings.json theme={null}
{
  "autoMemoryEnabled": false
}
```

<h3 id="bashoutputmaxchars">
  `bashOutputMaxChars`
</h3>

Définissez le nombre de caractères de la [sortie d'une commande Bash ou PowerShell réussie que Claude reçoit en ligne](/docs/fr/tools-reference#output-limits). Lorsque la sortie dépasse la limite, Claude Code l'enregistre dans un fichier et Claude reçoit un court aperçu plus le chemin du fichier. Augmentez la limite lorsque la sortie de commande, comme une compilation détaillée ou un journal complet de suite de tests, dépasse régulièrement la valeur par défaut et que vous souhaitez que Claude la lise sans ouvrir le fichier. Nécessite Claude Code v2.1.261 ou ultérieur.

* **Scope** : [`Any file`](#scopes)
* **Type** : nombre de caractères, un entier positif. Claude Code limite la valeur à la plage `4000` à `128000`
* **Default** : non défini, donc Claude reçoit jusqu'à 30 000 caractères en ligne

```json settings.json theme={null}
{
  "bashOutputMaxChars": 100000
}
```

Lorsque vous définissez cette clé, Claude Code ignore la variable d'environnement [`BASH_MAX_OUTPUT_LENGTH`](/docs/fr/env-vars).

<h3 id="claudemd">
  `claudeMd`
</h3>

Injectez les instructions de style CLAUDE.md en tant que mémoire gérée par l'organisation sans déployer un fichier séparé. Claude Code charge le texte en tant qu'entrée de mémoire gérée avant les fichiers CLAUDE.md utilisateur et projet.

* **Scope** : [`Managed`](#scopes)
* **Type** : string, le texte d'un fichier CLAUDE.md ; écrivez-le comme vous le feriez pour le fichier, Markdown inclus, avec les sauts de ligne comme `\n`
* **Default** : non défini

Cet exemple déploie deux règles sous forme de courte liste Markdown :

```json managed-settings.json theme={null}
{
  "claudeMd": "# Engineering rules\n\n- Always run make lint before committing.\n- Never push directly to main."
}
```

Voir [Déployer CLAUDE.md à l'échelle de l'organisation](/docs/fr/memory#deploy-organization-wide-claude-md).

<h3 id="claudemdexcludes">
  `claudeMdExcludes`
</h3>

Ignorez les fichiers `CLAUDE.md` spécifiques lorsque Claude Code charge la [mémoire](/docs/fr/memory#exclude-specific-claude-md-files). Dans un grand monorepo, utilisez-le pour ignorer les fichiers CLAUDE.md d'autres équipes qui ne sont pas pertinents pour votre travail ; [Exclure les fichiers CLAUDE.md non pertinents](/docs/fr/large-codebases#exclude-irrelevant-claude-md-files) dans le guide des grands codebases explique ce cas. Les motifs correspondent aux chemins de fichiers absolus.

* **Scope** : [`Any file`](#scopes)
* **Type** : array of strings, chacun un motif glob ou un chemin absolu
* **Default** : non défini, donc Claude Code charge chaque CLAUDE.md qu'il trouve

```json settings.json theme={null}
{
  "claudeMdExcludes": ["**/vendor/**/CLAUDE.md"]
}
```

Les exclusions s'appliquent uniquement aux fichiers de mémoire utilisateur, projet et local ; les fichiers CLAUDE.md de politique gérée ne peuvent pas être exclus.

<span id="environment-variables" />

<h3 id="env">
  `env`
</h3>

Définissez les variables d'environnement pour chaque session et pour les sous-processus que Claude Code démarre à partir de celle-ci. Toute variable dans la [référence des variables d'environnement](/docs/fr/env-vars) peut aller ici, ce qui est comment vous l'appliquez à chaque session ou la déployez à votre équipe. Les paramètres de projet et locaux ne peuvent pas définir [certaines d'entre elles](#variables-claude-code-ignores-in-env).

* **Scope** : [`Any file`](#scopes)
* **Type** : object mapping variable names to string values
* **Default** : non défini

Cet exemple désactive le compactage automatique et achemine les demandes d'API via un proxy :

```json settings.json theme={null}
{
  "env": {
    "DISABLE_AUTO_COMPACT": "1",
    "ANTHROPIC_BASE_URL": "https://proxy.example.com"
  }
}
```

<h4 id="how-env-values-interact-with-your-shell">
  Comment les valeurs `env` interagissent avec votre shell
</h4>

* Une valeur ici écrase la même variable exportée dans votre shell, et lorsque plus d'un fichier de paramètres définit une variable, celle avec la [plus haute priorité](/docs/fr/settings#settings-precedence) s'applique. [Variables que Claude Code ignore dans `env`](#variables-claude-code-ignores-in-env) répertorie les exceptions pour les paramètres de projet et locaux.
* Pour annuler une exportation de shell, définissez la variable sur `""`. Claude Code traite une valeur vide comme non définie pour la sélection du fournisseur, et les sous-processus héritent de la valeur vide.
* `NO_COLOR` et `FORCE_COLOR` définis ici ne parviennent qu'aux sous-processus. Pour modifier les couleurs de l'interface de Claude Code lui-même, définissez-les dans votre shell avant de lancer `claude`.
* Les valeurs ici sont du texte brut dans le fichier de paramètres et parviennent à chaque sous-processus que Claude Code démarre. Pour un jeton porteur OTLP qui tourne, utilisez [`otelHeadersHelper`](#otelheadershelper) ; pour les identifiants d'API, utilisez [`apiKeyHelper`](#apikeyhelper).

<h4 id="when-claude-code-applies-env-values">
  Quand Claude Code applique les valeurs `env`
</h4>

* À partir des paramètres utilisateur, `--settings` et des paramètres gérés : au démarrage, et à nouveau dans la session en cours lorsqu'une modification enregistrée modifie le `env` fusionné.
* À partir des paramètres de projet et locaux : après avoir approuvé l'espace de travail, ou au démarrage en mode `-p`, qui n'affiche jamais la boîte de dialogue de confiance, et à nouveau lorsqu'une modification enregistrée modifie le `env` fusionné.
* Les variables que Claude Code classe comme sûres, telles que la sélection du modèle, les délais d'attente et les limites, et les basculements de fonctionnalités : au démarrage à partir de chaque fichier de paramètres, à l'exception des [variables que les paramètres de projet et locaux ne peuvent pas définir](#variables-claude-code-ignores-in-env).
* Après avoir [déplacé la session avec `/cd`](/docs/fr/permissions#move-the-session-to-another-directory) sur v2.1.246 ou ultérieur : les valeurs `env` du nouveau répertoire du projet et locales, en plus de celles du répertoire précédent.

<h4 id="variables-claude-code-ignores-in-env">
  Variables que Claude Code ignore dans `env`
</h4>

* Les paramètres de projet et locaux ne peuvent pas définir les variables qu'un référentiel extrait ne devrait pas contrôler ; définissez-les plutôt dans votre shell, vos paramètres utilisateur ou vos paramètres gérés. Claude Code supprime chacun, à l'exception de quelques valeurs qui désactivent la télémétrie, et enregistre un avertissement que vous pouvez voir avec `claude --debug`. Ils incluent :

  * Les variables qui choisissent où Claude Code stocke ou écrit ses propres fichiers : `CLAUDE_CONFIG_DIR`, `CLAUDE_CODE_TMPDIR` et les variables de répertoire du système d'exploitation telles que `HOME`, `TMPDIR`, `TMP`, `TEMP` et la famille `XDG_*`.
  * Les variables qui exportent le contenu de la session : [`OTEL_LOG_RAW_API_BODIES`](/docs/fr/env-vars#variables) et la paire de traçage bêta détaillée `ENABLE_BETA_TRACING_DETAILED` et `BETA_TRACING_ENDPOINT`.
  * Les variables [OpenTelemetry exporter](/docs/fr/monitoring-usage) qui activent la télémétrie, choisissent où elle va ou choisissent quel contenu elle capture :

    * `CLAUDE_CODE_ENABLE_TELEMETRY`, plus la paire de télémétrie améliorée bêta `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` et `ENABLE_ENHANCED_TELEMETRY_BETA`
    * Les sélecteurs d'exportateur `OTEL_LOGS_EXPORTER`, `OTEL_METRICS_EXPORTER` et `OTEL_TRACES_EXPORTER`
    * Les variables de contenu `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_ASSISTANT_RESPONSES`, `OTEL_LOG_TOOL_CONTENT` et `OTEL_LOG_TOOL_DETAILS`
    * Les variables `OTEL_EXPORTER_OTLP_*` dont les noms se terminent par `_ENDPOINT`, `_HEADERS`, `_PROTOCOL`, `_CERTIFICATE`, `_CLIENT_KEY` ou `_INSECURE`, dans les formes génériques et par signal, telles que `OTEL_EXPORTER_OTLP_ENDPOINT` et `OTEL_EXPORTER_OTLP_METRICS_HEADERS`
    * `OTEL_EXPORTER_PROMETHEUS_HOST` et `OTEL_EXPORTER_PROMETHEUS_PORT`

    Seules ces valeurs s'appliquent toujours à partir des paramètres de projet et locaux, car elles désactivent quelque chose : `none` pour les trois sélecteurs d'exportateur, et une valeur désactivée telle que `0` pour `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_TOOL_CONTENT` et `OTEL_LOG_TOOL_DETAILS`. Une telle valeur remplace la même variable dans vos paramètres utilisateur, mais pas celle que l'environnement à partir duquel vous lancez Claude Code, un fichier `--settings` ou les paramètres gérés définissent.

    Lorsqu'un fichier de paramètres de projet ou locaux définit une variable dans ce groupe, une session interactive locale affiche un avis au démarrage. Exécutez `/status` ou `claude doctor` pour voir lesquels Claude Code a ignorés et lesquels ont désactivé la télémétrie ; les deux répertorient les noms, jamais les valeurs. Une exécution non interactive avec `-p` ou une session Agent SDK n'affiche aucun avis, donc vérifiez que votre collecteur reçoit toujours des données après la mise à niveau. S'il ne le fait pas, définissez les variables dans vos paramètres utilisateur, paramètres gérés, l'environnement du travail ou un fichier que vous transmettez avec `--settings`.

    Ignorer ce groupe dans les paramètres de projet et locaux nécessite Claude Code v2.1.282 ou ultérieur.
  * Les variables qui changent la façon dont Claude Code démarre ou se synchronise, telles que `CLAUDE_CODE_PROCESS_WRAPPER`, `CLAUDE_CODE_SYNC_SKILLS`, `CLAUDE_CODE_SYNC_PLUGINS`, `CLAUDE_CODE_PLUGIN_CACHE_DIR` et `CLAUDE_CODE_PLUGIN_SEED_DIR`.

  Avant v2.1.251, les paramètres de projet et locaux pouvaient également définir les variables de cette liste qui choisissent où Claude Code écrit ses fichiers ou qui exportent le contenu de la session, sauf `HOME` et `XDG_CONFIG_HOME`.
* Les variables d'identité que les environnements d'hébergement de Claude Code possèdent, telles que `CLAUDE_CODE_REMOTE` et `CLAUDE_CODE_ACCOUNT_UUID`, sont ignorées de chaque fichier.
* [`CLAUDE_CODE_MESSAGING_SOCKET` et `CLAUDE_CODE_MESSAGING_TOKEN`](/docs/fr/env-vars#variables), que Claude Code exporte lui-même, sont ignorées de chaque fichier. Ignorer la variable de socket nécessite Claude Code v2.1.224 ou ultérieur, et ignorer le jeton nécessite v2.1.228 ou ultérieur.
* [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/fr/sessions#name-the-project-directory-yourself), que Claude Code lit uniquement à partir de l'environnement de lancement, est ignorée de chaque fichier ; nécessite v2.1.234 ou ultérieur.
* [`CLAUDE_CODE_RESTRICTED`](/docs/fr/env-vars#variables), que Claude Code lit uniquement à partir de l'environnement de lancement, est ignorée de chaque fichier.

<h3 id="filecheckpointingenabled">
  `fileCheckpointingEnabled`
</h3>

Faites en sorte que Claude Code crée des instantanés de fichiers avant chaque modification afin que [`/rewind`](/docs/fr/checkpointing) puisse les restaurer. Apparaît dans `/config` sous **Rewind code (checkpoints)**, et le basculer là-bas écrit cette clé dans vos paramètres utilisateur.

* **Scope** : [`Any file`](#scopes)
* **Type** : Boolean
  * `true` : Claude Code crée des instantanés de fichiers avant chaque modification afin que `/rewind` puisse les restaurer
  * `false` : Claude Code ne crée pas d'instantanés de fichiers, donc `/rewind` ne peut pas les restaurer
* **Default** : `true`
* **Per-session overrides** : [`CLAUDE_CODE_DISABLE_FILE_CHECKPOINTING`](/docs/fr/env-vars) désactive les points de contrôle pour une session ; celui des deux qui le désactive, l'autre ne peut pas le réactiver

```json settings.json theme={null}
{
  "fileCheckpointingEnabled": false
}
```

Dans une exécution `-p` ou une session Agent SDK, Claude Code ignore cette clé. Le SDK active les points de contrôle avec son option `enableFileCheckpointing`, et une exécution `-p` nue a besoin de `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=true`. Voir [File checkpointing in the Agent SDK](/docs/fr/agent-sdk/file-checkpointing).

<h3 id="plansdirectory">
  `plansDirectory`
</h3>

Choisissez où Claude Code stocke les fichiers de plan qu'il écrit en [mode plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode). Claude Code résout le chemin par rapport à la racine du projet et conserve la valeur par défaut lorsque le chemin se résout en dehors de celle-ci.

* **Scope** : [`Any file`](#scopes)
* **Type** : string, a path relative to the project root
* **Default** : non défini, donc Claude Code utilise `~/.claude/plans`

```json settings.json theme={null}
{
  "plansDirectory": "./plans"
}
```

<h3 id="skilllistingbudgetfraction">
  `skillListingBudgetFraction`
</h3>

À chaque tour, Claude voit un [listing de vos compétences](/docs/fr/skills#skill-descriptions-are-cut-short) avec leurs descriptions, et Claude Code plafonne ce listing à une part de la fenêtre de contexte. Lorsque le listing dépasse le plafond, Claude Code conserve le nom de chaque compétence mais supprime les descriptions des compétences les moins utilisées, afin que Claude puisse toujours invoquer ces compétences mais soit moins susceptible d'en choisir une de sa propre initiative. Augmentez cette clé pour conserver plus de descriptions visibles au détriment de plus de contexte par tour.

* **Scope** : [`Any file`](#scopes)
* **Type** : number, a fraction greater than `0` and at most `1`
* **Default** : `0.01`, which reserves 1% of the context window

```json settings.json theme={null}
{
  "skillListingBudgetFraction": 0.02
}
```

Pour voir la quantité de contexte utilisée par le listing et les compétences qui contribuent le plus, exécutez `/doctor`.

<h3 id="skilllistingmaxdescchars">
  `skillListingMaxDescChars`
</h3>

À chaque tour, Claude voit un [listing de vos compétences](/docs/fr/skills#skill-descriptions-are-cut-short) qui affiche le texte `description` et `when_to_use` de chaque compétence. Cette clé plafonne le nombre de caractères de ce texte que Claude Code affiche par compétence ; le texte plus long est coupé au plafond.

* **Scope** : [`Any file`](#scopes)
* **Type** : number of characters, a positive integer
* **Default** : `1536`

```json settings.json theme={null}
{
  "skillListingMaxDescChars": 2048
}
```

Augmentez-le pour conserver les descriptions longues intactes au détriment de plus de contexte par tour ; diminuez-le pour adapter plus de compétences sous [`skillListingBudgetFraction`](#skilllistingbudgetfraction).

<h3 id="taskoutputmaxchars">
  `taskOutputMaxChars`
</h3>

<Warning>
  Supprimé dans v2.1.277, ainsi que l'outil `TaskOutput` qu'il dimensionnait. Le définir n'a aucun effet sur les versions actuelles. Claude lit le [fichier de sortie](/docs/fr/tools-reference#background-commands) d'une tâche en arrière-plan avec `Read` à la place.
</Warning>

Jusqu'à v2.1.276, vous définissiez cette clé au nombre de caractères de la [sortie d'une tâche en arrière-plan](/docs/fr/tools-reference#background-commands) que Claude reçoit en ligne lorsqu'il lisait la tâche avec l'outil `TaskOutput`.

<h2 id="interface-and-terminal">
  Interface et terminal
</h2>

Modifiez l'apparence et le comportement de Claude Code dans votre terminal : thème, mode d'éditeur, ligne d'état, spinner, notifications dans la session et accessibilité. Voir [Configuration du terminal](/docs/fr/terminal-config).

<h3 id="askuserquestiontimeout">
  `askUserQuestionTimeout`
</h3>

Permettez à une boîte de dialogue [`AskUserQuestion`](/docs/fr/tools-reference) sans réponse de continuer automatiquement après une période d'inactivité, en soumettant les options que vous aviez déjà sélectionnées. Définissez-le lorsque vous vous éloignez et que vous voulez que Claude continue sans vous. Par défaut, les questions attendent que vous y répondiez. Nécessite Claude Code v2.1.200 ou ultérieur.

* **Portée** : [`Utilisateur ou géré`](#scopes)
* **Type** : chaîne de caractères, l'une de `"60s"`, `"5m"`, `"10m"`, ou `"never"`
* **Défaut** : `"never"`
* **Remplacements par session** : [`CLAUDE_AFK_TIMEOUT_MS`](/docs/fr/env-vars) a la priorité sur cette clé pour une session

```json settings.json theme={null}
{
  "askUserQuestionTimeout": "5m"
}
```

Apparaît dans `/config` sous **Question auto-continue timeout**, qui écrit cette clé dans les paramètres utilisateur ; Claude Code masque la ligne tandis que les paramètres gérés ou l'indicateur `--settings` définissent la clé. Nécessite Claude Code v2.1.200 ou ultérieur.

<h3 id="autocontinueatusagelimit">
  `autoContinueAtUsageLimit`
</h3>

Après qu'une limite d'utilisation de claude.ai arrête votre session, attendez dans la session ouverte et continuez la tâche automatiquement après la réinitialisation. Voir [Désactiver la continuation automatique](/docs/fr/interactive-mode#turn-automatic-continue-off). Nécessite Claude Code v2.1.234 ou ultérieur.

* **Portée** : [`Utilisateur ou géré`](#scopes). Lire à partir des paramètres utilisateur, `--settings`, et des paramètres gérés uniquement. Lorsqu'aucun de ceux-ci ne définit la clé, un fichier de paramètres de projet ou local qui la définit désactive la fonctionnalité plutôt que d'être ignoré.
* **Type** : Booléen
  * `true` : après qu'une limite d'utilisation de claude.ai arrête votre session, Claude Code attend dans la session ouverte et continue la tâche automatiquement après la réinitialisation
  * `false` : Claude Code ne démarre pas l'attente de lui-même. Vous pouvez toujours [démarrer une attente vous-même](/docs/fr/interactive-mode#start-a-wait-yourself) à partir du menu des options de limite d'utilisation
* **Défaut** : `true`

```json settings.json theme={null}
{
  "autoContinueAtUsageLimit": false
}
```

Apparaît dans `/config` sous **Continue automatically at usage limit**, qui écrit cette clé dans les paramètres utilisateur ; Claude Code masque la ligne tandis que les paramètres gérés ou l'indicateur `--settings` définissent la clé.

<h3 id="autoscrollenabled">
  `autoScrollEnabled`
</h3>

Suivez la nouvelle sortie jusqu'au bas de la conversation dans [rendu plein écran](/docs/fr/fullscreen). Désactivez-le pour rester où vous avez fait défiler tandis que Claude continue à travailler ; les invites de permission défilent toujours dans la vue.

* **Portée** : [`N'importe quel fichier`](#scopes)
* **Type** : Booléen
  * `true` : la conversation suit la nouvelle sortie jusqu'au bas
  * `false` : vous restez où vous avez fait défiler tandis que Claude continue à travailler ; les invites de permission apparaissent toujours sous la transcription
* **Défaut** : `true`

```json settings.json theme={null}
{
  "autoScrollEnabled": false
}
```

Apparaît dans `/config` sous **Auto-scroll** lorsque le rendu plein écran est activé, qui écrit cette clé dans les paramètres utilisateur.

<h3 id="axscreenreader">
  `axScreenReader`
</h3>

Rendez la sortie compatible avec les lecteurs d'écran : texte plat sans bordures décoratives ni animations. Le mode lecteur d'écran utilise le moteur de rendu classique, donc le paramètre `tui` n'a aucun effet tant qu'il est actif ; les [sessions en arrière-plan](/docs/fr/agent-view) attachées restent rendues en plein écran.

* **Portée** : [`N'importe quel fichier`](#scopes)
* **Type** : Booléen
  * `true` : Claude Code rend le texte plat sans bordures décoratives ni animations, en utilisant le moteur de rendu classique
  * `false` : Claude Code rend normalement
* **Défaut** : non défini, donc le mode lecteur d'écran est désactivé
* **Remplacements par session** : [`--ax-screen-reader`](/docs/fr/cli-reference#cli-flags) a la priorité sur [`CLAUDE_AX_SCREEN_READER`](/docs/fr/env-vars), et les deux ont la priorité sur cette clé pour une session

```json settings.json theme={null}
{
  "axScreenReader": true
}
```

<h3 id="basheditdiffenabled">
  `bashEditDiffEnabled`
</h3>

Choisissez si Claude Code enregistre les fichiers qu'une commande Bash modifie dans un référentiel Git. Lorsqu'il les enregistre, vous voyez leur diff dans le terminal après la commande, et vos [hooks Bash PostToolUse](/docs/fr/hooks#bash) reçoivent la liste des fichiers modifiés.

Un fichier listé n'est pas toujours un fichier que la commande a modifié. Une modification qu'un autre programme ou un autre appel Bash a effectuée pendant que la commande s'exécutait peut également y apparaître.

Définissez la clé à `true` pour les enregistrer dans chaque mode de permission. Nécessite Claude Code v2.1.269 ou ultérieur.

* **Portée** : [`Utilisateur ou géré`](#scopes). Un `true` compte uniquement à partir de vos paramètres utilisateur, du JSON transmis avec `--settings`, ou des [paramètres gérés](/docs/fr/managed-settings), donc un `true` dans le `.claude/settings.json` ou `.claude/settings.local.json` d'un référentiel ne peut pas activer l'enregistrement. Un `false` dans l'un ou l'autre fichier de référentiel le désactive toujours à moins qu'un fichier de [priorité supérieure](/docs/fr/settings#settings-precedence) ne définisse `true`.
* **Type** : Booléen
* **Défaut** : non défini, donc Claude Code enregistre les modifications en mode auto et en mode `bypassPermissions` lorsqu'il dirige Claude à modifier les fichiers via Bash
* **Remplacements par session** : [`CLAUDE_CODE_BASH_EDIT_DIFF`](/docs/fr/env-vars) a la priorité sur cette clé pour une session

```json settings.json theme={null}
{
  "bashEditDiffEnabled": true
}
```

<h3 id="companyannouncements">
  `companyAnnouncements`
</h3>

Affichez les annonces de votre organisation aux utilisateurs au démarrage. Lorsque vous en listez plus d'une, Claude Code en choisit une au hasard pour chaque session ; au tout premier lancement d'une personne, il affiche la première entrée.

* **Portée** : [`N'importe quel fichier`](#scopes)
* **Type** : tableau de chaînes de caractères
* **Défaut** : non défini, donc aucune annonce ne s'affiche

```json settings.json theme={null}
{
  "companyAnnouncements": [
    "Welcome to Acme Corp! Review our code guidelines at docs.example.com"
  ]
}
```

<h3 id="defaultshell">
  `defaultShell`
</h3>

Choisissez si Bash ou PowerShell exécute les commandes shell que vous tapez avec le préfixe [`!`](/docs/fr/interactive-mode#shell-mode-with-prefix) dans la boîte d'entrée, celles que Claude Code exécute directement et ajoute à la session.

`"powershell"` fonctionne uniquement lorsque l'[outil PowerShell](/docs/fr/tools-reference#powershell-tool) est activé. L'outil est activé par défaut sur Windows sans Git Bash, et sur Windows avec Git Bash pour les comptes claude.ai et Console. Dans les sessions Amazon Bedrock, Google Cloud's Agent Platform, et Microsoft Foundry, et sur macOS, Linux, et WSL, définissez `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` pour activer l'outil. Définissez cette variable à `0` pour désactiver l'outil.

* **Portée** : [`N'importe quel fichier`](#scopes)
* **Type** : chaîne de caractères, l'une de :
  * `"bash"` : Claude Code exécute vos commandes `!` dans Bash
  * `"powershell"` : Claude Code exécute vos commandes `!` dans PowerShell
* **Défaut** : `"bash"`, ou `"powershell"` sur Windows lorsque Bash n'est pas disponible

```json settings.json theme={null}
{
  "defaultShell": "powershell"
}
```

Si le shell que vous nommez n'est pas disponible, Claude Code utilise l'autre : `"powershell"` revient à Bash lorsque l'outil PowerShell est désactivé, et `"bash"` revient à PowerShell lorsque Bash n'est pas installé.

<h3 id="dialogexpiry">
  `dialogExpiry`
</h3>

Définissez la date limite pour les boîtes de dialogue que Claude Code [transfère à un client distant](/docs/fr/remote-control#limitations), tel qu'un hôte Remote Control ou SDK, et pour la boîte de dialogue d'approbation d'un [message inter-session retenu](/docs/fr/cross-session-messaging#control-inbound-messages). Sur Claude Code v2.1.236 ou ultérieur, la même date limite limite l'invite de consentement d'utilisation de crédits Fable [en milieu de session](/docs/fr/model-config#fable-and-usage-credits) dans une session qui peut n'avoir personne au terminal. Lorsqu'aucune réponse n'arrive avant la date limite, Claude Code annule la boîte de dialogue et continue avec sa valeur par défaut sans action. Nécessite Claude Code v2.1.224 ou ultérieur.

* **Portée** : [`Utilisateur ou géré`](#scopes)
* **Type** : chaîne de caractères, l'une de `"60s"`, `"5m"`, `"10m"`, ou `"never"`, qui désactive la date limite
* **Défaut** : `"5m"`
* **Remplacements par session** : [`CLAUDE_CODE_USER_DIALOG_TIMEOUT_MS`](/docs/fr/env-vars) a la priorité sur cette clé pour une session

```json settings.json theme={null}
{
  "dialogExpiry": "10m"
}
```

Les invites de permission et les questions [`AskUserQuestion`](/docs/fr/tools-reference#askuserquestion-tool-behavior) utilisent leurs propres flux et ne sont pas régies par cette date limite. Apparaît dans `/config` sous **Dialog expiry**, qui écrit cette clé dans les paramètres utilisateur ; la ligne nécessite Claude Code v2.1.232 ou ultérieur, et Claude Code la masque tandis que les paramètres gérés ou l'indicateur `--settings` définissent la clé.

<h3 id="editormode">
  `editorMode`
</h3>

Choisissez le mode de liaison de touches pour l'invite d'entrée.

* **Portée** : [`N'importe quel fichier`](#scopes)
* **Type** : chaîne de caractères, l'une de :
  * `"normal"` : liaisons de touches standard dans l'entrée d'invite
  * `"vim"` : édition de style vim avec modes NORMAL, INSERT, et VISUAL
* **Défaut** : `"normal"`

```json settings.json theme={null}
{
  "editorMode": "vim"
}
```

Apparaît dans `/config` sous **Editor mode**, qui écrit cette clé dans les paramètres utilisateur.

<h3 id="emojicompletionenabled">
  `emojiCompletionEnabled`
</h3>

Affichez les suggestions d'emoji lorsque vous tapez `:` plus un raccourci dans l'entrée d'invite, et remplacez un raccourci complété tel que `:heart:` par son emoji. Définissez-le à `false` pour désactiver les deux.

* **Portée** : [`N'importe quel fichier`](#scopes)
* **Type** : Booléen
  * `true` : Claude Code affiche les suggestions d'emoji après `:` et remplace un raccourci complété par son emoji
  * `false` : Claude Code ne suggère pas d'emoji et ne remplace pas les raccourcis
* **Défaut** : `true`

```json settings.json theme={null}
{
  "emojiCompletionEnabled": false
}
```

Voir [Raccourcis emoji](/docs/fr/interactive-mode#emoji-shortcodes). Nécessite Claude Code v2.1.217 ou ultérieur.

<span id="file-suggestion-settings" />

<h3 id="filesuggestion">
  `fileSuggestion`
</h3>

Exécutez votre propre commande pour fournir l'autocomplétion du chemin de fichier `@` au lieu de la suggestion de fichier intégrée. La suggestion intégrée utilise la traversée rapide du système de fichiers ; un grand monorepo peut mieux fonctionner avec l'indexation spécifique au projet, comme un index de fichier pré-construit.

* **Portée** : [`N'importe quel fichier`](#scopes). Sous les [portes de ligne d'état et de suggestion de fichier](#status-line-and-file-suggestion-gates), Claude Code désactive la commande ou exécute uniquement une valeur gérée, et ignore la vôtre sans avertissement.
* **Type** : objet avec `type`, toujours `"command"`, et `command`, la commande shell à exécuter
* **Défaut** : non défini, donc Claude Code utilise la suggestion de fichier intégrée

```json settings.json theme={null}
{
  "fileSuggestion": {
    "type": "command",
    "command": "~/.claude/file-suggestion.sh"
  }
}
```

Après avoir enregistré ceci, tapez `@` suivi d'une partie d'un chemin dans l'invite : les suggestions proviennent de la sortie de votre commande.

<h4 id="command-input-and-output">
  Entrée et sortie de commande
</h4>

Claude Code exécute la commande avec les mêmes variables d'environnement que les [hooks](/docs/fr/hooks), y compris `CLAUDE_PROJECT_DIR`, et arrête d'attendre après cinq secondes. La commande reçoit du JSON sur stdin avec un champ `query` contenant ce que vous avez tapé jusqu'à présent :

```json theme={null}
{"query": "src/comp"}
```

Imprimez les chemins de fichier séparés par des sauts de ligne sur stdout. Claude Code en affiche au maximum 15 :

```text theme={null}
src/components/Button.tsx
src/components/Modal.tsx
src/components/Form.tsx
```

Le script suivant lit la requête et la transmet à un index de fichier de référentiel :

```bash theme={null}
#!/bin/bash
query=$(cat | jq -r '.query')
# Replace your-repo-file-index with your own file search command
your-repo-file-index --query "$query" | head -20
```

<span id="footer-link-badges" />

<h3 id="footerlinksregexes">
  `footerLinksRegexes`
</h3>

Rendez les badges cliquables supplémentaires dans le pied de page sous la boîte d'entrée lorsqu'une regex correspond à la sortie de tour : résultats d'outils, y compris le contenu des fichiers et les pages récupérées, et les propres réponses de Claude. Utilisez-le pour transformer les ID imprimés par les CLI de projet, tels que les outils d'examen et les suivi de problèmes, en liens de session.

* **Portée** : [`Utilisateur ou géré`](#scopes)
* **Type** : tableau d'objets, chacun avec `type` défini à `"regex"`, une regex `pattern`, un modèle `url`, et un `label` optionnel ; les espaces réservés `{name}` dans `url` et `label` sont remplis à partir des groupes de capture nommés dans `pattern`
* **Défaut** : non défini, donc aucun badge ne s'affiche

Cet exemple correspond aux clés de problème telles que `PROJ-1234` et construit chaque lien à partir de la clé capturée :

```json settings.json theme={null}
{
  "footerLinksRegexes": [
    {
      "type": "regex",
      "pattern": "\\b(?<key>PROJ-\\d+)\\b",
      "url": "https://issues.example.com/browse/{key}",
      "label": "{key}"
    }
  ]
}
```

Avec ceci configuré, lorsque `PROJ-1234` apparaît dans un résultat d'outil ou dans la réponse de Claude, un badge `PROJ-1234` apparaît dans le pied de page liant à `https://issues.example.com/browse/PROJ-1234`.

<h4 id="badge-constraints">
  Contraintes de badge
</h4>

L'URL, le label et le nombre de badges de chaque entrée sont limités comme suit :

| Contrainte        | Comportement                                                                                                                                                                                                                |
| :---------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Origine de l'URL  | Les valeurs capturées sont codées en URL et l'URL construite doit partager l'origine littérale du modèle. Une capture peut remplir un segment de chemin ou une valeur de requête mais ne peut pas changer où le lien pointe |
| Longueur de l'URL | Les URL construites plus longues que 2048 caractères sont supprimées                                                                                                                                                        |
| Schéma d'URL      | Doit être `https`, `http`, ou un schéma de lien profond d'éditeur ou d'espace de travail reconnu : `vscode`, `vscode-insiders`, `cursor`, `windsurf`, `zed`, `jetbrains`, `idea`, `slack`, `linear`, `notion`, `figma`      |
| Label             | Par défaut le texte correspondant et est tronqué à 28 colonnes d'affichage                                                                                                                                                  |
| Nombre de badges  | Au maximum 5 badges s'affichent. Le plus ancien est remplacé par les correspondances plus récentes et `/clear` les supprime                                                                                                 |

Lorsqu'un tour se termine, Claude Code correspond à la regex `pattern` de chaque entrée par rapport à la sortie de tour sur le thread principal, donc une regex lente bloque l'interface utilisateur jusqu'à ce qu'elle se termine. Les quantificateurs imbriqués tels que `(a+)+$` peuvent prendre un temps exponentiel par rapport à certaines entrées et geler la session, donc gardez chaque `pattern` linéaire et évitez d'imbriquer `+` ou `*`.

Les badges de pied de page s'affichent aux côtés d'une [ligne d'état personnalisée](/docs/fr/statusline) lorsqu'une est configurée ; ni l'une ni l'autre ne remplace l'autre. Utilisez une ligne d'état pour une ligne pilotée par script qui calcule son propre contenu à partir des données de session, et les badges de pied de page pour transformer les ID de la conversation en liens sans script.

<h3 id="keybindingflavor">
  `keybindingFlavor`
</h3>

<Warning>
  Déprécié depuis v2.1.261 et n'a aucun effet. Les touches d'édition de mots de l'invite suivent toujours les [conventions readline](/docs/fr/interactive-mode#make-ctrl-w-delete-back-to-whitespace), comme dans Bash. Claude Code accepte toujours `keybindingFlavor`, donc un fichier de paramètres qui le définit reste valide.
</Warning>

Dans v2.1.238 à v2.1.260, le définir à `"readline"` faisait que `Ctrl+W` supprime jusqu'à l'espace blanc précédent au lieu de seulement le mot précédent.

* **Portée** : [`N'importe quel fichier`](#scopes)
* **Type** : chaîne de caractères, `"classic"` ou `"readline"`
* **Défaut** : non défini

<h3 id="prefersreducedmotion">
  `prefersReducedMotion`
</h3>

Réduisez ou désactivez les animations de l'interface telles que le spinner, le shimmer, et les effets de flash. Apparaît dans `/config` sous **Reduce motion**.

* **Portée** : [`N'importe quel fichier`](#scopes)
* **Type** : Booléen
  * `true` : Claude Code réduit ou désactive les animations de l'interface telles que le spinner, le shimmer, et les effets de flash
  * `false` : identique à non défini ; Claude Code affiche ses animations
* **Défaut** : `false`

```json settings.json theme={null}
{
  "prefersReducedMotion": true
}
```

<h3 id="promptsuggestionenabled">
  `promptSuggestionEnabled`
</h3>

Affichez ou masquez les [suggestions d'invite](/docs/fr/interactive-mode#prompt-suggestions), les prédictions grisées qui apparaissent dans votre entrée d'invite. Définissez-le à `false`, ou désactivez **Prompt suggestions** dans `/config`, pour les masquer.

* **Portée** : [`N'importe quel fichier`](#scopes)
* **Type** : Booléen
  * `true` : vous voyez les suggestions d'invite dans votre entrée d'invite
  * `false` : Claude Code masque les suggestions d'invite
* **Défaut** : `true`
* **Remplacements par session** : [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](/docs/fr/env-vars) a la priorité sur cette clé pour une session

```json settings.json theme={null}
{
  "promptSuggestionEnabled": false
}
```

Les suggestions d'invite nécessitent un compte claude.ai ou Console avec la télémétrie activée. Sur Amazon Bedrock, Google Cloud's Agent Platform, et Microsoft Foundry, ou avec la télémétrie désactivée, comme par [`DISABLE_TELEMETRY`](/docs/fr/env-vars), cette clé n'a aucun effet et seul `CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=1` les active.

<h3 id="respectgitignore">
  `respectGitignore`
</h3>

Contrôlez si le sélecteur de fichier `@` laisse de côté les fichiers qui correspondent aux motifs `.gitignore`. Apparaît dans `/config` sous **Respect .gitignore in file picker**.

* **Portée** : [`N'importe quel fichier`](#scopes). Lorsqu'aucun fichier de paramètres ne le définit, Claude Code revient à `respectGitignore` dans `~/.claude.json`, que le bouton bascule `/config` écrit.
* **Type** : Booléen
  * `true` : le sélecteur de fichier `@` laisse de côté les fichiers qui correspondent aux motifs `.gitignore`
  * `false` : le sélecteur de fichier `@` inclut les fichiers qui correspondent aux motifs `.gitignore`
* **Défaut** : `true`

```json settings.json theme={null}
{
  "respectGitignore": false
}
```

<h3 id="respondtobashcommands">
  `respondToBashCommands`
</h3>

Choisissez si Claude répond après que vous exécutiez une commande shell avec le préfixe [`!`](/docs/fr/interactive-mode#shell-mode-with-prefix) dans la boîte d'entrée. Par défaut, Claude Code ajoute la sortie de la commande à la conversation et Claude y répond. Définissez cette clé à `false` pour ajouter la sortie au contexte sans réponse, afin que vous puissiez exécuter plusieurs commandes et les interroger ensemble.

* **Portée** : [`N'importe quel fichier`](#scopes)
* **Type** : Booléen
  * `true` : Claude Code ajoute la sortie de la commande à la conversation et Claude y répond
  * `false` : Claude Code ajoute la sortie au contexte sans réponse
* **Défaut** : `true`

```json settings.json theme={null}
{
  "respondToBashCommands": false
}
```

Voir [Mode shell avec préfixe `!`](/docs/fr/interactive-mode#shell-mode-with-prefix).

<h3 id="showclearcontextonplanaccept">
  `showClearContextOnPlanAccept`
</h3>

Lorsque Claude termine un plan en [mode plan](/docs/fr/permission-modes#review-and-approve-a-plan), il affiche un menu d'approbation. La planification peut utiliser beaucoup de contexte, donc cette clé ajoute une première option à ce menu, **Yes, clear context and …**, qui approuve le plan, efface le contexte de la conversation, et commence l'implémentation à partir du plan seul. Le reste du label nomme le mode de permission dans lequel la session continue, et affiche la quantité de contexte que la planification a utilisée.

* **Portée** : [`N'importe quel fichier`](#scopes)
* **Type** : Booléen
  * `true` : le menu d'approbation du plan obtient une première option, **Yes, clear context and …**, qui approuve le plan et efface le contexte de la conversation
  * `false` : le menu d'approbation du plan n'affiche aucune option d'effacement de contexte
* **Défaut** : `false`

```json settings.json theme={null}
{
  "showClearContextOnPlanAccept": true
}
```

<h3 id="showturnduration">
  `showTurnDuration`
</h3>

Affichez ou masquez le message de durée de tour après chaque réponse, tel que « Cooked for 1m 6s · done 6:05 PM ». L'horloge après « done » affiche quand le tour s'est terminé ; [`timeFormat`](#timeformat) et [`timeZone`](#timezone) contrôlent son format et sa zone. Apparaît dans `/config` sous **Show turn duration**.

* **Portée** : [`N'importe quel fichier`](#scopes). Une valeur dans `~/.claude.json` d'une version antérieure s'applique lorsqu'aucun fichier de paramètres ne la définit.
* **Type** : Booléen
  * `true` : vous voyez le message de durée de tour après chaque réponse
  * `false` : Claude Code masque le message de durée de tour
* **Défaut** : `true`

```json settings.json theme={null}
{
  "showTurnDuration": false
}
```

<h3 id="spellcheck">
  `spellcheck`
</h3>

Soulignez les mots mal orthographiés dans l'entrée d'invite au fur et à mesure que vous tapez, en utilisant un correcteur orthographique que vous installez. Claude Code vérifie uniquement le texte dans la boîte d'entrée. [Vérifier l'orthographe au fur et à mesure que vous tapez](/docs/fr/interactive-mode#check-spelling-as-you-type) couvre l'installation d'aspell, hunspell, ou ispell et ce que le correcteur couvre. Nécessite Claude Code v2.1.235 ou ultérieur.

* **Portée** : [`Utilisateur ou géré`](#scopes). Le bloc du niveau le plus élevé qui le définit s'applique dans son ensemble.
* **Type** : objet avec `enabled` (Booléen), `checker` (`"aspell"`, `"hunspell"`, `"ispell"`, ou `"auto"`), `language` (chaîne de caractères, passée au correcteur comme son nom de dictionnaire), et `color` (chaîne de caractères, un nom de couleur de terminal, `#rrggbb`, `rgb(r,g,b)`, `ansi256(n)`, ou `ansi:<name>`)
* **Défaut** : non défini, donc la vérification orthographique est désactivée ; `checker` par défaut à `"auto"`, le premier des trois trouvé sur `PATH` ; `language` par défaut au propre dictionnaire du correcteur ; `color` par défaut à la couleur d'erreur du thème

```json settings.json theme={null}
{
  "spellcheck": { "enabled": true, "language": "en_GB" }
}
```

<h3 id="spinnertipsenabled">
  `spinnerTipsEnabled`
</h3>

Tandis que Claude travaille, la ligne du spinner tourne à travers de courts conseils sur les fonctionnalités de Claude Code, tels que « Use Plan Mode to prepare for a complex request before making changes. Press Shift+Tab twice to enable. » Définissez cette clé à `false` pour les masquer. Apparaît dans `/config` sous **Show tips**.

* **Portée** : [`N'importe quel fichier`](#scopes)
* **Type** : Booléen
  * `true` : vous voyez les conseils dans le spinner tandis que Claude travaille
  * `false` : Claude Code masque les conseils du spinner
* **Défaut** : `true`

```json settings.json theme={null}
{
  "spinnerTipsEnabled": false
}
```

<h3 id="spinnertipsoverride">
  `spinnerTipsOverride`
</h3>

Ajoutez vos propres conseils aux [conseils du spinner](#spinnertipsenabled) que Claude Code affiche tandis que Claude travaille, ou remplacez les conseils intégrés par les vôtres. Claude Code met vos conseils dans la même rotation que les conseils intégrés : il choisit le conseil qui n'a pas été affiché le plus longtemps, ignore les conseils toujours dans leur période de refroidissement, et rompt les égalités par priorité.

Si vous définissez [`spinnerTipsEnabled`](#spinnertipsenabled) à `false`, Claude Code masque tous les conseils, les vôtres inclus.

* **Portée** : [`N'importe quel fichier`](#scopes). Claude Code honore les objets de conseil, `tipsFile`, `label`, et `excludeDefault` à partir des paramètres utilisateur, l'indicateur `--settings`, et les paramètres gérés ; à partir des paramètres de projet et local, il lit uniquement les conseils en chaîne de caractères simples.
* **Type** : objet avec les champs `tips`, `tipsFile`, `label`, et `excludeDefault`, chacun optionnel
* **Défaut** : non défini, donc Claude Code affiche uniquement les conseils intégrés

Les objets de conseil, `tipsFile`, `label`, et la règle de la ligne Portée selon laquelle les paramètres de projet et local ne contribuent que des chaînes de caractères simples nécessitent Claude Code v2.1.247 ou ultérieur. Sur les versions antérieures, l'`excludeDefault` d'un fichier de projet ou local s'applique également.

Chaque entrée `tips` est une chaîne de caractères simple ou un objet avec ces champs :

| Champ              | Requis | Description                                                                                                                                                                                                                                                         |
| :----------------- | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `id`               | Oui    | Jusqu'à 64 lettres, chiffres, `.`, `_`, ou `-`. Claude Code clé l'historique d'affichage du conseil sur lui, donc la période de refroidissement du conseil survit à la réorganisation de la liste. De deux entrées avec le même id, Claude Code utilise la première |
| `text`             | Oui    | Le conseil, une ligne de jusqu'à 500 caractères. Claude Code supprime les échappements ANSI et les caractères de contrôle et réduit les espaces                                                                                                                     |
| `cooldownSessions` | Non    | Sessions Claude Code attend avant d'afficher le conseil à nouveau, `0` à `1000`, par défaut `0`                                                                                                                                                                     |
| `priority`         | Non    | Ordre parmi les conseils qui n'ont pas été affichés aussi longtemps, plus élevé en premier, `-10` à `10`, par défaut `0`                                                                                                                                            |

Claude Code lit une chaîne de caractères simple comme un conseil avec ces valeurs par défaut et un id basé sur la position, donc son historique d'affichage se réinitialise lorsque vous réorganisez la liste. Donnez à un conseil un `id` pour conserver son historique à travers les modifications.

Claude Code lit au maximum 200 conseils à travers `tips` et `tipsFile`, et supprime une entrée invalide avec un avertissement de débogage au lieu de rejeter le fichier de paramètres.

Utilisez les champs restants pour nommer un fichier de conseils, définir le préfixe, et masquer les conseils intégrés :

* `tipsFile` : un chemin absolu ou `~/` vers un fichier JSON local contenant un tableau des mêmes entrées, ou un objet avec un tableau `tips`, jusqu'à 256 KB. Claude Code lit le fichier une fois par processus, donc il charge vos modifications au prochain démarrage. Vous ne pouvez pas le définir via les [paramètres gérés par serveur](/docs/fr/server-managed-settings) ; déployez les `tips` en ligne là, ou déployez le chemin dans un `managed-settings.json` sur disque.
* `label` : le préfixe Claude Code affiche avant les conseils à partir des paramètres utilisateur, `--settings`, et les paramètres gérés, jusqu'à 40 caractères. La valeur par défaut est `Tip`, le même préfixe que les conseils intégrés, et les conseils à partir des paramètres de projet et local l'utilisent toujours.
* `excludeDefault` : définissez-le à `true` pour masquer les conseils intégrés et afficher uniquement les vôtres. Lorsque Claude Code ne peut pas charger l'un de vos conseils, par exemple parce que `tipsFile` n'existe pas ou que chaque entrée est invalide, il conserve la rotation intégrée au lieu d'une rotation vide.

Lorsque plus d'un fichier de paramètres définit la clé, Claude Code affiche les conseils de tous et prend `tipsFile`, `label`, et `excludeDefault` à partir de celui des paramètres gérés, l'indicateur `--settings`, et les paramètres utilisateur qui est le plus élevé en priorité qui définit chacun.

Cet exemple, dans vos paramètres utilisateur, ajoute un conseil en chaîne de caractères simple et un conseil en objet à la rotation sous le préfixe `Acme tip` :

```json settings.json theme={null}
{
  "spinnerTipsOverride": {
    "label": "Acme tip",
    "tips": [
      "Run /review before opening a PR",
      {
        "id": "gateway-errors",
        "text": "Seeing 5xx errors? Check the gateway status page first",
        "cooldownSessions": 5,
        "priority": 2
      }
    ]
  }
}
```

Chaque champ dans l'exemple change une chose sur la façon dont Claude Code affiche les conseils :

* `label` : Claude Code affiche les deux conseils comme `Acme tip: ...` au lieu de `Tip: ...`.
* La chaîne de caractères simple : Claude Code lui donne les valeurs par défaut, donc elle peut réapparaître dans la session suivante.
* `id` : Claude Code clé l'historique d'affichage du deuxième conseil sur `gateway-errors`, donc sa période de refroidissement s'applique toujours après que vous ajoutiez ou réorganisiez les conseils.
* `cooldownSessions` : après que Claude Code affiche le conseil `gateway-errors`, il n'affiche pas ce conseil à nouveau jusqu'à cinq sessions plus tard.
* `priority` : lorsque le conseil `gateway-errors` et un autre conseil n'ont pas été affichés pendant le même nombre de sessions, par exemple lorsqu'aucun n'a été affiché encore, Claude Code affiche `gateway-errors` en premier. La chaîne de caractères simple a la priorité par défaut, `0`.

Tandis que Claude travaille, Claude Code affiche vos conseils dans le spinner avec votre préfixe, tel que `Acme tip: Run /review before opening a PR`.

<h3 id="spinnerverbs">
  `spinnerVerbs`
</h3>

Tandis qu'un tour est en cours, le spinner affiche un verbe rotatif tel que « Accomplishing », « Architecting », ou « Baking ». Utilisez cette clé pour ajouter vos propres verbes à cette rotation ou remplacer la liste intégrée par la vôtre.

* **Portée** : [`N'importe quel fichier`](#scopes)
* **Type** : objet avec un tableau `verbs` de chaînes de caractères et `mode`, l'un de :
  * `"append"` : Claude Code ajoute vos verbes à l'ensemble intégré
  * `"replace"` : Claude Code affiche uniquement vos verbes
* **Défaut** : non défini, donc Claude Code utilise les verbes intégrés

Cet exemple ajoute deux verbes à l'ensemble intégré :

```json settings.json theme={null}
{
  "spinnerVerbs": {
    "mode": "append",
    "verbs": ["Pondering", "Crafting"]
  }
}
```

En mode `"replace"` avec un tableau `verbs` vide, Claude Code conserve les verbes intégrés.

<h3 id="statusline">
  `statusLine`
</h3>

Exécutez votre propre commande pour rendre une [ligne d'état](/docs/fr/statusline) sous l'invite avec un contexte tel que le modèle, le coût, ou la branche git. Les champs optionnels ajustent l'espacement, ajoutent des réexécutions périodiques, et masquent l'indicateur de mode vim intégré lorsque votre script rend `vim.mode` lui-même.

* **Portée** : [`N'importe quel fichier`](#scopes). Lorsque [`allowManagedHooksOnly`](#allowmanagedhooksonly) est activé, ou [`disableAllHooks`](#disableallhooks) est défini en dehors des paramètres gérés, seule la valeur des paramètres gérés s'exécute.
* **Type** : objet avec `type` défini à `"command"` et une chaîne de caractères `command`, plus `padding` optionnel comme un nombre de caractères, `refreshInterval` comme un nombre de secondes, minimum `1`, et `hideVimModeIndicator` comme un Booléen
* **Défaut** : non défini, donc aucune ligne d'état

Cet exemple imprime le nom du modèle et l'utilisation du contexte, et ajoute deux caractères d'espacement horizontal :

```json settings.json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
    "padding": 2
  }
}
```

L'exemple a besoin de [`jq`](https://jqlang.org/) installé et s'exécute dans un shell. Pour les équivalents PowerShell et Git Bash, voir [Configuration Windows](/docs/fr/statusline#windows-configuration) ; pour la configuration complète, voir [Configurer manuellement une ligne d'état](/docs/fr/statusline#manually-configure-a-status-line).

<h3 id="subagentstatusline">
  `subagentStatusLine`
</h3>

Lorsque Claude exécute des [sous-agents](/docs/fr/sub-agents), Claude Code les liste dans un affichage de tâche sous l'invite, une ligne par sous-agent affichant `name · description · token count`. Cette clé vous permet d'exécuter votre propre commande pour réécrire ces lignes, par exemple pour afficher l'utilisation du contexte de chaque sous-agent en pourcentage. À chaque actualisation, Claude Code envoie les lignes visibles comme un objet JSON sur stdin, avec un tableau `tasks` portant l'`id`, `name`, `status`, `model`, `tokenCount`, et plus de chaque sous-agent, et remplace la ligne pour chaque `id` que vous écrivez en retour comme une ligne `{"id", "content"}`. Les lignes que vous n'écrivez pas en retour conservent le rendu par défaut.

* **Portée** : [`N'importe quel fichier`](#scopes). Lorsque [`allowManagedHooksOnly`](#allowmanagedhooksonly) est activé, ou [`disableAllHooks`](#disableallhooks) est défini en dehors des paramètres gérés, seule la valeur des paramètres gérés s'exécute.
* **Type** : objet avec `type` défini à `"command"` et une chaîne de caractères `command`
* **Défaut** : non défini, donc Claude Code rend les lignes par défaut

```json settings.json theme={null}
{
  "subagentStatusLine": {
    "type": "command",
    "command": "jq -c '.tasks[] | {id, content: \"\\(.name): \\(.tokenCount) tokens\"}'"
  }
}
```

Voir [Lignes d'état des sous-agents](/docs/fr/statusline#subagent-status-lines).

<h3 id="syntaxhighlightingdisabled">
  `syntaxHighlightingDisabled`
</h3>

Claude Code colore le code par langage dans les diffs, les blocs de code, et les aperçus de fichier qu'il affiche dans le terminal, avec son surlignage intégré ; aucun plugin ou serveur de langage n'est impliqué. Définissez cette clé à `true` pour les afficher en texte brut à la place, par exemple si les couleurs entrent en conflit avec votre thème de terminal ou ralentissent un lecteur d'écran.

* **Portée** : [`N'importe quel fichier`](#scopes)
* **Type** : Booléen
  * `true` : Claude Code désactive la coloration syntaxique dans les diffs, les blocs de code, et les aperçus de fichier
  * `false` : Claude Code colore la syntaxe
* **Défaut** : `false`

```json settings.json theme={null}
{
  "syntaxHighlightingDisabled": true
}
```

<h3 id="terminalprogressbarenabled">
  `terminalProgressBarEnabled`
</h3>

Certains terminaux peuvent afficher un indicateur de progression sur l'onglet ou dans la barre des tâches pour le programme qui s'exécute en eux. Tandis que Claude travaille, Claude Code signale un état en cours au terminal, afin que vous puissiez voir à partir d'un autre onglet ou fenêtre si la session est toujours occupée. L'indicateur reste visible après la fin du tour tandis que les [sous-agents en arrière-plan](/docs/fr/sub-agents#run-subagents-in-foreground-or-background) ou les [flux de travail dynamiques](/docs/fr/workflows) s'exécutent toujours, et s'efface une fois que la session est inactive.

Claude Code le signale uniquement dans les terminaux qui supportent l'indicateur : ConEmu, Ghostty 1.2.0 ou ultérieur, et iTerm2 3.6.6 ou ultérieur. Définissez cette clé à `false` pour arrêter Claude Code de le signaler. Apparaît dans `/config` sous **Terminal progress bar**.

* **Portée** : [`N'importe quel fichier`](#scopes). Une valeur dans `~/.claude.json` d'une version antérieure s'applique lorsqu'aucun fichier de paramètres ne la définit.
* **Type** : Booléen
  * `true` : vous voyez la barre de progression du terminal dans les terminaux qui la supportent
  * `false` : Claude Code masque la barre de progression du terminal
* **Défaut** : `true`

```json settings.json theme={null}
{
  "terminalProgressBarEnabled": false
}
```

<h3 id="terminaltitlefromrename">
  `terminalTitleFromRename`
</h3>

Claude Code définit le titre de l'onglet de votre terminal. Par défaut, il utilise un titre qu'il génère à partir de la conversation, et une fois que vous donnez à la session un [nom](/docs/fr/sessions#name-your-sessions) avec `/rename` ou `--name`, l'onglet affiche ce nom à la place. Définissez cette clé à `false` pour conserver le titre généré sur l'onglet même après que vous nommez la session. Le nom lui-même s'applique toujours, donc `/resume <name>` et le sélecteur de session le trouvent.

* **Portée** : [`N'importe quel fichier`](#scopes)
* **Type** : Booléen
  * `true` : le titre de l'onglet du terminal affiche le nom de session que vous avez défini
  * `false` : l'onglet conserve le titre que Claude Code génère à partir de votre conversation
* **Défaut** : `true`

```json settings.json theme={null}
{
  "terminalTitleFromRename": false
}
```

Pour arrêter Claude Code de mettre à jour le titre du terminal du tout, définissez [`CLAUDE_CODE_DISABLE_TERMINAL_TITLE`](/docs/fr/env-vars) à `1` à la place.

<h3 id="theme">
  `theme`
</h3>

Choisissez le thème de couleur pour l'interface. Apparaît dans `/config` sous **Theme**.

* **Portée** : [`N'importe quel fichier`](#scopes). Une valeur dans `~/.claude.json` d'une version antérieure s'applique lorsqu'aucun fichier de paramètres ne la définit.
* **Type** : chaîne de caractères, l'une de :
  * `"auto"` : correspond au fond clair ou sombre de votre terminal
  * `"dark"` : le thème sombre
  * `"light"` : le thème clair
  * `"dark-daltonized"` : le thème sombre avec des couleurs adaptées aux daltoniens
  * `"light-daltonized"` : le thème clair avec des couleurs adaptées aux daltoniens
  * `"dark-ansi"` : le thème sombre utilisant uniquement la palette de couleurs ANSI de votre terminal
  * `"light-ansi"` : le thème clair utilisant uniquement la palette de couleurs ANSI de votre terminal
  * `"custom:<slug>"` ou `"custom:<plugin-name>:<slug>"` : un thème personnalisé à partir de `~/.claude/themes/` ou un plugin
* **Défaut** : `"dark"`

```json settings.json theme={null}
{
  "theme": "light-daltonized"
}
```

Voir [Créer un thème personnalisé](/docs/fr/terminal-config#create-a-custom-theme).

<h3 id="timeformat">
  `timeFormat`
</h3>

Choisissez comment Claude Code écrit les heures qu'il affiche dans l'interface, telles que le `done 6:05 PM` à la fin de chaque message de durée de tour et les horodatages dans le [lecteur de transcription](/docs/fr/interactive-mode#transcript-viewer). Pour choisir un préréglage, exécutez `/config` et définissez **Time format**. Nécessite Claude Code v2.1.257 ou ultérieur.

* **Portée** : [`N'importe quel fichier`](#scopes)
* **Type** : chaîne de caractères, l'une de :
  * `"auto"` : identique à non défini ; chaque heure conserve son format intégré, qui suit votre locale sur le message de durée de tour
  * `"12-hour"` : une horloge 12 heures
  * `"24-hour"` : une horloge 24 heures
  * `"24-hour-utc"` : une horloge 24 heures en UTC avec `Z` après les minutes, telle que `18:05Z` ; Claude Code ignore [`timeZone`](#timezone) pour ce préréglage
  * Un motif strftime tel que `"%H:%M"` : Claude Code écrit chaque heure avec le motif. Toute valeur qui contient un `%` est un motif, et toute autre valeur en dehors des préréglages compte comme `"auto"`
* **Défaut** : `"auto"`

```json settings.json theme={null}
{
  "timeFormat": "24-hour"
}
```

`/config` offre uniquement les préréglages, donc pour utiliser un motif strftime, ajoutez la clé à un fichier de paramètres. Cet exemple affiche chaque heure comme une horloge 24 heures à deux chiffres :

```json settings.json theme={null}
{
  "timeFormat": "%H:%M"
}
```

Le message de durée de tour et le lecteur de transcription affichent alors les heures telles que `18:05`. Dans le lecteur de transcription, le motif est l'horodatage entier, donc ajoutez des directives de date lorsque vous voulez la date là. Cet exemple met la date devant l'horloge :

```json settings.json theme={null}
{
  "timeFormat": "%Y-%m-%d %H:%M"
}
```

Les mêmes surfaces affichent alors les heures telles que `2026-09-01 18:05`.

<h3 id="timezone">
  `timeZone`
</h3>

Affichez les heures dans l'interface dans un fuseau horaire autre que celui de votre système. Définissez-le à un [nom de fuseau horaire IANA](https://www.iana.org/time-zones), tel que `"UTC"` ou `"Europe/Dublin"`. Les heures que [`timeFormat`](#timeformat) contrôle affichent alors dans cette zone. Si `timeFormat` est `"24-hour-utc"`, les heures restent en UTC et Claude Code ignore cette clé. `/config` n'a aucune ligne pour cette clé, donc définissez-la dans un fichier de paramètres. Nécessite Claude Code v2.1.257 ou ultérieur.

* **Portée** : [`N'importe quel fichier`](#scopes)
* **Type** : chaîne de caractères, un nom de fuseau horaire IANA. Lorsque Claude Code ne reconnaît pas le nom, il utilise votre fuseau horaire système
* **Défaut** : non défini, donc les heures s'affichent dans votre fuseau horaire système

```json settings.json theme={null}
{
  "timeZone": "Europe/Dublin"
}
```

<h3 id="tui">
  `tui`
</h3>

Choisissez le moteur de rendu de l'interface utilisateur du terminal. Utilisez `"fullscreen"` pour le moteur de rendu [alt-screen](/docs/fr/fullscreen) sans scintillement avec défilement virtualisé, ou `"default"` pour le moteur de rendu classique de l'écran principal. Exécuter `/tui fullscreen` ou `/tui default` écrit cette clé pour vous.

* **Portée** : [`N'importe quel fichier`](#scopes)
* **Type** : chaîne de caractères, l'une de :
  * `"default"` : le moteur de rendu classique de l'écran principal
  * `"fullscreen"` : le moteur de rendu alt-screen sans scintillement avec défilement virtualisé
* **Défaut** : non défini, donc Claude Code [choisit le moteur de rendu pour vous](/docs/fr/fullscreen#fullscreen-by-default)
* **Remplacements par session** : [`CLAUDE_CODE_NO_FLICKER`](/docs/fr/env-vars) et [`CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN`](/docs/fr/env-vars) ont la priorité sur cette clé pour une session : `CLAUDE_CODE_NO_FLICKER=1` active le plein écran, et `CLAUDE_CODE_NO_FLICKER=0` ou `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1` le désactive ; lorsque les deux sont définis, Claude Code le désactive

```json settings.json theme={null}
{
  "tui": "fullscreen"
}
```

Sous tmux `-CC` ou via SSH vers Windows, Claude Code conserve le moteur de rendu classique à moins que vous définissiez `CLAUDE_CODE_NO_FLICKER=1`. Les sessions en arrière-plan ouvertes à partir de la [vue agent](/docs/fr/agent-view) utilisent toujours le moteur de rendu plein écran indépendamment de ce paramètre.

<h3 id="verbose">
  `verbose`
</h3>

Par défaut, la transcription réduit chaque appel d'outil à un court résumé, tel que la commande que Claude a exécutée et un nombre de lignes de sa sortie, et vous appuyez sur `Ctrl+O` pour basculer la transcription entière vers la vue développée lorsque vous voulez les détails. Définissez cette clé à `true` pour afficher l'entrée et la sortie complètes de chaque appel d'outil en ligne au fur et à mesure qu'elles se produisent, ce qui est utile lorsque vous déboguez un hook, un serveur MCP, ou une longue commande shell. Apparaît dans `/config` sous **Verbose output**.

* **Portée** : [`N'importe quel fichier`](#scopes). Une valeur dans `~/.claude.json` d'une version antérieure s'applique lorsqu'aucun fichier de paramètres ne la définit.
* **Type** : Booléen
  * `true` : vous voyez la sortie complète de l'outil
  * `false` : vous voyez les résumés tronqués de la sortie de l'outil
* **Défaut** : `false`
* **Remplacements par session** : [`--verbose`](/docs/fr/cli-reference#cli-flags) a la priorité sur cette clé pour une session

```json settings.json theme={null}
{
  "verbose": true
}
```

Une valeur [`viewMode`](#viewmode) ou une sélection `/focus` collante remplace cette clé chaque session.

<h3 id="viewmode">
  `viewMode`
</h3>

Définissez la vue de transcription dans laquelle Claude Code démarre : `"default"`, `"verbose"`, ou `"focus"`. Lorsqu'elle est définie, elle remplace à la fois la sélection `/focus` collante et le paramètre [`verbose`](#verbose).

* **Portée** : [`N'importe quel fichier`](#scopes)
* **Type** : chaîne de caractères, l'une de :
  * `"default"` : la transcription normale avec sortie d'outil tronquée
  * `"verbose"` : la transcription avec sortie d'outil complète
  * `"focus"` : uniquement votre dernière invite, un résumé d'une ligne des appels d'outil avec les statistiques de diff d'édition, et la réponse finale. La vue focus nécessite le [moteur de rendu plein écran](#tui)
* **Défaut** : non défini, donc le paramètre `verbose` et votre dernier choix `/focus` s'appliquent
* **Remplacements par session** : [`--verbose`](/docs/fr/cli-reference#cli-flags) a la priorité sur cette clé pour une session

```json settings.json theme={null}
{
  "viewMode": "focus"
}
```

<h3 id="viminsertmoderemaps">
  `vimInsertModeRemaps`
</h3>

Mappez les séquences INSERT-mode à deux touches à Escape en [mode d'éditeur vim](/docs/fr/interactive-mode#vim-editor-mode). Chaque clé est exactement deux caractères imprimables tapés en séquence, et `"<Esc>"` est la seule cible supportée ; Claude Code ignore les autres entrées. Nécessite Claude Code v2.1.208 ou ultérieur.

* **Portée** : [`Utilisateur ou géré`](#scopes). Un référentiel ne peut pas remapper vos frappes.
* **Type** : objet mappant une séquence de deux caractères à `"<Esc>"`
* **Défaut** : non défini

```json settings.json theme={null}
{
  "vimInsertModeRemaps": {
    "jj": "<Esc>"
  }
}
```

N'a aucun effet à moins que `editorMode` soit `"vim"`. Voir [Remapper les séquences de touches INSERT-mode](/docs/fr/interactive-mode#remap-insert-mode-key-sequences). Nécessite Claude Code v2.1.208 ou ultérieur.

<h3 id="voice">
  `voice`
</h3>

Activez la [dictée vocale](/docs/fr/voice-dictation) et choisissez comment la touche de dictée se comporte. Claude Code écrit cet objet pour vous lorsque vous exécutez `/voice`.

* **Portée** : [`N'importe quel fichier`](#scopes)
* **Type** : objet avec `enabled` comme Booléen, `autoSubmit` comme Booléen qui s'applique uniquement en mode maintien, et `mode`, l'un de :
  * `"hold"` : vous maintenez la touche de dictée enfoncée en parlant et la relâchez pour arrêter
  * `"tap"` : vous appuyez sur la touche une fois pour commencer l'enregistrement et à nouveau pour envoyer
* **Défaut** : non défini, donc la dictée est désactivée ; lorsque `enabled` est `true` et `mode` est non défini, Claude Code utilise `"hold"`

Cet exemple active la dictée et fait que la touche appuie une fois pour commencer l'enregistrement et à nouveau pour envoyer :

```json settings.json theme={null}
{
  "voice": {
    "enabled": true,
    "mode": "tap"
  }
}
```

`autoSubmit` envoie l'invite lorsque vous relâchez la touche en mode maintien. La dictée vocale nécessite un compte claude.ai.

<h3 id="voiceenabled">
  `voiceEnabled`
</h3>

<Warning>
  Déprécié depuis v2.1.92, lorsque l'objet [`voice`](#voice) l'a remplacé. Claude Code le lit toujours afin que les fichiers de paramètres plus anciens continuent à fonctionner, mais les nouvelles configurations doivent définir `voice.enabled`.
</Warning>

Activez la dictée vocale avec la forme Booléenne simple qui précède l'objet `voice`. Lorsque les deux sont définis, `voice.enabled` s'applique.

* **Portée** : [`N'importe quel fichier`](#scopes)
* **Type** : Booléen
  * `true` : la dictée vocale est activée lorsque vous êtes connecté avec un compte claude.ai et que la politique de votre organisation permet la voix, à moins que `voice.enabled` soit défini
  * `false` : la dictée vocale est désactivée, à moins que `voice.enabled` soit défini
* **Défaut** : non défini

```json settings.json theme={null}
{
  "voiceEnabled": true
}
```

<h3 id="wheelscrollaccelerationenabled">
  `wheelScrollAccelerationEnabled`
</h3>

Accélérez la vitesse de défilement à la molette de la souris lors de défilements rapides en [rendu plein écran](/docs/fr/fullscreen#mouse-wheel-scrolling). Définissez-le à `false` pour un taux de défilement constant par cran de molette.

* **Portée** : [`N'importe quel fichier`](#scopes)
* **Type** : Booléen
  * `true` : Claude Code accélère la vitesse de défilement à la molette de la souris lors de défilements rapides
  * `false` : Claude Code défile à un taux constant par cran de molette
* **Défaut** : `true`

```json settings.json theme={null}
{
  "wheelScrollAccelerationEnabled": false
}
```

<h2 id="git-and-attribution">
  Git et attribution
</h2>

Contrôlez l'attribution que Claude Code ajoute aux commits et aux pull requests et comment cela fonctionne avec git.

<span id="attribution-settings" />

<h3 id="attribution">
  `attribution`
</h3>

Personnalisez l'attribution que Claude Code ajoute aux commits git et aux pull requests. Les commits reçoivent une [git trailer](https://git-scm.com/docs/git-interpret-trailers) telle que `Co-Authored-By` par défaut ; les descriptions de pull request reçoivent du texte brut. Définissez chaque partie séparément avec les sous-clés ci-dessous.

* **Portée** : [`Any file`](#scopes)
* **Type** : objet avec les chaînes `commit` et `pr` et un booléen `sessionUrl`, ou `false` pour masquer toute attribution. La valeur `false` nécessite Claude Code v2.1.281 ou ultérieur ; les versions antérieures la rejettent et [ignorent l'intégralité du fichier de paramètres utilisateur, projet ou local](/docs/fr/settings#fix-a-broken-settings-file) qui la contient
* **Défaut** : non défini, donc Claude Code utilise l'attribution standard affichée sous chaque sous-clé

Pour masquer toute attribution, définissez `attribution` sur `false`. Dans un fichier de paramètres que les versions antérieures lisent également, définissez [`commit`](#attribution-commit) et [`pr`](#attribution-pr) sur des chaînes vides et [`sessionUrl`](#attribution-sessionurl) sur `false` à la place.

Cet exemple remplace l'attribution du commit, supprime l'attribution de la pull request et supprime le lien de session :

```json settings.json theme={null}
{
  "attribution": {
    "commit": "Generated with AI\n\nCo-Authored-By: AI <ai@example.com>",
    "pr": "",
    "sessionUrl": false
  }
}
```

Une fois que vous définissez `commit` ou `pr`, Claude Code ignore le paramètre `includeCoAuthoredBy` déprécié et utilise son texte par défaut pour celui des deux que vous avez laissé non défini.

Claude Code indique à Claude que vos propres instructions concernant l'attribution, telles qu'une règle CLAUDE.md ou [memory](/docs/fr/memory), ont la priorité sur ces lignes de commit et de PR, sauf si la ligne est définie dans les [paramètres gérés](/docs/fr/managed-settings).

<h3 id="includecoauthoredby">
  `includeCoAuthoredBy`
</h3>

<Warning>
  Déprécié depuis v2.0.62, quand [`attribution`](#attribution) l'a remplacé. Claude Code le lit toujours, mais les nouvelles configurations doivent définir `attribution`.
</Warning>

Utilisez [`attribution`](#attribution) à la place, qui remplace cette clé et vous permet de modifier ou masquer la trailer de commit, le texte de la pull request et le lien de session séparément. Claude Code honore toujours `includeCoAuthoredBy: false` des fichiers de paramètres antérieurs à `attribution`, mais l'ignore une fois que vous définissez `attribution.commit` ou `attribution.pr`.

* **Portée** : [`Any file`](#scopes)
* **Type** : booléen
  * `true` : identique à non défini ; Claude Code ajoute la trailer de commit et le texte d'attribution de la pull request
  * `false` : Claude Code omet à la fois la trailer de commit et le texte d'attribution de la pull request, sauf si `attribution` définit `commit` ou `pr`, auquel cas les règles [`attribution`](#attribution) s'appliquent
* **Défaut** : `true`

```json settings.json theme={null}
{
  "includeCoAuthoredBy": false
}
```

Pour masquer toute attribution, voir [`attribution`](#attribution).

<h3 id="includegitinstructions">
  `includeGitInstructions`
</h3>

Claude Code donne à Claude deux éléments de contexte liés à git : ses instructions intégrées sur la façon d'écrire des commits et des pull requests, dans la description de l'outil Bash, et un snapshot d'état git de votre référentiel. Le snapshot contient la branche actuelle, la branche principale, la sortie `git status` et les commits récents. Claude Code le lit au démarrage d'une conversation.

Définissez cette clé sur `false` pour laisser les deux de côté, par exemple lorsque vous utilisez vos propres skills de flux de travail git.

* **Portée** : [`Any file`](#scopes)
* **Type** : booléen
  * `true` : Claude Code inclut ses instructions intégrées de flux de travail de commit et de pull request et le snapshot d'état git. Les sessions cloud n'incluent jamais le snapshot
  * `false` : Claude Code laisse les deux de côté
* **Défaut** : `true`
* **Remplacements par session** : [`CLAUDE_CODE_DISABLE_GIT_INSTRUCTIONS`](/docs/fr/env-vars) a la priorité sur cette clé pour une session

```json settings.json theme={null}
{
  "includeGitInstructions": false
}
```

<h3 id="prurltemplate">
  `prUrlTemplate`
</h3>

Pointez les liens PR que Claude Code rend, dans le badge de pied de page et dans les résumés de résultats d'outils, vers un outil d'examen de code interne au lieu de `github.com`. Claude Code substitue `{host}`, `{owner}`, `{repo}`, `{number}` et `{url}` à partir de l'URL PR. Les liens de [demande de fusion GitLab](/docs/fr/interactive-mode#gitlab-merge-requests) sur les deux surfaces conservent leur URL GitLab.

* **Portée** : [`Any file`](#scopes)
* **Type** : chaîne, un modèle d'URL utilisant l'un des cinq espaces réservés
* **Défaut** : non défini

```json settings.json theme={null}
{
  "prUrlTemplate": "https://reviews.example.com/{owner}/{repo}/pull/{number}"
}
```

Claude Code applique le modèle uniquement aux liens qu'il rend lui-même ; un numéro PR que Claude écrit dans un message, tel que `#123`, reste tel que Claude l'a écrit. Une URL qui n'a pas la forme `/pull/<number>` est laissée inchangée.

<h3 id="attribution-commit">
  `attribution.commit`
</h3>

Définissez le texte d'attribution que Claude Code ajoute aux commits git, y compris les trailers. Définissez-le sur une chaîne vide pour masquer l'attribution du commit.

* **Portée** : [`Any file`](#scopes)
* **Type** : chaîne
* **Défaut** : non défini, donc Claude Code ajoute `Co-Authored-By: <name> <noreply@anthropic.com>`. Le nom est le modèle actif de la session, tel que `Claude Sonnet 5`.
  * Lorsque Claude Code reconnaît le modèle comme un modèle Claude mais ne peut pas confirmer sa version exacte, il écrit `Claude` seul.
  * Lorsqu'il ne peut pas faire correspondre l'ID du modèle à un modèle Claude, tel qu'un modèle tiers servi via une [`ANTHROPIC_BASE_URL`](/docs/fr/env-vars) personnalisée, il écrit `Claude Code`.

Cet exemple remplace la trailer par défaut par une ligne personnalisée et une trailer `Co-Authored-By` personnalisée :

```json settings.json theme={null}
{
  "attribution": {
    "commit": "Generated with AI\n\nCo-Authored-By: AI <ai@example.com>"
  }
}
```

<h3 id="attribution-pr">
  `attribution.pr`
</h3>

Définissez le texte d'attribution que Claude Code ajoute aux descriptions de pull request. Définissez-le sur une chaîne vide pour masquer l'attribution de la pull request.

* **Portée** : [`Any file`](#scopes)
* **Type** : chaîne
* **Défaut** : non défini, donc Claude Code ajoute `🤖 Generated with [Claude Code](https://claude.com/claude-code)`

```json settings.json theme={null}
{
  "attribution": {
    "pr": ""
  }
}
```

<h3 id="attribution-sessionurl">
  `attribution.sessionUrl`
</h3>

Choisissez si Claude Code ajoute le lien de session claude.ai lorsqu'il valide ou ouvre une pull request à partir d'une session [cloud](/docs/fr/claude-code-on-the-web) ou [Remote Control](/docs/fr/remote-control). Claude Code ajoute le lien en tant que trailer `Claude-Session` sur les commits et en tant que lien dans les descriptions de pull request. Définissez-le sur `false` pour omettre le lien.

* **Portée** : [`Any file`](#scopes)
* **Type** : booléen
  * `true` : Claude Code ajoute le lien de session claude.ai lorsqu'il valide ou ouvre une pull request à partir d'une session cloud ou Remote Control
  * `false` : Claude Code omet le lien
* **Défaut** : `true`

```json settings.json theme={null}
{
  "attribution": {
    "sessionUrl": false
  }
}
```

<span id="hook-configuration" />

<span id="hook-and-skill-settings" />

<h2 id="hooks-and-automation">
  Hooks et automatisation
</h2>

Enregistrez des hooks, limitez les hooks qui s'exécutent et contrôlez les workflows. Pour les événements et les charges utiles des hooks, consultez la [référence des hooks](/docs/fr/hooks).

<h3 id="allowedhttphookurls">
  `allowedHttpHookUrls`
</h3>

Limitez les URL que les [hooks HTTP](/docs/fr/hooks#http-hook-fields) peuvent cibler. Lorsque vous définissez cette clé, Claude Code exécute un hook HTTP uniquement si son URL correspond à l'un des modèles et bloque les autres sans les exécuter ; un tableau vide bloque tous les hooks HTTP.

* **Portée** : [`Any file`](#scopes). Les tableaux fusionnent dans les fichiers de paramètres.
* **Type** : tableau de modèles d'URL, avec `*` comme caractère générique
* **Valeur par défaut** : non défini, donc toute URL est autorisée

Cet exemple autorise toute URL sous `https://hooks.example.com/` et toute URL `http://localhost` :

```json settings.json theme={null}
{
  "allowedHttpHookUrls": ["https://hooks.example.com/*", "http://localhost:*"]
}
```

La correspondance du nom d'hôte est insensible à la casse et traite `hooks.example.com.`, avec le point final qui marque un nom de domaine pleinement qualifié, de la même manière que `hooks.example.com`, ce qui est la façon dont le DNS les traite. La liste d'autorisation s'applique aux hooks de toutes les sources, y compris les paramètres gérés.

<h3 id="allowmanagedhooksonly">
  `allowManagedHooksOnly`
</h3>

Limitez l'exécution des hooks aux hooks que votre organisation déploie.

* **Portée** : [`Managed`](#scopes)
* **Type** : Booléen
  * `true` : seuls les hooks gérés s'exécutent, plus les hooks Agent SDK et les hooks des plugins que vos paramètres gérés force-enable. Voir [Ce qui s'exécute sous `allowManagedHooksOnly`](#what-runs-under-allowmanagedhooksonly)
  * `false` : les hooks de chaque portée de paramètres et plugin s'exécutent
* **Valeur par défaut** : non défini, donc les hooks de chaque portée de paramètres et plugin s'exécutent

```json managed-settings.json theme={null}
{
  "allowManagedHooksOnly": true
}
```

<h4 id="what-runs-under-allowmanagedhooksonly">
  Ce qui s'exécute sous `allowManagedHooksOnly`
</h4>

Lorsque vous le définissez à `true`, Claude Code change les hooks et les commandes de type hook qui se chargent :

* **Les hooks gérés et SDK s'exécutent** : les hooks des paramètres gérés et les hooks que l'[Agent SDK](/docs/fr/agent-sdk/overview) enregistre en processus
* **Les hooks des plugins force-enabled s'exécutent** : les hooks des plugins que vos paramètres gérés force-enable via [`enabledPlugins`](#enabledplugins). Claude Code correspond sur l'ID complet `plugin@marketplace`, donc un plugin portant le même nom d'une marketplace différente reste bloqué. Cela vous permet de distribuer des hooks vérifiés via une marketplace d'organisation tout en bloquant tout le reste
* **Tout le reste est bloqué** : les hooks utilisateur, projet et local, les hooks d'autres plugins, et les hooks déclarés dans le frontmatter de l'agent
* **Les plugins sourced par commande sont désactivés** : Claude Code désactive également les plugins avec une [`command` source](/docs/fr/plugins/marketplace-reference#command-plugin-source), y compris les plugins force-enabled dans les `enabledPlugins` gérés, sauf si vous définissez [`disableCommandPluginSources`](#disablecommandpluginsources) explicitement à `false`
* **Les commandes `headersHelper` de la marketplace sont bloquées** : Claude Code bloque également les commandes [`headersHelper`](/docs/fr/plugins/host-marketplace#authenticate-archive-downloads) de la marketplace sauf si [`disableCommandPluginSources`](#disablecommandpluginsources) est explicitement défini à `false`, sauf pour une marketplace que les paramètres gérés eux-mêmes déclarent. Nécessite Claude Code v2.1.238 ou ultérieur
* **La ligne d'état et la suggestion de fichier se limitent aux paramètres gérés** : Claude Code lit [`statusLine`](/docs/fr/statusline), [`fileSuggestion`](#filesuggestion) et [`subagentStatusLine`](/docs/fr/statusline#subagent-status-lines) uniquement à partir des paramètres gérés, en suivant les [portes de ligne d'état et de suggestion de fichier](#status-line-and-file-suggestion-gates)

La commande [`/goal`](/docs/fr/goal) ne peut pas s'exécuter tant que cette clé est définie, car elle dépend des hooks.

<h3 id="disableallhooks">
  `disableAllHooks`
</h3>

Désactivez les [hooks](/docs/fr/hooks#disable-or-remove-hooks), toute [ligne d'état](/docs/fr/statusline) personnalisée et toute commande [suggestion de fichier](#filesuggestion) personnalisée. Utilisez-la pour désactiver tous ces éléments temporairement sans les supprimer de vos paramètres.

* **Portée** : [`Any file`](#scopes). Seuls les paramètres gérés peuvent désactiver les hooks gérés.
* **Type** : Booléen
  * `true` : Claude Code désactive les hooks, toute ligne d'état personnalisée et toute commande de suggestion de fichier personnalisée
  * `false` : les hooks, la ligne d'état et la commande de suggestion de fichier s'exécutent
* **Valeur par défaut** : non défini, donc les hooks s'exécutent

```json settings.json theme={null}
{
  "disableAllHooks": true
}
```

La portée dépend du fichier qui porte la clé :

* **Dans les paramètres gérés** : Claude Code désactive tous les hooks configurés, y compris les hooks gérés, et continue d'exécuter les hooks que l'[Agent SDK](/docs/fr/agent-sdk/overview) enregistre en processus
* **Dans tout autre fichier de paramètres** : Claude Code désactive les hooks utilisateur, projet, local et plugin ; les hooks gérés, les hooks Agent SDK et les hooks des plugins force-enabled dans les [`enabledPlugins`](#enabledplugins) gérés continuent de s'exécuter

Garder les hooks Agent SDK en cours d'exécution lorsque les paramètres gérés définissent cette clé nécessite Claude Code v2.1.242 ou ultérieur.

La commande [`/goal`](/docs/fr/goal) ne peut pas s'exécuter tant que les hooks sont désactivés, et le menu `/hooks` affiche un avis au lieu de vos hooks.

<h4 id="status-line-and-file-suggestion-gates">
  Portes de ligne d'état et de suggestion de fichier
</h4>

Claude Code prend deux décisions pour `statusLine`, `fileSuggestion` et `subagentStatusLine`, dans cet ordre :

* **Désactivé entièrement** : lorsque les paramètres gérés définissent `disableAllHooks`, ou lorsque le dossier n'est pas approuvé selon la même [règle de confiance d'espace de travail que les hooks dans les fichiers de paramètres](/docs/fr/permissions#what-runs-before-you-trust-a-folder)
* **Limité aux paramètres gérés** : lorsque [`allowManagedHooksOnly`](#allowmanagedhooksonly) est défini, lorsque `disableAllHooks` est `true` en dehors des paramètres gérés après l'application de la [précédence des paramètres](/docs/fr/hooks#disable-or-remove-hooks), ou lorsque vous démarrez Claude Code avec `--safe-mode`

Sous limitation, Claude Code exécute une valeur gérée si une est déployée. Sinon, il ignore votre valeur sans avertissement : la ligne d'état est désactivée et l'autocomplétion `@` revient à la suggestion de fichier intégrée.

<h3 id="disableworkflows">
  `disableWorkflows`
</h3>

Désactivez les [workflows dynamiques](/docs/fr/workflows#turn-workflows-off) et les commandes de workflow groupées pour tous ceux que vos paramètres atteignent, comme une organisation via les paramètres gérés. Pour activer ou désactiver les workflows uniquement pour vous-même, utilisez [`enableWorkflows`](#enableworkflows) à la place, que le bouton bascule **Dynamic workflows** dans `/config` écrit dans vos paramètres utilisateur.

* **Portée** : [`Any file`](#scopes)
* **Type** : Booléen
  * `true` : Claude Code désactive les workflows dynamiques et les commandes de workflow groupées pour tous ceux que vos paramètres atteignent
  * `false` : identique à non défini ; que les workflows soient activés dépend alors de [`enableWorkflows`](#enableworkflows) et de la valeur par défaut de votre plan
* **Valeur par défaut** : `false`
* **Remplacements par session** : [`CLAUDE_CODE_DISABLE_WORKFLOWS`](/docs/fr/env-vars) désactive les workflows pour une session ; quel que soit celui des deux qui les désactive, l'autre ne peut pas les réactiver

```json settings.json theme={null}
{
  "disableWorkflows": true
}
```

<h3 id="enableworkflows">
  `enableWorkflows`
</h3>

Activez ou désactivez les [workflows dynamiques](/docs/fr/workflows) pour vous-même lorsque la valeur par défaut de votre plan n'est pas ce que vous voulez. Apparaît dans `/config` comme **Dynamic workflows**, qui écrit cette clé dans vos paramètres utilisateur et la supprime à nouveau lorsque vous basculez vers la valeur par défaut de votre plan. Pour désactiver les workflows pour tout le monde à partir des paramètres gérés, utilisez [`disableWorkflows`](#disableworkflows) à la place.

* **Portée** : [`Any file`](#scopes)
* **Type** : Booléen
  * `true` : Claude Code active les workflows dynamiques pour vous
  * `false` : Claude Code désactive les workflows dynamiques pour vous
* **Valeur par défaut** : non défini, donc les workflows sont activés sauf si vous êtes sur le plan Pro, où ils sont désactivés
* **Remplacements par session** : [`CLAUDE_CODE_DISABLE_WORKFLOWS`](/docs/fr/env-vars) désactive les workflows pour une session, et `true` ici ne peut pas les réactiver tant qu'il est défini

```json settings.json theme={null}
{
  "enableWorkflows": true
}
```

[`disableWorkflows`](#disableworkflows) et la politique de workflows de votre organisation prennent également la priorité : `enableWorkflows: true` ne peut pas réactiver les workflows tant qu'une source les désactive. Claude Code masque la ligne `/config` tant qu'une source autre que vos paramètres utilisateur définit `enableWorkflows`, ou définit `disableWorkflows` à `true`.

<h3 id="hooks">
  `hooks`
</h3>

Exécutez vos propres commandes, invites, agents, requêtes HTTP ou outils MCP en tant que [hooks](/docs/fr/hooks) à des points du cycle de vie de Claude Code, comme avant un appel d'outil ou au démarrage d'une session ; la [référence des hooks](/docs/fr/hooks#hook-events) répertorie tous les événements, leur charge utile et leurs codes de sortie. Chaque événement correspond à une liste de groupes de correspondance, et chaque groupe répertorie les gestionnaires à exécuter lorsque la correspondance s'applique.

* **Portée** : [`Any file`](#scopes). Les hooks fusionnent dans les fichiers plutôt que de se remplacer les uns les autres, et les hooks des paramètres gérés ne peuvent pas être supprimés d'autres fichiers.
* **Type** : objet indexé par [événement hook](/docs/fr/hooks#hook-events) ; chaque valeur est un tableau de groupes `{ "matcher", "hooks" }` dont les entrées `hooks` ont un `type` de `"command"`, `"prompt"`, `"agent"`, `"http"` ou `"mcp_tool"`
* **Valeur par défaut** : non défini, donc aucun hook ne s'exécute

Cet exemple exécute un script avant chaque appel d'outil Bash :

```json settings.json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          { "type": "command", "command": "~/.claude/hooks/check-bash.sh" }
        ]
      }
    ]
  }
}
```

Pour chaque événement, modèle de correspondance et champ de gestionnaire, consultez la [référence des hooks](/docs/fr/hooks#configuration). Pour désactiver les hooks, consultez [`disableAllHooks`](#disableallhooks) ; pour limiter les hooks à ceux que votre organisation déploie, consultez [`allowManagedHooksOnly`](#allowmanagedhooksonly).

<h3 id="httphookallowedenvvars">
  `httpHookAllowedEnvVars`
</h3>

Un [hook HTTP](/docs/fr/hooks#http-hook-fields) peut mettre la valeur d'une variable d'environnement dans un en-tête de requête, par exemple un en-tête `Authorization: Bearer $HOOK_TOKEN`, mais uniquement pour les variables que le hook répertorie dans son propre `allowedEnvVars`. Cette clé définit une limite externe sur cette liste pour chaque hook HTTP : un hook ne peut utiliser une variable que si à la fois son propre `allowedEnvVars` et cette clé la nomment. Utilisez-la pour empêcher un hook de lire un secret qu'il ne devrait pas, même lorsque la définition du hook le demande.

* **Portée** : [`Any file`](#scopes). Les tableaux fusionnent dans les fichiers de paramètres.
* **Type** : tableau de noms de variables d'environnement
* **Valeur par défaut** : non défini, donc la liste `allowedEnvVars` de chaque hook s'applique

Cet exemple limite l'interpolation d'en-tête à `MY_TOKEN` et `HOOK_SECRET` :

```json settings.json theme={null}
{
  "httpHookAllowedEnvVars": ["MY_TOKEN", "HOOK_SECRET"]
}
```

La liste d'autorisation s'applique aux hooks de toutes les sources, y compris les paramètres gérés.

<h3 id="workflowkeywordtriggerenabled">
  `workflowKeywordTriggerEnabled`
</h3>

Choisissez si taper le mot-clé `ultracode` dans une invite déclenche un [workflow dynamique](/docs/fr/workflows#ask-for-a-workflow-in-your-prompt). Définissez-le à `false` pour taper le mot sans en déclencher un.

* **Portée** : [`Any file`](#scopes). Apparaît dans `/config` comme **Ultracode keyword trigger**.
* **Type** : Booléen
  * `true` : taper `ultracode` dans une invite déclenche un workflow dynamique
  * `false` : vous pouvez taper le mot sans en déclencher un
* **Valeur par défaut** : `true`

```json settings.json theme={null}
{
  "workflowKeywordTriggerEnabled": false
}
```

Le paramètre d'effort `ultracode`, `/workflows` et les commandes de workflow enregistrées ne sont pas affectés.

<h3 id="workflowsizeguideline">
  `workflowSizeGuideline`
</h3>

Définissez le [nombre d'agents que Claude vise](/docs/fr/workflows#set-a-size-guideline) dans les workflows dynamiques qu'il écrit. Claude Code envoie la valeur à Claude comme conseil, pas une limite appliquée : `"small"` demande moins de 5 agents, `"medium"` moins de 10, et `"large"` moins de 50. Choisissez `"small"` lorsque vous voulez limiter ce qu'un workflow dépense. Nécessite Claude Code v2.1.219 ou ultérieur.

* **Portée** : [`Any file`](#scopes). Une valeur là prend la priorité sur le choix **Dynamic workflow size** dans `/config`, que Claude Code stocke dans `~/.claude.json`, et Claude Code masque cette ligne tant qu'un fichier de paramètres définit la clé.
* **Type** : chaîne, l'une des :
  * `"unrestricted"` : pas de directive, donc Claude dimensionne le workflow à la tâche
  * `"small"` : Claude vise moins de 5 agents
  * `"medium"` : Claude vise moins de 10 agents
  * `"large"` : Claude vise moins de 50 agents
* **Valeur par défaut** : `"medium"`, ou `"small"` lorsque vous êtes connecté sur un plan Pro avec Claude Code v2.1.271 ou ultérieur

```json settings.json theme={null}
{
  "workflowSizeGuideline": "small"
}
```

Nécessite Claude Code v2.1.219 ou ultérieur ; sur v2.1.202 à v2.1.218, définissez la directive dans `/config` à la place.

<span id="plugin-configuration" />

<span id="manage-plugins" />

<span id="plugin-settings" />

<h2 id="plugins-and-skills">
  Plugins et skills
</h2>

Activez les plugins, enregistrez les marketplaces, limitez les sources de plugins que votre organisation autorise, et contrôlez les skills qui se chargent. Pour installer et créer des plugins, consultez [Plugins](/docs/fr/plugins/overview).

<h3 id="disablebundledskills">
  `disableBundledSkills`
</h3>

Désactivez les [skills](/docs/fr/skills) et workflows inclus avec Claude Code. Claude Code supprime complètement les skills et workflows fournis, tandis que les commandes intégrées telles que `/init` restent tapables mais sont masquées au modèle.

* **Scope** : [`Any file`](#scopes)
* **Type** : Boolean
  * `true` : Claude Code supprime les skills et workflows fournis et masque les commandes intégrées telles que `/init` au modèle
  * `false` : les skills fournis se chargent
* **Default** : unset, donc les skills fournis se chargent
* **Per-session overrides** : [`CLAUDE_CODE_DISABLE_BUNDLED_SKILLS`](/docs/fr/env-vars) défini à `1` désactive les skills fournis pour une session ; celui des deux qui les désactive, l'autre ne peut pas les réactiver

```json settings.json theme={null}
{
  "disableBundledSkills": true
}
```

Les skills provenant de plugins, `.claude/skills/`, et `.claude/commands/` ne sont pas affectés. `/doctor` reste tapable comme les commandes intégrées ; pour le masquer, définissez [`DISABLE_DOCTOR_COMMAND`](/docs/fr/env-vars) à la place.

<h3 id="disableskillshellexecution">
  `disableSkillShellExecution`
</h3>

Désactivez l'exécution de shell en ligne pour `` !`...` `` et ` ```! ` blocs dans [skills](/fr/skills) et commandes personnalisées provenant de sources utilisateur, projet, plugin ou répertoire supplémentaire. Claude Code remplace chaque commande par `[shell command execution disabled by policy]` au lieu de l'exécuter.

* **Scope** : [`Any file`](#scopes). Un `true` dans les paramètres gérés ne peut pas être remplacé par `false` ailleurs.
* **Type** : Boolean
  * `true` : Claude Code remplace chaque commande shell en ligne par `[shell command execution disabled by policy]` au lieu de l'exécuter
  * `false` : le shell en ligne s'exécute
* **Default** : unset, donc le shell en ligne s'exécute

```json settings.json theme={null}
{
  "disableSkillShellExecution": true
}
```

Les skills fournis et les skills déployés via les paramètres gérés ne sont pas affectés.

<h3 id="skilloverrides">
  `skillOverrides`
</h3>

Masquez ou réduisez un [skill](/docs/fr/skills#override-skill-visibility-from-settings) sans modifier son `SKILL.md`. Claude Code applique la valeur sous le nom de chaque skill à la liste des skills que Claude voit et à votre autocomplétion `/`.

* **Scope** : [`Any file`](#scopes). Le menu `/skills` écrit dans `.claude/settings.local.json`.
* **Type** : objet mappant le nom du skill à l'un des éléments suivants :
  * `"on"` : Claude voit le skill et vous pouvez taper `/name`
  * `"name-only"` : Claude voit le skill par son nom sans sa description
  * `"user-invocable-only"` : Claude ne voit pas le skill, mais vous pouvez toujours taper `/name`
  * `"off"` : Claude ne voit pas le skill et `/name` est masqué de l'autocomplétion
* **Default** : unset, donc chaque skill est `"on"`

Cet exemple liste `legacy-context` à Claude par nom uniquement et masque `deploy` à Claude et de l'autocomplétion `/` :

```json settings.json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

Les remplacements ne s'appliquent pas aux skills de plugin, que vous gérez via `/plugin`.

Dans les paramètres gérés et les fichiers passés avec `--settings`, une clé sur un alias de skill fourni, tel que `checkup` pour `/doctor`, s'applique également au skill ; consultez [comment les clés d'alias se combinent avec les clés sur le nom propre du skill](/docs/fr/skills#override-skill-visibility-from-settings).

<h3 id="syncclaudeaiskills">
  `syncClaudeAiSkills`
</h3>

Désactivez le téléchargement des [skills activés pour votre compte claude.ai](/docs/fr/skills#how-synced-skills-behave). Claude Code les télécharge dans `~/.claude/skills/synced/` dans [les sessions de terminal où vous vous connectez avec votre compte claude.ai](/docs/fr/skills#where-synced-skills-load), interactives ou non interactives, et dans les sessions Cowork et cloud. Définissez `false` pour arrêter ce téléchargement et arrêter le chargement des skills qu'il a déjà synchronisés. Claude Code honore uniquement `false` : `true` est identique à unset et n'active pas la synchronisation où elle est autrement désactivée.

* **Scope** : [`User, local, or managed`](#scopes), et fichiers passés avec `--settings`. Un référentiel ne peut pas le désactiver pour vous.
* **Type** : Boolean
  * `false` : Claude Code arrête de télécharger les skills synchronisés et arrête de charger ceux déjà dans `~/.claude/skills/synced/`. Dans les paramètres utilisateur ou gérés, il les déplace également vers `~/.claude/skills/.trash/`
  * `true` : identique à unset
* **Default** : unset, donc les sessions connectées avec votre compte claude.ai synchronisent vos skills

Cet exemple empêche une machine de télécharger les skills du compte dans n'importe quelle session :

```json settings.json theme={null}
{
  "syncClaudeAiSkills": false
}
```

<h3 id="syncclaudeaiplugins">
  `syncClaudeAiPlugins`
</h3>

Désactivez le téléchargement des [plugins activés pour votre compte claude.ai](/docs/fr/plugins/loading#synced-plugins). Claude Code les télécharge dans `~/.claude/plugins/synced/` au début des sessions de terminal où vous vous connectez avec votre compte claude.ai et dans les sessions Cowork, et charge chacun comme `<name>@synced`. Définissez `false` pour arrêter ce téléchargement et arrêter le chargement des plugins qu'il a déjà synchronisés. Claude Code honore uniquement `false` : `true` est identique à unset et n'active pas la synchronisation où elle est autrement désactivée. Nécessite Claude Code v2.1.273 ou ultérieur.

* **Scope** : [`User, local, or managed`](#scopes), et fichiers passés avec `--settings`. Un référentiel ne peut pas le désactiver pour vous.
* **Type** : Boolean
  * `false` : Claude Code arrête de télécharger les plugins synchronisés et arrête de charger ceux déjà dans `~/.claude/plugins/synced/`. Dans les paramètres utilisateur ou gérés, il les déplace également vers `~/.claude/plugins/.trash/`
  * `true` : identique à unset
* **Default** : unset, donc les sessions connectées avec votre compte claude.ai synchronisent vos plugins

Pour désactiver un seul plugin synchronisé plutôt que tous, définissez `"<name>@synced": false` dans [`enabledPlugins`](#enabledplugins).

Cet exemple empêche une machine de télécharger les plugins du compte dans n'importe quelle session :

```json settings.json theme={null}
{
  "syncClaudeAiPlugins": false
}
```

<h3 id="allowedchannelplugins">
  `allowedChannelPlugins`
</h3>

Choisissez dans quels [channel](/docs/fr/channels) les plugins peuvent envoyer des messages dans les sessions de votre organisation. Lorsque vous le définissez, Claude Code utilise votre liste à la place de la liste d'autorisation par défaut d'Anthropic ; chaque entrée nomme un plugin et la marketplace d'où il provient.

* **Scope** : [`Managed`](#scopes)
* **Type** : tableau d'objets, chacun avec des chaînes `marketplace` et `plugin`. Une entrée peut à la place être une chaîne `"plugin@marketplace"` telle que `"telegram@claude-plugins-official"`, que Claude Code traite comme l'objet équivalent. La forme de chaîne nécessite Claude Code v2.1.267 ou ultérieur ; les versions antérieures rejettent la valeur `allowedChannelPlugins` entière lorsqu'elle en contient une
* **Default** : unset, donc Claude Code utilise la liste d'autorisation par défaut d'Anthropic

Cet exemple active les channels et autorise uniquement le plugin Telegram de la marketplace officielle d'Anthropic :

```json managed-settings.json theme={null}
{
  "channelsEnabled": true,
  "allowedChannelPlugins": [
    { "marketplace": "claude-plugins-official", "plugin": "telegram" }
  ]
}
```

Un tableau vide bloque tous les plugins de channel.

Cette clé prend effet une fois que les channels passent la porte [`channelsEnabled`](#channelsenabled) pour le compte : sur les plans Team et Enterprise, et sur les comptes Console avec des paramètres gérés, cela signifie `channelsEnabled: true`. Consultez [Restreindre les plugins de channel qui peuvent s'exécuter](/docs/fr/channels#restrict-which-channel-plugins-can-run).

<h3 id="blockedmarketplaces">
  `blockedMarketplaces`
</h3>

Bloquez les sources de marketplace de plugins pour votre organisation. Claude Code vérifie la liste de blocage lors de l'ajout de marketplace et lors de l'installation, la mise à jour, l'actualisation et la mise à jour automatique du plugin, donc une marketplace que quelqu'un a ajoutée avant que vous ne définissiez la politique ne peut pas être utilisée pour récupérer des plugins non plus. Les sources bloquées sont vérifiées avant le téléchargement, donc elles ne touchent jamais le système de fichiers.

Si vous définissez cette clé dans la [console d'administration claude.ai](/docs/fr/server-managed-settings), claude.ai l'applique également lorsque quelqu'un de votre organisation ajoute une marketplace à partir d'un référentiel git sur claude.ai, comme [Comment les restrictions fonctionnent](/docs/fr/plugins/org#restrict-what-users-can-install) le décrit.

* **Scope** : [`Managed`](#scopes)
* **Type** : tableau d'objets source de marketplace, dans les mêmes formes que [`strictKnownMarketplaces`](#allowed-source-types)
* **Default** : unset, donc aucune marketplace n'est bloquée

Cet exemple bloque un référentiel GitHub comme source de marketplace :

```json managed-settings.json theme={null}
{
  "blockedMarketplaces": [
    { "source": "github", "repo": "untrusted/plugins" }
  ]
}
```

Une entrée `github` peut utiliser la forme [owner-wildcard](#owner-wildcards) `"owner/*"` pour bloquer tous les référentiels sous ce propriétaire GitHub, ce qui nécessite Claude Code v2.1.223 ou ultérieur. Ajoutez `{ "source": "skills-dir" }` pour arrêter le chargement des [plugins `@skills-dir`](/docs/fr/plugins/loading#plugins-shared-through-a-repository) par Claude Code depuis `~/.claude/skills/` sans restreindre aucune marketplace. Consultez [Restrictions de marketplace gérées](/docs/fr/plugins/org#restrict-what-users-can-install).

<h3 id="channelsenabled">
  `channelsEnabled`
</h3>

Autorisez les [channels](/docs/fr/channels) pour votre organisation. Sur les plans Team et Enterprise de claude.ai, Claude Code bloque les channels jusqu'à ce que vous le définissiez à `true`. Pour les comptes [Anthropic Console](/docs/fr/authentication#claude-console-authentication) qui s'authentifient avec une clé API, les channels sont autorisés par défaut. Si votre organisation déploie des paramètres gérés, Claude Code bloque également les channels sur ces comptes jusqu'à ce que vous définissiez cette clé à `true`.

* **Scope** : [`Managed`](#scopes)
* **Type** : Boolean
  * `true` : Claude Code autorise les channels pour votre organisation
  * `false` : identique à unset ; que les channels soient bloqués dépend de votre plan, comme le Default l'indique
* **Default** : unset ; les channels sont bloqués sur les plans Team et Enterprise et sur les comptes Console avec des paramètres gérés, et autorisés sur les plans Pro et Max et sur les comptes Console sans paramètres gérés

```json managed-settings.json theme={null}
{
  "channelsEnabled": true
}
```

Pour restreindre les plugins qui peuvent s'enregistrer en tant que channels une fois qu'ils sont activés, définissez [`allowedChannelPlugins`](#allowedchannelplugins). Consultez [Contrôles d'entreprise](/docs/fr/channels#enterprise-controls).

<h3 id="disablecommandpluginsources">
  `disableCommandPluginSources`
</h3>

Bloquez la [source de plugin `command`](/docs/fr/plugins/marketplace-reference#command-plugin-source), qui installe un plugin en exécutant une commande déclarée par la marketplace sur la machine de l'utilisateur. Lorsque vous le définissez à `true`, Claude Code n'exécute jamais la commande, n'installe ou ne met à jour les plugins provenant de sources de commande, et arrête de charger ceux déjà installés. Définissez-le à `false` pour les autoriser explicitement. Chaque fois qu'il bloque les sources de commande, que vous le définissiez à `true` ou que vous le laissiez unset sous [`allowManagedHooksOnly`](#allowmanagedhooksonly), il bloque également les commandes [`headersHelper`](/docs/fr/plugins/host-marketplace#authenticate-archive-downloads) de la marketplace, sauf pour une marketplace que les paramètres gérés eux-mêmes déclarent. Nécessite Claude Code v2.1.229 ou ultérieur, et le blocage `headersHelper` nécessite v2.1.238 ou ultérieur.

* **Scope** : [`Managed`](#scopes)
* **Type** : Boolean
  * `true` : Claude Code n'exécute jamais la commande déclarée par la marketplace, n'installe ou ne met à jour les plugins provenant de sources de commande, et arrête de charger ceux déjà installés
  * `false` : Claude Code autorise explicitement les plugins provenant de sources de commande
* **Default** : unset, donc Claude Code suit [`allowManagedHooksOnly`](#allowmanagedhooksonly) : une organisation qui restreint l'exécution des hooks aux paramètres gérés obtient également les sources de commande désactivées

```json managed-settings.json theme={null}
{
  "disableCommandPluginSources": true
}
```

Nécessite Claude Code v2.1.229 ou ultérieur.

<h3 id="pluginsuggestionmarketplaces">
  `pluginSuggestionMarketplaces`
</h3>

Nommez les marketplaces dont les plugins peuvent apparaître comme suggestions d'installation contextuelle, dans les conseils de spinner et épinglés en haut de l'onglet **Discover** de `/plugin`. Le conseil intégré de conception frontale propriétaire n'est pas affecté. Les suggestions proviennent de la déclaration `relevance` de chaque plugin dans son entrée de marketplace.

* **Scope** : [`Managed`](#scopes)
* **Type** : tableau de noms de marketplace
* **Default** : unset, donc aucune suggestion déclarée par la marketplace ne s'affiche

```json managed-settings.json theme={null}
{
  "pluginSuggestionMarketplaces": ["acme-corp-plugins"]
}
```

Un nom prend effet uniquement lorsque la marketplace est enregistrée sur la machine et que sa source enregistrée est également déclarée dans les mêmes paramètres gérés, soit comme l'entrée [`extraKnownMarketplaces`](#extraknownmarketplaces) pour ce nom, soit comme une entrée de [`strictKnownMarketplaces`](#strictknownmarketplaces). Claude Code ignore une marketplace enregistrée à partir d'une source différente sous un nom autorisé. La marketplace officielle est exempte de l'exigence de source : autoriser son nom seul suffit, puisque ce nom ne peut s'enregistrer que depuis la source Anthropic officielle. Consultez [Suggérer des plugins par contexte](/docs/fr/plugins/relevance).

<h3 id="plugintrustmessage">
  `pluginTrustMessage`
</h3>

Ajoutez le texte de votre organisation à l'avertissement de confiance du plugin que Claude Code affiche avant l'installation, par exemple pour confirmer que les plugins de votre marketplace interne sont vérifiés.

* **Scope** : [`Managed`](#scopes)
* **Type** : string
* **Default** : unset, donc Claude Code affiche uniquement l'avertissement standard

```json managed-settings.json theme={null}
{
  "pluginTrustMessage": "All plugins from our marketplace are approved by IT"
}
```

<h3 id="strictknownmarketplaces">
  `strictKnownMarketplaces`
</h3>

Limitez les sources de marketplace de plugins à partir desquelles les personnes de votre organisation peuvent ajouter et installer des plugins. Claude Code applique la liste d'autorisation lors de l'ajout de marketplace et lors de l'installation, la mise à jour, l'actualisation et la mise à jour automatique du plugin, avant toute opération réseau ou système de fichiers, donc une marketplace que quelqu'un a ajoutée avant que vous ne définissiez la politique ne peut pas être utilisée pour récupérer des plugins une fois que sa source ne correspond plus. Les utilisateurs bloqués voient une erreur nommant la politique gérée.

Si vous définissez cette clé dans la [console d'administration claude.ai](/docs/fr/server-managed-settings), claude.ai l'applique également lorsque quelqu'un de votre organisation ajoute une marketplace à partir d'un référentiel git sur claude.ai, comme [Comment les restrictions fonctionnent](/docs/fr/plugins/org#restrict-what-users-can-install) le décrit.

* **Scope** : [`Managed`](#scopes)
* **Type** : tableau d'objets source de marketplace ; consultez [Types de source autorisés](#allowed-source-types)
* **Default** : unset, donc les utilisateurs peuvent ajouter n'importe quelle marketplace. Un tableau vide est un verrouillage complet qui bloque toutes les sources de marketplace, y compris la marketplace officielle d'Anthropic

Cet exemple autorise deux référentiels GitHub, l'un épinglé à la ref `v2.0`, et une URL `marketplace.json` hébergée :

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/approved-plugins" },
    { "source": "github", "repo": "acme-corp/security-tools", "ref": "v2.0" },
    { "source": "url", "url": "https://plugins.example.com/marketplace.json" }
  ]
}
```

Vous pouvez également écrire cette clé comme `allowedMarketplaces` ; [Alias de clé Marketplace](#marketplace-key-aliases) décrit comment Claude Code traite l'alias et quelle version l'accepte. Cette clé est une porte de politique : elle contrôle ce que les utilisateurs peuvent ajouter mais n'enregistre rien. Pour restreindre et pré-enregistrer dans un seul fichier, consultez [Combiner avec `extraKnownMarketplaces`](#combine-with-extraknownmarketplaces). Pour la vue côté utilisateur, consultez [Restrictions de marketplace gérées](/docs/fr/plugins/org#restrict-what-users-can-install).

<h4 id="allowed-source-types">
  Types de source autorisés
</h4>

Chaque entrée ci-dessous montre une entrée de liste d'autorisation par type de source et les champs qu'elle accepte. La plupart des types correspondent exactement ; `hostPattern` et `pathPattern` correspondent par regex, et les entrées `github` peuvent utiliser un [wildcard de propriétaire](#owner-wildcards).

| Source        | Exemple d'entrée                                                                                                                | Champs                                                                                                                                                       |
| :------------ | :------------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `github`      | `{ "source": "github", "repo": "acme-corp/plugins", "ref": "main", "path": "marketplace" }`                                     | `repo` requis ; `ref` est une branche ou une étiquette ; `path` est un sous-répertoire                                                                       |
| `git`         | `{ "source": "git", "url": "https://gitlab.example.com/tools/plugins.git", "ref": "production" }`                               | `url` requis ; `ref` et `path` comme pour `github`                                                                                                           |
| `url`         | `{ "source": "url", "url": "https://plugins.example.com/marketplace.json", "headers": { "Authorization": "Bearer ${TOKEN}" } }` | `url` requis ; `headers` ajoute des en-têtes HTTP pour l'accès authentifié                                                                                   |
| `file`        | `{ "source": "file", "path": "/opt/acme-corp/plugins/marketplace.json" }`                                                       | `path` requis, le chemin absolu vers un fichier `marketplace.json`                                                                                           |
| `directory`   | `{ "source": "directory", "path": "/opt/acme-corp/approved-marketplaces" }`                                                     | `path` requis, le chemin absolu vers un répertoire contenant `.claude-plugin/marketplace.json`                                                               |
| `hostPattern` | `{ "source": "hostPattern", "hostPattern": "^github\\.example\\.com$" }`                                                        | `hostPattern` requis, une regex correspondant n'importe où dans l'hôte de la marketplace ; ancrez-la avec `^` et `$` pour correspondre à l'hôte entier       |
| `pathPattern` | `{ "source": "pathPattern", "pathPattern": "^/opt/approved/" }`                                                                 | `pathPattern` requis, une regex correspondant n'importe où dans le `path` des sources `file` et `directory` ; commencez-la avec `^` pour épingler un préfixe |
| `skills-dir`  | `{ "source": "skills-dir" }`                                                                                                    | Pas de champs. Réactive l'analyse du plugin `~/.claude/skills/`                                                                                              |

Trois types de source portent des règles au-delà du tableau :

* **`url`** : une marketplace URL télécharge uniquement le fichier `marketplace.json`, et Claude Code ne récupère pas les fichiers de plugin par chemin relatif depuis ce serveur, donc ses plugins doivent utiliser une [source de plugin](/docs/fr/plugins/marketplace-reference#plugin-sources) autre qu'un chemin relatif, comme une URL d'archive, qui peut être sur le même hôte. Pour les plugins avec des chemins relatifs, utilisez plutôt une marketplace basée sur Git. Consultez [Les plugins avec des chemins relatifs échouent dans les marketplaces basées sur URL](/docs/fr/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces).
* **`hostPattern`** : utilisez-le pour autoriser chaque marketplace sur un serveur GitHub Enterprise ou GitLab interne sans lister chaque référentiel. Claude Code correspond aux sources `github` par rapport à `github.com`, prend le nom d'hôte des sources `url`, et le prend des sources `git` selon la forme de l'[URL git](https://git-scm.com/docs/git-clone#_git_urls) :

  * Une URL avec un schéma, tel que `https://` ou `ssh://` : le nom d'hôte dans l'URL.
  * Une adresse SSH sans schéma, sous la forme `user@host:path` de git, telle que `git@git.example.com:tools/plugins.git` : l'hôte entre `@` et `:`, qui est l'hôte auquel git se connecte.
  * Toute autre forme sans schéma : pas d'hôte, donc aucune entrée `strictKnownMarketplaces` `hostPattern` ne la correspond. Pour une entrée `blockedMarketplaces` `hostPattern`, Claude Code prend un hôte à partir d'un ensemble plus large de formes, donc une entrée de liste de blocage peut toujours correspondre à une telle forme. Avant v2.1.234, une entrée `strictKnownMarketplaces` `hostPattern` correspondait également à certaines formes que git ne traite pas comme des adresses SSH.

  Les sources `file` et `directory` n'ont pas d'hôte et ne correspondent jamais à une entrée `hostPattern`.
* **`pathPattern`** : utilisez-le pour autoriser les marketplaces du système de fichiers aux côtés des entrées `hostPattern` pour les sources réseau. `".*"` autorise chaque chemin local ; un motif plus étroit tel que `"^/opt/approved/"` restreint à un répertoire.

Toute liste d'autorisation, même une vide, arrête également Claude Code de charger les [plugins `@skills-dir`](/docs/fr/plugins/loading#plugins-shared-through-a-repository) depuis `~/.claude/skills/`. Ajoutez l'entrée `{ "source": "skills-dir" }` pour continuer à les charger ; l'entrée n'a aucun sens en dehors de cette clé et `blockedMarketplaces`.

<h4 id="owner-wildcards">
  Wildcards de propriétaire
</h4>

Une entrée `github` dont la valeur `repo` est `"<owner>/*"` correspond à tous les référentiels sous ce propriétaire GitHub. Les wildcards de propriétaire nécessitent Claude Code v2.1.223 ou ultérieur et ne fonctionnent que dans `strictKnownMarketplaces` et `blockedMarketplaces`. Partout ailleurs où une source `github` apparaît, comme `extraKnownMarketplaces` ou `/plugin marketplace add`, la valeur `repo` doit nommer un seul référentiel. Avant v2.1.223, Claude Code comparait l'entrée littéralement, donc une entrée de liste d'autorisation ne correspondait à aucun référentiel et une entrée de liste de blocage ne bloquait rien ; les entrées de référentiel unique sont appliquées sur chaque version.

Cette entrée autorise n'importe quel référentiel de marketplace dans l'organisation `acme-corp` :

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/*" }
  ]
}
```

Seule la position du nom de référentiel entier peut être un wildcard. Claude Code ignore les entrées telles que `*`, `*/plugins`, ou `acme-corp/tools-*` comme invalides, donc elles ne correspondent à aucun référentiel.

Les règles de correspondance diffèrent entre les deux paramètres :

| Règle                                     | `strictKnownMarketplaces`                                                                                                                                                                            | `blockedMarketplaces`                                                                               |
| ----------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| Correspondance des orthographes de source | Forme `owner/repo` uniquement. Une URL git qui clone le même référentiel ne correspond pas                                                                                                           | N'importe quelle orthographe, y compris les URL git qui se résolvent au même référentiel github.com |
| Casse du propriétaire                     | Sensible à la casse, comme la correspondance d'entrée exacte                                                                                                                                         | Insensible à la casse                                                                               |
| `ref`                                     | Suit les règles d'entrée exacte : une entrée avec un `ref` correspond uniquement aux sources avec ce ref exact, et une entrée sans un correspond uniquement aux sources qui ne spécifient pas de ref | Une entrée sans `ref` bloque tous les refs des référentiels qu'elle correspond                      |
| `path`                                    | Plus lâche que les règles d'entrée exacte : une entrée avec un `path` nécessite cette valeur exacte, tandis qu'une entrée sans un correspond à n'importe quel chemin à l'intérieur du référentiel    | Une entrée sans `path` bloque tous les chemins des référentiels qu'elle correspond                  |

<h4 id="exact-matching">
  Correspondance exacte
</h4>

Pour chaque type de source sauf les entrées `github` de wildcard de propriétaire et les entrées `hostPattern` et `pathPattern` correspondant par regex, Claude Code autorise l'ajout d'un utilisateur uniquement lorsque la source de marketplace correspond exactement à une entrée. Pour les sources basées sur git `github` et `git`, la correspondance exacte inclut les champs optionnels :

* Le `repo` ou `url` doit correspondre exactement
* Le champ `ref` doit correspondre exactement, ou les deux doivent être undefined
* Le champ `path` doit correspondre exactement, ou les deux doivent être undefined

Par exemple, Claude Code traite chaque paire ci-dessous comme deux sources différentes :

* `{ "source": "github", "repo": "acme-corp/plugins" }` et `{ "source": "github", "repo": "acme-corp/plugins", "ref": "main" }`
* `{ "source": "github", "repo": "acme-corp/plugins", "path": "marketplace" }` et `{ "source": "github", "repo": "acme-corp/plugins" }`

<h4 id="allow-only-the-official-marketplace">
  Autoriser uniquement la marketplace officielle
</h4>

Pour autoriser la marketplace officielle d'Anthropic et rien d'autre, listez son référentiel :

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "anthropics/claude-plugins-official" }
  ]
}
```

Avec cette entrée, Claude Code garde une marketplace officielle déjà enregistrée disponible et, sur une machine vierge, enregistre la marketplace automatiquement la première fois que vous démarrez une session de terminal interactive. L'enregistrement automatique manque le plus souvent :

* Les environnements non interactifs qui s'exécutent avant la première session de terminal interactive de la machine.
* Les machines où Claude Code a déjà exécuté une session de terminal interactive sous une politique qui bloquait la marketplace, comme le verrouillage de tableau vide. Claude Code enregistre la tentative bloquée et ne réessaie pas après le changement de politique.

Sur ces machines, ajoutez la marketplace à [`extraKnownMarketplaces`](#extraknownmarketplaces) dans le même `managed-settings.json` pour que Claude Code l'enregistre automatiquement, ou exécutez `claude plugin marketplace add anthropics/claude-plugins-official`.

<h4 id="combine-with-extraknownmarketplaces">
  Combiner avec `extraKnownMarketplaces`
</h4>

Les deux clés font des travaux différents. Ce tableau les compare :

| Aspect                | `strictKnownMarketplaces`                          | `extraKnownMarketplaces`                                                                                                                                       |
| --------------------- | -------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Objectif              | Application de la politique organisationnelle      | Commodité d'équipe                                                                                                                                             |
| Fichier de paramètres | Paramètres gérés uniquement                        | N'importe quel fichier de paramètres                                                                                                                           |
| Comportement          | Bloque les ajouts non autorisés                    | Enregistre les marketplaces manquantes                                                                                                                         |
| Quand appliqué        | Avant les opérations réseau et système de fichiers | Immédiatement à partir des paramètres utilisateur ou gérés ; après la boîte de dialogue de confiance de l'espace de travail pour les fichiers d'un référentiel |
| Peut être remplacé    | Non, priorité la plus élevée                       | Oui, par des paramètres de priorité plus élevée                                                                                                                |
| Format de source      | Objet source direct                                | Marketplace nommée avec un objet `source` imbriqué                                                                                                             |

Pour restreindre et pré-enregistrer une marketplace pour tous les utilisateurs, définissez les deux dans `managed-settings.json` :

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/plugins" }
  ],
  "extraKnownMarketplaces": {
    "acme-tools": {
      "source": { "source": "github", "repo": "acme-corp/plugins" }
    }
  }
}
```

Avec uniquement `strictKnownMarketplaces` défini, les utilisateurs peuvent toujours ajouter une marketplace autorisée eux-mêmes avec `/plugin marketplace add`. La marketplace officielle d'Anthropic est la seule que Claude Code enregistre automatiquement, et uniquement lorsque la liste d'autorisation l'autorise. [Autoriser uniquement la marketplace officielle](#allow-only-the-official-marketplace) liste les machines qu'elle manque.

<h3 id="strictpluginonlycustomization">
  `strictPluginOnlyCustomization`
</h3>

Bloquez les skills, agents, hooks et serveurs MCP provenant de sources utilisateur et projet, afin qu'ils ne puissent provenir que de plugins ou de paramètres gérés. Combinez-le avec [`strictKnownMarketplaces`](#strictknownmarketplaces) pour contrôler la chaîne d'approvisionnement de personnalisation complète : la liste d'autorisation de marketplace contrôle les plugins que les utilisateurs peuvent installer.

* **Scope** : [`Managed`](#scopes)
* **Type** : `true` pour verrouiller les quatre types de personnalisation, ou un tableau nommant les types à verrouiller, parmi `"skills"`, `"agents"`, `"hooks"`, et `"mcp"`
* **Default** : unset, donc rien n'est verrouillé

Cet exemple verrouille les skills et hooks et laisse les agents et serveurs MCP déverrouillés :

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["skills", "hooks"]
}
```

Les quatre entrées de sous-clé ci-dessous listent ce que chaque surface bloque et ce qui se charge toujours. Claude Code ignore les noms de surface qu'il ne reconnaît pas plutôt que d'échouer le fichier de paramètres, donc vous pouvez ajouter de nouveaux noms de surface avant que chaque client ait mis à jour.

<h3 id="strictpluginonlycustomization-skills">
  `strictPluginOnlyCustomization.skills`
</h3>

Verrouillez la surface `skills`. Claude Code arrête de charger les skills depuis `~/.claude/skills/` et `.claude/skills/`, les commandes personnalisées depuis `~/.claude/commands/` et `.claude/commands/`, les skills sous les répertoires `--add-dir`, et les skills synchronisés depuis votre compte claude.ai, et continue de charger les skills de plugin, les skills fournis, et les skills dans le répertoire de politique gérée.

* **Scope** : [`Managed`](#scopes)
* **Type** : la chaîne `"skills"` dans le tableau [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default** : non verrouillé

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["skills"]
}
```

<h3 id="strictpluginonlycustomization-agents">
  `strictPluginOnlyCustomization.agents`
</h3>

Verrouillez la surface `agents`. Claude Code arrête de charger les agents depuis `~/.claude/agents/` et `.claude/agents/`, et continue de charger les agents de plugin, les agents intégrés, et les agents dans le répertoire de politique gérée.

* **Scope** : [`Managed`](#scopes)
* **Type** : la chaîne `"agents"` dans le tableau [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default** : non verrouillé

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["agents"]
}
```

<h3 id="strictpluginonlycustomization-hooks">
  `strictPluginOnlyCustomization.hooks`
</h3>

Verrouillez la surface `hooks`. Claude Code arrête d'exécuter les hooks provenant des paramètres utilisateur, projet et `settings.json` local, et continue d'exécuter les hooks de plugin et les hooks dans les paramètres gérés.

* **Scope** : [`Managed`](#scopes)
* **Type** : la chaîne `"hooks"` dans le tableau [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default** : non verrouillé

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["hooks"]
}
```

<h3 id="strictpluginonlycustomization-mcp">
  `strictPluginOnlyCustomization.mcp`
</h3>

Verrouillez la surface `mcp`. Claude Code arrête de charger les serveurs MCP depuis `~/.claude.json` et `.mcp.json`, et continue de charger les serveurs MCP de plugin, les serveurs [`managed-mcp.json`](/docs/fr/managed-mcp) et les serveurs de [`managedMcpServers`](#managedmcpservers).

* **Scope** : [`Managed`](#scopes)
* **Type** : la chaîne `"mcp"` dans le tableau [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default** : non verrouillé

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["mcp"]
}
```

<h3 id="enabledplugins">
  `enabledPlugins`
</h3>

Activez ou désactivez les [plugins](/docs/fr/plugins/overview) individuels, indexés par `plugin-name@marketplace-name`. Un plugin sans entrée à aucune portée revient à sa valeur [`defaultEnabled`](/docs/fr/plugins/manifest-reference#fields) value. Lorsque vous activez ou désactivez un plugin avec `/plugin` ou `claude plugin enable`, Claude Code écrit cette clé pour vous.

* **Scope** : [`Any file`](#scopes)
* **Type** : objet mappant `plugin-name@marketplace-name` à un Boolean
* **Default** : unset, donc chaque plugin suit sa valeur `defaultEnabled`

Cet exemple active deux plugins de la marketplace `team-tools` et en désactive un de `personal` :

```json settings.json theme={null}
{
  "enabledPlugins": {
    "code-formatter@team-tools": true,
    "deployment-tools@team-tools": true,
    "experimental-features@personal": false
  }
}
```

Chaque portée sert un objectif différent :

* **Paramètres utilisateur** : vos préférences personnelles de plugin
* **Paramètres de projet** : plugins partagés avec tout le monde dans le référentiel
* **Paramètres locaux** : remplacements par machine, ignorés lorsque Claude Code enregistre un paramètre là
* **Paramètres gérés** : politique à l'échelle de l'organisation. Un plugin défini à `false` ici est bloqué de l'installation à chaque portée et masqué de la marketplace

Les paramètres de projet ont priorité sur les paramètres utilisateur, donc définir un plugin à `false` dans `~/.claude/settings.json` ne désactive pas un plugin que le `.claude/settings.json` du projet active. Pour refuser un plugin activé par le projet sur votre machine, définissez-le à `false` dans `.claude/settings.local.json` à la place. Les plugins forcément activés par les paramètres gérés ne peuvent pas être désactivés de cette façon, puisque les paramètres gérés remplacent les paramètres locaux.

L'activation d'un plugin à partir d'une source externe telle qu'un référentiel GitHub ou un package npm dans le `.claude/settings.json` d'un projet ne l'installe pas pour d'autres personnes. Sur chaque chemin qui charge les plugins, Claude Code signale le plugin comme non installé jusqu'à ce que chaque utilisateur [l'installe lui-même](/docs/fr/plugins/org#require-plugins-per-repository).

<h3 id="extraknownmarketplaces">
  `extraKnownMarketplaces`
</h3>

Enregistrez des marketplaces de plugins supplémentaires par nom, afin que les personnes qui ouvrent le référentiel, ou tout le monde que vos paramètres gérés atteignent, obtiennent la marketplace sans l'ajouter eux-mêmes. Claude Code enregistre chaque marketplace qu'il ne connaît pas déjà. Que le plugin que [`enabledPlugins`](#enabledplugins) nomme à partir de celui-ci s'installe dépend de la source du plugin et du fichier qui l'active ; cette entrée a les règles.

* **Scope** : [`Any file`](#scopes). Claude Code honore les entrées dans le `.claude/settings.json` ou `.claude/settings.local.json` d'un référentiel uniquement après que vous acceptiez la boîte de dialogue de confiance de l'espace de travail pour ce dossier ; dans un dossier que vous n'avez pas approuvé, y compris une exécution `-p` là, il les ignore sans message.
* **Type** : objet mappant un nom de marketplace à un objet avec un objet `source` et un Boolean `autoUpdate` optionnel
* **Default** : unset

Cet exemple enregistre une marketplace GitHub et une marketplace à partir d'une URL git auto-hébergée :

```json settings.json theme={null}
{
  "extraKnownMarketplaces": {
    "acme-tools": {
      "source": {
        "source": "github",
        "repo": "acme-corp/claude-plugins"
      }
    },
    "security-plugins": {
      "source": {
        "source": "git",
        "url": "https://git.example.com/security/plugins.git"
      }
    }
  }
}
```

[Ce qui s'exécute avant que vous approuviez un dossier](/docs/fr/permissions#what-runs-before-you-trust-a-folder) compare la porte de confiance avec le contenu qu'un référentiel peut fournir. Vous pouvez également écrire cette clé comme `additionalMarketplaces` ; consultez [Alias de clé Marketplace](#marketplace-key-aliases).

Définissez `"autoUpdate": true` aux côtés de `source` pour que Claude Code actualise cette marketplace et mette à jour ses plugins installés en arrière-plan après le démarrage. Lorsqu'il est omis, `claude-plugins-official` et la plupart des autres marketplaces officielles d'Anthropic par défaut à `true`, et les marketplaces tierces par défaut à `false`. Consultez [Configurer les mises à jour automatiques](/docs/fr/plugins/install#keep-plugins-updated).

Lorsque plus d'un fichier de paramètres définit une entrée de marketplace sous le même nom, Claude Code utilise l'entrée du [fichier de priorité la plus élevée](/docs/fr/settings#settings-precedence) entier. Cette entrée remplace l'entrée de priorité inférieure et n'hérite d'aucun de ses champs, donc une redéfinition ne peut pas combiner les `source.headers` d'authentification d'un fichier avec une URL qu'un autre fichier contrôle. Avant v2.1.228, Claude Code fusionnait les entrées de même nom champ par champ, donc une entrée dans un fichier de priorité plus élevée pouvait hériter des champs qu'elle ne définissait pas, y compris les `headers` d'un autre fichier.

<h4 id="marketplace-source-types">
  Types de source de marketplace
</h4>

L'objet `source` prend l'une de ces formes :

* **`github`** : un référentiel GitHub, avec `repo`
* **`git`** : n'importe quelle URL git, avec `url`
* **`url`** : une URL directe vers un fichier `marketplace.json`, avec `url` et `headers` optionnel et `headersHelper` pour l'accès authentifié. `headersHelper` nomme une commande qui imprime les en-têtes dont les valeurs sont trop éphémères pour lister dans `headers`, et nécessite Claude Code v2.1.238 ou ultérieur
* **`file`** : un chemin local vers un fichier `marketplace.json`, avec `path`
* **`directory`** : un chemin du système de fichiers local, avec `path`, pour le développement uniquement
* **`settings`** : une marketplace en ligne déclarée directement dans le fichier de paramètres sans référentiel hébergé, avec `name` et `plugins`

Le type de source `git` fonctionne avec n'importe quel service d'hébergement git, y compris GitLab auto-hébergé et Bitbucket. Claude Code clone le référentiel avec la même authentification que `git clone` utiliserait sur cette machine : assistants d'authentification configurés ou clés SSH. Un jeton de fournisseur tel que `GITHUB_TOKEN` prend effet via un assistant d'authentification qui le lit. Consultez [Référentiels privés](/docs/fr/plugins/host-marketplace#grant-access-to-a-private-marketplace) pour les détails de configuration.

Pour les sources `github` et `git`, Claude Code ne télécharge jamais le contenu de [Git LFS](https://git-lfs.com) lorsqu'il clone le référentiel de marketplace pour l'ajouter ou le mettre à jour. Les fichiers suivis par LFS sont extraits en tant que fichiers pointeurs, et la sortie d'ajout ou de mise à jour signale combien.

Le champ `skipLfs` à l'intérieur de l'objet `source` est accepté et n'a aucun effet. Avant v2.1.274, Claude Code téléchargeait le contenu LFS sauf si vous définissiez `"skipLfs": true`.

Pour une source `url`, définissez `headersHelper` à l'intérieur de l'objet `source` lorsque les identifiants dans `headers` expirent et qu'une commande doit en produire un nouveau. Nécessite Claude Code v2.1.238 ou ultérieur. Pour ce que la commande doit imprimer et où Claude Code l'exécute, consultez [Écrire la commande headersHelper](/docs/fr/plugins/host-marketplace#write-the-headershelper-command), et pour les cas où Claude Code ne l'exécute pas, consultez [Quand Claude Code ignore une commande headersHelper](/docs/fr/plugins/host-marketplace#when-claude-code-skips-a-headershelper-command-or-drops-its-output). Une fois que vous définissez `headersHelper` sur une URL de marketplace `https://`, Claude Code exécute la commande à deux points, réutilisant la sortie d'une exécution pendant jusqu'à 60 secondes :

* Avant chaque récupération du `marketplace.json` de cette marketplace, y compris une actualisation ultérieure. Claude Code envoie les en-têtes imprimés avec cette récupération.
* Avant chaque téléchargement d'archive de plugin sur l'origine de l'URL de la marketplace, ce qui signifie le même schéma, hôte et port. Claude Code envoie la sortie avec ce téléchargement, et aucun autre téléchargement n'obtient les en-têtes.

Claude Code ignore tout `headersHelper` défini dans le `.claude/settings.json` ou `.claude/settings.local.json` d'un répertoire que vous ajoutez avec [`--add-dir`](/docs/fr/permissions#what-runs-before-you-trust-a-folder), sur une source `url` et sur une entrée de plugin en ligne, et envoie uniquement les `headers` fixes définis dans ce fichier. [Comment les utilisateurs acceptent une commande headersHelper](/docs/fr/plugins/host-marketplace#how-users-accept-a-headershelper-command) couvre les autres fichiers de paramètres.

Les plugins listés dans une source `settings` doivent référencer des sources externes telles que GitHub ou npm, et le `name` doit correspondre à la clé de marketplace. Vous activez toujours chaque plugin séparément dans `enabledPlugins`. Cet exemple déclare un plugin en ligne :

```json settings.json theme={null}
{
  "extraKnownMarketplaces": {
    "team-tools": {
      "source": {
        "source": "settings",
        "name": "team-tools",
        "plugins": [
          {
            "name": "code-formatter",
            "source": {
              "source": "github",
              "repo": "acme-corp/code-formatter"
            }
          }
        ]
      }
    }
  }
}
```

Une entrée de plugin sous `source: 'settings'` dont le propre `source` est une [`archive`](/docs/fr/plugins/marketplace-reference#archive-plugin-source) peut définir `headers` pour le téléchargement d'archive. Si la valeur que vous mettriez dans `headers` est éphémère, comme un jeton que votre registre frappe à la demande, définissez une commande `headersHelper` à la place. Une entrée peut définir les deux. Les deux champs nécessitent Claude Code v2.1.238 ou ultérieur.

Claude Code envoie les `headers` de l'entrée, et tout ce que la commande imprime, avec le téléchargement d'archive de ce plugin et avec aucun autre téléchargement. Claude Code exécute la commande uniquement lorsqu'un utilisateur [installe ou met à jour ce seul plugin par lui-même](/docs/fr/plugins/host-marketplace#how-users-accept-a-headershelper-command). Trois règles supplémentaires dépendent du fichier qui contient l'entrée :

* **`strict`** : contrairement à une entrée dans le `marketplace.json` d'une marketplace, une entrée dans les paramètres n'a pas besoin de `"strict": false`, car un fichier de paramètres ne porte aucun champ de manifeste à intégrer. Consultez [Mode strict](/docs/fr/plugins/marketplace-reference#strict-mode).
* **Confiance de dossier** : pour une entrée dans le `.claude/settings.json` ou `.claude/settings.local.json` d'un projet, Claude Code exécute la commande uniquement après que l'utilisateur ait également [approuvé ce dossier](/docs/fr/permissions#what-runs-before-you-trust-a-folder).
* **Filtre d'en-tête** : Claude Code supprime les [noms d'en-têtes de routage de demande et d'identité client](/docs/fr/plugins/host-marketplace#when-claude-code-skips-a-headershelper-command-or-drops-its-output) d'une entrée dans le `.claude/settings.json` ou `.claude/settings.local.json` d'un projet, car un référentiel peut fournir ces fichiers. Claude Code applique le même filtre à une entrée de catalogue et à une entrée dans un répertoire `--add-dir`, et aucun filtre à une entrée dans vos paramètres utilisateur, un fichier `--settings`, ou les paramètres gérés.

<h4 id="marketplace-key-aliases">
  Alias de clé Marketplace
</h4>

Sur Claude Code v2.1.232 ou ultérieur, vous pouvez écrire `extraKnownMarketplaces` comme `additionalMarketplaces` et `strictKnownMarketplaces` comme `allowedMarketplaces`. Claude Code traite chaque alias comme suit :

* Les versions antérieures ignorent l'alias, donc gardez l'orthographe canonique dans un fichier que les versions antérieures lisent également, comme un fichier de paramètres gérés pour une flotte avec des versions Claude Code mixtes.
* Dans n'importe quel fichier de paramètres qui accepte la clé canonique, Claude Code lit l'alias exactement comme il lit la clé canonique.
* Claude Code peut réécrire `additionalMarketplaces` en `extraKnownMarketplaces` lorsqu'il met à jour le fichier.
* Si vous définissez les deux orthographes dans un fichier, Claude Code utilise la valeur canonique et ignore l'alias.

<h3 id="pluginconfigs">
  `pluginConfigs`
</h3>

Stockez les réponses non sensibles que vous donnez au dialogue de configuration [`userConfig`](/docs/fr/plugins/manifest-reference#user-configuration) d'un plugin, indexées par ID de plugin. Claude Code écrit cette clé dans vos paramètres utilisateur lorsque vous remplissez le dialogue, donc vous n'avez pas besoin de l'éditer à la main. Claude Code stocke les options sensibles dans le Keychain macOS à la place, revenant à `~/.claude/.credentials.json` lorsque le Keychain rejette l'écriture ; sur les plates-formes sans un trousseau pris en charge, il les stocke dans `~/.claude/.credentials.json`.

* **Scope** : [`User or managed`](#scopes)
* **Type** : objet mappant un ID de plugin à un objet avec un champ `options`, mappant chaque nom d'option à une chaîne, un nombre, un Boolean, ou un tableau de chaînes, et un champ `mcpServers` optionnel contenant les valeurs de configuration utilisateur par serveur dans la même forme
* **Default** : unset

Cet exemple stocke l'option `api_endpoint` pour le plugin `deployer` de `acme-tools` :

```json settings.json theme={null}
{
  "pluginConfigs": {
    "deployer@acme-tools": {
      "options": {
        "api_endpoint": "https://api.example.com"
      }
    }
  }
}
```

Les plugins intégrés stockent leurs options sous la même clé avec un suffixe `@builtin`. Par exemple, le paramètre [**Project instructions**](/docs/fr/memory#choose-which-instruction-files-load) qui contrôle si Claude Code lit les fichiers `AGENTS.md` est `pluginConfigs["agents-md@builtin"].options.instructionFiles`.

Claude Code ignore les entrées de projet et locales car il substitue ces valeurs dans les configurations de hook, MCP et LSP du plugin, et un référentiel cloné ne doit pas pouvoir les fournir. Avant v2.1.207, les paramètres de projet et locaux étaient également lus.

<h2 id="mcp">
  MCP
</h2>

Contrôlez les serveurs MCP auxquels Claude Code se connecte et ceux qu'une organisation autorise. Consultez [Connecter à des outils externes avec MCP](/docs/fr/mcp) et [Configuration MCP gérée](/docs/fr/managed-mcp).

<h3 id="allowallclaudeaimcps">
  `allowAllClaudeAiMcps`
</h3>

Chargez les [connecteurs claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai) que Claude Code récupère lui-même aux côtés d'un `managed-mcp.json` déployé. Sans cette clé, `managed-mcp.json` prend le contrôle exclusif des serveurs MCP et supprime ces connecteurs.

* **Portée** : [`Managed`](#scopes). Les utilisateurs ne peuvent pas réactiver les connecteurs que le contrôle exclusif a supprimés.
* **Type** : Booléen
  * `true` : Claude Code charge les connecteurs claude.ai aux côtés d'un `managed-mcp.json` déployé
  * `false` : un `managed-mcp.json` déployé prend le contrôle exclusif des serveurs MCP et supprime les connecteurs claude.ai [que Claude Code récupère lui-même](/docs/fr/mcp#how-connectors-reach-claude-code)
* **Défaut** : `false`, donc un `managed-mcp.json` déployé supprime les connecteurs claude.ai que Claude Code récupère lui-même

```json managed-settings.json theme={null}
{
  "allowAllClaudeAiMcps": true
}
```

[`allowedMcpServers`](#allowedmcpservers) et [`deniedMcpServers`](#deniedmcpservers) s'appliquent toujours aux connecteurs que cette clé charge. Les connecteurs livrés à une [session cloud](/docs/fr/claude-code-on-the-web) dont l'hôte porte un `managed-mcp.json`, comme un exécuteur auto-hébergé, restent supprimés. Consultez [Autoriser les connecteurs claude.ai aux côtés de l'ensemble géré](/docs/fr/managed-mcp#allow-claude-ai-connectors-alongside-the-managed-set).

<h3 id="allowedmcpservers">
  `allowedMcpServers`
</h3>

Créez une liste blanche des serveurs MCP que les utilisateurs peuvent ajouter. Claude Code bloque tout serveur qui ne correspond pas à une entrée, où qu'elle soit définie, y compris les serveurs de plugins, les serveurs passés avec `--mcp-config`, et les serveurs de claude.ai.

Les serveurs intégrés tels que Claude dans Chrome, le serveur `ide` auquel Claude Code se connecte dans un IDE [VS Code](/docs/fr/vs-code#the-built-in-ide-mcp-server) ou [JetBrains](/docs/fr/jetbrains#the-built-in-ide-mcp-server) en cours d'exécution, et les serveurs que l'interface de ligne de commande elle-même configure sont exemptés de la liste blanche, et la liste noire s'applique toujours à eux. Les serveurs `type: "sdk"` en processus sont exemptés des deux listes ; [l'application qui a démarré la session](/docs/fr/mcp#how-connectors-reach-claude-code) les enregistre.

Les serveurs que votre organisation livre sont également exemptés de la liste blanche, et la liste noire s'applique toujours à eux. L'exemption couvre chaque entrée [`managedMcpServers`](#managedmcpservers), et toute entrée [`managed-mcp.json`](/docs/fr/managed-mcp#exclusive-control-with-managed-mcp-json) dont les valeurs n'utilisent pas d'expansion `${VAR}`. Consultez [Comment un serveur est évalué](/docs/fr/managed-mcp#how-a-server-is-evaluated) pour l'ordre de vérification complet. Avant la v2.1.259, les serveurs de `managed-mcp.json` devaient également correspondre.

* **Portée** : [`Any file`](#scopes). Les entrées de chaque fichier fusionnent en une seule liste blanche sauf si [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly) est défini. Déployez-le dans les paramètres gérés pour l'appliquer.
* **Type** : tableau d'objets, chacun avec exactement une clé : `serverName`, une chaîne limitée aux lettres, chiffres, tirets et traits de soulignement ; `serverCommand`, un tableau de la commande et de ses arguments correspondant exactement ; ou `serverUrl`, un modèle d'URL avec des caractères génériques `*`
* **Défaut** : non défini, donc chaque serveur est autorisé ; un tableau vide bloque chaque serveur que les utilisateurs ajoutent

Cet exemple autorise uniquement le serveur stdio que la commande `npx` listée démarre :

```json settings.json theme={null}
{
  "allowedMcpServers": [
    { "serverCommand": ["npx", "-y", "@modelcontextprotocol/server-filesystem"] }
  ]
}
```

Une entrée [`deniedMcpServers`](#deniedmcpservers) a la priorité, donc un serveur sur les deux listes est bloqué. Une fois que la liste contient une entrée `serverCommand`, un serveur stdio doit correspondre à une entrée `serverCommand`, et une fois qu'elle contient une entrée `serverUrl`, un serveur distant doit correspondre à une entrée `serverUrl` : une correspondance `serverName` n'admet plus ce type de serveur. Consultez [Contrôle basé sur les politiques avec listes blanches et listes noires](/docs/fr/managed-mcp#policy-based-control-with-allowlists-and-denylists).

<h3 id="allowmanagedmcpserversonly">
  `allowManagedMcpServersOnly`
</h3>

Rendez la liste blanche gérée la seule qui s'applique. Claude Code lit alors [`allowedMcpServers`](#allowedmcpservers) uniquement à partir des paramètres gérés et ignore les listes blanches dans les paramètres utilisateur, projet et locaux ; [`deniedMcpServers`](#deniedmcpservers) fusionne toujours à partir de chaque portée de paramètres, donc les utilisateurs peuvent toujours bloquer les serveurs pour eux-mêmes. Les administrateurs le définissent pour que les paramètres propres d'un utilisateur ne puissent pas élargir ce que la liste blanche gérée permet.

* **Portée** : [`Managed`](#scopes)
* **Type** : Booléen
  * `true` : Claude Code lit `allowedMcpServers` uniquement à partir des paramètres gérés et ignore les listes blanches dans les paramètres utilisateur, projet et locaux
  * `false` : les listes blanches de chaque portée de paramètres fusionnent
* **Défaut** : `false`, donc les listes blanches de chaque portée de paramètres fusionnent

Cet exemple verrouille la liste blanche aux paramètres gérés et autorise uniquement le serveur nommé `github` :

```json managed-settings.json theme={null}
{
  "allowManagedMcpServersOnly": true,
  "allowedMcpServers": [
    { "serverName": "github" }
  ]
}
```

Les utilisateurs peuvent toujours ajouter leurs propres serveurs MCP ; seuls les serveurs qui correspondent à la liste blanche gérée se chargent. Consultez [Restreindre la liste blanche aux paramètres gérés uniquement](/docs/fr/managed-mcp#restrict-the-allowlist-to-managed-settings-only).

<h3 id="deniedmcpservers">
  `deniedMcpServers`
</h3>

Bloquez des serveurs MCP spécifiques. Claude Code refuse de charger un serveur correspondant, où qu'il soit défini, y compris les serveurs de plugins, les serveurs passés avec `--mcp-config`, les serveurs de `managed-mcp.json`, les serveurs de [`managedMcpServers`](#managedmcpservers), et les connecteurs claude.ai [qu'il récupère lui-même](/docs/fr/mcp#how-connectors-reach-claude-code). Les serveurs `type: "sdk"` en processus sont exemptés ; l'application qui a démarré la session les enregistre.

* **Portée** : [`Any file`](#scopes). Les entrées de chaque fichier fusionnent en une seule liste noire, et [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly) ne change pas cela. Déployez-le dans les paramètres gérés pour l'appliquer.
* **Type** : tableau d'objets, chacun avec exactement une clé : `serverName`, une chaîne, donc le nom d'affichage d'un connecteur claude.ai comme `"claude.ai Slack"` fonctionne ; `serverCommand`, un tableau de la commande et de ses arguments correspondant exactement ; ou `serverUrl`, un modèle d'URL avec des caractères génériques `*`
* **Défaut** : non défini, donc aucun serveur n'est bloqué ; un tableau vide ne bloque rien non plus

```json settings.json theme={null}
{
  "deniedMcpServers": [
    { "serverName": "filesystem" }
  ]
}
```

La liste noire a la priorité sur [`allowedMcpServers`](#allowedmcpservers), donc un serveur sur les deux listes est bloqué. Consultez [Contrôle basé sur les politiques avec listes blanches et listes noires](/docs/fr/managed-mcp#policy-based-control-with-allowlists-and-denylists).

<h3 id="disableclaudeaiconnectors">
  `disableClaudeAiConnectors`
</h3>

Désactivez les [connecteurs MCP claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai) [que Claude Code récupère lui-même](/docs/fr/mcp#how-connectors-reach-claude-code), afin qu'il ne les récupère ni ne les connecte. Un `true` dans n'importe quel fichier de paramètres s'applique : un `.claude/settings.json` de projet enregistré peut exclure un référentiel de ces connecteurs, mais un `false` au niveau du projet ne peut pas remplacer un `true` au niveau utilisateur ou géré.

* **Portée** : [`Any file`](#scopes)
* **Type** : Booléen
  * `true` : Claude Code ne récupère ni ne connecte ces connecteurs
  * `false` : identique à non défini ; Claude Code récupère vos connecteurs sauf si un autre fichier de paramètres ou `ENABLE_CLAUDEAI_MCP_SERVERS` les désactive
* **Défaut** : `false`, donc Claude Code récupère vos connecteurs
* **Remplacements par session** : [`ENABLE_CLAUDEAI_MCP_SERVERS`](/docs/fr/env-vars) défini sur `false` désactive les connecteurs pour une session ; quel que soit celui des deux qui les désactive, l'autre ne peut pas les réactiver

```json settings.json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

Les serveurs que vous passez explicitement avec `--mcp-config` ne sont pas affectés. Pour bloquer des connecteurs individuels au lieu de tous les bloquer, utilisez [`deniedMcpServers`](#deniedmcpservers). Consultez [Désactiver les connecteurs claude.ai](/docs/fr/mcp#disable-claude-ai-connectors).

<h3 id="disabledmcpjsonservers">
  `disabledMcpjsonServers`
</h3>

Rejetez des serveurs spécifiques définis dans le fichier `.mcp.json` d'un projet afin que Claude Code ne les connecte jamais ou ne vous demande de les approuver. Un rejet dans n'importe quel fichier de paramètres s'applique, y compris un `.claude/settings.json` de projet enregistré dans le référentiel.

* **Portée** : [`Any file`](#scopes)
* **Type** : tableau de chaînes, les noms de serveur tels qu'ils apparaissent dans `.mcp.json`
* **Défaut** : non défini

```json settings.json theme={null}
{
  "disabledMcpjsonServers": ["filesystem"]
}
```

Claude Code écrit cette clé dans `.claude/settings.local.json` lorsque vous rejetez un serveur dans la boîte de dialogue d'approbation. `claude mcp get <name>` affiche un serveur rejeté comme `✘ Rejected (see disabledMcpjsonServers in settings)`. Le rejet a la priorité sur [`enabledMcpjsonServers`](#enabledmcpjsonservers) et [`enableAllProjectMcpServers`](#enableallprojectmcpservers).

<h3 id="enableallprojectmcpservers">
  `enableAllProjectMcpServers`
</h3>

Approuvez chaque serveur MCP défini dans les fichiers `.mcp.json` du projet sans invite. Claude Code écrit cette clé dans `.claude/settings.local.json` lorsque vous choisissez d'approuver tous les serveurs dans la boîte de dialogue d'approbation.

* **Portée** : [`Any file`](#scopes). Dans un dossier dont vous n'avez pas accepté la boîte de dialogue de confiance, Claude Code l'honore à partir des paramètres utilisateur, des paramètres gérés et de `--settings` et l'ignore dans le fichier de projet partagé, à la fois dans la session et pour `claude mcp list` et `claude mcp get` ; [Approbations des serveurs de projet et confiance de l'espace de travail](/docs/fr/mcp#project-server-approvals-and-workspace-trust) indique quand un `.claude/settings.local.json` non suivi compte également.
* **Type** : Booléen
  * `true` : Claude Code approuve chaque serveur MCP défini dans les fichiers `.mcp.json` du projet sans invite
  * `false` : Claude Code vous demande d'approuver chaque serveur. Dans un dossier de confiance, un `false` dans un fichier de priorité plus élevée remplace un `true` dans un fichier de priorité plus basse ; dans un dossier que vous n'avez pas approuvé, un `true` dans n'importe quel fichier honoré suffit
* **Défaut** : non défini, donc Claude Code vous demande d'approuver chaque serveur

```json settings.json theme={null}
{
  "enableAllProjectMcpServers": true
}
```

Une entrée [`disabledMcpjsonServers`](#disabledmcpjsonservers) rejette toujours un serveur.

<h3 id="enabledmcpjsonservers">
  `enabledMcpjsonServers`
</h3>

Approuvez des serveurs spécifiques définis dans les fichiers `.mcp.json` du projet afin que Claude Code les connecte sans demander. Claude Code écrit cette clé dans `.claude/settings.local.json` lorsque vous approuvez un serveur dans la boîte de dialogue d'approbation.

* **Portée** : [`Any file`](#scopes). Dans un dossier dont vous n'avez pas accepté la boîte de dialogue de confiance, Claude Code l'honore à partir des paramètres utilisateur, des paramètres gérés et de `--settings` et l'ignore dans le fichier de projet partagé, à la fois dans la session et pour `claude mcp list` et `claude mcp get` ; [Approbations des serveurs de projet et confiance de l'espace de travail](/docs/fr/mcp#project-server-approvals-and-workspace-trust) indique quand un `.claude/settings.local.json` non suivi compte également.
* **Type** : tableau de chaînes, les noms de serveur tels qu'ils apparaissent dans `.mcp.json`
* **Défaut** : non défini

Cet exemple approuve les serveurs `memory` et `github` du `.mcp.json` du projet :

```json settings.json theme={null}
{
  "enabledMcpjsonServers": ["memory", "github"]
}
```

Une entrée [`disabledMcpjsonServers`](#disabledmcpjsonservers) rejette toujours un serveur.

<h3 id="managedmcpservers">
  `managedMcpServers`
</h3>

Fournissez des serveurs MCP distants à chaque utilisateur à partir des paramètres gérés. Les utilisateurs conservent les serveurs qu'ils ajoutent eux-mêmes et ne peuvent pas modifier ou supprimer ceux que vous fournissez. Nécessite Claude Code v2.1.259 ou ultérieur.

* **Portée** : [`Managed`](#scopes). Claude Code supprime la clé avec un avertissement dans les paramètres utilisateur, projet et locaux, et ne la lit pas dans l'onglet Code de l'application Claude Desktop sur un déploiement tiers ou dans les sessions Cowork de l'application, où Claude Desktop fournit et verrouille les serveurs MCP de ces sessions lui-même.
* **Type** : objet indexé par nom de serveur. Chaque entrée a la forme `.mcp.json` pour un serveur `http` ou `sse` : une `url` `https://` requise, et optionnellement `headers`, `oauth`, et les autres options HTTP et SSE. Claude Code supprime les entrées qui échouent la validation, et [Ce qu'une entrée peut contenir](/docs/fr/managed-mcp#what-an-entry-can-contain) énumère les conditions
* **Défaut** : non défini, donc les paramètres gérés ne fournissent aucun serveur

Cet exemple fournit un serveur HTTP nommé `search` :

```json managed-settings.json theme={null}
{
  "managedMcpServers": {
    "search": {
      "type": "http",
      "url": "https://search.example.com/mcp"
    }
  }
}
```

Pour la priorité, comment les serveurs fournis se combinent avec `managed-mcp.json` et les listes d'autorisation et de refus, et ce que les utilisateurs voient, consultez [Fournir des serveurs via les paramètres gérés](/docs/fr/managed-mcp#provide-servers-through-managed-settings).

<h2 id="agents-sessions-and-worktrees">
  Agents, sessions, et worktrees
</h2>

Définissez l'agent par défaut, contrôlez les coéquipiers et la messagerie entre sessions, et configurez les worktrees. Voir [Subagents](/docs/fr/sub-agents) et [Worktrees](/docs/fr/worktrees).

<h3 id="agent">
  `agent`
</h3>

Exécutez le thread principal en tant que [subagent](/docs/fr/sub-agents#invoke-subagents-explicitly) nommé, de sorte que Claude Code applique l'invite système, les restrictions d'outils et le modèle de ce subagent à votre session. La même clé définit l'agent par défaut pour les sessions que vous distribuez à partir de `claude agents`.

* **Scope** : [`Any file`](#scopes)
* **Type** : string, le nom d'un agent intégré ou personnalisé
* **Default** : unset, de sorte que le thread principal s'exécute en tant qu'agent par défaut de Claude Code
* **Per-session overrides** : `--agent` prend la priorité sur cette clé pour une session

```json settings.json theme={null}
{
  "agent": "code-reviewer"
}
```

Le propre `settings.json` d'un plugin peut également fournir cette clé ; voir [Ship default settings with your plugin](/docs/fr/plugins/components#default-settings).

<h3 id="crosssessioninbound">
  `crossSessionInbound`
</h3>

Choisissez ce que cette session fait avec les [messages provenant de vos autres sessions Claude Code](/docs/fr/cross-session-messaging#control-inbound-messages). Quand aucune valeur ne s'applique, Claude Code décide par message à partir des classes de mode de permission des deux sessions. Nécessite Claude Code v2.1.224 ou ultérieur.

* **Scope** : [`Any file`](#scopes). Une valeur de projet ou locale s'applique uniquement quand elle est plus stricte que la valeur des paramètres gérés, l'indicateur `--settings`, ou les paramètres utilisateur.
* **Type** : string, l'un de :
  * `"accept"` : Claude Code remet le message à Claude
  * `"hold"` : Claude Code affiche un avis pour le message sans le remettre
  * `"refuse"` : Claude Code supprime le message
* **Default** : unset, de sorte que Claude Code décide par message

```json settings.json theme={null}
{
  "crossSessionInbound": "hold"
}
```

Claude Code lit d'abord les paramètres gérés, puis l'indicateur `--settings`, puis les paramètres utilisateur, et applique la première valeur trouvée. `refuse` est plus strict que `hold`, et `hold` est plus strict que `accept`. Quand aucune des sources de confiance ne définit une valeur, un `hold` ou `refuse` de projet ou local s'applique toujours, remplaçant la valeur par défaut par message. Dans les sessions avec messagerie entre sessions, cette clé apparaît dans `/config` en tant que **Messages from your other sessions**, qui l'écrit dans les paramètres utilisateur ; la ligne nécessite Claude Code v2.1.232 ou ultérieur, et Claude Code la masque tandis que l'indicateur `--settings` ou les paramètres gérés définissent la clé.

Claude Code [avertit](/docs/fr/errors#crosssessioninbound-must-be-one-of-accept-hold-refuse) quand vous définissez une valeur qu'il ne reconnaît pas. Tant que cette valeur est présente dans un fichier utilisateur, projet, local ou `--settings`, Claude Code retient les messages entrants, même quand une source qui a la priorité définit `accept`. Un `refuse` qu'une autre source définit s'applique toujours. Corrigez ou supprimez la valeur pour effacer la rétention.

Quand la valeur non reconnue se trouve dans les [paramètres gérés](/docs/fr/managed-settings), Claude Code la traite plutôt comme `refuse` jusqu'à ce qu'un administrateur la corrige. Avant v2.1.248, Claude Code ignorait une valeur non reconnue sans avertissement.

<h3 id="disableagentview">
  `disableAgentView`
</h3>

Désactivez les [agents d'arrière-plan et la vue agent](/docs/fr/agent-view) : `claude agents`, `--bg`, `/background`, et le superviseur à la demande. Définissez-le dans les [paramètres gérés](/docs/fr/managed-settings) pour l'appliquer à une organisation.

* **Scope** : [`Any file`](#scopes)
* **Type** : Boolean
  * `true` : Claude Code désactive `claude agents`, `--bg`, `/background`, et le superviseur à la demande
  * `false` : la vue agent est disponible
* **Default** : unset, de sorte que la vue agent est disponible
* **Per-session overrides** : [`CLAUDE_CODE_DISABLE_AGENT_VIEW`](/docs/fr/env-vars) désactive la vue agent pour une session ; quel que soit celui des deux qui la désactive, l'autre ne peut pas la réactiver

```json settings.json theme={null}
{
  "disableAgentView": true
}
```

<h3 id="isolatepeermachines">
  `isolatePeerMachines`
</h3>

Exigez votre approbation explicite avant que `SendMessage` de Claude n'atteigne l'une de vos sessions au-delà de cette machine ; voir [Require approval for cross-machine messages](/docs/fr/cross-session-messaging#require-approval-for-cross-machine-messages). L'invite d'approbation apparaît même en mode [`bypassPermissions`](/docs/fr/permission-modes#skip-all-checks-with-bypasspermissions-mode).

* **Scope** : [`Any file`](#scopes). Un `true` de n'importe quel scope s'applique, de sorte qu'un fichier de projet enregistré peut activer l'exigence mais pas la désactiver.
* **Type** : Boolean
  * `true` : Claude Code vous demande votre approbation avant que `SendMessage` de Claude n'atteigne l'une de vos sessions au-delà de cette machine
  * `false` : les messages entre machines ne demandent pas
* **Default** : unset, de sorte que les messages entre machines ne demandent pas

```json settings.json theme={null}
{
  "isolatePeerMachines": true
}
```

L'approbation `SendMessage` entre machines nécessite Claude Code v2.1.224 ou ultérieur.

<h3 id="processwrapper">
  `processWrapper`
</h3>

Sur macOS et Linux, placez une commande de lanceur d'entreprise devant les [processus d'arrière-plan que Claude Code démarre](/docs/fr/corporate-launcher#what-the-launcher-covers). Claude Code exécute le lanceur avec sa propre ligne de commande ajoutée, de sorte que le lanceur doit exec dans Claude Code ; voir [Run Claude Code behind a corporate launcher](/docs/fr/corporate-launcher) pour le contrat du lanceur. Nécessite Claude Code v2.1.210 ou ultérieur.

* **Scope** : [`User or managed`](#scopes)
* **Type** : string, la commande du lanceur en tant que préfixe argv, comme un chemin absolu avec des arguments optionnels
* **Default** : unset, de sorte que les processus d'arrière-plan démarrent sans wrapper
* **Per-session overrides** : [`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/fr/env-vars) prend la priorité sur cette clé pour une session

```json settings.json theme={null}
{
  "processWrapper": "/opt/corp/launcher --profile claude"
}
```

Claude Code ignore le lanceur sur Windows et démarre chaque processus sans wrapper. Nécessite Claude Code v2.1.210 ou ultérieur.

<h3 id="teammatemode">
  `teammateMode`
</h3>

Choisissez où Claude Code affiche les coéquipiers de l'[équipe agent](/docs/fr/agent-teams) : à l'intérieur de votre volet terminal principal, ou dans des volets divisés quand votre terminal les supporte. Voir [Choose a display mode](/docs/fr/agent-teams#choose-a-display-mode).

* **Scope** : [`Any file`](#scopes). Claude Code lit également une valeur laissée dans `~/.claude.json` par les versions antérieures.
* **Type** : string, l'un de :
  * `"in-process"` : les coéquipiers s'exécutent à l'intérieur de votre volet terminal principal
  * `"auto"` : volets divisés quand vous exécutez à l'intérieur de tmux, ou à l'intérieur d'iTerm2 avec `it2` sur votre `PATH` ou tmux installé ; in-process sinon
  * `"tmux"` : volets divisés utilisant tmux ou iTerm2, détectés à partir de votre terminal
  * `"iterm2"` : volets divisés natifs iTerm2 via le CLI `it2`
* **Default** : `"in-process"`
* **Per-session overrides** : `--teammate-mode` prend la priorité sur cette clé pour une session

```json settings.json theme={null}
{
  "teammateMode": "auto"
}
```

<span id="worktree-settings" />

<h3 id="worktree">
  `worktree`
</h3>

Configurez comment Claude Code crée et gère les [git worktrees](/docs/fr/worktrees) pour `--worktree`, l'outil `EnterWorktree`, et les subagents isolés et les sessions d'arrière-plan.

* **Scope** : [`Any file`](#scopes)
* **Type** : object avec `baseRef`, `symlinkDirectories`, `sparsePaths`, et `bgIsolation`
* **Default** : unset

Cet exemple crée des branches de nouveaux worktrees à partir de votre `HEAD` actuel et crée des liens symboliques `node_modules` dans chacun :

```json settings.json theme={null}
{
  "worktree": {
    "baseRef": "head",
    "symlinkDirectories": ["node_modules"]
  }
}
```

Pour copier les fichiers ignorés par git comme `.env` dans les nouveaux worktrees, ajoutez plutôt un [fichier `.worktreeinclude`](/docs/fr/worktrees#copy-gitignored-files-into-worktrees) à la racine de votre projet.

<h3 id="worktree-baseref">
  `worktree.baseRef`
</h3>

Choisissez à partir de quel ref les nouveaux worktrees créent des branches. `"fresh"` crée des branches à partir de `origin/<default-branch>` pour un arbre propre correspondant au distant ; `"head"` crée des branches à partir de votre `HEAD` local actuel, de sorte que les commits non poussés et l'état de la branche de fonctionnalité sont présents dans le worktree.

* **Scope** : [`Any file`](#scopes)
* **Type** : string, l'un de :
  * `"fresh"` : les nouveaux worktrees créent des branches à partir de `origin/<default-branch>`
  * `"head"` : les nouveaux worktrees créent des branches à partir de votre `HEAD` local actuel, y compris les commits non poussés
* **Default** : `"fresh"`

```json settings.json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

À l'intérieur d'un worktree lié, `"head"` se résout en `HEAD` de ce worktree, pas celui du checkout principal.

<h3 id="worktree-symlinkdirectories">
  `worktree.symlinkDirectories`
</h3>

Créez des liens symboliques vers les répertoires du référentiel principal dans chaque worktree afin de ne pas dupliquer les grands répertoires sur le disque.

* **Scope** : [`Any file`](#scopes)
* **Type** : array of strings, chemins de répertoires relatifs à la racine du référentiel
* **Default** : unset, de sorte que Claude Code ne crée de liens symboliques vers aucun répertoire

Cet exemple crée des liens symboliques vers `node_modules` et `.cache` du référentiel principal dans chaque nouveau worktree :

```json settings.json theme={null}
{
  "worktree": {
    "symlinkDirectories": ["node_modules", ".cache"]
  }
}
```

<h3 id="worktree-sparsepaths">
  `worktree.sparsePaths`
</h3>

Vérifiez uniquement les répertoires listés dans chaque worktree via git sparse-checkout. Claude Code écrit uniquement ces répertoires plus les fichiers au niveau racine sur le disque, ce qui est plus rapide dans les grands monorepos ; voir [Check out only the directories you need](/docs/fr/large-codebases#check-out-only-the-directories-you-need).

* **Scope** : [`Any file`](#scopes)
* **Type** : array of strings, chemins de répertoires relatifs à la racine du référentiel
* **Default** : unset, de sorte que chaque worktree vérifie l'arbre entier

Cet exemple vérifie uniquement `packages/my-app` et `shared/utils`, plus les fichiers au niveau racine, dans chaque worktree :

```json settings.json theme={null}
{
  "worktree": {
    "sparsePaths": ["packages/my-app", "shared/utils"]
  }
}
```

Tant qu'un worktree clairsemé existe, git active `extensions.worktreeConfig` dans le `.git/config` partagé du référentiel.

<h3 id="worktree-bgisolation">
  `worktree.bgIsolation`
</h3>

Choisissez comment les [sessions d'arrière-plan](/docs/fr/agent-view#how-file-edits-are-isolated) isolent leurs modifications de fichiers. Avec `"worktree"`, Claude Code bloque `Edit` et `Write` dans le checkout principal jusqu'à ce que la session appelle `EnterWorktree` ; avec `"none"`, les travaux d'arrière-plan modifient la copie de travail directement. Définissez `"none"` pour un référentiel où les git worktrees ne sont pas pratiques.

* **Scope** : [`Any file`](#scopes)
* **Type** : string, l'un de :
  * `"worktree"` : Claude Code bloque `Edit` et `Write` dans le checkout principal jusqu'à ce que la session appelle `EnterWorktree`
  * `"none"` : les travaux d'arrière-plan modifient la copie de travail directement
* **Default** : `"worktree"`

```json settings.json theme={null}
{
  "worktree": {
    "bgIsolation": "none"
  }
}
```

En dehors d'un référentiel git, un [hook `WorktreeCreate`](/docs/fr/worktrees#non-git-version-control) qui échoue libère le bloc de sorte que la session puisse modifier le répertoire de travail sur place ; cette libération nécessite Claude Code v2.1.203 ou ultérieur.

<h2 id="remote-desktop-and-notifications">
  Contrôle à distance, bureau et notifications
</h2>

Configurez le contrôle à distance, les environnements cloud, l'application de bureau et les notifications que Claude Code vous envoie quand il a besoin de vous. Voir [Contrôle à distance](/docs/fr/remote-control).

<h3 id="agentpushnotifenabled">
  `agentPushNotifEnabled`
</h3>

Permettre à Claude d'envoyer une notification push à votre téléphone quand il décide qu'elle en vaut la peine, par exemple quand une tâche longue se termine. Claude Code synchronise ce choix avec votre compte, et les notifications arrivent pendant que le [Contrôle à distance](/docs/fr/remote-control) est connecté. Apparaît dans `/config` sous **Envoyer une notification quand Claude le décide**.

* **Portée** : [`Tout fichier`](#scopes). Claude Code lit également une valeur laissée dans `~/.claude.json` par les versions antérieures.
* **Type** : Booléen
  * `true` : Claude peut envoyer une notification push à votre téléphone quand il décide qu'elle en vaut la peine
  * `false` : Claude n'envoie pas ces notifications
* **Défaut** : `false`

```json settings.json theme={null}
{
  "agentPushNotifEnabled": true
}
```

Voir [Notifications push mobiles](/docs/fr/remote-control#mobile-push-notifications).

<h3 id="awaysummaryenabled">
  `awaySummaryEnabled`
</h3>

Afficher un résumé de session d'une ligne quand vous revenez au terminal après quelques minutes d'absence. Définissez-le sur `false`, ou désactivez **Résumé de session** dans `/config`, pour arrêter le résumé.

* **Portée** : [`Tout fichier`](#scopes)
* **Type** : Booléen
  * `true` : vous voyez un résumé de session d'une ligne quand vous revenez après quelques minutes d'absence
  * `false` : Claude Code n'affiche aucun résumé
* **Défaut** : non défini, donc le résumé est activé
* **Remplacements par session** : [`CLAUDE_CODE_ENABLE_AWAY_SUMMARY`](/docs/fr/env-vars) a la priorité sur cette clé pour une session, dans les deux sens

```json settings.json theme={null}
{
  "awaySummaryEnabled": false
}
```

Claude Code n'affiche jamais le résumé en mode non interactif.

<h3 id="disableartifact">
  `disableArtifact`
</h3>

<Warning>
  Obsolète et remplacé par [`enableArtifact`](#enableartifact). Claude Code honore toujours `disableArtifact: true` comme équivalent à `enableArtifact: false`, et ignore `disableArtifact: false`.
</Warning>

Utilisez [`enableArtifact`](#enableartifact) à la place pour désactiver l'outil [Artifact](/docs/fr/artifacts), qui publie la sortie de session en tant que page web privée sur claude.ai. Quand vous désactivez la ligne **Artifacts** dans `/config`, Claude Code écrit `enableArtifact` dans vos paramètres utilisateur et efface cette clé.

* **Portée** : [`Tout fichier`](#scopes)
* **Type** : Booléen
  * `true` : Claude Code désactive l'outil Artifact pour chaque session à laquelle le fichier s'applique, et aucun autre fichier ne le réactive. Avant v2.1.242, un fichier de priorité plus élevée pouvait remplacer le `true` d'un fichier de priorité inférieure plutôt que la clé agissant comme un verrou
  * `false` : ignoré ; pour laisser l'outil activé, supprimez la clé
* **Défaut** : non défini, donc l'outil suit la [disponibilité](/docs/fr/artifacts#availability) de votre compte
* **Remplacements par session** : [`CLAUDE_CODE_DISABLE_ARTIFACT`](/docs/fr/env-vars) défini sur `1` désactive l'outil pour une session

```json settings.json theme={null}
{
  "disableArtifact": true
}
```

[Désactiver les artifacts](/docs/fr/artifacts#disable-artifacts) énumère tous les moyens de désactiver l'outil.

<h3 id="disabledeeplinkregistration">
  `disableDeepLinkRegistration`
</h3>

Empêcher Claude Code d'enregistrer le gestionnaire de protocole `claude-cli://` auprès du système d'exploitation, ce qu'il fait autrement après que vous ayez envoyé la première invite d'une session interactive. Les [Liens profonds](/docs/fr/deep-links) permettent aux outils externes d'ouvrir une session Claude Code avec une invite préremplie. Définissez ceci dans les environnements où l'enregistrement du gestionnaire de protocole est restreint ou géré séparément.

* **Portée** : [`Tout fichier`](#scopes)
* **Type** : la chaîne `"disable"`
* **Défaut** : non défini, donc Claude Code enregistre le gestionnaire

```json settings.json theme={null}
{
  "disableDeepLinkRegistration": "disable"
}
```

<h3 id="disabledesktoplocalsessions">
  `disableDesktopLocalSessions`
</h3>

Désactiver les sessions Code qui s'exécutent sur l'appareil dans l'[application de bureau](/docs/fr/desktop#local-sessions-on-managed-devices), pour les déploiements où les développeurs doivent travailler sur des machines distantes via SSH. Dans l'onglet Code, l'environnement **Local** reste dans la liste déroulante des environnements mais est grisé et ne peut pas être sélectionné, avec une info-bulle indiquant que votre organisation l'a désactivé ; sur Windows, l'entrée WSL est grisée de la même manière, bien que le fait que les sessions WSL s'exécutent sur un appareil géré soit [gouverné séparément](/docs/fr/admin-setup#wsl-sessions-in-claude-code-desktop). Les nouvelles sessions utilisent par défaut la première [connexion SSH](/docs/fr/desktop#ssh-sessions) si une est configurée, et l'application refuse de démarrer ou de reprendre une session sur l'appareil, y compris une connexion SSH vers la même machine. Les sessions SSH vers d'autres hôtes et les sessions cloud ne sont pas affectées. L'application de bureau lit cette clé ; le CLI du terminal l'ignore. Nécessite Claude Desktop v1.37937.0 ou ultérieur.

* **Portée** : [`Géré`](#scopes)
* **Type** : Booléen ; seul le Booléen JSON `true` a un effet
  * `true` : l'application de bureau n'offre aucune session Code sur l'appareil ; les sessions locales existantes restent listées mais ne peuvent pas continuer
  * `false` : les sessions locales restent disponibles
* **Défaut** : non défini, donc les sessions locales sont disponibles

```json managed-settings.json theme={null}
{
  "disableDesktopLocalSessions": true
}
```

L'application de bureau ignore toute autre valeur, et une valeur qui n'est pas un Booléen, comme la chaîne `"true"` ou `1`, enregistre également un avertissement. Associez-le à [`sshConfigs`](#sshconfigs) pour que les utilisateurs accèdent à une connexion fonctionnelle, et à [`sshHostAllowlist`](#sshhostallowlist) pour limiter les hôtes qu'ils peuvent atteindre. Voir [Sessions locales sur les appareils gérés](/docs/fr/desktop#local-sessions-on-managed-devices).

Claude Desktop fournit aux sessions Code une politique dérivée de votre configuration de bureau, par exemple la liste d'autorisation de sortie, le bac à sable du système de fichiers et les restrictions MCP dans les déploiements tiers. Claude Code ignore ces paramètres parents chaque fois qu'une [source d'administrateur](/docs/fr/managed-settings#how-claude-code-combines-managed-sources) est présente : paramètres gérés par le serveur, une politique MDM ou au niveau du système d'exploitation, ou un fichier de paramètres gérés. Déployer cette clé via l'un de ceux-ci sur un appareil qui n'en avait aucun auparavant, comme dans les déploiements tiers, arrête donc l'application des politiques dérivées du bureau. [Laisser un hôte d'intégration ajouter une politique](/docs/fr/managed-settings#let-an-embedding-host-add-policy) couvre le moment où les paramètres parents peuvent toujours fusionner ; cela s'applique à toute clé que vous déployez de cette manière, pas seulement celle-ci.

<h3 id="disableremotecontrol">
  `disableRemoteControl`
</h3>

Désactiver le [Contrôle à distance](/docs/fr/remote-control) : Claude Code refuse alors `claude remote-control`, l'indicateur `--remote-control`, le démarrage automatique et le basculement en session, et signale que la politique de votre organisation l'a désactivé. Placez-le dans les [paramètres gérés](/docs/fr/managed-settings) pour l'application de la politique MDM par appareil.

* **Portée** : [`Tout fichier`](#scopes)
* **Type** : Booléen
  * `true` : Claude Code refuse `claude remote-control`, l'indicateur `--remote-control`, le démarrage automatique et le basculement en session
  * `false` : le Contrôle à distance reste disponible
* **Défaut** : `false`

```json settings.json theme={null}
{
  "disableRemoteControl": true
}
```

<h3 id="enableartifact">
  `enableArtifact`
</h3>

Désactiver l'outil [Artifact](/docs/fr/artifacts), qui publie la sortie de session en tant que page web privée sur claude.ai. Quand vous désactivez la ligne **Artifacts** dans `/config`, Claude Code écrit cette clé dans vos paramètres utilisateur, donc vous ne l'éditez généralement pas à la main. Nécessite Claude Code v2.1.196 ou ultérieur.

* **Portée** : [`Tout fichier`](#scopes). Chaque fichier peut désactiver l'outil, et aucun ne peut le réactiver.
* **Type** : Booléen
  * `false` : Claude Code désactive l'outil Artifact pour chaque session à laquelle le fichier s'applique
  * `true` : identique à laisser la clé non définie, car elle ne remplace jamais un `false` d'un autre fichier, de [`CLAUDE_CODE_DISABLE_ARTIFACT`](/docs/fr/env-vars), ou du [paramètre d'administrateur](/docs/fr/artifacts#manage-artifacts-for-your-organization) de votre organisation
* **Défaut** : non défini, donc l'outil suit la [disponibilité](/docs/fr/artifacts#availability) de votre compte

```json settings.json theme={null}
{
  "enableArtifact": false
}
```

Tant qu'une source autre que vos propres paramètres utilisateur garde l'outil désactivé, Claude Code masque la ligne **Artifacts** dans `/config`, car l'activer là ne changerait rien. [Désactiver les artifacts](/docs/fr/artifacts#disable-artifacts) énumère tous les moyens de désactiver l'outil. Avant v2.1.242, Claude Code ignorait cette clé dans les paramètres de projet et locaux, et un fichier plus haut dans la [pile de priorité](/docs/fr/settings#settings-precedence) pouvait réactiver l'outil sur le désactiver d'un fichier inférieur.

<h3 id="inputneedednotifenabled">
  `inputNeededNotifEnabled`
</h3>

Recevoir une notification push sur votre téléphone quand une invite de permission ou une question attend votre entrée. Claude Code envoie celles-ci uniquement pendant que le [Contrôle à distance](/docs/fr/remote-control) est connecté. Apparaît dans `/config` sous **Envoyer une notification quand des actions sont requises**.

* **Portée** : [`Tout fichier`](#scopes). Claude Code lit également une valeur laissée dans `~/.claude.json` par les versions antérieures.
* **Type** : Booléen
  * `true` : vous recevez une notification push sur votre téléphone quand une invite de permission ou une question attend, pendant que le Contrôle à distance est connecté
  * `false` : Claude Code n'envoie pas ces notifications
* **Défaut** : `false`

```json settings.json theme={null}
{
  "inputNeededNotifEnabled": true
}
```

Voir [Notifications push mobiles](/docs/fr/remote-control#mobile-push-notifications).

<h3 id="preferrednotifchannel">
  `preferredNotifChannel`
</h3>

Choisir comment Claude Code vous notifie quand une tâche se termine ou qu'une invite de permission attend. Apparaît dans `/config` sous **Notifications locales**.

* **Portée** : [`Tout fichier`](#scopes). Claude Code lit également une valeur laissée dans `~/.claude.json` par les versions antérieures.
* **Type** : chaîne, l'une des :
  * `"auto"` : Claude Code envoie une notification de bureau dans iTerm2, Ghostty et Kitty, sonne la cloche dans Terminal.app uniquement quand sa cloche audible est désactivée, et ne fait rien ailleurs
  * `"terminal_bell"` : Claude Code sonne le caractère de cloche dans n'importe quel terminal
  * `"iterm2"` : Claude Code envoie une notification de bureau iTerm2
  * `"iterm2_with_bell"` : Claude Code envoie une notification de bureau iTerm2 et sonne la cloche
  * `"kitty"` : Claude Code envoie une notification de bureau Kitty
  * `"ghostty"` : Claude Code envoie une notification de bureau Ghostty
  * `"notifications_disabled"` : Claude Code n'envoie aucune notification
* **Défaut** : `"auto"`

```json settings.json theme={null}
{
  "preferredNotifChannel": "terminal_bell"
}
```

Avec `"auto"`, Claude Code envoie une notification de bureau dans iTerm2, Ghostty et Kitty. Dans Terminal.app, il sonne le caractère de cloche uniquement quand vous avez désactivé la cloche audible du Terminal, et dans les autres terminaux, il ne fait rien. Définissez `"terminal_bell"` pour sonner le caractère de cloche dans n'importe quel terminal. Voir [Obtenir une cloche de terminal ou une notification](/docs/fr/terminal-config#get-a-terminal-bell-or-notification).

<h3 id="remote-defaultenvironmentid">
  `remote.defaultEnvironmentId`
</h3>

Choisir l'[environnement cloud](/docs/fr/cloud-environments) par défaut pour les sessions cloud que vous créez à partir de la CLI, comme avec `claude --cloud`. Claude Code écrit cette clé dans vos paramètres utilisateur quand vous choisissez un environnement avec [`/remote-env`](/docs/fr/cloud-environments#select-an-environment-from-the-cli).

* **Portée** : [`Tout fichier`](#scopes). Pour un ID d'environnement auto-hébergé, paramètres utilisateur ou gérés, ou l'indicateur `--settings` uniquement.
* **Type** : chaîne, un ID d'environnement tel que `env_...` ou `ccpool_...`
* **Défaut** : non défini, donc Claude Code utilise l'environnement hébergé par Anthropic quand votre liste en a un, et sinon le premier environnement de votre liste qui n'est pas un [environnement de pont de Contrôle à distance](/docs/fr/cloud-environments#the-default-environment), ou le premier environnement quand chacun est un environnement de pont
* **Remplacements par session** : `--environment` a la priorité sur cette clé pour la session cloud unique qu'il crée

```json settings.json theme={null}
{
  "remote": {
    "defaultEnvironmentId": "env_0123abcd"
  }
}
```

Un ID d'environnement hébergé par Anthropic, qui commence par `env_`, suit la priorité des paramètres standard, donc une valeur dans les paramètres de projet d'un référentiel remplace votre choix au niveau utilisateur. Un ID d'[environnement auto-hébergé](/docs/fr/self-hosted-environments), qui commence par `ccpool_`, n'est honoré que depuis les paramètres utilisateur, les paramètres gérés et l'indicateur `--settings` ; Claude Code l'ignore dans les paramètres de projet ou locaux d'un référentiel, et `/remote-env` affiche quelle valeur il a ignorée, donc un fichier archivé ne peut pas diriger les sessions vers un environnement auto-hébergé que vous n'avez pas choisi.

<h3 id="remotecontrolatstartup">
  `remoteControlAtStartup`
</h3>

Connecter le [Contrôle à distance](/docs/fr/remote-control) automatiquement quand chaque session interactive démarre, au lieu d'attendre `/remote-control`. Définissez-le sur `true` pour activer la connexion automatique, `false` pour la désactiver. Apparaît dans `/config` sous **Activer le Contrôle à distance pour toutes les sessions**.

* **Portée** : [`Tout fichier`](#scopes). Claude Code lit également une valeur laissée dans `~/.claude.json` par les versions antérieures.
* **Type** : Booléen
  * `true` : Claude Code connecte le Contrôle à distance automatiquement quand chaque session interactive démarre
  * `false` : Claude Code attend `/remote-control`
* **Défaut** : non défini, donc la connexion automatique suit la valeur par défaut d'administrateur de votre organisation quand une est définie, et sinon la valeur par défaut actuelle de Claude Code
* **Remplacements par session** : `--remote-control` active le Contrôle à distance pour une session même quand cette clé est `false`, et aucun indicateur ne le désactive pour une session

```json settings.json theme={null}
{
  "remoteControlAtStartup": true
}
```

Claude Code ignore un `true` des paramètres de projet ou locaux, donc un référentiel peut désactiver la connexion automatique pour son extraction mais ne peut pas l'activer. Pour le comportement complet par portée, voir [Activer le Contrôle à distance pour toutes les sessions](/docs/fr/remote-control#enable-remote-control-for-all-sessions) et les [clés de sécurité où la valeur plus stricte s'applique](/docs/fr/settings#security-keys-where-the-stricter-value-applies).

<h3 id="sshconfigs">
  `sshConfigs`
</h3>

Ajouter des connexions SSH à la liste déroulante de l'environnement [Bureau](/docs/fr/desktop#pre-configure-ssh-connections-for-your-team). Les administrateurs l'utilisent pour distribuer les connexions partagées à une équipe. Les connexions que vous définissez dans les paramètres gérés s'affichent comme gérées, donc les utilisateurs peuvent les sélectionner mais ne peuvent pas les modifier ou les supprimer dans l'application.

* **Portée** : [`Utilisateur ou géré`](#scopes). L'application de bureau lit cette clé.
* **Type** : tableau d'objets, chacun avec `id`, `name` et `sshHost` requis et `sshPort` et `sshIdentityFile` optionnels
* **Défaut** : non défini

Cet exemple ajoute une connexion nommée `Dev VM` qui se connecte à `user@dev.example.com` :

```json settings.json theme={null}
{
  "sshConfigs": [
    {
      "id": "dev-vm",
      "name": "Dev VM",
      "sshHost": "user@dev.example.com"
    }
  ]
}
```

<h3 id="sshhostallowlist">
  `sshHostAllowlist`
</h3>

Limiter les hôtes auxquels une [session SSH de Bureau](/docs/fr/desktop#restrict-which-ssh-hosts-users-can-connect-to) peut se connecter. Seule l'application de Bureau lit cette clé ; la CLI ne le fait pas. Les modèles ne sont pas sensibles à la casse : `*` correspond à n'importe quel hôte, `*.example.com` correspond à `example.com` et à chaque sous-domaine, et tout le reste est une correspondance exacte avec le nom d'hôte après la résolution de `~/.ssh/config`. Un tableau vide désactive les sessions SSH.

* **Portée** : [`Géré`](#scopes)
* **Type** : tableau de modèles de nom d'hôte
* **Défaut** : non défini, donc n'importe quel hôte est autorisé

Cet exemple autorise `devboxes.example.com` et ses sous-domaines, plus l'hôte exact `bastion.example.com` :

```json managed-settings.json theme={null}
{
  "sshHostAllowlist": ["*.devboxes.example.com", "bastion.example.com"]
}
```

<span id="authentication-and-login" />

<h2 id="authentication-and-providers">
  Authentification et fournisseurs
</h2>

Fournissez les identifiants via des scripts d'aide et, pour les organisations, forcez une méthode de connexion ou une organisation. Voir [Authentification](/docs/fr/authentication).

<h3 id="apikeyhelper">
  `apiKeyHelper`
</h3>

Exécutez votre propre commande pour produire l'identifiant que Claude Code envoie avec les demandes de modèle. Claude Code exécute la commande via le shell système, `/bin/sh` sur macOS et Linux et `cmd` sur Windows, et envoie sa sortie comme en-têtes `X-Api-Key` et `Authorization: Bearer`. Utilisez-le pour les identifiants dynamiques ou rotatifs, tels que les jetons de courte durée récupérés à partir d'un coffre-fort.

* **Portée** : [`Any file`](#scopes)
* **Type** : chaîne, une ligne de commande shell
* **Par défaut** : non défini, donc Claude Code n'exécute pas d'aide

```json settings.json theme={null}
{
  "apiKeyHelper": "/bin/generate_temp_api_key.sh"
}
```

Claude Code met en cache la valeur et réexécute la commande dans ces cas :

* Après la durée de vie du cache, cinq minutes par défaut ou l'intervalle que vous définissez avec [`CLAUDE_CODE_API_KEY_HELPER_TTL_MS`](/docs/fr/env-vars).
* Lorsqu'une demande à l'API Anthropic, directement ou via une [passerelle LLM](/docs/fr/llm-gateway), échoue avec `401` ou `403`.
* Avant d'envoyer une demande à l'API Anthropic, directement ou via une passerelle LLM, lorsque la sortie mise en cache est un JWT qui a expiré après que l'aide l'ait produit. Nécessite Claude Code v2.1.246 ou ultérieur.

Les deux derniers cas s'appliquent uniquement lorsque la sortie de l'aide est l'identifiant que Claude Code envoie et que `ANTHROPIC_AUTH_TOKEN` n'est pas défini.

Dans les sessions interactives, lorsque la commande provient des paramètres du projet ou locaux, Claude Code ne l'exécute pas jusqu'à ce que vous acceptiez l'invite de confiance de l'espace de travail. Voir [Gestion des identifiants](/docs/fr/authentication#credential-management).

<h3 id="awsauthrefresh">
  `awsAuthRefresh`
</h3>

Exécutez votre propre commande, telle que `aws sso login`, pour actualiser les identifiants dans votre répertoire `.aws` lorsque ceux que Claude Code a pour [Amazon Bedrock](/docs/fr/amazon-bedrock) cessent de fonctionner. Claude Code vérifie d'abord les identifiants actuels par rapport à STS et n'exécute la commande que lorsque cette vérification échoue, puis lit le répertoire `.aws` actualisé.

* **Portée** : [`Any file`](#scopes)
* **Type** : chaîne, une ligne de commande shell
* **Par défaut** : non défini, donc Claude Code n'actualise pas les identifiants AWS pour vous

```json settings.json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile myprofile"
}
```

Utilisez cette clé lorsque votre flux d'actualisation écrit dans `.aws` ; utilisez [`awsCredentialExport`](#awscredentialexport) lorsqu'il imprime plutôt les identifiants. Voir [configuration avancée des identifiants](/docs/fr/amazon-bedrock#advanced-credential-configuration).

<h3 id="awscredentialexport">
  `awsCredentialExport`
</h3>

Exécutez votre propre commande qui imprime les identifiants AWS en JSON, afin que Claude Code puisse appeler [Amazon Bedrock](/docs/fr/amazon-bedrock) avec des identifiants qui ne vivent pas dans votre répertoire `.aws`. Claude Code accepte la forme de sortie `aws sts` et la forme plate `aws configure export-credentials`, et limite les identifiants à son propre client Bedrock, de sorte que les commandes shell que Claude Code exécute voient toujours vos identifiants ambiants.

* **Portée** : [`Any file`](#scopes)
* **Type** : chaîne, une ligne de commande shell
* **Par défaut** : non défini, donc Claude Code utilise la chaîne d'identifiants AWS ambiante

```json settings.json theme={null}
{
  "awsCredentialExport": "/bin/generate_aws_grant.sh"
}
```

Contrairement à [`awsAuthRefresh`](#awsauthrefresh), Claude Code exécute toujours cette commande lorsqu'elle est définie, sans vérifier d'abord les identifiants ambiants. Voir [configuration avancée des identifiants](/docs/fr/amazon-bedrock#advanced-credential-configuration).

<h3 id="forceloginmethod">
  `forceLoginMethod`
</h3>

Limitez le type de compte avec lequel les gens peuvent se connecter. Définissez `"claudeai"` pour autoriser uniquement les comptes claude.ai, `"console"` pour autoriser uniquement les comptes Claude Console, ou `"gateway"` pour envoyer les gens vers une [passerelle cloud](/docs/fr/claude-apps-gateway) au lieu d'une connexion propriétaire. Les administrateurs la définissent dans les paramètres gérés et l'associent à [`forceLoginOrgUUID`](#forceloginorguuid) pour garder les connexions claude.ai des développeurs au sein d'une seule organisation. Si vous la définissez sur `"claudeai"` ou `"console"` dans n'importe quel fichier de paramètres, Claude Code arrête également d'offrir la [connexion Console sans clé](/docs/fr/authentication#sign-in-without-an-api-key) dans les sessions auxquelles ce fichier s'applique.

* **Portée** : [`Any file`](#scopes). Claude Code honore `"gateway"` uniquement à partir d'une source gérée sur la machine : `managed-settings.json`, la plist macOS ou le registre Windows HKLM, ou un aide de politique. Il traite `"gateway"` comme non défini dans les paramètres utilisateur, projet, local, HKCU et gérés par serveur, la même règle que [`forceLoginGatewayUrl`](#forcelogingatewayurl).
* **Type** : chaîne, l'une des :
  * `"claudeai"` : seuls les comptes claude.ai peuvent se connecter
  * `"console"` : seuls les comptes Claude Console peuvent se connecter
  * `"gateway"` : Claude Code envoie les gens vers une passerelle cloud au lieu d'une connexion propriétaire
* **Par défaut** : non défini, donc les gens choisissent une méthode de connexion

```json settings.json theme={null}
{
  "forceLoginMethod": "claudeai"
}
```

Chaque chemin de connexion propriétaire applique la restriction, y compris l'[extension VS Code](/docs/fr/vs-code), le SDK Agent, `claude setup-token`, et `/install-github-app`, sauf l'écran de connexion interactif du terminal, accessible via `/login` ou l'intégration au premier lancement, qui présélectionne la méthode sans l'appliquer. Avant v2.1.212, seules les connexions au terminal l'appliquaient. Voir [Restreindre la connexion à votre organisation](/docs/fr/authentication#restrict-login-to-your-organization) pour savoir comment chaque chemin de connexion, les identifiants d'environnement et les fournisseurs tiers sont traités.

Lorsqu'une source gérée sur la machine définit `"gateway"`, Claude Code n'utilise pas une connexion restante, une clé API ou un identifiant d'aide `apiKeyHelper`. Voir [La politique de l'administrateur nécessite une connexion à la passerelle Cloud](/docs/fr/errors#administrator-policy-requires-a-cloud-gateway-sign-in) pour le message que chacun produit. Si vous sélectionnez un fournisseur cloud via `CLAUDE_CODE_USE_BEDROCK` ou une variable d'environnement similaire, la session n'a pas besoin de la connexion à la passerelle. Avant v2.1.261, Claude Code utilisait une connexion restante sur ces machines.

<h3 id="forcelogingatewayurl">
  `forceLoginGatewayUrl`
</h3>

Définissez l'URL de la passerelle à laquelle l'écran `/login` Cloud gateway se connecte, afin que les gens atteignent votre [passerelle cloud](/docs/fr/claude-apps-gateway) sans taper son adresse. L'écran n'a pas de champ URL : avec cette clé définie, il affiche l'URL de votre passerelle et se connecte lorsque la personne appuie sur Entrée ; sans elle, il leur dit de contacter leur administrateur informatique.

Soit cette clé, soit `forceLoginMethod: "gateway"` rend la machine réservée à la passerelle, donc `/login` s'ouvre sur l'écran Cloud gateway sans sélecteur de méthode de connexion. Voir [La politique de l'administrateur nécessite une connexion à la passerelle Cloud](/docs/fr/errors#administrator-policy-requires-a-cloud-gateway-sign-in) pour ce qui se passe avec une connexion propriétaire restante ou une clé API. Définissez les deux clés afin que l'écran se connecte au lieu d'afficher une erreur.

* **Portée** : [`Managed`](#scopes). Lecture uniquement à partir d'une source sur la machine : `managed-settings.json`, la plist macOS ou le registre Windows HKLM, ou un aide de politique. Claude Code l'ignore dans les paramètres HKCU et gérés par serveur.
* **Type** : chaîne, une URL complète incluant le schéma
* **Par défaut** : non défini, donc l'écran Cloud gateway affiche une erreur indiquant aux gens de contacter leur administrateur informatique

```json managed-settings.json theme={null}
{
  "forceLoginGatewayUrl": "https://claude-gateway.example.com"
}
```

Si la valeur n'est pas une URL valide, l'écran de connexion la signale, et le reste du fichier de paramètres gérés s'applique toujours. Voir [Définir l'URL de la passerelle](/docs/fr/claude-apps-gateway#set-the-gateway-url).

<h3 id="forceloginorguuid">
  `forceLoginOrgUUID`
</h3>

À partir d'une source gérée, exigez que les connexions de compte claude.ai appartiennent à une organisation Anthropic, donnée comme un UUID unique, ou à l'une de plusieurs organisations, donnée comme un tableau. À partir de n'importe quel fichier de paramètres, Claude Code utilise également un UUID unique pour présélectionner cette organisation lors d'une connexion claude.ai ou Claude Console, et ne présélectionne rien pour un tableau. Si vous définissez la clé dans n'importe quel fichier de paramètres, Claude Code arrête également d'offrir la [connexion Console sans clé](/docs/fr/authentication#sign-in-without-an-api-key) dans les sessions auxquelles ce fichier s'applique et crée plutôt une clé API.

* **Portée** : [`Any file`](#scopes). Seule une source gérée applique la restriction ; un UUID unique dans n'importe quel autre fichier de paramètres présélectionne l'organisation lors de la connexion sans la restreindre.
* **Type** : chaîne, un UUID, ou tableau de chaînes, plusieurs UUID
* **Par défaut** : non défini, donc n'importe quelle organisation peut se connecter

Cet exemple accepte les connexions de l'une ou l'autre de deux organisations sans en présélectionner une :

```json managed-settings.json theme={null}
{
  "forceLoginOrgUUID": ["xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx", "yyyyyyyy-yyyy-yyyy-yyyy-yyyyyyyyyyyy"]
}
```

Si une source gérée définit un tableau vide, ou une valeur que Claude Code ne peut pas analyser, Claude Code bloque chaque connexion avec un message de mauvaise configuration.

Voir [Restreindre la connexion à votre organisation](/docs/fr/authentication#restrict-login-to-your-organization) pour savoir comment Claude Code traite les connexions Claude Console, les autres chemins de connexion et les identifiants d'environnement.

<h3 id="gatewayinternalnetworks">
  `gatewayInternalNetworks`
</h3>

Déclarez les blocs IPv4 publics à partir desquels votre organisation numérote son réseau interne, afin que `/login` accepte une [passerelle cloud](/docs/fr/claude-apps-gateway) là-bas. Nécessite Claude Code v2.1.268 ou ultérieur.

Sans cette clé, `/login` se connecte à n'importe quelle passerelle sur une adresse privée et rien d'autre. Avec elle, `/login` accepte également une passerelle à l'intérieur d'un bloc listé, sur une connexion directe uniquement. L'adresse propre de la machine sur cette connexion doit également être à l'intérieur du même bloc.

* **Portée** : [`Managed`](#scopes). Lecture uniquement à partir d'une source sur la machine : `managed-settings.json`, la plist macOS ou le registre Windows HKLM, ou un aide de politique. Claude Code l'ignore dans les paramètres HKCU et gérés par serveur.
* **Type** : tableau de chaînes, au maximum quatre blocs IPv4 CIDR, chacun `/8` à `/32`, ne se chevauchant pas les uns les autres, et aucun ne chevauchant l'espace privé.
* **Par défaut** : non défini, donc `/login` accepte uniquement les passerelles sur des adresses privées

```json managed-settings.json theme={null}
{
  "gatewayInternalNetworks": ["203.0.113.0/24"]
}
```

Remplacez la plage de documentation dans l'exemple par votre propre bloc. Claude Code refuse les plages de documentation, les plages que les clients VPN et NAT64 utilisent localement, et l'espace réservé qu'aucun réseau n'est numéroté à partir de, comme la multidiffusion.

Si une entrée est invalide, ou la valeur n'est pas une liste de chaînes, `/login` nomme le problème et refuse chaque nouvelle connexion à la passerelle sur la machine jusqu'à ce que vous corrigiez la valeur. Les connexions existantes continuent de fonctionner. Voir [Autoriser une passerelle sur l'espace d'adresses publiques que vous possédez](/docs/fr/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) pour les règles complètes et ce que les développeurs voient.

<h3 id="gcpauthrefresh">
  `gcpAuthRefresh`
</h3>

Exécutez votre propre commande pour actualiser les identifiants Google Cloud Application Default lorsque Claude Code découvre qu'ils ont expiré ou ne peuvent pas être chargés, afin que les demandes de [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai) continuent de fonctionner sans que vous vous réauthentifiiez manuellement.

* **Portée** : [`Any file`](#scopes)
* **Type** : chaîne, une ligne de commande shell
* **Par défaut** : non défini, donc l'erreur d'identifiant de Claude Code vous dit d'exécuter `gcloud auth application-default login` vous-même

```json settings.json theme={null}
{
  "gcpAuthRefresh": "gcloud auth application-default login"
}
```

Voir [configuration avancée des identifiants](/docs/fr/google-vertex-ai#advanced-credential-configuration).

<h3 id="otelheadershelper">
  `otelHeadersHelper`
</h3>

Exécutez votre propre commande pour générer les en-têtes que Claude Code envoie avec les exportations OpenTelemetry, pour les backends dont les jetons tournent. Claude Code l'exécute au démarrage et périodiquement après cela, et s'attend à un objet JSON de valeurs d'en-têtes de chaîne sur stdout.

* **Portée** : [`Any file`](#scopes)
* **Type** : chaîne, un chemin exécutable ou une ligne de commande shell
* **Par défaut** : non défini, donc Claude Code n'ajoute pas d'en-têtes générés par l'aide

```json settings.json theme={null}
{
  "otelHeadersHelper": "/bin/generate_otel_headers.sh"
}
```

Définissez l'intervalle d'actualisation avec [`CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`](/docs/fr/env-vars). Voir [En-têtes dynamiques](/docs/fr/monitoring-usage#dynamic-headers) pour les exigences du script et ce que les développeurs voient lorsque l'aide échoue.

<h2 id="updates-and-versioning">
  Mises à jour et versioning
</h2>

Choisissez un canal de mise à jour et, pour les organisations, épinglez les versions que les utilisateurs peuvent exécuter. Voir [Mettre à jour Claude Code](/docs/fr/setup#update-claude-code).

<h3 id="autoupdateschannel">
  `autoUpdatesChannel`
</h3>

Choisissez quel [canal de version](/docs/fr/setup#configure-release-channel) les mises à jour automatiques en arrière-plan et `claude update` suivent. Définissez `"stable"` pour une version qui a généralement environ une semaine et ignore les versions avec des régressions majeures, ou `"latest"` pour la version la plus récente.

* **Portée** : [`Any file`](#scopes). Définissez-le dans les paramètres gérés pour appliquer un canal dans toute votre organisation.
* **Type** : string, l'un des :
  * `"latest"` : les mises à jour suivent la version la plus récente
  * `"stable"` : les mises à jour suivent une version qui a généralement environ une semaine et ignore les versions avec des régressions majeures
* **Défaut** : non défini, donc Claude Code suit `"latest"`

```json settings.json theme={null}
{
  "autoUpdatesChannel": "stable"
}
```

Claude Code écrit `"stable"` dans vos paramètres utilisateur lorsque vous le choisissez sous **Auto-update channel** dans `/config`, et supprime la clé lorsque vous revenez à latest. `claude install stable` et `claude install latest` enregistrent également le canal que vous nommez. Le passage de `"latest"` à `"stable"` dans `/config` demande si vous autorisez une rétrogradation ou si vous restez sur votre version actuelle ; rester définit [`minimumVersion`](#minimumversion). Les installations Homebrew ignorent cette clé : le cask `claude-code` suit stable et `claude-code@latest` suit latest, et `claude update` s'en remet à `brew upgrade`. Pour désactiver complètement les mises à jour automatiques, définissez [`DISABLE_AUTOUPDATER`](/docs/fr/setup#disable-auto-updates) dans `env`.

<h3 id="minimumversion">
  `minimumVersion`
</h3>

Empêchez les mises à jour automatiques en arrière-plan et `claude update` d'installer une version inférieure à celle-ci, de sorte que le passage au canal `"stable"` ne vous rétrograde pas à partir d'une version `"latest"` plus récente. Claude Code écrit cette clé pour vous lorsque vous choisissez de rester sur votre version actuelle lors du changement de canal dans `/config`, et la supprime lorsque vous revenez à `"latest"`.

* **Portée** : [`Any file`](#scopes). Définissez-le dans les paramètres gérés pour épingler un minimum à l'échelle de l'organisation que les paramètres utilisateur et projet ne peuvent pas réduire.
* **Type** : string, un numéro de version tel que `"2.1.100"` ; une valeur qui n'est pas une version valide est ignorée
* **Défaut** : non défini, donc les mises à jour peuvent installer n'importe quelle version que le canal propose

Cet exemple suit le canal stable et refuse d'installer une version inférieure à 2.1.100 :

```json settings.json theme={null}
{
  "autoUpdatesChannel": "stable",
  "minimumVersion": "2.1.100"
}
```

Cette clé ne contraint que les mises à jour. Pour faire refuser à Claude Code de démarrer en dessous d'une version, utilisez [`requiredMinimumVersion`](#requiredminimumversion) à la place. Voir [Épingler une version minimale](/docs/fr/setup#pin-a-minimum-version).

<h3 id="requiredmaximumversion">
  `requiredMaximumVersion`
</h3>

Définissez la version la plus récente de Claude Code que votre organisation autorise à démarrer. Lorsque la version en cours d'exécution est plus récente, Claude Code se ferme au démarrage et indique à l'utilisateur d'installer une version approuvée par la méthode approuvée de votre organisation ; `claude install <version>` peut également fonctionner. Nécessite Claude Code v2.1.163 ou version ultérieure.

* **Portée** : [`Managed`](#scopes). Claude Code ne donne aucun avertissement lorsqu'il ignore la clé ailleurs.
* **Type** : string, un numéro de version tel que `"2.1.150"` ; une valeur qui n'est pas une version valide est ignorée
* **Défaut** : non défini, donc aucun plafond ne s'applique

```json managed-settings.json theme={null}
{
  "requiredMaximumVersion": "2.1.150"
}
```

Les mises à jour automatiques en arrière-plan et `claude update` ignorent les versions au-dessus du plafond, de sorte qu'une installation dans la plage reste à l'intérieur. `claude update`, `claude install` et `claude doctor` continuent de fonctionner au-dessus du plafond pour que les utilisateurs puissent récupérer. Associez-le à [`requiredMinimumVersion`](#requiredminimumversion) pour appliquer une plage.

<h3 id="requiredminimumversion">
  `requiredMinimumVersion`
</h3>

Définissez la version la plus ancienne de Claude Code que votre organisation autorise à démarrer. Lorsque la version en cours d'exécution est plus ancienne, Claude Code se ferme au démarrage et indique à l'utilisateur de mettre à jour par la méthode approuvée de votre organisation. La vérification s'exécute uniquement au démarrage, de sorte qu'une session déjà en cours d'exécution continue. Nécessite Claude Code v2.1.163 ou version ultérieure.

* **Portée** : [`Managed`](#scopes). Claude Code ne donne aucun avertissement lorsqu'il ignore la clé ailleurs.
* **Type** : string, un numéro de version tel que `"2.1.150"` ; une valeur qui n'est pas une version valide est ignorée
* **Défaut** : non défini, donc aucun plancher ne s'applique

```json managed-settings.json theme={null}
{
  "requiredMinimumVersion": "2.1.150"
}
```

`claude update`, `claude install` et `claude doctor` continuent de fonctionner en dessous du plancher pour que les utilisateurs puissent récupérer. Contrairement à [`minimumVersion`](#minimumversion), qui empêche uniquement les rétrograder, cette clé bloque le démarrage. Associez-le à [`requiredMaximumVersion`](#requiredmaximumversion) pour appliquer une plage.

<h2 id="tools">
  Outils
</h2>

Désactivez des outils spécifiques dans l'[application de bureau Claude Code](/docs/fr/desktop). L'interface de ligne de commande du terminal ignore ces clés. Pour les outils eux-mêmes, consultez [Outils disponibles pour Claude](/docs/fr/tools-reference).

<h3 id="browserexternalpagetools">
  `browserExternalPageTools`
</h3>

Empêchez Claude d'utiliser ses outils pour lire ou agir sur des pages externes dans le [volet Navigateur](/docs/fr/desktop#browse-external-sites) de l'application de bureau. Les personnes de votre organisation peuvent toujours ouvrir des sites externes eux-mêmes, et les aperçus des serveurs de développement locaux continuent de fonctionner avec les outils de Claude. L'application de bureau lit cette clé ; l'interface de ligne de commande du terminal l'ignore.

* **Portée** : [`Managed`](#scopes)
* **Type** : chaîne de caractères, `"disabled"` ; l'application de bureau accepte également `"disable"`, dans les deux cas
* **Valeur par défaut** : non définie, donc les outils de Claude fonctionnent sur les pages externes

```json managed-settings.json theme={null}
{
  "browserExternalPageTools": "disabled"
}
```

Toute autre valeur laisse les outils de Claude activés, et une chaîne non vide qui n'est pas l'une des deux valeurs acceptées enregistre un avertissement. Pour bloquer les sites externes pour les personnes et Claude, définissez [`disableBrowserExternalNavigation`](#disablebrowserexternalnavigation) à la place. Consultez [Restreindre la navigation externe pour votre organisation](/docs/fr/desktop#restrict-external-browsing-for-your-organization).

<h3 id="disablebrowserexternalnavigation">
  `disableBrowserExternalNavigation`
</h3>

Désactivez la navigation externe dans le [volet Navigateur](/docs/fr/desktop#browse-external-sites) de l'application de bureau pour les personnes et Claude. Les aperçus des serveurs de développement localhost continuent de fonctionner. L'application de bureau lit cette clé ; l'interface de ligne de commande du terminal l'ignore.

* **Portée** : [`Managed`](#scopes)
* **Type** : Booléen ; seul le Booléen JSON `true` prend effet
  * `true` : l'application de bureau désactive la navigation externe dans le volet Navigateur pour les personnes et Claude ; les aperçus localhost continuent de fonctionner
  * `false` : la navigation externe reste activée
* **Valeur par défaut** : non définie, donc la navigation externe est activée

```json managed-settings.json theme={null}
{
  "disableBrowserExternalNavigation": true
}
```

L'application de bureau ignore toute autre valeur, et une valeur qui n'est pas un Booléen, comme la chaîne `"true"` ou `1`, enregistre également un avertissement. Pour laisser la navigation externe activée mais garder les outils de Claude désactivés sur les pages externes, définissez [`browserExternalPageTools`](#browserexternalpagetools) à la place. Consultez [Restreindre la navigation externe pour votre organisation](/docs/fr/desktop#restrict-external-browsing-for-your-organization).

<h3 id="disablemobilesimulatortools">
  `disableMobileSimulatorTools`
</h3>

Bloquez les outils de Claude pour le [volet Simulateur iOS](/docs/fr/desktop-ios-simulator#turn-off-simulator-access) de l'application de bureau. Les personnes conservent l'utilisation manuelle du volet ; seul l'accès de Claude est supprimé, et personne ne peut le réactiver depuis l'intérieur de l'application. L'application de bureau lit cette clé ; l'interface de ligne de commande du terminal l'ignore.

* **Portée** : [`Managed`](#scopes)
* **Type** : Booléen ; seul le Booléen JSON `true` prend effet
  * `true` : l'application de bureau bloque les outils de Claude pour le volet Simulateur iOS
  * `false` : les outils de simulateur de Claude suivent le paramètre de basculement des paramètres de chaque personne dans l'application de bureau
* **Valeur par défaut** : non définie, donc les outils de simulateur de Claude suivent le paramètre de basculement des paramètres de chaque personne dans l'application de bureau

```json managed-settings.json theme={null}
{
  "disableMobileSimulatorTools": true
}
```

L'application de bureau ignore toute autre valeur, et une valeur qui n'est pas un Booléen, comme la chaîne `"true"` ou `1`, enregistre également un avertissement.

<span id="data-and-privacy" />

<h2 id="privacy-and-telemetry">
  Confidentialité et télémétrie
</h2>

Contrôlez la durée pendant laquelle Claude Code conserve les données de session et ce qu'il envoie. Les commutateurs qui désactivent les métriques d'utilisation et les rapports d'erreurs sont des variables d'environnement, pas des clés de paramètres : définissez `DISABLE_TELEMETRY`, `DISABLE_ERROR_REPORTING`, ou `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` dans la clé [`env`](#env) ou dans le shell. [Les services de télémétrie](/docs/fr/data-usage#telemetry-services) indiquent ce que chacun arrête. Deux exceptions se désactivent à partir d'un fichier de paramètres : [`feedbackDrafts`](#feedbackdrafts) ci-dessous pour les commentaires rédigés par Claude, et [`feedbackSurveyRate`](#feedbacksurveyrate) ci-dessous pour l'enquête de session.

<h3 id="cleanupperioddays">
  `cleanupPeriodDays`
</h3>

Définissez le nombre de jours pendant lesquels Claude Code conserve les [transcriptions de session et autres données d'application](/docs/fr/claude-directory#cleaned-up-automatically) avant de les supprimer. Claude Code exécute la suppression en tant que balayage en arrière-plan après le démarrage d'une session, tant qu'il peut déterminer en toute sécurité la période de rétention.

* **Portée** : [`Any file`](#scopes)
* **Type** : nombre de jours, un nombre entier, minimum `1`
* **Valeur par défaut** : `30`

```json settings.json theme={null}
{
  "cleanupPeriodDays": 20
}
```

La définition de `0` échoue la validation, donc choisissez une grande valeur comme `3650` pour une rétention longue. Pour empêcher Claude Code d'écrire des transcriptions du tout, consultez [Stockage en texte brut](/docs/fr/claude-directory#plaintext-storage).

<h3 id="desktopsessioncleanupperioddays">
  `desktopSessionCleanupPeriodDays`
</h3>

Définissez une limite d'âge en jours pour les transcriptions des sessions que vous avez démarrées ou continuées le plus récemment dans Claude Desktop ou Cowork. Sans cette clé, Claude Code [conserve ces transcriptions à tout âge](/docs/fr/claude-directory#cleaned-up-automatically). Claude Code supprime chacune une fois qu'elle est plus ancienne que cette limite et [`cleanupPeriodDays`](#cleanupperioddays), donc avec `cleanupPeriodDays` à sa valeur par défaut de 30, une valeur de `7` les conserve toujours 30 jours. Lorsque les paramètres gérés définissent `cleanupPeriodDays`, cette période s'applique à la place et cette clé est ignorée. Nécessite Claude Code v2.1.248 ou version ultérieure.

* **Portée** : [`User or managed`](#scopes). Claude Code lit également la clé à partir d'un fichier que vous transmettez avec `--settings`, et l'ignore dans les paramètres de projet et locaux.
* **Type** : nombre de jours, un nombre entier, minimum `0`
* **Valeur par défaut** : `0`, qui ne définit aucune limite d'âge

```json settings.json theme={null}
{
  "desktopSessionCleanupPeriodDays": 90
}
```

<h3 id="feedbackdrafts">
  `feedbackDrafts`
</h3>

Contrôlez les [commentaires rédigés par Claude](/docs/fr/tools-reference#sendfeedback-tool-behavior) : si Claude peut mettre en file d'attente les brouillons de commentaires pour que vous les examiniez, et si Claude Code affiche une carte lorsque Claude en met une en file d'attente.

* **Portée** : [`User or managed`](#scopes)
* **Type** : chaîne, l'une de `"notify"`, `"quiet"`, ou `"off"`
  * `"notify"` : Claude Code affiche une carte au-dessus de l'invite lorsque Claude met un brouillon en file d'attente, jusqu'à [trois cartes dans une session](/docs/fr/tools-reference#what-you-see-when-claude-drafts) par défaut
  * `"quiet"` : Claude rédige sans carte. Vous voyez le nombre de brouillons en file d'attente dans le pied de page de l'invite et les examinez dans `/feedback`
  * `"off"` : Claude Code supprime l'outil SendFeedback, donc Claude ne peut pas mettre en file d'attente les brouillons
* **Valeur par défaut** : `"notify"`
* **Remplacements par session** : [`CLAUDE_CODE_SEND_FEEDBACK`](/docs/fr/env-vars) défini à `0` désactive la fonctionnalité pour une session

```json settings.json theme={null}
{
  "feedbackDrafts": "quiet"
}
```

Apparaît dans `/config` sous **Claude-drafted feedback**, qui écrit cette clé dans vos paramètres utilisateur. Vous voyez la ligne `/config` uniquement dans les sessions [où Claude peut rédiger des commentaires](/docs/fr/tools-reference#sessions-without-claude-drafted-feedback) ; la définition de `"off"` ne la masque pas, vous pouvez donc réactiver la fonctionnalité à partir de la même ligne. Une valeur dans les paramètres gérés prend précédence sur votre paramètre utilisateur, donc lorsqu'un administrateur définit cette clé, la ligne affiche la valeur gérée et la modifier n'a aucun effet. Claude Code ignore cette clé dans les paramètres de projet et locaux.

<h3 id="feedbacksurveyrate">
  `feedbackSurveyRate`
</h3>

Définissez la probabilité que l'[enquête de qualité de session](/docs/fr/data-usage#session-quality-surveys) apparaisse lorsqu'une session est admissible. Définissez `0` pour empêcher l'enquête d'apparaître.

* **Portée** : [`Any file`](#scopes)
* **Type** : nombre entre `0` et `1`
* **Valeur par défaut** : non défini, donc Claude Code utilise le taux qu'Anthropic définit à distance, ou son taux intégré de `0.005` sur Amazon Bedrock, Google Cloud's Agent Platform, et Microsoft Foundry, qui ne reçoivent pas de configuration à distance
* **Remplacements par session** : [`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY`](/docs/fr/env-vars) défini à `1` désactive l'enquête pour une session quel que soit le taux que cette clé définit

```json settings.json theme={null}
{
  "feedbackSurveyRate": 0.05
}
```

Le même taux s'applique à l'enquête dans l'extension VS Code.

<h3 id="skipwebfetchpreflight">
  `skipWebFetchPreflight`
</h3>

Ignorez la [vérification de sécurité du domaine WebFetch](/docs/fr/data-usage#webfetch-domain-safety-check), qui envoie chaque nom d'hôte demandé à `api.anthropic.com` avant la récupération. Définissez `true` dans les environnements qui bloquent le trafic vers Anthropic, comme Amazon Bedrock, Google Cloud's Agent Platform, ou les déploiements Microsoft Foundry avec une sortie restrictive.

* **Portée** : [`Any file`](#scopes)
* **Type** : Booléen
  * `true` : Claude Code ignore la vérification de sécurité du domaine WebFetch
  * `false` : la vérification s'exécute avant la première récupération à chaque nom d'hôte dans une session, et à nouveau pour un nom d'hôte dont la vérification antérieure a été bloquée ou a échoué
* **Valeur par défaut** : non défini, donc la vérification s'exécute avant la première récupération à chaque nom d'hôte dans une session

```json settings.json theme={null}
{
  "skipWebFetchPreflight": true
}
```

Avec la vérification ignorée, WebFetch tente n'importe quelle URL sans consulter la liste de blocage, donc associez-la aux [règles de permission `WebFetch`](/docs/fr/permissions#webfetch) si vous devez restreindre les domaines que Claude peut atteindre.

<span id="managed-policy" />

<h2 id="enterprise-and-managed-settings">
  Paramètres d'entreprise et gérés
</h2>

Clés qu'une organisation utilise pour calculer, actualiser et combiner les paramètres gérés. Voir [Configurer les paramètres gérés](/docs/fr/admin-setup).

<h3 id="disablesideloadflags">
  `disableSideloadFlags`
</h3>

Rejeter les drapeaux CLI `--plugin-dir`, `--plugin-url`, `--agents` et `--mcp-config` au démarrage, que les utilisateurs pourraient autrement transmettre pour contourner [`strictKnownMarketplaces`](#strictknownmarketplaces) pour une seule exécution. Claude Code se termine avec une erreur nommant les drapeaux rejetés et applique la même vérification aux surfaces qui démarrent le CLI avec ces drapeaux en interne, actuellement les sessions locales [Cowork](/docs/fr/desktop) dans l'application de bureau. Dans les [sessions cloud](/docs/fr/claude-code-on-the-web), Claude Code supprime les serveurs MCP que le serveur a livrés via `--mcp-config`, à l'exception des entrées `type: "sdk"` en processus, et démarre la session. Nécessite Claude Code v2.1.193 ou ultérieur.

* **Portée** : [`Managed`](#scopes)
* **Type** : Booléen
  * `true` : Claude Code rejette `--plugin-dir`, `--plugin-url`, `--agents` et `--mcp-config` au démarrage et se termine avec une erreur les nommant, sauf que dans les sessions cloud il supprime les serveurs MCP que le serveur a livrés via `--mcp-config`, à l'exception des entrées `type: "sdk"` en processus, et démarre la session
  * `false` : Claude Code accepte ces drapeaux
* **Défaut** : `false`

```json managed-settings.json theme={null}
{
  "disableSideloadFlags": true
}
```

Claude Code accepte toujours un `--mcp-config` dont les serveurs sont tous des entrées `type: "sdk"` en processus, de sorte que le SDK Agent et l'extension VS Code continuent de fonctionner. Les utilisateurs peuvent toujours ajouter des serveurs avec `claude mcp add` ou un fichier `.mcp.json` ; pour un contrôle par serveur, définissez également [`allowedMcpServers`](/docs/fr/managed-mcp). Nécessite Claude Code v2.1.193 ou ultérieur.

La même vérification couvre les dossiers de plugins nommés dans la variable d'environnement [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/fr/env-vars#variables), ce qui nécessite Claude Code v2.1.280 ou ultérieur. Lorsque la variable nomme un dossier, Claude Code se termine avec la même erreur, et l'erreur dit de désactiver la variable.

Dans les sessions cloud, Claude Code ignore également les mises à jour MCP livrées par le serveur en milieu de session, le chemin derrière la configuration de session cloud et les appels SDK `setMcpServers()` qui atteignent ces sessions. Les entrées `type: "sdk"` en processus restent exemptes là aussi. Avant v2.1.239, un `--mcp-config` livré par le serveur bloquait le démarrage d'une session cloud.

<h3 id="forceremotesettingsrefresh">
  `forceRemoteSettingsRefresh`
</h3>

Bloquer le démarrage du CLI jusqu'à ce que Claude Code ait récemment récupéré les [paramètres gérés par le serveur](/docs/fr/server-managed-settings). Si la récupération échoue, Claude Code se termine au lieu de continuer avec les paramètres en cache ou aucun paramètre. Définissez-le lorsque votre environnement ne peut pas accepter même une brève fenêtre au cours de laquelle une session s'exécute sans sa politique gérée.

Lorsque la clé n'est pas définie, Claude Code ne bloque pas le démarrage sur la récupération, bien que lorsque le développeur se connecte au démarrage, il attend jusqu'à cinq secondes pour la récupération. Une session de passerelle Cloud attend toujours et se termine si la passerelle ne peut pas être atteinte.

* **Portée** : [`Managed`](#scopes). Claude Code honore un `true` de n'importe quelle source gérée contrôlée par l'administrateur, même celle qui n'est pas la source de plus haute priorité.
* **Type** : Booléen
  * `true` : Claude Code bloque le démarrage jusqu'à ce qu'il ait récemment récupéré les paramètres gérés par le serveur, et se termine si la récupération échoue
  * `false` : Claude Code ne bloque pas le démarrage sur la récupération, bien qu'au démarrage d'une connexion il attende jusqu'à cinq secondes pour la récupération
* **Défaut** : `false`

```json managed-settings.json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

Définissez-le dans un profil MDM ou le fichier de paramètres gérés pour appliquer un démarrage fermé avant l'arrivée du premier payload du serveur. Claude Code applique la vérification uniquement dans les sessions qui récupèrent les paramètres gérés par le serveur, de sorte qu'une session qui [ne les récupère pas](/docs/fr/server-managed-settings#platform-availability) démarre sans attendre. Les sous-commandes `claude auth` sont exemptes, de sorte que les utilisateurs peuvent se réauthentifier lorsque les identifiants expirés sont la raison de l'échec de la récupération. Voir [Appliquer un démarrage fermé](/docs/fr/server-managed-settings#enforce-fail-closed-startup).

<h3 id="managedsourcesbehavior">
  `managedSourcesBehavior`
</h3>

Choisissez si Claude Code applique uniquement la [source gérée](/docs/fr/managed-settings#how-claude-code-combines-managed-sources) de plus haute priorité que votre organisation livre, ou combine chaque source d'administrateur qu'elle livre. Par défaut, Claude Code prend la source de plus haute priorité qui porte une [clé de politique](/docs/fr/managed-settings#how-claude-code-combines-managed-sources) et ignore le reste. Une clé de politique est n'importe quelle clé de paramètres autre que celle-ci et `wslInheritsWindowsSettings`. Donc une fois que les paramètres gérés par le serveur ou une politique MDM livrent une clé de politique, un fichier `managed-settings.json` ne contribue que les [clés que Claude Code lit de chaque source d'administrateur](/docs/fr/managed-settings#keys-read-from-every-admin-source). Avec `"merge"`, chaque source d'administrateur que vous livrez contribue ses clés à une politique combinée. Nécessite Claude Code v2.1.242 ou ultérieur.

Définissez `"merge"` uniquement où chaque source [classée](/docs/fr/managed-settings#how-claude-code-combines-managed-sources) en dessous de votre plus haute est sous le contrôle d'un administrateur, car Claude Code ajoute alors des entrées d'une source inférieure, telle que les règles `permissions.allow`, à la politique.

* **Portée** : [`Managed`](#scopes). Claude Code lit cette clé de la source de plus haute priorité qui porte soit cette clé soit une clé de politique, et ignore cette clé dans chaque source classée plus bas, de sorte qu'une source inférieure ne peut pas se choisir pour se combiner avec la source au-dessus. Ni le registre HKCU Windows ni les [paramètres parents d'un hôte d'intégration](/docs/fr/managed-settings#let-an-embedding-host-add-policy) ne participent à la fusion.
* **Type** : chaîne, l'une des :
  * `"first-wins"` : la source de plus haute priorité qui porte une clé de politique fournit la politique, et les sources inférieures ne contribuent que les [clés que Claude Code lit de chaque source d'administrateur](/docs/fr/managed-settings#keys-read-from-every-admin-source)
  * `"merge"` : chaque source d'administrateur que vous livrez contribue ses clés, combinées par les règles ci-dessous
* **Défaut** : `"first-wins"`

Livrez la clé dans la source de plus haute priorité que vous déployez. Une machine qui ne reçoit jamais les paramètres gérés par le serveur a besoin de la clé dans son profil MDM aussi, car Claude Code lit la clé de la source de plus haute priorité qui la porte ou une clé de politique. Un fichier `managed-settings.json` est la source d'administrateur la plus basse classée, donc `"merge"` défini là n'a pas de source en dessous pour se combiner avec. Dans les paramètres gérés par le serveur, la clé ressemble à ceci :

```json theme={null}
{
  "managedSourcesBehavior": "merge"
}
```

Sous `"merge"`, Claude Code combine chaque clé par son type. Ce tableau donne la règle pour chaque type. Les lignes de liste d'autorisation de restriction, valeurs prises en entier et plus haute source uniquement nomment chaque clé qu'elles couvrent, et les autres lignes donnent des exemples :

| Type de clé                                         | Comment Claude Code la combine                                                                                                                                                                                                      | Clés                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :-------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Listes                                              | Combine les entrées de chaque source                                                                                                                                                                                                | [`permissions.allow`](#permissions-allow), [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) et autres clés de liste                                                                                                                                                                                                                                                                                                                                                                                            |
| Verrous                                             | Applique la valeur la plus stricte que n'importe quelle source définit. Lorsqu'aucune source ne définit une valeur stricte, applique une valeur plus souple uniquement de la source la plus haute                                   | [`allowManagedPermissionRulesOnly`](#allowmanagedpermissionrulesonly), [`permissions.disableBypassPermissionsMode`](#permissions-disablebypasspermissionsmode) et autres verrous booléens ou énumérés                                                                                                                                                                                                                                                                                                                             |
| Listes d'autorisation de restriction                | Prend la liste en entier de la source la plus haute qui la définit, sans ajouter d'entrées de sources inférieures. Lorsque la source la plus haute ne la définit pas, la prend en entier de la source suivante en bas               | [`availableModels`](#availablemodels), [`allowedMcpServers`](#allowedmcpservers), [`strictKnownMarketplaces`](#strictknownmarketplaces), [`allowedChannelPlugins`](#allowedchannelplugins) et la chaîne [`fallbackModel`](#fallbackmodel)                                                                                                                                                                                                                                                                                         |
| Valeurs prises en entier                            | Prend la valeur en entier de la source la plus haute qui la définit, sans combiner les entrées ou champs de sources inférieures. Lorsque la source la plus haute ne la définit pas, la prend en entier de la source suivante en bas | [`sandbox.credentials.awsPairs`](#sandbox-credentials-awspairs), [`sandbox.ripgrep`](#sandbox-ripgrep)                                                                                                                                                                                                                                                                                                                                                                                                                            |
| Serveurs MCP fournis                                | Combine les noms de serveur de chaque source. Lorsque deux sources définissent le même nom, applique l'entrée entière de la source la plus haute                                                                                    | [`managedMcpServers`](#managedmcpservers)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| Lire uniquement de la source de plus haute priorité | Lit la clé uniquement de la source de plus haute priorité qui porte une clé de politique, de sorte que la valeur d'une source inférieure est ignorée même lorsque la source la plus haute n'en définit aucune                       | [`apiKeyHelper`](#apikeyhelper), [`awsAuthRefresh`](#awsauthrefresh), [`awsCredentialExport`](#awscredentialexport), [`gcpAuthRefresh`](#gcpauthrefresh), [`otelHeadersHelper`](#otelheadershelper), `proxyAuthHelper`, [`forceLoginOrgUUID`](#forceloginorguuid), les valeurs `"claudeai"` et `"console"` de [`forceLoginMethod`](#forceloginmethod), [`parentSettingsBehavior`](#parentsettingsbehavior), [`modelPicker`](#modelpicker), [`policyHelper`](#policyhelper), [`permissions.defaultMode`](#permissions-defaultmode) |
| `env`                                               | [Fusionne par variable sur les sources d'administrateur](/docs/fr/managed-settings#keys-read-from-every-admin-source), sous `"first-wins"` et `"merge"`                                                                                  | [`env`](#env)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| Chaque autre clé                                    | Prend la valeur de la source la plus haute qui la définit                                                                                                                                                                           | [`cleanupPeriodDays`](#cleanupperioddays), [`model`](#model)                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |

Prendre `sandbox.credentials.awsPairs` et `sandbox.ripgrep` en entier nécessite Claude Code v2.1.257 ou ultérieur.

Quelques clés ajoutent une condition que le tableau ne montre pas :

* **[`policyHelper`](#policyhelper)** : Claude Code ne l'honore que lorsque la source la plus haute qui porte une clé de politique est une politique MDM ou un fichier de paramètres gérés, donc sous les paramètres gérés par le serveur elle ne s'applique pas.
* **[`modelOverrides`](#modeloverrides)** : s'associe avec `availableModels`. Claude Code prend `modelOverrides` de la source la plus haute qui la définit, à moins qu'une source plus haute ne définisse `availableModels` sans `modelOverrides`. Dans ce cas, il ignore `modelOverrides` de chaque source.
* **[`forceLoginGatewayUrl`](#forcelogingatewayurl), [`gatewayInternalNetworks`](#gatewayinternalnetworks) et la valeur `"gateway"` de [`forceLoginMethod`](#forceloginmethod)** : Claude Code ne les lit jamais à partir des paramètres gérés par le serveur, de sorte qu'une valeur là-bas ne s'applique ni ne cache celle définie dans une politique MDM ou un fichier de paramètres gérés. Parmi les sources d'administrateur sur la machine, seule la source la plus haute classée qui porte une clé de politique les fournit, que les paramètres gérés par le serveur soient également présents ou non.

Pour confirmer quelles sources se sont combinées sur une machine, exécutez `/status` et [lisez la ligne `Setting sources`](/docs/fr/managed-settings#read-the-source-in-/status).

<h3 id="parentsettingsbehavior">
  `parentSettingsBehavior`
</h3>

Choisissez si Claude Code applique les paramètres gérés fournis par un processus hôte d'intégration, tel que le SDK Agent ou une extension IDE, lorsqu'une couche gérée déployée par l'administrateur est également présente. Avec `"first-wins"`, Claude Code supprime les paramètres fournis par l'hôte ; avec `"merge"`, il les applique sous la couche d'administrateur via un filtre restrictif uniquement. Définissez `"merge"` lorsqu'un hôte doit transmettre ses propres restrictions aux sessions qu'il lance, par exemple Claude Desktop livrant la liste d'autorisation de sortie d'une passerelle.

* **Portée** : [`Managed`](#scopes). Claude Code le lit de la source gérée contrôlée par l'administrateur de plus haute priorité.
* **Type** : chaîne, l'une des :
  * `"first-wins"` : Claude Code supprime les paramètres fournis par l'hôte lorsqu'une couche gérée déployée par l'administrateur est présente
  * `"merge"` : Claude Code applique les paramètres fournis par l'hôte sous la couche d'administrateur via un filtre restrictif uniquement
* **Défaut** : `"first-wins"`

```json managed-settings.json theme={null}
{
  "parentSettingsBehavior": "merge"
}
```

Cette clé n'a aucun effet lorsqu'aucune couche gérée déployée par l'administrateur n'existe : les paramètres de l'hôte s'appliquent alors comme la seule couche gérée, toujours filtrés aux valeurs restrictives. Pour les limites du filtre et comment les sources gérées interagissent, voir [Paramètres parents des hôtes d'intégration](/docs/fr/managed-settings#parent-settings-from-embedding-hosts) et [Restreindre les paramètres parents](/docs/fr/claude-apps-gateway#restrict-parent-settings).

<span id="compute-managed-settings-with-a-policy-helper" />

<h3 id="policyhelper">
  `policyHelper`
</h3>

Exécutez un exécutable que vous déployez qui calcule les paramètres gérés au démarrage, de sorte que vous pouvez dériver la politique de la posture de l'appareil, de l'identité ou d'un service distant au lieu d'un fichier statique. Claude Code exécute l'assistant avant d'accepter la première invite et traite les paramètres qu'il émet comme les paramètres gérés pour la session.

* **Portée** : [`Managed`](#scopes). Lire à partir de la plist macOS, du registre Windows HKLM ou du fichier de paramètres gérés. Claude Code lit la clé de la source gérée de plus haute priorité qui porte une [clé de politique](/docs/fr/managed-settings#how-claude-code-combines-managed-sources) et exécute l'assistant uniquement lorsque cette source est l'une de ces trois ; il ignore la clé dans les paramètres gérés par le serveur, le registre HKCU et les paramètres parents fournis par l'hôte.
* **Type** : objet avec `path`, `timeoutMs` et `refreshIntervalMs`
* **Défaut** : non défini, donc aucun assistant ne s'exécute

Lorsque les paramètres gérés par le serveur livrent la politique au lancement, ils prennent la priorité sur la source de l'assistant et l'assistant ne s'exécute pas.

Si une récupération de paramètres ultérieure signale que les paramètres gérés par le serveur ont été supprimés, Claude Code exécute l'assistant à ce moment plutôt que d'attendre le prochain lancement. Sa sortie gouverne le reste de la session, et une exécution qui échoue termine la session avec le même message qu'une [exécution de démarrage échouée](#helper-failures).

Cet exemple exécute l'assistant avec un délai d'expiration de 5 secondes et le réexécute toutes les cinq minutes :

```json managed-settings.json theme={null}
{
  "policyHelper": {
    "path": "/usr/local/bin/claude-policy",
    "timeoutMs": 5000,
    "refreshIntervalMs": 300000
  }
}
```

<h4 id="write-the-helper-output">
  Écrire la sortie de l'assistant
</h4>

Claude Code exécute l'assistant sans arguments, définit `CLAUDE_CODE_VERSION` dans son environnement et lit une enveloppe JSON à partir de stdout, limitée à 1 MiB.

Mettez les paramètres sous une clé `managedSettings`. Un objet de paramètres nu sans clé `managedSettings` analyse avec `managedSettings` non défini et n'applique rien, et Claude Code ne signale aucune erreur :

```json theme={null}
{
  "managedSettings": {
    "permissions": { "deny": ["Read(//etc/secrets/**)"] }
  }
}
```

Lorsque l'assistant émet `managedSettings`, cet objet devient la seule source de paramètres gérés pour l'exécution : Claude Code ignore les sources MDM, fichier et HKCU, lit les [clés inter-sources](/docs/fr/managed-settings#keys-read-from-every-admin-source) uniquement à partir de la sortie de l'assistant, et ne fusionne jamais les [paramètres parents](/docs/fr/managed-settings#parent-settings-from-embedding-hosts).

La vérification `forceRemoteSettingsRefresh` au démarrage s'exécute avant l'assistant et lit n'importe quelle source d'administrateur. Un assistant qui se termine avec `0` avec une enveloppe qui omet `managedSettings` ne contribue aucun paramètre gérés, et les autres sources s'appliquent comme d'habitude.

<h4 id="helper-failures">
  Défaillances de l'assistant
</h4>

Une exécution d'assistant échoue lorsque :

* `path` viole les règles dans [`policyHelper.path`](#policyhelper-path).
* Aucun fichier régulier n'existe à `path`. Claude Code vérifie le fichier avant de démarrer l'assistant, dans le même budget `timeoutMs`, de sorte qu'un montage réseau qui ne répond pas peut causer l'échec de l'exécution.
* L'assistant se termine non-zéro, s'exécute toujours lorsque `timeoutMs` s'écoule, ou ne démarre pas du tout, par exemple parce qu'il n'est pas exécutable.
* L'assistant écrit plus de 1 MiB sur stdout ou stderr.
* stdout n'est pas un seul objet JSON, ou son `managedSettings` a une [violation de schéma que Claude Code ne peut pas réparer](/docs/fr/managed-settings#find-entries-claude-code-dropped).

Lorsque l'exécution au démarrage échoue, Claude Code imprime la raison et refuse de démarrer. Après une sortie non-zéro, la raison inclut stderr de l'assistant, ou sa sortie stdout lorsque stderr est vide. Après un délai d'expiration, la raison nomme la limite `timeoutMs` et n'inclut aucune sortie de l'assistant. Le refus couvre les sessions interactives, `claude -p`, les sessions du SDK Agent, les [sessions en arrière-plan](/docs/fr/agent-view) et la plupart des sous-commandes.

Le refus est délibéré, de sorte qu'un assistant qui a besoin de résilience aux pannes doit servir à partir de son propre cache et se terminer avec `0`.

Lorsqu'une actualisation en arrière-plan échoue, Claude Code maintient la dernière politique réussie en vigueur, et `/status` affiche l'actualisation échouée avec sa raison jusqu'à ce qu'une actualisation réussisse. Chaque actualisation s'exécute sous les mêmes `timeoutMs` et règles d'échec que l'exécution au démarrage.

Avec `--debug`, Claude Code écrit stderr de l'assistant de chaque exécution dans le [journal de débogage](/docs/fr/debug-your-config).

Claude Code signale une valeur `policyHelper` invalide comme une [entrée supprimée](/docs/fr/managed-settings#find-entries-claude-code-dropped) et démarre la session sur les paramètres gérés restants sans exécuter d'assistant. Les valeurs invalides incluent une chaîne de chemin nu et un `timeoutMs` en dessous de [son minimum](#policyhelper-timeoutms).

Pour désactiver un assistant, supprimez la clé de la source qui la définit.

<h3 id="policyhelper-path">
  `policyHelper.path`
</h3>

Nommez l'exécutable d'assistant que Claude Code exécute. Pour ce qui se passe lorsque le chemin viole les règles ci-dessous, voir [Défaillances de l'assistant](#helper-failures).

* **Portée** : [`Managed`](#scopes). Lire à partir de la plist macOS, du registre Windows HKLM ou du fichier de paramètres gérés, où [`policyHelper`](#policyhelper) est lu.
* **Type** : chaîne, un chemin absolu sous forme normalisée, sans segments `.` ou `..` ; sur Windows, un chemin de lettre de lecteur ou UNC qui se termine par `.exe`
* **Défaut** : aucun ; requis lorsque `policyHelper` est défini

```json managed-settings.json theme={null}
{
  "policyHelper": {
    "path": "/usr/local/bin/claude-policy"
  }
}
```

<h3 id="policyhelper-timeoutms">
  `policyHelper.timeoutMs`
</h3>

Définissez combien de temps Claude Code attend l'assistant avant de traiter l'exécution comme échouée. Une exécution expirée échoue de la même manière qu'une sortie non-zéro, de sorte qu'au démarrage Claude Code refuse de démarrer.

* **Portée** : [`Managed`](#scopes). Lire à partir de la plist macOS, du registre Windows HKLM ou du fichier de paramètres gérés, où [`policyHelper`](#policyhelper) est lu.
* **Type** : entier, millisecondes, minimum `1000`
* **Défaut** : `10000`

```json managed-settings.json theme={null}
{
  "policyHelper": {
    "path": "/usr/local/bin/claude-policy",
    "timeoutMs": 5000
  }
}
```

<h3 id="policyhelper-refreshintervalms">
  `policyHelper.refreshIntervalMs`
</h3>

Faites en sorte que Claude Code réexécute l'assistant en arrière-plan à un intervalle de sorte que les modifications de politique atteignent une session en cours d'exécution. Lorsqu'une actualisation réussit, sa sortie remplace les paramètres gérés précédents sans redémarrage ; lorsqu'une actualisation échoue, Claude Code maintient la politique qu'il a déjà.

* **Portée** : [`Managed`](#scopes). Lire à partir de la plist macOS, du registre Windows HKLM ou du fichier de paramètres gérés, où [`policyHelper`](#policyhelper) est lu.
* **Type** : entier, millisecondes : `0` pour désactiver l'actualisation, sinon au moins `60000`
* **Défaut** : non défini, de sorte que Claude Code exécute l'assistant une fois au démarrage

Cet exemple réexécute l'assistant toutes les cinq minutes :

```json managed-settings.json theme={null}
{
  "policyHelper": {
    "path": "/usr/local/bin/claude-policy",
    "refreshIntervalMs": 300000
  }
}
```

<h3 id="wslinheritswindowssettings">
  `wslInheritsWindowsSettings`
</h3>

Faites en sorte que Claude Code sur WSL lise les paramètres gérés de la chaîne de politique Windows, avec HKLM et le fichier de paramètres gérés Windows prenant la priorité sur `/etc/claude-code` et HKCU en dessous. Pendant que la chaîne est activée, Claude Code lit `/etc/claude-code` uniquement lorsqu'aucun fichier de paramètres gérés ou drop-in sous `C:\Program Files\ClaudeCode\` ne livre une [clé de politique](/docs/fr/managed-settings#how-claude-code-combines-managed-sources). Définissez-le pour étendre la politique que vous déployez déjà sur Windows aux sessions WSL sur la même machine, de sorte qu'elles suivent les mêmes règles que les sessions hôte. Claude Code ne l'honore que lorsqu'il est défini dans la clé de registre HKLM ou dans un fichier de paramètres gérés ou drop-in sous `C:\Program Files\ClaudeCode\`, tous deux nécessitant l'administrateur Windows pour écrire.

* **Portée** : [`Managed`](#scopes). Dans une source Windows contrôlée par l'administrateur.
* **Type** : Booléen
  * `true` : Claude Code sur WSL lit les paramètres gérés de la chaîne de politique Windows, et lit `/etc/claude-code` uniquement lorsqu'aucun fichier de paramètres gérés ou drop-in sous `C:\Program Files\ClaudeCode\` ne livre une [clé de politique](/docs/fr/managed-settings#how-claude-code-combines-managed-sources)
  * `false` : WSL lit uniquement `/etc/claude-code`
* **Défaut** : `false`, de sorte que WSL lit uniquement `/etc/claude-code`

```json managed-settings.json theme={null}
{
  "wslInheritsWindowsSettings": true
}
```

Une fois qu'une source d'administrateur active la chaîne, la politique HKCU la rejoint sur WSL uniquement lorsque HKCU définit également la clé à `true`. Cette copie n'active pas la chaîne par elle-même. Une source Windows qui contient uniquement cette clé ne compte pas comme une source de politique, de sorte qu'une source de priorité inférieure fournit toujours la politique. Cette clé n'a aucun effet sur Windows natif.

<h2 id="global-config-settings">
  Paramètres de configuration globale
</h2>

Enregistrez ces clés dans `~/.claude.json`, pas dans un fichier de paramètres. Claude Code les ignore partout ailleurs. Claude Code et `/config` en écrivent la plupart pour vous, et vous pouvez aussi les modifier manuellement.

<h3 id="autoconnectide">
  `autoConnectIde`
</h3>

Connectez-vous à un IDE en cours d'exécution automatiquement lorsque vous démarrez Claude Code à partir d'un terminal externe. Apparaît dans `/config` sous **Auto-connect to IDE (external terminal)** lorsque vous exécutez Claude Code en dehors d'un terminal VS Code ou JetBrains.

* **Scope** : [`Global config`](#scopes)
* **Type** : Boolean
  * `true` : Claude Code se connecte à un IDE en cours d'exécution automatiquement lorsque vous le démarrez à partir d'un terminal externe
  * `false` : Claude Code ne se connecte pas automatiquement à partir d'un terminal externe ; à l'intérieur d'un terminal VS Code ou JetBrains, ou avec `--ide`, il se connecte toujours
* **Default** : `false`
* **Per-session overrides** : [`CLAUDE_CODE_AUTO_CONNECT_IDE`](/docs/fr/env-vars) prend la priorité sur cette clé pour une session, dans les deux sens

```json ~/.claude.json theme={null}
{
  "autoConnectIde": true
}
```

Claude Code ignore cette clé dans `settings.json`.

<h3 id="autoinstallideextension">
  `autoInstallIdeExtension`
</h3>

Installez l'extension Claude Code IDE automatiquement lorsque vous exécutez Claude Code à partir d'un terminal VS Code. Apparaît dans `/config` sous **Auto-install IDE extension** lorsque vous exécutez Claude Code à l'intérieur d'un terminal VS Code ou JetBrains.

* **Scope** : [`Global config`](#scopes)
* **Type** : Boolean
  * `true` : Claude Code installe l'extension IDE automatiquement lorsque vous l'exécutez à partir d'un terminal VS Code
  * `false` : Claude Code n'installe pas l'extension automatiquement
* **Default** : `true`
* **Per-session overrides** : [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](/docs/fr/env-vars) défini à `1` ignore l'installation pour une session même lorsque cette clé est `true`

```json ~/.claude.json theme={null}
{
  "autoInstallIdeExtension": false
}
```

Claude Code ignore cette clé dans `settings.json`.

<h3 id="copyonselect">
  `copyOnSelect`
</h3>

Copiez le texte dans votre presse-papiers automatiquement lorsque vous terminez de le sélectionner avec la souris dans [fullscreen rendering](/docs/fr/fullscreen#use-the-mouse) ou [agent view](/docs/fr/agent-view). Apparaît dans `/config` sous **Copy on select** pendant que le rendu en plein écran est activé.

* **Scope** : [`Global config`](#scopes)
* **Type** : Boolean
  * `true` : Claude Code copie le texte dans votre presse-papiers lorsque vous terminez de le sélectionner
  * `false` : la sélection de texte laisse votre presse-papiers inchangé, et vous [copiez la sélection avec un raccourci clavier](/docs/fr/fullscreen#use-the-mouse) à la place
* **Default** : `true`

```json ~/.claude.json theme={null}
{
  "copyOnSelect": false
}
```

Claude Code ignore cette clé dans `settings.json`.

<h3 id="difftool">
  `diffTool`
</h3>

Choisissez où Claude Code affiche le diff d'une modification `Edit` ou `Write` qu'il propose lorsqu'un IDE [VS Code](/docs/fr/vs-code) ou [JetBrains](/docs/fr/jetbrains#features) est connecté : `"auto"` l'ouvre dans la visionneuse de diff de l'IDE, `"terminal"` le garde dans le terminal. Apparaît dans `/config` sous **Diff tool** uniquement lorsque Claude Code est connecté à un IDE VS Code ou JetBrains.

* **Scope** : [`Global config`](#scopes)
* **Type** : string, l'un de :
  * `"auto"` : Claude Code ouvre le diff dans la visionneuse de diff de l'IDE lorsqu'un IDE VS Code ou JetBrains est connecté
  * `"terminal"` : Claude Code garde le diff dans le terminal
* **Default** : `"auto"`

```json ~/.claude.json theme={null}
{
  "diffTool": "terminal"
}
```

Claude Code ignore cette clé dans `settings.json`.

<h3 id="externaleditorcontext">
  `externalEditorContext`
</h3>

Lorsque vous appuyez sur `Ctrl+G`, Claude Code ouvre l'invite que vous tapez dans votre [external editor](/docs/fr/interactive-mode#general-controls). Avec cette clé activée, le buffer de l'éditeur commence par la réponse précédente de Claude sous forme de lignes de commentaire `#`, afin que vous puissiez la lire pendant que vous écrivez, et Claude Code supprime ces lignes lorsque vous enregistrez. Apparaît dans `/config` sous **Show last response in external editor**.

* **Scope** : [`Global config`](#scopes)
* **Type** : Boolean
  * `true` : le buffer de l'éditeur commence par la réponse précédente de Claude sous forme de lignes de commentaire `#`, que Claude Code supprime lorsque vous enregistrez
  * `false` : le buffer de l'éditeur s'ouvre avec uniquement votre invite
* **Default** : `false`

```json ~/.claude.json theme={null}
{
  "externalEditorContext": true
}
```

Avec cette option activée, le buffer que Claude Code ouvre ressemble à ceci, et seul le texte sous la ligne de marqueur est envoyé comme votre invite :

```text theme={null}
# ─── Claude's last response (for reference; removed on save) ───
# I added the retry loop to fetchUser in src/api.ts and a test
# for the timeout case. Want me to wire the same retry into
# fetchOrders?
# ─── Write your reply below this line ──────────────────────────

Yes, and cap it at three attempts.
```

Claude Code conserve les 50 dernières lignes de la réponse et marque la coupure avec `# … (earlier output truncated)`.

Claude Code ignore cette clé dans `settings.json`.

<h3 id="permissionexplainerenabled">
  `permissionExplainerEnabled`
</h3>

<Warning>
  Supprimé dans v2.1.257, ainsi que l'explication de la commande `Ctrl+E` sur les invites de permission Bash et PowerShell. Le définir n'a aucun effet sur les versions actuelles.
</Warning>

Jusqu'à v2.1.256, vous pouviez appuyer sur `Ctrl+E` sur une invite de permission Bash ou PowerShell pour voir une explication générée par le modèle de la commande, et définir cette clé à `false` pour désactiver ce raccourci.

* **Scope** : [`Global config`](#scopes). Sur v2.1.256 et antérieures.
* **Type** : Boolean
* **Default** : `true`

<h3 id="teammatedefaultmodel">
  `teammateDefaultModel`
</h3>

<Warning>
  Supprimé dans v2.1.234, ainsi que sa ligne `/config` **Default teammate model**. Le définir n'a aucun effet sur les versions actuelles.
</Warning>

Jusqu'à v2.1.233, vous définissiez cette clé sur le modèle pour les coéquipiers de [agent team](/docs/fr/agent-teams#specify-teammates-and-models) que votre invite n'a pas nommé un modèle pour : un alias tel que `"sonnet"`, ou `null` pour suivre le modèle du leader. Pour le modèle que Claude Code choisit maintenant pour ces coéquipiers, voir [specify teammates and models](/docs/fr/agent-teams#specify-teammates-and-models).

* **Scope** : [`Global config`](#scopes). Sur v2.1.233 et antérieures.
* **Type** : string, un alias de modèle ou un ID de modèle complet, ou `null`
* **Default** : unset

<h2 id="see-also">
  Voir aussi
</h2>

* [Configurer les autorisations](/docs/fr/permissions) : syntaxe des règles, modes d'autorisation et confiance de l'espace de travail
* [Variables d'environnement](/docs/fr/env-vars) : chaque variable `CLAUDE_*`, `ANTHROPIC_*` et variable de fournisseur que Claude Code lit
* [Outils disponibles pour Claude](/docs/fr/tools-reference) : les outils intégrés et ceux qui nécessitent une approbation
* [Exemples de fichiers de paramètres](/docs/fr/settings-example) : un fichier personnel, un fichier d'équipe et un fichier géré par une organisation
* [Configurer les paramètres gérés](/docs/fr/admin-setup) : comment les organisations décident ce qu'elles doivent appliquer
* [Déployer les paramètres gérés](/docs/fr/managed-settings) : mécanismes de livraison, précédence au sein du niveau géré et entrées invalides dans les paramètres gérés
* [Déboguer votre configuration](/docs/fr/debug-your-config) : `claude doctor` et la boîte de dialogue Erreur de paramètres
