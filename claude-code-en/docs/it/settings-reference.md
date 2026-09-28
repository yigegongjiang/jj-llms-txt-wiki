> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Tutte le impostazioni

> Riferimento completo per ogni chiave settings.json di Claude Code: dove va ciascuna, il suo tipo e valore predefinito, e un esempio pronto da incollare, con un indice di ogni chiave.

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

<BackToIndex href="#all-settings" label="Torna all'indice" />

Questa pagina di riferimento elenca ogni chiave che Claude Code legge da un file di impostazioni, più il [breve gruppo di chiavi](#global-config-settings) che mantiene in `~/.claude.json` invece. Per scegliere un file, o controllare la precedenza, inizia con [File di impostazioni e precedenza](/docs/it/settings).

<span id="available-settings" />

<span id="scopes" />

<span id="all-settings" />

<h2 id="settings-index">
  Indice delle impostazioni
</h2>

Ogni chiave sottostante è collegata alla sua voce. L'ambito elenca i [file](/docs/it/settings#settings-files-and-who-they-affect) in cui può trovarsi: `User` è `~/.claude/settings.json`, `Project` è `.claude/settings.json`, `Local` è `.claude/settings.local.json`, e `Managed` è [quello che la vostra organizzazione distribuisce](/docs/it/managed-settings). `Any file` significa tutti e quattro, e `Global config` significa [`~/.claude.json`](#global-config-settings).

<ReferenceFilter
  noun="settings"
  placeholder="Filter settings by key or purpose"
  facetOrder={{ scope: ["Any file", "User, local, or managed", "User or managed", "Managed", "Global config"] }}
  columnHelp={{
topic: "The section of this page that holds the entry. Use Sort by to group the table by topic.",
scope: "Which settings files can set the key: user (~/.claude/settings.json), project (.claude/settings.json), local (.claude/settings.local.json), or managed (deployed by your organization). Global config keys are in ~/.claude.json instead.",
}}
/>

| Key                                                                                                   | Description                                                                                                                                                                                                                                                                 | Topic                              | Scope                   |
| :---------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------- | :---------------------- |
| [`advisorModel`](#advisormodel)                                                                       | Scegliete quale modello risponde quando Claude chiede lo [strumento advisor](/docs/it/advisor)                                                                                                                                                                                   | Model and responses                | Any file                |
| [`agent`](#agent)                                                                                     | Iniziate ogni sessione come un [subagent](/docs/it/sub-agents) denominato con il suo prompt, strumenti e modello                                                                                                                                                                 | Agents, sessions, and worktrees    | Any file                |
| [`agentPushNotifEnabled`](#agentpushnotifenabled)                                                     | Lasciate che Claude invii una [notifica push al vostro telefono](/docs/it/remote-control#mobile-push-notifications) quando decide di farlo                                                                                                                                       | Remote, desktop, and notifications | Any file                |
| [`allowAllClaudeAiMcps`](#allowallclaudeaimcps)                                                       | Caricate i [connettori claude.ai](/docs/it/mcp) che Claude Code recupera da solo insieme a un [`managed-mcp.json`](/docs/it/managed-mcp#exclusive-control-with-managed-mcp-json) distribuito                                                                                          | MCP                                | Managed                 |
| [`allowedChannelPlugins`](#allowedchannelplugins)                                                     | Sostituite l'elenco di autorizzazione predefinito dei [plugin di canale](/docs/it/channels#restrict-which-channel-plugins-can-run) che possono inviare messaggi                                                                                                                  | Plugins and skills                 | Managed                 |
| [`allowedHttpHookUrls`](#allowedhttphookurls)                                                         | Limitate gli URL che gli [hook HTTP](/docs/it/hooks) possono raggiungere                                                                                                                                                                                                         | Hooks and automation               | Any file                |
| [`allowedMcpServers`](#allowedmcpservers)                                                             | Elenco di autorizzazione per i [server MCP](/docs/it/mcp) che gli utenti possono aggiungere                                                                                                                                                                                      | MCP                                | Any file                |
| [`allowManagedHooksOnly`](#allowmanagedhooksonly)                                                     | Eseguite solo gli [hook](/docs/it/hooks) che la vostra organizzazione distribuisce                                                                                                                                                                                               | Hooks and automation               | Managed                 |
| [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly)                                           | Rendete l'elenco di autorizzazione [MCP](/docs/it/mcp) gestito l'unico che si applica                                                                                                                                                                                            | MCP                                | Managed                 |
| [`allowManagedPermissionRulesOnly`](#allowmanagedpermissionrulesonly)                                 | Rendete le [impostazioni gestite](/docs/it/managed-settings) l'unica fonte di impostazioni delle [regole di autorizzazione](/docs/it/permissions#managed-settings)                                                                                                                    | Permission settings                | Managed                 |
| [`alwaysThinkingEnabled`](#alwaysthinkingenabled)                                                     | Disattivate il [pensiero esteso](/docs/it/model-config#extended-thinking) per ogni sessione                                                                                                                                                                                      | Model and responses                | Any file                |
| [`apiKeyHelper`](#apikeyhelper)                                                                       | Generare le [credenziali API](/docs/it/authentication#credential-management) con il vostro comando                                                                                                                                                                               | Authentication and providers       | Any file                |
| [`askUserQuestionTimeout`](#askuserquestiontimeout)                                                   | Lasciate che una domanda senza risposta [continui automaticamente](/docs/it/tools-reference#question-auto-continue-timeout) dopo il tempo di inattività                                                                                                                          | Interface and terminal             | User or managed         |
| [`attribution`](#attribution)                                                                         | Personalizzate l'attribuzione che Claude Code aggiunge ai commit e alle pull request                                                                                                                                                                                        | Git and attribution                | Any file                |
| [`attribution.commit`](#attribution-commit)                                                           | Cambiate o nascondeteil trailer che Claude Code aggiunge ai commit                                                                                                                                                                                                          | Git and attribution                | Any file                |
| [`attribution.pr`](#attribution-pr)                                                                   | Cambiate o nascondetela riga di attribuzione nelle descrizioni delle pull request                                                                                                                                                                                           | Git and attribution                | Any file                |
| [`attribution.sessionUrl`](#attribution-sessionurl)                                                   | Omettete il collegamento della sessione claude.ai dai commit di [cloud](/docs/it/claude-code-on-the-web) e [Remote Control](/docs/it/remote-control)                                                                                                                                  | Git and attribution                | Any file                |
| [`autoCompactEnabled`](#autocompactenabled)                                                           | Disattivate o attivate la [compattazione automatica](/docs/it/context-window)                                                                                                                                                                                                    | Memory and context                 | Any file                |
| [`autoCompactWindow`](#autocompactwindow)                                                             | Impostate quanto pieno diventa il contesto prima che Claude Code [compatti](/docs/it/context-window)                                                                                                                                                                             | Memory and context                 | Any file                |
| [`autoConnectIde`](#autoconnectide)                                                                   | Connettetevi automaticamente a un IDE [VS Code](/docs/it/vs-code) o [JetBrains](/docs/it/jetbrains#from-external-terminals) in esecuzione da un terminale esterno                                                                                                                     | Global config settings             | Global config           |
| [`autoContinueAtUsageLimit`](#autocontinueatusagelimit)                                               | Aspettate nella sessione aperta e [continuate il compito automaticamente](/docs/it/interactive-mode#wait-for-a-usage-limit-to-reset) dopo il ripristino di un limite di utilizzo di claude.ai                                                                                    | Interface and terminal             | User or managed         |
| [`autoInstallIdeExtension`](#autoinstallideextension)                                                 | Disattivate l'installazione automatica dell'[estensione IDE](/docs/it/vs-code#install-the-extension) da un terminale VS Code                                                                                                                                                     | Global config settings             | Global config           |
| [`autoMemoryDirectory`](#automemorydirectory)                                                         | Archiviate la [memoria automatica](/docs/it/memory#auto-memory) in una directory che scegliete                                                                                                                                                                                   | Memory and context                 | Any file                |
| [`autoMemoryEnabled`](#automemoryenabled)                                                             | Disattivate o attivate la [memoria automatica](/docs/it/memory#auto-memory)                                                                                                                                                                                                      | Memory and context                 | Any file                |
| [`autoMode`](#automode)                                                                               | Aggiungete le vostre regole di autorizzazione e negazione al classificatore della [modalità automatica](/docs/it/permission-modes#eliminate-prompts-with-auto-mode)                                                                                                              | Permission settings                | User or managed         |
| [`autoMode.classifyAllShell`](#automode-classifyallshell)                                             | Inviate ogni comando shell attraverso il [classificatore della modalità automatica](/docs/it/permission-modes#what-the-classifier-blocks-by-default), anche quelli che corrispondono a una regola di autorizzazione ristretta                                                    | Permission settings                | User or managed         |
| [`autoScrollEnabled`](#autoscrollenabled)                                                             | [Seguite il nuovo output](/docs/it/fullscreen#auto-follow) fino in fondo nel rendering a schermo intero                                                                                                                                                                          | Interface and terminal             | Any file                |
| [`autoUpdatesChannel`](#autoupdateschannel)                                                           | Seguite il [canale di rilascio](/docs/it/setup#configure-release-channel) stabile invece dell'ultimo                                                                                                                                                                             | Updates and versioning             | Any file                |
| [`availableModels`](#availablemodels)                                                                 | [Limitate quali modelli](/docs/it/model-config#restrict-model-selection) le persone possono scegliere                                                                                                                                                                            | Model and responses                | Any file                |
| [`awaySummaryEnabled`](#awaysummaryenabled)                                                           | Disattivate il [riepilogo della sessione](/docs/it/interactive-mode#session-recap) mostrato quando tornate al terminale                                                                                                                                                          | Remote, desktop, and notifications | Any file                |
| [`awsAuthRefresh`](#awsauthrefresh)                                                                   | Aggiornate le [credenziali Bedrock](/docs/it/amazon-bedrock#advanced-credential-configuration) scadute in `.aws` con il vostro comando                                                                                                                                           | Authentication and providers       | Any file                |
| [`awsCredentialExport`](#awscredentialexport)                                                         | Fornite le [credenziali Bedrock](/docs/it/amazon-bedrock#advanced-credential-configuration) come JSON dal vostro comando                                                                                                                                                         | Authentication and providers       | Any file                |
| [`axScreenReader`](#axscreenreader)                                                                   | Rendete l'[output adatto ai lettori di schermo](/docs/it/accessibility)                                                                                                                                                                                                          | Interface and terminal             | Any file                |
| [`bashEditDiffEnabled`](#basheditdiffenabled)                                                         | Registrate i [file che un comando Bash ha modificato](/docs/it/hooks#bash) in ogni modalità di autorizzazione                                                                                                                                                                    | Interface and terminal             | User or managed         |
| [`bashOutputMaxChars`](#bashoutputmaxchars)                                                           | Impostate quanto dell'[output](/docs/it/tools-reference#output-limits) di un comando riuscito Claude riceve inline                                                                                                                                                               | Memory and context                 | Any file                |
| [`blockedMarketplaces`](#blockedmarketplaces)                                                         | Bloccate le fonti del [marketplace di plugin](/docs/it/plugins/overview) per la vostra organizzazione                                                                                                                                                                            | Plugins and skills                 | Managed                 |
| [`browserExternalPageTools`](#browserexternalpagetools)                                               | Mantenete gli strumenti di Claude disattivati sulle pagine esterne nel riquadro [desktop](/docs/it/desktop) Browser                                                                                                                                                              | Tools                              | Managed                 |
| [`channelsEnabled`](#channelsenabled)                                                                 | Consentite i [canali](/docs/it/channels#enable-channels-for-your-organization) per la vostra organizzazione                                                                                                                                                                      | Plugins and skills                 | Managed                 |
| [`claudeMd`](#claudemd)                                                                               | Iniettate le istruzioni [CLAUDE.md](/docs/it/memory#deploy-organization-wide-claude-md) a livello di organizzazione dalle impostazioni gestite                                                                                                                                   | Memory and context                 | Managed                 |
| [`claudeMdExcludes`](#claudemdexcludes)                                                               | Saltate i file [CLAUDE.md](/docs/it/memory#exclude-specific-claude-md-files) specifici quando la memoria si carica                                                                                                                                                               | Memory and context                 | Any file                |
| [`cleanupPeriodDays`](#cleanupperioddays)                                                             | Scegliete quanti giorni Claude Code mantiene i [trascritti](/docs/it/data-usage#data-retention) prima di eliminarli                                                                                                                                                              | Privacy and telemetry              | Any file                |
| [`companyAnnouncements`](#companyannouncements)                                                       | Mostrate gli annunci della vostra organizzazione all'avvio                                                                                                                                                                                                                  | Interface and terminal             | Any file                |
| [`copyOnSelect`](#copyonselect)                                                                       | Disattivate la copia automatica del testo che selezionate con il mouse nel [rendering a schermo intero](/docs/it/fullscreen#use-the-mouse) e nella vista agente                                                                                                                  | Global config settings             | Global config           |
| [`crossSessionInbound`](#crosssessioninbound)                                                         | Scegliete se Claude Code consegna i [messaggi dalle vostre altre sessioni](/docs/it/cross-session-messaging#control-inbound-messages), mostra un avviso senza consegnarli, o li rifiuta                                                                                          | Agents, sessions, and worktrees    | Any file                |
| [`defaultShell`](#defaultshell)                                                                       | Scegliete se Bash o PowerShell esegue i comandi shell che digitate con il prefisso [`!`](/docs/it/interactive-mode#shell-mode-with-prefix)                                                                                                                                       | Interface and terminal             | Any file                |
| [`deniedMcpServers`](#deniedmcpservers)                                                               | Bloccate i [server MCP](/docs/it/mcp) specifici per URL, comando o nome                                                                                                                                                                                                          | MCP                                | Any file                |
| [`desktopSessionCleanupPeriodDays`](#desktopsessioncleanupperioddays)                                 | Impostate un limite di età in giorni per i [trascritti di Claude Desktop e Cowork](/docs/it/claude-directory#cleaned-up-automatically)                                                                                                                                           | Privacy and telemetry              | User or managed         |
| [`dialogExpiry`](#dialogexpiry)                                                                       | Impostate quanto a lungo Claude Code aspetta che [Remote Control](/docs/it/remote-control) o un host SDK risponda a una finestra di dialogo inoltrata prima di annullarla                                                                                                        | Interface and terminal             | User or managed         |
| [`diffTool`](#difftool)                                                                               | Scegliete se le modifiche ai file proposte da Claude si aprono nel visualizzatore diff di [VS Code](/docs/it/vs-code) o [JetBrains](/docs/it/jetbrains#features) o rimangono nel terminale                                                                                            | Global config settings             | Global config           |
| [`disableAgentView`](#disableagentview)                                                               | Disattivate gli agenti in background e la [vista agente](/docs/it/agent-view)                                                                                                                                                                                                    | Agents, sessions, and worktrees    | Any file                |
| [`disableAllHooks`](#disableallhooks)                                                                 | Disattivate gli [hook](/docs/it/hooks), una [riga di stato](/docs/it/statusline) personalizzata, e un comando [`@`](/docs/it/interactive-mode#quick-commands) personalizzato di suggerimento di file tutto in una volta                                                                    | Hooks and automation               | Any file                |
| [`disableArtifact`](#disableartifact)                                                                 | Deprecato; usate `enableArtifact` per disattivare lo [strumento Artifact](/docs/it/artifacts)                                                                                                                                                                                    | Remote, desktop, and notifications | Any file                |
| [`disableAutoMode`](#disableautomode)                                                                 | Rimuovete la [modalità automatica](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) dal ciclo della modalità di autorizzazione                                                                                                                                        | Permission settings                | Any file                |
| [`disableBrowserExternalNavigation`](#disablebrowserexternalnavigation)                               | Limitate il riquadro [desktop](/docs/it/desktop) Browser a localhost per le persone e Claude                                                                                                                                                                                     | Tools                              | Managed                 |
| [`disableBundledSkills`](#disablebundledskills)                                                       | Disattivate le [skill](/docs/it/skills#bundled-skills) e i [workflow](/docs/it/workflows) inclusi in Claude Code                                                                                                                                                                      | Plugins and skills                 | Any file                |
| [`disableClaudeAiConnectors`](#disableclaudeaiconnectors)                                             | Disattivate i [connettori claude.ai](/docs/it/mcp#disable-claude-ai-connectors) in modo che Claude Code non li recuperi                                                                                                                                                          | MCP                                | Any file                |
| [`disableCommandPluginSources`](#disablecommandpluginsources)                                         | Bloccate i [plugin](/docs/it/plugins/overview) che si installano eseguendo un comando dichiarato dal marketplace                                                                                                                                                                 | Plugins and skills                 | Managed                 |
| [`disableDeepLinkRegistration`](#disabledeeplinkregistration)                                         | Impedite a Claude Code di registrare il [gestore `claude-cli://`](/docs/it/deep-links)                                                                                                                                                                                           | Remote, desktop, and notifications | Any file                |
| [`disableDesktopLocalSessions`](#disabledesktoplocalsessions)                                         | Disattivate le [sessioni Desktop Code](/docs/it/desktop#local-sessions-on-managed-devices) che vengono eseguite sul dispositivo, lasciando SSH ad altri host e al cloud                                                                                                          | Remote, desktop, and notifications | Managed                 |
| [`disabledMcpjsonServers`](#disabledmcpjsonservers)                                                   | Rifiutate i server specifici dal [`.mcp.json`](/docs/it/mcp#project-scope) di un progetto                                                                                                                                                                                        | MCP                                | Any file                |
| [`disableMobileSimulatorTools`](#disablemobilesimulatortools)                                         | Bloccate gli strumenti di Claude nel riquadro [desktop](/docs/it/desktop) iOS Simulator                                                                                                                                                                                          | Tools                              | Managed                 |
| [`disableRemoteControl`](#disableremotecontrol)                                                       | Disattivate [Remote Control](/docs/it/remote-control) ovunque possa iniziare                                                                                                                                                                                                     | Remote, desktop, and notifications | Any file                |
| [`disableSideloadFlags`](#disablesideloadflags)                                                       | Rifiutate i flag CLI che caricano lateralmente i [plugin](/docs/it/plugins/overview), i [subagent](/docs/it/sub-agents), e i [server MCP](/docs/it/mcp)                                                                                                                                    | Enterprise and managed settings    | Managed                 |
| [`disableSkillShellExecution`](#disableskillshellexecution)                                           | Impedite alle [skill](/docs/it/skills) e ai comandi personalizzati di eseguire shell inline                                                                                                                                                                                      | Plugins and skills                 | Any file                |
| [`disableWorkflows`](#disableworkflows)                                                               | Disattivate i [workflow dinamici](/docs/it/workflows) per tutti; usate `enableWorkflows` per voi stessi                                                                                                                                                                          | Hooks and automation               | Any file                |
| [`editorMode`](#editormode)                                                                           | Usate le [scorciatoie da tastiera vim](/docs/it/interactive-mode#vim-editor-mode) nel prompt di input                                                                                                                                                                            | Interface and terminal             | Any file                |
| [`effortLevel`](#effortlevel)                                                                         | Impostate un [livello di sforzo](/docs/it/model-config#adjust-effort-level) predefinito per i modelli senza un livello salvato                                                                                                                                                   | Model and responses                | Any file                |
| [`emojiCompletionEnabled`](#emojicompletionenabled)                                                   | Disattivate i suggerimenti e la sostituzione di emoji [`:shortcode:`](/docs/it/interactive-mode#emoji-shortcodes) nell'input del prompt                                                                                                                                          | Interface and terminal             | Any file                |
| [`enableAllProjectMcpServers`](#enableallprojectmcpservers)                                           | Approvate ogni server nel file [`.mcp.json`](/docs/it/mcp#project-server-approvals-and-workspace-trust) del progetto senza un prompt                                                                                                                                             | MCP                                | Any file                |
| [`enableArtifact`](#enableartifact)                                                                   | Disattivate lo [strumento Artifact](/docs/it/artifacts) con un `false` in qualsiasi file; nessun file può riattivarlo                                                                                                                                                            | Remote, desktop, and notifications | Any file                |
| [`enabledMcpjsonServers`](#enabledmcpjsonservers)                                                     | Approvate i server specifici dal [`.mcp.json`](/docs/it/mcp#project-server-approvals-and-workspace-trust) di un progetto                                                                                                                                                         | MCP                                | Any file                |
| [`enabledPlugins`](#enabledplugins)                                                                   | Attivate o disattivate i singoli [plugin](/docs/it/plugins/overview) per ambito                                                                                                                                                                                                  | Plugins and skills                 | Any file                |
| [`enableWorkflows`](#enableworkflows)                                                                 | Attivate o disattivate i [workflow dinamici](/docs/it/workflows) rispetto al valore predefinito del vostro piano                                                                                                                                                                 | Hooks and automation               | Any file                |
| [`enforceAvailableModels`](#enforceavailablemodels)                                                   | Mantenete la scelta predefinita di [`/model`](/docs/it/model-config#enforce-the-allowlist-for-the-default-model) all'interno dell'elenco di autorizzazione `availableModels`                                                                                                     | Model and responses                | Any file                |
| [`env`](#env)                                                                                         | Impostate le [variabili di ambiente](/docs/it/env-vars#in-settings-files) per ogni sessione e i suoi sottoprocessi                                                                                                                                                               | Memory and context                 | Any file                |
| [`externalEditorContext`](#externaleditorcontext)                                                     | Mostrate l'ultima risposta di Claude come commenti quando premete [Ctrl+G](/docs/it/interactive-mode#general-controls) per modificare                                                                                                                                            | Global config settings             | Global config           |
| [`extraKnownMarketplaces`](#extraknownmarketplaces)                                                   | Registrate i [marketplace](/docs/it/plugins/overview) per un repository o un'organizzazione                                                                                                                                                                                      | Plugins and skills                 | Any file                |
| [`fallbackModel`](#fallbackmodel)                                                                     | Nominate i [modelli di backup](/docs/it/model-config#fallback-model-chains) per quando il primario è sovraccarico                                                                                                                                                                | Model and responses                | Any file                |
| [`fastMode`](#fastmode)                                                                               | Attivate la [modalità veloce](/docs/it/fast-mode) per le sessioni dove è disponibile                                                                                                                                                                                             | Model and responses                | Any file                |
| [`fastModePerSessionOptIn`](#fastmodepersessionoptin)                                                 | Richiedete alle persone di attivare la [modalità veloce](/docs/it/fast-mode) ogni sessione                                                                                                                                                                                       | Model and responses                | Any file                |
| [`feedbackDrafts`](#feedbackdrafts)                                                                   | Controllate se Claude mette in coda le [bozze di feedback](/docs/it/tools-reference#sendfeedback-tool-behavior) per voi da rivedere                                                                                                                                              | Privacy and telemetry              | User or managed         |
| [`feedbackSurveyRate`](#feedbacksurveyrate)                                                           | Cambiate la frequenza con cui appare il [sondaggio sulla qualità della sessione](/docs/it/data-usage#session-quality-surveys)                                                                                                                                                    | Privacy and telemetry              | Any file                |
| [`fileCheckpointingEnabled`](#filecheckpointingenabled)                                               | Disattivate o attivate gli snapshot di file che [`/rewind`](/docs/it/checkpointing) ripristina                                                                                                                                                                                   | Memory and context                 | Any file                |
| [`fileSuggestion`](#filesuggestion)                                                                   | Fornite l'[autocompletamento dei file `@`](/docs/it/interactive-mode#quick-commands) dal vostro comando                                                                                                                                                                          | Interface and terminal             | Any file                |
| [`footerLinksRegexes`](#footerlinksregexes)                                                           | Rendete gli ID di problema o revisione nell'output in [link cliccabili](/docs/it/statusline#clickable-links) sotto la casella di input                                                                                                                                           | Interface and terminal             | User or managed         |
| [`forceLoginGatewayUrl`](#forcelogingatewayurl)                                                       | Impostate l'[URL del gateway](/docs/it/claude-apps-gateway#set-the-gateway-url) a cui si connette la schermata di accesso                                                                                                                                                        | Authentication and providers       | Managed                 |
| [`forceLoginMethod`](#forceloginmethod)                                                               | [Limitate l'accesso](/docs/it/authentication#restrict-login-to-your-organization) a claude.ai, Claude Console, o a un [gateway cloud](/docs/it/claude-apps-gateway)                                                                                                                   | Authentication and providers       | Any file                |
| [`forceLoginOrgUUID`](#forceloginorguuid)                                                             | [Fissate gli accessi a claude.ai alla vostra organizzazione](/docs/it/authentication#restrict-login-to-your-organization); solo una fonte gestita lo applica                                                                                                                     | Authentication and providers       | Any file                |
| [`forceRemoteSettingsRefresh`](#forceremotesettingsrefresh)                                           | Bloccate l'avvio fino a quando le [impostazioni gestite dal server](/docs/it/server-managed-settings) non vengono recuperate di recente                                                                                                                                          | Enterprise and managed settings    | Managed                 |
| [`gatewayInternalNetworks`](#gatewayinternalnetworks)                                                 | Lasciate che `/login` raggiunga un [gateway cloud](/docs/it/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) su spazio IPv4 pubblico che la vostra organizzazione utilizza internamente                                                                      | Authentication and providers       | Managed                 |
| [`gcpAuthRefresh`](#gcpauthrefresh)                                                                   | Aggiornate le [credenziali Google Cloud](/docs/it/google-vertex-ai#advanced-credential-configuration) con il vostro comando                                                                                                                                                      | Authentication and providers       | Any file                |
| [`hooks`](#hooks)                                                                                     | Eseguite i vostri comandi come [hook](/docs/it/hooks) nei punti del ciclo di vita di Claude Code                                                                                                                                                                                 | Hooks and automation               | Any file                |
| [`httpHookAllowedEnvVars`](#httphookallowedenvvars)                                                   | Limitate quali variabili di ambiente gli [hook HTTP](/docs/it/hooks) possono mettere nelle intestazioni                                                                                                                                                                          | Hooks and automation               | Any file                |
| [`includeCoAuthoredBy`](#includecoauthoredby)                                                         | Deprecato; usate `attribution` per nascondere o modificare l'attribuzione di commit e PR                                                                                                                                                                                    | Git and attribution                | Any file                |
| [`includeGitInstructions`](#includegitinstructions)                                                   | Rimuovete le istruzioni di commit e PR integrate dal contesto di Claude                                                                                                                                                                                                     | Git and attribution                | Any file                |
| [`inputNeededNotifEnabled`](#inputneedednotifenabled)                                                 | Ricevete una [notifica push](/docs/it/remote-control#mobile-push-notifications) quando Claude vi sta aspettando                                                                                                                                                                  | Remote, desktop, and notifications | Any file                |
| [`isolatePeerMachines`](#isolatepeermachines)                                                         | Chiedete prima che Claude [messaggi una delle vostre sessioni su un'altra macchina](/docs/it/cross-session-messaging#require-approval-for-cross-machine-messages)                                                                                                                | Agents, sessions, and worktrees    | Any file                |
| [`keybindingFlavor`](#keybindingflavor)                                                               | Deprecato e non ha effetto; le scorciatoie di modifica delle parole seguono sempre le [convenzioni readline](/docs/it/interactive-mode#make-ctrl-w-delete-back-to-whitespace)                                                                                                    | Interface and terminal             | Any file                |
| [`language`](#language)                                                                               | Fate in modo che Claude risponda in una lingua diversa dall'inglese                                                                                                                                                                                                         | Model and responses                | Any file                |
| [`managedMcpServers`](#managedmcpservers)                                                             | Fornite i [server MCP](/docs/it/managed-mcp#provide-servers-through-managed-settings) remoti a ogni utente insieme a quelli che aggiungono                                                                                                                                       | MCP                                | Managed                 |
| [`managedSourcesBehavior`](#managedsourcesbehavior)                                                   | Componete ogni [fonte gestita](/docs/it/managed-settings#how-claude-code-combines-managed-sources) che distribuite invece di usare solo quella con la priorità più alta                                                                                                          | Enterprise and managed settings    | Managed                 |
| [`maxEffortLevel`](#maxeffortlevel)                                                                   | Limitate il [livello di sforzo](/docs/it/model-config#adjust-effort-level) per ogni modello o per modello, su ogni provider                                                                                                                                                      | Model and responses                | Any file                |
| [`minimumVersion`](#minimumversion)                                                                   | Impedite agli [aggiornamenti automatici](/docs/it/setup#pin-a-minimum-version) di installare qualsiasi cosa al di sotto di una versione                                                                                                                                          | Updates and versioning             | Any file                |
| [`model`](#model)                                                                                     | Cambiate il [modello](/docs/it/model-config#set-a-default-model-for-new-sessions) con cui Claude Code inizia                                                                                                                                                                     | Model and responses                | Any file                |
| [`modelOverrides`](#modeloverrides)                                                                   | [Mappate gli ID dei modelli](/docs/it/model-config#override-model-ids-per-version) agli ID del vostro provider, come gli ARN di Bedrock                                                                                                                                          | Model and responses                | Any file                |
| [`modelPicker`](#modelpicker)                                                                         | Scegliete quali modelli il [selezionatore `/model`](/docs/it/model-config#available-models) elenca, nel vostro ordine e con le vostre etichette                                                                                                                                  | Model and responses                | User or managed         |
| [`modelPricing`](#modelpricing)                                                                       | Segnalate la spesa alle tariffe contrattuali della vostra organizzazione invece del prezzo di listino                                                                                                                                                                       | Model and responses                | Managed                 |
| [`modelSettings`](#modelsettings)                                                                     | Mantenete un [livello di sforzo](/docs/it/model-config#adjust-effort-level) salvato per modello, o limitate lo sforzo di un modello                                                                                                                                              | Model and responses                | Any file                |
| [`otelHeadersHelper`](#otelheadershelper)                                                             | Generare le intestazioni [OpenTelemetry](/docs/it/monitoring-usage#dynamic-headers) rotanti con il vostro comando                                                                                                                                                                | Authentication and providers       | Any file                |
| [`outputStyle`](#outputstyle)                                                                         | Cambiate il ruolo, il tono e il formato di output di Claude con uno [stile di output](/docs/it/output-styles)                                                                                                                                                                    | Model and responses                | Any file                |
| [`parentSettingsBehavior`](#parentsettingsbehavior)                                                   | Applicate o eliminate le restrizioni che un [host SDK o IDE](/docs/it/managed-settings#let-an-embedding-host-add-policy) passa quando distribuite le [impostazioni gestite](/docs/it/managed-settings)                                                                                | Enterprise and managed settings    | Managed                 |
| [`permissionExplainerEnabled`](#permissionexplainerenabled)                                           | Rimosso nella v2.1.257, insieme al comando di spiegazione `Ctrl+E` sui prompt di autorizzazione shell                                                                                                                                                                       | Global config settings             | Global config           |
| [`permissions`](#permissions)                                                                         | Impostate le regole di autorizzazione, domanda e negazione e la [modalità di autorizzazione](/docs/it/permission-modes) iniziale                                                                                                                                                 | Permission settings                | Any file                |
| [`permissions.additionalDirectories`](#permissions-additionaldirectories)                             | Fornite a Claude l'accesso ai file alle [directory al di fuori di quella attuale](/docs/it/permissions#working-directories)                                                                                                                                                      | Permission settings                | Any file                |
| [`permissions.allow`](#permissions-allow)                                                             | Approvate gli [usi dello strumento](/docs/it/permissions#permission-rule-syntax) elencati senza un prompt                                                                                                                                                                        | Permission settings                | Any file                |
| [`permissions.ask`](#permissions-ask)                                                                 | Chiedete sempre prima degli [usi dello strumento](/docs/it/permissions#permission-rule-syntax) elencati                                                                                                                                                                          | Permission settings                | Any file                |
| [`permissions.blockReadsOutsideWorkingDirectories`](#permissions-blockreadsoutsideworkingdirectories) | Fate in modo che gli strumenti di file rifiutino le letture al di fuori delle [directory di lavoro](/docs/it/permissions#working-directories) in ogni modalità di autorizzazione                                                                                                 | Permission settings                | Any file                |
| [`permissions.defaultMode`](#permissions-defaultmode)                                                 | Impostate la [modalità di autorizzazione](/docs/it/permission-modes#which-mode-a-session-starts-in) in cui iniziano le nuove sessioni                                                                                                                                            | Permission settings                | Any file                |
| [`permissions.deny`](#permissions-deny)                                                               | Bloccate gli [usi dello strumento](/docs/it/permissions#permission-rule-syntax) elencati, incluse le letture di file che contengono segreti                                                                                                                                      | Permission settings                | Any file                |
| [`permissions.disableBypassPermissionsMode`](#permissions-disablebypasspermissionsmode)               | Impedite a chiunque di entrare nella [modalità bypassPermissions](/docs/it/permission-modes#skip-all-checks-with-bypasspermissions-mode)                                                                                                                                         | Permission settings                | Any file                |
| [`plansDirectory`](#plansdirectory)                                                                   | Scegliete dove la [modalità plan](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode) scrive i file di piano                                                                                                                                                       | Memory and context                 | Any file                |
| [`pluginConfigs`](#pluginconfigs)                                                                     | Archiviate le risposte che avete dato alla finestra di dialogo di configurazione di un [plugin](/docs/it/plugins/overview)                                                                                                                                                       | Plugins and skills                 | User or managed         |
| [`pluginSuggestionMarketplaces`](#pluginsuggestionmarketplaces)                                       | Scegliete quali [marketplace](/docs/it/plugins/org#restrict-what-users-can-install) possono far emergere i suggerimenti di installazione dei plugin in `/plugin`                                                                                                                 | Plugins and skills                 | Managed                 |
| [`pluginTrustMessage`](#plugintrustmessage)                                                           | Aggiungete il vostro testo all'avviso di fiducia del [plugin](/docs/it/plugins/overview)                                                                                                                                                                                         | Plugins and skills                 | Managed                 |
| [`policyHelper`](#policyhelper)                                                                       | Eseguite un eseguibile che calcola le [impostazioni gestite](/docs/it/managed-settings#compute-the-policy-with-a-helper-program) all'avvio                                                                                                                                       | Enterprise and managed settings    | Managed                 |
| [`policyHelper.path`](#policyhelper-path)                                                             | Nominate l'[eseguibile helper](/docs/it/managed-settings#compute-the-policy-with-a-helper-program) che Claude Code esegue                                                                                                                                                        | Enterprise and managed settings    | Managed                 |
| [`policyHelper.refreshIntervalMs`](#policyhelper-refreshintervalms)                                   | Rieseguite l'[helper](/docs/it/managed-settings#compute-the-policy-with-a-helper-program) in background a intervalli                                                                                                                                                             | Enterprise and managed settings    | Managed                 |
| [`policyHelper.timeoutMs`](#policyhelper-timeoutms)                                                   | Impostate quanto a lungo Claude Code aspetta l'[helper](/docs/it/managed-settings#compute-the-policy-with-a-helper-program)                                                                                                                                                      | Enterprise and managed settings    | Managed                 |
| [`preferredNotifChannel`](#preferrednotifchannel)                                                     | Scegliete un [campanello del terminale o una notifica desktop](/docs/it/terminal-config#get-a-terminal-bell-or-notification) per il completamento del compito                                                                                                                    | Remote, desktop, and notifications | Any file                |
| [`prefersReducedMotion`](#prefersreducedmotion)                                                       | [Riducete o disattivate](/docs/it/accessibility#accessibility-settings) le animazioni di spinner, shimmer e flash                                                                                                                                                                | Interface and terminal             | Any file                |
| [`processWrapper`](#processwrapper)                                                                   | Eseguite i processi in background di Claude Code attraverso un [launcher aziendale](/docs/it/corporate-launcher) su macOS e Linux                                                                                                                                                | Agents, sessions, and worktrees    | User or managed         |
| [`promptCacheTtl`](#promptcachettl)                                                                   | Scegliete la [durata della cache del prompt](/docs/it/prompt-caching#cache-lifetime) per la conversazione principale                                                                                                                                                             | Model and responses                | Any file                |
| [`promptSuggestionEnabled`](#promptsuggestionenabled)                                                 | Nascondetei [suggerimenti di prompt](/docs/it/interactive-mode#prompt-suggestions) in grigio nella casella di input                                                                                                                                                              | Interface and terminal             | Any file                |
| [`prUrlTemplate`](#prurltemplate)                                                                     | Puntate i link PR a uno strumento di revisione del codice interno invece di github.com                                                                                                                                                                                      | Git and attribution                | Any file                |
| [`remote.defaultEnvironmentId`](#remote-defaultenvironmentid)                                         | Scegliete l'[ambiente cloud](/docs/it/cloud-environments) predefinito per `claude --cloud`; un ID `ccpool_` auto-ospitato è di sola lettura dalle impostazioni utente e gestite e `--settings`                                                                                   | Remote, desktop, and notifications | Any file                |
| [`remoteControlAtStartup`](#remotecontrolatstartup)                                                   | Connettetevi a [Remote Control](/docs/it/remote-control#enable-remote-control-for-all-sessions) automaticamente quando una sessione inizia                                                                                                                                       | Remote, desktop, and notifications | Any file                |
| [`requiredMaximumVersion`](#requiredmaximumversion)                                                   | [Rifiutate di avviare](/docs/it/setup#pin-a-minimum-version) su una versione più recente di quella che la vostra organizzazione consente                                                                                                                                         | Updates and versioning             | Managed                 |
| [`requiredMinimumVersion`](#requiredminimumversion)                                                   | [Rifiutate di avviare](/docs/it/setup#pin-a-minimum-version) su una versione più vecchia di quella che la vostra organizzazione richiede                                                                                                                                         | Updates and versioning             | Managed                 |
| [`respectGitignore`](#respectgitignore)                                                               | Mantenete i file ignorati da git fuori dal [selezionatore di file `@`](/docs/it/interactive-mode#quick-commands)                                                                                                                                                                 | Interface and terminal             | Any file                |
| [`respondToBashCommands`](#respondtobashcommands)                                                     | Impedite a Claude di rispondere dopo l'esecuzione di un [comando shell `!`](/docs/it/interactive-mode#shell-mode-with-prefix)                                                                                                                                                    | Interface and terminal             | Any file                |
| [`sandbox`](#sandbox)                                                                                 | [Isolate i comandi Bash](/docs/it/sandboxing) dal vostro filesystem e dalla rete su macOS, Linux e WSL2                                                                                                                                                                          | Sandbox settings                   | Any file                |
| [`sandbox.allowAppleEvents`](#sandbox-allowappleevents)                                               | Lasciate che i [comandi in sandbox](/docs/it/sandboxing) inviino Apple Events su macOS                                                                                                                                                                                           | Sandbox settings                   | User or managed         |
| [`sandbox.allowUnsandboxedCommands`](#sandbox-allowunsandboxedcommands)                               | Lasciate che Claude riprovi un comando bloccato al di fuori della [sandbox](/docs/it/sandboxing#the-unsandboxed-retry-escape-hatch), o vietatelo                                                                                                                                 | Sandbox settings                   | Any file                |
| [`sandbox.autoAllowBashIfSandboxed`](#sandbox-autoallowbashifsandboxed)                               | Eseguite i [comandi in sandbox](/docs/it/sandboxing#auto-allow-mode) senza un prompt di autorizzazione                                                                                                                                                                           | Sandbox settings                   | Any file                |
| [`sandbox.bwrapPath`](#sandbox-bwrappath)                                                             | Puntate la [sandbox](/docs/it/sandboxing) a un binario bubblewrap al di fuori di `PATH`                                                                                                                                                                                          | Sandbox settings                   | Managed                 |
| [`sandbox.credentials`](#sandbox-credentials)                                                         | Nascondeteo mascherate i file e le variabili di credenziale all'interno della [sandbox](/docs/it/sandboxing#protect-credentials)                                                                                                                                                 | Sandbox settings                   | Any file                |
| [`sandbox.credentials.allowPlaintextInject`](#sandbox-credentials-allowplaintextinject)               | Lasciate che le [credenziali mascherate](/docs/it/sandboxing#mask-credentials) raggiungano i servizi HTTP semplici su reti di test affidabili                                                                                                                                    | Sandbox settings                   | User or managed         |
| [`sandbox.credentials.awsPairs`](#sandbox-credentials-awspairs)                                       | Collegate le variabili di chiave AWS con nome personalizzato in una credenziale per la [ri-firma](/docs/it/sandboxing#re-sign-aws-requests)                                                                                                                                      | Sandbox settings                   | User or managed         |
| [`sandbox.credentials.envVars`](#sandbox-credentials-envvars)                                         | Annullate o mascherate una variabile di ambiente all'interno della [sandbox](/docs/it/sandboxing#mask-environment-variables)                                                                                                                                                     | Sandbox settings                   | Any file                |
| [`sandbox.credentials.files`](#sandbox-credentials-files)                                             | Bloccate o mascherate le letture di un file di credenziale all'interno della [sandbox](/docs/it/sandboxing#mask-credential-files)                                                                                                                                                | Sandbox settings                   | Any file                |
| [`sandbox.credentials.sigv4`](#sandbox-credentials-sigv4)                                             | Scegliete se le richieste AWS [SigV4A](/docs/it/sandboxing#re-sign-aws-requests) in streaming, prescritte o falliscono o passano                                                                                                                                                 | Sandbox settings                   | User or managed         |
| [`sandbox.enabled`](#sandbox-enabled)                                                                 | Attivate il [sandboxing di Bash](/docs/it/sandboxing#get-started) su macOS, Linux e WSL2                                                                                                                                                                                         | Sandbox settings                   | Any file                |
| [`sandbox.enableWeakerNestedSandbox`](#sandbox-enableweakernestedsandbox)                             | Eseguite la [sandbox](/docs/it/sandboxing) Linux all'interno di un contenitore senza privilegi                                                                                                                                                                                   | Sandbox settings                   | Any file                |
| [`sandbox.enableWeakerNetworkIsolation`](#sandbox-enableweakernetworkisolation)                       | Lasciate che `gh`, `gcloud` e `terraform` verifichino TLS dietro un proxy MITM all'interno della [sandbox](/docs/it/sandboxing#troubleshooting) su macOS                                                                                                                         | Sandbox settings                   | Any file                |
| [`sandbox.excludedCommands`](#sandbox-excludedcommands)                                               | Nominate i comandi che Claude Code può eseguire al di fuori della [sandbox](/docs/it/sandboxing)                                                                                                                                                                                 | Sandbox settings                   | Any file                |
| [`sandbox.failIfUnavailable`](#sandbox-failifunavailable)                                             | Rifiutate di avviare quando la [sandbox](/docs/it/sandboxing) non può, invece di eseguire senza sandbox                                                                                                                                                                          | Sandbox settings                   | Any file                |
| [`sandbox.filesystem`](#sandbox-filesystem)                                                           | Controllate quali percorsi i [comandi in sandbox](/docs/it/sandboxing#filesystem-isolation) possono leggere e scrivere                                                                                                                                                           | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly)       | Impedite agli sviluppatori di riaprire i [percorsi di lettura che la vostra organizzazione ha bloccato](/docs/it/sandboxing#keep-developers-from-widening-the-policy)                                                                                                            | Sandbox settings                   | Managed                 |
| [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread)                                       | Riaprire la lettura all'interno di una regione che [`denyRead`](#sandbox-filesystem-denyread) blocca                                                                                                                                                                        | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.allowWrite`](#sandbox-filesystem-allowwrite)                                     | Aggiungete i percorsi che i [comandi in sandbox](/docs/it/sandboxing) possono scrivere                                                                                                                                                                                           | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.denyRead`](#sandbox-filesystem-denyread)                                         | Bloccate i [comandi in sandbox](/docs/it/sandboxing) dalla lettura di percorsi specifici                                                                                                                                                                                         | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.denyWrite`](#sandbox-filesystem-denywrite)                                       | Bloccate i [comandi in sandbox](/docs/it/sandboxing) dalla scrittura in percorsi specifici                                                                                                                                                                                       | Sandbox settings                   | Any file                |
| [`sandbox.filesystem.disabled`](#sandbox-filesystem-disabled)                                         | [Disattivate l'isolamento del filesystem](/docs/it/sandboxing#disable-filesystem-isolation) mantenendo l'isolamento della rete                                                                                                                                                   | Sandbox settings                   | User or managed         |
| [`sandbox.ignoreViolations`](#sandbox-ignoreviolations)                                               | Silenziategli avvisi di violazione per i percorsi che un comando dovrebbe sondare                                                                                                                                                                                           | Sandbox settings                   | Any file                |
| [`sandbox.network`](#sandbox-network)                                                                 | Controllate quali host, porte e socket i [comandi in sandbox](/docs/it/sandboxing#network-isolation) raggiungono                                                                                                                                                                 | Sandbox settings                   | Any file                |
| [`sandbox.network.allowAllUnixSockets`](#sandbox-network-allowallunixsockets)                         | Lasciate che i [comandi in sandbox](/docs/it/sandboxing) si connettano a ogni socket Unix                                                                                                                                                                                        | Sandbox settings                   | Any file                |
| [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains)                                   | Pre-autorizzate i domini in modo che i [comandi in sandbox](/docs/it/sandboxing) non chiedano loro                                                                                                                                                                               | Sandbox settings                   | Any file                |
| [`sandbox.network.allowLocalBinding`](#sandbox-network-allowlocalbinding)                             | Lasciate che i [comandi in sandbox](/docs/it/sandboxing) si colleghino alle porte localhost su macOS                                                                                                                                                                             | Sandbox settings                   | Any file                |
| [`sandbox.network.allowMachLookup`](#sandbox-network-allowmachlookup)                                 | Lasciate che gli strumenti macOS [in sandbox](/docs/it/sandboxing) come il simulatore iOS o Playwright raggiungano i loro servizi XPC                                                                                                                                            | Sandbox settings                   | Any file                |
| [`sandbox.network.allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly)                 | Bloccate l'elenco di autorizzazione della rete alle [impostazioni gestite](/docs/it/sandboxing#keep-developers-from-widening-the-policy)                                                                                                                                         | Sandbox settings                   | Managed                 |
| [`sandbox.network.allowUnixSockets`](#sandbox-network-allowunixsockets)                               | Elencate i percorsi dei socket Unix che i [comandi in sandbox](/docs/it/sandboxing) possono utilizzare su macOS                                                                                                                                                                  | Sandbox settings                   | Any file                |
| [`sandbox.network.deniedDomains`](#sandbox-network-denieddomains)                                     | Bloccate i domini per i [comandi in sandbox](/docs/it/sandboxing), anche all'interno di un wildcard consentito                                                                                                                                                                   | Sandbox settings                   | Any file                |
| [`sandbox.network.httpProxyPort`](#sandbox-network-httpproxyport)                                     | Instradateil traffico HTTP della [sandbox](/docs/it/sandboxing#custom-proxy-configuration) attraverso il vostro proxy                                                                                                                                                            | Sandbox settings                   | Any file                |
| [`sandbox.network.socksProxyPort`](#sandbox-network-socksproxyport)                                   | Instradateil traffico SOCKS della [sandbox](/docs/it/sandboxing#custom-proxy-configuration) attraverso il vostro proxy                                                                                                                                                           | Sandbox settings                   | Any file                |
| [`sandbox.network.strictAllowlist`](#sandbox-network-strictallowlist)                                 | Negate gli host al di fuori dell'[elenco di autorizzazione](/docs/it/sandboxing#network-isolation) invece di chiedere                                                                                                                                                            | Sandbox settings                   | User or managed         |
| [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate)                                       | Fate in modo che la [sandbox](/docs/it/sandboxing#network-isolation) proxy termini TLS in modo che possa leggere le richieste HTTPS                                                                                                                                              | Sandbox settings                   | User or managed         |
| [`sandbox.ripgrep`](#sandbox-ripgrep)                                                                 | Usate il vostro binario ripgrep all'interno della [sandbox](/docs/it/sandboxing)                                                                                                                                                                                                 | Sandbox settings                   | User or managed         |
| [`sandbox.socatPath`](#sandbox-socatpath)                                                             | Puntate il proxy della [sandbox](/docs/it/sandboxing) a un binario `socat` al di fuori di `PATH`                                                                                                                                                                                 | Sandbox settings                   | Managed                 |
| [`showClearContextOnPlanAccept`](#showclearcontextonplanaccept)                                       | Mostrate un'opzione "cancella contesto" sulla [schermata di accettazione del piano](/docs/it/permission-modes#review-and-approve-a-plan)                                                                                                                                         | Interface and terminal             | Any file                |
| [`showThinkingSummaries`](#showthinkingsummaries)                                                     | Vedete i riassunti del [pensiero](/docs/it/model-config#extended-thinking) di Claude invece di uno stub compresso                                                                                                                                                                | Model and responses                | Any file                |
| [`showTurnDuration`](#showturnduration)                                                               | Nascondetela durata "Cooked for" dopo ogni risposta                                                                                                                                                                                                                         | Interface and terminal             | Any file                |
| [`skillListingBudgetFraction`](#skilllistingbudgetfraction)                                           | Riservate più o meno contesto per l'[elenco di skill](/docs/it/skills#skill-descriptions-are-cut-short)                                                                                                                                                                          | Memory and context                 | Any file                |
| [`skillListingMaxDescChars`](#skilllistingmaxdescchars)                                               | Limitate la lunghezza della descrizione di ogni skill nell'[elenco di skill](/docs/it/skills#skill-descriptions-are-cut-short)                                                                                                                                                   | Memory and context                 | Any file                |
| [`skillOverrides`](#skilloverrides)                                                                   | [Nascondeteo comprimete una skill](/docs/it/skills#override-skill-visibility-from-settings) senza modificare il suo SKILL.md                                                                                                                                                     | Plugins and skills                 | Any file                |
| [`skipAutoPermissionPrompt`](#skipautopermissionprompt)                                               | Saltate l'avviso una tantum che Claude Code mostra quando entrate per la prima volta nella [modalità automatica](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) voi stessi piuttosto che attraverso il valore predefinito integrato                                 | Permission settings                | User or managed         |
| [`skipDangerousModePermissionPrompt`](#skipdangerousmodepermissionprompt)                             | Saltate la finestra di dialogo di conferma prima della [modalità bypassPermissions](/docs/it/permission-modes#skip-all-checks-with-bypasspermissions-mode)                                                                                                                       | Permission settings                | User, local, or managed |
| [`skipWebFetchPreflight`](#skipwebfetchpreflight)                                                     | Saltate il [controllo del nome host WebFetch](/docs/it/tools-reference#webfetch-tool-behavior) quando Anthropic non è raggiungibile                                                                                                                                              | Privacy and telemetry              | Any file                |
| [`spellcheck`](#spellcheck)                                                                           | Sottolineate le parole scritte male nell'input del prompt con un [correttore ortografico](/docs/it/interactive-mode#check-spelling-as-you-type) che installate                                                                                                                   | Interface and terminal             | User or managed         |
| [`spinnerTipsEnabled`](#spinnertipsenabled)                                                           | Nascondetei suggerimenti nello spinner mentre Claude lavora                                                                                                                                                                                                                 | Interface and terminal             | Any file                |
| [`spinnerTipsOverride`](#spinnertipsoverride)                                                         | Aggiungete i vostri suggerimenti alla rotazione dello spinner, o sostituite i suggerimenti integrati                                                                                                                                                                        | Interface and terminal             | Any file                |
| [`spinnerVerbs`](#spinnerverbs)                                                                       | Aggiungete o sostituite i verbi mostrati mentre un turno viene eseguito                                                                                                                                                                                                     | Interface and terminal             | Any file                |
| [`sshConfigs`](#sshconfigs)                                                                           | Aggiungete le [connessioni SSH](/docs/it/desktop#pre-configure-ssh-connections-for-your-team) al menu a discesa dell'ambiente Desktop                                                                                                                                            | Remote, desktop, and notifications | User or managed         |
| [`sshHostAllowlist`](#sshhostallowlist)                                                               | Limitate gli host che le [sessioni SSH Desktop](/docs/it/desktop#restrict-which-ssh-hosts-users-can-connect-to) possono raggiungere                                                                                                                                              | Remote, desktop, and notifications | Managed                 |
| [`statusLine`](#statusline)                                                                           | Eseguite il vostro comando per rendere una [riga di stato](/docs/it/statusline) sotto il prompt                                                                                                                                                                                  | Interface and terminal             | Any file                |
| [`strictKnownMarketplaces`](#strictknownmarketplaces)                                                 | Elencate in whitelist le fonti del [marketplace](/docs/it/plugins/overview) che gli utenti possono aggiungere e installare                                                                                                                                                       | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)                                     | Bloccate le [skill](/docs/it/skills), gli [agenti](/docs/it/sub-agents), gli [hook](/docs/it/hooks) e i [server MCP](/docs/it/mcp) dalle fonti utente e progetto                                                                                                                                | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization.agents`](#strictpluginonlycustomization-agents)                       | Bloccate gli [agenti](/docs/it/sub-agents) alle fonti plugin e gestite                                                                                                                                                                                                           | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization.hooks`](#strictpluginonlycustomization-hooks)                         | Bloccate gli [hook](/docs/it/hooks) alle fonti plugin e gestite                                                                                                                                                                                                                  | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization.mcp`](#strictpluginonlycustomization-mcp)                             | Bloccate i [server MCP](/docs/it/mcp) alle fonti plugin e gestite                                                                                                                                                                                                                | Plugins and skills                 | Managed                 |
| [`strictPluginOnlyCustomization.skills`](#strictpluginonlycustomization-skills)                       | Bloccate le [skill](/docs/it/skills) alle fonti plugin e gestite                                                                                                                                                                                                                 | Plugins and skills                 | Managed                 |
| [`subagentPromptCacheTtl`](#subagentpromptcachettl)                                                   | Scegliete la [durata della cache del prompt](/docs/it/prompt-caching#cache-lifetime) per i subagent e altre richieste al di fuori della conversazione principale                                                                                                                 | Model and responses                | Any file                |
| [`subagentStatusLine`](#subagentstatusline)                                                           | Riscrivetle righe nella [visualizzazione del subagent](/docs/it/sub-agents) con il vostro comando                                                                                                                                                                                | Interface and terminal             | Any file                |
| [`switchModelsOnFlag`](#switchmodelsonflag)                                                           | Cambiate i modelli automaticamente o fate una pausa quando un [classificatore di sicurezza](/docs/it/model-config#ask-before-switching) contrassegna una richiesta                                                                                                               | Model and responses                | Any file                |
| [`syncClaudeAiPlugins`](#syncclaudeaiplugins)                                                         | Smettete di caricare i [plugin abilitati sul vostro account claude.ai](/docs/it/plugins/loading#synced-plugins) e smettete di scaricare quelli nuovi                                                                                                                             | Plugins and skills                 | User, local, or managed |
| [`syncClaudeAiSkills`](#syncclaudeaiskills)                                                           | Smettete di scaricare le [skill abilitate sul vostro account claude.ai](/docs/it/skills#how-synced-skills-behave) e nascondetequelle già sincronizzate                                                                                                                           | Plugins and skills                 | User, local, or managed |
| [`syntaxHighlightingDisabled`](#syntaxhighlightingdisabled)                                           | Disattivate l'evidenziazione della sintassi nei diff e nei blocchi di codice                                                                                                                                                                                                | Interface and terminal             | Any file                |
| [`taskOutputMaxChars`](#taskoutputmaxchars)                                                           | Rimosso nella v2.1.277, insieme allo strumento `TaskOutput` che ha dimensionato                                                                                                                                                                                             | Memory and context                 | Any file                |
| [`teammateDefaultModel`](#teammatedefaultmodel)                                                       | Rimosso nella v2.1.234; vedere [Specificare i compagni di squadra e i modelli](/docs/it/agent-teams#specify-teammates-and-models) per come Claude Code sceglie il modello di un compagno di squadra                                                                              | Global config settings             | Global config           |
| [`teammateMode`](#teammatemode)                                                                       | Scegliete come i [compagni di squadra del team di agenti](/docs/it/agent-teams#choose-a-display-mode) vengono visualizzati                                                                                                                                                       | Agents, sessions, and worktrees    | Any file                |
| [`terminalProgressBarEnabled`](#terminalprogressbarenabled)                                           | Nascondetela barra di avanzamento del terminale nei terminali che la supportano                                                                                                                                                                                             | Interface and terminal             | Any file                |
| [`terminalTitleFromRename`](#terminaltitlefromrename)                                                 | Impedite a [`/rename`](/docs/it/sessions#name-your-sessions) e `--name` di modificare il titolo della scheda del terminale                                                                                                                                                       | Interface and terminal             | Any file                |
| [`theme`](#theme)                                                                                     | Scegliete il [tema colore](/docs/it/terminal-config#match-the-color-theme) dell'interfaccia, integrato o personalizzato                                                                                                                                                          | Interface and terminal             | Any file                |
| [`timeFormat`](#timeformat)                                                                           | Mostrate i tempi nell'interfaccia su un orologio a 12 ore o 24 ore, in UTC, o con un modello strftime                                                                                                                                                                       | Interface and terminal             | Any file                |
| [`timeZone`](#timezone)                                                                               | Mostrate i tempi nell'interfaccia in un fuso orario diverso da quello del vostro sistema                                                                                                                                                                                    | Interface and terminal             | Any file                |
| [`tui`](#tui)                                                                                         | Scegliete il renderer [a schermo intero](/docs/it/fullscreen) o terminale classico                                                                                                                                                                                               | Interface and terminal             | Any file                |
| [`ultracode`](#ultracode)                                                                             | Fate in modo che Claude pianifichi un [workflow](/docs/it/workflows#let-claude-decide-with-ultracode) per ogni compito sostanziale senza essere chiesto                                                                                                                          | Model and responses                | Any file                |
| [`useAutoModeDuringPlan`](#useautomodeduringplan)                                                     | Lasciate che il classificatore della [modalità automatica](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) riveda i comandi shell nella [modalità plan](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode); impostate `false` per ottenere i prompt invece | Permission settings                | User, local, or managed |
| [`verbose`](#verbose)                                                                                 | Mostrate l'[output completo dello strumento](/docs/it/cli-reference#cli-flags) invece di riassunti troncati; `viewMode` ha la precedenza quando entrambi sono impostati                                                                                                          | Interface and terminal             | Any file                |
| [`viewMode`](#viewmode)                                                                               | Iniziate ogni sessione nella [visualizzazione predefinita, dettagliata o focalizzata](/docs/it/cli-reference#cli-flags)                                                                                                                                                          | Interface and terminal             | Any file                |
| [`vimInsertModeRemaps`](#viminsertmoderemaps)                                                         | Mappate una [sequenza in modalità INSERT](/docs/it/interactive-mode#remap-insert-mode-key-sequences) a due tasti come `jj` su Escape                                                                                                                                             | Interface and terminal             | User or managed         |
| [`voice`](#voice)                                                                                     | Attivate la [dettatura vocale](/docs/it/voice-dictation) e scegliete la modalità di mantenimento o tocco                                                                                                                                                                         | Interface and terminal             | Any file                |
| [`voiceEnabled`](#voiceenabled)                                                                       | Attivate la [dettatura vocale](/docs/it/voice-dictation) con il modulo a chiave singola più vecchio                                                                                                                                                                              | Interface and terminal             | Any file                |
| [`wheelScrollAccelerationEnabled`](#wheelscrollaccelerationenabled)                                   | Disattivate l'[accelerazione della rotella del mouse](/docs/it/fullscreen#mouse-wheel-scrolling) nel rendering a schermo intero                                                                                                                                                  | Interface and terminal             | Any file                |
| [`workflowKeywordTriggerEnabled`](#workflowkeywordtriggerenabled)                                     | Lasciate che la parola `ultracode` in un prompt avvii un [workflow](/docs/it/workflows); impostate `false` per digitarla senza avviarne uno                                                                                                                                      | Hooks and automation               | Any file                |
| [`workflowSizeGuideline`](#workflowsizeguideline)                                                     | Impostate il numero di agenti a cui Claude mira nei [workflow dinamici](/docs/it/workflows)                                                                                                                                                                                      | Hooks and automation               | Any file                |
| [`worktree`](#worktree)                                                                               | Configurate come Claude Code crea git [worktree](/docs/it/worktrees)                                                                                                                                                                                                             | Agents, sessions, and worktrees    | Any file                |
| [`worktree.baseRef`](#worktree-baseref)                                                               | Ramificate i nuovi [worktree](/docs/it/worktrees) dal ramo predefinito remoto o dal vostro HEAD locale                                                                                                                                                                           | Agents, sessions, and worktrees    | Any file                |
| [`worktree.bgIsolation`](#worktree-bgisolation)                                                       | Lasciate che le sessioni in background modifichino la copia di lavoro senza un [worktree](/docs/it/worktrees)                                                                                                                                                                    | Agents, sessions, and worktrees    | Any file                |
| [`worktree.sparsePaths`](#worktree-sparsepaths)                                                       | Controllate solo le directory di cui avete bisogno in ogni [worktree](/docs/it/worktrees)                                                                                                                                                                                        | Agents, sessions, and worktrees    | Any file                |
| [`worktree.symlinkDirectories`](#worktree-symlinkdirectories)                                         | Collegate simbolicamente le directory di grandi dimensioni in ogni [worktree](/docs/it/worktrees) invece di duplicarle                                                                                                                                                           | Agents, sessions, and worktrees    | Any file                |
| [`wslInheritsWindowsSettings`](#wslinheritswindowssettings)                                           | Fate in modo che WSL legga le [impostazioni gestite](/docs/it/managed-settings) dalla catena di criteri di Windows                                                                                                                                                               | Enterprise and managed settings    | Managed                 |

<h2 id="model-and-responses">
  Modello e risposte
</h2>

Scegliere quali modelli Claude Code utilizza e come risponde. Per informazioni su come queste impostazioni interagiscono con il comando `/model` e le variabili di ambiente, vedere [Configurazione del modello](/docs/it/model-config).

<h3 id="advisormodel">
  `advisorModel`
</h3>

Scegliere quale modello risponde quando Claude chiama lo [strumento advisor](/docs/it/advisor) lato server. Lasciarlo non impostato per disattivare l'advisor. L'advisor deve essere almeno altrettanto capace del modello principale. Vedere [Scegliere un modello advisor](/docs/it/advisor#choose-an-advisor-model) per gli accoppiamenti accettati e cosa accade quando se ne sceglie uno che non è accettato.

Di solito non si modifica questa chiave manualmente. Eseguire `/advisor` per aprire un selettore che mostra la scelta corrente, i modelli che possono fornire consulenza e **No advisor**. Claude Code salva la scelta in questa chiave in `~/.claude/settings.json`. Se si sceglie da un client [Remote Control](/docs/it/remote-control) o in una sessione collegata a un worker remoto, la scelta si applica solo a quella sessione e non modifica questa chiave.

Se l'account richiede il [consenso usage-credits](/docs/it/advisor#fable-advisor-and-usage-credits), accettarlo prima eseguendo `/model fable`. Fino a quando non lo si fa, scegliere Fable in `/advisor` non salva nulla e Claude Code comunica di eseguire prima `/model fable`.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, uno degli alias `"fable"`, `"opus"` o `"sonnet"`, che si risolvono nella versione predefinita corrente di Claude Code di quella famiglia di modelli, oppure un ID modello completo come `"claude-opus-5-5"`
* **Default**: non impostato, quindi l'advisor è disattivato
* **Per-session overrides**: `--advisor` ha la precedenza su questa chiave per una sessione. [`CLAUDE_CODE_DISABLE_ADVISOR_TOOL`](/docs/it/env-vars) disattiva l'advisor e questa chiave non può riattivarlo

```json settings.json theme={null}
{
  "advisorModel": "opus"
}
```

La chiave non ha effetto su provider dove l'advisor [non è disponibile](/docs/it/advisor#requirements), come Amazon Bedrock e Claude Platform su AWS. `"fable"` richiede [accesso a Fable](/docs/it/advisor#choose-an-advisor-model).

<h3 id="alwaysthinkingenabled">
  `alwaysThinkingEnabled`
</h3>

Disattivare il [pensiero esteso](/docs/it/model-config#extended-thinking) per ogni sessione impostando questo su `false`. Il pensiero è attivato per impostazione predefinita, quindi `true` non cambia nulla. La maggior parte delle persone imposta questo tramite `/config` piuttosto che modificando il file.

Su modelli che pensano sempre, come Opus 5.5 e i modelli Fable, `false` non ha effetto. Su [provider di terze parti](/docs/it/third-party-integrations) Claude Code omette il parametro `thinking` invece di disattivare il pensiero, quindi i modelli di ragionamento adattivo potrebbero comunque pensare. Con il pensiero disattivato sull'API Anthropic, Claude Code invia effort `high` invece di un livello superiore ai modelli che sa [non accettano quella combinazione](/docs/it/errors#effort-isnt-available-with-thinking-turned-off), come Opus 5.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: nessun effetto; il pensiero è già attivato
  * `false`: Claude Code disattiva il pensiero esteso per ogni sessione
* **Default**: non impostato, quindi il pensiero è attivato per i modelli che lo supportano
* **Per-session overrides**: [`MAX_THINKING_TOKENS`](/docs/it/env-vars) ha la precedenza su questa chiave per una sessione: `0` disattiva il pensiero, con gli stessi limiti di modello e provider di `false`, e un valore positivo attiva il pensiero anche quando questa chiave è `false`. Su modelli di ragionamento adattivo il numero stesso viene ignorato

```json settings.json theme={null}
{
  "alwaysThinkingEnabled": false
}
```

<h3 id="availablemodels">
  `availableModels`
</h3>

Limitare quali modelli le persone possono selezionare per la sessione principale, [subagenti](/docs/it/sub-agents), [skills](/docs/it/skills) e l'[advisor](/docs/it/advisor). Un elenco gestito vincola `/model`, `--model` e la chiave `model` nei file propri dello sviluppatore; un modello al di fuori di esso non può essere selezionato. Di per sé questo non tocca l'opzione Default; abbinarlo a [`enforceAvailableModels`](#enforceavailablemodels) per quello.

* **Scope**: [`Any file`](#scopes). Distribuirlo nelle impostazioni gestite per applicarlo a un'organizzazione.
* **Type**: array di alias di modelli o ID
* **Default**: non impostato, quindi ogni modello è disponibile

Questo esempio consente alle persone di selezionare solo modelli Sonnet e Haiku:

```json settings.json theme={null}
{
  "availableModels": ["sonnet", "haiku"]
}
```

Vedere [Limitare la selezione del modello](/docs/it/model-config#restrict-model-selection).

<h3 id="effortlevel">
  `effortLevel`
</h3>

Impostare un [livello di effort](/docs/it/model-config#adjust-effort-level) predefinito per i modelli per i quali non è stato salvato un livello. I livelli inferiori sono più veloci e meno costosi su compiti semplici, e i livelli superiori ragionano più profondamente su problemi complessi.

Quando si esegue `/effort low`, `medium`, `high` o `xhigh` in una sessione interattiva sulla propria macchina, Claude Code salva il livello per il modello attivo sotto [`modelSettings`](#modelsettings) piuttosto che scrivere questa chiave. Prima della v2.1.251, `/effort` scriveva questa chiave.

All'interno dello stesso file di impostazioni, Claude Code utilizza il livello salvato di un modello piuttosto che questa chiave. [`modelSettings`](#modelsettings) indica la precedenza tra file.

In una sessione collegata a un worker remoto, in un'esecuzione `-p` e nell'Agent SDK, `/effort` si applica solo a quella sessione. [Regolare il livello di effort](/docs/it/model-config#adjust-effort-level) elenca le scelte interattive che si applicano anche solo a quella sessione. Il messaggio che `/effort` stampa dice quale è accaduto.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, uno di:
  * `"low"`: il ragionamento minimo, per compiti brevi, circoscritti, sensibili alla latenza che non sono sensibili all'intelligenza
  * `"medium"`: riduce l'utilizzo di token per il lavoro sensibile ai costi che può scambiare un po' di intelligenza
  * `"high"`: bilancia l'utilizzo di token e l'intelligenza
  * `"xhigh"`: ragionamento più profondo con spesa di token più elevata
* **Default**: non impostato
* **Per-session overrides**: `--effort` ha la precedenza su questa chiave per una sessione, e [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/it/env-vars) ha la precedenza su entrambi

```json settings.json theme={null}
{
  "effortLevel": "xhigh"
}
```

Nel file di impostazioni utente, `~/.claude/settings.json`, questa chiave è la forma più vecchia che `/effort` scriveva prima di salvare i livelli per modello, e continua ad applicarsi dove si applicava prima, su Opus 5, Fable 5.1 e modelli precedenti. Opus 5.5 e i modelli rilasciati dopo di esso la ignorano e iniziano al loro predefinito fino a quando non si salva un livello per essi, che `/effort` scrive sotto [`modelSettings`](#modelsettings). Nelle impostazioni progetto, locali e gestite, e con `--settings`, questa chiave si applica a ogni modello.

<h3 id="enforceavailablemodels">
  `enforceAvailableModels`
</h3>

Il selettore `/model` ha un'opzione **Default** che si risolve nel [modello predefinito dell'organizzazione](/docs/it/model-config#organization-default-model) quando uno si applica, e altrimenti al predefinito del tipo di account. Un elenco [`availableModels`](#availablemodels) limita i modelli che è possibile nominare, ma di per sé lascia **Default** da solo, quindi **Default** può comunque risolversi in un modello al di fuori dell'elenco. Questa chiave colma quel divario. Richiede Claude Code v2.1.175 o successivo.

Quando l'organizzazione distribuisce impostazioni gestite, Claude Code legge questa chiave solo dalla fonte gestita e la ignora negli altri file.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: quando **Default** si risolverebbe in un modello al di fuori di `availableModels`, Claude Code lo risolve nel primo modello disponibile nell'elenco
  * `false`: **Default** si risolve come al solito, anche in un modello al di fuori di `availableModels`
* **Default**: `false`

Questo esempio limita le selezioni nominate ai modelli Sonnet e Haiku e fa sì che **Default** si risolva nel primo di essi disponibile:

```json settings.json theme={null}
{
  "availableModels": ["sonnet", "haiku"],
  "enforceAvailableModels": true
}
```

Questa chiave non ha effetto quando `availableModels` non è impostato o è vuoto. Vedere [Applicare l'elenco consentito al modello Default](/docs/it/model-config#enforce-the-allowlist-for-the-default-model). Richiede Claude Code v2.1.175 o successivo.

<h3 id="fallbackmodel">
  `fallbackModel`
</h3>

Nominare modelli di backup per Claude Code da provare, in ordine, quando il modello principale è sovraccarico o non disponibile. Claude Code passa al modello disponibile successivo nella catena per il resto del turno e mostra un avviso. Senza una catena, Claude Code ritenta lo stesso modello e quindi visualizza l'errore del server, e si ritenta o si cambiano i modelli manualmente.

Un cambio significa un turno con una [prompt cache](/docs/it/prompt-caching#switching-models) fredda sul modello di fallback; il messaggio successivo ritenta il modello principale per primo.

* **Scope**: [`Any file`](#scopes)
* **Type**: array di alias di modelli o ID; `"default"` si espande al modello predefinito
* **Default**: non impostato, quindi una richiesta non riuscita non viene ritentata su un altro modello
* **Per-session overrides**: `--fallback-model` ha la precedenza su questa chiave per una sessione

Questo esempio prova Sonnet 5 per primo, poi Haiku 4.5, quando il modello principale non riesce:

```json settings.json theme={null}
{
  "fallbackModel": ["claude-sonnet-5", "claude-haiku-4-5"]
}
```

A differenza della maggior parte delle impostazioni di array, questa chiave non si unisce tra file di impostazioni: il file con la precedenza più alta che la definisce fornisce l'intera catena. Se il file del progetto imposta `["claude-sonnet-5"]` e il file dell'utente imposta `["claude-haiku-4-5"]`, la catena è solo `["claude-sonnet-5"]`. Claude Code mantiene al massimo tre modelli consentiti distinti dall'elenco e ignora il resto. Vedere [Catene di modelli di fallback](/docs/it/model-config#fallback-model-chains).

<h3 id="fastmode">
  `fastMode`
</h3>

Attivare la [modalità veloce](/docs/it/fast-mode) per sessioni dove è disponibile, per il lavoro interattivo come l'iterazione rapida o il debug dal vivo dove si desidera velocità a un costo più elevato per token. Di solito non si modifica questa chiave manualmente: eseguire `/fast` scrive `fastMode: true` in `~/.claude/settings.json`, e eseguirlo di nuovo per disattivare la modalità veloce rimuove la chiave. La modalità veloce funziona solo su Opus 5.5, Opus 5 e Opus 4.8: attivarla da un altro modello passa a Opus, e passare a un modello non supportato la disattiva. Vedere [Cambiare modelli mentre la modalità veloce è attiva](/docs/it/fast-mode#switch-models-while-fast-mode-is-on).

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code attiva la modalità veloce per sessioni dove è disponibile
  * `false`: la modalità veloce rimane disattivata
* **Default**: non impostato, quindi la modalità veloce è disattivata
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_FAST_MODE`](/docs/it/env-vars) disattiva la modalità veloce per una sessione, e questa chiave non può riattivarlo

```json settings.json theme={null}
{
  "fastMode": true
}
```

<h3 id="fastmodepersessionoptin">
  `fastModePerSessionOptIn`
</h3>

Normalmente, eseguire `/fast` salva [`fastMode`](#fastmode) nelle impostazioni utente di una persona, quindi la modalità veloce è attiva all'inizio di ogni sessione successiva. Impostare questa chiave su `true` per fermare questo: un `fastMode: true` salvato non attiva più la modalità veloce all'inizio della sessione, e ogni persona deve eseguire `/fast` in ogni sessione in cui la desidera. Claude Code lascia la chiave `fastMode` nel file, quindi disattivare questa chiave ripristina il comportamento precedente.

I proprietari su piani Team o Enterprise possono distribuirlo a livello di organizzazione tramite [impostazioni gestite dal server](/docs/it/server-managed-settings). Quando le impostazioni gestite impostano la chiave, `/fast on` viene rifiutato al di fuori delle sessioni di terminale interattive e segnala che l'organizzazione ha disabilitato la modalità veloce. Questo copre la [modalità non interattiva](/docs/it/headless), l'[estensione VS Code](/docs/it/vs-code) e le [sessioni cloud](/docs/it/claude-code-on-the-web).

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: un `fastMode: true` salvato non attiva più la modalità veloce all'inizio della sessione, quindi ogni persona esegue `/fast` in ogni sessione in cui la desidera; un `fastMode: true` passato con `--settings` conta comunque per quella sessione a meno che le impostazioni gestite non impostino questa chiave
  * `false`: un `fastMode: true` salvato attiva la modalità veloce all'inizio di ogni sessione successiva
* **Default**: `false`

```json settings.json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

Vedere [Richiedere il consenso per sessione](/docs/it/fast-mode#require-per-session-opt-in).

<h3 id="language">
  `language`
</h3>

Fare in modo che Claude risponda in una lingua diversa dall'inglese per impostazione predefinita. Non esiste un elenco fisso per le risposte: Claude Code aggiunge il valore verbatim al prompt di sistema come istruzione per rispondere sempre in quella lingua, quindi qualsiasi nome di lingua che Claude può leggere funziona. Claude Code non controlla il valore, quindi un nome scritto male raggiunge Claude così come scritto piuttosto che produrre un errore. Lo stesso valore imposta la lingua per la [dettatura vocale](/docs/it/voice-dictation#change-the-dictation-language), che ha un elenco fisso di [lingue di dettatura supportate](/docs/it/voice-dictation#change-the-dictation-language), e per i titoli di sessione generati automaticamente.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, qualsiasi nome di lingua, come `"japanese"`, `"spanish"` o `"french"`; Claude Code non lo convalida
* **Default**: non impostato; i titoli di sessione corrispondono quindi alla lingua della conversazione

```json settings.json theme={null}
{
  "language": "japanese"
}
```

<h3 id="maxeffortlevel">
  `maxEffortLevel`
</h3>

Limitare il [livello di effort](/docs/it/model-config#adjust-effort-level) che una sessione può utilizzare, lasciando disponibili i livelli inferiori. Qualsiasi livello superiore funziona al limite, incluso uno da `/effort`, il selettore `/model`, `--effort`, [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/it/env-vars), il frontmatter `effort` di una skill o subagente, o il predefinito del modello stesso. Claude Code applica il limite stesso prima di ogni richiesta, quindi vale su ogni provider, inclusi Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry. Richiede Claude Code v2.1.267 o successivo.

* **Scope**: [`Any file`](#scopes). Distribuirlo nelle impostazioni gestite per applicarlo a un'organizzazione. Quando più scope impostano un limite, si applica il più basso, quindi un limite impostato in uno scope non può essere aumentato da un altro
* **Type**: string, uno di `"low"`, `"medium"`, `"high"`, `"xhigh"` o `"max"`. Un valore `"max"` non imposta alcun limite
* **Default**: non impostato, quindi non si applica alcun limite
* **Effect on ultracode**: un limite inferiore a `xhigh` rende [ultracode](#ultracode) non disponibile sui modelli a cui si applica il limite
* **Per-model caps**: aggiungere `maxEffortLevel` alla voce [`modelSettings`](#modelsettings) di un modello. Quella voce sostituisce questa chiave solo per il modello all'interno della fonte di impostazioni che imposta entrambi, come le impostazioni utente o una [fonte gestita](/docs/it/managed-settings#how-claude-code-combines-managed-sources). Impostare `"max"` lì per esentare il modello dal limite di quella fonte; Claude Code applica comunque i limiti da altre fonti

Questo esempio limita ogni modello a `medium` ed esenenta Sonnet 4.6:

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

Quando l'organizzazione imposta anche un [limite di effort](/docs/it/model-config#organization-effort-limits) per un modello, si applica il limite inferiore dei due.

<h3 id="model">
  `model`
</h3>

Impostare il modello che ogni nuova sessione utilizza, quindi non è necessario sceglierne uno con `/model` ogni volta. Impostarlo qui non impedisce di cambiare modello a metà sessione. Se l'amministratore ha impostato un [modello predefinito dell'organizzazione](/docs/it/model-config#organization-default-model) per ignorare la selezione dell'utente, si ottiene quel modello anche quando si imposta questa chiave nelle impostazioni utente, progetto o locali.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, un alias di modello o ID modello completo
* **Default**: non impostato, quindi Claude Code utilizza il modello predefinito dell'account
* **Per-session overrides**: `--model` ha la precedenza su [`ANTHROPIC_MODEL`](/docs/it/env-vars), e entrambi hanno la precedenza su questa chiave per una sessione, incluso su un `model` gestito; un elenco [`availableModels`](#availablemodels) si applica comunque alla scelta

```json settings.json theme={null}
{
  "model": "claude-sonnet-5"
}
```

Un valore qui supera [`ANTHROPIC_DEFAULT_MODEL`](/docs/it/model-config#set-a-default-model-for-new-sessions), che Claude Code utilizza solo quando nient'altro seleziona un modello.

<h3 id="modeloverrides">
  `modelOverrides`
</h3>

Mappare gli ID dei modelli Anthropic agli ID dei modelli specifici del provider, come gli ARN del profilo di inferenza di Amazon Bedrock. Ogni voce del selettore di modelli utilizza quindi il suo valore mappato quando chiama l'API del provider. Gli amministratori lo utilizzano su [Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry](/docs/it/model-config#override-model-ids-per-version) per instradare ogni versione del modello a un profilo di inferenza specifico, nome di versione o distribuzione per governance, allocazione dei costi o instradamento regionale.

* **Scope**: [`Any file`](#scopes)
* **Type**: object che mappa l'ID del modello all'ID del modello del provider
* **Default**: non impostato

Questo esempio instrada ogni chiamata per Opus 4.6 al profilo di inferenza Bedrock denominato:

```json settings.json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-6": "arn:aws:bedrock:us-east-1:123456789012:inference-profile/example"
  }
}
```

Vedere [Ignorare gli ID dei modelli per versione](/docs/it/model-config#override-model-ids-per-version).

<h3 id="modelpicker">
  `modelPicker`
</h3>

Elencare i modelli che il selettore `/model` offre, nell'ordine in cui li si scrive e sotto le etichette che si scelgono, quindi il selettore elenca i modelli che l'organizzazione esegue, dopo la lineup integrata o al suo posto. Il `model` di ogni riga viene preso verbatim, quindi accetta qualsiasi cosa accetti `--model`: un alias come `opus`, un ID modello Anthropic, o un ID in formato provider per Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, o un gateway LLM. Richiede Claude Code v2.1.242 o successivo.

* **Scope**: [`User or managed`](#scopes). Claude Code legge la chiave dalle impostazioni gestite, `--settings` e impostazioni utente, e la ignora nelle impostazioni progetto e locali quindi un repository che si clona non può rietichettare il selettore. Il più alto dei tre che imposta la chiave fornisce l'intera lineup, e Claude Code non combina mai lineup da due fonti.
* **Type**: object con un array `options` di righe e un Boolean `replaceBuiltInOptions` opzionale
* **Default**: non impostato, quindi il selettore mostra la lineup integrata

Questo esempio aggiunge due distribuzioni Bedrock dopo la lineup integrata, sotto nomi che il team riconosce:

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
  Campi per `modelPicker`
</h4>

La chiave accetta due campi, uno per le righe stesse e uno per se sostituiscono la lineup integrata o la aggiungono.

| Field                   | Type                                                                                      | What it does                                                                                                                                                                                                                                                                                       |
| :---------------------- | :---------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options`               | array di righe, ognuna con un `model` obbligatorio e un `label` e `description` opzionali | Le righe che il selettore mostra, in questo ordine, tranne che una riga disattivata si sposta in fondo. Senza un `label`, Claude Code intitola la riga con il nome integrato per un modello che conosce, o l'ID del modello altrimenti, e senza una `description` scrive una seconda riga generica |
| `replaceBuiltInOptions` | Boolean, default `false`                                                                  | Impostarlo su `true` per mostrare solo queste righe, **Default** e una riga per il modello che la sessione sta già utilizzando. Lasciarlo non impostato per aggiungere queste righe dopo la lineup integrata                                                                                       |

Con `replaceBuiltInOptions` attivato, Claude Code nasconde ogni altra riga: la lineup integrata, le righe che aggiunge per le voci [`availableModels`](#availablemodels), i modelli che la [scoperta del gateway](/docs/it/llm-gateway-protocol#model-discovery) ha trovato, e [`ANTHROPIC_CUSTOM_MODEL_OPTION`](/docs/it/model-config#add-a-custom-model-option). Con esso disattivato, Claude Code salta un modello elencato che la lineup integrata copre già. Un'etichetta cambia ciò che il selettore mostra, non quale modello Claude Code esegue.

Un elenco [`availableModels`](#availablemodels) si applica comunque a queste righe. Prima di aggiungere un modello elencato all'elenco consentito, leggere [Comportamento di unione](/docs/it/model-config#merge-behavior): un ID modello specifico restringe la voce wildcard della sua famiglia. Claude Code controlla anche ogni riga rispetto alla sessione prima di mostrare il selettore:

* **Dropped**: una riga che Claude Code non può servire, come un modello ritirato o un modello a cui l'organizzazione non ha accesso
* **Grayed out**: una riga che non è possibile selezionare ancora, mostrata con il motivo
* **No row survives**: Claude Code mantiene la lineup integrata, filtrata dall'elenco consentito come al solito

Claude Code elimina una riga che non può analizzare e mantiene il resto. Vedere [Correggere un file di impostazioni rotto](/docs/it/settings#fix-a-broken-settings-file).

<h3 id="modelpricing">
  `modelPricing`
</h3>

Segnalare la spesa alle tariffe che l'organizzazione paga invece del prezzo di listino. Impostarlo quando l'organizzazione ha tariffe contrattuali, quindi le cifre in dollari che gli sviluppatori vedono corrispondono alla fattura. Claude Code applica le tariffe in `/usage`, la [riga di stato](/docs/it/statusline), l'`total_cost_usd` dell'Agent SDK, il limite [`--max-budget-usd`](/docs/it/cli-reference) e la metrica di costo [OpenTelemetry](/docs/it/monitoring-usage) e gli eventi. Si forniscono le tariffe: Claude Code non le legge dal contratto o dalla Claude Console. Richiede Claude Code v2.1.242 o successivo.

* **Scope**: [`Managed`](#scopes). Distribuire la chiave tramite impostazioni gestite dal server, una politica MDM, un file `managed-settings.json` o un [helper di politica](/docs/it/managed-settings#compute-the-policy-with-a-helper-program). Claude Code la ignora nelle impostazioni utente, progetto e locali, in `--settings` e su Windows nel [registro HKCU](/docs/it/managed-settings#where-each-mechanism-stores-the-policy) scrivibile dall'utente. Con impostazioni gestite dal server, ogni sessione segnala i costi al prezzo di listino fino a quando il [fetch delle impostazioni](/docs/it/server-managed-settings#fetch-and-caching-behavior) di quella sessione non ha confermato l'impostazione. Un'applicazione host che incorpora Claude Code e imposta [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/it/env-vars) può fornire una tabella propria tramite l'opzione SDK [`managedSettings`](/docs/it/agent-sdk/typescript#options), che Claude Code utilizza solo quando nessuna fonte gestita imposta la chiave e solo in Claude Code v2.1.246 o successivo.
* **Type**: object con un `multiplier` opzionale e una mappa `overrides` opzionale
* **Default**: non impostato, quindi Claude Code segnala il prezzo di listino a meno che un'applicazione host non fornisca una tabella

Impostare `multiplier` da solo per uno sconto fisso o un ricarico, `overrides` da solo per tariffe per modello, o entrambi.

Questo esempio imposta tariffe contrattuali per Sonnet 4.6 e quindi riduce ogni cifra, la riga Sonnet inclusa, del 15%:

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

Impostare `multiplier` sopra 1, fino a 10, per contrassegnare ogni cifra. Un ricarico richiede Claude Code v2.1.271 o successivo. Le versioni precedenti ignorano un `multiplier` sopra 1 con un avviso e mantengono il resto dell'impostazione.

Per i passaggi, incluso come confermare che le tariffe sono in vigore, vedere [Segnalare la spesa alle tariffe contrattuali](/docs/it/costs#report-spend-at-your-contracted-rates).

<span id="modelpricing-multiplier" />

<span id="modelpricing-overrides" />

<h4 id="fields-for-modelpricing">
  Campi per `modelPricing`
</h4>

| Field        | Type                                                                                                               | What it does                                                                                                                                                                                                                                                         |
| :----------- | :----------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `multiplier` | numero maggiore di 0 e al massimo 10                                                                               | Scala ogni costo che Claude Code calcola, indipendentemente dal fatto che una riga `overrides` lo copra. Sotto 1 è uno sconto, sopra 1 un ricarico                                                                                                                   |
| `overrides`  | mappa di ID modello a un oggetto di tariffa con `input`, `output`, `cacheRead` e `cacheWrite`, ognuno da 0 a 10000 | Le tariffe USD-per-milione-token per quel modello, tutti e quattro obbligatori. `cacheWrite` copre sia le scritture della cache di cinque minuti che di un'ora. Vedere [Quali modelli si applica una riga modelPricing](#which-models-a-modelpricing-row-applies-to) |

Claude Code utilizza le tariffe di una riga esattamente come le si è scritte, senza aggiungere il supplemento della modalità veloce o la [tariffa di inferenza solo negli Stati Uniti](https://platform.claude.com/docs/en/about-claude/pricing). Se si imposta anche `multiplier`, Claude Code lo applica in aggiunta alle tariffe della riga. Claude Code elimina una riga con una tariffa che non può analizzare, o un `multiplier` che non può analizzare, e mantiene il resto; vedere [Correggere un file di impostazioni rotto](/docs/it/settings#fix-a-broken-settings-file).

<h4 id="which-models-a-modelpricing-row-applies-to">
  Quali modelli si applica una riga `modelPricing`
</h4>

Claude Code decide quali modelli si applica una riga dalla chiave della riga:

* **L'ID di un modello integrato**: una chiave che Claude Code stesso utilizza per un modello integrato, indipendentemente dal fatto che quella chiave sia l'ID del modello stesso, come `claude-sonnet-4-6`, o il suo ID Bedrock, Agent Platform o Foundry. Claude Code applica la riga a ogni ID snapshot datato e ID specifico del provider di quel modello.
* **Qualsiasi altra chiave**: una chiave che non è l'ID di un modello integrato, come un alias di modello gateway. Claude Code applica la riga solo a quell'ID. Quando un ID modello corrisponde esattamente a una delle chiavi e rientra anche in una riga con chiave dall'ID di un modello integrato, Claude Code utilizza la corrispondenza esatta.
* **Un profilo di inferenza dell'applicazione Bedrock**: una volta che Claude Code ha risolto il profilo al modello a cui instrada, tramite la mappa [`modelOverrides`](#modeloverrides) o la ricerca [`bedrock:GetInferenceProfile`](/docs/it/amazon-bedrock#iam-configuration), Claude Code applica la riga di quel modello al profilo.

<h3 id="modelsettings">
  `modelSettings`
</h3>

Salvare un [livello di effort](/docs/it/model-config#adjust-effort-level) per ogni modello che si utilizza. Richiede Claude Code v2.1.251 o successivo.

In una sessione interattiva sulla propria macchina, quando si salva `low`, `medium`, `high` o `xhigh` come predefinito con `/effort` o il cursore di effort del selettore `/model`, Claude Code scrive quel livello qui sotto il modello che si sta utilizzando, quindi raramente si modifica questa chiave manualmente. Quando si sceglie uno di questi livelli nel [selettore di modelli dell'estensione VS Code](/docs/it/vs-code#use-the-prompt-box), Claude Code lo salva qui allo stesso modo. La voce [`effortLevel`](#effortlevel) elenca le sessioni dove `/effort` si applica solo a quella sessione.

Modificare la chiave manualmente per cambiare o rimuovere un livello salvato.

Un `effortLevel` di un modello qui ha la precedenza sul [`effortLevel`](#effortlevel) di livello superiore nello stesso file di impostazioni. Tra file, Claude Code risolve ogni modello separatamente: il [file di impostazioni](/docs/it/settings#settings-precedence) con la precedenza più alta che imposta un `effortLevel` per quel modello o il `effortLevel` di livello superiore che [si applica a quel modello](#effortlevel) decide, quindi un `effortLevel` nelle impostazioni gestite supera un livello salvato nelle impostazioni utente. [Regolare il livello di effort](/docs/it/model-config#adjust-effort-level) elenca cos'altro può ignorare un livello salvato, come `--effort` al lancio.

Per limitare l'effort di un modello piuttosto che impostare il suo livello, aggiungere un campo [`maxEffortLevel`](#maxeffortlevel) alla voce di quel modello. Il campo richiede Claude Code v2.1.267 o successivo.

* **Scope**: [`Any file`](#scopes)
* **Type**: object che mappa un nome di modello a un object con un campo `effortLevel`, uno di `"low"`, `"medium"`, `"high"` o `"xhigh"`, un campo [`maxEffortLevel`](#maxeffortlevel) o entrambi
* **Default**: non impostato

Claude Code scrive ogni voce sotto il nome canonico del modello, come `claude-opus-5-5`, e corrisponde all'alias di quel modello, con suffisso di data, `[1m]` e ID specifici del provider riconosciuti alla stessa voce.

Questo esempio mantiene Opus 5.5 a `high` mentre altri modelli utilizzano i loro livelli salvati o predefiniti:

```json settings.json theme={null}
{
  "modelSettings": {
    "claude-opus-5-5": {
      "effortLevel": "high"
    }
  }
}
```

Eseguire `/effort auto` per cancellare il livello salvato per il modello che si sta utilizzando. Claude Code lascia le altre voci e qualsiasi `effortLevel` di livello superiore in vigore.

<h3 id="outputstyle">
  `outputStyle`
</h3>

Selezionare uno [stile di output](/docs/it/output-styles) per nome. Uno stile di output è un insieme salvato di istruzioni che cambia il ruolo, il tono e il formato di output di Claude, come gli stili Explanatory e Learning integrati o uno che si è scritto.

Se si cambia questa chiave durante una sessione, Claude utilizza il nuovo stile a partire dal messaggio successivo. Per il costo di quella cache di prompt, vedere [Cambiare lo stile di output](/docs/it/prompt-caching#changing-output-style). Prima della v2.1.251, la modifica si applicava solo dopo aver eseguito `/clear` o avviato una nuova sessione.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, il nome di uno stile di output [integrato](/docs/it/output-styles#built-in-output-styles) o [personalizzato](/docs/it/output-styles#create-a-custom-output-style)
* **Default**: non impostato, quindi Claude Code utilizza lo stile predefinito

Questo esempio seleziona lo stile Explanatory integrato, che aggiunge approfondimenti educativi tra i compiti:

```json settings.json theme={null}
{
  "outputStyle": "Explanatory"
}
```

<h3 id="promptcachettl">
  `promptCacheTtl`
</h3>

Scegliere quanto tempo la [prompt cache](/docs/it/prompt-caching) mantiene la conversazione principale. Questa chiave si applica ai turni interattivi, `-p` e Agent SDK, insieme agli helper che Claude Code esegue inline con essi. La durata di un'ora mantiene la cache calda durante pause più lunghe, e l'API [fattura ogni scrittura della cache a una tariffa più elevata](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing) rispetto alla durata di cinque minuti. Richiede Claude Code v2.1.242 o successivo.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, uno di:
  * `"5m"`: la cache si mantiene per cinque minuti
  * `"1h"`: la cache si mantiene per un'ora
* **Default**: non impostato, quindi ogni richiesta di conversazione principale ottiene la [durata predefinita](/docs/it/prompt-caching#which-ttl-each-request-gets)
* **Per-session overrides**: [`FORCE_PROMPT_CACHING_5M`](/docs/it/env-vars) ha la precedenza su tutto il resto, poi [`CLAUDE_CODE_PROMPT_CACHE_TTL`](/docs/it/env-vars), poi questa chiave, e infine [`ENABLE_PROMPT_CACHING_1H`](/docs/it/env-vars)

Questo esempio mantiene la conversazione principale sulla durata di un'ora e lascia i subagenti su cinque minuti:

```json settings.json theme={null}
{
  "promptCacheTtl": "1h",
  "subagentPromptCacheTtl": "5m"
}
```

Per il costo di ogni durata, vedere [Durata della cache](/docs/it/prompt-caching#cache-lifetime).

<h3 id="showthinkingsummaries">
  `showThinkingSummaries`
</h3>

Vedere i riassunti del [pensiero esteso](/docs/it/model-config#extended-thinking) di Claude nelle sessioni interattive. Impostarlo se si desidera i riassunti completi quando si espande il pensiero con `Ctrl+O`. Quando non impostato o `false`, l'API Anthropic redige i blocchi di pensiero e Claude Code mostra uno stub compresso; i provider di terze parti non redigono.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: si vedono i riassunti completi del pensiero quando si espande il pensiero con `Ctrl+O`
  * `false`: l'API Anthropic redige i blocchi di pensiero e Claude Code mostra uno stub compresso
* **Default**: `false`

```json settings.json theme={null}
{
  "showThinkingSummaries": true
}
```

La redazione cambia solo ciò che si vede, non ciò che il modello genera. Per ridurre la spesa di pensiero, [abbassare il budget o disattivare il pensiero](/docs/it/model-config#extended-thinking) invece.

<h3 id="subagentpromptcachettl">
  `subagentPromptCacheTtl`
</h3>

Scegliere quanto tempo la [prompt cache](/docs/it/prompt-caching) mantiene le richieste che Claude Code effettua al di fuori della conversazione principale. Questa chiave si applica a [subagenti](/docs/it/sub-agents), [workflows](/docs/it/workflows) e le richieste di background e helper proprie di Claude Code, come la compattazione e i titoli di sessione. La durata di un'ora mantiene la cache calda durante pause più lunghe, e l'API [fattura ogni scrittura della cache a una tariffa più elevata](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing) rispetto alla durata di cinque minuti. Richiede Claude Code v2.1.242 o successivo.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, uno di:
  * `"5m"`: la cache si mantiene per cinque minuti
  * `"1h"`: la cache si mantiene per un'ora
* **Default**: non impostato, quindi ognuna di queste richieste ottiene la [durata predefinita](/docs/it/prompt-caching#which-ttl-each-request-gets)
* **Per-session overrides**: [`FORCE_PROMPT_CACHING_5M`](/docs/it/env-vars) ha la precedenza su tutto il resto, poi [`CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`](/docs/it/env-vars), poi questa chiave, poi [`ENABLE_PROMPT_CACHING_1H`](/docs/it/env-vars), che chiede la durata di un'ora su ogni richiesta. Per dove il valore del frontmatter proprio di un subagente si classifica, vedere [Scegliere il TTL da soli](/docs/it/prompt-caching#choose-the-ttl-yourself)

Questo esempio fornisce ai subagenti e alle altre richieste al di fuori della conversazione principale la durata di un'ora:

```json settings.json theme={null}
{
  "subagentPromptCacheTtl": "1h"
}
```

Questa chiave copre le richieste che [`promptCacheTtl`](#promptcachettl) non copre, quindi impostare entrambi per scegliere una durata per ogni richiesta che Claude Code effettua. Per come la cache di un subagente differisce da quella della conversazione principale, vedere [Subagenti e la cache](/docs/it/prompt-caching#subagents-and-the-cache).

<h3 id="switchmodelsonflag">
  `switchModelsOnFlag`
</h3>

Scegliere cosa accade quando un [classificatore di sicurezza contrassegna una richiesta](/docs/it/model-config#automatic-model-fallback): passare al modello di fallback e continuare, o mettere in pausa in modo da poter scegliere tra passare e modificare il prompt.

* **Scope**: [`Any file`](#scopes). Appare in `/config` come **Switch models when a message is flagged**.
* **Type**: Boolean
  * `true`: Claude Code passa al modello di fallback e continua
  * `false`: in una sessione interattiva Claude Code mette in pausa in modo da poter scegliere tra passare e modificare il prompt; dove nessuna finestra di dialogo può mostrare, come un'esecuzione `-p`, la richiesta contrassegnata termina come errore
* **Default**: `true`, passa automaticamente

```json settings.json theme={null}
{
  "switchModelsOnFlag": false
}
```

Vedere [Chiedere prima di passare](/docs/it/model-config#ask-before-switching).

<h3 id="ultracode">
  `ultracode`
</h3>

Avviare sessioni con [ultracode](/docs/it/workflows#let-claude-decide-with-ultracode) attivato. Con esso attivato, Claude pianifica un workflow per ogni compito sostanziale invece di aspettare che lo si chieda. Claude pianifica workflow solo quando i [workflow dinamici](/docs/it/workflows) sono abilitati per l'utente, il modello supporta `xhigh` effort, e nessun [limite di effort](/docs/it/model-config#organization-effort-limits) inferiore a `xhigh` si applica. In ogni caso, `ultracode: true` esegue la sessione a `xhigh` effort, o al limite quando un limite di effort è inferiore. Claude Code legge questa chiave ma non la scrive mai: `/effort ultracode` attiva ultracode solo per la sessione corrente.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: le sessioni iniziano a `xhigh` effort, con ultracode attivato quando i workflow dinamici sono abilitati per l'utente, il modello supporta `xhigh` e nessun limite di effort è inferiore a `xhigh`
  * `false`: le sessioni iniziano con ultracode disattivato
* **Default**: non impostato, quindi ultracode è disattivato
* **Per-session overrides**: `/effort ultracode` attiva ultracode per una sessione senza questa chiave. Lo fa anche `--effort ultracode`, che richiede Claude Code v2.1.203 o successivo

```json settings.json theme={null}
{
  "ultracode": true
}
```

Ultracode esegue la sessione a `xhigh` effort e ha la precedenza su `effortLevel` e voci [`modelSettings`](#modelsettings). Se un [limite di effort](/docs/it/model-config#organization-effort-limits) inferiore a `xhigh` si applica al modello, come un'impostazione [`maxEffortLevel`](#maxeffortlevel), la sessione funziona al limite e ultracode rimane disattivato. Claude quindi non pianifica workflow da solo, e `/effort` non offre `ultracode`. Una richiesta di controllo `apply_flag_settings` dell'Agent SDK accetta anche la chiave.

<h2 id="permission-settings">
  Impostazioni di autorizzazione
</h2>

Decidi cosa Claude può fare senza chiedere, quale modalità di autorizzazione una sessione inizia e cosa consente il classificatore della modalità automatica. Per la sintassi delle regole e il modello di autorizzazione, vedere [Configurare le autorizzazioni](/docs/it/permissions).

<h3 id="allowmanagedpermissionrulesonly">
  `allowManagedPermissionRulesOnly`
</h3>

Rendi le impostazioni gestite l'unica fonte di impostazioni delle regole di autorizzazione. Claude Code ignora quindi le regole `allow`, `ask` e `deny` nei file utente, progetto, locale e `--settings`, ignora `--allowedTools`, nasconde le scelte sempre-consenti nei prompt di autorizzazione e smette di salvare nuove regole.

Quando [le impostazioni padre da un host di incorporamento](/docs/it/managed-settings#let-an-embedding-host-add-policy) si applicano, Claude Code le tratta come parte del livello gestito. Scarta le loro regole `allow` e `additionalDirectories` e mantiene le loro regole `deny` e `ask` eccetto le regole `Read` e `Edit` il cui modello inizia con `!`. Un host non può ricavare percorsi dalle regole gestite con una regola `!`, indipendentemente dal fatto che tu imposti questa chiave.

Le regole `--disallowedTools` e le regole `deny` e `ask` della sessione corrente si applicano ancora, anche dopo che Claude Code ricarica le impostazioni a metà sessione. Poiché solo limitano, non possono ampliare ciò che le regole gestite concedono. Prima della v2.1.257, Claude Code scartava quelle regole da riga di comando e di sessione al primo ricaricamento delle impostazioni.

Per cosa un modello `!` in una regola `--disallowedTools` o di sessione può ricavare, vedere [Regole Read e Edit](/docs/it/permissions#read-and-edit).

* **Ambito**: [`Managed`](#scopes)
* **Tipo**: Booleano
  * `true`: le impostazioni gestite diventano l'unica fonte di impostazioni delle regole di autorizzazione
  * `false`: Claude Code applica le regole di autorizzazione dai file utente, progetto, locale e `--settings` oltre a quelle gestite
* **Predefinito**: non impostato, quindi Claude Code applica le regole di autorizzazione dalle impostazioni utente, progetto e locale e da `--settings`, oltre a quelle gestite

```json managed-settings.json theme={null}
{
  "allowManagedPermissionRulesOnly": true
}
```

Questa chiave non blocca l'elenco di autorizzazione del server MCP; per farlo, imposta [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly). Vedere [Impostazioni solo gestite](/docs/it/managed-settings#managed-only-settings).

<h3 id="automode">
  `autoMode`
</h3>

Aggiungi le tue regole a ciò che il classificatore della [modalità automatica](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) blocca e consente. Usalo per dire al classificatore quali repository, bucket e domini la tua organizzazione ritiene affidabili, in modo che smetta di bloccare le operazioni interne di routine. Il classificatore viene fornito con [regole di autorizzazione e negazione integrate](/docs/it/auto-mode-config#inspect-the-defaults-and-your-effective-config). Includi la stringa letterale `"$defaults"` in un array per mantenere quelle regole integrate in quella posizione e aggiungere le tue intorno; omettila per sostituirle con le tue.

* **Ambito**: [`User or managed`](#scopes)
* **Tipo**: oggetto con array `environment`, `allow`, `soft_deny` e `hard_deny` di regole in prosa, più il Booleano [`classifyAllShell`](#automode-classifyallshell)
* **Predefinito**: non impostato, quindi il classificatore utilizza solo le sue [regole integrate](/docs/it/auto-mode-config#inspect-the-defaults-and-your-effective-config)

Questo esempio mantiene le regole `soft_deny` integrate, tramite `"$defaults"`, e aggiunge un'altra che blocca `terraform apply`:

```json settings.json theme={null}
{
  "autoMode": {
    "soft_deny": ["$defaults", "Never run terraform apply"]
  }
}
```

Quando più di uno di questi file imposta lo stesso array, Claude Code concatena le voci. Per il formato della regola e come ogni array viene applicato, vedere [Configurare la modalità automatica](/docs/it/auto-mode-config).

<h3 id="automode-classifyallshell">
  `autoMode.classifyAllShell`
</h3>

Invia ogni comando Bash e PowerShell attraverso il classificatore della modalità automatica mentre la modalità automatica è attiva. Per impostazione predefinita, la modalità automatica sospende solo le regole di autorizzazione che potrebbero eseguire codice arbitrario: regole a livello di strumento e wildcard come `Bash(*)` e prefissi di interprete o shell-wrapper come `Bash(python *)`. Un comando che corrisponde a qualsiasi altra regola di autorizzazione, come `Bash(npm test)`, salta il classificatore a meno che non porti [domini consentiti per comando](/docs/it/sandboxing#per-command-allowed-domains-in-auto-mode) e un argomento distruttivo che il prefisso della regola non ha anticipato può passare inosservato. L'impostazione di questa chiave sospende ogni regola di autorizzazione shell per la sessione in modo che il classificatore veda ogni comando. Richiede Claude Code v2.1.193 o successivo.

* **Ambito**: [`User or managed`](#scopes). Leggi ovunque [`autoMode`](#automode) viene letto.
* **Tipo**: Booleano
  * `true`: mentre la modalità automatica è attiva, Claude Code invia ogni comando Bash e PowerShell attraverso il classificatore e sospende le tue regole di autorizzazione shell; al di fuori della modalità automatica le regole si applicano ancora
  * `false`: la modalità automatica sospende solo le regole di autorizzazione che potrebbero eseguire codice arbitrario, come `Bash(*)` e `Bash(python *)`; un comando che corrisponde a qualsiasi altra regola di autorizzazione salta il classificatore a meno che non porti [domini consentiti per comando](/docs/it/sandboxing#per-command-allowed-domains-in-auto-mode) e ogni altro comando shell passa attraverso di esso
* **Predefinito**: `false`

```json settings.json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

Vedere [Instrada tutti i comandi shell attraverso il classificatore](/docs/it/auto-mode-config#route-all-shell-commands-through-the-classifier). Richiede Claude Code v2.1.193 o successivo.

<h3 id="disableautomode">
  `disableAutoMode`
</h3>

Rimuovi la [modalità automatica](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) dal ciclo `Shift+Tab`. Qualsiasi sessione che altrimenti [inizierebbe in modalità automatica](/docs/it/permission-modes#which-mode-a-session-starts-in), sia da `--permission-mode auto`, da un file di impostazioni o dal predefinito integrato, inizia invece in `default`. Gli amministratori lo impostano nelle impostazioni gestite per impedire agli sviluppatori della loro organizzazione di utilizzare la modalità automatica.

* **Ambito**: [`Any file`](#scopes). Più utile nelle [impostazioni gestite](/docs/it/managed-settings), dove gli utenti non possono sovrascriverlo. Accettato anche sotto `permissions` come `permissions.disableAutoMode`.
* **Tipo**: la stringa `"disable"`
* **Predefinito**: non impostato

```json settings.json theme={null}
{
  "disableAutoMode": "disable"
}
```

<h3 id="permissions">
  `permissions`
</h3>

Controlla quali strumenti Claude può utilizzare senza chiedere, quali richiedono sempre un prompt e quali sono bloccati, e imposta la [modalità di autorizzazione](/docs/it/permission-modes) in cui una sessione inizia. Ogni chiave `permissions.*` di seguito si annida sotto questo oggetto.

* **Ambito**: [`Any file`](#scopes)
* **Tipo**: oggetto con `allow`, `ask`, `deny`, `additionalDirectories`, `blockReadsOutsideWorkingDirectories`, `defaultMode`, `disableBypassPermissionsMode` e `disableAutoMode`
* **Predefinito**: non impostato

Questo esempio approva i comandi `npm run` senza chiedere, richiede una conferma prima di `git push`, blocca le letture di `.env` e avvia le sessioni in `acceptEdits`:

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

I tre array di regole condividono una sintassi; vedere [Sintassi della regola di autorizzazione](#permission-rule-syntax) sotto `permissions.allow`. Per come le regole di autorizzazione da file diversi si combinano, vedere [come le regole di autorizzazione si uniscono tra gli ambiti](/docs/it/permissions#settings-precedence); per come le chiavi di impostazioni in generale si combinano, vedere [Precedenza delle impostazioni](/docs/it/settings#settings-precedence) nella guida alle impostazioni.

<h3 id="useautomodeduringplan">
  `useAutoModeDuringPlan`
</h3>

Scegli se Claude Code utilizza il classificatore della modalità automatica per esaminare i comandi shell in modalità piano. Con il valore predefinito `true`, il classificatore esamina ogni comando durante la pianificazione quando la modalità automatica è disponibile e non vedi alcun prompt. Imposta `false` per ottenere un prompt di autorizzazione per ogni comando al di fuori dell'insieme integrato di sola lettura. Appare in `/config` come **Usa modalità automatica durante il piano**.

* **Ambito**: [`User, local, or managed`](#scopes). Un repository non può disattivarlo per te.
* **Tipo**: Booleano
  * `true`: lo stesso di non impostato; quando la modalità automatica è disponibile, il classificatore esamina ogni comando shell durante la pianificazione invece di chiederti. Un `false` in uno qualsiasi di questi file lo disattiva comunque
  * `false`: ricevi un prompt di autorizzazione per ogni comando al di fuori dell'insieme integrato di sola lettura
* **Predefinito**: `true`

```json settings.json theme={null}
{
  "useAutoModeDuringPlan": false
}
```

<h3 id="permissions-allow">
  `permissions.allow`
</h3>

Elenca gli usi degli strumenti che Claude Code approva senza chiederti. In una regola MCP, `*` può apparire solo nel nome dello strumento dopo il prefisso `mcp__<server>__`, come `mcp__github__get_*`; non può apparire nel nome del server.

* **Ambito**: [`Any file`](#scopes)
* **Tipo**: array di stringhe di regole di autorizzazione
* **Predefinito**: non impostato
* **Override per sessione**: `--allowedTools` aggiunge regole di autorizzazione per una sessione e una regola di negazione da qualsiasi file di impostazioni blocca comunque uno strumento che nomina

Questo esempio approva `git diff` e consente a Claude Code di leggere il tuo `.zshrc` senza chiedere:

```json settings.json theme={null}
{
  "permissions": {
    "allow": ["Bash(git diff *)", "Read(~/.zshrc)"]
  }
}
```

Claude Code applica le regole `allow` dal `.claude/settings.json` di un progetto solo dopo che accetti la [finestra di dialogo di fiducia dell'area di lavoro](/docs/it/permissions#project-allow-rules-and-workspace-trust) per quella cartella.

<h4 id="permission-rule-syntax">
  Sintassi della regola di autorizzazione
</h4>

Le regole di autorizzazione seguono il formato `Tool` o `Tool(specifier)`. Claude Code valuta prima le regole `deny`, poi `ask`, poi `allow`, e la prima corrispondenza decide indipendentemente da quanto specifica sia ogni regola; vedere l'[ordine di valutazione della regola di autorizzazione](/docs/it/permissions#manage-permissions).

Ogni riga mostra una forma di regola e cosa corrisponde.

| Regola                         | Cosa corrisponde                    |
| :----------------------------- | :---------------------------------- |
| `Bash`                         | Ogni comando Bash                   |
| `Bash(npm run *)`              | Comandi che iniziano con `npm run`  |
| `Read(./.env)`                 | Letture del file `.env`             |
| `WebFetch(domain:example.com)` | Richieste di recupero a example.com |

Per la sintassi completa della regola, incluso il comportamento dei wildcard, i modelli specifici dello strumento per Read, Edit, WebFetch, MCP e regole Agent e i limiti di sicurezza dei modelli Bash, vedere [Sintassi della regola di autorizzazione](/docs/it/permissions#permission-rule-syntax).

<h3 id="permissions-ask">
  `permissions.ask`
</h3>

Elenca gli usi degli strumenti che ti richiedono una conferma anche in una modalità di autorizzazione che altrimenti li approverebbe, come `acceptEdits` o `bypassPermissions`. In modalità `dontAsk` Claude Code nega un uso dello strumento corrispondente invece di richiedere.

* **Ambito**: [`Any file`](#scopes)
* **Tipo**: array di stringhe di regole di autorizzazione
* **Predefinito**: non impostato

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

Elenca gli usi degli strumenti che Claude Code blocca. Usalo per file che contengono chiavi API, segreti o valori di ambiente: Claude Code esclude i file corrispondenti dalla scoperta dei file e dai risultati della ricerca, nega le letture di essi e blocca gli [strumenti Edit e Write](/docs/it/permissions#read-and-edit) sui percorsi corrispondenti.

Le regole di negazione Read e Edit si applicano agli strumenti di file integrati di Claude, ai comandi di file che Claude Code riconosce in Bash, come `cat`, `head`, `tail`, `sed` e `tee`, e ai target dei [reindirizzamenti](/docs/it/permissions#redirections) Bash come `> file` e `< file`; non si applicano a un comando che legge file senza nominarli, come `grep -r pattern .`, o a sottoprocessi arbitrari, quindi per l'applicazione a livello di sistema operativo [abilita la sandbox](/docs/it/sandboxing).

* **Ambito**: [`Any file`](#scopes)
* **Tipo**: array di stringhe di regole di autorizzazione
* **Predefinito**: non impostato
* **Override per sessione**: `--disallowedTools` aggiunge regole di negazione per una sessione insieme a questa chiave

Questo esempio nega le letture dei file `.env`, della directory `secrets` e di un file di credenziali e blocca i comandi `curl`:

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

I nomi degli strumenti accettano modelli glob, quindi `"*"` nega ogni strumento e `"mcp__*"` nega ogni strumento MCP. Claude Code ignora una regola di negazione per lo strumento [`EndConversation`](/docs/it/tools-reference#endconversation-tool-behavior) finché qualsiasi altro strumento è ancora disponibile per Claude. Una regola di negazione `Bash` corrisponde al comando come Claude lo scrive, quindi `Bash(curl *)` non ferma `/usr/bin/curl` o `sh -c 'curl …'`; vedere [cosa una regola Bash non corrisponde](/docs/it/permissions#bash-rule-limits). Questa chiave sostituisce la configurazione deprecata `ignorePatterns`.

<h3 id="permissions-additionaldirectories">
  `permissions.additionalDirectories`
</h3>

Dai a Claude l'accesso ai file alle directory al di fuori di quella in cui hai iniziato, come [directory di lavoro](/docs/it/permissions#working-directories) aggiuntive. La maggior parte della configurazione `.claude/` [non viene scoperta](/docs/it/permissions#additional-directories-grant-file-access-not-configuration) da queste directory.

* **Ambito**: [`Any file`](#scopes)
* **Tipo**: array di percorsi di directory
* **Predefinito**: non impostato
* **Override per sessione**: `--add-dir` e `/add-dir` aggiungono directory per una sessione insieme a questa chiave

```json settings.json theme={null}
{
  "permissions": {
    "additionalDirectories": ["../docs/"]
  }
}
```

Come le regole `allow`, le voci nel `.claude/settings.json` di un progetto hanno effetto solo dopo che accetti la [finestra di dialogo di fiducia dell'area di lavoro](/docs/it/permissions#project-allow-rules-and-workspace-trust) per quella cartella.

<h3 id="permissions-blockreadsoutsideworkingdirectories">
  `permissions.blockReadsOutsideWorkingDirectories`
</h3>

Impedisci a Claude di leggere percorsi al di fuori delle [directory di lavoro](/docs/it/permissions#working-directories) della sessione con gli strumenti Read, Grep, Glob e LSP, in ogni modalità di autorizzazione inclusa `bypassPermissions`. Un comando Bash che legge un percorso corrispondente attraverso un comando di file che Claude Code riconosce, come `cat`, ti richiede anche in modalità automatica e modalità `bypassPermissions`. Richiede Claude Code v2.1.257 o successivo.

Un comando Bash che il parser della shell non può tracciare, come uno che cambia directory più di una volta o esegue una subshell, ti richiede anche in modalità automatica e modalità `bypassPermissions`. Il prompt appare anche quando il comando non nomina alcun percorso al di fuori delle directory di lavoro. Questo prompt non si applica quando il comando viene eseguito nella [sandbox](/docs/it/sandboxing) e la sandbox applica il blocco.

Claude Code scrive anche `true` qui quando scegli di bloccare tali letture sul [prompt della modalità automatica prima della prima lettura al di fuori delle directory di lavoro](/docs/it/permission-modes#first-read-outside-the-working-directories).

* **Ambito**: [`Any file`](#scopes). Se qualsiasi fonte di impostazioni imposta `true`, il blocco si applica, quindi il file archiviato di un repository può attivare il blocco per un progetto ma non può sollevare un blocco che hai impostato.
* **Tipo**: Booleano
  * `true`: le letture di file al di fuori delle directory di lavoro sono bloccate
  * `false`: lo stesso di non impostato; un `true` in qualsiasi altro file di impostazioni blocca comunque
* **Predefinito**: non impostato, quindi le letture al di fuori delle directory di lavoro seguono la tua modalità di autorizzazione e le regole

```json settings.json theme={null}
{
  "permissions": {
    "blockReadsOutsideWorkingDirectories": true
  }
}
```

Se solo il file di impostazioni archiviato di un repository aggiunge una directory, il blocco si applica comunque alle letture lì. Quando [`autoMemoryDirectory`](#automemorydirectory) proviene dal `.claude/settings.json` del progetto, o da un `.claude/settings.local.json` [trattato come fornito dal repository](/docs/it/permissions#when-your-local-settings-file-needs-trust), Claude Code non carica alcuna [memoria automatica](/docs/it/memory#storage-location) da quella directory e non ne salva alcuna. I file che Claude Code stesso ha bisogno rimangono leggibili, come le tue skill, plugin, regole, agent, comandi e il file di memoria `CLAUDE.md` sotto `~/.claude/`.

Quando la [sandbox](/docs/it/sandboxing) è attiva, il blocco nega anche ai comandi in sandbox l'accesso in lettura alle directory home e alle radici dei volumi montati al di fuori delle directory di lavoro. Un nuovo tentativo che ha bisogno di approvazione per [eseguire al di fuori della sandbox](/docs/it/sandboxing#the-unsandboxed-retry-escape-hatch) ti richiede anche in modalità `bypassPermissions`. I file che uno strumento legge dalla tua directory home, come `~/.gitconfig`, vengono negati con il resto; riapri un percorso specifico con [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread) quando uno strumento ha bisogno di esso.

Quando la directory di lavoro della sessione è un [git worktree](/docs/it/worktrees) collegato, incluso uno che Claude Code ha inserito a metà sessione, la directory `.git` comune del repository rimane leggibile e scrivibile ai comandi in sandbox, in modo che git continui a funzionare lì.

<h3 id="permissions-defaultmode">
  `permissions.defaultMode`
</h3>

Imposta la [modalità di autorizzazione](/docs/it/permission-modes) in cui le nuove sessioni iniziano. Quando la lasci non impostata, le sessioni iniziano nel [predefinito integrato](/docs/it/permission-modes#which-mode-a-session-starts-in) per il tuo piano e superficie.

* **Ambito**: [`Any file`](#scopes). `auto` e `bypassPermissions` non hanno effetto dalle impostazioni di progetto o locale, quindi impostali in `~/.claude/settings.json` invece. Prima della v2.1.257, `bypassPermissions` aveva effetto da qualsiasi file. Per le conversazioni che l'estensione VS Code avvia, Claude Code legge solo i valori utente, gestiti e `--settings`.
* **Tipo**: stringa, uno di:
  * `"default"`: Claude Code esegue solo letture senza chiedere
  * `"acceptEdits"`: Claude Code esegue anche modifiche di file e comandi comuni del file system come `mkdir` e `mv` senza chiedere
  * `"plan"`: Claude Code legge e pianifica ma blocca le modifiche finché non approvi un piano
  * `"auto"`: Claude Code esegue tutto, con controlli di sicurezza in background
  * `"dontAsk"`: Claude Code nega automaticamente ogni chiamata che altrimenti richiederebbe; le letture, altre azioni che non richiedono approvazione e gli strumenti pre-approvati si eseguono comunque
  * `"bypassPermissions"`: Claude Code esegue tutto senza chiedere
  * `"manual"`: un alias per `"default"`, in Claude Code v2.1.200 o successivo
* **Predefinito**: non impostato
* **Override per sessione**: `--permission-mode` e il suo equivalente `--dangerously-skip-permissions` per `bypassPermissions` hanno la precedenza su questa chiave per una sessione

```json settings.json theme={null}
{
  "permissions": {
    "defaultMode": "acceptEdits"
  }
}
```

Le regole di autorizzazione si sovrappongono a ogni modalità: le regole `deny` bloccano in ogni modalità, inclusa `bypassPermissions`. Vedere [Modalità di autorizzazione](/docs/it/permission-modes). `manual` nomina la modalità di autorizzazione etichettata Manual nella CLI e nell'estensione VS Code; l'alias richiede Claude Code v2.1.200 o successivo. In Claude Code sul web, Claude Code onora solo `acceptEdits`, `plan`, `default` e `auto` da questa chiave. Per le conversazioni che l'estensione VS Code avvia, vedere [quale impostazione l'estensione legge per la modalità di autorizzazione iniziale](/docs/it/permission-modes#switch-permission-modes).

<h3 id="permissions-disablebypasspermissionsmode">
  `permissions.disableBypassPermissionsMode`
</h3>

Impedisci a chiunque di entrare in modalità `bypassPermissions`. Claude Code rifiuta quindi il flag `--dangerously-skip-permissions` e ignora la [definizione di un agent](/docs/it/sub-agents#permission-modes) `permissionMode: bypassPermissions`, quindi il subagent viene eseguito con la modalità di autorizzazione della sessione padre.

* **Ambito**: [`Any file`](#scopes). Tipicamente impostato nelle [impostazioni gestite](/docs/it/managed-settings) per applicare la politica organizzativa.
* **Tipo**: la stringa `"disable"`
* **Predefinito**: non impostato
* **Override per sessione**: questa chiave ha la precedenza su `--dangerously-skip-permissions`, che Claude Code rifiuta mentre la chiave è impostata

```json settings.json theme={null}
{
  "permissions": {
    "disableBypassPermissionsMode": "disable"
  }
}
```

Prima della v2.1.223, Claude Code applicava la modalità di autorizzazione del frontmatter anche con il bypass disabilitato.

<h3 id="skipautopermissionprompt">
  `skipAutoPermissionPrompt`
</h3>

Salta l'avviso una tantum che descrive la [modalità automatica](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) che Claude Code mostra quando entri per la prima volta in modalità automatica tu stesso, ad esempio attraverso le tue impostazioni o il selettore di modalità, piuttosto che quando il predefinito integrato avvia una sessione in essa. Claude Code mostra quell'avviso una volta e poi registra che è stato mostrato, quindi questa chiave ha importanza solo dove l'avviso non è ancora apparso.

* **Ambito**: [`User or managed`](#scopes). Un repository non può impostarlo per te.
* **Tipo**: Booleano
  * `true`: Claude Code salta l'avviso
  * `false`: lo stesso di non impostato; l'avviso appare una volta a meno che un altro di questi file non imposti `true`
* **Predefinito**: non impostato, quindi l'avviso appare una volta

```json settings.json theme={null}
{
  "skipAutoPermissionPrompt": true
}
```

<h3 id="skipdangerousmodepermissionprompt">
  `skipDangerousModePermissionPrompt`
</h3>

Salta la finestra di dialogo di conferma che Claude Code mostra prima che una sessione entri in modalità `bypassPermissions`, sia da `--dangerously-skip-permissions` che da `defaultMode: "bypassPermissions"`. Claude Code scrive `true` qui nelle tue impostazioni utente quando accetti quella finestra di dialogo una volta.

* **Ambito**: [`User, local, or managed`](#scopes). Un repository non affidabile non può saltare la finestra di dialogo per te.
* **Tipo**: Booleano
  * `true`: Claude Code salta la finestra di dialogo di conferma prima che una sessione entri in modalità `bypassPermissions`
  * `false`: lo stesso di non impostato; la finestra di dialogo appare a meno che un altro di questi file non imposti `true`
* **Predefinito**: non impostato, quindi la finestra di dialogo appare

```json settings.json theme={null}
{
  "skipDangerousModePermissionPrompt": true
}
```

<h2 id="sandbox-settings">
  Impostazioni sandbox
</h2>

Isola i comandi che Claude esegue dal tuo filesystem, dalla tua rete e dalle tue credenziali. Per informazioni su come funziona il sandboxing e sui requisiti della piattaforma, vedi [Sandboxing](/docs/it/sandboxing).

<h3 id="sandbox">
  `sandbox`
</h3>

Isola i comandi Bash che Claude esegue dal tuo filesystem e dalla rete con il [sandboxing](/docs/it/sandboxing). Attiva la sandbox con `enabled`, quindi restringi o amplia ciò che i comandi in sandbox possono toccare con i sotto-oggetti `filesystem`, `network` e `credentials`. La sandbox funziona su macOS, Linux e WSL2.

* **Scope**: [`Any file`](#scopes)
* **Type**: object con `enabled`, `failIfUnavailable`, `autoAllowBashIfSandboxed`, `excludedCommands`, `allowUnsandboxedCommands`, `enableWeakerNestedSandbox`, `enableWeakerNetworkIsolation`, `allowAppleEvents`, `bwrapPath`, `socatPath`, `ignoreViolations` e `ripgrep`, più gli oggetti `filesystem`, `network` e `credentials`
* **Default**: non impostato, quindi Claude Code esegue i comandi senza sandbox

Questo attiva la sandbox, salta i prompt di autorizzazione per i comandi in sandbox, esegue `docker` al di fuori della sandbox, apre due percorsi di scrittura aggiuntivi, nasconde il tuo file di credenziali AWS e pre-consente GitHub e npm:

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

Claude Code prende il valore di una chiave booleana dall'ambito di impostazioni con la precedenza più alta che la imposta, quindi un `enabled` o `failIfUnavailable` gestito sovrascrive qualsiasi cosa uno sviluppatore imposti. Unisce le chiavi array in ogni ambito di impostazioni che la sessione carica, quindi uno sviluppatore può aggiungere voci; vedi [Keep developers from widening the policy](/docs/it/sandboxing#keep-developers-from-widening-the-policy) per i blocchi solo gestiti. Per richiedere la sandbox per un'organizzazione, vedi [Enforce sandboxing with managed settings](/docs/it/sandboxing#enforce-sandboxing-with-managed-settings).

<h3 id="sandbox-enabled">
  `sandbox.enabled`
</h3>

Attiva il [sandboxing](/docs/it/sandboxing) per i comandi Bash. Quando scegli una modalità nel pannello `/sandbox`, Claude Code scrive questa chiave in `.claude/settings.local.json` per il progetto corrente; impostala in `~/.claude/settings.json` per mettere in sandbox ogni progetto.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code mette in sandbox i comandi Bash
  * `false`: i comandi Bash vengono eseguiti senza sandbox
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true
  }
}
```

Su Linux e WSL2 la sandbox ha bisogno di `bubblewrap` e `socat`; vedi [Set up Linux and WSL2](/docs/it/sandboxing#set-up-linux-and-wsl2). Quando la sandbox non può avviarsi, Claude Code mostra un avviso ed esegue i comandi senza sandbox a meno che tu non imposti anche [`failIfUnavailable`](#sandbox-failifunavailable).

<h3 id="sandbox-failifunavailable">
  `sandbox.failIfUnavailable`
</h3>

Fai uscire Claude Code con un errore all'avvio quando `sandbox.enabled` è `true` ma la sandbox non può avviarsi, perché una dipendenza è mancante o la piattaforma non è supportata. Senza di essa, Claude Code mostra un avviso ed esegue i comandi senza sandbox. Usala nelle impostazioni gestite quando la tua organizzazione richiede il sandboxing come un gate rigido.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code esce con un errore all'avvio quando `sandbox.enabled` è `true` ma la sandbox non può avviarsi
  * `false`: Claude Code mostra un avviso ed esegue i comandi senza sandbox
* **Default**: `false`

Questo fa sì che ogni macchina gestita metta in sandbox i comandi o rifiuti di avviarsi:

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "failIfUnavailable": true
  }
}
```

Vedi [Enforce sandboxing with managed settings](/docs/it/sandboxing#enforce-sandboxing-with-managed-settings).

<h3 id="sandbox-autoallowbashifsandboxed">
  `sandbox.autoAllowBashIfSandboxed`
</h3>

Consenti a Claude Code di eseguire comandi Bash in sandbox senza un prompt di autorizzazione. I comandi che non possono essere eseguiti nella sandbox seguono comunque il flusso di autorizzazione regolare, e le regole `deny` e le regole `ask` con ambito di contenuto come `Bash(git push *)` si applicano comunque; una regola `ask` Bash semplice viene saltata per i comandi in sandbox. Impostala su `false` per inviare i comandi in sandbox anche attraverso il flusso di autorizzazione regolare, che la scheda **Mode** di `/sandbox` chiama modalità di autorizzazioni regolari.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code esegue i comandi Bash in sandbox senza un prompt di autorizzazione, soggetto alle regole `deny` e alle regole `ask` con ambito di contenuto; `CLAUDE_CODE_SUBPROCESS_ENV_SCRUB` disattiva l'auto-consentimento
  * `false`: i comandi in sandbox seguono il flusso di autorizzazione regolare, quindi le tue regole di consentimento e la modalità di autorizzazione decidono. La scheda **Mode** di `/sandbox` chiama questa modalità di autorizzazioni regolari
* **Default**: `true`

Questo mantiene la sandbox attiva e invia i comandi in sandbox attraverso il flusso di autorizzazione regolare:

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "autoAllowBashIfSandboxed": false
  }
}
```

Vedi [Sandbox modes](/docs/it/sandboxing#sandbox-modes) per ciò che la modalità auto-consentimento ancora richiede e come si comporta in plan mode.

<h3 id="sandbox-excludedcommands">
  `sandbox.excludedCommands`
</h3>

Nomina i comandi che Claude Code esegue al di fuori della sandbox, come gli strumenti che non funzionano sotto di essa. Ogni voce utilizza la stessa sintassi del contenuto di una [regola di autorizzazione](/docs/it/permissions#permission-rule-syntax) `Bash(...)`: un comando esatto, un prefisso come `docker *` o un pattern con wildcard.

Le tue voci tolgono una chiamata Bash dalla sandbox solo quando coprono ogni comando in essa, e alcune forme di chiamata rimangono in sandbox anche allora. Una voce `docker *` da sola non toglie `npm ci && docker build .` dalla sandbox.

* **Scope**: [`Any file`](#scopes)
* **Type**: array di pattern di comando
* **Default**: non impostato, quindi nessun comando è escluso

```json settings.json theme={null}
{
  "sandbox": {
    "excludedCommands": ["docker *"]
  }
}
```

Claude Code mantiene una chiamata Bash in sandbox quando ha una di queste forme, tra le altre:

* Un comando che inizia con `sudo`, `eval` o `xargs`
* Un `cd`, `pushd` o `popd`, ovunque appaia nella chiamata
* Una sostituzione di comando, una subshell o un blocco di controllo di flusso come `if` o `for`
* Un reindirizzamento, come `docker build . > build.log`, diverso da uno che duplica solo un descrittore di file, come `2>&1`
* Un nome di comando che proviene da una variabile

Ad esempio, `cd build && docker compose up` rimane in sandbox sotto una voce `docker *`, e aggiungere una voce `cd` non cambia questo.

I comandi esclusi seguono comunque il flusso di autorizzazione regolare. L'esclusione è una comodità, non un confine di sicurezza: preferisci [`filesystem.allowWrite`](#sandbox-filesystem-allowwrite) quando uno strumento ha solo bisogno di scrivere da qualche parte di specifico. Claude Code unisce le voci in ogni ambito di impostazioni che la sessione carica, e non c'è un blocco solo gestito per questo elenco, quindi mantieni un elenco gestito ristretto.

<h3 id="sandbox-allowunsandboxedcommands">
  `sandbox.allowUnsandboxedCommands`
</h3>

Consenti a Claude di ritentare un comando al di fuori della sandbox con il parametro `dangerouslyDisableSandbox` dopo che la sandbox lo blocca. Impostalo su `false` in modo che Claude Code ignori completamente quel parametro e ogni comando che Claude esegue deve essere in sandbox o apparire in [`excludedCommands`](#sandbox-excludedcommands). La scheda **Overrides** di `/sandbox` mostra quello stato come **Strict sandbox mode**. Usa `false` nelle impostazioni gestite per le politiche che richiedono il sandboxing rigoroso.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude può ritentare un comando al di fuori della sandbox con il parametro `dangerouslyDisableSandbox` dopo che la sandbox lo blocca
  * `false`: Claude Code ignora quel parametro, quindi ogni comando che Claude esegue è in sandbox o appare in `excludedCommands`
* **Default**: `true`

Questo applica la modalità sandbox rigorosa per tutti coloro che le impostazioni gestite coprono:

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "allowUnsandboxedCommands": false
  }
}
```

Un ritentativo senza sandbox passa attraverso il flusso di autorizzazione regolare, con un prompt in modalità Manual. Vedi [The unsandboxed retry escape hatch](/docs/it/sandboxing#the-unsandboxed-retry-escape-hatch).

Per vedere quando i comandi che digiti tu stesso al prompt di modalità shell [`!`](/docs/it/interactive-mode#shell-mode-with-prefix) vengono eseguiti in sandbox, vedi [strict sandbox mode](/docs/it/sandboxing#the-unsandboxed-retry-escape-hatch).

<h3 id="sandbox-filesystem">
  `sandbox.filesystem`
</h3>

Controlla quali percorsi i comandi in sandbox possono leggere e scrivere. Per impostazione predefinita possono scrivere nella directory di lavoro, nella directory temporanea della sessione e nelle directory che aggiungi con `--add-dir`, `/add-dir` o `permissions.additionalDirectories`, e possono leggere il resto del filesystem, inclusi i file di credenziali. Amplia o restringi con i quattro elenchi di percorsi, o disattiva il livello del filesystem con `disabled`. Vedi [Filesystem isolation](/docs/it/sandboxing#filesystem-isolation) per i confini predefiniti.

* **Scope**: [`Any file`](#scopes)
* **Type**: object con array `allowWrite`, `denyWrite`, `denyRead` e `allowRead`, più i booleani `allowManagedReadPathsOnly` e `disabled`
* **Default**: non impostato, quindi si applicano i confini di lettura e scrittura predefiniti

Questo consente ai comandi in sandbox di scrivere in una directory di build e nel tuo kubeconfig, e nasconde il tuo file di credenziali AWS:

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

Claude Code applica questi elenchi al confine della sandbox del sistema operativo, quindi si applicano a ogni sottoprocesso che un comando in sandbox avvia, come `kubectl`, `terraform` o `npm`. Claude Code aggiunge le tue [regole di autorizzazione](/docs/it/sandboxing#permission-rules) agli stessi elenchi: le regole `Edit` allow e deny a `allowWrite` e `denyWrite`, le regole `Read` deny a `denyRead` e le regole `WebFetch(domain:...)` allow e deny agli elenchi di domini [`network`](#sandbox-network).

A meno che non sia impostato un blocco solo gestito, Claude Code unisce ogni elenco nei file di impostazioni che la sessione carica. [`allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly) limita `allowRead` alle voci dalle impostazioni gestite, e [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) fa lo stesso per i domini consentiti.

[Configure sandboxing](/docs/it/sandboxing#configure-sandboxing) copre le fonti che escludi con `--setting-sources`. Quando modifichi un elenco durante una sessione, Claude Code [applica la modifica alla sessione in esecuzione](/docs/it/settings#when-edits-take-effect).

<h4 id="sandbox-path-prefixes">
  Prefissi di percorso sandbox
</h4>

I percorsi in `allowWrite`, `denyWrite`, `denyRead`, `allowRead` e [`credentials.files`](#sandbox-credentials-files) si risolvono in base al loro prefisso:

| Prefisso               | Significato                                                                                                         | Esempio                                                                     |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------- |
| `/`                    | Percorso assoluto dalla radice del filesystem                                                                       | `/tmp/build` rimane `/tmp/build`                                            |
| `~/`                   | Relativo alla directory home                                                                                        | `~/.kube` diventa `$HOME/.kube`                                             |
| `./` o nessun prefisso | Relativo alla radice del progetto per le impostazioni del progetto, o a `~/.claude` per le impostazioni dell'utente | `./output` in `.claude/settings.json` si risolve in `<project-root>/output` |

Il prefisso `//path` per i percorsi assoluti funziona anche. Se usi un singolo slash `/path` aspettandoti una risoluzione relativa al progetto, passa a `./path`. Questa sintassi differisce dalle [regole di autorizzazione Read e Edit](/docs/it/permissions#read-and-edit), che usano `//path` per assoluto e `/path` per relativo al progetto: i percorsi del filesystem sandbox usano convenzioni standard, quindi `/tmp/build` è un percorso assoluto.

Claude Code rimuove uno slash finale da un percorso di directory, quindi `~/.aws` e `~/.aws/` corrispondono alla stessa directory. Prima della v2.1.224, Claude Code passava lo slash finale alla sandbox, e Claude poteva comunque leggere o scrivere percorsi sotto una voce `denyRead` o `denyWrite` scritta con uno.

Claude Code rimuove anche un `/**` finale, quindi `~/build/**` e `~/build` coprono la stessa directory. Se un wildcard come `*` funziona dipende da quale elenco è la voce e dalla piattaforma:

* **`allowWrite` e `denyWrite`**: su macOS, i wildcard funzionano. Su Linux e WSL2, la sandbox monta percorsi concreti, quindi Claude Code salta una voce che contiene `*`, `?` o `[` una volta rimosso il `/**` finale, e quella voce non ha effetto. Claude Code aggiunge i percorsi dalle tue regole di autorizzazione `Edit` a questi elenchi, quindi lo stesso limite si applica a loro, e la scheda **Config** di `/sandbox` avverte le regole di autorizzazione `Edit` e `Read` che contengono wildcard.
* **`denyRead` e `allowRead`**: i wildcard funzionano su ogni piattaforma. Su Linux e WSL2, Claude Code espande una voce di lettura ai percorsi concreti che corrisponde, cosa che non fa per gli elenchi di scrittura.

<h3 id="sandbox-filesystem-allowwrite">
  `sandbox.filesystem.allowWrite`
</h3>

Aggiungi percorsi dove i comandi in sandbox possono scrivere, oltre alla directory di lavoro, alla directory temporanea della sessione e alle directory che hai aggiunto con `--add-dir`, `/add-dir` o `permissions.additionalDirectories`. Usalo quando un sottoprocesso come `kubectl` o uno strumento di build ha bisogno di scrivere al di fuori del progetto.

* **Scope**: [`Any file`](#scopes)
* **Type**: array di stringhe di percorso, usando i [prefissi di percorso sandbox](#sandbox-path-prefixes)
* **Default**: non impostato, quindi i comandi in sandbox possono scrivere nella directory di lavoro, nella directory temporanea della sessione, nelle directory che hai aggiunto con `--add-dir` o `/add-dir` e nelle directory in [`permissions.additionalDirectories`](#permissions-additionaldirectories)

Questo consente a una build di scrivere sotto `/tmp/build` e consente a `kubectl` di aggiornare il tuo kubeconfig:

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "allowWrite": ["/tmp/build", "~/.kube"]
    }
  }
}
```

Claude Code unisce le voci in ogni ambito di impostazioni che la sessione carica: i percorsi utente, progetto, locale e gestito si combinano piuttosto che sostituirsi a vicenda, e Claude Code aggiunge i percorsi dalle tue regole di autorizzazione `Edit(...)` allow. Una voce `allowWrite` non può sollevare un [percorso protetto](/docs/it/sandboxing#protected-paths).

<h3 id="sandbox-filesystem-denywrite">
  `sandbox.filesystem.denyWrite`
</h3>

Blocca i comandi in sandbox dallo scrivere su percorsi specifici, inclusi i percorsi all'interno di una directory che è altrimenti scrivibile.

* **Scope**: [`Any file`](#scopes)
* **Type**: array di stringhe di percorso, usando i [prefissi di percorso sandbox](#sandbox-path-prefixes)
* **Default**: non impostato

Questo impedisce ai comandi in sandbox di modificare la configurazione del sistema o installare binari:

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyWrite": ["/etc", "/usr/local/bin"]
    }
  }
}
```

Claude Code unisce le voci in ogni ambito di impostazioni che la sessione carica, e aggiunge i percorsi dalle tue regole di autorizzazione `Edit(...)` deny.

<h3 id="sandbox-filesystem-denyread">
  `sandbox.filesystem.denyRead`
</h3>

Blocca i comandi in sandbox dal leggere percorsi specifici, come i file di credenziali che la politica di lettura predefinita esporrebbe altrimenti. Per proteggere un file di credenziali e mantenerlo utilizzabile attraverso il proxy della sandbox, vedi [`sandbox.credentials`](#sandbox-credentials) invece.

* **Scope**: [`Any file`](#scopes)
* **Type**: array di stringhe di percorso, usando i [prefissi di percorso sandbox](#sandbox-path-prefixes)
* **Default**: non impostato, quindi i comandi in sandbox mantengono l'[accesso in lettura predefinito](/docs/it/sandboxing#filesystem-isolation), che include i file di credenziali come `~/.aws/credentials`

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyRead": ["~/.aws/credentials"]
    }
  }
}
```

Claude Code unisce le voci in ogni ambito di impostazioni che la sessione carica, e aggiunge i percorsi dalle tue regole di autorizzazione `Read(...)` deny. Quando [`filesystem.disabled`](#sandbox-filesystem-disabled) è `true`, Claude Code non applica queste voci.

<h3 id="sandbox-filesystem-allowread">
  `sandbox.filesystem.allowRead`
</h3>

Riapri la lettura per percorsi specifici all'interno di una regione che [`denyRead`](#sandbox-filesystem-denyread) blocca, per costruire un accesso in lettura solo per l'area di lavoro. Una voce `denyRead` esatta o con wildcard rimane bloccata all'interno di un `allowRead` più ampio, come mostra la [tabella di sovrapposizione](/docs/it/sandboxing#configure-sandboxing). Quando una voce `denyRead` con wildcard come `~/**/.env` corrisponde a una directory, Claude Code blocca le letture dei suoi contenuti anche. Prima della v2.1.236 su macOS, Claude Code riaprì i percorsi che una voce `denyRead` con wildcard corrispondeva ovunque una voce `allowRead` più ampia li copriva, e lasciò i contenuti di una directory corrispondente leggibili.

* **Scope**: [`Any file`](#scopes)
* **Type**: array di stringhe di percorso, usando i [prefissi di percorso sandbox](#sandbox-path-prefixes)
* **Default**: non impostato

Questo blocca le letture della tua directory home tranne il progetto stesso:

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

Claude Code risolve una voce `.` alla radice del progetto nelle impostazioni del progetto e a `~/.claude` nelle impostazioni dell'utente. Claude Code unisce le voci in ogni file di impostazioni che la sessione carica a meno che [`allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly) non sia impostato.

<h3 id="sandbox-filesystem-allowmanagedreadpathsonly">
  `sandbox.filesystem.allowManagedReadPathsOnly`
</h3>

Onora solo le voci [`allowRead`](#sandbox-filesystem-allowread) che provengono dalle impostazioni gestite, in modo che gli sviluppatori non possano riaprire l'accesso in lettura ai percorsi che la tua organizzazione ha bloccato. Claude Code unisce comunque le voci `denyRead` da ogni ambito di impostazioni che la sessione carica.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code onora solo le voci `allowRead` dalle impostazioni gestite
  * `false`: le voci `allowRead` si uniscono da ogni ambito di impostazioni che la sessione carica
* **Default**: `false`

Questo blocca le letture della directory home, riapre `~/work` e impedisce agli sviluppatori di riaprire qualsiasi altra cosa:

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

Vedi [Keep developers from widening the policy](/docs/it/sandboxing#keep-developers-from-widening-the-policy).

<h3 id="sandbox-filesystem-disabled">
  `sandbox.filesystem.disabled`
</h3>

Salta l'isolamento del filesystem mantenendo l'isolamento della rete. I comandi in sandbox ottengono accesso in lettura e scrittura senza restrizioni al filesystem host, e il loro egresso di rete rimane confinato a [`network.allowedDomains`](#sandbox-network-alloweddomains). Usalo quando metti in sandbox per controllare dove i comandi si connettono piuttosto che cosa scrivono. Richiede Claude Code v2.1.216 o successivo.

* **Scope**: [`User or managed`](#scopes). Quando le impostazioni gestite configurano `sandbox.filesystem` affatto, o elencano una voce `sandbox.credentials.files` con `"mode": "deny"`, solo le impostazioni gestite possono impostarla.
* **Type**: Boolean
  * `true`: Claude Code salta l'isolamento del filesystem e mantiene l'isolamento della rete
  * `false`: l'isolamento del filesystem rimane attivo
* **Default**: `false`, quindi l'isolamento del filesystem rimane attivo

Questo lascia il filesystem aperto e confina l'egresso di rete a GitHub e npm:

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

Con il livello disattivato, Claude Code non applica le voci `denyRead` o `credentials.files` `deny`, mentre le voci `credentials.envVars` e le voci `mask` applicate continuano a funzionare. [`autoAllowBashIfSandboxed`](#sandbox-autoallowbashifsandboxed) continua a impostazione predefinita su `true`, quindi impostalo su `false` per continuare a richiedere. Vedi [Disable filesystem isolation](/docs/it/sandboxing#disable-filesystem-isolation) per l'elenco completo delle fonti che possono impostarla e cosa cambia quando l'isolamento è disattivato. Richiede Claude Code v2.1.216 o successivo.

<h3 id="sandbox-ignoreviolations">
  `sandbox.ignoreViolations`
</h3>

Silenzia i rapporti di violazione della sandbox per i percorsi che ti aspetti che un comando sonda e sia rifiutato, come uno strumento che controlla `/etc/hosts` all'avvio, in modo che quei rifiuti non vengano visualizzati come violazioni o in ciò che Claude vede. La sandbox blocca comunque l'accesso; solo il rapporto è soppresso. Le chiavi sono sottostringhe da abbinare al comando, con `*` che corrisponde a ogni comando, e i valori sono sottostringhe della violazione da ignorare per quel comando, come un percorso del filesystem.

* **Scope**: [`Any file`](#scopes)
* **Type**: object che mappa una sottostringa di comando a un array di sottostringhe di violazione, solitamente percorsi
* **Default**: non impostato, quindi ogni violazione è segnalata

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

Esegui la sandbox Linux all'interno di un contenitore Docker senza privilegi, dove bubblewrap non può montare un `/proc` fresco. Invece la sandbox interna bind-monta il `/proc` esistente del contenitore, che espone informazioni di processo che un mount fresco nasconderebbe. Questo riduce la sicurezza; usalo solo quando il contenitore esterno fornisce già l'isolamento di cui hai bisogno.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: la sandbox interna bind-monta il `/proc` esistente del contenitore invece di montarne uno fresco
  * `false`: la sandbox monta un `/proc` fresco, che non funziona in un contenitore Docker senza privilegi
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "enableWeakerNestedSandbox": true
  }
}
```

Solo Linux e WSL2. Vedi [Bubblewrap fails to start inside a container](/docs/it/sandboxing#troubleshooting).

<h3 id="sandbox-enableweakernetworkisolation">
  `sandbox.enableWeakerNetworkIsolation`
</h3>

Consenti ai comandi in sandbox su macOS di raggiungere il servizio di fiducia TLS del sistema, `com.apple.trustd.agent`. Gli strumenti basati su Go come `gh`, `gcloud` e `terraform` ne hanno bisogno per verificare i certificati TLS quando usi [`network.httpProxyPort`](#sandbox-network-httpproxyport) con un proxy MITM e una CA personalizzata. Questo riduce la sicurezza aprendo un potenziale percorso di esfiltrazione dei dati attraverso il servizio di fiducia.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: i comandi in sandbox su macOS possono raggiungere `com.apple.trustd.agent`
  * `false`: i comandi in sandbox su macOS non possono raggiungere il servizio di fiducia TLS del sistema
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "enableWeakerNetworkIsolation": true
  }
}
```

Se non usi un proxy MITM, elenca gli strumenti che falliscono in [`excludedCommands`](#sandbox-excludedcommands) invece; vedi [Go-based CLIs fail TLS verification on macOS](/docs/it/sandboxing#troubleshooting).

<h3 id="sandbox-allowappleevents">
  `sandbox.allowAppleEvents`
</h3>

Consenti ai comandi in sandbox su macOS di inviare Apple Events, che `open`, `osascript` e gli strumenti che aprono URL in un browser hanno bisogno; senza di esso falliscono con errore `-600`. Questo rimuove l'isolamento dell'esecuzione del codice: i comandi in sandbox possono lanciare altre applicazioni senza sandbox senza un prompt dell'utente, e possono inviare comandi AppleScript alle applicazioni in esecuzione come Terminal, soggetto al prompt di consenso per l'automazione per app macOS (TCC).

* **Scope**: [`User or managed`](#scopes)
* **Type**: Boolean
  * `true`: i comandi in sandbox su macOS possono inviare Apple Events
  * `false`: i comandi in sandbox su macOS non possono inviare Apple Events, quindi `open` e `osascript` falliscono con errore `-600`
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "allowAppleEvents": true
  }
}
```

Per mantenere l'isolamento e comunque eseguire uno di questi strumenti, aggiungilo a [`excludedCommands`](#sandbox-excludedcommands) invece. Vedi [Apple Events on macOS](/docs/it/sandboxing#security-limitations).

<h3 id="sandbox-ripgrep">
  `sandbox.ripgrep`
</h3>

Punta la sandbox a un binario ripgrep tuo invece di quello che Claude Code usa, ad esempio quando la tua piattaforma ha bisogno di un `rg` costruito diversamente.

* **Scope**: [`User or managed`](#scopes)
* **Type**: object con `command`, il percorso al binario ripgrep, e opzionale `args`, un array di argomenti da anteporre
* **Default**: non impostato, quindi la sandbox usa lo stesso binario ripgrep di Claude Code. Questo è il binario in bundle a meno che tu non imposti [`USE_BUILTIN_RIPGREP`](/docs/it/env-vars) su `0`

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

Punta la sandbox a un binario bubblewrap installato al di fuori di `PATH`, come una copia venduta su un host air-gapped. Claude Code usa il percorso sia per il controllo della dipendenza di avvio che quando avvolge ogni comando in sandbox.

* **Scope**: [`Managed`](#scopes). Claude Code lo legge solo dalle impostazioni gestite in modo che un file utente, progetto o locale non possa puntare la sandbox a un binario diverso.
* **Type**: string, un percorso assoluto; Claude Code scarta un percorso relativo e ricade sulla ricerca `PATH`
* **Default**: non impostato, quindi Claude Code trova `bwrap` su `PATH`

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "bwrapPath": "/opt/admin/bwrap"
  }
}
```

Solo Linux e WSL2.

<h3 id="sandbox-socatpath">
  `sandbox.socatPath`
</h3>

Punta il proxy di rete della sandbox a un binario `socat` installato al di fuori di `PATH`.

* **Scope**: [`Managed`](#scopes)
* **Type**: string, un percorso assoluto; Claude Code scarta un percorso relativo e ricade sulla ricerca `PATH`
* **Default**: non impostato, quindi Claude Code trova `socat` su `PATH`

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "socatPath": "/opt/admin/socat"
  }
}
```

Solo Linux e WSL2.

<h3 id="sandbox-credentials">
  `sandbox.credentials`
</h3>

Dichiara i file di credenziali e le variabili di ambiente da [proteggere dai comandi in sandbox](/docs/it/sandboxing#protect-credentials). Ogni voce nomina un file `path` o una variabile `name` e una `mode`: `deny` nasconde la credenziale all'interno della sandbox, e `mask` mostra ai comandi in sandbox un segnaposto mentre il [proxy della sandbox](/docs/it/sandboxing#mask-credentials) sostituisce il valore reale sulle richieste in uscita. Claude Code protegge solo le voci che elenchi; non c'è un elenco di negazione di credenziali incorporato.

* **Scope**: [`Any file`](#scopes). Claude Code onora le voci `mask`, `allowPlaintextInject`, `awsPairs` e `sigv4` solo dalle impostazioni utente, dalle impostazioni gestite e dal flag `--settings`.
* **Type**: object con `files`, `envVars`, `allowPlaintextInject`, `awsPairs` e `sigv4`
* **Default**: non impostato, quindi nessuna credenziale è protetta

Questo nasconde il tuo file di credenziali AWS e rimuove `GITHUB_TOKEN` dai comandi in sandbox:

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

La protezione del file `deny` fa parte del livello del filesystem, quindi non si applica quando [disabiliti l'isolamento del filesystem](/docs/it/sandboxing#disable-filesystem-isolation); la protezione della variabile di ambiente continua comunque.

<h4 id="invalid-credential-entries-in-managed-settings">
  Voci di credenziali non valide nelle impostazioni gestite
</h4>

Quando una voce `sandbox.credentials` gestita non supera la convalida, Claude Code continua a proteggere la credenziale dove può:

* Una voce in `files` o `envVars` che ha ancora un `path` o `name` valido e una `mode` di `mask` o `deny`, come una il cui pattern `extract` non ha un gruppo di cattura, è degradata a `mode: "deny"` con un avviso, quindi la credenziale rimane bloccata, non mascherata, finché non fissi la voce. Una voce `files` degradata fissa [`filesystem.disabled`](/docs/it/sandboxing#disable-filesystem-isolation) come una voce `deny` esplicita, e l'avviso nota che il suo blocco di lettura non è applicato se le impostazioni gestite disattivano l'isolamento del filesystem.
* Una voce con una `mode` sconosciuta o un `path` o `name` non valido è rimossa.
* Ogni caso avvisa; che una voce sia degradata o rimossa, le voci valide rimanenti sono ancora applicate, e un valore `credentials` interamente non valido viene scartato mentre il resto di `sandbox` si applica comunque.

Si applica nella v2.1.191 e successivo; prima della v2.1.221, ogni voce non valida era rimossa. Per le altre chiavi gestite con gestione per campo, vedi [Invalid entries in managed settings](/docs/it/managed-settings#invalid-entries-in-managed-settings).

<h3 id="sandbox-credentials-files">
  `sandbox.credentials.files`
</h3>

Proteggi i file o le directory di credenziali dai comandi in sandbox. Con `"mode": "deny"`, Claude Code blocca le letture del percorso all'interno della sandbox, lo stesso blocco di lettura di [`sandbox.filesystem.denyRead`](#sandbox-filesystem-denyread). Con `"mode": "mask"`, i comandi in sandbox su Linux e WSL2 leggono una copia sentinella del file, e il proxy della sandbox sostituisce il valore reale sulle richieste in uscita a `injectHosts` di quella voce; su macOS il file è illeggibile all'interno della sandbox invece. `"mode": "mask"` richiede Claude Code v2.1.221 o successivo.

* **Scope**: [`Any file`](#scopes). Claude Code scarta le voci `mask` da `.claude/settings.json` del progetto e da `.claude/settings.local.json` locale.
* **Type**: array di object, ognuno con `path` e una `mode` di `"deny"` o `"mask"`, più i [campi mask opzionali per i file](#mask-fields-for-files)
* **Default**: non impostato, quindi nessun file di credenziali è protetto

Questo nasconde il tuo file di credenziali AWS e maschera il file host `gh`, sostituendo il valore reale solo sulle richieste a `api.github.com`:

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

I percorsi usano gli stessi [prefissi](#sandbox-path-prefixes) delle impostazioni `sandbox.filesystem.*`, e Claude Code unisce gli array da ogni ambito di impostazioni che la sessione carica. [Protect credentials](/docs/it/sandboxing#protect-credentials) copre cosa si applica ancora dalle fonti che escludi con `--setting-sources`. `mask` entries richiedono Claude Code v2.1.221 o successivo.

La sostituzione `mask` viene eseguita solo attraverso il proxy della sandbox, quindi imposta [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate), o [`allowPlaintextInject`](#sandbox-credentials-allowplaintextinject) per le reti di test HTTP semplice. `mask` si applica a un singolo file, quindi elenca ogni file di credenziali individualmente. Claude Code accetta ma ignora i campi `mask` su una voce `deny`. [Mask credential files](/docs/it/sandboxing#mask-credential-files) copre quali fonti di impostazioni sono onorate e quando una voce ricade a `deny`.

<span id="sandbox-credentials-files-extract" />

<span id="sandbox-credentials-files-onextractnomatch" />

<span id="sandbox-credentials-files-decode" />

<span id="sandbox-credentials-files-maskclaims" />

<span id="sandbox-credentials-files-maskduplicates" />

<span id="sandbox-credentials-files-injecthosts" />

<h4 id="mask-fields-for-files">
  Campi mask per i file
</h4>

Una voce `mask` accetta questi campi opzionali. Senza `extract` o `decode`, Claude Code sostituisce l'intero contenuto del file con un sentinella. Su macOS con isolamento del filesystem attivo, Claude Code applica una voce `mask` come `deny` prima che `extract` o `decode` venga eseguito; vedi [Mask credential files](/docs/it/sandboxing#mask-credential-files).

| Campo              | Tipo                                                                                                                    | Cosa fa                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :----------------- | :---------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `extract`          | string, un'espressione regolare con almeno un gruppo di cattura                                                         | Maschera solo il testo catturato dal gruppo 1 di ogni corrispondenza, quindi il resto del file rimane analizzabile. Con `decode` anche impostato, Claude Code controlla ogni cattura come un possibile JWT invece di sostituirlo direttamente. Richiede v2.1.221 o successivo                                                                                                                                                                                                                                                                                                                                                        |
| `onExtractNoMatch` | `"warn"`, `"deny"` o `"error"`; predefinito `"warn"`                                                                    | Cosa succede quando `extract` o `decode` non trova nulla da mascherare. `warn` lascia il file leggibile così com'è all'interno della sandbox, `deny` lo rende illeggibile, e `error` ferma la configurazione della sandbox finché non fissi la configurazione. Claude Code tratta `deny` come `error` quando il blocco di lettura non sarebbe applicato, perché [disabiliti l'isolamento del filesystem](/docs/it/sandboxing#disable-filesystem-isolation) o una voce [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread) riapre il percorso. Richiede v2.1.221 o successivo; il caso `decode` richiede v2.1.224 o successivo |
| `decode`           | la string `"jwt"`                                                                                                       | Trova JSON Web Token (JWT) nel file, con un pattern incorporato o con `extract` quando impostato, verifica ogni candidato, e sostituiscilo con un token falso strutturalmente valido, quindi il codice all'interno della sandbox che decodifica il token continua a funzionare. Quando nessun candidato verifica, `onExtractNoMatch` governa il risultato. Richiede v2.1.224 o successivo                                                                                                                                                                                                                                            |
| `maskClaims`       | array di stringhe, almeno un nome di claim; richiede `decode`                                                           | Maschera solo i claim di payload di primo livello denominati all'interno di ogni JWT verificato e ricostruisci il token attorno al payload modificato, quindi gli altri claim rimangono leggibili. Quando nessun claim denominato corrisponde, `onExtractNoMatch` governa il risultato. Richiede v2.1.224 o successivo                                                                                                                                                                                                                                                                                                               |
| `maskDuplicates`   | Boolean, predefinito `false`                                                                                            | Sostituisci anche copie verbatim di ogni valore mascherato altrove nel file, come un segreto incollato in un commento. Claude Code corrisponde a sottostringhe grezze, quindi riservalo per segreti lunghi e ad alta entropia. Consultato solo quando `extract` o `decode` è impostato. Richiede v2.1.221 o successivo                                                                                                                                                                                                                                                                                                               |
| `injectHosts`      | array di stringhe, ognuno un host che [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) ammette anche | Restringi gli host dove il proxy della sandbox sostituisce il valore reale. Quando non impostato, il proxy lo sostituisce sulle richieste a ogni host in `sandbox.network.allowedDomains`. Richiede v2.1.221 o successivo                                                                                                                                                                                                                                                                                                                                                                                                            |

Questo maschera solo il valore `oauth_token` nel file host `gh`, sostituisce ogni altra copia di esso nel file, rende il file illeggibile se il pattern non corrisponde a nulla, e sostituisce il token reale solo sulle richieste a `api.github.com`:

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

Proteggi le variabili di ambiente dai comandi in sandbox. Con `"mode": "deny"`, Claude Code rimuove la variabile dall'ambiente dei comandi in sandbox. Con `"mode": "mask"`, i comandi in sandbox vedono un valore sentinella per sessione, e il proxy della sandbox sostituisce il valore reale sulle richieste in uscita a `injectHosts` di quella voce, quindi gli strumenti come `gh` e `npm` continuano ad autenticarsi senza mai tenere la credenziale reale. `"mode": "mask"` richiede Claude Code v2.1.199 o successivo.

* **Scope**: [`Any file`](#scopes). Claude Code scarta le voci `mask` da `.claude/settings.json` del progetto e da `.claude/settings.local.json` locale.
* **Type**: array di object, ognuno con `name` e una `mode` di `"deny"` o `"mask"`, più i [campi mask opzionali per le variabili di ambiente](#mask-fields-for-environment-variables)
* **Default**: non impostato, quindi nessuna variabile di ambiente è protetta

Questo rimuove `NPM_TOKEN` dai comandi in sandbox e maschera `GITHUB_TOKEN`, sostituendo il valore reale solo sulle richieste a `api.github.com`:

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

Il `name` deve iniziare con una lettera o un underscore e contenere solo lettere, cifre e underscore. Claude Code unisce gli array da ogni ambito di impostazioni che la sessione carica, e applica `deny` quando la stessa variabile appare con entrambe le modalità. [Protect credentials](/docs/it/sandboxing#protect-credentials) copre cosa si applica ancora dalle fonti che escludi con `--setting-sources`. `mask` entries richiedono Claude Code v2.1.199 o successivo.

La sostituzione `mask` viene eseguita solo attraverso il proxy della sandbox, quindi imposta [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate), o [`allowPlaintextInject`](#sandbox-credentials-allowplaintextinject) per le reti di test HTTP semplice; vedi [Mask environment variables](/docs/it/sandboxing#mask-environment-variables). Claude Code accetta ma ignora i campi `mask` su una voce `deny`.

<span id="sandbox-credentials-envvars-extract" />

<span id="sandbox-credentials-envvars-onextractnomatch" />

<span id="sandbox-credentials-envvars-decode" />

<span id="sandbox-credentials-envvars-maskclaims" />

<span id="sandbox-credentials-envvars-injecthosts" />

<h4 id="mask-fields-for-environment-variables">
  Campi mask per le variabili di ambiente
</h4>

Una voce `mask` accetta questi campi opzionali. Senza `extract` o `decode`, Claude Code sostituisce l'intero valore con un sentinella. `extract` e `decode` non possono essere combinati sulla stessa voce.

| Campo              | Tipo                                                                                                                    | Cosa fa                                                                                                                                                                                                                                                                                                                                                                                                                       |
| :----------------- | :---------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `extract`          | string, un'espressione regolare con almeno un gruppo di cattura                                                         | Maschera solo il testo catturato dal gruppo 1 di ogni corrispondenza, come la password all'interno di una stringa di connessione `DATABASE_URL`, quindi il resto del valore rimane analizzabile. Richiede v2.1.224 o successivo                                                                                                                                                                                               |
| `onExtractNoMatch` | `"warn"`, `"deny"` o `"error"`; predefinito `"warn"`. Su una voce con `decode`, solo `"warn"` è accettato               | Cosa succede quando `extract` non corrisponde a nulla. `warn` passa la variabile attraverso senza mascherare, `deny` la annulla all'interno della sandbox, e `error` ferma la configurazione della sandbox finché non fissi la configurazione. Richiede v2.1.224 o successivo                                                                                                                                                 |
| `decode`           | la string `"jwt"`                                                                                                       | Verifica che l'intero valore sia un JWT e sostituiscilo con un token falso strutturalmente valido, quindi il codice all'interno della sandbox che decodifica il token continua a funzionare; il proxy sostituisce l'intero token reale all'uscita. Un valore che non verifica passa attraverso senza mascherare con un avviso. Richiede v2.1.224 o successivo                                                                 |
| `maskClaims`       | array di stringhe, almeno un nome di claim; richiede `decode`                                                           | Maschera solo i claim di payload di primo livello denominati all'interno del JWT decodificato e ricostruisci il token attorno al payload modificato, quindi gli altri claim rimangono leggibili. Quando nessun claim denominato corrisponde, la variabile passa attraverso senza mascherare con un avviso. Richiede v2.1.224 o successivo                                                                                     |
| `injectHosts`      | array di stringhe, ognuno un host che [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) ammette anche | Restringi gli host dove il proxy della sandbox sostituisce il valore reale. Quando non impostato, il proxy lo sostituisce sulle richieste a ogni host in `sandbox.network.allowedDomains`. Scrivi una destinazione IPv6 come l'indirizzo compresso nudo, come `"::1"`, non la forma tra parentesi; vedi [IPv6 destinations in `injectHosts`](/docs/it/sandboxing#ipv6-destinations-in-injecthosts). Richiede v2.1.199 o successivo |

Questo maschera solo la password all'interno di `DATABASE_URL`, annulla la variabile se il pattern non corrisponde a nulla, e maschera un JWT in `SERVICE_JWT` mentre lascia ogni claim tranne `api_key` leggibile:

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

Consenti la sostituzione `mask` anche sulle richieste HTTP semplice oltre a HTTPS con terminazione TLS. Su HTTP semplice l'identità upstream non è verificata e la credenziale viaggia in testo in chiaro, quindi lascia questo disattivato al di fuori delle reti di test affidabili. Richiede Claude Code v2.1.199 o successivo.

* **Scope**: [`User or managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code consente la sostituzione `mask` anche sulle richieste HTTP semplice oltre a HTTPS con terminazione TLS
  * `false`: Claude Code consente la sostituzione `mask` solo su HTTPS con terminazione TLS
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "allowPlaintextInject": true
    }
  }
}
```

Richiede Claude Code v2.1.199 o successivo.

<h3 id="sandbox-credentials-awspairs">
  `sandbox.credentials.awsPairs`
</h3>

Raggruppa le variabili di ambiente mascherate che formano una credenziale AWS per la [ri-firma SigV4](/docs/it/sandboxing#re-sign-aws-requests) quando la tua credenziale vive in variabili con nomi non standard. Claude Code collega automaticamente il trio convenzionale `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` e `AWS_SESSION_TOKEN` quando mascheri i loro interi valori, quindi hai bisogno di questa chiave solo per altri nomi. Richiede Claude Code v2.1.224 o successivo.

* **Scope**: [`User or managed`](#scopes)
* **Type**: array di object, ognuno con `accessKeyIdVar`, `secretAccessKeyVar` e opzionalmente `sessionTokenVar`, nominando le voci `sandbox.credentials.envVars`
* **Default**: non impostato, quindi solo il trio convenzionale è accoppiato

Questo collega tre variabili con nomi personalizzati in una credenziale AWS per la ri-firma:

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

Ogni variabile denominata deve essere una voce `mask` di valore intero in [`sandbox.credentials.envVars`](#sandbox-credentials-envvars), senza `extract` o `decode`, e può riempire solo uno slot in tutte le coppie.

<h3 id="sandbox-credentials-sigv4">
  `sandbox.credentials.sigv4`
</h3>

Scegli cosa fa il proxy della sandbox con i moduli di richiesta AWS che [non può ri-firmare](/docs/it/sandboxing#re-sign-aws-requests): `streaming` per i caricamenti di streaming aws-chunked, `presigned` per gli URL pre-firmati, e `sigv4a` per le firme asimmetriche SigV4A. Questo si applica solo alle richieste firmate con l'ID della chiave di accesso segnaposto di una coppia mascherata. Richiede Claude Code v2.1.224 o successivo.

* **Scope**: [`User or managed`](#scopes)
* **Type**: object con `streaming`, `presigned` e `sigv4a`, ognuno uno di:
  * `"deny"`: il proxy fallisce la richiesta
  * `"passthrough"`: il proxy invia la richiesta firmata con il segnaposto mascherato, quindi lo strumento riceve il rifiuto di AWS
* **Default**: non impostato, quindi ogni modulo è `"deny"`

Questo invia i caricamenti di streaming invece di farli fallire al proxy:

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

Con `deny`, il proxy fallisce la richiesta. Con `passthrough`, il proxy invia la richiesta con la sua firma calcolata dal segnaposto mascherato, quindi AWS la rifiuta e lo strumento che chiama riceve la risposta di AWS stessa invece di un errore del proxy.

<h3 id="sandbox-network">
  `sandbox.network`
</h3>

Controlla quali host, porte e socket i comandi in sandbox possono raggiungere. La sandbox instrada il traffico in uscita attraverso un proxy che applica questi elenchi; vedi [Network isolation](/docs/it/sandboxing#network-isolation) per come il proxy decide e quando richiede.

* **Scope**: [`Any file`](#scopes). `strictAllowlist`, `allowManagedDomainsOnly` e `tlsTerminate` vengono letti da meno fonti, come dicono le loro voci.
* **Type**: object con le sotto-chiavi di seguito
* **Default**: non impostato, quindi nessun dominio è pre-consentito e la sandbox richiede per ogni nuovo host

Questo pre-consente GitHub e npm, blocca `uploads.github.com` e consente ai comandi di associarsi a localhost:

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

Claude Code unisce le sotto-chiavi array in ambiti di impostazioni e le deduplica, quindi un progetto può aggiungere domini al tuo elenco utente. Le regole di autorizzazione `WebFetch(domain:...)` allow e deny [permission rules](/docs/it/sandboxing#permission-rules) alimentano gli stessi elenchi allow e deny.

<h3 id="sandbox-network-allowunixsockets">
  `sandbox.network.allowUnixSockets`
</h3>

Elenca i percorsi dei socket Unix che i comandi in sandbox possono connettere su macOS. Claude Code ignora questo elenco su Linux e WSL2, dove il filtro seccomp non può ispezionare i percorsi dei socket; usa [`allowAllUnixSockets`](#sandbox-network-allowallunixsockets) invece.

* **Scope**: [`Any file`](#scopes)
* **Type**: array di stringhe, ognuno un percorso di socket
* **Default**: non impostato, quindi la sandbox macOS blocca ogni socket Unix

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowUnixSockets": ["~/.ssh/agent-socket"]
    }
  }
}
```

Un percorso di socket può concedere un accesso ampio: consentire `/var/run/docker.sock`, ad esempio, consente a un comando in sandbox di controllare il daemon Docker. Vedi [Security limitations](/docs/it/sandboxing#security-limitations).

<h3 id="sandbox-network-allowallunixsockets">
  `sandbox.network.allowAllUnixSockets`
</h3>

Consenti ai comandi in sandbox di connettersi a ogni socket Unix. Su Linux e WSL2, il [filtro seccomp](/docs/it/sandboxing#set-up-linux-and-wsl2) della sandbox blocca le chiamate `socket(AF_UNIX, ...)`, quindi questo è l'unico modo per consentire i socket Unix lì. Quando il filtro è mancante, che `/sandbox` segnala sulla sua scheda Dependencies, la sandbox non blocca le chiamate ai socket Unix. Vedi [Set up Linux and WSL2](/docs/it/sandboxing#set-up-linux-and-wsl2) per dove viene il filtro.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: i comandi in sandbox possono connettersi a ogni socket Unix
  * `false`: la sandbox blocca le connessioni ai socket Unix: su macOS tranne i percorsi in `allowUnixSockets`, e su Linux e WSL2 attraverso il filtro seccomp quando è presente
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowAllUnixSockets": true
    }
  }
}
```

Su WSL2, `true` riapre anche il socket interop che lancia binari Windows come `cmd.exe` e `powershell.exe`.

<h3 id="sandbox-network-allowlocalbinding">
  `sandbox.network.allowLocalBinding`
</h3>

Consenti ai comandi in sandbox di associarsi alle porte localhost su macOS, ad esempio per avviare un server di sviluppo.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: i comandi in sandbox possono associarsi alle porte localhost su macOS
  * `false`: i comandi in sandbox su macOS non possono associarsi alle porte localhost
* **Default**: `false`

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

Elenca i nomi di servizio XPC e Mach aggiuntivi che la sandbox macOS può cercare. Gli strumenti che comunicano su XPC, come il Simulatore iOS o Playwright, hanno bisogno che i loro servizi siano elencati qui.

* **Scope**: [`Any file`](#scopes)
* **Type**: array di stringhe, ognuno un nome di servizio; un singolo `*` finale corrisponde a un prefisso, e `"*"` da solo corrisponde a ogni servizio
* **Default**: non impostato

Questo consente ogni servizio sotto il prefisso `com.apple.coresimulator.`:

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

Pre-consenti i domini per il traffico in uscita dai comandi in sandbox, quindi la sandbox non li richiede. I wildcard come `*.example.com` corrispondono ai sottodomini, e un suffisso opzionale `:port` limita una voce a una porta; una voce senza una porta corrisponde a ogni porta.

* **Scope**: [`Any file`](#scopes). Solo impostazioni gestite quando [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) è impostato.
* **Type**: array di stringhe, ognuno un dominio, pattern con wildcard o letterale IP, con un suffisso opzionale `:port`
* **Default**: non impostato, quindi la sandbox richiede la prima volta che un comando raggiunge un nuovo host

Questo pre-consente GitHub su ogni porta, ogni sottodominio npm e un host API su porta 443 solo:

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org", "api.example.com:443"]
    }
  }
}
```

Scrivi i letterali IPv6 tra parentesi, con una porta opzionale: `"[::1]"` consente ogni porta e `"[::1]:443"` una porta. La forma tra parentesi richiede Claude Code v2.1.229 o successivo. Vedi [IPv6 addresses in domain lists](/docs/it/sandboxing#ipv6-addresses-in-domain-lists).

<h3 id="sandbox-network-denieddomains">
  `sandbox.network.deniedDomains`
</h3>

Blocca i domini per il traffico in uscita dai comandi in sandbox, usando la stessa sintassi di wildcard, porta e IPv6 di [`allowedDomains`](#sandbox-network-alloweddomains). Un dominio negato rimane bloccato anche quando una voce `allowedDomains` lo corrisponde anche.

* **Scope**: [`Any file`](#scopes)
* **Type**: array di stringhe, ognuno un dominio, pattern con wildcard o letterale IP, con un suffisso opzionale `:port`
* **Default**: non impostato

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "deniedDomains": ["sensitive.cloud.example.com"]
    }
  }
}
```

Claude Code unisce questo elenco da ogni fonte di impostazioni che la sessione carica anche quando `allowManagedDomainsOnly` è impostato, quindi uno sviluppatore può sempre stringere l'elenco di negazione. Per i letterali IPv6, vedi [IPv6 addresses in domain lists](/docs/it/sandboxing#ipv6-addresses-in-domain-lists).

Una voce scritta con il punto finale che marca un nome di dominio completamente qualificato, come `example.com.`, blocca le stesse connessioni di `example.com`.

<h3 id="sandbox-network-strictallowlist">
  `sandbox.network.strictAllowlist`
</h3>

Nega ai comandi in sandbox l'accesso agli host al di fuori dell'elenco consentito invece di richiedere l'approvazione. L'elenco consentito è [`allowedDomains`](#sandbox-network-alloweddomains) più i domini dalle regole allow `WebFetch(domain:...)`, o solo le voci delle impostazioni gestite quando [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) è impostato. Richiede Claude Code v2.1.219 o successivo.

* **Scope**: [`User or managed`](#scopes). Un repository non può attivarlo o disattivarlo.
* **Type**: Boolean
  * `true`: Claude Code nega ai comandi in sandbox l'accesso agli host al di fuori dell'elenco consentito
  * `false`: a meno che un altro file di impostazioni affidabile non imposti `true`, Claude Code decide un host al di fuori dell'elenco consentito in base alla modalità di autorizzazione invece di negarlo direttamente: controlla l'host rispetto ai [domini consentiti per comando in modalità auto](/docs/it/sandboxing#per-command-allowed-domains-in-auto-mode) in modalità auto, nega in modalità `dontAsk`, consente in modalità `bypassPermissions` e in plan mode quando il bypass è disponibile, e altrimenti ti chiede
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "strictAllowlist": true
    }
  }
}
```

Claude Code applica questo solo per i comandi in sandbox; gli strumenti in-process come `WebFetch` seguono comunque le loro [regole di autorizzazione](/docs/it/sandboxing#permission-rules). Quando una qualsiasi delle fonti onorate lo imposta su `true`, rimane attivo. Vedi [Network isolation](/docs/it/sandboxing#network-isolation). Richiede Claude Code v2.1.219 o successivo.

<h3 id="sandbox-network-allowmanageddomainsonly">
  `sandbox.network.allowManagedDomainsOnly`
</h3>

Blocca l'elenco consentito di rete a ciò che le impostazioni gestite definiscono. Claude Code quindi onora solo `allowedDomains` e le regole allow `WebFetch(domain:...)` dalle impostazioni gestite, ignora i domini dalle impostazioni utente, progetto, locale e `--settings`, e blocca automaticamente un dominio non consentito invece di richiedere.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code onora solo `allowedDomains` e le regole allow `WebFetch(domain:...)` dalle impostazioni gestite e blocca automaticamente un dominio non consentito invece di richiedere
  * `false`: i domini dalle impostazioni utente, progetto, locale e `--settings` si uniscono all'elenco consentito
* **Default**: `false`

Questo blocca l'elenco consentito a GitHub e npm e ignora qualsiasi dominio che gli sviluppatori aggiungono:

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

I domini negati si uniscono comunque da ogni fonte che la sessione carica. Vedi [Keep developers from widening the policy](/docs/it/sandboxing#keep-developers-from-widening-the-policy).

<h3 id="sandbox-network-httpproxyport">
  `sandbox.network.httpProxyPort`
</h3>

Punta la sandbox al tuo proxy HTTP invece di quello che Claude Code esegue. Le organizzazioni lo fanno per ispezionare il traffico HTTPS, applicare le loro regole di filtro o registrare ogni richiesta. Quando non impostato, Claude Code avvia il suo proxy per il traffico HTTP.

* **Scope**: [`Any file`](#scopes)
* **Type**: number, una porta TCP locale
* **Default**: non impostato, quindi Claude Code esegue il suo proxy

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "httpProxyPort": 8080
    }
  }
}
```

Imposta anche [`socksProxyPort`](#sandbox-network-socksproxyport) se il tuo proxy dovrebbe portare il traffico SOCKS anche; con solo uno dei due impostato, Claude Code continua a eseguire il suo proxy per l'altro protocollo. Vedi [Custom proxy configuration](/docs/it/sandboxing#custom-proxy-configuration).

<h3 id="sandbox-network-socksproxyport">
  `sandbox.network.socksProxyPort`
</h3>

Punta la sandbox al tuo proxy SOCKS5 invece di quello che Claude Code esegue. Quando non impostato, Claude Code avvia il suo proxy per il traffico SOCKS.

* **Scope**: [`Any file`](#scopes)
* **Type**: number, una porta TCP locale
* **Default**: non impostato, quindi Claude Code esegue il suo proxy

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "socksProxyPort": 8081
    }
  }
}
```

Vedi [Custom proxy configuration](/docs/it/sandboxing#custom-proxy-configuration).

<h3 id="sandbox-network-tlsterminate">
  `sandbox.network.tlsTerminate`
</h3>

Fai terminare il proxy della sandbox TLS in modo che possa leggere i contenuti delle richieste HTTPS. Questo è sperimentale, e la [sostituzione di credenziali](/docs/it/sandboxing#mask-credentials) `mask` lo richiede. Imposta `{}` per generare un'autorità di certificazione effimera per la sessione, o imposta `caCertPath` e `caKeyPath` per usare la tua.

* **Scope**: [`User or managed`](#scopes). Un repository non può attivarlo o fornire un'autorità di certificazione.
* **Type**: object con stringhe opzionali `caCertPath` e `caKeyPath`, ognuno un percorso di file
* **Default**: non impostato, quindi il proxy non termina o ispeziona TLS

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "tlsTerminate": {}
    }
  }
}
```

Quando più di una fonte onorata lo imposta, Claude Code usa il valore dalla fonte con la precedenza più alta: impostazioni gestite, quindi il flag `--settings`, quindi impostazioni utente. Richiede Claude Code v2.1.199 o successivo.

<span id="context-and-memory" />

<h2 id="memory-and-context">
  Memoria e contesto
</h2>

Controlla cosa Claude Code carica nel contesto, come lo compatta e dove mantiene la memoria e i piani. Vedi [Gestisci contesto](/docs/it/context-window) e [Memoria](/docs/it/memory).

<h3 id="autocompactenabled">
  `autoCompactEnabled`
</h3>

Fai in modo che Claude Code [compatti la conversazione automaticamente](/docs/it/context-window#when-your-context-fills-up) quando il contesto si avvicina al limite. Appare in `/config` come **Auto-compact**, e attivarlo/disattivarlo lì scrive questa chiave nelle tue impostazioni utente.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code compatta la conversazione automaticamente quando il contesto si avvicina al limite
  * `false`: Claude Code non compatta automaticamente
* **Default**: `true`
* **Per-session overrides**: [`DISABLE_AUTO_COMPACT`](/docs/it/env-vars) disattiva l'auto-compact per una sessione; quale dei due lo disattiva, l'altro non può riattivarlo

```json settings.json theme={null}
{
  "autoCompactEnabled": false
}
```

Il comando manuale `/compact` continua a funzionare mentre l'auto-compact è disattivato.

<h3 id="autocompactwindow">
  `autoCompactWindow`
</h3>

Imposta quanto pieno diventa il contesto prima che Claude Code [compatti automaticamente](/docs/it/context-window#when-your-context-fills-up).

* **Scope**: [`Any file`](#scopes)
* **Type**: numero di token, da `100000` a `1000000`. Claude Code limita il valore alla finestra di contesto del tuo modello; la [panoramica dei modelli](https://platform.claude.com/docs/en/about-claude/models/overview) elenca la finestra di ogni modello
* **Default**: non impostato, quindi Claude Code sceglie una finestra ottimizzata per il tuo modello
* **Per-session overrides**: [`--autocompact`](/docs/it/cli-reference#cli-flags) ha la precedenza su questa chiave per una sessione, e [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/it/env-vars) ha la precedenza su entrambi

```json settings.json theme={null}
{
  "autoCompactWindow": 500000
}
```

Impostalo con il comando [`/autocompact`](/docs/it/commands#all-commands), che scrive questa chiave nelle tue impostazioni utente. [Imposta la finestra di auto-compact](/docs/it/model-config#set-the-auto-compact-window) spiega come il comando, il flag, la variabile e l'impostazione interagiscono.

<h3 id="automemorydirectory">
  `autoMemoryDirectory`
</h3>

Archivia la [memoria automatica](/docs/it/memory#storage-location) in una directory di tua scelta invece del valore predefinito per progetto.

* **Scope**: [`Any file`](#scopes)
* **Type**: stringa, un percorso di directory assoluto o con prefisso `~/`
* **Default**: non impostato, quindi Claude Code utilizza `~/.claude/projects/<project>/memory/`

```json settings.json theme={null}
{
  "autoMemoryDirectory": "~/my-memory-dir"
}
```

Dalle impostazioni di progetto o locali, Claude Code rispetta questa chiave secondo la stessa [regola di fiducia dell'area di lavoro degli hook](/docs/it/permissions#what-runs-before-you-trust-a-folder), poiché un repository clonato può fornire questi file.

<h3 id="automemoryenabled">
  `autoMemoryEnabled`
</h3>

Attiva o disattiva la [memoria automatica](/docs/it/memory#enable-or-disable-auto-memory). Quando `false`, Claude non legge da o scrive nella directory di memoria automatica. Puoi anche attivarlo/disattivarlo con `/memory` durante una sessione, che scrive questa chiave nelle tue impostazioni utente.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: lo stesso di non impostato; la memoria automatica rimane attiva a meno che qualcosa che ha la precedenza su questa chiave non la disattivi per la sessione, come `--bare`, modalità sicura, o `CLAUDE_CODE_DISABLE_AUTO_MEMORY`
  * `false`: Claude non legge da o scrive nella directory di memoria automatica
* **Default**: `true`
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_AUTO_MEMORY`](/docs/it/env-vars) ha la precedenza su questa chiave per una sessione, in entrambe le direzioni

```json settings.json theme={null}
{
  "autoMemoryEnabled": false
}
```

<h3 id="bashoutputmaxchars">
  `bashOutputMaxChars`
</h3>

Imposta quanti caratteri dell'output di un comando Bash o PowerShell riuscito [Claude riceve inline](/docs/it/tools-reference#output-limits). Quando l'output supera il limite, Claude Code lo salva in un file e Claude riceve un'anteprima breve più il percorso del file. Aumenta il limite quando l'output del comando, come una build dettagliata o un log completo della suite di test, regolarmente supera il valore predefinito e vuoi che Claude lo legga senza aprire il file. Richiede Claude Code v2.1.261 o successivo.

* **Scope**: [`Any file`](#scopes)
* **Type**: numero di caratteri, un intero positivo. Claude Code limita il valore nell'intervallo `4000` a `128000`
* **Default**: non impostato, quindi Claude riceve fino a 30.000 caratteri inline

```json settings.json theme={null}
{
  "bashOutputMaxChars": 100000
}
```

Quando imposti questa chiave, Claude Code ignora la variabile di ambiente [`BASH_MAX_OUTPUT_LENGTH`](/docs/it/env-vars).

<h3 id="claudemd">
  `claudeMd`
</h3>

Inietta istruzioni in stile CLAUDE.md come memoria gestita dall'organizzazione senza distribuire un file separato. Claude Code carica il testo come voce di memoria gestita prima dei file CLAUDE.md utente e di progetto.

* **Scope**: [`Managed`](#scopes)
* **Type**: stringa, il testo di un file CLAUDE.md; scrivilo come faresti con il file, Markdown incluso, con interruzioni di riga come `\n`
* **Default**: non impostato

Questo esempio distribuisce due regole come un breve elenco Markdown:

```json managed-settings.json theme={null}
{
  "claudeMd": "# Engineering rules\n\n- Always run make lint before committing.\n- Never push directly to main."
}
```

Vedi [Distribuisci CLAUDE.md a livello di organizzazione](/docs/it/memory#deploy-organization-wide-claude-md).

<h3 id="claudemdexcludes">
  `claudeMdExcludes`
</h3>

Salta file `CLAUDE.md` specifici quando Claude Code carica la [memoria](/docs/it/memory#exclude-specific-claude-md-files). In un grande monorepo, usalo per saltare file CLAUDE.md da altri team che non sono rilevanti per il tuo lavoro; [Escludi file CLAUDE.md irrilevanti](/docs/it/large-codebases#exclude-irrelevant-claude-md-files) nella guida dei grandi codebase spiega quel caso. I pattern corrispondono ai percorsi di file assoluti.

* **Scope**: [`Any file`](#scopes)
* **Type**: array di stringhe, ciascuna un pattern glob o un percorso assoluto
* **Default**: non impostato, quindi Claude Code carica ogni CLAUDE.md che trova

```json settings.json theme={null}
{
  "claudeMdExcludes": ["**/vendor/**/CLAUDE.md"]
}
```

Le esclusioni si applicano solo ai file di memoria utente, progetto e locale; i file CLAUDE.md della politica gestita non possono essere esclusi.

<span id="environment-variables" />

<h3 id="env">
  `env`
</h3>

Imposta variabili di ambiente per ogni sessione e per i sottoprocessi che Claude Code avvia da essa. La maggior parte delle variabili nel [riferimento delle variabili di ambiente](/docs/it/env-vars) può andare qui, che è come applichi una a ogni sessione o la distribuisci al tuo team. Le impostazioni di progetto e locali non possono impostare [alcune di esse](#variables-claude-code-ignores-in-env).

* **Scope**: [`Any file`](#scopes)
* **Type**: oggetto che mappa i nomi delle variabili ai valori stringa
* **Default**: non impostato

Questo esempio disattiva la compattazione automatica e instrada le richieste API attraverso un proxy:

```json settings.json theme={null}
{
  "env": {
    "DISABLE_AUTO_COMPACT": "1",
    "ANTHROPIC_BASE_URL": "https://proxy.example.com"
  }
}
```

<h4 id="how-env-values-interact-with-your-shell">
  Come i valori di `env` interagiscono con la tua shell
</h4>

* Un valore qui sovrascrive la stessa variabile esportata nella tua shell, e quando più di un file di impostazioni imposta una variabile, si applica quello con la [precedenza più alta](/docs/it/settings#settings-precedence). [Variabili che Claude Code ignora in `env`](#variables-claude-code-ignores-in-env) elenca le eccezioni per le impostazioni di progetto e locali.
* Per annullare un'esportazione della shell, imposta la variabile su `""`. Claude Code tratta un valore vuoto come non impostato per la selezione del provider, e i sottoprocessi ereditano il valore vuoto.
* `NO_COLOR` e `FORCE_COLOR` impostati qui raggiungono solo i sottoprocessi. Per cambiare i colori dell'interfaccia di Claude Code stesso, impostali nella tua shell prima di lanciare `claude`.
* I valori qui sono testo semplice nel file di impostazioni e raggiungono ogni sottoprocesso che Claude Code avvia. Per un token bearer OTLP che ruota, usa [`otelHeadersHelper`](#otelheadershelper); per le credenziali API, usa [`apiKeyHelper`](#apikeyhelper).

<h4 id="when-claude-code-applies-env-values">
  Quando Claude Code applica i valori di `env`
</h4>

* Dalle impostazioni utente, `--settings` e impostazioni gestite: all'avvio, e di nuovo nella sessione in esecuzione quando una modifica salvata altera l'`env` unito.
* Dalle impostazioni di progetto e locali: dopo che hai fiducia dell'area di lavoro, o all'avvio in modalità `-p`, che non mostra mai la finestra di dialogo di fiducia, e di nuovo quando una modifica salvata altera l'`env` unito.
* Variabili che Claude Code classifica come sicure, come la selezione del modello, timeout e limiti, e interruttori di funzionalità: all'avvio da ogni file di impostazioni, a parte le [variabili che le impostazioni di progetto e locali non possono impostare](#variables-claude-code-ignores-in-env).
* Dopo che [sposti la sessione con `/cd`](/docs/it/permissions#move-the-session-to-another-directory) su v2.1.246 o successivo: i valori di `env` della nuova directory di progetto e locali, in aggiunta a quelli della directory precedente.

<h4 id="variables-claude-code-ignores-in-env">
  Variabili che Claude Code ignora in `env`
</h4>

* Le impostazioni di progetto e locali non possono impostare variabili che un repository estratto non dovrebbe controllare; impostale nella tua shell, impostazioni utente o impostazioni gestite invece. Claude Code elimina ciascuna, a parte alcuni valori che disattivano la telemetria, e registra un avviso che puoi vedere con `claude --debug`. Includono:

  * Variabili che scelgono dove Claude Code archivia o scrive i suoi file: `CLAUDE_CONFIG_DIR`, `CLAUDE_CODE_TMPDIR` e le variabili di directory del sistema operativo come `HOME`, `TMPDIR`, `TMP`, `TEMP` e la famiglia `XDG_*`.
  * Variabili che esportano il contenuto della sessione: [`OTEL_LOG_RAW_API_BODIES`](/docs/it/env-vars#variables) e la coppia di tracciamento beta dettagliata `ENABLE_BETA_TRACING_DETAILED` e `BETA_TRACING_ENDPOINT`.
  * Le variabili dell'[esportatore OpenTelemetry](/docs/it/monitoring-usage) che attivano la telemetria, scelgono dove va, o scelgono quale contenuto cattura:

    * `CLAUDE_CODE_ENABLE_TELEMETRY`, più la coppia beta di telemetria migliorata `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` e `ENABLE_ENHANCED_TELEMETRY_BETA`
    * I selettori dell'esportatore `OTEL_LOGS_EXPORTER`, `OTEL_METRICS_EXPORTER` e `OTEL_TRACES_EXPORTER`
    * Le variabili di contenuto `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_ASSISTANT_RESPONSES`, `OTEL_LOG_TOOL_CONTENT` e `OTEL_LOG_TOOL_DETAILS`
    * Variabili `OTEL_EXPORTER_OTLP_*` i cui nomi terminano in `_ENDPOINT`, `_HEADERS`, `_PROTOCOL`, `_CERTIFICATE`, `_CLIENT_KEY` o `_INSECURE`, nelle forme generiche e per segnale, come `OTEL_EXPORTER_OTLP_ENDPOINT` e `OTEL_EXPORTER_OTLP_METRICS_HEADERS`
    * `OTEL_EXPORTER_PROMETHEUS_HOST` e `OTEL_EXPORTER_PROMETHEUS_PORT`

    Solo questi valori si applicano ancora dalle impostazioni di progetto e locali, perché disattivano qualcosa: `none` per i tre selettori dell'esportatore, e un valore disattivato come `0` per `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_TOOL_CONTENT` e `OTEL_LOG_TOOL_DETAILS`. Tale valore sostituisce la stessa variabile nelle tue impostazioni utente, ma non una che l'ambiente da cui avvii Claude Code, un file `--settings` o impostazioni gestite impostano.

    Quando un file di impostazioni di progetto o locale imposta una variabile in questo gruppo, una sessione interattiva locale mostra un avviso all'avvio. Esegui `/status` o `claude doctor` per vedere quali Claude Code ha ignorato e quali hanno disattivato la telemetria; entrambi elencano i nomi, mai i valori. Un'esecuzione non interattiva con `-p` o una sessione Agent SDK non mostra alcun avviso, quindi controlla che il tuo raccoglitore riceva ancora dati dopo l'aggiornamento. Se non lo fa, imposta le variabili nelle tue impostazioni utente, impostazioni gestite, l'ambiente del lavoro, o un file che passi con `--settings`.

    Ignorare questo gruppo nelle impostazioni di progetto e locale richiede Claude Code v2.1.282 o successivo.
  * Variabili che cambiano come Claude Code si avvia o si sincronizza, come `CLAUDE_CODE_PROCESS_WRAPPER`, `CLAUDE_CODE_SYNC_SKILLS`, `CLAUDE_CODE_SYNC_PLUGINS`, `CLAUDE_CODE_PLUGIN_CACHE_DIR` e `CLAUDE_CODE_PLUGIN_SEED_DIR`.

  Prima di v2.1.251, le impostazioni di progetto e locali potevano impostare ogni variabile che questo elenco nomina tranne `HOME` e `XDG_CONFIG_HOME`.
* Variabili di identità che gli ambienti di hosting di Claude Code possiedono, come `CLAUDE_CODE_REMOTE` e `CLAUDE_CODE_ACCOUNT_UUID`, sono ignorate da ogni file.
* [`CLAUDE_CODE_MESSAGING_SOCKET` e `CLAUDE_CODE_MESSAGING_TOKEN`](/docs/it/env-vars#variables), che Claude Code esporta stesso, sono ignorate da ogni file. Ignorare la variabile socket richiede Claude Code v2.1.224 o successivo, e ignorare il token richiede v2.1.228 o successivo.
* [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/it/sessions#name-the-project-directory-yourself), che Claude Code legge solo dall'ambiente di avvio, è ignorata da ogni file; richiede v2.1.234 o successivo.
* [`CLAUDE_CODE_RESTRICTED`](/docs/it/env-vars#variables), che Claude Code legge solo dall'ambiente di avvio, è ignorata da ogni file.

<h3 id="filecheckpointingenabled">
  `fileCheckpointingEnabled`
</h3>

Fai in modo che Claude Code crei snapshot dei file prima di ogni modifica in modo che [`/rewind`](/docs/it/checkpointing) possa ripristinarli. Appare in `/config` come **Rewind code (checkpoints)**, e attivarlo/disattivarlo lì scrive questa chiave nelle tue impostazioni utente.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code crea snapshot dei file prima di ogni modifica in modo che `/rewind` possa ripristinarli
  * `false`: Claude Code non crea snapshot dei file, quindi `/rewind` non può ripristinarli
* **Default**: `true`
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_FILE_CHECKPOINTING`](/docs/it/env-vars) disattiva il checkpointing per una sessione; quale dei due lo disattiva, l'altro non può riattivarlo

```json settings.json theme={null}
{
  "fileCheckpointingEnabled": false
}
```

In un'esecuzione `-p` o una sessione Agent SDK, Claude Code ignora questa chiave. L'SDK attiva il checkpointing con la sua opzione `enableFileCheckpointing`, e un'esecuzione `-p` nuda ha bisogno di `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=true`. Vedi [File checkpointing in Agent SDK](/docs/it/agent-sdk/file-checkpointing).

<h3 id="plansdirectory">
  `plansDirectory`
</h3>

Scegli dove Claude Code archivia i file di piano che scrive in [plan mode](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode). Claude Code risolve il percorso relativo alla radice del progetto e mantiene il valore predefinito quando il percorso si risolve al di fuori di esso.

* **Scope**: [`Any file`](#scopes)
* **Type**: stringa, un percorso relativo alla radice del progetto
* **Default**: non impostato, quindi Claude Code utilizza `~/.claude/plans`

```json settings.json theme={null}
{
  "plansDirectory": "./plans"
}
```

<h3 id="skilllistingbudgetfraction">
  `skillListingBudgetFraction`
</h3>

Ogni turno, Claude vede un [elenco delle tue skill](/docs/it/skills#skill-descriptions-are-cut-short) con le loro descrizioni, e Claude Code limita quell'elenco a una quota della finestra di contesto. Quando l'elenco supera il limite, Claude Code mantiene il nome di ogni skill ma elimina le descrizioni delle skill meno utilizzate, in modo che Claude possa ancora invocare quelle skill ma è meno probabile che ne scelga una da solo. Aumenta questa chiave per mantenere più descrizioni visibili al costo di più contesto per turno.

* **Scope**: [`Any file`](#scopes)
* **Type**: numero, una frazione maggiore di `0` e al massimo `1`
* **Default**: `0.01`, che riserva l'1% della finestra di contesto

```json settings.json theme={null}
{
  "skillListingBudgetFraction": 0.02
}
```

Per vedere quanto contesto utilizza l'elenco e quali skill contribuiscono di più, esegui `/doctor`.

<h3 id="skilllistingmaxdescchars">
  `skillListingMaxDescChars`
</h3>

Ogni turno, Claude vede un [elenco delle tue skill](/docs/it/skills#skill-descriptions-are-cut-short) che mostra il testo `description` e `when_to_use` di ogni skill. Questa chiave limita quanti caratteri di quel testo Claude Code mostra per skill; il testo più lungo viene tagliato al limite.

* **Scope**: [`Any file`](#scopes)
* **Type**: numero di caratteri, un intero positivo
* **Default**: `1536`

```json settings.json theme={null}
{
  "skillListingMaxDescChars": 2048
}
```

Aumentalo per mantenere le descrizioni lunghe intatte al costo di più contesto per turno; abbassalo per adattare più skill sotto [`skillListingBudgetFraction`](#skilllistingbudgetfraction).

<h3 id="taskoutputmaxchars">
  `taskOutputMaxChars`
</h3>

<Warning>
  Rimosso in v2.1.277, insieme allo strumento `TaskOutput` che lo dimensionava. Impostarlo non ha effetto sulle versioni attuali. Claude legge un [file di output](/docs/it/tools-reference#background-commands) di un compito in background con `Read` invece.
</Warning>

Fino a v2.1.276, impostavi questa chiave al numero di caratteri dell'output di un [compito in background](/docs/it/tools-reference#background-commands) che Claude riceveva inline quando leggeva il compito con lo strumento `TaskOutput`.

<h2 id="interface-and-terminal">
  Interfaccia e terminale
</h2>

Cambia come Claude Code appare e si comporta nel tuo terminale: tema, modalità editor, riga di stato, spinner, notifiche all'interno della sessione e accessibilità. Vedi [Configurazione del terminale](/docs/it/terminal-config).

<h3 id="askuserquestiontimeout">
  `askUserQuestionTimeout`
</h3>

Consenti a una finestra di dialogo [`AskUserQuestion`](/docs/it/tools-reference) senza risposta di continuare automaticamente dopo un periodo di inattività, inviando qualsiasi opzione tu avessi già selezionato. Impostalo quando ti allontani e vuoi che Claude continui senza di te. Con l'impostazione predefinita, le domande attendono fino a quando non le rispondi. Richiede Claude Code v2.1.200 o successivo.

* **Scope**: [`User or managed`](#scopes)
* **Type**: string, uno di `"60s"`, `"5m"`, `"10m"`, o `"never"`
* **Default**: `"never"`
* **Per-session overrides**: [`CLAUDE_AFK_TIMEOUT_MS`](/docs/it/env-vars) ha la precedenza su questa chiave per una sessione

```json settings.json theme={null}
{
  "askUserQuestionTimeout": "5m"
}
```

Appare in `/config` come **Question auto-continue timeout**, che scrive questa chiave nelle impostazioni utente; Claude Code nasconde la riga mentre le impostazioni gestite o il flag `--settings` impostano la chiave. Richiede Claude Code v2.1.200 o successivo.

<h3 id="autocontinueatusagelimit">
  `autoContinueAtUsageLimit`
</h3>

Dopo che un limite di utilizzo di claude.ai interrompe la tua sessione, attendi nella sessione aperta e continua l'attività automaticamente dopo il ripristino. Vedi [Disattiva continuazione automatica](/docs/it/interactive-mode#turn-automatic-continue-off). Richiede Claude Code v2.1.234 o successivo.

* **Scope**: [`User or managed`](#scopes). Leggi dalle impostazioni utente, `--settings` e impostazioni gestite solo. Quando nessuno di questi imposta la chiave, un file di impostazioni di progetto o locale che la imposta disattiva la funzione piuttosto che essere ignorato.
* **Type**: Boolean
  * `true`: dopo che un limite di utilizzo di claude.ai interrompe la tua sessione, Claude Code attende nella sessione aperta e continua l'attività automaticamente dopo il ripristino
  * `false`: Claude Code non avvia l'attesa da solo. Puoi comunque [avviare un'attesa tu stesso](/docs/it/interactive-mode#start-a-wait-yourself) dal menu delle opzioni del limite di utilizzo
* **Default**: `true`

```json settings.json theme={null}
{
  "autoContinueAtUsageLimit": false
}
```

Appare in `/config` come **Continue automatically at usage limit**, che scrive questa chiave nelle impostazioni utente; Claude Code nasconde la riga mentre le impostazioni gestite o il flag `--settings` impostano la chiave.

<h3 id="autoscrollenabled">
  `autoScrollEnabled`
</h3>

Segui il nuovo output fino in fondo alla conversazione nel [rendering a schermo intero](/docs/it/fullscreen). Disattivalo per rimanere dove hai fatto scorrere mentre Claude continua a lavorare; i prompt di autorizzazione scorrono comunque in vista.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: la conversazione segue il nuovo output fino in fondo
  * `false`: rimani dove hai fatto scorrere mentre Claude continua a lavorare; i prompt di autorizzazione appaiono comunque sotto la trascrizione
* **Default**: `true`

```json settings.json theme={null}
{
  "autoScrollEnabled": false
}
```

Appare in `/config` come **Auto-scroll** quando il rendering a schermo intero è attivo, che scrive questa chiave nelle impostazioni utente.

<h3 id="axscreenreader">
  `axScreenReader`
</h3>

Renderizza output compatibile con i lettori di schermo: testo piatto senza bordi decorativi o animazioni. La modalità lettore di schermo utilizza il renderer classico, quindi l'impostazione `tui` non ha effetto mentre è attiva; le [sessioni in background](/docs/it/agent-view) allegate eseguono comunque il rendering a schermo intero.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code renderizza testo piatto senza bordi decorativi o animazioni, utilizzando il renderer classico
  * `false`: Claude Code renderizza normalmente
* **Default**: unset, quindi la modalità lettore di schermo è disattivata
* **Per-session overrides**: [`--ax-screen-reader`](/docs/it/cli-reference#cli-flags) ha la precedenza su [`CLAUDE_AX_SCREEN_READER`](/docs/it/env-vars), e entrambi hanno la precedenza su questa chiave per una sessione

```json settings.json theme={null}
{
  "axScreenReader": true
}
```

<h3 id="basheditdiffenabled">
  `bashEditDiffEnabled`
</h3>

Scegli se Claude Code registra i file che un comando Bash modifica in un repository Git. Quando li registra, vedi il loro diff nel terminale dopo il comando, e i tuoi [hook Bash PostToolUse](/docs/it/hooks#bash) ricevono l'elenco dei file modificati.

Un file elencato non è sempre uno che il comando ha modificato. Una modifica che un altro programma o un'altra chiamata Bash ha fatto mentre il comando era in esecuzione può apparire lì anche.

Imposta la chiave su `true` per registrarli in ogni modalità di autorizzazione. Richiede Claude Code v2.1.269 o successivo.

* **Scope**: [`User or managed`](#scopes). Un `true` conta solo dalle tue impostazioni utente, JSON passato con `--settings`, o [impostazioni gestite](/docs/it/managed-settings), quindi un `true` nel `.claude/settings.json` o `.claude/settings.local.json` di un repository non può attivare la registrazione. Un `false` in uno qualsiasi dei file del repository disattiva comunque la registrazione a meno che un file con [precedenza più alta](/docs/it/settings#settings-precedence) non imposti `true`.
* **Type**: Boolean
* **Default**: unset, quindi Claude Code registra le modifiche in modalità auto e modalità `bypassPermissions` quando indirizza Claude a modificare i file tramite Bash
* **Per-session overrides**: [`CLAUDE_CODE_BASH_EDIT_DIFF`](/docs/it/env-vars) ha la precedenza su questa chiave per una sessione

```json settings.json theme={null}
{
  "bashEditDiffEnabled": true
}
```

<h3 id="companyannouncements">
  `companyAnnouncements`
</h3>

Mostra gli annunci della tua organizzazione agli utenti all'avvio. Quando ne elenchi più di uno, Claude Code ne sceglie uno a caso per ogni sessione; al primo avvio di una persona mostra la prima voce.

* **Scope**: [`Any file`](#scopes)
* **Type**: array di stringhe
* **Default**: unset, quindi nessun annuncio viene visualizzato

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

Scegli se Bash o PowerShell esegue i comandi shell che digiti con il prefisso [`!`](/docs/it/interactive-mode#shell-mode-with-prefix) nella casella di input, quelli che Claude Code esegue direttamente e aggiunge alla sessione.

`"powershell"` funziona solo mentre lo [strumento PowerShell](/docs/it/tools-reference#powershell-tool) è attivo. Lo strumento è attivo per impostazione predefinita su Windows senza Git Bash, e su Windows con Git Bash per account claude.ai e Console. Nelle sessioni Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry, e su macOS, Linux e WSL, imposta `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` per attivare lo strumento. Imposta quella variabile su `0` per disattivare lo strumento.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, uno di:
  * `"bash"`: Claude Code esegue i tuoi comandi `!` in Bash
  * `"powershell"`: Claude Code esegue i tuoi comandi `!` in PowerShell
* **Default**: `"bash"`, o `"powershell"` su Windows quando Bash non è disponibile

```json settings.json theme={null}
{
  "defaultShell": "powershell"
}
```

Se la shell che nomini non è disponibile, Claude Code utilizza l'altra: `"powershell"` ritorna a Bash quando lo strumento PowerShell è disattivato, e `"bash"` ritorna a PowerShell quando Bash non è installato.

<h3 id="dialogexpiry">
  `dialogExpiry`
</h3>

Imposta la scadenza per le finestre di dialogo che Claude Code [inoltra a un client remoto](/docs/it/remote-control#limitations), come un host Remote Control o SDK, e per la finestra di dialogo di approvazione per un [messaggio tra sessioni mantenuto](/docs/it/cross-session-messaging#control-inbound-messages). Su Claude Code v2.1.236 o successivo, la stessa scadenza limita il prompt di consenso per i crediti di utilizzo [Fable](/docs/it/model-config#fable-and-usage-credits) a metà sessione in una sessione che potrebbe non avere nessuno al terminale. Quando nessuna risposta arriva prima della scadenza, Claude Code annulla la finestra di dialogo e continua con il suo default senza azione. Richiede Claude Code v2.1.224 o successivo.

* **Scope**: [`User or managed`](#scopes)
* **Type**: string, uno di `"60s"`, `"5m"`, `"10m"`, o `"never"`, che disabilita la scadenza
* **Default**: `"5m"`
* **Per-session overrides**: [`CLAUDE_CODE_USER_DIALOG_TIMEOUT_MS`](/docs/it/env-vars) ha la precedenza su questa chiave per una sessione

```json settings.json theme={null}
{
  "dialogExpiry": "10m"
}
```

I prompt di autorizzazione e le domande [`AskUserQuestion`](/docs/it/tools-reference#askuserquestion-tool-behavior) utilizzano i loro flussi propri e non sono governati da questa scadenza. Appare in `/config` come **Dialog expiry**, che scrive questa chiave nelle impostazioni utente; la riga richiede Claude Code v2.1.232 o successivo, e Claude Code la nasconde mentre le impostazioni gestite o il flag `--settings` impostano la chiave.

<h3 id="editormode">
  `editorMode`
</h3>

Scegli la modalità di associazione dei tasti per il prompt di input.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, uno di:
  * `"normal"`: scorciatoie da tastiera standard nel prompt di input
  * `"vim"`: editing in stile vim con modalità NORMAL, INSERT e VISUAL
* **Default**: `"normal"`

```json settings.json theme={null}
{
  "editorMode": "vim"
}
```

Appare in `/config` come **Editor mode**, che scrive questa chiave nelle impostazioni utente.

<h3 id="emojicompletionenabled">
  `emojiCompletionEnabled`
</h3>

Mostra suggerimenti emoji quando digiti `:` più un codice breve nel prompt di input, e sostituisci un codice breve completato come `:heart:` con la sua emoji. Impostalo su `false` per disattivare entrambi.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code mostra suggerimenti emoji dopo `:` e sostituisce un codice breve completato con la sua emoji
  * `false`: Claude Code non suggerisce emoji né sostituisce codici brevi
* **Default**: `true`

```json settings.json theme={null}
{
  "emojiCompletionEnabled": false
}
```

Vedi [Codici brevi emoji](/docs/it/interactive-mode#emoji-shortcodes). Richiede Claude Code v2.1.217 o successivo.

<span id="file-suggestion-settings" />

<h3 id="filesuggestion">
  `fileSuggestion`
</h3>

Esegui il tuo comando per fornire il completamento automatico del percorso file `@` invece del suggerimento file integrato. Il suggerimento integrato utilizza l'attraversamento del filesystem veloce; un grande monorepo potrebbe fare meglio con l'indicizzazione specifica del progetto come un indice file pre-costruito.

* **Scope**: [`Any file`](#scopes). Secondo i [gate di riga di stato e suggerimento file](#status-line-and-file-suggestion-gates), Claude Code disattiva il comando o esegue solo un valore gestito, e salta il tuo senza avviso.
* **Type**: object con `type`, sempre `"command"`, e `command`, il comando shell da eseguire
* **Default**: unset, quindi Claude Code utilizza il suggerimento file integrato

```json settings.json theme={null}
{
  "fileSuggestion": {
    "type": "command",
    "command": "~/.claude/file-suggestion.sh"
  }
}
```

Dopo aver salvato questo, digita `@` seguito da parte di un percorso nel prompt: i suggerimenti provengono dall'output del tuo comando.

<h4 id="command-input-and-output">
  Input e output del comando
</h4>

Claude Code esegue il comando con le stesse variabili di ambiente dei [hooks](/docs/it/hooks), incluso `CLAUDE_PROJECT_DIR`, e smette di attendere dopo cinque secondi. Il comando riceve JSON su stdin con un campo `query` che contiene quello che hai digitato finora:

```json theme={null}
{"query": "src/comp"}
```

Stampa percorsi file separati da newline su stdout. Claude Code mostra al massimo 15:

```text theme={null}
src/components/Button.tsx
src/components/Modal.tsx
src/components/Form.tsx
```

Lo script seguente legge la query e la passa a un indice file del repository:

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

Renderizza badge cliccabili extra nel footer sotto la casella di input quando una regex corrisponde all'output del turno: risultati degli strumenti, inclusi contenuti di file e pagine recuperate, e risposte di Claude. Usalo per trasformare gli ID stampati dai CLI del progetto, come strumenti di revisione e tracker di problemi, in link di sessione.

* **Scope**: [`User or managed`](#scopes)
* **Type**: array di oggetti, ognuno con `type` impostato su `"regex"`, una regex `pattern`, un template `url`, e un `label` opzionale; i placeholder `{name}` in `url` e `label` vengono riempiti dai gruppi di cattura denominati in `pattern`
* **Default**: unset, quindi nessun badge viene renderizzato

Questo esempio corrisponde a chiavi di problemi come `PROJ-1234` e costruisce ogni link dalla chiave catturata:

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

Con questo configurato, quando `PROJ-1234` appare in un risultato dello strumento o nella risposta di Claude, un badge `PROJ-1234` appare nel footer collegato a `https://issues.example.com/browse/PROJ-1234`.

<h4 id="badge-constraints">
  Vincoli dei badge
</h4>

L'URL, l'etichetta e il conteggio dei badge di ogni voce sono limitati come segue:

| Vincolo         | Comportamento                                                                                                                                                                                                               |
| :-------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Origine URL     | I valori catturati sono codificati in URL e l'URL costruito deve condividere l'origine letterale del template. Una cattura può riempire un segmento di percorso o un valore di query ma non può cambiare dove punta il link |
| Lunghezza URL   | Gli URL costruiti più lunghi di 2048 caratteri vengono eliminati                                                                                                                                                            |
| Schema URL      | Deve essere `https`, `http`, o uno schema di deep-link riconosciuto per editor o workspace: `vscode`, `vscode-insiders`, `cursor`, `windsurf`, `zed`, `jetbrains`, `idea`, `slack`, `linear`, `notion`, `figma`             |
| Etichetta       | Predefinita al testo corrispondente e troncata a 28 colonne di visualizzazione                                                                                                                                              |
| Conteggio badge | Al massimo 5 badge vengono renderizzati. Il più vecchio viene sostituito da corrispondenze più recenti e `/clear` li rimuove                                                                                                |

Quando un turno si completa, Claude Code corrisponde a ogni regex `pattern` della voce rispetto all'output del turno sul thread principale, quindi una regex lenta blocca l'interfaccia utente fino al completamento. I quantificatori annidati come `(a+)+$` possono richiedere un tempo esponenziale rispetto a certi input e bloccare la sessione, quindi mantieni ogni `pattern` lineare ed evita di annidare `+` o `*`.

I badge del footer vengono renderizzati insieme a una [riga di stato personalizzata](/docs/it/statusline) quando una è configurata; nessuno sostituisce l'altro. Usa una riga di stato per una riga guidata da script che calcola il suo contenuto dai dati della sessione, e badge del footer per trasformare gli ID dalla conversazione in link senza uno script.

<h3 id="keybindingflavor">
  `keybindingFlavor`
</h3>

<Warning>
  Deprecato da v2.1.261 e non ha effetto. Le scorciatoie da tastiera di modifica delle parole del prompt seguono sempre [le convenzioni readline](/docs/it/interactive-mode#make-ctrl-w-delete-back-to-whitespace), come in Bash. Claude Code accetta comunque `keybindingFlavor`, quindi un file di impostazioni che lo imposta rimane valido.
</Warning>

Nella v2.1.238 fino a v2.1.260, impostarlo su `"readline"` faceva sì che `Ctrl+W` eliminasse fino allo spazio bianco precedente invece di solo la parola precedente.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, `"classic"` o `"readline"`
* **Default**: unset

<h3 id="prefersreducedmotion">
  `prefersReducedMotion`
</h3>

Riduci o disattiva le animazioni dell'interfaccia come lo spinner, lo shimmer e gli effetti flash. Appare in `/config` come **Reduce motion**.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code riduce o disattiva le animazioni dell'interfaccia come lo spinner, lo shimmer e gli effetti flash
  * `false`: lo stesso di unset; Claude Code mostra le sue animazioni
* **Default**: `false`

```json settings.json theme={null}
{
  "prefersReducedMotion": true
}
```

<h3 id="promptsuggestionenabled">
  `promptSuggestionEnabled`
</h3>

Mostra o nascondi i [suggerimenti del prompt](/docs/it/interactive-mode#prompt-suggestions), le previsioni grigie che appaiono nel tuo input del prompt. Impostalo su `false`, o disattiva **Prompt suggestions** in `/config`, per nasconderli.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: vedi suggerimenti del prompt nel tuo input del prompt
  * `false`: Claude Code nasconde i suggerimenti del prompt
* **Default**: `true`
* **Per-session overrides**: [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](/docs/it/env-vars) ha la precedenza su questa chiave per una sessione

```json settings.json theme={null}
{
  "promptSuggestionEnabled": false
}
```

I suggerimenti del prompt necessitano di un account claude.ai o Console con telemetria attiva. Su Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry, o con telemetria disattivata, come da [`DISABLE_TELEMETRY`](/docs/it/env-vars), questa chiave non ha effetto e solo `CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=1` li attiva.

<h3 id="respectgitignore">
  `respectGitignore`
</h3>

Controlla se il selettore file `@` esclude i file che corrispondono ai pattern `.gitignore`. Appare in `/config` come **Respect .gitignore in file picker**.

* **Scope**: [`Any file`](#scopes). Quando nessun file di impostazioni lo imposta, Claude Code ritorna a `respectGitignore` in `~/.claude.json`, che l'interruttore `/config` scrive.
* **Type**: Boolean
  * `true`: il selettore file `@` esclude i file che corrispondono ai pattern `.gitignore`
  * `false`: il selettore file `@` include i file che corrispondono ai pattern `.gitignore`
* **Default**: `true`

```json settings.json theme={null}
{
  "respectGitignore": false
}
```

<h3 id="respondtobashcommands">
  `respondToBashCommands`
</h3>

Scegli se Claude risponde dopo che esegui un comando shell con il prefisso [`!`](/docs/it/interactive-mode#shell-mode-with-prefix) nella casella di input. Per impostazione predefinita, Claude Code aggiunge l'output del comando alla conversazione e Claude vi risponde. Imposta questa chiave su `false` per aggiungere l'output al contesto senza una risposta, così puoi eseguire diversi comandi e chiedere informazioni su di essi insieme.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code aggiunge l'output del comando alla conversazione e Claude vi risponde
  * `false`: Claude Code aggiunge l'output al contesto senza una risposta
* **Default**: `true`

```json settings.json theme={null}
{
  "respondToBashCommands": false
}
```

Vedi [Modalità shell con prefisso `!`](/docs/it/interactive-mode#shell-mode-with-prefix).

<h3 id="showclearcontextonplanaccept">
  `showClearContextOnPlanAccept`
</h3>

Quando Claude termina un piano in [modalità piano](/docs/it/permission-modes#review-and-approve-a-plan), mostra un menu di approvazione. La pianificazione può utilizzare molto contesto, quindi questa chiave aggiunge una prima opzione a quel menu, **Yes, clear context and …**, che approva il piano, cancella il contesto della conversazione e inizia l'implementazione dal piano solo. Il resto dell'etichetta nomina la modalità di autorizzazione in cui la sessione continua, e mostra quanto contesto la pianificazione ha utilizzato.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: il menu di approvazione del piano ottiene una prima opzione, **Yes, clear context and …**, che approva il piano e cancella il contesto della conversazione
  * `false`: il menu di approvazione del piano non mostra alcuna opzione di cancellazione del contesto
* **Default**: `false`

```json settings.json theme={null}
{
  "showClearContextOnPlanAccept": true
}
```

<h3 id="showturnduration">
  `showTurnDuration`
</h3>

Mostra o nascondi il messaggio di durata del turno dopo ogni risposta, come "Cooked for 1m 6s · done 6:05 PM". L'orologio dopo "done" mostra quando il turno è terminato; [`timeFormat`](#timeformat) e [`timeZone`](#timezone) controllano il suo formato e la sua zona. Appare in `/config` come **Show turn duration**.

* **Scope**: [`Any file`](#scopes). Un valore in `~/.claude.json` da una versione precedente si applica quando nessun file di impostazioni lo imposta.
* **Type**: Boolean
  * `true`: vedi il messaggio di durata del turno dopo ogni risposta
  * `false`: Claude Code nasconde il messaggio di durata del turno
* **Default**: `true`

```json settings.json theme={null}
{
  "showTurnDuration": false
}
```

<h3 id="spellcheck">
  `spellcheck`
</h3>

Sottolinea le parole scritte male nel prompt di input mentre digiti, utilizzando un correttore ortografico che installi. Claude Code controlla solo il testo nella casella di input. [Controlla l'ortografia mentre digiti](/docs/it/interactive-mode#check-spelling-as-you-type) copre l'installazione di aspell, hunspell o ispell e cosa il correttore copre. Richiede Claude Code v2.1.235 o successivo.

* **Scope**: [`User or managed`](#scopes). Il blocco dal livello più alto che lo imposta si applica nel suo insieme.
* **Type**: object con `enabled` (Boolean), `checker` (`"aspell"`, `"hunspell"`, `"ispell"`, o `"auto"`), `language` (string, passato al correttore come nome del suo dizionario), e `color` (string, un nome di colore del terminale, `#rrggbb`, `rgb(r,g,b)`, `ansi256(n)`, o `ansi:<name>`)
* **Default**: unset, quindi il controllo ortografico è disattivato; `checker` predefinito su `"auto"`, il primo dei tre trovato su `PATH`; `language` predefinito sul dizionario del correttore stesso; `color` predefinito sul colore di errore del tema

```json settings.json theme={null}
{
  "spellcheck": { "enabled": true, "language": "en_GB" }
}
```

<h3 id="spinnertipsenabled">
  `spinnerTipsEnabled`
</h3>

Mentre Claude lavora, la riga dello spinner ruota attraverso brevi suggerimenti sulle funzioni di Claude Code, come "Use Plan Mode to prepare for a complex request before making changes. Press Shift+Tab twice to enable." Imposta questa chiave su `false` per nasconderli. Appare in `/config` come **Show tips**.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: vedi suggerimenti nello spinner mentre Claude sta lavorando
  * `false`: Claude Code nasconde i suggerimenti dello spinner
* **Default**: `true`

```json settings.json theme={null}
{
  "spinnerTipsEnabled": false
}
```

<h3 id="spinnertipsoverride">
  `spinnerTipsOverride`
</h3>

Aggiungi i tuoi suggerimenti ai [suggerimenti dello spinner](#spinnertipsenabled) che Claude Code mostra mentre Claude lavora, o sostituisci i suggerimenti integrati con i tuoi. Claude Code mette i tuoi suggerimenti nella stessa rotazione di quelli integrati: sceglie il suggerimento che non è stato mostrato più a lungo, salta i suggerimenti ancora nel loro cooldown, e rompe i pareggi per priorità.

Se imposti [`spinnerTipsEnabled`](#spinnertipsenabled) su `false`, Claude Code nasconde tutti i suggerimenti, inclusi i tuoi.

* **Scope**: [`Any file`](#scopes). Claude Code onora gli oggetti suggerimento, `tipsFile`, `label`, e `excludeDefault` dalle impostazioni utente, il flag `--settings` e impostazioni gestite; dai file di impostazioni di progetto e locale legge solo suggerimenti di stringa semplice.
* **Type**: object con campi `tips`, `tipsFile`, `label`, e `excludeDefault`, ognuno opzionale
* **Default**: unset, quindi Claude Code mostra solo i suggerimenti integrati

Gli oggetti suggerimento, `tipsFile`, `label`, e la regola della riga Scope che i file di impostazioni di progetto e locale contribuiscono solo stringhe semplici richiedono Claude Code v2.1.247 o successivo. Nelle versioni precedenti, `excludeDefault` di un file di progetto o locale si applica anche.

Ogni voce `tips` è una stringa semplice o un oggetto con questi campi:

| Campo              | Obbligatorio | Descrizione                                                                                                                                                                                                                                                         |
| :----------------- | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `id`               | Sì           | Fino a 64 lettere, cifre, `.`, `_`, o `-`. Claude Code chiave la storia della visualizzazione del suggerimento su di esso, quindi il cooldown del suggerimento sopravvive al riordinamento dell'elenco. Di due voci con lo stesso id, Claude Code utilizza la prima |
| `text`             | Sì           | Il suggerimento, una riga di fino a 500 caratteri. Claude Code rimuove le sequenze ANSI e i caratteri di controllo e comprime lo spazio bianco                                                                                                                      |
| `cooldownSessions` | No           | Sessioni che Claude Code attende prima di mostrare di nuovo il suggerimento, da `0` a `1000`, predefinito `0`                                                                                                                                                       |
| `priority`         | No           | Ordine tra i suggerimenti che non sono stati mostrati ugualmente a lungo, più alto prima, da `-10` a `10`, predefinito `0`                                                                                                                                          |

Claude Code legge una stringa semplice come un suggerimento con quei valori predefiniti e un id basato sulla posizione, quindi la sua storia di visualizzazione si ripristina quando riordini l'elenco. Dai a un suggerimento un `id` per mantenere la sua storia attraverso le modifiche.

Claude Code legge al massimo 200 suggerimenti tra `tips` e `tipsFile`, e elimina una voce non valida con un avviso di debug invece di rifiutare il file di impostazioni.

Usa i campi rimanenti per nominare un file di suggerimenti, impostare il prefisso e nascondere i suggerimenti integrati:

* `tipsFile`: un percorso assoluto o `~/` a un file JSON locale che contiene un array delle stesse voci, o un oggetto con un array `tips`, fino a 256 KB. Claude Code legge il file una volta per processo, quindi carica le tue modifiche al prossimo avvio. Non puoi impostarlo attraverso [impostazioni gestite dal server](/docs/it/server-managed-settings); distribuisci suggerimenti `tips` inline lì, o distribuisci il percorso in un `managed-settings.json` su disco.
* `label`: il prefisso che Claude Code mostra prima dei suggerimenti dalle impostazioni utente, `--settings` e gestite, fino a 40 caratteri. L'impostazione predefinita è `Tip`, lo stesso prefisso dei suggerimenti integrati, e i suggerimenti dai file di impostazioni di progetto e locale lo usano sempre.
* `excludeDefault`: impostalo su `true` per nascondere i suggerimenti integrati e mostrare solo i tuoi. Quando Claude Code non riesce a caricare nessuno dei tuoi suggerimenti, ad esempio perché `tipsFile` non esiste o ogni voce non è valida, mantiene la rotazione integrata invece di uno spinner vuoto.

Quando più di un file di impostazioni imposta la chiave, Claude Code mostra suggerimenti da tutti loro e prende `tipsFile`, `label`, e `excludeDefault` da qualunque delle impostazioni gestite, il flag `--settings` e impostazioni utente sia il più alto precedente che imposta ognuno.

Questo esempio, nelle tue impostazioni utente, aggiunge un suggerimento di stringa semplice e un suggerimento di oggetto alla rotazione sotto il prefisso `Acme tip`:

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

Ogni campo nell'esempio cambia una cosa su come Claude Code mostra i suggerimenti:

* `label`: Claude Code mostra entrambi i suggerimenti come `Acme tip: ...` invece di `Tip: ...`.
* La stringa semplice: Claude Code le dà i valori predefiniti, quindi può venire di nuovo nella sessione successiva.
* `id`: Claude Code chiave la storia della visualizzazione del secondo suggerimento su `gateway-errors`, quindi il suo cooldown si applica ancora dopo che aggiungi o riordini i suggerimenti.
* `cooldownSessions`: dopo che Claude Code mostra il suggerimento `gateway-errors`, non mostra quel suggerimento di nuovo fino a cinque sessioni dopo.
* `priority`: quando il suggerimento `gateway-errors` e un altro suggerimento non sono stati mostrati per lo stesso numero di sessioni, ad esempio quando nessuno è stato ancora mostrato, Claude Code mostra `gateway-errors` per primo. La stringa semplice ha la priorità predefinita, `0`.

Mentre Claude lavora, Claude Code mostra i tuoi suggerimenti nello spinner con il tuo prefisso, come `Acme tip: Run /review before opening a PR`.

<h3 id="spinnerverbs">
  `spinnerVerbs`
</h3>

Mentre un turno è in corso, lo spinner mostra un verbo rotante come "Accomplishing", "Architecting", o "Baking". Usa questa chiave per aggiungere i tuoi verbi a quella rotazione o sostituire l'elenco integrato con il tuo.

* **Scope**: [`Any file`](#scopes)
* **Type**: object con un array `verbs` di stringhe e `mode`, uno di:
  * `"append"`: Claude Code aggiunge i tuoi verbi al set integrato
  * `"replace"`: Claude Code mostra solo i tuoi verbi
* **Default**: unset, quindi Claude Code utilizza i verbi integrati

Questo esempio aggiunge due verbi al set integrato:

```json settings.json theme={null}
{
  "spinnerVerbs": {
    "mode": "append",
    "verbs": ["Pondering", "Crafting"]
  }
}
```

In modalità `"replace"` con un array `verbs` vuoto, Claude Code mantiene i verbi integrati.

<h3 id="statusline">
  `statusLine`
</h3>

Esegui il tuo comando per renderizzare una [riga di stato](/docs/it/statusline) sotto il prompt con contesto come il modello, il costo o il ramo git. I campi opzionali regolano la spaziatura, aggiungono re-esecuzioni periodiche e nascondono l'indicatore di modalità vim integrato quando il tuo script renderizza `vim.mode` stesso.

* **Scope**: [`Any file`](#scopes). Quando [`allowManagedHooksOnly`](#allowmanagedhooksonly) è attivo, o [`disableAllHooks`](#disableallhooks) è impostato al di fuori delle impostazioni gestite, solo il valore delle impostazioni gestite viene eseguito.
* **Type**: object con `type` impostato su `"command"` e una stringa `command`, più `padding` opzionale come numero di caratteri, `refreshInterval` come numero di secondi, minimo `1`, e `hideVimModeIndicator` come Boolean
* **Default**: unset, quindi nessuna riga di stato

Questo esempio stampa il nome del modello e l'utilizzo del contesto, e aggiunge due caratteri di spaziatura orizzontale:

```json settings.json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
    "padding": 2
  }
}
```

L'esempio ha bisogno di [`jq`](https://jqlang.org/) installato e viene eseguito in una shell. Per equivalenti PowerShell e Git Bash, vedi [Configurazione di Windows](/docs/it/statusline#windows-configuration); per la configurazione completa, vedi [Configura manualmente una riga di stato](/docs/it/statusline#manually-configure-a-status-line).

<h3 id="subagentstatusline">
  `subagentStatusLine`
</h3>

Quando Claude esegue [subagenti](/docs/it/sub-agents), Claude Code li elenca in una visualizzazione di attività sotto il prompt, una riga per subagente che mostra `name · description · token count`. Questa chiave ti consente di eseguire il tuo comando per riscrivere quelle righe, ad esempio per mostrare l'utilizzo del contesto di ogni subagente come percentuale. Su ogni aggiornamento, Claude Code invia le righe visibili come un oggetto JSON su stdin, con un array `tasks` che contiene `id`, `name`, `status`, `model`, `tokenCount` di ogni subagente e altro, e sostituisce la riga per ogni `id` che scrivi di nuovo come una riga `{"id", "content"}`. Le righe che non scrivi di nuovo mantengono il rendering predefinito.

* **Scope**: [`Any file`](#scopes). Quando [`allowManagedHooksOnly`](#allowmanagedhooksonly) è attivo, o [`disableAllHooks`](#disableallhooks) è impostato al di fuori delle impostazioni gestite, solo il valore delle impostazioni gestite viene eseguito.
* **Type**: object con `type` impostato su `"command"` e una stringa `command`
* **Default**: unset, quindi Claude Code renderizza le righe predefinite

```json settings.json theme={null}
{
  "subagentStatusLine": {
    "type": "command",
    "command": "jq -c '.tasks[] | {id, content: \"\\(.name): \\(.tokenCount) tokens\"}'"
  }
}
```

Vedi [Righe di stato dei subagenti](/docs/it/statusline#subagent-status-lines).

<h3 id="syntaxhighlightingdisabled">
  `syntaxHighlightingDisabled`
</h3>

Claude Code colora il codice per linguaggio nei diff, blocchi di codice e anteprime di file che mostra nel terminale, con il suo evidenziatore integrato; nessun plugin o language server è coinvolto. Imposta questa chiave su `true` per mostrarli come testo semplice invece, ad esempio se i colori si scontrano con il tuo tema del terminale o rallentano un lettore di schermo.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code disattiva l'evidenziazione della sintassi nei diff, blocchi di codice e anteprime di file
  * `false`: Claude Code evidenzia la sintassi
* **Default**: `false`

```json settings.json theme={null}
{
  "syntaxHighlightingDisabled": true
}
```

<h3 id="terminalprogressbarenabled">
  `terminalProgressBarEnabled`
</h3>

Alcuni terminali possono mostrare un indicatore di progresso sulla scheda o nella barra delle applicazioni per il programma in esecuzione in essi. Mentre Claude sta lavorando, Claude Code segnala uno stato in corso al terminale, così puoi vedere da un'altra scheda o finestra se la sessione è ancora occupata. L'indicatore rimane visibile dopo che il turno termina mentre i [subagenti in background](/docs/it/sub-agents#run-subagents-in-foreground-or-background) o i [flussi di lavoro dinamici](/docs/it/workflows) sono ancora in esecuzione, e si cancella una volta che la sessione è inattiva.

Claude Code lo segnala solo nei terminali che supportano l'indicatore: ConEmu, Ghostty 1.2.0 o successivo, e iTerm2 3.6.6 o successivo. Imposta questa chiave su `false` per impedire a Claude Code di segnalarlo. Appare in `/config` come **Terminal progress bar**.

* **Scope**: [`Any file`](#scopes). Un valore in `~/.claude.json` da una versione precedente si applica quando nessun file di impostazioni lo imposta.
* **Type**: Boolean
  * `true`: vedi la barra di progresso del terminale nei terminali che la supportano
  * `false`: Claude Code nasconde la barra di progresso del terminale
* **Default**: `true`

```json settings.json theme={null}
{
  "terminalProgressBarEnabled": false
}
```

<h3 id="terminaltitlefromrename">
  `terminalTitleFromRename`
</h3>

Claude Code imposta il titolo della scheda del tuo terminale. Per impostazione predefinita utilizza un titolo che genera dalla conversazione, e una volta che dai alla sessione un [nome](/docs/it/sessions#name-your-sessions) con `/rename` o `--name`, la scheda mostra quel nome invece. Imposta questa chiave su `false` per mantenere il titolo generato sulla scheda anche dopo che nomini la sessione. Il nome stesso si applica comunque, quindi `/resume <name>` e il selettore di sessione lo trovano.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: il titolo della scheda del terminale mostra il nome della sessione che hai impostato
  * `false`: la scheda mantiene il titolo che Claude Code genera dalla tua conversazione
* **Default**: `true`

```json settings.json theme={null}
{
  "terminalTitleFromRename": false
}
```

Per impedire a Claude Code di aggiornare il titolo del terminale del tutto, imposta [`CLAUDE_CODE_DISABLE_TERMINAL_TITLE`](/docs/it/env-vars) su `1` invece.

<h3 id="theme">
  `theme`
</h3>

Scegli il tema di colore per l'interfaccia. Appare in `/config` come **Theme**.

* **Scope**: [`Any file`](#scopes). Un valore in `~/.claude.json` da una versione precedente si applica quando nessun file di impostazioni lo imposta.
* **Type**: string, uno di:
  * `"auto"`: corrisponde allo sfondo chiaro o scuro del tuo terminale
  * `"dark"`: il tema scuro
  * `"light"`: il tema chiaro
  * `"dark-daltonized"`: il tema scuro con colori adatti ai daltonici
  * `"light-daltonized"`: il tema chiaro con colori adatti ai daltonici
  * `"dark-ansi"`: il tema scuro utilizzando solo la tavolozza di colori ANSI del tuo terminale
  * `"light-ansi"`: il tema chiaro utilizzando solo la tavolozza di colori ANSI del tuo terminale
  * `"custom:<slug>"` o `"custom:<plugin-name>:<slug>"`: un tema personalizzato da `~/.claude/themes/` o un plugin
* **Default**: `"dark"`

```json settings.json theme={null}
{
  "theme": "light-daltonized"
}
```

Vedi [Crea un tema personalizzato](/docs/it/terminal-config#create-a-custom-theme).

<h3 id="timeformat">
  `timeFormat`
</h3>

Scegli come Claude Code scrive i tempi che mostra nell'interfaccia, come il `done 6:05 PM` alla fine di ogni messaggio di durata del turno e i timestamp nel [visualizzatore di trascrizione](/docs/it/interactive-mode#transcript-viewer). Per scegliere un preset, esegui `/config` e imposta **Time format**. Richiede Claude Code v2.1.257 o successivo.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, uno di:
  * `"auto"`: lo stesso di unset; ogni tempo mantiene il suo formato integrato, che segue il tuo locale nel messaggio di durata del turno
  * `"12-hour"`: un orologio a 12 ore
  * `"24-hour"`: un orologio a 24 ore
  * `"24-hour-utc"`: un orologio a 24 ore in UTC con `Z` dopo i minuti, come `18:05Z`; Claude Code ignora [`timeZone`](#timezone) per questo preset
  * Un pattern strftime come `"%H:%M"`: Claude Code scrive ogni tempo con il pattern. Qualsiasi valore che contiene un `%` è un pattern, e qualsiasi altro valore al di fuori dei preset conta come `"auto"`
* **Default**: `"auto"`

```json settings.json theme={null}
{
  "timeFormat": "24-hour"
}
```

`/config` offre solo i preset, quindi per usare un pattern strftime, aggiungi la chiave a un file di impostazioni. Questo esempio mostra ogni tempo come un orologio a 24 ore a due cifre:

```json settings.json theme={null}
{
  "timeFormat": "%H:%M"
}
```

Il messaggio di durata del turno e il visualizzatore di trascrizione mostrano quindi tempi come `18:05`. Nel visualizzatore di trascrizione, il pattern è l'intero timestamp, quindi aggiungi direttive di data quando vuoi la data lì. Questo esempio mette la data davanti all'orologio:

```json settings.json theme={null}
{
  "timeFormat": "%Y-%m-%d %H:%M"
}
```

Le stesse superfici mostrano quindi tempi come `2026-09-01 18:05`.

<h3 id="timezone">
  `timeZone`
</h3>

Mostra i tempi nell'interfaccia in un fuso orario diverso dal tuo sistema. Impostalo su un [nome di fuso orario IANA](https://www.iana.org/time-zones), come `"UTC"` o `"Europe/Dublin"`. I tempi che [`timeFormat`](#timeformat) controlla mostrano quindi in questa zona. Se `timeFormat` è `"24-hour-utc"`, i tempi rimangono in UTC e Claude Code ignora questa chiave. `/config` non ha una riga per questa chiave, quindi impostala in un file di impostazioni. Richiede Claude Code v2.1.257 o successivo.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, un nome di fuso orario IANA. Quando Claude Code non riconosce il nome, utilizza il tuo fuso orario di sistema
* **Default**: unset, quindi i tempi mostrano nel tuo fuso orario di sistema

```json settings.json theme={null}
{
  "timeZone": "Europe/Dublin"
}
```

<h3 id="tui">
  `tui`
</h3>

Scegli il renderer dell'interfaccia utente del terminale. Usa `"fullscreen"` per il renderer [alt-screen](/docs/it/fullscreen) senza sfarfallio con scrollback virtualizzato, o `"default"` per il renderer classico dello schermo principale. Eseguire `/tui fullscreen` o `/tui default` scrive questa chiave per te.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, uno di:
  * `"default"`: il renderer classico dello schermo principale
  * `"fullscreen"`: il renderer alt-screen senza sfarfallio con scrollback virtualizzato
* **Default**: unset, quindi Claude Code [sceglie il renderer per te](/docs/it/fullscreen#fullscreen-by-default)
* **Per-session overrides**: [`CLAUDE_CODE_NO_FLICKER`](/docs/it/env-vars) e [`CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN`](/docs/it/env-vars) hanno la precedenza su questa chiave per una sessione: `CLAUDE_CODE_NO_FLICKER=1` attiva fullscreen, e `CLAUDE_CODE_NO_FLICKER=0` o `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1` lo disattiva; quando entrambi sono impostati, Claude Code lo disattiva

```json settings.json theme={null}
{
  "tui": "fullscreen"
}
```

Sotto tmux `-CC` o su SSH a Windows, Claude Code mantiene il renderer classico a meno che tu non imposti `CLAUDE_CODE_NO_FLICKER=1`. Le sessioni in background aperte da [agent view](/docs/it/agent-view) utilizzano sempre il renderer fullscreen indipendentemente da questa impostazione.

<h3 id="verbose">
  `verbose`
</h3>

Per impostazione predefinita, la trascrizione comprime ogni chiamata di strumento a un breve riepilogo, come il comando che Claude ha eseguito e un conteggio delle righe del suo output, e premi `Ctrl+O` per passare l'intera trascrizione alla visualizzazione espansa quando vuoi i dettagli. Imposta questa chiave su `true` per mostrare l'input e l'output completi di ogni chiamata di strumento inline mentre accade, il che è utile quando stai eseguendo il debug di un hook, di un server MCP o di un lungo comando shell. Appare in `/config` come **Verbose output**.

* **Scope**: [`Any file`](#scopes). Un valore in `~/.claude.json` da una versione precedente si applica quando nessun file di impostazioni lo imposta.
* **Type**: Boolean
  * `true`: vedi l'output completo dello strumento
  * `false`: vedi riepiloghi troncati dell'output dello strumento
* **Default**: `false`
* **Per-session overrides**: [`--verbose`](/docs/it/cli-reference#cli-flags) ha la precedenza su questa chiave per una sessione

```json settings.json theme={null}
{
  "verbose": true
}
```

Un valore [`viewMode`](#viewmode) o una selezione sticky `/focus` sostituisce questa chiave ogni sessione.

<h3 id="viewmode">
  `viewMode`
</h3>

Imposta la visualizzazione della trascrizione in cui Claude Code inizia: `"default"`, `"verbose"`, o `"focus"`. Quando impostato, sostituisce sia la selezione sticky `/focus` che l'impostazione [`verbose`](#verbose).

* **Scope**: [`Any file`](#scopes)
* **Type**: string, uno di:
  * `"default"`: la trascrizione normale con output dello strumento troncato
  * `"verbose"`: la trascrizione con output dello strumento completo
  * `"focus"`: solo il tuo ultimo prompt, un riepilogo di una riga delle chiamate di strumento con diffstat di modifica, e la risposta finale. La visualizzazione focus necessita del [renderer fullscreen](#tui)
* **Default**: unset, quindi l'impostazione `verbose` e la tua ultima scelta `/focus` si applicano
* **Per-session overrides**: [`--verbose`](/docs/it/cli-reference#cli-flags) ha la precedenza su questa chiave per una sessione

```json settings.json theme={null}
{
  "viewMode": "focus"
}
```

<h3 id="viminsertmoderemaps">
  `vimInsertModeRemaps`
</h3>

Mappa sequenze INSERT-mode a due tasti su Escape in [modalità editor vim](/docs/it/interactive-mode#vim-editor-mode). Ogni chiave è esattamente due caratteri stampabili digitati in sequenza, e `"<Esc>"` è l'unico target supportato; Claude Code ignora altre voci. Richiede Claude Code v2.1.208 o successivo.

* **Scope**: [`User or managed`](#scopes). Un repository non può rimappare le tue scorciatoie da tastiera.
* **Type**: object che mappa una sequenza di due caratteri su `"<Esc>"`
* **Default**: unset

```json settings.json theme={null}
{
  "vimInsertModeRemaps": {
    "jj": "<Esc>"
  }
}
```

Non ha effetto a meno che `editorMode` non sia `"vim"`. Vedi [Rimappa sequenze di tasti INSERT-mode](/docs/it/interactive-mode#remap-insert-mode-key-sequences). Richiede Claude Code v2.1.208 o successivo.

<h3 id="voice">
  `voice`
</h3>

Attiva la [dettatura vocale](/docs/it/voice-dictation) e scegli come il tasto di dettatura si comporta. Claude Code scrive questo oggetto per te quando esegui `/voice`.

* **Scope**: [`Any file`](#scopes)
* **Type**: object con `enabled` come Boolean, `autoSubmit` come Boolean che si applica solo in modalità hold, e `mode`, uno di:
  * `"hold"`: tieni premuto il tasto di dettatura mentre parli e rilascialo per fermarti
  * `"tap"`: tocca il tasto una volta per iniziare la registrazione e di nuovo per inviare
* **Default**: unset, quindi la dettatura è disattivata; quando `enabled` è `true` e `mode` è unset, Claude Code utilizza `"hold"`

Questo esempio attiva la dettatura e fa sì che il tasto tocchi una volta per iniziare la registrazione e di nuovo per inviare:

```json settings.json theme={null}
{
  "voice": {
    "enabled": true,
    "mode": "tap"
  }
}
```

`autoSubmit` invia il prompt quando rilasci il tasto in modalità hold. La dettatura vocale richiede un account claude.ai.

<h3 id="voiceenabled">
  `voiceEnabled`
</h3>

<Warning>
  Deprecato da v2.1.92, quando l'oggetto [`voice`](#voice) lo ha sostituito. Claude Code lo legge comunque così i file di impostazioni precedenti continuano a funzionare, ma le nuove configurazioni dovrebbero impostare `voice.enabled`.
</Warning>

Attiva la dettatura vocale con il modulo Boolean singolo che precede l'oggetto `voice`. Quando entrambi sono impostati, `voice.enabled` si applica.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: la dettatura vocale è attiva quando sei connesso con un account claude.ai e la politica della tua organizzazione consente la voce, a meno che `voice.enabled` non sia impostato
  * `false`: la dettatura vocale è disattivata, a meno che `voice.enabled` non sia impostato
* **Default**: unset

```json settings.json theme={null}
{
  "voiceEnabled": true
}
```

<h3 id="wheelscrollaccelerationenabled">
  `wheelScrollAccelerationEnabled`
</h3>

Accelera la velocità di scorrimento della rotella del mouse durante scorrimenti veloci nel [rendering a schermo intero](/docs/it/fullscreen#mouse-wheel-scrolling). Impostalo su `false` per una velocità di scorrimento costante per tacca della rotella.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code accelera la velocità di scorrimento della rotella del mouse durante scorrimenti veloci
  * `false`: Claude Code scorre a una velocità costante per tacca della rotella
* **Default**: `true`

```json settings.json theme={null}
{
  "wheelScrollAccelerationEnabled": false
}
```

<h2 id="git-and-attribution">
  Git e attribuzione
</h2>

Controllate l'attribuzione che Claude Code aggiunge ai commit e alle pull request e come funziona con git.

<span id="attribution-settings" />

<h3 id="attribution">
  `attribution`
</h3>

Personalizzate l'attribuzione che Claude Code aggiunge ai commit git e alle pull request. I commit ricevono un [git trailer](https://git-scm.com/docs/git-interpret-trailers) come `Co-Authored-By` per impostazione predefinita; le descrizioni delle pull request ricevono testo semplice. Impostate ogni parte separatamente con le sotto-chiavi di seguito.

* **Ambito**: [`Any file`](#scopes)
* **Tipo**: oggetto con stringhe `commit` e `pr` e un Boolean `sessionUrl`, oppure `false` per nascondere tutta l'attribuzione. Il valore `false` richiede Claude Code v2.1.281 o successivo; le versioni precedenti lo rifiutano e [saltano l'intero file di impostazioni utente, progetto o locale](/docs/it/settings#fix-a-broken-settings-file) che lo contiene
* **Predefinito**: non impostato, quindi Claude Code utilizza l'attribuzione standard mostrata sotto ogni sotto-chiave

Per nascondere tutta l'attribuzione, impostate `attribution` su `false`. In un file di impostazioni che anche le versioni precedenti leggono, impostate [`commit`](#attribution-commit) e [`pr`](#attribution-pr) su stringhe vuote e [`sessionUrl`](#attribution-sessionurl) su `false` invece.

Questo esempio sostituisce l'attribuzione del commit, rimuove l'attribuzione della pull request e elimina il collegamento della sessione:

```json settings.json theme={null}
{
  "attribution": {
    "commit": "Generated with AI\n\nCo-Authored-By: AI <ai@example.com>",
    "pr": "",
    "sessionUrl": false
  }
}
```

Una volta impostato `commit` o `pr`, Claude Code ignora l'impostazione deprecata `includeCoAuthoredBy` e utilizza il testo predefinito per quello dei due che avete lasciato non impostato.

Claude Code comunica a Claude che le vostre istruzioni personali sull'attribuzione, come una regola CLAUDE.md o [memory](/docs/it/memory), hanno la precedenza su queste righe di commit e PR, a meno che la riga non sia impostata in [managed settings](/docs/it/managed-settings).

<h3 id="includecoauthoredby">
  `includeCoAuthoredBy`
</h3>

<Warning>
  Deprecato dalla v2.0.62, quando [`attribution`](#attribution) lo ha sostituito. Claude Code lo legge ancora, ma le nuove configurazioni dovrebbero impostare `attribution`.
</Warning>

Utilizzate [`attribution`](#attribution) invece, che sostituisce questa chiave e vi consente di modificare o nascondere il trailer del commit, il testo della pull request e il collegamento della sessione separatamente. Claude Code onora ancora `includeCoAuthoredBy: false` dai file di impostazioni precedenti a `attribution`, ma lo ignora una volta impostato `attribution.commit` o `attribution.pr`.

* **Ambito**: [`Any file`](#scopes)
* **Tipo**: Boolean
  * `true`: lo stesso di non impostato; Claude Code aggiunge il trailer del commit e il testo di attribuzione della pull request
  * `false`: Claude Code omette sia il trailer del commit che il testo di attribuzione della pull request, a meno che `attribution` non imposti `commit` o `pr`, nel qual caso si applicano le regole [`attribution`](#attribution)
* **Predefinito**: `true`

```json settings.json theme={null}
{
  "includeCoAuthoredBy": false
}
```

Per nascondere tutta l'attribuzione, vedete [`attribution`](#attribution).

<h3 id="includegitinstructions">
  `includeGitInstructions`
</h3>

Claude Code fornisce a Claude due elementi correlati a git: le sue istruzioni integrate su come scrivere commit e pull request, nella descrizione dello strumento Bash, e uno snapshot dello stato git del vostro repository. Lo snapshot contiene il ramo corrente, il ramo principale, l'output di `git status` e i commit recenti. Claude Code lo legge quando inizia una conversazione.

Impostate questa chiave su `false` per escludere entrambi, ad esempio quando utilizzate le vostre skill di flusso di lavoro git personalizzate.

* **Ambito**: [`Any file`](#scopes)
* **Tipo**: Boolean
  * `true`: Claude Code include le sue istruzioni integrate per il flusso di lavoro di commit e pull request e lo snapshot dello stato git. Le sessioni cloud non includono mai lo snapshot
  * `false`: Claude Code esclude entrambi
* **Predefinito**: `true`
* **Override per sessione**: [`CLAUDE_CODE_DISABLE_GIT_INSTRUCTIONS`](/docs/it/env-vars) ha la precedenza su questa chiave per una sessione

```json settings.json theme={null}
{
  "includeGitInstructions": false
}
```

<h3 id="prurltemplate">
  `prUrlTemplate`
</h3>

Indirizzate i link PR che Claude Code renderizza, nel badge del footer e nei riepiloghi dei risultati degli strumenti, verso uno strumento di revisione del codice interno invece di `github.com`. Claude Code sostituisce `{host}`, `{owner}`, `{repo}`, `{number}` e `{url}` dall'URL della PR. I link delle [richieste di merge GitLab](/docs/it/interactive-mode#gitlab-merge-requests) su entrambe le superfici mantengono il loro URL GitLab.

* **Ambito**: [`Any file`](#scopes)
* **Tipo**: stringa, un modello di URL utilizzando uno qualsiasi dei cinque segnaposti
* **Predefinito**: non impostato

```json settings.json theme={null}
{
  "prUrlTemplate": "https://reviews.example.com/{owner}/{repo}/pull/{number}"
}
```

Claude Code applica il modello solo ai link che renderizza stesso; un numero PR che Claude scrive in un messaggio, come `#123`, rimane come Claude lo ha scritto. Un URL che non ha la forma `/pull/<number>` viene lasciato invariato.

<h3 id="attribution-commit">
  `attribution.commit`
</h3>

Impostate il testo di attribuzione che Claude Code aggiunge ai commit git, inclusi eventuali trailer. Impostatelo su una stringa vuota per nascondere l'attribuzione del commit.

* **Ambito**: [`Any file`](#scopes)
* **Tipo**: stringa
* **Predefinito**: non impostato, quindi Claude Code aggiunge `Co-Authored-By: <name> <noreply@anthropic.com>`. Il nome è il modello attivo della sessione, come `Claude Sonnet 5`.
  * Quando Claude Code riconosce il modello come un modello Claude ma non può confermarne la versione esatta, scrive `Claude` da solo.
  * Quando non può abbinare l'ID del modello a nessun modello Claude, come un modello di terze parti servito attraverso un [`ANTHROPIC_BASE_URL`](/docs/it/env-vars) personalizzato, scrive `Claude Code`.

Questo esempio sostituisce il trailer predefinito con una riga personalizzata e un trailer `Co-Authored-By` personalizzato:

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

Impostate il testo di attribuzione che Claude Code aggiunge alle descrizioni delle pull request. Impostatelo su una stringa vuota per nascondere l'attribuzione della pull request.

* **Ambito**: [`Any file`](#scopes)
* **Tipo**: stringa
* **Predefinito**: non impostato, quindi Claude Code aggiunge `🤖 Generated with [Claude Code](https://claude.com/claude-code)`

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

Scegliete se Claude Code aggiunge il collegamento della sessione claude.ai quando esegue il commit o apre una pull request da una sessione [cloud](/docs/it/claude-code-on-the-web) o [Remote Control](/docs/it/remote-control). Claude Code aggiunge il collegamento come trailer `Claude-Session` sui commit e come collegamento nelle descrizioni delle pull request. Impostatelo su `false` per omettere il collegamento.

* **Ambito**: [`Any file`](#scopes)
* **Tipo**: Boolean
  * `true`: Claude Code aggiunge il collegamento della sessione claude.ai quando esegue il commit o apre una pull request da una sessione cloud o Remote Control
  * `false`: Claude Code omette il collegamento
* **Predefinito**: `true`

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
  Hooks e automazione
</h2>

Registra gli hooks, limita quali hooks vengono eseguiti e controlla i flussi di lavoro. Per gli eventi e i payload degli hooks, consulta il [riferimento degli hooks](/docs/it/hooks).

<h3 id="allowedhttphookurls">
  `allowedHttpHookUrls`
</h3>

Limita gli URL che gli [HTTP hooks](/docs/it/hooks#http-hook-fields) possono raggiungere. Quando definisci questa chiave, Claude Code esegue un HTTP hook solo se il suo URL corrisponde a uno dei modelli e blocca il resto senza eseguirli; un array vuoto blocca ogni HTTP hook.

* **Scope**: [`Any file`](#scopes). Gli array si uniscono tra i file di impostazioni.
* **Type**: array di modelli URL, con `*` come carattere jolly
* **Default**: non impostato, quindi qualsiasi URL è consentito

Questo esempio consente qualsiasi URL sotto `https://hooks.example.com/` e qualsiasi URL `http://localhost`:

```json settings.json theme={null}
{
  "allowedHttpHookUrls": ["https://hooks.example.com/*", "http://localhost:*"]
}
```

La corrispondenza del nome host non distingue tra maiuscole e minuscole e tratta `hooks.example.com.`, con il punto finale che contrassegna un nome di dominio completamente qualificato, allo stesso modo di `hooks.example.com`, come fa il DNS. L'elenco di autorizzazione si applica agli hooks da ogni fonte, incluse le impostazioni gestite.

<h3 id="allowmanagedhooksonly">
  `allowManagedHooksOnly`
</h3>

Limita l'esecuzione degli hooks agli hooks che la tua organizzazione distribuisce.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: solo gli hooks gestiti vengono eseguiti, più gli hooks dell'Agent SDK e gli hooks dai plugin che le tue impostazioni gestite forzano l'abilitazione. Vedi [Cosa viene eseguito sotto `allowManagedHooksOnly`](#what-runs-under-allowmanagedhooksonly)
  * `false`: gli hooks da ogni scope di impostazioni e plugin vengono eseguiti
* **Default**: non impostato, quindi gli hooks da ogni scope di impostazioni e plugin vengono eseguiti

```json managed-settings.json theme={null}
{
  "allowManagedHooksOnly": true
}
```

<h4 id="what-runs-under-allowmanagedhooksonly">
  Cosa viene eseguito sotto `allowManagedHooksOnly`
</h4>

Quando lo imposti su `true`, Claude Code cambia quali hooks e comandi simili agli hooks vengono caricati:

* **Gli hooks gestiti e SDK vengono eseguiti**: gli hooks dalle impostazioni gestite e gli hooks che l'[Agent SDK](/docs/it/agent-sdk/overview) registra nel processo
* **Gli hooks dei plugin forzati vengono eseguiti**: gli hooks dai plugin che le tue impostazioni gestite forzano l'abilitazione tramite [`enabledPlugins`](#enabledplugins). Claude Code corrisponde all'ID completo `plugin@marketplace`, quindi un plugin con lo stesso nome da un marketplace diverso rimane bloccato. Questo ti consente di distribuire hooks verificati tramite un marketplace dell'organizzazione mentre blocchi tutto il resto
* **Tutto il resto è bloccato**: gli hooks dell'utente, del progetto e locali, gli hooks da altri plugin e gli hooks dichiarati nel frontmatter dell'agente
* **I plugin con origine comando sono disabilitati**: Claude Code disabilita anche i plugin con un'[origine `command`](/docs/it/plugins/marketplace-reference#command-plugin-source), inclusi i plugin forzati abilitati in `enabledPlugins` gestito, a meno che tu non imposti [`disableCommandPluginSources`](#disablecommandpluginsources) su `false` esplicitamente
* **I comandi marketplace `headersHelper` sono bloccati**: Claude Code blocca anche i comandi marketplace [`headersHelper`](/docs/it/plugins/host-marketplace#authenticate-archive-downloads) a meno che [`disableCommandPluginSources`](#disablecommandpluginsources) non sia esplicitamente impostato su `false`, ad eccezione di un marketplace che le impostazioni gestite stesse dichiarano. Richiede Claude Code v2.1.238 o successivo
* **La riga di stato e il suggerimento di file si restringono alle impostazioni gestite**: Claude Code legge [`statusLine`](/docs/it/statusline), [`fileSuggestion`](#filesuggestion) e [`subagentStatusLine`](/docs/it/statusline#subagent-status-lines) solo dalle impostazioni gestite, seguendo i [gate della riga di stato e del suggerimento di file](#status-line-and-file-suggestion-gates)

Il comando [`/goal`](/docs/it/goal) non può essere eseguito mentre questa chiave è impostata, perché dipende dagli hooks.

<h3 id="disableallhooks">
  `disableAllHooks`
</h3>

Disattiva gli [hooks](/docs/it/hooks#disable-or-remove-hooks), qualsiasi [riga di stato](/docs/it/statusline) personalizzata e qualsiasi comando [suggerimento di file](#filesuggestion) personalizzato. Usalo per disattivare tutti questi temporaneamente senza eliminarli dalle tue impostazioni.

* **Scope**: [`Any file`](#scopes). Solo le impostazioni gestite possono disabilitare gli hooks gestiti.
* **Type**: Boolean
  * `true`: Claude Code disattiva gli hooks, qualsiasi riga di stato personalizzata e qualsiasi comando suggerimento di file personalizzato
  * `false`: gli hooks, la riga di stato e il comando suggerimento di file vengono eseguiti
* **Default**: non impostato, quindi gli hooks vengono eseguiti

```json settings.json theme={null}
{
  "disableAllHooks": true
}
```

La portata dipende da quale file contiene la chiave:

* **Nelle impostazioni gestite**: Claude Code disabilita ogni hook configurato, inclusi quelli gestiti, e continua a eseguire gli hooks che l'[Agent SDK](/docs/it/agent-sdk/overview) registra nel processo
* **In qualsiasi altro file di impostazioni**: Claude Code disabilita gli hooks dell'utente, del progetto, locali e dei plugin; gli hooks gestiti, gli hooks dell'Agent SDK e gli hooks dai plugin forzati abilitati in [`enabledPlugins`](#enabledplugins) gestito continuano a essere eseguiti

Mantenere gli hooks dell'Agent SDK in esecuzione quando le impostazioni gestite impostano questa chiave richiede Claude Code v2.1.242 o successivo.

Il comando [`/goal`](/docs/it/goal) non può essere eseguito mentre gli hooks sono disabilitati e il menu `/hooks` mostra un avviso invece dei tuoi hooks.

<h4 id="status-line-and-file-suggestion-gates">
  Gate della riga di stato e del suggerimento di file
</h4>

Claude Code prende due decisioni per `statusLine`, `fileSuggestion` e `subagentStatusLine`, in questo ordine:

* **Disattivato completamente**: quando le impostazioni gestite impostano `disableAllHooks`, o quando la cartella non è attendibile secondo la stessa [regola di trust dell'area di lavoro degli hooks nei file di impostazioni](/docs/it/permissions#what-runs-before-you-trust-a-folder)
* **Ristretto alle impostazioni gestite**: quando [`allowManagedHooksOnly`](#allowmanagedhooksonly) è impostato, quando `disableAllHooks` è `true` al di fuori delle impostazioni gestite dopo l'applicazione della [precedenza delle impostazioni](/docs/it/hooks#disable-or-remove-hooks), o quando avvii Claude Code con `--safe-mode`

Sotto il restringimento, Claude Code esegue un valore gestito se uno è distribuito. Altrimenti salta il tuo valore senza avviso: la riga di stato è disabilitata e l'autocompletamento `@` torna al suggerimento di file integrato.

<h3 id="disableworkflows">
  `disableWorkflows`
</h3>

Disattiva i [flussi di lavoro dinamici](/docs/it/workflows#turn-workflows-off) e i comandi del flusso di lavoro in bundle per tutti coloro che le tue impostazioni raggiungono, come un'organizzazione tramite impostazioni gestite. Per attivare o disattivare i flussi di lavoro solo per te stesso, usa [`enableWorkflows`](#enableworkflows) invece, che l'interruttore **Dynamic workflows** in `/config` scrive nelle tue impostazioni utente.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code disattiva i flussi di lavoro dinamici e i comandi del flusso di lavoro in bundle per tutti coloro che le tue impostazioni raggiungono
  * `false`: lo stesso di non impostato; se i flussi di lavoro sono attivi dipende da [`enableWorkflows`](#enableworkflows) e dal default del tuo piano
* **Default**: `false`
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_WORKFLOWS`](/docs/it/env-vars) disattiva i flussi di lavoro per una sessione; qualunque dei due li disattivi, l'altro non può riattivarli

```json settings.json theme={null}
{
  "disableWorkflows": true
}
```

<h3 id="enableworkflows">
  `enableWorkflows`
</h3>

Attiva o disattiva i [flussi di lavoro dinamici](/docs/it/workflows) per te stesso quando il default del tuo piano non è quello che desideri. Appare in `/config` come **Dynamic workflows**, che scrive questa chiave nelle tue impostazioni utente e la rimuove di nuovo quando torni al default del tuo piano. Per disattivare i flussi di lavoro per tutti dalle impostazioni gestite, usa [`disableWorkflows`](#disableworkflows) invece.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code attiva i flussi di lavoro dinamici per te
  * `false`: Claude Code disattiva i flussi di lavoro dinamici per te
* **Default**: non impostato, quindi i flussi di lavoro sono attivi a meno che tu non sia su un piano Pro, dove sono disattivi
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_WORKFLOWS`](/docs/it/env-vars) disattiva i flussi di lavoro per una sessione, e `true` qui non può riattivarli mentre è impostato

```json settings.json theme={null}
{
  "enableWorkflows": true
}
```

[`disableWorkflows`](#disableworkflows) e la politica dei flussi di lavoro della tua organizzazione hanno anche la precedenza: `enableWorkflows: true` non può riattivare i flussi di lavoro mentre una fonte disattiva i flussi di lavoro. Claude Code nasconde la riga `/config` mentre una fonte diversa dalle tue impostazioni utente imposta `enableWorkflows`, o imposta `disableWorkflows` su `true`.

<h3 id="hooks">
  `hooks`
</h3>

Esegui i tuoi comandi, prompt, agenti, richieste HTTP o strumenti MCP come [hooks](/docs/it/hooks) in punti del ciclo di vita di Claude Code, come prima di una chiamata a uno strumento o quando una sessione inizia; il [riferimento degli hooks](/docs/it/hooks#hook-events) elenca ogni evento, il suo payload e i suoi codici di uscita. Ogni evento è mappato a un elenco di gruppi di matcher, e ogni gruppo elenca i gestori da eseguire quando il matcher si applica.

* **Scope**: [`Any file`](#scopes). Gli hooks si uniscono tra i file piuttosto che sostituirsi a vicenda, e gli hooks dalle impostazioni gestite non possono essere rimossi da altri file.
* **Type**: oggetto con chiave [hook event](/docs/it/hooks#hook-events); ogni valore è un array di gruppi `{ "matcher", "hooks" }` le cui voci `hooks` hanno un `type` di `"command"`, `"prompt"`, `"agent"`, `"http"` o `"mcp_tool"`
* **Default**: non impostato, quindi nessun hook viene eseguito

Questo esempio esegue uno script prima di ogni chiamata allo strumento Bash:

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

Per ogni evento, modello di matcher e campo del gestore, consulta il [riferimento degli hooks](/docs/it/hooks#configuration). Per disattivare gli hooks, vedi [`disableAllHooks`](#disableallhooks); per limitare gli hooks a quelli che la tua organizzazione distribuisce, vedi [`allowManagedHooksOnly`](#allowmanagedhooksonly).

<h3 id="httphookallowedenvvars">
  `httpHookAllowedEnvVars`
</h3>

Un [HTTP hook](/docs/it/hooks#http-hook-fields) può inserire il valore di una variabile di ambiente in un'intestazione di richiesta, ad esempio un'intestazione `Authorization: Bearer $HOOK_TOKEN`, ma solo per le variabili che l'hook elenca nel suo `allowedEnvVars`. Questa chiave imposta un limite esterno su tale elenco per ogni HTTP hook: un hook può utilizzare una variabile solo se sia il suo `allowedEnvVars` che questa chiave la nominano. Usalo per impedire a un hook di leggere un segreto che non dovrebbe, anche quando la definizione dell'hook lo richiede.

* **Scope**: [`Any file`](#scopes). Gli array si uniscono tra i file di impostazioni.
* **Type**: array di nomi di variabili di ambiente
* **Default**: non impostato, quindi l'elenco `allowedEnvVars` di ogni hook si applica

Questo esempio limita l'interpolazione dell'intestazione a `MY_TOKEN` e `HOOK_SECRET`:

```json settings.json theme={null}
{
  "httpHookAllowedEnvVars": ["MY_TOKEN", "HOOK_SECRET"]
}
```

L'elenco di autorizzazione si applica agli hooks da ogni fonte, incluse le impostazioni gestite.

<h3 id="workflowkeywordtriggerenabled">
  `workflowKeywordTriggerEnabled`
</h3>

Scegli se digitare la parola chiave `ultracode` in un prompt attiva un [flusso di lavoro dinamico](/docs/it/workflows#ask-for-a-workflow-in-your-prompt). Impostalo su `false` per digitare la parola senza attivarne uno.

* **Scope**: [`Any file`](#scopes). Appare in `/config` come **Ultracode keyword trigger**.
* **Type**: Boolean
  * `true`: digitare `ultracode` in un prompt attiva un flusso di lavoro dinamico
  * `false`: puoi digitare la parola senza attivarne uno
* **Default**: `true`

```json settings.json theme={null}
{
  "workflowKeywordTriggerEnabled": false
}
```

L'impostazione dello sforzo `ultracode`, `/workflows` e i comandi del flusso di lavoro salvati non sono interessati.

<h3 id="workflowsizeguideline">
  `workflowSizeGuideline`
</h3>

Imposta il [numero di agenti a cui Claude mira](/docs/it/workflows#set-a-size-guideline) nei flussi di lavoro dinamici che scrive. Claude Code invia il valore a Claude come consiglio, non come limite imposto: `"small"` chiede meno di 5 agenti, `"medium"` meno di 10 e `"large"` meno di 50. Scegli `"small"` quando vuoi limitare ciò che un flusso di lavoro spende. Richiede Claude Code v2.1.219 o successivo.

* **Scope**: [`Any file`](#scopes). Un valore lì ha la precedenza sulla scelta **Dynamic workflow size** in `/config`, che Claude Code memorizza in `~/.claude.json`, e Claude Code nasconde quella riga mentre un file di impostazioni imposta la chiave.
* **Type**: stringa, uno di:
  * `"unrestricted"`: nessuna linea guida, quindi Claude dimensiona il flusso di lavoro al compito
  * `"small"`: Claude mira a meno di 5 agenti
  * `"medium"`: Claude mira a meno di 10 agenti
  * `"large"`: Claude mira a meno di 50 agenti
* **Default**: `"medium"`, o `"small"` quando sei connesso su un piano Pro con Claude Code v2.1.271 o successivo

```json settings.json theme={null}
{
  "workflowSizeGuideline": "small"
}
```

Richiede Claude Code v2.1.219 o successivo; su v2.1.202 attraverso v2.1.218, imposta la linea guida in `/config` invece.

<span id="plugin-configuration" />

<span id="manage-plugins" />

<span id="plugin-settings" />

<h2 id="plugins-and-skills">
  Plugin e skills
</h2>

Abilita i plugin, registra i marketplace, limita le fonti di plugin che un'organizzazione consente e controlla quali skills si caricano. Per l'installazione e la creazione di plugin, vedi [Plugin](/docs/it/plugins/overview).

<h3 id="disablebundledskills">
  `disableBundledSkills`
</h3>

Disattiva le [skills](/docs/it/skills) e i flussi di lavoro inclusi con Claude Code. Claude Code rimuove completamente le skills e i flussi di lavoro in bundle, mentre i comandi incorporati come `/init` rimangono digitabili ma sono nascosti dal modello.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code rimuove le skills e i flussi di lavoro in bundle e nasconde i comandi incorporati come `/init` dal modello
  * `false`: le skills in bundle si caricano
* **Default**: non impostato, quindi le skills in bundle si caricano
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_BUNDLED_SKILLS`](/docs/it/env-vars) impostato su `1` disattiva le skills in bundle per una sessione; qualunque dei due le disattivi, l'altro non può riattivarle

```json settings.json theme={null}
{
  "disableBundledSkills": true
}
```

Le skills dai plugin, da `.claude/skills/` e da `.claude/commands/` non sono interessate. `/doctor` rimane digitabile come i comandi incorporati; per nasconderlo, imposta [`DISABLE_DOCTOR_COMMAND`](/docs/it/env-vars) invece.

<h3 id="disableskillshellexecution">
  `disableSkillShellExecution`
</h3>

Disattiva l'esecuzione inline della shell per `` !`...` `` e ` ```! ` blocchi in [skills](/it/skills) e comandi personalizzati da fonti utente, progetto, plugin o directory aggiuntiva. Claude Code sostituisce ogni comando con `[shell command execution disabled by policy]` invece di eseguirlo.

* **Scope**: [`Any file`](#scopes). Un `true` nelle impostazioni gestite non può essere sovrascritto da `false` altrove.
* **Type**: Boolean
  * `true`: Claude Code sostituisce ogni comando shell inline con `[shell command execution disabled by policy]` invece di eseguirlo
  * `false`: la shell inline si esegue
* **Default**: non impostato, quindi la shell inline si esegue

```json settings.json theme={null}
{
  "disableSkillShellExecution": true
}
```

Le skills in bundle e le skills distribuite tramite impostazioni gestite non sono interessate.

<h3 id="skilloverrides">
  `skillOverrides`
</h3>

Nascondi o comprimi una [skill](/docs/it/skills#override-skill-visibility-from-settings) senza modificare il suo `SKILL.md`. Claude Code applica il valore sotto il nome di ogni skill all'elenco delle skills che Claude vede e al tuo completamento automatico `/`.

* **Scope**: [`Any file`](#scopes). Il menu `/skills` scrive in `.claude/settings.local.json`.
* **Type**: oggetto che mappa il nome della skill a uno di:
  * `"on"`: Claude vede la skill e puoi digitare `/name`
  * `"name-only"`: Claude vede la skill per nome senza la sua descrizione
  * `"user-invocable-only"`: Claude non vede la skill, ma puoi comunque digitare `/name`
  * `"off"`: Claude non vede la skill e `/name` è nascosto dal completamento automatico
* **Default**: non impostato, quindi ogni skill è `"on"`

Questo esempio elenca `legacy-context` a Claude solo per nome e nasconde `deploy` da Claude e dal completamento automatico `/`:

```json settings.json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

Gli override non si applicano alle skills dei plugin, che gestisci tramite `/plugin`.

Nelle impostazioni gestite e nei file passati con `--settings`, una chiave su un alias di una skill in bundle, come `checkup` per `/doctor`, si applica anche alla skill; vedi [come le chiavi alias si combinano con le chiavi sul nome della skill stessa](/docs/it/skills#override-skill-visibility-from-settings).

<h3 id="syncclaudeaiskills">
  `syncClaudeAiSkills`
</h3>

Disattiva il download delle [skills abilitate per il tuo account claude.ai](/docs/it/skills#how-synced-skills-behave). Claude Code le scarica in `~/.claude/skills/synced/` nelle [sessioni di terminale in cui accedi con il tuo account claude.ai](/docs/it/skills#where-synced-skills-load), interattive o non interattive, e nelle sessioni Cowork e cloud. Imposta `false` per interrompere quel download e smettere di caricare le skills che ha già sincronizzato. Claude Code onora solo `false`: `true` è lo stesso di non impostato e non attiva la sincronizzazione dove è altrimenti disattivata.

* **Scope**: [`User, local, or managed`](#scopes), e file passati con `--settings`. Un repository non può disattivarla per te.
* **Type**: Boolean
  * `false`: Claude Code interrompe il download delle skills sincronizzate e smette di caricare quelle già in `~/.claude/skills/synced/`. Nelle impostazioni utente o gestite, le sposta anche in `~/.claude/skills/.trash/`
  * `true`: lo stesso di non impostato
* **Default**: non impostato, quindi le sessioni che accedono con il tuo account claude.ai sincronizzano le tue skills

Questo esempio impedisce a una macchina di scaricare le skills dell'account in qualsiasi sessione:

```json settings.json theme={null}
{
  "syncClaudeAiSkills": false
}
```

<h3 id="syncclaudeaiplugins">
  `syncClaudeAiPlugins`
</h3>

Disattiva il download dei [plugin abilitati per il tuo account claude.ai](/docs/it/plugins/loading#synced-plugins). Claude Code li scarica in `~/.claude/plugins/synced/` all'inizio delle sessioni di terminale in cui accedi con il tuo account claude.ai e nelle sessioni Cowork, e carica ognuno come `<name>@synced`. Imposta `false` per interrompere quel download e smettere di caricare i plugin che ha già sincronizzato. Claude Code onora solo `false`: `true` è lo stesso di non impostato e non attiva la sincronizzazione dove è altrimenti disattivata. Richiede Claude Code v2.1.273 o successivo.

* **Scope**: [`User, local, or managed`](#scopes), e file passati con `--settings`. Un repository non può disattivarla per te.
* **Type**: Boolean
  * `false`: Claude Code interrompe il download dei plugin sincronizzati e smette di caricare quelli già in `~/.claude/plugins/synced/`. Nelle impostazioni utente o gestite, li sposta anche in `~/.claude/plugins/.trash/`
  * `true`: lo stesso di non impostato
* **Default**: non impostato, quindi le sessioni che accedono con il tuo account claude.ai sincronizzano i tuoi plugin

Per disattivare un plugin sincronizzato piuttosto che tutti, imposta `"<name>@synced": false` in [`enabledPlugins`](#enabledplugins).

Questo esempio impedisce a una macchina di scaricare i plugin dell'account in qualsiasi sessione:

```json settings.json theme={null}
{
  "syncClaudeAiPlugins": false
}
```

<h3 id="allowedchannelplugins">
  `allowedChannelPlugins`
</h3>

Scegli quali plugin di [channel](/docs/it/channels) possono inviare messaggi nelle sessioni della tua organizzazione. Quando lo imposti, Claude Code utilizza il tuo elenco al posto della lista di autorizzazione predefinita di Anthropic; ogni voce nomina un plugin e il marketplace da cui proviene.

* **Scope**: [`Managed`](#scopes)
* **Type**: array di oggetti, ciascuno con stringhe `marketplace` e `plugin`. Una voce può invece essere una stringa `"plugin@marketplace"` come `"telegram@claude-plugins-official"`, che Claude Code tratta come l'oggetto equivalente. La forma stringa richiede Claude Code v2.1.267 o successivo; le versioni precedenti rifiutano l'intero valore `allowedChannelPlugins` quando ne contiene una
* **Default**: non impostato, quindi Claude Code utilizza la lista di autorizzazione predefinita di Anthropic

Questo esempio attiva i channel e consente solo il plugin Telegram dal marketplace ufficiale di Anthropic:

```json managed-settings.json theme={null}
{
  "channelsEnabled": true,
  "allowedChannelPlugins": [
    { "marketplace": "claude-plugins-official", "plugin": "telegram" }
  ]
}
```

Un array vuoto blocca ogni plugin di channel.

Questa chiave ha effetto una volta che i channel superano il gate [`channelsEnabled`](#channelsenabled) per l'account: su piani Team e Enterprise, e su account Console con impostazioni gestite, significa `channelsEnabled: true`. Vedi [Limita quali plugin di channel possono essere eseguiti](/docs/it/channels#restrict-which-channel-plugins-can-run).

<h3 id="blockedmarketplaces">
  `blockedMarketplaces`
</h3>

Blocca le fonti di marketplace dei plugin per la tua organizzazione. Claude Code controlla la lista di blocco all'aggiunta del marketplace e all'installazione, aggiornamento, aggiornamento e auto-aggiornamento del plugin, quindi un marketplace che qualcuno ha aggiunto prima di impostare la policy non può essere utilizzato per recuperare plugin. Le fonti bloccate vengono controllate prima del download, quindi non toccano mai il filesystem.

Se imposti questa chiave nella [console di amministrazione claude.ai](/docs/it/server-managed-settings), claude.ai la applica anche quando chiunque nella tua organizzazione aggiunge un marketplace da un repository git su claude.ai, come [Come funzionano le restrizioni](/docs/it/plugins/org#restrict-what-users-can-install) descrive.

* **Scope**: [`Managed`](#scopes)
* **Type**: array di oggetti di fonte marketplace, nelle stesse forme di [`strictKnownMarketplaces`](#allowed-source-types)
* **Default**: non impostato, quindi nessun marketplace è bloccato

Questo esempio blocca un repository GitHub come fonte di marketplace:

```json managed-settings.json theme={null}
{
  "blockedMarketplaces": [
    { "source": "github", "repo": "untrusted/plugins" }
  ]
}
```

Una voce `github` può utilizzare la forma [owner-wildcard](#owner-wildcards) `"owner/*"` per bloccare ogni repository sotto quel proprietario GitHub, che richiede Claude Code v2.1.223 o successivo. Aggiungi `{ "source": "skills-dir" }` per impedire a Claude Code di caricare i plugin [`@skills-dir`](/docs/it/plugins/loading#plugins-shared-through-a-repository) da `~/.claude/skills/` senza limitare alcun marketplace. Vedi [Restrizioni di marketplace gestite](/docs/it/plugins/org#restrict-what-users-can-install).

<h3 id="channelsenabled">
  `channelsEnabled`
</h3>

Consenti i [channel](/docs/it/channels) per la tua organizzazione. Su piani Team e Enterprise di claude.ai, Claude Code blocca i channel finché non imposti questo su `true`. Per account [Anthropic Console](/docs/it/authentication#claude-console-authentication) che si autenticano con una chiave API, i channel sono consentiti per impostazione predefinita. Se la tua organizzazione distribuisce impostazioni gestite, Claude Code blocca i channel anche su quegli account finché non imposti questa chiave su `true`.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code consente i channel per la tua organizzazione
  * `false`: lo stesso di non impostato; se i channel sono bloccati dipende dal tuo piano, come dice il Default
* **Default**: non impostato; i channel sono bloccati su piani Team e Enterprise e su account Console con impostazioni gestite, e consentiti su piani Pro e Max e su account Console senza impostazioni gestite

```json managed-settings.json theme={null}
{
  "channelsEnabled": true
}
```

Per limitare quali plugin possono registrarsi come channel una volta abilitati, imposta [`allowedChannelPlugins`](#allowedchannelplugins). Vedi [Controlli Enterprise](/docs/it/channels#enterprise-controls).

<h3 id="disablecommandpluginsources">
  `disableCommandPluginSources`
</h3>

Blocca la [fonte plugin `command`](/docs/it/plugins/marketplace-reference#command-plugin-source), che installa un plugin eseguendo un comando dichiarato dal marketplace sulla macchina dell'utente. Quando lo imposti su `true`, Claude Code non esegue mai il comando, non installa o aggiorna plugin con origine comando e interrompe il caricamento di quelli già installati. Imposta su `false` per consentirli esplicitamente. Ogni volta che blocca le fonti comando, che tu lo imposti su `true` o lo lasci non impostato sotto [`allowManagedHooksOnly`](#allowmanagedhooksonly), blocca anche i comandi [`headersHelper`](/docs/it/plugins/host-marketplace#authenticate-archive-downloads) del marketplace, tranne per un marketplace che le impostazioni gestite stesse dichiarano. Richiede Claude Code v2.1.229 o successivo, e il blocco `headersHelper` richiede v2.1.238 o successivo.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code non esegue mai il comando dichiarato dal marketplace, non installa o aggiorna plugin con origine comando e interrompe il caricamento di quelli già installati
  * `false`: Claude Code consente i plugin con origine comando esplicitamente
* **Default**: non impostato, quindi Claude Code segue [`allowManagedHooksOnly`](#allowmanagedhooksonly): un'organizzazione che limita l'esecuzione degli hook alle impostazioni gestite ottiene anche le fonti comando disabilitate

```json managed-settings.json theme={null}
{
  "disableCommandPluginSources": true
}
```

Richiede Claude Code v2.1.229 o successivo.

<h3 id="pluginsuggestionmarketplaces">
  `pluginSuggestionMarketplaces`
</h3>

Nomina i marketplace i cui plugin possono apparire come suggerimenti di installazione contestuali, nei suggerimenti spinner e fissati in cima alla scheda **Discover** di `/plugin`. Il suggerimento incorporato di prima parte per il design del frontend non è interessato. I suggerimenti provengono dalla dichiarazione `relevance` di ogni plugin nella sua voce di marketplace.

* **Scope**: [`Managed`](#scopes)
* **Type**: array di nomi di marketplace
* **Default**: non impostato, quindi nessun suggerimento dichiarato dal marketplace viene visualizzato

```json managed-settings.json theme={null}
{
  "pluginSuggestionMarketplaces": ["acme-corp-plugins"]
}
```

Un nome ha effetto solo quando il marketplace è registrato sulla macchina e la sua fonte registrata è anche dichiarata nelle stesse impostazioni gestite, come voce [`extraKnownMarketplaces`](#extraknownmarketplaces) per quel nome o come voce di [`strictKnownMarketplaces`](#strictknownmarketplaces). Claude Code ignora un marketplace registrato da una fonte diversa sotto un nome nella lista di autorizzazione. Il marketplace ufficiale è esente dal requisito di fonte: autorizzare solo il suo nome è sufficiente, poiché quel nome può registrarsi solo dalla fonte ufficiale di Anthropic. Vedi [Suggerisci plugin per contesto](/docs/it/plugins/relevance).

<h3 id="plugintrustmessage">
  `pluginTrustMessage`
</h3>

Aggiungi il testo della tua organizzazione all'avviso di fiducia del plugin che Claude Code mostra prima dell'installazione, ad esempio per confermare che i plugin dal tuo marketplace interno sono controllati.

* **Scope**: [`Managed`](#scopes)
* **Type**: string
* **Default**: non impostato, quindi Claude Code mostra solo l'avviso standard

```json managed-settings.json theme={null}
{
  "pluginTrustMessage": "All plugins from our marketplace are approved by IT"
}
```

<h3 id="strictknownmarketplaces">
  `strictKnownMarketplaces`
</h3>

Limita le fonti di marketplace dei plugin da cui le persone nella tua organizzazione possono aggiungere e installare plugin. Claude Code applica la lista di autorizzazione all'aggiunta del marketplace e all'installazione, aggiornamento, aggiornamento e auto-aggiornamento del plugin, prima di qualsiasi operazione di rete o filesystem, quindi un marketplace che qualcuno ha aggiunto prima di impostare la policy non può essere utilizzato per recuperare plugin una volta che la sua fonte non corrisponde più. Gli utenti bloccati vedono un errore che nomina la policy gestita.

Se imposti questa chiave nella [console di amministrazione claude.ai](/docs/it/server-managed-settings), claude.ai la applica anche quando chiunque nella tua organizzazione aggiunge un marketplace da un repository git su claude.ai, come [Come funzionano le restrizioni](/docs/it/plugins/org#restrict-what-users-can-install) descrive.

* **Scope**: [`Managed`](#scopes)
* **Type**: array di oggetti di fonte marketplace; vedi [Tipi di fonte consentiti](#allowed-source-types)
* **Default**: non impostato, quindi gli utenti possono aggiungere qualsiasi marketplace. Un array vuoto è un blocco completo che blocca ogni fonte di marketplace, incluso il marketplace ufficiale di Anthropic

Questo esempio consente due repository GitHub, uno fissato al ref `v2.0` e uno URL di `marketplace.json` ospitato:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/approved-plugins" },
    { "source": "github", "repo": "acme-corp/security-tools", "ref": "v2.0" },
    { "source": "url", "url": "https://plugins.example.com/marketplace.json" }
  ]
}
```

Puoi anche scrivere questa chiave come `allowedMarketplaces`; [Alias di chiave Marketplace](#marketplace-key-aliases) descrive come Claude Code tratta l'alias e quale versione lo accetta. Questa chiave è un gate di policy: controlla cosa gli utenti possono aggiungere ma non registra nulla. Per limitare e pre-registrare in un file, vedi [Combina con `extraKnownMarketplaces`](#combine-with-extraknownmarketplaces). Per la vista rivolta all'utente, vedi [Restrizioni di marketplace gestite](/docs/it/plugins/org#restrict-what-users-can-install).

<h4 id="allowed-source-types">
  Tipi di fonte consentiti
</h4>

Ogni voce di seguito mostra una voce della lista di autorizzazione per tipo di fonte e i campi che accetta. La maggior parte dei tipi corrisponde esattamente; `hostPattern` e `pathPattern` corrispondono per regex, e le voci `github` possono utilizzare un [wildcard del proprietario](#owner-wildcards).

| Source        | Example entry                                                                                                                   | Fields                                                                                                 |
| :------------ | :------------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------- |
| `github`      | `{ "source": "github", "repo": "acme-corp/plugins", "ref": "main", "path": "marketplace" }`                                     | `repo` obbligatorio; `ref` è un ramo o un tag; `path` è una sottodirectory                             |
| `git`         | `{ "source": "git", "url": "https://gitlab.example.com/tools/plugins.git", "ref": "production" }`                               | `url` obbligatorio; `ref` e `path` come per `github`                                                   |
| `url`         | `{ "source": "url", "url": "https://plugins.example.com/marketplace.json", "headers": { "Authorization": "Bearer ${TOKEN}" } }` | `url` obbligatorio; `headers` aggiunge intestazioni HTTP per l'accesso autenticato                     |
| `file`        | `{ "source": "file", "path": "/opt/acme-corp/plugins/marketplace.json" }`                                                       | `path` obbligatorio, il percorso assoluto a un file `marketplace.json`                                 |
| `directory`   | `{ "source": "directory", "path": "/opt/acme-corp/approved-marketplaces" }`                                                     | `path` obbligatorio, il percorso assoluto a una directory contenente `.claude-plugin/marketplace.json` |
| `hostPattern` | `{ "source": "hostPattern", "hostPattern": "^github\\.example\\.com$" }`                                                        | `hostPattern` obbligatorio, una regex confrontata con l'host del marketplace                           |
| `pathPattern` | `{ "source": "pathPattern", "pathPattern": "^/opt/approved/" }`                                                                 | `pathPattern` obbligatorio, una regex confrontata con il `path` delle fonti `file` e `directory`       |
| `skills-dir`  | `{ "source": "skills-dir" }`                                                                                                    | Nessun campo. Riattiva la scansione del plugin `~/.claude/skills/`                                     |

Tre tipi di fonte portano regole oltre la tabella:

* **`url`**: un marketplace URL scarica solo il file `marketplace.json` e Claude Code non recupera i file plugin per percorso relativo da quel server, quindi i suoi plugin devono utilizzare una [fonte plugin](/docs/it/plugins/marketplace-reference#plugin-sources) diversa da un percorso relativo, come un URL di archivio, che può essere sullo stesso host. Per i plugin con percorsi relativi, utilizza un marketplace basato su Git. Vedi [I plugin con percorsi relativi falliscono nei marketplace basati su URL](/docs/it/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces).
* **`hostPattern`**: usalo per consentire ogni marketplace su un server GitHub Enterprise o GitLab interno senza elencare ogni repository. Claude Code confronta le fonti `github` con `github.com`, prende il nome host dalle fonti `url` e lo prende dalle fonti `git` a seconda della forma dell'[URL git](https://git-scm.com/docs/git-clone#_git_urls):

  * Un URL con uno schema, come `https://` o `ssh://`: il nome host nell'URL.
  * Un indirizzo SSH senza schema, nella forma `user@host:path` di git, come `git@git.example.com:tools/plugins.git`: l'host tra `@` e `:`, che è l'host a cui git si connette.
  * Qualsiasi altra forma senza schema: nessun host, quindi nessuna voce `strictKnownMarketplaces` `hostPattern` la corrisponde. Per una `blockedMarketplaces` `hostPattern`, Claude Code prende un host da un insieme più ampio di forme, quindi una voce della lista di blocco può comunque corrispondere a tale forma. Prima di v2.1.234, una `strictKnownMarketplaces` `hostPattern` corrispondeva anche ad alcune forme che git non tratta come indirizzi SSH.

  Le fonti `file` e `directory` non hanno host e non corrispondono mai a una voce `hostPattern`.
* **`pathPattern`**: usalo per consentire marketplace del filesystem insieme alle voci `hostPattern` per le fonti di rete. `".*"` consente ogni percorso locale; un pattern più stretto come `"^/opt/approved/"` limita a una directory.

Qualsiasi lista di autorizzazione, anche una vuota, interrompe anche il caricamento dei plugin [`@skills-dir`](/docs/it/plugins/loading#plugins-shared-through-a-repository) da `~/.claude/skills/` da parte di Claude Code. Aggiungi la voce `{ "source": "skills-dir" }` per continuare a caricarli; la voce non ha significato al di fuori di questa chiave e `blockedMarketplaces`.

<h4 id="owner-wildcards">
  Wildcard del proprietario
</h4>

Una voce `github` il cui valore `repo` è `"<owner>/*"` corrisponde a ogni repository sotto quel proprietario GitHub. I wildcard del proprietario richiedono Claude Code v2.1.223 o successivo e funzionano solo in `strictKnownMarketplaces` e `blockedMarketplaces`. Ovunque altrove appaia una fonte `github`, come `extraKnownMarketplaces` o `/plugin marketplace add`, il valore `repo` deve nominare un singolo repository. Prima di v2.1.223, Claude Code confrontava la voce letteralmente, quindi una voce della lista di autorizzazione non corrispondeva a nessun repository e una voce della lista di blocco non bloccava nulla; le voci di singolo repository vengono applicate su ogni versione.

Questa voce consente qualsiasi repository di marketplace nell'organizzazione `acme-corp`:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/*" }
  ]
}
```

Solo l'intera posizione del nome del repository può essere un wildcard. Claude Code confronta voci come `*`, `*/plugins` o `acme-corp/tools-*` letteralmente, quindi non corrispondono a nessun repository.

Le regole di corrispondenza differiscono tra le due impostazioni:

| Rule                      | `strictKnownMarketplaces`                                                                                                                                                                    | `blockedMarketplaces`                                                                         |
| ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| Matching source spellings | Solo forma `owner/repo`. Un URL git che clona lo stesso repository non corrisponde                                                                                                           | Qualsiasi ortografia, inclusi gli URL git che si risolvono nello stesso repository github.com |
| Owner case                | Sensibile alle maiuscole, come la corrispondenza esatta della voce                                                                                                                           | Insensibile alle maiuscole                                                                    |
| `ref`                     | Segue le regole di corrispondenza esatta: una voce con un `ref` corrisponde solo alle fonti con quel ref esatto, e una voce senza uno corrisponde solo alle fonti che non specificano un ref | Una voce senza un `ref` blocca tutti i ref dei repository che corrisponde                     |
| `path`                    | Più lasco delle regole di corrispondenza esatta: una voce con un `path` richiede quel valore esatto, mentre una voce senza uno corrisponde a qualsiasi percorso all'interno del repository   | Una voce senza un `path` blocca tutti i percorsi dei repository che corrisponde               |

<h4 id="exact-matching">
  Corrispondenza esatta
</h4>

Per ogni tipo di fonte tranne le voci `github` con wildcard del proprietario e le voci `hostPattern` e `pathPattern` confrontate per regex, Claude Code consente l'aggiunta di un utente solo quando la fonte del marketplace corrisponde a una voce esattamente. Per le fonti basate su git `github` e `git`, la corrispondenza esatta include i campi facoltativi:

* Il `repo` o `url` deve corrispondere esattamente
* Il campo `ref` deve corrispondere esattamente, o entrambi devono essere non definiti
* Il campo `path` deve corrispondere esattamente, o entrambi devono essere non definiti

Ad esempio, Claude Code tratta ogni coppia di seguito come due fonti diverse:

* `{ "source": "github", "repo": "acme-corp/plugins" }` e `{ "source": "github", "repo": "acme-corp/plugins", "ref": "main" }`
* `{ "source": "github", "repo": "acme-corp/plugins", "path": "marketplace" }` e `{ "source": "github", "repo": "acme-corp/plugins" }`

<h4 id="allow-only-the-official-marketplace">
  Consenti solo il marketplace ufficiale
</h4>

Per consentire il marketplace ufficiale di Anthropic e nient'altro, elenca il suo repository:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "anthropics/claude-plugins-official" }
  ]
}
```

Con questa voce, Claude Code mantiene disponibile un marketplace ufficiale già registrato e, su una macchina nuova, registra il marketplace automaticamente la prima volta che avvii una sessione di terminale interattiva. La registrazione automatica più comunemente manca:

* Ambienti non interattivi che vengono eseguiti prima della prima sessione di terminale interattiva della macchina.
* Macchine dove Claude Code è stato eseguito solo tramite l'estensione VS Code.
* Macchine dove Claude Code ha già eseguito una sessione di terminale interattiva sotto una policy che ha bloccato il marketplace, come il blocco dell'array vuoto. Claude Code registra il tentativo bloccato e non ritenta dopo il cambio della policy.

Su queste macchine, aggiungi il marketplace a [`extraKnownMarketplaces`](#extraknownmarketplaces) nello stesso `managed-settings.json` in modo che Claude Code lo registri automaticamente, o esegui `claude plugin marketplace add anthropics/claude-plugins-official`.

<h4 id="combine-with-extraknownmarketplaces">
  Combina con `extraKnownMarketplaces`
</h4>

Le due chiavi svolgono lavori diversi. Questa tabella le confronta:

| Aspect            | `strictKnownMarketplaces`                   | `extraKnownMarketplaces`                                                                                                                   |
| ----------------- | ------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| Purpose           | Applicazione della policy organizzativa     | Comodità del team                                                                                                                          |
| Settings file     | Solo impostazioni gestite                   | Qualsiasi file di impostazioni                                                                                                             |
| Behavior          | Blocca le aggiunte non autorizzate          | Registra i marketplace mancanti                                                                                                            |
| When enforced     | Prima delle operazioni di rete e filesystem | Immediatamente dalle impostazioni utente o gestite; dopo la finestra di dialogo di fiducia dell'area di lavoro per i file di un repository |
| Can be overridden | No, precedenza massima                      | Sì, da impostazioni di precedenza superiore                                                                                                |
| Source format     | Oggetto di fonte diretto                    | Marketplace denominato con un oggetto `source` annidato                                                                                    |

Per limitare e pre-registrare un marketplace per tutti gli utenti, imposta entrambi in `managed-settings.json`:

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

Con solo `strictKnownMarketplaces` impostato, gli utenti possono comunque aggiungere un marketplace autorizzato da soli con `/plugin marketplace add`. Il marketplace ufficiale di Anthropic è l'unico che Claude Code registra automaticamente, e solo quando la lista di autorizzazione lo consente. [Consenti solo il marketplace ufficiale](#allow-only-the-official-marketplace) elenca le macchine che manca.

<h3 id="strictpluginonlycustomization">
  `strictPluginOnlyCustomization`
</h3>

Blocca skills, agenti, hooks e server MCP da fonti utente e progetto, quindi possono provenire solo da plugin o impostazioni gestite. Combinalo con [`strictKnownMarketplaces`](#strictknownmarketplaces) per controllare l'intera catena di approvvigionamento della personalizzazione: la lista di autorizzazione del marketplace controlla quali plugin gli utenti possono installare.

* **Scope**: [`Managed`](#scopes)
* **Type**: `true` per bloccare tutti e quattro i tipi di personalizzazione, o un array che nomina i tipi da bloccare, da `"skills"`, `"agents"`, `"hooks"` e `"mcp"`
* **Default**: non impostato, quindi nulla è bloccato

Questo esempio blocca skills e hooks e lascia agenti e server MCP sbloccati:

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["skills", "hooks"]
}
```

Le quattro voci di sub-chiave di seguito elencano cosa blocca ogni superficie e cosa continua a caricarsi. Claude Code ignora i nomi di superficie che non riconosce piuttosto che fallire il file di impostazioni, quindi puoi aggiungere nuovi nomi di superficie prima che ogni client si sia aggiornato.

<h3 id="strictpluginonlycustomization-skills">
  `strictPluginOnlyCustomization.skills`
</h3>

Blocca la superficie `skills`. Claude Code interrompe il caricamento delle skills da `~/.claude/skills/` e `.claude/skills/`, comandi personalizzati da `~/.claude/commands/` e `.claude/commands/`, skills sotto directory `--add-dir` e skills sincronizzate dal tuo account claude.ai, e continua a caricare skills dei plugin, skills in bundle e skills nella directory della policy gestita.

* **Scope**: [`Managed`](#scopes)
* **Type**: la stringa `"skills"` nell'array [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default**: non bloccato

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["skills"]
}
```

<h3 id="strictpluginonlycustomization-agents">
  `strictPluginOnlyCustomization.agents`
</h3>

Blocca la superficie `agents`. Claude Code interrompe il caricamento degli agenti da `~/.claude/agents/` e `.claude/agents/`, e continua a caricare agenti dei plugin, agenti incorporati e agenti nella directory della policy gestita.

* **Scope**: [`Managed`](#scopes)
* **Type**: la stringa `"agents"` nell'array [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default**: non bloccato

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["agents"]
}
```

<h3 id="strictpluginonlycustomization-hooks">
  `strictPluginOnlyCustomization.hooks`
</h3>

Blocca la superficie `hooks`. Claude Code interrompe l'esecuzione degli hooks da impostazioni utente, progetto e locale `settings.json`, e continua a eseguire hooks dei plugin e hooks nelle impostazioni gestite.

* **Scope**: [`Managed`](#scopes)
* **Type**: la stringa `"hooks"` nell'array [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default**: non bloccato

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["hooks"]
}
```

<h3 id="strictpluginonlycustomization-mcp">
  `strictPluginOnlyCustomization.mcp`
</h3>

Blocca la superficie `mcp`. Claude Code interrompe il caricamento dei server MCP da `~/.claude.json` e `.mcp.json`, e continua a caricare server MCP dei plugin, server [`managed-mcp.json`](/docs/it/managed-mcp) e server da [`managedMcpServers`](#managedmcpservers).

* **Scope**: [`Managed`](#scopes)
* **Type**: la stringa `"mcp"` nell'array [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default**: non bloccato

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["mcp"]
}
```

<h3 id="enabledplugins">
  `enabledPlugins`
</h3>

Attiva o disattiva i singoli [plugin](/docs/it/plugins/overview) codificati per `plugin-name@marketplace-name`. Un plugin senza voce in alcuno scope ricade al suo valore [`defaultEnabled`](/docs/it/plugins/manifest-reference#fields). Quando abiliti o disabiliti un plugin con `/plugin` o `claude plugin enable`, Claude Code scrive questa chiave per te.

* **Scope**: [`Any file`](#scopes)
* **Type**: oggetto che mappa `plugin-name@marketplace-name` a un Boolean
* **Default**: non impostato, quindi ogni plugin segue il suo valore `defaultEnabled`

Questo esempio abilita due plugin dal marketplace `team-tools` e disabilita uno da `personal`:

```json settings.json theme={null}
{
  "enabledPlugins": {
    "code-formatter@team-tools": true,
    "deployment-tools@team-tools": true,
    "experimental-features@personal": false
  }
}
```

Ogni scope serve a uno scopo diverso:

* **Impostazioni utente**: le tue preferenze personali di plugin
* **Impostazioni progetto**: plugin condivisi con tutti nel repository
* **Impostazioni locali**: override per macchina, gitignored quando Claude Code salva un'impostazione lì
* **Impostazioni gestite**: policy a livello di organizzazione. Un plugin impostato su `false` qui è bloccato dall'installazione in ogni scope e nascosto dal marketplace

Le impostazioni del progetto hanno precedenza sulle impostazioni utente, quindi impostare un plugin su `false` in `~/.claude/settings.json` non disabilita un plugin che il `.claude/settings.json` del progetto abilita. Per rinunciare a un plugin abilitato dal progetto sulla tua macchina, impostalo su `false` in `.claude/settings.local.json` invece. I plugin forzatamente abilitati dalle impostazioni gestite non possono essere disabilitati in questo modo, poiché le impostazioni gestite sovrascrivono le impostazioni locali.

Abilitare un plugin da una fonte esterna come un repository GitHub o un pacchetto npm nel `.claude/settings.json` di un progetto non lo installa per altre persone. Su ogni percorso che carica i plugin, Claude Code segnala il plugin come non installato finché ogni utente non lo [installa da solo](/docs/it/plugins/org#require-plugins-per-repository).

<h3 id="extraknownmarketplaces">
  `extraKnownMarketplaces`
</h3>

Registra marketplace di plugin aggiuntivi per nome, in modo che le persone che aprono il repository, o tutti quelli che le impostazioni gestite raggiungono, ottengono il marketplace senza aggiungerlo da soli. Claude Code registra ogni marketplace che non conosce già. Se un plugin che [`enabledPlugins`](#enabledplugins) nomina da esso si installa dipende dalla fonte del plugin e da quale file lo abilita; quella voce ha le regole.

* **Scope**: [`Any file`](#scopes). Claude Code onora le voci nel `.claude/settings.json` o `.claude/settings.local.json` di un repository solo dopo che accetti la finestra di dialogo di fiducia dell'area di lavoro per quella cartella; in una cartella che non hai fidato, inclusa un'esecuzione `-p` lì, le ignora senza un messaggio.
* **Type**: oggetto che mappa un nome di marketplace a un oggetto con un oggetto `source` e un Boolean `autoUpdate` facoltativo
* **Default**: non impostato

Questo esempio registra un marketplace GitHub e un marketplace da un URL git auto-ospitato:

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

[Cosa viene eseguito prima di fidare una cartella](/docs/it/permissions#what-runs-before-you-trust-a-folder) confronta il gate di fiducia con l'altro contenuto che un repository può fornire. Puoi anche scrivere questa chiave come `additionalMarketplaces`; vedi [Alias di chiave Marketplace](#marketplace-key-aliases).

Imposta `"autoUpdate": true` insieme a `source` per fare in modo che Claude Code aggiorni quel marketplace e aggiorni i suoi plugin installati in background dopo l'avvio. Quando omesso, `claude-plugins-official` e la maggior parte degli altri marketplace ufficiali di Anthropic predefiniti su `true`, e i marketplace di terze parti predefiniti su `false`. Vedi [Configura gli auto-aggiornamenti](/docs/it/plugins/install#keep-plugins-updated).

Quando più di un file di impostazioni definisce una voce di marketplace con lo stesso nome, Claude Code utilizza la voce dal [file di precedenza più alta](/docs/it/settings#settings-precedence) nel complesso. Quella voce sostituisce la voce di precedenza inferiore e non eredita nessuno dei suoi campi, quindi una ridefinizione non può combinare le credenziali `source.headers` di un file con un URL che un altro file controlla. Prima di v2.1.228, Claude Code univa le voci con lo stesso nome campo per campo, quindi una voce in un file di precedenza superiore poteva ereditare campi che non impostava, incluso `headers` di un altro file.

<h4 id="marketplace-source-types">
  Tipi di fonte di marketplace
</h4>

L'oggetto `source` assume una di queste forme:

* **`github`**: un repository GitHub, con `repo`
* **`git`**: qualsiasi URL git, con `url`
* **`url`**: un URL diretto a un file `marketplace.json`, con `url` e `headers` facoltativo e `headersHelper` per l'accesso autenticato. `headersHelper` nomina un comando che stampa intestazioni i cui valori sono troppo effimeri per elencare in `headers`, e richiede Claude Code v2.1.238 o successivo
* **`file`**: un percorso locale a un file `marketplace.json`, con `path`
* **`directory`**: un percorso del filesystem locale, con `path`, solo per lo sviluppo
* **`settings`**: un marketplace inline dichiarato direttamente nel file di impostazioni senza un repository ospitato, con `name` e `plugins`

Il tipo di fonte `git` funziona con qualsiasi servizio di hosting git, incluso GitLab auto-ospitato e Bitbucket. Claude Code clona il repository con la stessa autenticazione che `git clone` userebbe su quella macchina: helper di credenziali configurati o chiavi SSH. Un token del provider come `GITHUB_TOKEN` ha effetto solo attraverso un helper di credenziali che lo legge. Vedi [Repository privati](/docs/it/plugins/host-marketplace#grant-access-to-a-private-marketplace) per i dettagli di configurazione.

Per le fonti `github` e `git`, Claude Code non scarica mai il contenuto di [Git LFS](https://git-lfs.com) quando clona il repository del marketplace per aggiungerlo o aggiornarlo. I file puntatore LFS rimangono come puntatori invece di scaricare il loro contenuto, e l'output di aggiunta o aggiornamento segnala quanti.

Il campo `skipLfs` all'interno dell'oggetto `source` è accettato e non ha effetto. Prima di v2.1.274, Claude Code scaricava il contenuto LFS a meno che non impostassi `"skipLfs": true`.

Per una fonte `url`, imposta `headersHelper` all'interno dell'oggetto `source` quando la credenziale in `headers` scade e un comando deve produrne una nuova. Richiede Claude Code v2.1.238 o successivo. Per cosa il comando deve stampare e dove Claude Code lo esegue, vedi [Scrivi il comando headersHelper](/docs/it/plugins/host-marketplace#write-the-headershelper-command), e per i casi in cui Claude Code non lo esegue, vedi [Quando Claude Code salta un comando headersHelper](/docs/it/plugins/host-marketplace#when-claude-code-skips-a-headershelper-command-or-drops-its-output). Una volta che imposti `headersHelper` su un URL di marketplace `https://`, Claude Code esegue il comando in due punti, riutilizzando l'output di un'esecuzione per fino a 60 secondi:

* Prima di ogni recupero del `marketplace.json` di quel marketplace, incluso un aggiornamento successivo. Claude Code invia le intestazioni stampate con quel recupero.
* Prima di ogni download di archivio plugin sull'origine dell'URL del marketplace, significando lo stesso schema, host e porta. Claude Code invia l'output con quel download, e nessun altro download ottiene le intestazioni.

Claude Code ignora qualsiasi `headersHelper` impostato nel `.claude/settings.json` o `.claude/settings.local.json` di una directory che aggiungi con [`--add-dir`](/docs/it/permissions#what-runs-before-you-trust-a-folder), su una fonte `url` e su una voce di plugin inline allo stesso modo, e invia solo gli `headers` fissi impostati in quel file. [Come gli utenti accettano un comando headersHelper](/docs/it/plugins/host-marketplace#how-users-accept-a-headershelper-command) copre gli altri file di impostazioni.

I plugin elencati in una fonte `settings` devono fare riferimento a fonti esterne come GitHub o npm, e il `name` deve corrispondere alla chiave del marketplace. Abiliti comunque ogni plugin separatamente in `enabledPlugins`. Questo esempio dichiara un plugin inline:

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

Una voce di plugin sotto `source: 'settings'` la cui propria `source` è un [`archive`](/docs/it/plugins/marketplace-reference#archive-plugin-source) può impostare `headers` per il download dell'archivio. Se il valore che metteresti in `headers` è effimero, come un token che il tuo registro conia su richiesta, imposta un comando `headersHelper` invece. Una voce può impostare entrambi. Entrambi i campi richiedono Claude Code v2.1.238 o successivo.

Claude Code invia gli `headers` della voce e tutto ciò che il comando stampa, con il download dell'archivio di quel plugin e con nessun altro download. Claude Code esegue il comando solo quando un utente [installa o aggiorna quel singolo plugin da solo](/docs/it/plugins/host-marketplace#how-users-accept-a-headershelper-command). Tre ulteriori regole dipendono da quale file contiene la voce:

* **`strict`**: a differenza di una voce nel `marketplace.json` di un marketplace, una voce nelle impostazioni non ha bisogno di `"strict": false`, perché un file di impostazioni non porta campi di manifesto da inline. Vedi [Modalità strict](/docs/it/plugins/marketplace-reference#strict-mode).
* **Fiducia della cartella**: per una voce nel `.claude/settings.json` o `.claude/settings.local.json` di un progetto, Claude Code esegue il comando solo dopo che l'utente ha anche [fidato quella cartella](/docs/it/permissions#what-runs-before-you-trust-a-folder).
* **Filtro intestazione**: Claude Code elimina i [nomi di intestazione di routing delle richieste e identità del client](/docs/it/plugins/host-marketplace#when-claude-code-skips-a-headershelper-command-or-drops-its-output) da una voce nel `.claude/settings.json` o `.claude/settings.local.json` di un progetto, perché un repository può fornire quei file. Claude Code applica lo stesso filtro a una voce di catalogo e a una voce in una directory `--add-dir`, e nessun filtro a una voce nelle tue impostazioni utente, un file `--settings` o impostazioni gestite.

<h4 id="marketplace-key-aliases">
  Alias di chiave Marketplace
</h4>

Su Claude Code v2.1.232 o successivo, puoi scrivere `extraKnownMarketplaces` come `additionalMarketplaces` e `strictKnownMarketplaces` come `allowedMarketplaces`. Claude Code tratta ogni alias come segue:

* Le versioni precedenti ignorano l'alias, quindi mantieni l'ortografia canonica in un file che le versioni precedenti leggono anche, come un file di impostazioni gestite per una flotta con versioni Claude Code miste.
* In qualsiasi file di impostazioni che accetta la chiave canonica, Claude Code legge l'alias esattamente come legge la chiave canonica.
* Claude Code può riscrivere `additionalMarketplaces` a `extraKnownMarketplaces` quando aggiorna il file.
* Se imposti entrambe le ortografie in un file, Claude Code utilizza il valore canonico e ignora l'alias.

<h3 id="pluginconfigs">
  `pluginConfigs`
</h3>

Archivia le risposte non sensibili che dai al dialogo di configurazione [`userConfig`](/docs/it/plugins/manifest-reference#user-configuration) di un plugin, codificato per ID plugin. Claude Code scrive questa chiave alle tue impostazioni utente quando riempi il dialogo, quindi non devi modificarla a mano. Claude Code archivia le opzioni sensibili nel Portachiavi di macOS invece, ricadendo a `~/.claude/.credentials.json` quando il Portachiavi rifiuta la scrittura; su piattaforme senza un portachiavi supportato, le archivia in `~/.claude/.credentials.json`.

* **Scope**: [`User or managed`](#scopes)
* **Type**: oggetto che mappa un ID plugin a un oggetto con un campo `options`, mappando ogni nome di opzione a una stringa, numero, Boolean o array di stringhe, e un campo `mcpServers` facoltativo che contiene valori di configurazione utente per server nella stessa forma
* **Default**: non impostato

Questo esempio archivia l'opzione `api_endpoint` per il plugin `deployer` da `acme-tools`:

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

I plugin incorporati archiviano le loro opzioni sotto la stessa chiave con un suffisso `@builtin`. Ad esempio, l'impostazione [**Istruzioni di progetto**](/docs/it/memory#choose-which-instruction-files-load) che controlla se Claude Code legge i file `AGENTS.md` è `pluginConfigs["agents-md@builtin"].options.instructionFiles`.

Claude Code ignora le voci di progetto e locale perché sostituisce questi valori nelle configurazioni di hook, MCP e LSP del plugin, e un repository clonato non deve essere in grado di fornirli. Prima di v2.1.207, anche le impostazioni di progetto e locale venivano lette.

<h2 id="mcp">
  MCP
</h2>

Controllare a quali server MCP Claude Code si connette e quali un'organizzazione consente. Vedere [Connettere a strumenti esterni con MCP](/docs/it/mcp) e [Configurazione MCP gestita](/docs/it/managed-mcp).

<h3 id="allowallclaudeaimcps">
  `allowAllClaudeAiMcps`
</h3>

Caricare i [connettori claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai) che Claude Code recupera da solo insieme a un `managed-mcp.json` distribuito. Senza questa chiave, `managed-mcp.json` assume il controllo esclusivo dei server MCP e sopprime quei connettori.

* **Scope**: [`Managed`](#scopes). Gli utenti non possono riabilitare i connettori che il controllo esclusivo ha soppresso.
* **Type**: Boolean
  * `true`: Claude Code carica i connettori claude.ai insieme a un `managed-mcp.json` distribuito
  * `false`: un `managed-mcp.json` distribuito assume il controllo esclusivo dei server MCP e sopprime i connettori claude.ai [che Claude Code recupera da solo](/docs/it/mcp#how-connectors-reach-claude-code)
* **Default**: `false`, quindi un `managed-mcp.json` distribuito sopprime i connettori claude.ai che Claude Code recupera da solo

```json managed-settings.json theme={null}
{
  "allowAllClaudeAiMcps": true
}
```

[`allowedMcpServers`](#allowedmcpservers) e [`deniedMcpServers`](#deniedmcpservers) si applicano ancora ai connettori che questa chiave carica. I connettori consegnati a una [sessione cloud](/docs/it/claude-code-on-the-web) il cui host contiene un `managed-mcp.json`, come un runner auto-ospitato, rimangono soppressi. Vedere [Consentire i connettori claude.ai insieme al set gestito](/docs/it/managed-mcp#allow-claude-ai-connectors-alongside-the-managed-set).

<h3 id="allowedmcpservers">
  `allowedMcpServers`
</h3>

Creare un elenco di autorizzazione dei server MCP che le persone possono aggiungere. Claude Code blocca qualsiasi server che non corrisponde a una voce ovunque sia definito, inclusi i server plugin, i server passati con `--mcp-config` e i server da claude.ai.

I server incorporati come Claude in Chrome, il server `ide` a cui Claude Code si connette in un [VS Code](/docs/it/vs-code#the-built-in-ide-mcp-server) o [JetBrains](/docs/it/jetbrains#the-built-in-ide-mcp-server) IDE in esecuzione e i server che la CLI stessa configura sono esenti dall'elenco di autorizzazione e l'elenco di negazione si applica ancora a loro. I server `type: "sdk"` in-process sono esenti da entrambi gli elenchi; [l'app che ha avviato la sessione](/docs/it/mcp#how-connectors-reach-claude-code) li registra.

I server che l'organizzazione fornisce sono anche esenti dall'elenco di autorizzazione e l'elenco di negazione si applica ancora a loro. L'esenzione copre ogni voce [`managedMcpServers`](#managedmcpservers) e qualsiasi voce [`managed-mcp.json`](/docs/it/managed-mcp#exclusive-control-with-managed-mcp-json) i cui valori non utilizzano l'espansione `${VAR}`. Vedere [Come viene valutato un server](/docs/it/managed-mcp#how-a-server-is-evaluated) per l'ordine di controllo completo. Prima della v2.1.259, i server da `managed-mcp.json` dovevano corrispondere anche loro.

* **Scope**: [`Any file`](#scopes). Le voci di ogni file si uniscono in un elenco di autorizzazione a meno che [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly) non sia impostato. Distribuirlo nelle impostazioni gestite per applicarlo.
* **Type**: array di oggetti, ognuno con esattamente una chiave: `serverName`, una stringa limitata a lettere, numeri, trattini e sottolineature; `serverCommand`, un array del comando e dei suoi argomenti abbinati esattamente; o `serverUrl`, un modello di URL con caratteri jolly `*`
* **Default**: non impostato, quindi ogni server è consentito; un array vuoto blocca ogni server che gli utenti aggiungono

Questo esempio consente solo il server stdio che il comando `npx` elencato avvia:

```json settings.json theme={null}
{
  "allowedMcpServers": [
    { "serverCommand": ["npx", "-y", "@modelcontextprotocol/server-filesystem"] }
  ]
}
```

Una voce [`deniedMcpServers`](#deniedmcpservers) ha la precedenza, quindi un server in entrambi gli elenchi è bloccato. Una volta che l'elenco contiene qualsiasi voce `serverCommand`, un server stdio deve corrispondere a una voce `serverCommand`, e una volta che contiene qualsiasi voce `serverUrl`, un server remoto deve corrispondere a una voce `serverUrl`: una corrispondenza `serverName` non ammette più quel tipo di server. Vedere [Controllo basato su criteri con elenchi di autorizzazione e negazione](/docs/it/managed-mcp#policy-based-control-with-allowlists-and-denylists).

<h3 id="allowmanagedmcpserversonly">
  `allowManagedMcpServersOnly`
</h3>

Rendere l'elenco di autorizzazione gestito l'unico che si applica. Claude Code legge quindi [`allowedMcpServers`](#allowedmcpservers) solo dalle impostazioni gestite e ignora gli elenchi di autorizzazione nelle impostazioni utente, progetto e locale; [`deniedMcpServers`](#deniedmcpservers) si unisce ancora da ogni ambito di impostazioni, quindi gli utenti possono ancora bloccare i server per se stessi. Gli amministratori lo impostano in modo che le impostazioni proprie di un utente non possano ampliare ciò che l'elenco di autorizzazione gestito consente.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code legge `allowedMcpServers` solo dalle impostazioni gestite e ignora gli elenchi di autorizzazione nelle impostazioni utente, progetto e locale
  * `false`: gli elenchi di autorizzazione da ogni ambito di impostazioni si uniscono
* **Default**: `false`, quindi gli elenchi di autorizzazione da ogni ambito di impostazioni si uniscono

Questo esempio blocca l'elenco di autorizzazione alle impostazioni gestite e consente solo il server denominato `github`:

```json managed-settings.json theme={null}
{
  "allowManagedMcpServersOnly": true,
  "allowedMcpServers": [
    { "serverName": "github" }
  ]
}
```

Gli utenti possono comunque aggiungere server MCP propri; solo i server che corrispondono all'elenco di autorizzazione gestito vengono caricati. Vedere [Limitare l'elenco di autorizzazione alle sole impostazioni gestite](/docs/it/managed-mcp#restrict-the-allowlist-to-managed-settings-only).

<h3 id="deniedmcpservers">
  `deniedMcpServers`
</h3>

Bloccare server MCP specifici. Claude Code rifiuta di caricare un server corrispondente ovunque sia definito, inclusi i server plugin, i server passati con `--mcp-config`, i server da `managed-mcp.json`, i server da [`managedMcpServers`](#managedmcpservers) e i connettori claude.ai [che recupera da solo](/docs/it/mcp#how-connectors-reach-claude-code). I server `type: "sdk"` in-process sono esenti; l'app che ha avviato la sessione li registra.

* **Scope**: [`Any file`](#scopes). Le voci di ogni file si uniscono in un elenco di negazione e [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly) non cambia questo. Distribuirlo nelle impostazioni gestite per applicarlo.
* **Type**: array di oggetti, ognuno con esattamente una chiave: `serverName`, una stringa, quindi il nome visualizzato di un connettore claude.ai come `"claude.ai Slack"` funziona; `serverCommand`, un array del comando e dei suoi argomenti abbinati esattamente; o `serverUrl`, un modello di URL con caratteri jolly `*`
* **Default**: non impostato, quindi nessun server è bloccato; un array vuoto non blocca nulla

```json settings.json theme={null}
{
  "deniedMcpServers": [
    { "serverName": "filesystem" }
  ]
}
```

L'elenco di negazione ha la precedenza su [`allowedMcpServers`](#allowedmcpservers), quindi un server in entrambi gli elenchi è bloccato. Vedere [Controllo basato su criteri con elenchi di autorizzazione e negazione](/docs/it/managed-mcp#policy-based-control-with-allowlists-and-denylists).

<h3 id="disableclaudeaiconnectors">
  `disableClaudeAiConnectors`
</h3>

Disattivare i [connettori MCP claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai) [che Claude Code recupera da solo](/docs/it/mcp#how-connectors-reach-claude-code), in modo che non li recuperi né li connetta. Un `true` in qualsiasi file di impostazioni si applica: un `.claude/settings.json` di progetto archiviato può escludere un repository da quei connettori, ma un `false` a livello di progetto non può sovrascrivere un `true` a livello di utente o gestito.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code non recupera né connette quei connettori
  * `false`: lo stesso di non impostato; Claude Code recupera i tuoi connettori a meno che un altro file di impostazioni o `ENABLE_CLAUDEAI_MCP_SERVERS` non li disattivi
* **Default**: `false`, quindi Claude Code recupera i tuoi connettori
* **Per-session overrides**: [`ENABLE_CLAUDEAI_MCP_SERVERS`](/docs/it/env-vars) impostato su `false` disattiva i connettori per una sessione; qualunque dei due li disattivi, l'altro non può riattivarli

```json settings.json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

I server che passi esplicitamente con `--mcp-config` non sono interessati. Per bloccare i singoli connettori invece di tutti, usa [`deniedMcpServers`](#deniedmcpservers). Vedere [Disattivare i connettori claude.ai](/docs/it/mcp#disable-claude-ai-connectors).

<h3 id="disabledmcpjsonservers">
  `disabledMcpjsonServers`
</h3>

Rifiutare server specifici definiti nel file `.mcp.json` di un progetto in modo che Claude Code non li connetta mai o non ti chieda di approvarli. Un rifiuto in qualsiasi file di impostazioni si applica, incluso un `.claude/settings.json` di progetto archiviato nel repository.

* **Scope**: [`Any file`](#scopes)
* **Type**: array di stringhe, i nomi dei server come appaiono in `.mcp.json`
* **Default**: non impostato

```json settings.json theme={null}
{
  "disabledMcpjsonServers": ["filesystem"]
}
```

Claude Code scrive questa chiave in `.claude/settings.local.json` quando rifiuti un server nella finestra di dialogo di approvazione. `claude mcp get <name>` mostra un server rifiutato come `✘ Rejected (see disabledMcpjsonServers in settings)`. Il rifiuto ha la precedenza su [`enabledMcpjsonServers`](#enabledmcpjsonservers) e [`enableAllProjectMcpServers`](#enableallprojectmcpservers).

<h3 id="enableallprojectmcpservers">
  `enableAllProjectMcpServers`
</h3>

Approvare ogni server MCP definito nei file `.mcp.json` del progetto senza un prompt. Claude Code scrive questa chiave in `.claude/settings.local.json` quando scegli di approvare tutti i server nella finestra di dialogo di approvazione.

* **Scope**: [`Any file`](#scopes). In una cartella la cui finestra di dialogo di fiducia non hai accettato, Claude Code la onora dalle impostazioni utente, impostazioni gestite e `--settings` e la ignora nel file di progetto condiviso, sia nella sessione che per `claude mcp list` e `claude mcp get`; [Approvazioni dei server di progetto e fiducia dell'area di lavoro](/docs/it/mcp#project-server-approvals-and-workspace-trust) dice quando un `.claude/settings.local.json` non tracciato conta anche.
* **Type**: Boolean
  * `true`: Claude Code approva ogni server MCP definito nei file `.mcp.json` del progetto senza un prompt
  * `false`: Claude Code ti chiede di approvare ogni server. In una cartella attendibile, un `false` in un file con precedenza più alta sovrascrive un `true` in uno inferiore; in una cartella che non hai attendibile, un `true` in qualsiasi file onorato è sufficiente
* **Default**: non impostato, quindi Claude Code ti chiede di approvare ogni server

```json settings.json theme={null}
{
  "enableAllProjectMcpServers": true
}
```

Una voce [`disabledMcpjsonServers`](#disabledmcpjsonservers) rifiuta comunque un server.

<h3 id="enabledmcpjsonservers">
  `enabledMcpjsonServers`
</h3>

Approvare server specifici definiti nei file `.mcp.json` del progetto in modo che Claude Code li connetta senza chiedere. Claude Code scrive questa chiave in `.claude/settings.local.json` quando approvi un server nella finestra di dialogo di approvazione.

* **Scope**: [`Any file`](#scopes). In una cartella la cui finestra di dialogo di fiducia non hai accettato, Claude Code la onora dalle impostazioni utente, impostazioni gestite e `--settings` e la ignora nel file di progetto condiviso, sia nella sessione che per `claude mcp list` e `claude mcp get`; [Approvazioni dei server di progetto e fiducia dell'area di lavoro](/docs/it/mcp#project-server-approvals-and-workspace-trust) dice quando un `.claude/settings.local.json` non tracciato conta anche.
* **Type**: array di stringhe, i nomi dei server come appaiono in `.mcp.json`
* **Default**: non impostato

Questo esempio approva i server `memory` e `github` dal `.mcp.json` del progetto:

```json settings.json theme={null}
{
  "enabledMcpjsonServers": ["memory", "github"]
}
```

Una voce [`disabledMcpjsonServers`](#disabledmcpjsonservers) rifiuta comunque un server.

<h3 id="managedmcpservers">
  `managedMcpServers`
</h3>

Fornire server MCP remoti a ogni utente dalle impostazioni gestite. Gli utenti mantengono i server che aggiungono da soli e non possono modificare o rimuovere quelli che fornisci. Richiede Claude Code v2.1.259 o successivo.

* **Scope**: [`Managed`](#scopes). Claude Code elimina la chiave con un avviso nelle impostazioni utente, progetto e locale e non la legge nella scheda Code dell'app Claude Desktop su una distribuzione di terze parti o nelle sessioni Cowork dell'app, dove Claude Desktop fornisce e blocca i server MCP di quelle sessioni stesso.
* **Type**: oggetto con chiave per nome del server. Ogni voce ha la forma `.mcp.json` per un server `http` o `sse`: un `url` `https://` obbligatorio e facoltativamente `headers`, `oauth` e le altre opzioni HTTP e SSE. Claude Code elimina le voci che non superano la convalida e [Cosa può contenere una voce](/docs/it/managed-mcp#what-an-entry-can-contain) elenca le condizioni
* **Default**: non impostato, quindi le impostazioni gestite non forniscono server

Questo esempio fornisce un server HTTP denominato `search`:

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

Per la precedenza, come i server forniti si combinano con `managed-mcp.json` e gli elenchi di autorizzazione e negazione, e cosa vedono gli utenti, vedere [Fornire server tramite impostazioni gestite](/docs/it/managed-mcp#provide-servers-through-managed-settings).

<h2 id="agents-sessions-and-worktrees">
  Agenti, sessioni e worktrees
</h2>

Impostare l'agente predefinito, controllare i compagni di squadra e la messaggistica tra sessioni, e configurare i worktrees. Vedere [Subagenti](/docs/it/sub-agents) e [Worktrees](/docs/it/worktrees).

<h3 id="agent">
  `agent`
</h3>

Eseguire il thread principale come un [subagente](/docs/it/sub-agents#invoke-subagents-explicitly) denominato, in modo che Claude Code applichi il prompt di sistema, le restrizioni degli strumenti e il modello di quel subagente alla sessione. La stessa chiave imposta l'agente predefinito per le sessioni che si inviano da `claude agents`.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, il nome di un agente integrato o personalizzato
* **Default**: non impostato, quindi il thread principale viene eseguito come agente predefinito di Claude Code
* **Per-session overrides**: `--agent` ha la precedenza su questa chiave per una sessione

```json settings.json theme={null}
{
  "agent": "code-reviewer"
}
```

Il `settings.json` di un plugin può anche fornire questa chiave; vedere [Ship default settings with your plugin](/docs/it/plugins/components#default-settings).

<h3 id="crosssessioninbound">
  `crossSessionInbound`
</h3>

Scegliere cosa fa questa sessione con i [messaggi provenienti dalle altre sessioni di Claude Code](/docs/it/cross-session-messaging#control-inbound-messages). Quando nessun valore si applica, Claude Code decide per messaggio dalle classi della modalità di autorizzazione delle due sessioni. Richiede Claude Code v2.1.224 o successivo.

* **Scope**: [`Any file`](#scopes). Un valore di progetto o locale si applica solo quando è più rigoroso del valore delle impostazioni gestite, del flag `--settings` o delle impostazioni utente.
* **Type**: string, uno di:
  * `"accept"`: Claude Code consegna il messaggio a Claude
  * `"hold"`: Claude Code mostra un avviso per il messaggio senza consegnarlo
  * `"refuse"`: Claude Code scarta il messaggio
* **Default**: non impostato, quindi Claude Code decide per messaggio

```json settings.json theme={null}
{
  "crossSessionInbound": "hold"
}
```

Claude Code legge prima le impostazioni gestite, poi il flag `--settings`, poi le impostazioni utente, e applica il primo valore trovato. `refuse` è più rigoroso di `hold`, e `hold` è più rigoroso di `accept`. Quando nessuna delle fonti attendibili imposta un valore, un `hold` o `refuse` di progetto o locale si applica comunque, sostituendo il valore predefinito per messaggio. Nelle sessioni con messaggistica tra sessioni, questa chiave appare in `/config` come **Messages from your other sessions**, che la scrive nelle impostazioni utente; la riga richiede Claude Code v2.1.232 o successivo, e Claude Code la nasconde mentre il flag `--settings` o le impostazioni gestite impostano la chiave.

Claude Code [avverte](/docs/it/errors#crosssessioninbound-must-be-one-of-accept-hold-refuse) quando si imposta un valore che non riconosce. Mentre quel valore è presente in un file utente, progetto, locale o `--settings`, Claude Code trattiene i messaggi in entrata, anche quando una fonte che ha la precedenza imposta `accept`. Un `refuse` che un'altra fonte imposta si applica comunque. Correggere o rimuovere il valore per cancellare la sospensione.

Quando il valore non riconosciuto è nelle [impostazioni gestite](/docs/it/managed-settings), Claude Code lo tratta invece come `refuse` finché un amministratore non lo corregge. Prima della v2.1.248, Claude Code ignorava un valore non riconosciuto senza avviso.

<h3 id="disableagentview">
  `disableAgentView`
</h3>

Disattivare gli [agenti di background e la visualizzazione agente](/docs/it/agent-view): `claude agents`, `--bg`, `/background` e il supervisore su richiesta. Impostarlo nelle [impostazioni gestite](/docs/it/managed-settings) per applicarlo a un'organizzazione.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code disattiva `claude agents`, `--bg`, `/background` e il supervisore su richiesta
  * `false`: la visualizzazione agente è disponibile
* **Default**: non impostato, quindi la visualizzazione agente è disponibile
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_AGENT_VIEW`](/docs/it/env-vars) disattiva la visualizzazione agente per una sessione; qualunque dei due la disattivi, l'altro non può riattivarla

```json settings.json theme={null}
{
  "disableAgentView": true
}
```

<h3 id="isolatepeermachines">
  `isolatePeerMachines`
</h3>

Richiedere l'approvazione esplicita prima che `SendMessage` di Claude raggiunga una delle sessioni oltre questa macchina; vedere [Require approval for cross-machine messages](/docs/it/cross-session-messaging#require-approval-for-cross-machine-messages). Il prompt di approvazione appare anche nella [modalità `bypassPermissions`](/docs/it/permission-modes#skip-all-checks-with-bypasspermissions-mode).

* **Scope**: [`Any file`](#scopes). Un `true` da qualsiasi scope si applica, quindi un file di progetto archiviato può attivare il requisito ma non disattivarlo.
* **Type**: Boolean
  * `true`: Claude Code chiede l'approvazione prima che `SendMessage` di Claude raggiunga una delle sessioni oltre questa macchina
  * `false`: i messaggi tra macchine non richiedono un prompt
* **Default**: non impostato, quindi i messaggi tra macchine non richiedono un prompt

```json settings.json theme={null}
{
  "isolatePeerMachines": true
}
```

L'approvazione `SendMessage` tra macchine richiede Claude Code v2.1.224 o successivo.

<h3 id="processwrapper">
  `processWrapper`
</h3>

Su macOS e Linux, posizionare un comando di avvio aziendale davanti ai [processi di background che Claude Code avvia](/docs/it/corporate-launcher#what-the-launcher-covers). Claude Code esegue l'avvio con la propria riga di comando aggiunta, quindi l'avvio deve eseguire in Claude Code; vedere [Run Claude Code behind a corporate launcher](/docs/it/corporate-launcher) per il contratto dell'avvio. Richiede Claude Code v2.1.210 o successivo.

* **Scope**: [`User or managed`](#scopes)
* **Type**: string, il comando dell'avvio come prefisso argv, come un percorso assoluto con argomenti facoltativi
* **Default**: non impostato, quindi i processi di background si avviano senza wrapper
* **Per-session overrides**: [`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/it/env-vars) ha la precedenza su questa chiave per una sessione

```json settings.json theme={null}
{
  "processWrapper": "/opt/corp/launcher --profile claude"
}
```

Claude Code ignora l'avvio su Windows e avvia ogni processo senza wrapper. Richiede Claude Code v2.1.210 o successivo.

<h3 id="teammatemode">
  `teammateMode`
</h3>

Scegliere dove Claude Code mostra i compagni di squadra del [team agente](/docs/it/agent-teams): all'interno del riquadro terminale principale, o in riquadri divisi quando il terminale li supporta. Vedere [Choose a display mode](/docs/it/agent-teams#choose-a-display-mode).

* **Scope**: [`Any file`](#scopes). Claude Code legge anche un valore lasciato in `~/.claude.json` da versioni precedenti.
* **Type**: string, uno di:
  * `"in-process"`: i compagni di squadra vengono eseguiti all'interno del riquadro terminale principale
  * `"auto"`: riquadri divisi quando si esegue all'interno di tmux, o all'interno di iTerm2 con `it2` sul `PATH` o tmux installato; in-process altrimenti
  * `"tmux"`: riquadri divisi utilizzando tmux o iTerm2, rilevati dal terminale
  * `"iterm2"`: riquadri divisi nativi di iTerm2 tramite la CLI `it2`
* **Default**: `"in-process"`
* **Per-session overrides**: `--teammate-mode` ha la precedenza su questa chiave per una sessione

```json settings.json theme={null}
{
  "teammateMode": "auto"
}
```

<span id="worktree-settings" />

<h3 id="worktree">
  `worktree`
</h3>

Configurare come Claude Code crea e gestisce i [git worktrees](/docs/it/worktrees) per `--worktree`, lo strumento `EnterWorktree` e i subagenti isolati e le sessioni di background.

* **Scope**: [`Any file`](#scopes)
* **Type**: object con `baseRef`, `symlinkDirectories`, `sparsePaths` e `bgIsolation`
* **Default**: non impostato

Questo esempio crea rami di nuovi worktrees dal `HEAD` corrente e crea symlink di `node_modules` in ognuno:

```json settings.json theme={null}
{
  "worktree": {
    "baseRef": "head",
    "symlinkDirectories": ["node_modules"]
  }
}
```

Per copiare file ignorati da git come `.env` nei nuovi worktrees, aggiungere un [file `.worktreeinclude`](/docs/it/worktrees#copy-gitignored-files-into-worktrees) alla radice del progetto invece di un'impostazione.

<h3 id="worktree-baseref">
  `worktree.baseRef`
</h3>

Scegliere da quale ref i nuovi worktrees si diramano. `"fresh"` si dirama da `origin/<default-branch>` per un albero pulito che corrisponde al remoto; `"head"` si dirama dal `HEAD` locale corrente, quindi i commit non inviati e lo stato del ramo di funzionalità sono presenti nel worktree.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, uno di:
  * `"fresh"`: i nuovi worktrees si diramano da `origin/<default-branch>`
  * `"head"`: i nuovi worktrees si diramano dal `HEAD` locale corrente, inclusi i commit non inviati
* **Default**: `"fresh"`

```json settings.json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

All'interno di un worktree collegato, `"head"` si risolve nel `HEAD` di quel worktree, non nel checkout principale.

<h3 id="worktree-symlinkdirectories">
  `worktree.symlinkDirectories`
</h3>

Creare symlink di directory dal repository principale in ogni worktree in modo da non duplicare directory di grandi dimensioni su disco.

* **Scope**: [`Any file`](#scopes)
* **Type**: array di strings, percorsi di directory relativi alla radice del repository
* **Default**: non impostato, quindi Claude Code non crea symlink di directory

Questo esempio crea symlink di `node_modules` e `.cache` dal repository principale in ogni nuovo worktree:

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

Estrarre solo le directory elencate in ogni worktree tramite git sparse-checkout. Claude Code scrive solo quelle directory più i file a livello di radice su disco, il che è più veloce nei monorepo di grandi dimensioni; vedere [Check out only the directories you need](/docs/it/large-codebases#check-out-only-the-directories-you-need).

* **Scope**: [`Any file`](#scopes)
* **Type**: array di strings, percorsi di directory relativi alla radice del repository
* **Default**: non impostato, quindi ogni worktree estrae l'intero albero

Questo esempio estrae solo `packages/my-app` e `shared/utils`, più i file a livello di radice, in ogni worktree:

```json settings.json theme={null}
{
  "worktree": {
    "sparsePaths": ["packages/my-app", "shared/utils"]
  }
}
```

Mentre esiste un worktree sparse, git abilita `extensions.worktreeConfig` nel `.git/config` condiviso del repository.

<h3 id="worktree-bgisolation">
  `worktree.bgIsolation`
</h3>

Scegliere come le [sessioni di background](/docs/it/agent-view#how-file-edits-are-isolated) isolano le loro modifiche ai file. Con `"worktree"`, Claude Code blocca `Edit` e `Write` nel checkout principale finché la sessione non chiama `EnterWorktree`; con `"none"`, i lavori di background modificano direttamente la copia di lavoro. Impostare `"none"` per un repository dove i git worktrees non sono pratici.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, uno di:
  * `"worktree"`: Claude Code blocca `Edit` e `Write` nel checkout principale finché la sessione non chiama `EnterWorktree`
  * `"none"`: i lavori di background modificano direttamente la copia di lavoro
* **Default**: `"worktree"`

```json settings.json theme={null}
{
  "worktree": {
    "bgIsolation": "none"
  }
}
```

Al di fuori di un repository git, un [hook `WorktreeCreate`](/docs/it/worktrees#non-git-version-control) che fallisce rilascia il blocco in modo che la sessione possa modificare la directory di lavoro in posizione; quel rilascio richiede Claude Code v2.1.203 o successivo.

<h2 id="remote-desktop-and-notifications">
  Controllo remoto, desktop e notifiche
</h2>

Configura il Controllo remoto, gli ambienti cloud, l'app desktop e le notifiche che Claude Code invia quando ha bisogno di te. Vedi [Controllo remoto](/docs/it/remote-control).

<h3 id="agentpushnotifenabled">
  `agentPushNotifEnabled`
</h3>

Consenti a Claude di inviare una notifica push al tuo telefono quando decide che ne vale la pena, ad esempio quando un'attività lunga si conclude. Claude Code sincronizza questa scelta al tuo account e le notifiche push arrivano mentre il [Controllo remoto](/docs/it/remote-control) è connesso. Appare in `/config` come **Push quando Claude decide**.

* **Scope**: [`Any file`](#scopes). Claude Code legge anche un valore lasciato in `~/.claude.json` da versioni precedenti.
* **Type**: Boolean
  * `true`: Claude può inviare una notifica push al tuo telefono quando decide che ne vale la pena
  * `false`: Claude non invia quelle notifiche
* **Default**: `false`

```json settings.json theme={null}
{
  "agentPushNotifEnabled": true
}
```

Vedi [Notifiche push mobile](/docs/it/remote-control#mobile-push-notifications).

<h3 id="awaysummaryenabled">
  `awaySummaryEnabled`
</h3>

Mostra un riepilogo di una riga della sessione quando torni al terminale dopo alcuni minuti di assenza. Impostalo su `false` o disattiva **Session recap** in `/config` per interrompere il riepilogo.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: vedi un riepilogo di una riga della sessione quando torni dopo alcuni minuti di assenza
  * `false`: Claude Code non mostra alcun riepilogo
* **Default**: non impostato, quindi il riepilogo è attivo
* **Per-session overrides**: [`CLAUDE_CODE_ENABLE_AWAY_SUMMARY`](/docs/it/env-vars) ha la precedenza su questa chiave per una sessione, in entrambe le direzioni

```json settings.json theme={null}
{
  "awaySummaryEnabled": false
}
```

Claude Code non mostra mai il riepilogo in modalità non interattiva.

<h3 id="disableartifact">
  `disableArtifact`
</h3>

<Warning>
  Deprecato e sostituito da [`enableArtifact`](#enableartifact). Claude Code onora ancora `disableArtifact: true` come equivalente a `enableArtifact: false` e ignora `disableArtifact: false`.
</Warning>

Usa [`enableArtifact`](#enableartifact) invece per disattivare lo strumento [Artifact](/docs/it/artifacts), che pubblica l'output della sessione come pagina web privata su claude.ai. Quando disattivi la riga **Artifacts** in `/config`, Claude Code scrive `enableArtifact` nelle tue impostazioni utente e cancella questa chiave.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code disattiva lo strumento Artifact per ogni sessione a cui il file si applica e nessun altro file lo riattiva. Prima della v2.1.242, un file con precedenza più alta potrebbe sovrascrivere un `true` di un file inferiore piuttosto che la chiave agire come un blocco
  * `false`: ignorato; per lasciare lo strumento attivo, rimuovi la chiave
* **Default**: non impostato, quindi lo strumento segue la [disponibilità](/docs/it/artifacts#availability) del tuo account
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_ARTIFACT`](/docs/it/env-vars) impostato su `1` disattiva lo strumento per una sessione

```json settings.json theme={null}
{
  "disableArtifact": true
}
```

[Disabilita artefatti](/docs/it/artifacts#disable-artifacts) elenca ogni modo per disattivare lo strumento.

<h3 id="disabledeeplinkregistration">
  `disableDeepLinkRegistration`
</h3>

Impedisci a Claude Code di registrare il gestore del protocollo `claude-cli://` con il sistema operativo, che altrimenti fa dopo che invii il primo prompt di una sessione interattiva. I [Deep link](/docs/it/deep-links) consentono agli strumenti esterni di aprire una sessione Claude Code con un prompt precompilato. Impostalo in ambienti in cui la registrazione del gestore del protocollo è limitata o gestita separatamente.

* **Scope**: [`Any file`](#scopes)
* **Type**: la stringa `"disable"`
* **Default**: non impostato, quindi Claude Code registra il gestore

```json settings.json theme={null}
{
  "disableDeepLinkRegistration": "disable"
}
```

<h3 id="disabledesktoplocalsessions">
  `disableDesktopLocalSessions`
</h3>

Disattiva le sessioni Code che vengono eseguite sul dispositivo nell'[app desktop](/docs/it/desktop#local-sessions-on-managed-devices), per distribuzioni in cui gli sviluppatori dovrebbero lavorare su macchine remote tramite SSH. Nella scheda Code, l'ambiente **Local** rimane nel menu a discesa dell'ambiente ma è disattivato e non può essere selezionato, con un tooltip che dice che la tua organizzazione lo ha disattivato; su Windows la voce WSL è disattivata allo stesso modo, anche se se le sessioni WSL vengono eseguite su un dispositivo gestito è [governato separatamente](/docs/it/admin-setup#wsl-sessions-in-claude-code-desktop). Le nuove sessioni predefinite alla prima [connessione SSH](/docs/it/desktop#ssh-sessions) se ne è configurata una, e l'app rifiuta di avviare o riprendere una sessione sul dispositivo, inclusa una connessione SSH allo stesso computer. Le sessioni SSH ad altri host e le sessioni cloud non sono interessate. L'app desktop legge questa chiave; il CLI del terminale la ignora. Richiede Claude Desktop v1.37937.0 o successivo.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean; solo il Boolean JSON `true` ha effetto
  * `true`: l'app desktop non offre sessioni Code on-device; le sessioni locali esistenti rimangono elencate ma non possono continuare
  * `false`: le sessioni locali rimangono disponibili
* **Default**: non impostato, quindi le sessioni locali sono disponibili

```json managed-settings.json theme={null}
{
  "disableDesktopLocalSessions": true
}
```

L'app desktop ignora qualsiasi altro valore e un valore che non è un Boolean, come la stringa `"true"` o `1`, registra anche un avviso. Abbinalo a [`sshConfigs`](#sshconfigs) in modo che gli utenti si trovino su una connessione funzionante e con [`sshHostAllowlist`](#sshhostallowlist) per limitare quali host possono raggiungere. Vedi [Sessioni locali su dispositivi gestiti](/docs/it/desktop#local-sessions-on-managed-devices).

Claude Desktop fornisce sessioni Code con policy derivata dalla tua configurazione desktop, ad esempio l'allowlist di uscita, la sandbox del filesystem e le restrizioni MCP nelle distribuzioni di terze parti. Claude Code ignora quelle impostazioni padre ogni volta che è presente un'[origine amministratore](/docs/it/managed-settings#how-claude-code-combines-managed-sources): impostazioni gestite dal server, una policy MDM o a livello di sistema operativo, o un file di impostazioni gestite. Distribuire questa chiave attraverso uno di questi su un dispositivo che non ne aveva nessuno prima, come nelle distribuzioni di terze parti, quindi interrompe l'applicazione delle policy derivate dal desktop. [Consenti a un host di incorporamento di aggiungere policy](/docs/it/managed-settings#let-an-embedding-host-add-policy) copre quando le impostazioni padre possono ancora unirsi; questo vale per qualsiasi chiave che distribuisci in quel modo, non solo questa.

<h3 id="disableremotecontrol">
  `disableRemoteControl`
</h3>

Disattiva il [Controllo remoto](/docs/it/remote-control): Claude Code rifiuta quindi `claude remote-control`, il flag `--remote-control`, l'avvio automatico e l'interruttore in-sessione e segnala che la policy della tua organizzazione lo ha disabilitato. Posizionalo nelle [impostazioni gestite](/docs/it/managed-settings) per l'applicazione MDM per dispositivo.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code rifiuta `claude remote-control`, il flag `--remote-control`, l'avvio automatico e l'interruttore in-sessione
  * `false`: il Controllo remoto rimane disponibile
* **Default**: `false`

```json settings.json theme={null}
{
  "disableRemoteControl": true
}
```

<h3 id="enableartifact">
  `enableArtifact`
</h3>

Disattiva lo strumento [Artifact](/docs/it/artifacts), che pubblica l'output della sessione come pagina web privata su claude.ai. Quando disattivi la riga **Artifacts** in `/config`, Claude Code scrive questa chiave nelle tue impostazioni utente, quindi di solito non la modifichi manualmente. Richiede Claude Code v2.1.196 o successivo.

* **Scope**: [`Any file`](#scopes). Ogni file può disattivare lo strumento e nessuno può riattivarlo.
* **Type**: Boolean
  * `false`: Claude Code disattiva lo strumento Artifact per ogni sessione a cui il file si applica
  * `true`: lo stesso di lasciare la chiave non impostata, perché non sovrascrive mai un `false` da un altro file, da [`CLAUDE_CODE_DISABLE_ARTIFACT`](/docs/it/env-vars) o dall'[impostazione amministratore](/docs/it/artifacts#manage-artifacts-for-your-organization) della tua organizzazione
* **Default**: non impostato, quindi lo strumento segue la [disponibilità](/docs/it/artifacts#availability) del tuo account

```json settings.json theme={null}
{
  "enableArtifact": false
}
```

Mentre un'origine diversa dalle tue impostazioni utente mantiene lo strumento disattivato, Claude Code nasconde la riga **Artifacts** in `/config`, perché attivarlo lì non cambierebbe nulla. [Disabilita artefatti](/docs/it/artifacts#disable-artifacts) elenca ogni modo per disattivare lo strumento. Prima della v2.1.242, Claude Code ignorava questa chiave nelle impostazioni di progetto e locali e un file più alto nello [stack di precedenza](/docs/it/settings#settings-precedence) potrebbe riattivare lo strumento su un file inferiore spento.

<h3 id="inputneedednotifenabled">
  `inputNeededNotifEnabled`
</h3>

Ricevi una notifica push sul tuo telefono quando un prompt di autorizzazione o una domanda è in attesa del tuo input. Claude Code invia questi solo mentre il [Controllo remoto](/docs/it/remote-control) è connesso. Appare in `/config` come **Push quando azioni richieste**.

* **Scope**: [`Any file`](#scopes). Claude Code legge anche un valore lasciato in `~/.claude.json` da versioni precedenti.
* **Type**: Boolean
  * `true`: ricevi una notifica push sul tuo telefono quando un prompt di autorizzazione o una domanda è in attesa, mentre il Controllo remoto è connesso
  * `false`: Claude Code non invia tali notifiche
* **Default**: `false`

```json settings.json theme={null}
{
  "inputNeededNotifEnabled": true
}
```

Vedi [Notifiche push mobile](/docs/it/remote-control#mobile-push-notifications).

<h3 id="preferrednotifchannel">
  `preferredNotifChannel`
</h3>

Scegli come Claude Code ti notifica quando un'attività si completa o un prompt di autorizzazione è in attesa. Appare in `/config` come **Local notifications**.

* **Scope**: [`Any file`](#scopes). Claude Code legge anche un valore lasciato in `~/.claude.json` da versioni precedenti.
* **Type**: stringa, una di:
  * `"auto"`: Claude Code invia una notifica desktop in iTerm2, Ghostty e Kitty, suona il campanello in Terminal.app solo quando il suo campanello udibile è disattivato e non fa nulla altrove
  * `"terminal_bell"`: Claude Code suona il carattere campanello in qualsiasi terminale
  * `"iterm2"`: Claude Code invia una notifica desktop iTerm2
  * `"iterm2_with_bell"`: Claude Code invia una notifica desktop iTerm2 e suona il campanello
  * `"kitty"`: Claude Code invia una notifica desktop Kitty
  * `"ghostty"`: Claude Code invia una notifica desktop Ghostty
  * `"notifications_disabled"`: Claude Code non invia alcuna notifica
* **Default**: `"auto"`

```json settings.json theme={null}
{
  "preferredNotifChannel": "terminal_bell"
}
```

Con `"auto"`, Claude Code invia una notifica desktop in iTerm2, Ghostty e Kitty. In Terminal.app suona il carattere campanello solo quando hai disattivato il campanello udibile di Terminal e in altri terminali non fa nulla. Imposta `"terminal_bell"` per suonare il carattere campanello in qualsiasi terminale. Vedi [Ottieni un campanello terminale o una notifica](/docs/it/terminal-config#get-a-terminal-bell-or-notification).

<h3 id="remote-defaultenvironmentid">
  `remote.defaultEnvironmentId`
</h3>

Scegli l'[ambiente cloud](/docs/it/cloud-environments) predefinito per le sessioni cloud che crei dalla CLI, come con `claude --cloud`. Claude Code scrive questa chiave nelle tue impostazioni utente quando scegli un ambiente con [`/remote-env`](/docs/it/cloud-environments#select-an-environment-from-the-cli).

* **Scope**: [`Any file`](#scopes). Per un ID ambiente self-hosted, impostazioni utente o gestite o il flag `--settings` solo.
* **Type**: stringa, un ID ambiente come `env_...` o `ccpool_...`
* **Default**: non impostato, quindi Claude Code utilizza l'ambiente ospitato da Anthropic quando il tuo elenco ne ha uno e altrimenti il primo ambiente nel tuo elenco che non è un'[ambiente ponte Controllo remoto](/docs/it/cloud-environments#the-default-environment), o il primo ambiente quando tutti sono ambienti ponte
* **Per-session overrides**: `--environment` ha la precedenza su questa chiave per la sessione cloud che crea

```json settings.json theme={null}
{
  "remote": {
    "defaultEnvironmentId": "env_0123abcd"
  }
}
```

Un ID ambiente ospitato da Anthropic, che inizia con `env_`, segue la precedenza delle impostazioni standard, quindi un valore nelle impostazioni di progetto di un repository sovrascrive la tua scelta a livello utente. Un ID [ambiente self-hosted](/docs/it/self-hosted-environments), che inizia con `ccpool_`, è onorato solo dalle impostazioni utente, impostazioni gestite e il flag `--settings`; Claude Code ignora uno nelle impostazioni di progetto o locali di un repository e `/remote-env` mostra quale valore ha ignorato, quindi un file archiviato non può indirizzare le sessioni su un ambiente self-hosted che non hai scelto.

<h3 id="remotecontrolatstartup">
  `remoteControlAtStartup`
</h3>

Connetti il [Controllo remoto](/docs/it/remote-control) automaticamente quando ogni sessione interattiva inizia, invece di aspettare `/remote-control`. Impostalo su `true` per attivare la connessione automatica, `false` per disattivarla. Appare in `/config` come **Abilita Controllo remoto per tutte le sessioni**.

* **Scope**: [`Any file`](#scopes). Claude Code legge anche un valore lasciato in `~/.claude.json` da versioni precedenti.
* **Type**: Boolean
  * `true`: Claude Code connette il Controllo remoto automaticamente quando ogni sessione interattiva inizia
  * `false`: Claude Code aspetta `/remote-control`
* **Default**: non impostato, quindi la connessione automatica segue il default amministratore della tua organizzazione quando ne è impostato uno e altrimenti il default attuale di Claude Code
* **Per-session overrides**: `--remote-control` attiva il Controllo remoto per una sessione anche quando questa chiave è `false` e nessun flag lo disattiva per una sessione

```json settings.json theme={null}
{
  "remoteControlAtStartup": true
}
```

Claude Code ignora un `true` dalle impostazioni di progetto o locali, quindi un repository può disattivare la connessione automatica per il suo checkout ma non può attivarla. Per il comportamento completo per scope, vedi [Abilita Controllo remoto per tutte le sessioni](/docs/it/remote-control#enable-remote-control-for-all-sessions) e le [chiavi di sicurezza in cui il valore più rigoroso si applica](/docs/it/settings#security-keys-where-the-stricter-value-applies).

<h3 id="sshconfigs">
  `sshConfigs`
</h3>

Aggiungi connessioni SSH al menu a discesa dell'ambiente [Desktop](/docs/it/desktop#pre-configure-ssh-connections-for-your-team). Gli amministratori lo usano per distribuire connessioni condivise a un team. Le connessioni che definisci nelle impostazioni gestite vengono mostrate come gestite, quindi gli utenti possono selezionarle ma non possono modificarle o eliminarle nell'app.

* **Scope**: [`User or managed`](#scopes). L'app desktop legge questa chiave.
* **Type**: array di oggetti, ognuno con `id`, `name` e `sshHost` obbligatori e `sshPort` e `sshIdentityFile` opzionali
* **Default**: non impostato

Questo esempio aggiunge una connessione denominata `Dev VM` che si connette a `user@dev.example.com`:

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

Limita gli host a cui una [sessione SSH Desktop](/docs/it/desktop#restrict-which-ssh-hosts-users-can-connect-to) può connettersi. Solo l'app Desktop legge questa chiave; il CLI non lo fa. I pattern sono case-insensitive: `*` corrisponde a qualsiasi host, `*.example.com` corrisponde a `example.com` e a ogni sottodominio e qualsiasi altra cosa è una corrispondenza esatta rispetto al nome host dopo la risoluzione di `~/.ssh/config`. Un array vuoto disattiva le sessioni SSH.

* **Scope**: [`Managed`](#scopes)
* **Type**: array di pattern di nome host
* **Default**: non impostato, quindi qualsiasi host è consentito

Questo esempio consente `devboxes.example.com` e i suoi sottodomini, più l'host esatto `bastion.example.com`:

```json managed-settings.json theme={null}
{
  "sshHostAllowlist": ["*.devboxes.example.com", "bastion.example.com"]
}
```

<span id="authentication-and-login" />

<h2 id="authentication-and-providers">
  Autenticazione e provider
</h2>

Fornisci credenziali tramite script helper e, per le organizzazioni, forza un metodo di accesso o un'organizzazione. Vedi [Autenticazione](/docs/it/authentication).

<h3 id="apikeyhelper">
  `apiKeyHelper`
</h3>

Esegui il tuo comando per produrre le credenziali che Claude Code invia con le richieste del modello. Claude Code esegue il comando attraverso la shell di sistema, `/bin/sh` su macOS e Linux e `cmd` su Windows, e invia il suo output come intestazioni sia `X-Api-Key` che `Authorization: Bearer`. Usalo per credenziali dinamiche o rotanti, come token di breve durata recuperati da un vault.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, una riga di comando shell
* **Default**: non impostato, quindi Claude Code non esegue un helper

```json settings.json theme={null}
{
  "apiKeyHelper": "/bin/generate_temp_api_key.sh"
}
```

Claude Code memorizza nella cache il valore e riesegue il comando in questi casi:

* Dopo la durata della cache, cinque minuti per impostazione predefinita o l'intervallo che imposti con [`CLAUDE_CODE_API_KEY_HELPER_TTL_MS`](/docs/it/env-vars).
* Quando una richiesta all'API Anthropic, direttamente o tramite un [gateway LLM](/docs/it/llm-gateway), fallisce con `401` o `403`.
* Prima di inviare una richiesta all'API Anthropic, direttamente o tramite un gateway LLM, quando l'output memorizzato nella cache è un JWT scaduto dopo che l'helper lo ha prodotto. Richiede Claude Code v2.1.246 o successivo.

Gli ultimi due casi si applicano solo quando l'output dell'helper è la credenziale che Claude Code invia e `ANTHROPIC_AUTH_TOKEN` non è impostato.

Nelle sessioni interattive, quando il comando proviene dalle impostazioni del progetto o locali, Claude Code non lo esegue finché non accetti il prompt di fiducia dell'area di lavoro. Vedi [Gestione delle credenziali](/docs/it/authentication#credential-management).

<h3 id="awsauthrefresh">
  `awsAuthRefresh`
</h3>

Esegui il tuo comando, come `aws sso login`, per aggiornare le credenziali nella tua directory `.aws` quando quelle che Claude Code ha per [Amazon Bedrock](/docs/it/amazon-bedrock) smettono di funzionare. Claude Code controlla prima le credenziali attuali rispetto a STS ed esegue il comando solo quando quel controllo fallisce, quindi legge la directory `.aws` aggiornata.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, una riga di comando shell
* **Default**: non impostato, quindi Claude Code non aggiorna le credenziali AWS per te

```json settings.json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile myprofile"
}
```

Usa questa chiave quando il tuo flusso di aggiornamento scrive in `.aws`; usa [`awsCredentialExport`](#awscredentialexport) quando stampa credenziali invece. Vedi [configurazione avanzata delle credenziali](/docs/it/amazon-bedrock#advanced-credential-configuration).

<h3 id="awscredentialexport">
  `awsCredentialExport`
</h3>

Esegui il tuo comando che stampa le credenziali AWS come JSON, in modo che Claude Code possa chiamare [Amazon Bedrock](/docs/it/amazon-bedrock) con credenziali che non vivono nella tua directory `.aws`. Claude Code accetta la forma di output `aws sts` e la forma piatta `aws configure export-credentials`, e limita le credenziali al suo client Bedrock, quindi i comandi shell che Claude esegue vedono ancora le tue credenziali ambientali.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, una riga di comando shell
* **Default**: non impostato, quindi Claude Code utilizza la catena di credenziali AWS ambientale

```json settings.json theme={null}
{
  "awsCredentialExport": "/bin/generate_aws_grant.sh"
}
```

A differenza di [`awsAuthRefresh`](#awsauthrefresh), Claude Code esegue sempre questo comando quando è impostato, senza controllare prima le credenziali ambientali. Vedi [configurazione avanzata delle credenziali](/docs/it/amazon-bedrock#advanced-credential-configuration).

<h3 id="forceloginmethod">
  `forceLoginMethod`
</h3>

Limita il tipo di account con cui le persone possono accedere. Imposta `"claudeai"` per consentire solo account claude.ai, `"console"` per consentire solo account Claude Console, o `"gateway"` per inviare le persone a un [cloud gateway](/docs/it/claude-apps-gateway) invece di un accesso di prima parte. Gli amministratori lo impostano nelle impostazioni gestite e lo associano a [`forceLoginOrgUUID`](#forceloginorguuid) per mantenere gli accessi claude.ai degli sviluppatori all'interno di un'organizzazione. Se lo imposti su `"claudeai"` o `"console"` in qualsiasi file di impostazioni, Claude Code smette anche di offrire l'[accesso Console senza chiave](/docs/it/authentication#sign-in-without-an-api-key) nelle sessioni a cui si applica quel file.

* **Scope**: [`Any file`](#scopes). Claude Code onora `"gateway"` solo da una fonte gestita sulla macchina: `managed-settings.json`, il plist macOS o il registro HKLM di Windows, o un helper di policy. Lo tratta come non impostato nelle impostazioni utente, progetto, locali, HKCU e gestite dal server, la stessa regola di [`forceLoginGatewayUrl`](#forcelogingatewayurl).
* **Type**: string, uno di:
  * `"claudeai"`: solo gli account claude.ai possono accedere
  * `"console"`: solo gli account Claude Console possono accedere
  * `"gateway"`: Claude Code invia le persone a un cloud gateway invece di un accesso di prima parte
* **Default**: non impostato, quindi le persone scelgono un metodo di accesso

```json settings.json theme={null}
{
  "forceLoginMethod": "claudeai"
}
```

Ogni percorso di accesso di prima parte applica la restrizione, inclusa l'[estensione VS Code](/docs/it/vs-code), l'Agent SDK, `claude setup-token`, e `/install-github-app`, ad eccezione della schermata di accesso interattiva del terminale, raggiunta da `/login` o dall'onboarding al primo avvio, che pre-seleziona il metodo senza applicarlo. Prima della v2.1.212, solo gli accessi al terminale lo applicavano. Vedi [Limita l'accesso alla tua organizzazione](/docs/it/authentication#restrict-login-to-your-organization) per come ogni percorso di accesso, le credenziali ambientali e i provider di terze parti vengono gestiti.

Quando una fonte gestita sulla macchina imposta `"gateway"`, Claude Code non utilizza un accesso residuo, una chiave API o una credenziale `apiKeyHelper`. Vedi [La policy dell'amministratore richiede un accesso Cloud gateway](/docs/it/errors#administrator-policy-requires-a-cloud-gateway-sign-in) per il messaggio che ognuno produce. Se selezioni un provider cloud tramite `CLAUDE_CODE_USE_BEDROCK` o una variabile di ambiente simile, la sessione non ha bisogno dell'accesso al gateway. Prima della v2.1.261, Claude Code utilizzava un accesso residuo su queste macchine.

<h3 id="forcelogingatewayurl">
  `forceLoginGatewayUrl`
</h3>

Imposta l'URL del gateway a cui si connette la schermata `/login` Cloud gateway, in modo che le persone raggiungano il tuo [cloud gateway](/docs/it/claude-apps-gateway) senza digitare il suo indirizzo. La schermata non ha un campo URL: con questa chiave impostata, mostra l'URL del tuo gateway e si connette quando la persona preme Invio; senza di essa, dice loro di contattare il loro amministratore IT.

O questa chiave o `forceLoginMethod: "gateway"` rende la macchina solo gateway, quindi `/login` si apre sulla schermata Cloud gateway senza un selettore di metodo di accesso. Vedi [La policy dell'amministratore richiede un accesso Cloud gateway](/docs/it/errors#administrator-policy-requires-a-cloud-gateway-sign-in) per cosa succede a un accesso di prima parte residuo o a una chiave API. Imposta entrambe le chiavi in modo che la schermata si connetta invece di mostrare un errore.

* **Scope**: [`Managed`](#scopes). Leggi solo da una fonte sulla macchina: `managed-settings.json`, il plist macOS o il registro HKLM di Windows, o un helper di policy. Claude Code lo ignora nelle impostazioni HKCU e gestite dal server.
* **Type**: string, un URL completo incluso lo schema
* **Default**: non impostato, quindi la schermata Cloud gateway mostra un errore che dice alle persone di contattare il loro amministratore IT

```json managed-settings.json theme={null}
{
  "forceLoginGatewayUrl": "https://claude-gateway.example.com"
}
```

Se il valore non è un URL valido, la schermata di accesso lo segnala e il resto del file di impostazioni gestite si applica comunque. Vedi [Imposta l'URL del gateway](/docs/it/claude-apps-gateway#set-the-gateway-url).

<h3 id="forceloginorguuid">
  `forceLoginOrgUUID`
</h3>

Da una fonte gestita, richiedi che gli accessi dell'account claude.ai appartengano a un'organizzazione Anthropic, data come un singolo UUID, o a una qualsiasi di diverse organizzazioni, data come un array. Da qualsiasi file di impostazioni, Claude Code utilizza anche un singolo UUID per pre-selezionare quell'organizzazione durante un accesso claude.ai o Claude Console, e non pre-seleziona nulla per un array. Se imposti la chiave in qualsiasi file di impostazioni, Claude Code smette anche di offrire l'[accesso Console senza chiave](/docs/it/authentication#sign-in-without-an-api-key) nelle sessioni a cui si applica quel file e crea una chiave API invece.

* **Scope**: [`Any file`](#scopes). Solo una fonte gestita applica la restrizione; un singolo UUID in qualsiasi altro file di impostazioni pre-seleziona l'organizzazione durante l'accesso senza limitarla.
* **Type**: string, un UUID, o array di stringhe, diversi UUID
* **Default**: non impostato, quindi qualsiasi organizzazione può accedere

Questo esempio accetta accessi da una qualsiasi di due organizzazioni senza pre-selezionarne una:

```json managed-settings.json theme={null}
{
  "forceLoginOrgUUID": ["xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx", "yyyyyyyy-yyyy-yyyy-yyyy-yyyyyyyyyyyy"]
}
```

Se una fonte gestita imposta un array vuoto, o un valore che Claude Code non può analizzare, Claude Code blocca ogni accesso con un messaggio di configurazione errata.

Vedi [Limita l'accesso alla tua organizzazione](/docs/it/authentication#restrict-login-to-your-organization) per come Claude Code tratta gli accessi Claude Console, gli altri percorsi di accesso e le credenziali ambientali.

<h3 id="gatewayinternalnetworks">
  `gatewayInternalNetworks`
</h3>

Dichiara i blocchi IPv4 pubblici da cui la tua organizzazione numera la sua rete interna, in modo che `/login` accetti un [cloud gateway](/docs/it/claude-apps-gateway) lì. Richiede Claude Code v2.1.268 o successivo.

Senza questa chiave, `/login` si connette a qualsiasi gateway su un indirizzo privato e nient'altro. Con essa, `/login` accetta anche un gateway all'interno di un blocco elencato, solo su una connessione diretta. L'indirizzo della macchina stessa su quella connessione deve essere anche all'interno dello stesso blocco.

* **Scope**: [`Managed`](#scopes). Leggi solo da una fonte sulla macchina: `managed-settings.json`, il plist macOS o il registro HKLM di Windows, o un helper di policy. Claude Code lo ignora nelle impostazioni HKCU e gestite dal server.
* **Type**: array di stringhe, al massimo quattro blocchi IPv4 CIDR, ognuno `/8` a `/32`, non sovrapponendosi l'uno con l'altro, e nessuno che si sovrappone allo spazio privato.
* **Default**: non impostato, quindi `/login` accetta solo gateway su indirizzi privati

```json managed-settings.json theme={null}
{
  "gatewayInternalNetworks": ["203.0.113.0/24"]
}
```

Sostituisci l'intervallo di documentazione nell'esempio con il tuo blocco. Claude Code rifiuta gli intervalli di documentazione, gli intervalli che i client VPN e NAT64 usano localmente, e lo spazio riservato da cui nessuna rete è numerata, come il multicast.

Se una voce non è valida, o il valore non è un elenco di stringhe, `/login` nomina il problema e rifiuta ogni nuovo accesso al gateway sulla macchina finché non correggi il valore. Gli accessi esistenti continuano a funzionare. Vedi [Consenti un gateway su spazio di indirizzi pubblico che possiedi](/docs/it/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) per le regole complete e cosa vedono gli sviluppatori.

<h3 id="gcpauthrefresh">
  `gcpAuthRefresh`
</h3>

Esegui il tuo comando per aggiornare le credenziali predefinite dell'applicazione Google Cloud quando Claude Code scopre che sono scadute o non possono essere caricate, in modo che le richieste di [Google Cloud's Agent Platform](/docs/it/google-vertex-ai) continuino a funzionare senza che tu ti autentica di nuovo manualmente.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, una riga di comando shell
* **Default**: non impostato, quindi l'errore di credenziale di Claude Code ti dice di eseguire `gcloud auth application-default login` tu stesso

```json settings.json theme={null}
{
  "gcpAuthRefresh": "gcloud auth application-default login"
}
```

Vedi [configurazione avanzata delle credenziali](/docs/it/google-vertex-ai#advanced-credential-configuration).

<h3 id="otelheadershelper">
  `otelHeadersHelper`
</h3>

Esegui il tuo comando per generare le intestazioni che Claude Code invia con le esportazioni OpenTelemetry, per backend i cui token ruotano. Claude Code lo esegue all'avvio e periodicamente dopo, e si aspetta un oggetto JSON di valori di intestazione stringa su stdout.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, un percorso eseguibile o una riga di comando shell
* **Default**: non impostato, quindi Claude Code non aggiunge intestazioni generate da helper

```json settings.json theme={null}
{
  "otelHeadersHelper": "/bin/generate_otel_headers.sh"
}
```

Imposta l'intervallo di aggiornamento con [`CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`](/docs/it/env-vars). Vedi [Dynamic headers](/docs/it/monitoring-usage#dynamic-headers) per i requisiti dello script e cosa succede quando l'helper fallisce.

<h2 id="updates-and-versioning">
  Aggiornamenti e versioning
</h2>

Scegli un canale di aggiornamento e, per le organizzazioni, fissa le versioni che le persone possono eseguire. Vedi [Aggiorna Claude Code](/docs/it/setup#update-claude-code).

<h3 id="autoupdateschannel">
  `autoUpdatesChannel`
</h3>

Scegli quale [canale di rilascio](/docs/it/setup#configure-release-channel) seguono gli aggiornamenti automatici in background e `claude update`. Imposta `"stable"` per una versione che è tipicamente di circa una settimana fa e salta i rilasci con regressioni importanti, oppure `"latest"` per il rilascio più recente.

* **Scope**: [`Any file`](#scopes). Impostalo nelle impostazioni gestite per applicare un canale in tutta l'organizzazione.
* **Type**: string, uno di:
  * `"latest"`: gli aggiornamenti seguono il rilascio più recente
  * `"stable"`: gli aggiornamenti seguono una versione che è tipicamente di circa una settimana fa e salta i rilasci con regressioni importanti
* **Default**: non impostato, quindi Claude Code segue `"latest"`

```json settings.json theme={null}
{
  "autoUpdatesChannel": "stable"
}
```

Claude Code scrive `"stable"` nelle tue impostazioni utente quando lo scegli in **Auto-update channel** in `/config`, e rimuove la chiave quando torni a latest lì. `claude install stable` e `claude install latest` salvano anche il canale che nomini. Passare da `"latest"` a `"stable"` in `/config` chiede se consentire un downgrade o rimanere sulla versione corrente; rimanere imposta [`minimumVersion`](#minimumversion). Gli install di Homebrew ignorano questa chiave: il cask `claude-code` traccia stable e `claude-code@latest` traccia latest, e `claude update` si rimette a `brew upgrade`. Per disattivare completamente gli aggiornamenti automatici, imposta [`DISABLE_AUTOUPDATER`](/docs/it/setup#disable-auto-updates) in `env`.

<h3 id="minimumversion">
  `minimumVersion`
</h3>

Impedisci agli aggiornamenti automatici in background e a `claude update` di installare qualsiasi versione al di sotto di questa, quindi il passaggio al canale `"stable"` non ti fa eseguire il downgrade da una build `"latest"` più recente. Claude Code scrive questa chiave per te quando scegli di rimanere sulla versione corrente mentre cambi canale in `/config`, e la cancella quando torni a `"latest"`.

* **Scope**: [`Any file`](#scopes). Impostalo nelle impostazioni gestite per fissare un minimo a livello di organizzazione che le impostazioni utente e di progetto non possono abbassare.
* **Type**: string, un numero di versione come `"2.1.100"`; un valore che non è una versione valida viene ignorato
* **Default**: non impostato, quindi gli aggiornamenti possono installare qualsiasi versione che il canale offre

Questo esempio segue il canale stable e rifiuta di installare qualsiasi versione al di sotto di 2.1.100:

```json settings.json theme={null}
{
  "autoUpdatesChannel": "stable",
  "minimumVersion": "2.1.100"
}
```

Questa chiave vincola solo gli aggiornamenti. Per fare in modo che Claude Code rifiuti di avviarsi al di sotto di una versione, usa [`requiredMinimumVersion`](#requiredminimumversion) invece. Vedi [Fissa una versione minima](/docs/it/setup#pin-a-minimum-version).

<h3 id="requiredmaximumversion">
  `requiredMaximumVersion`
</h3>

Imposta la versione più recente di Claude Code che la tua organizzazione consente di avviare. Quando la versione in esecuzione è più recente, Claude Code esce all'avvio e dice all'utente di installare una versione approvata attraverso il metodo approvato della tua organizzazione; `claude install <version>` potrebbe funzionare anche. Richiede Claude Code v2.1.163 o successivo.

* **Scope**: [`Managed`](#scopes). Claude Code non dà alcun avviso quando ignora la chiave altrove.
* **Type**: string, un numero di versione come `"2.1.150"`; un valore che non è una versione valida viene ignorato
* **Default**: non impostato, quindi non si applica alcun limite massimo

```json managed-settings.json theme={null}
{
  "requiredMaximumVersion": "2.1.150"
}
```

Gli aggiornamenti automatici in background e `claude update` saltano le versioni al di sopra del limite, quindi un'installazione all'interno dell'intervallo rimane all'interno di esso. `claude update`, `claude install`, e `claude doctor` continuano a funzionare al di sopra del limite in modo che gli utenti possano recuperare. Abbinalo a [`requiredMinimumVersion`](#requiredminimumversion) per applicare un intervallo.

<h3 id="requiredminimumversion">
  `requiredMinimumVersion`
</h3>

Imposta la versione più vecchia di Claude Code che la tua organizzazione consente di avviare. Quando la versione in esecuzione è più vecchia, Claude Code esce all'avvio e dice all'utente di aggiornare attraverso il metodo approvato della tua organizzazione. Il controllo viene eseguito solo all'avvio, quindi una sessione già in esecuzione continua. Richiede Claude Code v2.1.163 o successivo.

* **Scope**: [`Managed`](#scopes). Claude Code non dà alcun avviso quando ignora la chiave altrove.
* **Type**: string, un numero di versione come `"2.1.150"`; un valore che non è una versione valida viene ignorato
* **Default**: non impostato, quindi non si applica alcun limite minimo

```json managed-settings.json theme={null}
{
  "requiredMinimumVersion": "2.1.150"
}
```

`claude update`, `claude install`, e `claude doctor` continuano a funzionare al di sotto del limite in modo che gli utenti possano recuperare. A differenza di [`minimumVersion`](#minimumversion), che previene solo i downgrade, questa chiave blocca l'avvio. Abbinalo a [`requiredMaximumVersion`](#requiredmaximumversion) per applicare un intervallo.

<h2 id="tools">
  Tools
</h2>

Disabilita strumenti specifici nell'[app desktop Claude](/docs/it/desktop). La CLI del terminale ignora queste chiavi. Per gli strumenti stessi, vedi [Strumenti disponibili per Claude](/docs/it/tools-reference).

<h3 id="browserexternalpagetools">
  `browserExternalPageTools`
</h3>

Impedisci a Claude di utilizzare i suoi strumenti per leggere o agire su pagine esterne nel [riquadro Browser](/docs/it/desktop#browse-external-sites) dell'app desktop. Le persone nella tua organizzazione possono comunque aprire siti esterni da sole, e le anteprime dei server di sviluppo locali continuano a funzionare con gli strumenti di Claude. L'app desktop legge questa chiave; la CLI del terminale la ignora.

* **Scope**: [`Managed`](#scopes)
* **Type**: stringa, `"disabled"`; l'app desktop accetta anche `"disable"`, in entrambi i casi
* **Default**: non impostato, quindi gli strumenti di Claude funzionano su pagine esterne

```json managed-settings.json theme={null}
{
  "browserExternalPageTools": "disabled"
}
```

Qualsiasi altro valore lascia gli strumenti di Claude attivi, e una stringa non vuota che non sia uno dei due valori accettati registra un avviso. Per bloccare i siti esterni sia per le persone che per Claude, imposta invece [`disableBrowserExternalNavigation`](#disablebrowserexternalnavigation). Vedi [Limitare la navigazione esterna per la tua organizzazione](/docs/it/desktop#restrict-external-browsing-for-your-organization).

<h3 id="disablebrowserexternalnavigation">
  `disableBrowserExternalNavigation`
</h3>

Disabilita la navigazione esterna nel [riquadro Browser](/docs/it/desktop#browse-external-sites) dell'app desktop sia per le persone che per Claude. Le anteprime dei server di sviluppo localhost continuano a funzionare. L'app desktop legge questa chiave; la CLI del terminale la ignora.

* **Scope**: [`Managed`](#scopes)
* **Type**: Booleano; solo il Booleano JSON `true` ha effetto
  * `true`: l'app desktop disabilita la navigazione esterna nel riquadro Browser sia per le persone che per Claude; le anteprime localhost continuano a funzionare
  * `false`: la navigazione esterna rimane attiva
* **Default**: non impostato, quindi la navigazione esterna è attiva

```json managed-settings.json theme={null}
{
  "disableBrowserExternalNavigation": true
}
```

L'app desktop ignora qualsiasi altro valore, e un valore che non sia un Booleano, come la stringa `"true"` o `1`, registra anche un avviso. Per lasciare la navigazione esterna attiva ma mantenere gli strumenti di Claude disabilitati su pagine esterne, imposta invece [`browserExternalPageTools`](#browserexternalpagetools). Vedi [Limitare la navigazione esterna per la tua organizzazione](/docs/it/desktop#restrict-external-browsing-for-your-organization).

<h3 id="disablemobilesimulatortools">
  `disableMobileSimulatorTools`
</h3>

Blocca gli strumenti di Claude per il [riquadro iOS Simulator](/docs/it/desktop-ios-simulator#turn-off-simulator-access) dell'app desktop. Le persone mantengono l'uso manuale del riquadro; solo l'accesso di Claude viene rimosso, e nessuno può riattivarlo dall'interno dell'app. L'app desktop legge questa chiave; la CLI del terminale la ignora.

* **Scope**: [`Managed`](#scopes)
* **Type**: Booleano; solo il Booleano JSON `true` ha effetto
  * `true`: l'app desktop blocca gli strumenti di Claude per il riquadro iOS Simulator
  * `false`: gli strumenti del simulatore di Claude seguono l'interruttore delle impostazioni di ogni persona nell'app desktop
* **Default**: non impostato, quindi gli strumenti del simulatore di Claude seguono l'interruttore delle impostazioni di ogni persona nell'app desktop

```json managed-settings.json theme={null}
{
  "disableMobileSimulatorTools": true
}
```

L'app desktop ignora qualsiasi altro valore, e un valore che non sia un Booleano, come la stringa `"true"` o `1`, registra anche un avviso.

<span id="data-and-privacy" />

<h2 id="privacy-and-telemetry">
  Privacy e telemetria
</h2>

Controlla per quanto tempo Claude Code mantiene i dati della sessione e cosa invia. Gli interruttori che disattivano le metriche di utilizzo e i rapporti di errore sono variabili di ambiente, non chiavi di impostazione: imposta `DISABLE_TELEMETRY`, `DISABLE_ERROR_REPORTING`, o `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` nella chiave [`env`](#env) o nella shell. [Telemetry services](/docs/it/data-usage#telemetry-services) dice cosa disattiva ognuno. Due eccezioni si disattivano da un file di impostazioni: [`feedbackDrafts`](#feedbackdrafts) di seguito per il feedback redatto da Claude, e [`feedbackSurveyRate`](#feedbacksurveyrate) di seguito per il sondaggio della sessione.

<h3 id="cleanupperioddays">
  `cleanupPeriodDays`
</h3>

Imposta quanti giorni Claude Code mantiene [i trascritti della sessione e altri dati dell'applicazione](/docs/it/claude-directory#cleaned-up-automatically) prima di eliminarli. Claude Code esegue l'eliminazione come una scansione di background dopo l'avvio di una sessione, purché possa determinare in modo sicuro il periodo di conservazione.

* **Scope**: [`Any file`](#scopes)
* **Type**: numero di giorni, un numero intero, minimo `1`
* **Default**: `30`

```json settings.json theme={null}
{
  "cleanupPeriodDays": 20
}
```

L'impostazione di `0` non supera la convalida, quindi scegli un valore grande come `3650` per una conservazione a lungo termine. Per impedire a Claude Code di scrivere trascritti, vedi [Plaintext storage](/docs/it/claude-directory#plaintext-storage).

<h3 id="desktopsessioncleanupperioddays">
  `desktopSessionCleanupPeriodDays`
</h3>

Imposta un limite di età in giorni per i trascritti delle sessioni che hai avviato o continuato più di recente in Claude Desktop o Cowork. Senza questa chiave, Claude Code [mantiene quei trascritti a qualsiasi età](/docs/it/claude-directory#cleaned-up-automatically). Claude Code elimina ognuno una volta che è più vecchio sia di questo limite che di [`cleanupPeriodDays`](#cleanupperioddays), quindi con `cleanupPeriodDays` al suo valore predefinito di 30, un valore di `7` li mantiene comunque 30 giorni. Quando le impostazioni gestite impostano `cleanupPeriodDays`, quel periodo si applica invece e questa chiave viene ignorata. Richiede Claude Code v2.1.248 o successivo.

* **Scope**: [`User or managed`](#scopes). Claude Code legge anche la chiave da un file che passi con `--settings`, e la ignora nelle impostazioni di progetto e locali.
* **Type**: numero di giorni, un numero intero, minimo `0`
* **Default**: `0`, che non imposta alcun limite di età

```json settings.json theme={null}
{
  "desktopSessionCleanupPeriodDays": 90
}
```

<h3 id="feedbackdrafts">
  `feedbackDrafts`
</h3>

Controlla [il feedback redatto da Claude](/docs/it/tools-reference#sendfeedback-tool-behavior): se Claude può mettere in coda le bozze di feedback per la tua revisione, e se Claude Code mostra una scheda quando Claude ne mette in coda una.

* **Scope**: [`User or managed`](#scopes)
* **Type**: stringa, uno di `"notify"`, `"quiet"`, o `"off"`
  * `"notify"`: Claude Code mostra una scheda sopra il prompt quando Claude mette in coda una bozza, fino a [tre schede in una sessione](/docs/it/tools-reference#what-you-see-when-claude-drafts) per impostazione predefinita
  * `"quiet"`: Claude redige senza una scheda. Vedi il conteggio delle bozze in coda nel footer del prompt e le rivedi in `/feedback`
  * `"off"`: Claude Code rimuove lo strumento SendFeedback, quindi Claude non può mettere in coda le bozze
* **Default**: `"notify"`
* **Per-session overrides**: [`CLAUDE_CODE_SEND_FEEDBACK`](/docs/it/env-vars) impostato a `0` disattiva la funzione per una sessione

```json settings.json theme={null}
{
  "feedbackDrafts": "quiet"
}
```

Appare in `/config` come **Claude-drafted feedback**, che scrive questa chiave nelle tue impostazioni utente. Vedi la riga `/config` solo nelle sessioni [dove Claude può redigere feedback](/docs/it/tools-reference#sessions-without-claude-drafted-feedback); l'impostazione di `"off"` non la nasconde, quindi puoi riattivare la funzione dalla stessa riga. Un valore nelle impostazioni gestite ha la precedenza sulla tua impostazione utente, quindi quando un amministratore imposta questa chiave, la riga mostra il valore gestito e modificarlo non ha effetto. Claude Code ignora questa chiave nelle impostazioni di progetto e locali.

<h3 id="feedbacksurveyrate">
  `feedbackSurveyRate`
</h3>

Imposta la probabilità che il [sondaggio sulla qualità della sessione](/docs/it/data-usage#session-quality-surveys) appaia quando una sessione è idonea per esso. Imposta `0` per impedire che il sondaggio appaia.

* **Scope**: [`Any file`](#scopes)
* **Type**: numero tra `0` e `1`
* **Default**: non impostato, quindi Claude Code utilizza la velocità che Anthropic imposta da remoto, o la sua velocità incorporata di `0.005` su Amazon Bedrock, Google Cloud's Agent Platform, e Microsoft Foundry, che non ricevono configurazione remota
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY`](/docs/it/env-vars) impostato a `1` disattiva il sondaggio per una sessione qualunque sia la velocità che questa chiave imposta

```json settings.json theme={null}
{
  "feedbackSurveyRate": 0.05
}
```

La stessa velocità si applica al sondaggio nell'estensione VS Code.

<h3 id="skipwebfetchpreflight">
  `skipWebFetchPreflight`
</h3>

Salta il [controllo di sicurezza del dominio WebFetch](/docs/it/data-usage#webfetch-domain-safety-check), che invia ogni nome host richiesto a `api.anthropic.com` prima di recuperare. Imposta `true` negli ambienti che bloccano il traffico verso Anthropic, come Amazon Bedrock, Google Cloud's Agent Platform, o distribuzioni Microsoft Foundry con egress restrittivo.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code salta il controllo di sicurezza del dominio WebFetch
  * `false`: il controllo viene eseguito prima del primo recupero per ogni nome host in una sessione, e di nuovo per un nome host il cui controllo precedente è stato bloccato o non riuscito
* **Default**: non impostato, quindi il controllo viene eseguito prima del primo recupero per ogni nome host in una sessione

```json settings.json theme={null}
{
  "skipWebFetchPreflight": true
}
```

Con il controllo saltato, WebFetch tenta qualsiasi URL senza consultare la lista di blocco, quindi abbinalo alle [regole di autorizzazione `WebFetch`](/docs/it/permissions#webfetch) se hai bisogno di limitare quali domini Claude può raggiungere.

<span id="managed-policy" />

<h2 id="enterprise-and-managed-settings">
  Impostazioni aziendali e gestite
</h2>

Chiavi che un'organizzazione utilizza per calcolare, aggiornare e combinare le impostazioni gestite. Vedere [Configurare le impostazioni gestite](/docs/it/admin-setup).

<h3 id="disablesideloadflags">
  `disableSideloadFlags`
</h3>

Rifiuta i flag CLI `--plugin-dir`, `--plugin-url`, `--agents` e `--mcp-config` all'avvio, che gli utenti potrebbero altrimenti passare per aggirare [`strictKnownMarketplaces`](#strictknownmarketplaces) per una singola esecuzione. Claude Code esce con un errore che nomina i flag rifiutati e applica lo stesso controllo alle superfici che avviano la CLI con questi flag internamente, attualmente le sessioni locali di [Cowork](/docs/it/desktop) nell'app desktop. Nelle [sessioni cloud](/docs/it/claude-code-on-the-web), Claude Code elimina i server MCP che il server ha fornito tramite `--mcp-config`, ad eccezione delle voci in-process `type: "sdk"`, e avvia la sessione. Richiede Claude Code v2.1.193 o successivo.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code rifiuta `--plugin-dir`, `--plugin-url`, `--agents` e `--mcp-config` all'avvio e esce con un errore che li nomina, tranne che nelle sessioni cloud dove elimina i server MCP che il server ha fornito tramite `--mcp-config`, ad eccezione delle voci in-process `type: "sdk"`, e avvia la sessione
  * `false`: Claude Code accetta questi flag
* **Default**: `false`

```json managed-settings.json theme={null}
{
  "disableSideloadFlags": true
}
```

Claude Code accetta comunque un `--mcp-config` i cui server sono tutti voci in-process `type: "sdk"`, quindi l'Agent SDK e l'estensione VS Code continuano a funzionare. Gli utenti possono comunque aggiungere server con `claude mcp add` o un file `.mcp.json`; per il controllo per server, impostare anche [`allowedMcpServers`](/docs/it/managed-mcp). Richiede Claude Code v2.1.193 o successivo.

Lo stesso controllo copre le cartelle di plugin nominate nella variabile di ambiente [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/it/env-vars#variables), che richiede Claude Code v2.1.280 o successivo. Quando la variabile nomina una cartella, Claude Code esce con lo stesso errore e l'errore dice di annullare l'impostazione della variabile.

Nelle sessioni cloud, Claude Code ignora anche gli aggiornamenti MCP forniti dal server a metà sessione, il percorso dietro la configurazione della sessione cloud e le chiamate SDK `setMcpServers()` che raggiungono quelle sessioni. Le voci in-process `type: "sdk"` rimangono esenti anche lì. Prima della v2.1.239, un `--mcp-config` fornito dal server bloccava l'avvio di una sessione cloud.

<h3 id="forceremotesettingsrefresh">
  `forceRemoteSettingsRefresh`
</h3>

Blocca l'avvio della CLI fino a quando Claude Code non ha recuperato di recente le [impostazioni gestite dal server](/docs/it/server-managed-settings). Se il recupero non riesce, Claude Code esce invece di continuare con le impostazioni memorizzate nella cache o nessuna impostazione. Impostarlo quando il tuo ambiente non può accettare nemmeno una breve finestra in cui una sessione viene eseguita senza la sua politica gestita.

Quando la chiave non è impostata, Claude Code non blocca l'avvio sul recupero, anche se quando lo sviluppatore accede all'avvio attende fino a cinque secondi per il recupero. Una sessione del gateway Cloud attende sempre e esce se il gateway non può essere raggiunto.

* **Scope**: [`Managed`](#scopes). Claude Code onora un `true` da qualsiasi fonte gestita controllata dall'amministratore, anche una che non è la fonte con la priorità più alta.
* **Type**: Boolean
  * `true`: Claude Code blocca l'avvio fino a quando non ha recuperato di recente le impostazioni gestite dal server e esce se il recupero non riesce
  * `false`: Claude Code non blocca l'avvio sul recupero, anche se all'avvio di un accesso attende fino a cinque secondi per il recupero
* **Default**: `false`

```json managed-settings.json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

Impostarlo in un profilo MDM o nel file delle impostazioni gestite per applicare l'avvio fail-closed prima che arrivi il primo payload del server. Claude Code applica il controllo solo nelle sessioni che recuperano le impostazioni gestite dal server, quindi una sessione che [non le recupera](/docs/it/server-managed-settings#platform-availability) si avvia senza attendere. I sottocomandi `claude auth` sono esenti, quindi gli utenti possono autenticarsi di nuovo quando le credenziali scadute sono il motivo per cui il recupero non riesce. Vedere [Applicare l'avvio fail-closed](/docs/it/server-managed-settings#enforce-fail-closed-startup).

<h3 id="managedsourcesbehavior">
  `managedSourcesBehavior`
</h3>

Scegli se Claude Code applica solo la [fonte gestita](/docs/it/managed-settings#how-claude-code-combines-managed-sources) con la priorità più alta che la tua organizzazione fornisce, o combina ogni fonte amministrativa che fornisce. Per impostazione predefinita, Claude Code prende la fonte con la priorità più alta che contiene una [chiave di politica](/docs/it/managed-settings#how-claude-code-combines-managed-sources) e ignora il resto. Una chiave di politica è qualsiasi chiave di impostazioni diversa da questa e da `wslInheritsWindowsSettings`. Quindi una volta che le impostazioni gestite dal server o una politica MDM forniscono una chiave di politica, un file `managed-settings.json` contribuisce solo alle [chiavi che Claude Code legge da ogni fonte amministrativa](/docs/it/managed-settings#keys-read-from-every-admin-source). Con `"merge"`, ogni fonte amministrativa che fornisci contribuisce con le sue chiavi a una politica combinata. Richiede Claude Code v2.1.242 o successivo.

Imposta `"merge"` solo dove ogni fonte [classificata](/docs/it/managed-settings#how-claude-code-combines-managed-sources) al di sotto della tua più alta è sotto il controllo di un amministratore, perché Claude Code quindi aggiunge voci da una fonte inferiore, come le regole `permissions.allow`, alla politica.

* **Scope**: [`Managed`](#scopes). Claude Code legge questa chiave dalla fonte con la priorità più alta che contiene questa chiave o una chiave di politica, e ignora questa chiave in ogni fonte classificata più in basso, quindi una fonte inferiore non può optare per la combinazione con la fonte sopra di essa. Né il registro HKCU di Windows né le [impostazioni padre da un host di incorporamento](/docs/it/managed-settings#let-an-embedding-host-add-policy) partecipano alla fusione.
* **Type**: string, uno di:
  * `"first-wins"`: la fonte con la priorità più alta che contiene una chiave di politica fornisce la politica, e le fonti inferiori contribuiscono solo alle [chiavi che Claude Code legge da ogni fonte amministrativa](/docs/it/managed-settings#keys-read-from-every-admin-source)
  * `"merge"`: ogni fonte amministrativa che fornisci contribuisce con le sue chiavi, combinate secondo le regole sottostanti
* **Default**: `"first-wins"`

Fornisci la chiave nella fonte con la priorità più alta che distribuisci. Una macchina che non riceve mai le impostazioni gestite dal server ha bisogno della chiave anche nel suo profilo MDM, perché Claude Code legge la chiave dalla fonte con la priorità più alta che la contiene o una chiave di politica. Un file `managed-settings.json` è la fonte amministrativa con la priorità più bassa, quindi `"merge"` impostato lì non ha alcuna fonte al di sotto di essa con cui combinarsi. Nelle impostazioni gestite dal server, la chiave appare così:

```json theme={null}
{
  "managedSourcesBehavior": "merge"
}
```

Sotto `"merge"`, Claude Code combina ogni chiave per il suo tipo. Questa tabella fornisce la regola per ogni tipo. Le righe della lista di restrizioni, valori-presi-interi e solo-fonte-più-alta nominano ogni chiave che coprono, e le altre righe forniscono esempi:

| Tipo di chiave                                  | Come Claude Code la combina                                                                                                                                                                                                    | Chiavi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :---------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Liste                                           | Combina voci da ogni fonte                                                                                                                                                                                                     | [`permissions.allow`](#permissions-allow), [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) e altre chiavi di lista                                                                                                                                                                                                                                                                                                                                                                                        |
| Blocchi                                         | Applica il valore più rigoroso che qualsiasi fonte imposta. Quando nessuna fonte imposta un valore rigoroso, applica un valore più lasco solo dalla fonte con la priorità più alta                                             | [`allowManagedPermissionRulesOnly`](#allowmanagedpermissionrulesonly), [`permissions.disableBypassPermissionsMode`](#permissions-disablebypasspermissionsmode) e altri blocchi booleani o enum                                                                                                                                                                                                                                                                                                                                |
| Liste di restrizioni                            | Prende la lista intera dalla fonte con la priorità più alta che la imposta, senza aggiungere voci da fonti inferiori. Quando la fonte con la priorità più alta non ne imposta una, la prende intera dalla fonte successiva     | [`availableModels`](#availablemodels), [`allowedMcpServers`](#allowedmcpservers), [`strictKnownMarketplaces`](#strictknownmarketplaces), [`allowedChannelPlugins`](#allowedchannelplugins) e la catena [`fallbackModel`](#fallbackmodel)                                                                                                                                                                                                                                                                                      |
| Valori presi interi                             | Prende il valore intero dalla fonte con la priorità più alta che lo imposta, senza combinare voci o campi da fonti inferiori. Quando la fonte con la priorità più alta non lo imposta, lo prende intero dalla fonte successiva | [`sandbox.credentials.awsPairs`](#sandbox-credentials-awspairs), [`sandbox.ripgrep`](#sandbox-ripgrep)                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Server MCP forniti                              | Combina i nomi dei server da ogni fonte. Quando due fonti impostano lo stesso nome, applica la voce intera della fonte più alta                                                                                                | [`managedMcpServers`](#managedmcpservers)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| Leggi solo dalla fonte con la priorità più alta | Legge la chiave solo dalla fonte con la priorità più alta che contiene una chiave di politica, quindi il valore di una fonte inferiore viene ignorato anche quando la fonte con la priorità più alta non ne imposta nessuno    | [`apiKeyHelper`](#apikeyhelper), [`awsAuthRefresh`](#awsauthrefresh), [`awsCredentialExport`](#awscredentialexport), [`gcpAuthRefresh`](#gcpauthrefresh), [`otelHeadersHelper`](#otelheadershelper), `proxyAuthHelper`, [`forceLoginOrgUUID`](#forceloginorguuid), i valori `"claudeai"` e `"console"` di [`forceLoginMethod`](#forceloginmethod), [`parentSettingsBehavior`](#parentsettingsbehavior), [`modelPicker`](#modelpicker), [`policyHelper`](#policyhelper), [`permissions.defaultMode`](#permissions-defaultmode) |
| `env`                                           | [Unisce per variabile tra fonti amministrative](/docs/it/managed-settings#keys-read-from-every-admin-source), sia sotto `"first-wins"` che `"merge"`                                                                                | [`env`](#env)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| Ogni altra chiave                               | Prende il valore dalla fonte con la priorità più alta che lo imposta                                                                                                                                                           | [`cleanupPeriodDays`](#cleanupperioddays), [`model`](#model)                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

Prendere `sandbox.credentials.awsPairs` e `sandbox.ripgrep` interi richiede Claude Code v2.1.257 o successivo.

Poche chiavi aggiungono una condizione che la tabella non mostra:

* **[`policyHelper`](#policyhelper)**: Claude Code lo onora solo quando la fonte con la priorità più alta che contiene una chiave di politica è una politica MDM o un file di impostazioni gestite, quindi sotto le impostazioni gestite dal server non si applica.
* **[`modelOverrides`](#modeloverrides)**: si accoppia con `availableModels`. Claude Code prende `modelOverrides` dalla fonte con la priorità più alta che lo imposta, a meno che una fonte più alta non imposti `availableModels` senza `modelOverrides`. In quel caso ignora `modelOverrides` da ogni fonte.
* **[`forceLoginGatewayUrl`](#forcelogingatewayurl), [`gatewayInternalNetworks`](#gatewayinternalnetworks) e il valore `"gateway"` di [`forceLoginMethod`](#forceloginmethod)**: Claude Code non legge mai nessuno di loro dalle impostazioni gestite dal server, quindi un valore lì non si applica né nasconde uno impostato in una politica MDM o file di impostazioni gestite. Tra le fonti amministrative sulla macchina, solo la fonte con la priorità più alta che contiene una chiave di politica li fornisce, indipendentemente dal fatto che le impostazioni gestite dal server siano presenti o meno.

Per confermare quali fonti si sono combinate su una macchina, esegui `/status` e [leggi la riga `Setting sources`](/docs/it/managed-settings#read-the-source-in-/status).

<h3 id="parentsettingsbehavior">
  `parentSettingsBehavior`
</h3>

Scegli se Claude Code applica le impostazioni gestite fornite da un processo host di incorporamento, come l'Agent SDK o un'estensione IDE, quando è presente anche un livello gestito distribuito dall'amministratore. Con `"first-wins"`, Claude Code elimina le impostazioni fornite dall'host; con `"merge"`, le applica sotto il livello amministrativo attraverso un filtro solo restrittivo. Imposta `"merge"` quando un host ha bisogno di passare le sue stesse restrizioni alle sessioni che avvia, ad esempio Claude Desktop che fornisce la lista di egresso consentita di un gateway.

* **Scope**: [`Managed`](#scopes). Claude Code lo legge dalla fonte gestita controllata dall'amministratore con la priorità più alta.
* **Type**: string, uno di:
  * `"first-wins"`: Claude Code elimina le impostazioni fornite dall'host quando è presente un livello gestito distribuito dall'amministratore
  * `"merge"`: Claude Code applica le impostazioni fornite dall'host sotto il livello amministrativo attraverso un filtro solo restrittivo
* **Default**: `"first-wins"`

```json managed-settings.json theme={null}
{
  "parentSettingsBehavior": "merge"
}
```

Questa chiave non ha effetto quando non esiste un livello gestito distribuito dall'amministratore: le impostazioni dell'host si applicano quindi come l'unico livello gestito, ancora filtrate a valori restrittivi. Per i limiti del filtro e come le fonti gestite interagiscono, vedere [Impostazioni padre da host di incorporamento](/docs/it/managed-settings#parent-settings-from-embedding-hosts) e [Limitare le impostazioni padre](/docs/it/claude-apps-gateway#restrict-parent-settings).

<span id="compute-managed-settings-with-a-policy-helper" />

<h3 id="policyhelper">
  `policyHelper`
</h3>

Esegui un eseguibile che distribuisci che calcola le impostazioni gestite all'avvio, in modo da poter derivare la politica dalla postura del dispositivo, dall'identità o da un servizio remoto invece di un file statico. Claude Code esegue l'helper prima di accettare il primo prompt e tratta le impostazioni che emette come le impostazioni gestite per la sessione.

* **Scope**: [`Managed`](#scopes). Leggi dal plist macOS, dal registro Windows HKLM o dal file delle impostazioni gestite. Claude Code legge la chiave dalla fonte gestita con la priorità più alta che contiene una [chiave di politica](/docs/it/managed-settings#how-claude-code-combines-managed-sources) e esegue l'helper solo quando quella fonte è una di queste tre; ignora la chiave nelle impostazioni gestite dal server, nel registro HKCU e nelle impostazioni padre fornite dall'host.
* **Type**: object con `path`, `timeoutMs` e `refreshIntervalMs`
* **Default**: non impostato, quindi nessun helper viene eseguito

Quando le impostazioni gestite dal server forniscono la politica all'avvio, hanno la precedenza sulla fonte dell'helper e l'helper non viene eseguito.

Se un recupero di impostazioni successivo segnala che le impostazioni gestite dal server sono state rimosse, Claude Code esegue l'helper a quel punto piuttosto che attendere il prossimo avvio. Il suo output governa il resto della sessione e un'esecuzione che non riesce termina la sessione con lo stesso messaggio di un'[esecuzione di avvio non riuscita](#helper-failures).

Questo esempio esegue l'helper con un timeout di 5 secondi e lo riesegue ogni cinque minuti:

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
  Scrivi l'output dell'helper
</h4>

Claude Code esegue l'helper senza argomenti, imposta `CLAUDE_CODE_VERSION` nel suo ambiente e legge un envelope JSON da stdout, limitato a 1 MiB.

Metti le impostazioni sotto una chiave `managedSettings`. Un oggetto di impostazioni bare senza una chiave `managedSettings` analizza con `managedSettings` non definito e non applica nulla, e Claude Code non segnala alcun errore:

```json theme={null}
{
  "managedSettings": {
    "permissions": { "deny": ["Read(//etc/secrets/**)"] }
  }
}
```

Quando l'helper emette `managedSettings`, quell'oggetto diventa l'unica fonte di impostazioni gestite per l'esecuzione: Claude Code ignora le fonti MDM, file e HKCU, legge le [chiavi tra fonti](/docs/it/managed-settings#keys-read-from-every-admin-source) solo dall'output dell'helper e non unisce mai le [impostazioni padre](/docs/it/managed-settings#parent-settings-from-embedding-hosts).

Il controllo `forceRemoteSettingsRefresh` all'avvio viene eseguito prima dell'helper e legge qualsiasi fonte amministrativa. Un helper che esce con `0` con un envelope che omette `managedSettings` non contribuisce con impostazioni gestite e le altre fonti si applicano come al solito.

<h4 id="helper-failures">
  Errori dell'helper
</h4>

Un'esecuzione dell'helper non riesce quando:

* `path` viola le regole in [`policyHelper.path`](#policyhelper-path).
* Nessun file regolare è in `path`. Claude Code controlla il file prima di avviare l'helper, entro lo stesso budget `timeoutMs`, quindi un mount di rete non responsivo può causare il fallimento dell'esecuzione.
* L'helper esce con un valore diverso da zero, è ancora in esecuzione quando `timeoutMs` trascorre, o non si avvia affatto, ad esempio perché non è eseguibile.
* L'helper scrive più di 1 MiB su stdout o su stderr.
* stdout non è un singolo oggetto JSON, o il suo `managedSettings` ha una [violazione dello schema che Claude Code non può riparare](/docs/it/managed-settings#find-entries-claude-code-dropped).

Quando l'esecuzione all'avvio non riesce, Claude Code stampa il motivo e rifiuta di avviarsi. Dopo un'uscita con valore diverso da zero, il motivo include stderr dell'helper, o il suo stdout quando stderr è vuoto. Dopo un timeout, il motivo nomina il limite `timeoutMs` e non include nessuno dell'output dell'helper. Il rifiuto copre le sessioni interattive, `claude -p`, le sessioni dell'Agent SDK, le [sessioni in background](/docs/it/agent-view) e la maggior parte dei sottocomandi.

Il rifiuto è deliberato, quindi un helper che ha bisogno di resilienza alle interruzioni dovrebbe servire dalla sua stessa cache e uscire con `0`.

Quando un aggiornamento in background non riesce, Claude Code mantiene l'ultima politica riuscita in vigore e `/status` mostra l'aggiornamento non riuscito con il suo motivo fino a quando un aggiornamento non riesce. Ogni aggiornamento viene eseguito secondo le stesse regole di `timeoutMs` e fallimento dell'esecuzione all'avvio.

Con `--debug`, Claude Code scrive stderr dell'helper da ogni esecuzione al [log di debug](/docs/it/debug-your-config).

Claude Code segnala un valore `policyHelper` non valido come una [voce eliminata](/docs/it/managed-settings#find-entries-claude-code-dropped) e avvia la sessione sulle impostazioni gestite rimanenti senza eseguire un helper. I valori non validi includono una stringa di percorso bare e un `timeoutMs` al di sotto del [suo minimo](#policyhelper-timeoutms).

Per disattivare un helper, rimuovi la chiave dalla fonte che la imposta.

<h3 id="policyhelper-path">
  `policyHelper.path`
</h3>

Nomina l'eseguibile dell'helper che Claude Code esegue. Per quello che accade quando il percorso viola le regole sottostanti, vedere [Errori dell'helper](#helper-failures).

* **Scope**: [`Managed`](#scopes). Leggi dal plist macOS, dal registro Windows HKLM o dal file delle impostazioni gestite, ovunque [`policyHelper`](#policyhelper) sia letto.
* **Type**: string, un percorso assoluto in forma normalizzata, senza segmenti `.` o `..`; su Windows, un percorso con lettera di unità o UNC che termina in `.exe`
* **Default**: nessuno; obbligatorio quando `policyHelper` è impostato

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

Imposta quanto tempo Claude Code attende l'helper prima di trattare l'esecuzione come non riuscita. Un'esecuzione scaduta non riesce allo stesso modo di un'uscita con valore diverso da zero, quindi all'avvio Claude Code rifiuta di avviarsi.

* **Scope**: [`Managed`](#scopes). Leggi dal plist macOS, dal registro Windows HKLM o dal file delle impostazioni gestite, ovunque [`policyHelper`](#policyhelper) sia letto.
* **Type**: integer, millisecondi, minimo `1000`
* **Default**: `10000`

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

Fai in modo che Claude Code riesegua l'helper in background su un intervallo in modo che i cambiamenti di politica raggiungano una sessione in esecuzione. Quando un aggiornamento ha successo, il suo output sostituisce le impostazioni gestite precedenti senza un riavvio; quando un aggiornamento non riesce, Claude Code mantiene la politica che ha già.

* **Scope**: [`Managed`](#scopes). Leggi dal plist macOS, dal registro Windows HKLM o dal file delle impostazioni gestite, ovunque [`policyHelper`](#policyhelper) sia letto.
* **Type**: integer, millisecondi: `0` per disabilitare l'aggiornamento, altrimenti almeno `60000`
* **Default**: non impostato, quindi Claude Code esegue l'helper una volta all'avvio

Questo esempio riesegue l'helper ogni cinque minuti:

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

Fai in modo che Claude Code su WSL legga le impostazioni gestite dalla catena di politica Windows, con HKLM e il file delle impostazioni gestite Windows che hanno priorità su `/etc/claude-code` e HKCU al di sotto. Mentre la catena è attiva, Claude Code legge `/etc/claude-code` solo quando nessun file di impostazioni gestite o drop-in sotto `C:\Program Files\ClaudeCode\` fornisce una [chiave di politica](/docs/it/managed-settings#how-claude-code-combines-managed-sources). Impostalo per estendere la politica che già distribuisci su Windows alle sessioni WSL sulla stessa macchina, in modo che seguano le stesse regole delle sessioni host. Claude Code lo onora solo quando impostato nella chiave del registro HKLM o in un file di impostazioni gestite o drop-in sotto `C:\Program Files\ClaudeCode\`, entrambi i quali richiedono l'amministratore Windows per scrivere.

* **Scope**: [`Managed`](#scopes). In una fonte Windows controllata dall'amministratore.
* **Type**: Boolean
  * `true`: Claude Code su WSL legge le impostazioni gestite dalla catena di politica Windows e legge `/etc/claude-code` solo quando nessun file di impostazioni gestite o drop-in sotto `C:\Program Files\ClaudeCode\` fornisce una [chiave di politica](/docs/it/managed-settings#how-claude-code-combines-managed-sources)
  * `false`: WSL legge solo `/etc/claude-code`
* **Default**: `false`, quindi WSL legge solo `/etc/claude-code`

```json managed-settings.json theme={null}
{
  "wslInheritsWindowsSettings": true
}
```

Una volta che una fonte amministrativa attiva la catena, la politica HKCU si unisce ad essa su WSL solo quando HKCU imposta anche la chiave su `true`. Quella copia non attiva la catena da sola. Una fonte Windows che contiene solo questa chiave non conta come fonte di politica, quindi una fonte con priorità inferiore fornisce comunque la politica. Questa chiave non ha effetto su Windows nativo.

<h2 id="global-config-settings">
  Impostazioni di configurazione globale
</h2>

Salvate queste chiavi in `~/.claude.json`, non in un file di impostazioni. Claude Code le ignora ovunque altrove. Claude Code e `/config` le scrivono per voi nella maggior parte dei casi, e potete anche modificarle manualmente.

<h3 id="autoconnectide">
  `autoConnectIde`
</h3>

Connettiti a un IDE in esecuzione automaticamente quando avvii Claude Code da un terminale esterno. Appare in `/config` come **Auto-connect to IDE (external terminal)** quando esegui Claude Code al di fuori di un terminale VS Code o JetBrains.

* **Scope**: [`Global config`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code si connette a un IDE in esecuzione automaticamente quando lo avvii da un terminale esterno
  * `false`: Claude Code non si connette automaticamente da un terminale esterno; all'interno di un terminale VS Code o JetBrains, o con `--ide`, si connette comunque
* **Default**: `false`
* **Per-session overrides**: [`CLAUDE_CODE_AUTO_CONNECT_IDE`](/docs/it/env-vars) ha la precedenza su questa chiave per una sessione, in entrambe le direzioni

```json ~/.claude.json theme={null}
{
  "autoConnectIde": true
}
```

Claude Code ignora questa chiave in `settings.json`.

<h3 id="autoinstallideextension">
  `autoInstallIdeExtension`
</h3>

Installa l'estensione IDE di Claude Code automaticamente quando esegui Claude Code da un terminale VS Code. Appare in `/config` come **Auto-install IDE extension** quando esegui Claude Code all'interno di un terminale VS Code o JetBrains.

* **Scope**: [`Global config`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code installa l'estensione IDE automaticamente quando lo esegui da un terminale VS Code
  * `false`: Claude Code non installa l'estensione automaticamente
* **Default**: `true`
* **Per-session overrides**: [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](/docs/it/env-vars) impostato su `1` salta l'installazione per una sessione anche quando questa chiave è `true`

```json ~/.claude.json theme={null}
{
  "autoInstallIdeExtension": false
}
```

Claude Code ignora questa chiave in `settings.json`.

<h3 id="copyonselect">
  `copyOnSelect`
</h3>

Copia il testo negli appunti automaticamente quando finisci di selezionarlo con il mouse nel [rendering a schermo intero](/docs/it/fullscreen#use-the-mouse) o nella [vista agente](/docs/it/agent-view). Appare in `/config` come **Copy on select** mentre il rendering a schermo intero è attivo.

* **Scope**: [`Global config`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code copia il testo negli appunti quando finisci di selezionarlo
  * `false`: la selezione del testo lascia gli appunti invariati, e [copi la selezione con una scorciatoia da tastiera](/docs/it/fullscreen#use-the-mouse) invece
* **Default**: `true`

```json ~/.claude.json theme={null}
{
  "copyOnSelect": false
}
```

Claude Code ignora questa chiave in `settings.json`.

<h3 id="difftool">
  `diffTool`
</h3>

Scegli dove Claude Code mostra il diff di una modifica `Edit` o `Write` che propone quando un IDE [VS Code](/docs/it/vs-code) o [JetBrains](/docs/it/jetbrains#features) è connesso: `"auto"` lo apre nel visualizzatore diff dell'IDE, `"terminal"` lo mantiene nel terminale. Appare in `/config` come **Diff tool** solo mentre Claude Code è connesso a un IDE VS Code o JetBrains.

* **Scope**: [`Global config`](#scopes)
* **Type**: string, uno di:
  * `"auto"`: Claude Code apre il diff nel visualizzatore diff dell'IDE quando un IDE VS Code o JetBrains è connesso
  * `"terminal"`: Claude Code mantiene il diff nel terminale
* **Default**: `"auto"`

```json ~/.claude.json theme={null}
{
  "diffTool": "terminal"
}
```

Claude Code ignora questa chiave in `settings.json`.

<h3 id="externaleditorcontext">
  `externalEditorContext`
</h3>

Quando premi `Ctrl+G`, Claude Code apre il prompt che stai digitando nel tuo [editor esterno](/docs/it/interactive-mode#general-controls). Con questa chiave attivata, il buffer dell'editor inizia con la risposta precedente di Claude come righe di commento `#`, così puoi leggerla mentre scrivi, e Claude Code rimuove quelle righe quando salvi. Appare in `/config` come **Show last response in external editor**.

* **Scope**: [`Global config`](#scopes)
* **Type**: Boolean
  * `true`: il buffer dell'editor inizia con la risposta precedente di Claude come righe di commento `#`, che Claude Code rimuove quando salvi
  * `false`: il buffer dell'editor si apre solo con il tuo prompt
* **Default**: `false`

```json ~/.claude.json theme={null}
{
  "externalEditorContext": true
}
```

Con questa opzione attivata, il buffer che Claude Code apre assomiglia a questo, e solo il testo sotto la riga del marcatore viene inviato come tuo prompt:

```text theme={null}
# ─── Claude's last response (for reference; removed on save) ───
# I added the retry loop to fetchUser in src/api.ts and a test
# for the timeout case. Want me to wire the same retry into
# fetchOrders?
# ─── Write your reply below this line ──────────────────────────

Yes, and cap it at three attempts.
```

Claude Code mantiene le ultime 50 righe della risposta e contrassegna il taglio con `# … (earlier output truncated)`.

Claude Code ignora questa chiave in `settings.json`.

<h3 id="permissionexplainerenabled">
  `permissionExplainerEnabled`
</h3>

<Warning>
  Rimosso nella v2.1.257, insieme al comando di spiegazione `Ctrl+E` sui prompt di autorizzazione Bash e PowerShell. Impostarlo non ha alcun effetto sulle versioni attuali.
</Warning>

Fino alla v2.1.256, potevi premere `Ctrl+E` su un prompt di autorizzazione Bash o PowerShell per vedere una spiegazione generata dal modello del comando, e impostare questa chiave su `false` per disattivare quella scorciatoia.

* **Scope**: [`Global config`](#scopes). Sulla v2.1.256 e precedenti.
* **Type**: Boolean
* **Default**: `true`

<h3 id="teammatedefaultmodel">
  `teammateDefaultModel`
</h3>

<Warning>
  Rimosso nella v2.1.234, insieme alla sua riga `/config` **Default teammate model**. Impostarlo non ha alcun effetto sulle versioni attuali.
</Warning>

Fino alla v2.1.233, impostavi questa chiave sul modello per i compagni di squadra del [team agente](/docs/it/agent-teams#specify-teammates-and-models) che il tuo prompt non ha nominato un modello per: un alias come `"sonnet"`, o `null` per seguire il modello del lead. Per il modello che Claude Code sceglie per tali compagni di squadra ora, vedi [specifica compagni di squadra e modelli](/docs/it/agent-teams#specify-teammates-and-models).

* **Scope**: [`Global config`](#scopes). Sulla v2.1.233 e precedenti.
* **Type**: string, un alias di modello o ID modello completo, o `null`
* **Default**: unset

<h2 id="see-also">
  Vedi anche
</h2>

* [Configurare i permessi](/docs/it/permissions): sintassi delle regole, modalità di permesso e trust dell'area di lavoro
* [Variabili di ambiente](/docs/it/env-vars): ogni variabile `CLAUDE_*`, `ANTHROPIC_*` e di provider che Claude Code legge
* [Strumenti disponibili per Claude](/docs/it/tools-reference): gli strumenti integrati e quali richiedono approvazione
* [File di impostazioni di esempio](/docs/it/settings-example): un file personale, un file di team e un file gestito da un'organizzazione
* [Configurare le impostazioni gestite](/docs/it/admin-setup): come le organizzazioni decidono cosa applicare
* [Distribuire le impostazioni gestite](/docs/it/managed-settings): meccanismi di consegna, precedenza all'interno del livello gestito e voci non valide nelle impostazioni gestite
* [Eseguire il debug della configurazione](/docs/it/debug-your-config): `claude doctor` e la finestra di dialogo Errore impostazioni
