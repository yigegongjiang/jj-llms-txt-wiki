> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Todas as configurações

> Referência completa para cada chave settings.json do Claude Code: onde cada uma vai, seu tipo e padrão, e um exemplo pronto para colar, com um índice de cada chave.

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

<BackToIndex href="#all-settings" label="Voltar ao índice" />

Esta página de referência lista cada chave que Claude Code lê de um arquivo de configurações, além do [pequeno grupo de chaves](#global-config-settings) que mantém em `~/.claude.json`. Para escolher um arquivo ou verificar a precedência, comece com [Arquivos de configurações e precedência](/docs/pt/settings).

<span id="available-settings" />

<span id="scopes" />

<span id="all-settings" />

<h2 id="settings-index">
  Índice de configurações
</h2>

Cada chave abaixo está vinculada à sua entrada. O escopo lista os [arquivos](/docs/pt/settings#settings-files-and-who-they-affect) em que pode estar: `User` é `~/.claude/settings.json`, `Project` é `.claude/settings.json`, `Local` é `.claude/settings.local.json`, e `Managed` é [o que sua organização implanta](/docs/pt/managed-settings). `Any file` significa todos os quatro, e `Global config` significa [`~/.claude.json`](#global-config-settings).

<ReferenceFilter
  noun="settings"
  placeholder="Filter settings by key or purpose"
  facetOrder={{ scope: ["Any file", "User, local, or managed", "User or managed", "Managed", "Global config"] }}
  columnHelp={{
topic: "The section of this page that holds the entry. Use Sort by to group the table by topic.",
scope: "Which settings files can set the key: user (~/.claude/settings.json), project (.claude/settings.json), local (.claude/settings.local.json), or managed (deployed by your organization). Global config keys are in ~/.claude.json instead.",
}}
/>

| Chave                                                                                                 | Descrição                                                                                                                                                                                                                                                     | Tópico                                   | Escopo                  |
| :---------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------- | :---------------------- |
| [`advisorModel`](#advisormodel)                                                                       | Escolha qual modelo responde quando Claude usa a [ferramenta de assessor](/docs/pt/advisor)                                                                                                                                                                        | Modelo e respostas                       | Any file                |
| [`agent`](#agent)                                                                                     | Inicie cada sessão como um [subagente](/docs/pt/sub-agents) nomeado com seu prompt, ferramentas e modelo                                                                                                                                                           | Agentes, sessões e worktrees             | Any file                |
| [`agentPushNotifEnabled`](#agentpushnotifenabled)                                                     | Permita que Claude envie uma [notificação push para seu telefone](/docs/pt/remote-control#mobile-push-notifications) quando decidir                                                                                                                                | Remoto, desktop e notificações           | Any file                |
| [`allowAllClaudeAiMcps`](#allowallclaudeaimcps)                                                       | Carregue os [conectores claude.ai](/docs/pt/mcp) que Claude Code busca por conta própria junto com um [`managed-mcp.json`](/docs/pt/managed-mcp#exclusive-control-with-managed-mcp-json) implantado                                                                     | MCP                                      | Managed                 |
| [`allowedChannelPlugins`](#allowedchannelplugins)                                                     | Substitua a lista de permissões padrão de [plugins de canal](/docs/pt/channels#restrict-which-channel-plugins-can-run) que podem enviar mensagens                                                                                                                  | Plugins e skills                         | Managed                 |
| [`allowedHttpHookUrls`](#allowedhttphookurls)                                                         | Limite quais URLs os [hooks HTTP](/docs/pt/hooks) podem atingir                                                                                                                                                                                                    | Hooks e automação                        | Any file                |
| [`allowedMcpServers`](#allowedmcpservers)                                                             | Lista de permissões de quais [servidores MCP](/docs/pt/mcp) os usuários podem adicionar                                                                                                                                                                            | MCP                                      | Any file                |
| [`allowManagedHooksOnly`](#allowmanagedhooksonly)                                                     | Execute apenas os [hooks](/docs/pt/hooks) que sua organização implanta                                                                                                                                                                                             | Hooks e automação                        | Managed                 |
| [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly)                                           | Faça a lista de permissões de [MCP](/docs/pt/mcp) gerenciada ser a única que se aplica                                                                                                                                                                             | MCP                                      | Managed                 |
| [`allowManagedPermissionRulesOnly`](#allowmanagedpermissionrulesonly)                                 | Faça as [configurações gerenciadas](/docs/pt/managed-settings) serem a única fonte de [regras de permissão](/docs/pt/permissions#managed-settings)                                                                                                                      | Configurações de permissão               | Managed                 |
| [`alwaysThinkingEnabled`](#alwaysthinkingenabled)                                                     | Desative o [pensamento estendido](/docs/pt/model-config#extended-thinking) para cada sessão                                                                                                                                                                        | Modelo e respostas                       | Any file                |
| [`apiKeyHelper`](#apikeyhelper)                                                                       | Gere a [credencial de API](/docs/pt/authentication#credential-management) com seu próprio comando                                                                                                                                                                  | Autenticação e provedores                | Any file                |
| [`askUserQuestionTimeout`](#askuserquestiontimeout)                                                   | Permita que uma pergunta sem resposta [continue automaticamente](/docs/pt/tools-reference#question-auto-continue-timeout) após tempo ocioso                                                                                                                        | Interface e terminal                     | User or managed         |
| [`attribution`](#attribution)                                                                         | Personalize a atribuição que Claude Code adiciona a commits e pull requests                                                                                                                                                                                   | Git e atribuição                         | Any file                |
| [`attribution.commit`](#attribution-commit)                                                           | Altere ou oculte o trailer que Claude Code adiciona aos commits                                                                                                                                                                                               | Git e atribuição                         | Any file                |
| [`attribution.pr`](#attribution-pr)                                                                   | Altere ou oculte a linha de atribuição nas descrições de pull request                                                                                                                                                                                         | Git e atribuição                         | Any file                |
| [`attribution.sessionUrl`](#attribution-sessionurl)                                                   | Omita o link de sessão claude.ai dos commits de [nuvem](/docs/pt/claude-code-on-the-web) e [Remote Control](/docs/pt/remote-control)                                                                                                                                    | Git e atribuição                         | Any file                |
| [`autoCompactEnabled`](#autocompactenabled)                                                           | Desative ou ative a [compactação automática](/docs/pt/context-window)                                                                                                                                                                                              | Memória e contexto                       | Any file                |
| [`autoCompactWindow`](#autocompactwindow)                                                             | Defina o quão cheio o contexto fica antes de Claude Code [compactar](/docs/pt/context-window)                                                                                                                                                                      | Memória e contexto                       | Any file                |
| [`autoConnectIde`](#autoconnectide)                                                                   | Conecte-se automaticamente a um [VS Code](/docs/pt/vs-code) ou [JetBrains](/docs/pt/jetbrains#from-external-terminals) IDE em execução a partir de um terminal externo                                                                                                  | Configurações de config global           | Global config           |
| [`autoContinueAtUsageLimit`](#autocontinueatusagelimit)                                               | Aguarde na sessão aberta e [continue a tarefa automaticamente](/docs/pt/interactive-mode#wait-for-a-usage-limit-to-reset) após um limite de uso claude.ai ser redefinido                                                                                           | Interface e terminal                     | User or managed         |
| [`autoInstallIdeExtension`](#autoinstallideextension)                                                 | Desative a instalação automática da [extensão IDE](/docs/pt/vs-code#install-the-extension) a partir de um terminal VS Code                                                                                                                                         | Configurações de config global           | Global config           |
| [`autoMemoryDirectory`](#automemorydirectory)                                                         | Armazene a [memória automática](/docs/pt/memory#auto-memory) em um diretório de sua escolha                                                                                                                                                                        | Memória e contexto                       | Any file                |
| [`autoMemoryEnabled`](#automemoryenabled)                                                             | Desative ou ative a [memória automática](/docs/pt/memory#auto-memory)                                                                                                                                                                                              | Memória e contexto                       | Any file                |
| [`autoMode`](#automode)                                                                               | Adicione suas próprias regras de permissão e negação ao classificador de [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode)                                                                                                             | Configurações de permissão               | User or managed         |
| [`autoMode.classifyAllShell`](#automode-classifyallshell)                                             | Envie cada comando shell através do [classificador de modo automático](/docs/pt/permission-modes#what-the-classifier-blocks-by-default), mesmo aqueles que uma regra de permissão estreita corresponde                                                             | Configurações de permissão               | User or managed         |
| [`autoScrollEnabled`](#autoscrollenabled)                                                             | [Siga a nova saída](/docs/pt/fullscreen#auto-follow) até o final na renderização em tela cheia                                                                                                                                                                     | Interface e terminal                     | Any file                |
| [`autoUpdatesChannel`](#autoupdateschannel)                                                           | Siga o [canal de lançamento](/docs/pt/setup#configure-release-channel) estável em vez do mais recente                                                                                                                                                              | Atualizações e versionamento             | Any file                |
| [`availableModels`](#availablemodels)                                                                 | [Restrinja quais modelos](/docs/pt/model-config#restrict-model-selection) as pessoas podem escolher                                                                                                                                                                | Modelo e respostas                       | Any file                |
| [`awaySummaryEnabled`](#awaysummaryenabled)                                                           | Desative o [resumo da sessão](/docs/pt/interactive-mode#session-recap) mostrado quando você volta ao terminal                                                                                                                                                      | Remoto, desktop e notificações           | Any file                |
| [`awsAuthRefresh`](#awsauthrefresh)                                                                   | Atualize as [credenciais Bedrock](/docs/pt/amazon-bedrock#advanced-credential-configuration) expiradas em `.aws` com seu próprio comando                                                                                                                           | Autenticação e provedores                | Any file                |
| [`awsCredentialExport`](#awscredentialexport)                                                         | Forneça as [credenciais Bedrock](/docs/pt/amazon-bedrock#advanced-credential-configuration) como JSON a partir de seu próprio comando                                                                                                                              | Autenticação e provedores                | Any file                |
| [`axScreenReader`](#axscreenreader)                                                                   | Renderize a [saída amigável ao leitor de tela](/docs/pt/accessibility)                                                                                                                                                                                             | Interface e terminal                     | Any file                |
| [`bashEditDiffEnabled`](#basheditdiffenabled)                                                         | Registre os [arquivos que um comando Bash alterou](/docs/pt/hooks#bash) em cada modo de permissão                                                                                                                                                                  | Interface e terminal                     | User or managed         |
| [`bashOutputMaxChars`](#bashoutputmaxchars)                                                           | Defina quanto da [saída](/docs/pt/tools-reference#output-limits) de um comando bem-sucedido Claude recebe inline                                                                                                                                                   | Memória e contexto                       | Any file                |
| [`blockedMarketplaces`](#blockedmarketplaces)                                                         | Bloqueie as fontes do [marketplace de plugins](/docs/pt/plugins/overview) para sua organização                                                                                                                                                                     | Plugins e skills                         | Managed                 |
| [`browserExternalPageTools`](#browserexternalpagetools)                                               | Mantenha as ferramentas de Claude desativadas em páginas externas no painel [desktop](/docs/pt/desktop) Browser                                                                                                                                                    | Ferramentas                              | Managed                 |
| [`channelsEnabled`](#channelsenabled)                                                                 | Permita [canais](/docs/pt/channels#enable-channels-for-your-organization) para sua organização                                                                                                                                                                     | Plugins e skills                         | Managed                 |
| [`claudeMd`](#claudemd)                                                                               | Injete instruções [CLAUDE.md](/docs/pt/memory#deploy-organization-wide-claude-md) em toda a organização a partir de configurações gerenciadas                                                                                                                      | Memória e contexto                       | Managed                 |
| [`claudeMdExcludes`](#claudemdexcludes)                                                               | Pule arquivos [CLAUDE.md](/docs/pt/memory#exclude-specific-claude-md-files) específicos quando a memória carrega                                                                                                                                                   | Memória e contexto                       | Any file                |
| [`cleanupPeriodDays`](#cleanupperioddays)                                                             | Escolha quantos dias Claude Code mantém [transcrições](/docs/pt/data-usage#data-retention) antes de deletá-las                                                                                                                                                     | Privacidade e telemetria                 | Any file                |
| [`companyAnnouncements`](#companyannouncements)                                                       | Mostre os anúncios de sua organização na inicialização                                                                                                                                                                                                        | Interface e terminal                     | Any file                |
| [`copyOnSelect`](#copyonselect)                                                                       | Desative a cópia automática de texto que você seleciona com o mouse na [renderização em tela cheia](/docs/pt/fullscreen#use-the-mouse) e visualização de agente                                                                                                    | Configurações de config global           | Global config           |
| [`crossSessionInbound`](#crosssessioninbound)                                                         | Escolha se Claude Code entrega [mensagens de suas outras sessões](/docs/pt/cross-session-messaging#control-inbound-messages), mostra um aviso sem entregá-las, ou as recusa                                                                                        | Agentes, sessões e worktrees             | Any file                |
| [`defaultShell`](#defaultshell)                                                                       | Escolha se Bash ou PowerShell executa os comandos shell que você digita com o prefixo [`!`](/docs/pt/interactive-mode#shell-mode-with-prefix)                                                                                                                      | Interface e terminal                     | Any file                |
| [`deniedMcpServers`](#deniedmcpservers)                                                               | Bloqueie [servidores MCP](/docs/pt/mcp) específicos por URL, comando ou nome                                                                                                                                                                                       | MCP                                      | Any file                |
| [`desktopSessionCleanupPeriodDays`](#desktopsessioncleanupperioddays)                                 | Defina um limite de idade em dias para [transcrições de Claude Desktop e Cowork](/docs/pt/claude-directory#cleaned-up-automatically)                                                                                                                               | Privacidade e telemetria                 | User or managed         |
| [`dialogExpiry`](#dialogexpiry)                                                                       | Defina quanto tempo Claude Code aguarda [Remote Control](/docs/pt/remote-control) ou um host SDK responder a um diálogo encaminhado antes de cancelar o diálogo                                                                                                    | Interface e terminal                     | User or managed         |
| [`diffTool`](#difftool)                                                                               | Escolha se as alterações de arquivo propostas por Claude abrem no visualizador de diff do [VS Code](/docs/pt/vs-code) ou [JetBrains](/docs/pt/jetbrains#features) ou permanecem no terminal                                                                             | Configurações de config global           | Global config           |
| [`disableAgentView`](#disableagentview)                                                               | Desative agentes em segundo plano e [visualização de agente](/docs/pt/agent-view)                                                                                                                                                                                  | Agentes, sessões e worktrees             | Any file                |
| [`disableAllHooks`](#disableallhooks)                                                                 | Desative [hooks](/docs/pt/hooks), uma [linha de status](/docs/pt/statusline) personalizada, e um comando de [sugestão de arquivo `@`](/docs/pt/interactive-mode#quick-commands) personalizado de uma vez                                                                     | Hooks e automação                        | Any file                |
| [`disableArtifact`](#disableartifact)                                                                 | Descontinuado; use `enableArtifact` para desativar a [ferramenta Artifact](/docs/pt/artifacts)                                                                                                                                                                     | Remoto, desktop e notificações           | Any file                |
| [`disableAutoMode`](#disableautomode)                                                                 | Remova o [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) do ciclo de modo de permissão                                                                                                                                               | Configurações de permissão               | Any file                |
| [`disableBrowserExternalNavigation`](#disablebrowserexternalnavigation)                               | Limite o painel [desktop](/docs/pt/desktop) Browser para localhost para pessoas e Claude                                                                                                                                                                           | Ferramentas                              | Managed                 |
| [`disableBundledSkills`](#disablebundledskills)                                                       | Desative as [skills](/docs/pt/skills#bundled-skills) e [workflows](/docs/pt/workflows) inclusos com Claude Code                                                                                                                                                         | Plugins e skills                         | Any file                |
| [`disableClaudeAiConnectors`](#disableclaudeaiconnectors)                                             | Desative os [conectores claude.ai](/docs/pt/mcp#disable-claude-ai-connectors) para que Claude Code não os busque                                                                                                                                                   | MCP                                      | Any file                |
| [`disableCommandPluginSources`](#disablecommandpluginsources)                                         | Bloqueie [plugins](/docs/pt/plugins/overview) que instalam executando um comando declarado pelo marketplace                                                                                                                                                        | Plugins e skills                         | Managed                 |
| [`disableDeepLinkRegistration`](#disabledeeplinkregistration)                                         | Impeça Claude Code de registrar o manipulador [`claude-cli://`](/docs/pt/deep-links)                                                                                                                                                                               | Remoto, desktop e notificações           | Any file                |
| [`disableDesktopLocalSessions`](#disabledesktoplocalsessions)                                         | Desative as [sessões Desktop Code](/docs/pt/desktop#local-sessions-on-managed-devices) que executam no dispositivo, deixando SSH para outros hosts e nuvem                                                                                                         | Remoto, desktop e notificações           | Managed                 |
| [`disabledMcpjsonServers`](#disabledmcpjsonservers)                                                   | Rejeite servidores específicos do [`.mcp.json`](/docs/pt/mcp#project-scope) de um projeto                                                                                                                                                                          | MCP                                      | Any file                |
| [`disableMobileSimulatorTools`](#disablemobilesimulatortools)                                         | Bloqueie as ferramentas de Claude no painel [desktop](/docs/pt/desktop) iOS Simulator                                                                                                                                                                              | Ferramentas                              | Managed                 |
| [`disableRemoteControl`](#disableremotecontrol)                                                       | Desative o [Remote Control](/docs/pt/remote-control) em todos os lugares onde pode começar                                                                                                                                                                         | Remoto, desktop e notificações           | Any file                |
| [`disableSideloadFlags`](#disablesideloadflags)                                                       | Rejeite os sinalizadores CLI que carregam [plugins](/docs/pt/plugins/overview), [subagentes](/docs/pt/sub-agents) e [servidores MCP](/docs/pt/mcp)                                                                                                                           | Configurações empresariais e gerenciadas | Managed                 |
| [`disableSkillShellExecution`](#disableskillshellexecution)                                           | Impeça [skills](/docs/pt/skills) e comandos personalizados de executar shell inline                                                                                                                                                                                | Plugins e skills                         | Any file                |
| [`disableWorkflows`](#disableworkflows)                                                               | Desative [workflows dinâmicos](/docs/pt/workflows) para todos; use `enableWorkflows` para você mesmo                                                                                                                                                               | Hooks e automação                        | Any file                |
| [`editorMode`](#editormode)                                                                           | Use [atalhos de teclado vim](/docs/pt/interactive-mode#vim-editor-mode) no prompt de entrada                                                                                                                                                                       | Interface e terminal                     | Any file                |
| [`effortLevel`](#effortlevel)                                                                         | Defina um [nível de esforço](/docs/pt/model-config#adjust-effort-level) padrão para modelos sem um nível salvo próprio                                                                                                                                             | Modelo e respostas                       | Any file                |
| [`emojiCompletionEnabled`](#emojicompletionenabled)                                                   | Desative as [sugestões e substituição de emoji `:shortcode:`](/docs/pt/interactive-mode#emoji-shortcodes) na entrada do prompt                                                                                                                                     | Interface e terminal                     | Any file                |
| [`enableAllProjectMcpServers`](#enableallprojectmcpservers)                                           | Aprove cada servidor no arquivo [`.mcp.json`](/docs/pt/mcp#project-server-approvals-and-workspace-trust) do projeto sem um prompt                                                                                                                                  | MCP                                      | Any file                |
| [`enableArtifact`](#enableartifact)                                                                   | Desative a [ferramenta Artifact](/docs/pt/artifacts) com um `false` em qualquer arquivo; nenhum arquivo pode ativá-la novamente                                                                                                                                    | Remoto, desktop e notificações           | Any file                |
| [`enabledMcpjsonServers`](#enabledmcpjsonservers)                                                     | Aprove servidores específicos do [`.mcp.json`](/docs/pt/mcp#project-server-approvals-and-workspace-trust) de um projeto                                                                                                                                            | MCP                                      | Any file                |
| [`enabledPlugins`](#enabledplugins)                                                                   | Ative ou desative [plugins](/docs/pt/plugins/overview) individuais por escopo                                                                                                                                                                                      | Plugins e skills                         | Any file                |
| [`enableWorkflows`](#enableworkflows)                                                                 | Ative ou desative [workflows dinâmicos](/docs/pt/workflows) contra o padrão de seu plano                                                                                                                                                                           | Hooks e automação                        | Any file                |
| [`enforceAvailableModels`](#enforceavailablemodels)                                                   | Mantenha a [escolha Padrão de `/model`](/docs/pt/model-config#enforce-the-allowlist-for-the-default-model) dentro de sua lista de permissões `availableModels`                                                                                                     | Modelo e respostas                       | Any file                |
| [`env`](#env)                                                                                         | Defina [variáveis de ambiente](/docs/pt/env-vars#in-settings-files) para cada sessão e seus subprocessos                                                                                                                                                           | Memória e contexto                       | Any file                |
| [`externalEditorContext`](#externaleditorcontext)                                                     | Mostre a última resposta de Claude como comentários quando você pressiona [Ctrl+G](/docs/pt/interactive-mode#general-controls) para editar                                                                                                                         | Configurações de config global           | Global config           |
| [`extraKnownMarketplaces`](#extraknownmarketplaces)                                                   | Registre [marketplaces](/docs/pt/plugins/overview) para um repositório ou uma organização                                                                                                                                                                          | Plugins e skills                         | Any file                |
| [`fallbackModel`](#fallbackmodel)                                                                     | Nomeie [modelos de backup](/docs/pt/model-config#fallback-model-chains) para quando o primário estiver sobrecarregado                                                                                                                                              | Modelo e respostas                       | Any file                |
| [`fastMode`](#fastmode)                                                                               | Ative o [modo rápido](/docs/pt/fast-mode) para sessões onde está disponível                                                                                                                                                                                        | Modelo e respostas                       | Any file                |
| [`fastModePerSessionOptIn`](#fastmodepersessionoptin)                                                 | Exija que as pessoas ativem o [modo rápido](/docs/pt/fast-mode) em cada sessão                                                                                                                                                                                     | Modelo e respostas                       | Any file                |
| [`feedbackDrafts`](#feedbackdrafts)                                                                   | Controle se Claude enfileira [rascunhos de feedback](/docs/pt/tools-reference#sendfeedback-tool-behavior) para você revisar                                                                                                                                        | Privacidade e telemetria                 | User or managed         |
| [`feedbackSurveyRate`](#feedbacksurveyrate)                                                           | Altere a frequência com que a [pesquisa de qualidade da sessão](/docs/pt/data-usage#session-quality-surveys) aparece                                                                                                                                               | Privacidade e telemetria                 | Any file                |
| [`fileCheckpointingEnabled`](#filecheckpointingenabled)                                               | Desative ou ative os snapshots de arquivo que [`/rewind`](/docs/pt/checkpointing) restaura                                                                                                                                                                         | Memória e contexto                       | Any file                |
| [`fileSuggestion`](#filesuggestion)                                                                   | Forneça o [preenchimento automático de arquivo `@`](/docs/pt/interactive-mode#quick-commands) a partir de seu próprio comando                                                                                                                                      | Interface e terminal                     | Any file                |
| [`footerLinksRegexes`](#footerlinksregexes)                                                           | Transforme IDs de issue ou review na saída em [links clicáveis](/docs/pt/statusline#clickable-links) abaixo da caixa de entrada                                                                                                                                    | Interface e terminal                     | User or managed         |
| [`forceLoginGatewayUrl`](#forcelogingatewayurl)                                                       | Defina a [URL do gateway](/docs/pt/claude-apps-gateway#set-the-gateway-url) à qual a tela de login se conecta                                                                                                                                                      | Autenticação e provedores                | Managed                 |
| [`forceLoginMethod`](#forceloginmethod)                                                               | [Restrinja o login](/docs/pt/authentication#restrict-login-to-your-organization) a claude.ai, Claude Console, ou um [gateway de nuvem](/docs/pt/claude-apps-gateway)                                                                                                    | Autenticação e provedores                | Any file                |
| [`forceLoginOrgUUID`](#forceloginorguuid)                                                             | [Fixe os logins claude.ai à sua organização](/docs/pt/authentication#restrict-login-to-your-organization); apenas uma fonte gerenciada a impõe                                                                                                                     | Autenticação e provedores                | Any file                |
| [`forceRemoteSettingsRefresh`](#forceremotesettingsrefresh)                                           | Bloqueie a inicialização até que as [configurações gerenciadas por servidor](/docs/pt/server-managed-settings) sejam buscadas recentemente                                                                                                                         | Configurações empresariais e gerenciadas | Managed                 |
| [`gatewayInternalNetworks`](#gatewayinternalnetworks)                                                 | Permita que `/login` alcance um [gateway de nuvem](/docs/pt/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) no espaço IPv4 público que sua organização usa internamente                                                                       | Autenticação e provedores                | Managed                 |
| [`gcpAuthRefresh`](#gcpauthrefresh)                                                                   | Atualize as [credenciais do Google Cloud](/docs/pt/google-vertex-ai#advanced-credential-configuration) com seu próprio comando                                                                                                                                     | Autenticação e provedores                | Any file                |
| [`hooks`](#hooks)                                                                                     | Execute seus próprios comandos como [hooks](/docs/pt/hooks) em pontos do ciclo de vida de Claude Code                                                                                                                                                              | Hooks e automação                        | Any file                |
| [`httpHookAllowedEnvVars`](#httphookallowedenvvars)                                                   | Limite quais variáveis de ambiente os [hooks HTTP](/docs/pt/hooks) podem colocar em cabeçalhos                                                                                                                                                                     | Hooks e automação                        | Any file                |
| [`includeCoAuthoredBy`](#includecoauthoredby)                                                         | Descontinuado; use `attribution` para ocultar ou alterar a atribuição de commit e PR                                                                                                                                                                          | Git e atribuição                         | Any file                |
| [`includeGitInstructions`](#includegitinstructions)                                                   | Remova as instruções de commit e PR integradas do contexto de Claude                                                                                                                                                                                          | Git e atribuição                         | Any file                |
| [`inputNeededNotifEnabled`](#inputneedednotifenabled)                                                 | Receba uma [notificação push](/docs/pt/remote-control#mobile-push-notifications) quando Claude estiver esperando por você                                                                                                                                          | Remoto, desktop e notificações           | Any file                |
| [`isolatePeerMachines`](#isolatepeermachines)                                                         | Peça-lhe antes de Claude [enviar mensagem para uma de suas sessões em outra máquina](/docs/pt/cross-session-messaging#require-approval-for-cross-machine-messages)                                                                                                 | Agentes, sessões e worktrees             | Any file                |
| [`keybindingFlavor`](#keybindingflavor)                                                               | Descontinuado e sem efeito; os atalhos de edição de palavras sempre [seguem as convenções readline](/docs/pt/interactive-mode#make-ctrl-w-delete-back-to-whitespace)                                                                                               | Interface e terminal                     | Any file                |
| [`language`](#language)                                                                               | Faça Claude responder em um idioma diferente do inglês                                                                                                                                                                                                        | Modelo e respostas                       | Any file                |
| [`managedMcpServers`](#managedmcpservers)                                                             | Forneça [servidores MCP](/docs/pt/managed-mcp#provide-servers-through-managed-settings) remotos a cada usuário junto com os que eles adicionam                                                                                                                     | MCP                                      | Managed                 |
| [`managedSourcesBehavior`](#managedsourcesbehavior)                                                   | Componha cada [fonte gerenciada](/docs/pt/managed-settings#how-claude-code-combines-managed-sources) que você implanta em vez de usar apenas a de maior prioridade                                                                                                 | Configurações empresariais e gerenciadas | Managed                 |
| [`maxEffortLevel`](#maxeffortlevel)                                                                   | Limite o [nível de esforço](/docs/pt/model-config#adjust-effort-level) para cada modelo ou por modelo, em cada provedor                                                                                                                                            | Modelo e respostas                       | Any file                |
| [`minimumVersion`](#minimumversion)                                                                   | Mantenha as [atualizações automáticas](/docs/pt/setup#pin-a-minimum-version) de instalar qualquer coisa abaixo de uma versão                                                                                                                                       | Atualizações e versionamento             | Any file                |
| [`model`](#model)                                                                                     | Altere o [modelo](/docs/pt/model-config#set-a-default-model-for-new-sessions) com o qual Claude Code começa                                                                                                                                                        | Modelo e respostas                       | Any file                |
| [`modelOverrides`](#modeloverrides)                                                                   | [Mapeie IDs de modelo](/docs/pt/model-config#override-model-ids-per-version) para os IDs do seu provedor, como ARNs do Bedrock                                                                                                                                     | Modelo e respostas                       | Any file                |
| [`modelPicker`](#modelpicker)                                                                         | Escolha quais modelos o seletor [`/model`](/docs/pt/model-config#available-models) lista, em sua própria ordem e com seus próprios rótulos                                                                                                                         | Modelo e respostas                       | User or managed         |
| [`modelPricing`](#modelpricing)                                                                       | Relate gastos nas taxas contratadas de sua organização em vez do preço de tabela                                                                                                                                                                              | Modelo e respostas                       | Managed                 |
| [`modelSettings`](#modelsettings)                                                                     | Mantenha um [nível de esforço](/docs/pt/model-config#adjust-effort-level) salvo por modelo, ou limite o esforço de um modelo                                                                                                                                       | Modelo e respostas                       | Any file                |
| [`otelHeadersHelper`](#otelheadershelper)                                                             | Gere cabeçalhos [OpenTelemetry](/docs/pt/monitoring-usage#dynamic-headers) rotativos com seu próprio comando                                                                                                                                                       | Autenticação e provedores                | Any file                |
| [`outputStyle`](#outputstyle)                                                                         | Altere o papel, tom e formato de saída de Claude com um [estilo de saída](/docs/pt/output-styles)                                                                                                                                                                  | Modelo e respostas                       | Any file                |
| [`parentSettingsBehavior`](#parentsettingsbehavior)                                                   | Aplique ou descarte restrições que um [host SDK ou IDE](/docs/pt/managed-settings#let-an-embedding-host-add-policy) passa quando você implanta [configurações gerenciadas](/docs/pt/managed-settings)                                                                   | Configurações empresariais e gerenciadas | Managed                 |
| [`permissionExplainerEnabled`](#permissionexplainerenabled)                                           | Removido na v2.1.257, junto com a explicação do comando `Ctrl+E` nos prompts de permissão de shell                                                                                                                                                            | Configurações de config global           | Global config           |
| [`permissions`](#permissions)                                                                         | Defina regras de permissão, pergunta e negação e o [modo de permissão](/docs/pt/permission-modes) inicial                                                                                                                                                          | Configurações de permissão               | Any file                |
| [`permissions.additionalDirectories`](#permissions-additionaldirectories)                             | Dê a Claude acesso a arquivos em [diretórios fora do atual](/docs/pt/permissions#working-directories)                                                                                                                                                              | Configurações de permissão               | Any file                |
| [`permissions.allow`](#permissions-allow)                                                             | Aprove [usos de ferramentas](/docs/pt/permissions#permission-rule-syntax) listados sem um prompt                                                                                                                                                                   | Configurações de permissão               | Any file                |
| [`permissions.ask`](#permissions-ask)                                                                 | Sempre solicite antes de [usos de ferramentas](/docs/pt/permissions#permission-rule-syntax) listados                                                                                                                                                               | Configurações de permissão               | Any file                |
| [`permissions.blockReadsOutsideWorkingDirectories`](#permissions-blockreadsoutsideworkingdirectories) | Faça as ferramentas de arquivo recusarem leituras fora dos [diretórios de trabalho](/docs/pt/permissions#working-directories) em cada modo de permissão                                                                                                            | Configurações de permissão               | Any file                |
| [`permissions.defaultMode`](#permissions-defaultmode)                                                 | Defina o [modo de permissão](/docs/pt/permission-modes#which-mode-a-session-starts-in) em que novas sessões começam                                                                                                                                                | Configurações de permissão               | Any file                |
| [`permissions.deny`](#permissions-deny)                                                               | Bloqueie [usos de ferramentas](/docs/pt/permissions#permission-rule-syntax) listados, incluindo leituras de arquivos que contêm segredos                                                                                                                           | Configurações de permissão               | Any file                |
| [`permissions.disableBypassPermissionsMode`](#permissions-disablebypasspermissionsmode)               | Impeça qualquer pessoa de entrar no [modo bypassPermissions](/docs/pt/permission-modes#skip-all-checks-with-bypasspermissions-mode)                                                                                                                                | Configurações de permissão               | Any file                |
| [`plansDirectory`](#plansdirectory)                                                                   | Escolha onde o [modo de plano](/docs/pt/permission-modes#analyze-before-you-edit-with-plan-mode) escreve arquivos de plano                                                                                                                                         | Memória e contexto                       | Any file                |
| [`pluginConfigs`](#pluginconfigs)                                                                     | Armazene as respostas que você deu ao diálogo de configuração de um [plugin](/docs/pt/plugins/overview)                                                                                                                                                            | Plugins e skills                         | User or managed         |
| [`pluginSuggestionMarketplaces`](#pluginsuggestionmarketplaces)                                       | Escolha quais [marketplaces](/docs/pt/plugins/org#restrict-what-users-can-install) podem exibir sugestões de instalação de plugin em `/plugin`                                                                                                                     | Plugins e skills                         | Managed                 |
| [`pluginTrustMessage`](#plugintrustmessage)                                                           | Adicione seu próprio texto ao aviso de confiança de [plugin](/docs/pt/plugins/overview)                                                                                                                                                                            | Plugins e skills                         | Managed                 |
| [`policyHelper`](#policyhelper)                                                                       | Execute um executável que calcula [configurações gerenciadas](/docs/pt/managed-settings#compute-the-policy-with-a-helper-program) na inicialização                                                                                                                 | Configurações empresariais e gerenciadas | Managed                 |
| [`policyHelper.path`](#policyhelper-path)                                                             | Nomeie o [executável auxiliar](/docs/pt/managed-settings#compute-the-policy-with-a-helper-program) que Claude Code executa                                                                                                                                         | Configurações empresariais e gerenciadas | Managed                 |
| [`policyHelper.refreshIntervalMs`](#policyhelper-refreshintervalms)                                   | Execute novamente o [auxiliar](/docs/pt/managed-settings#compute-the-policy-with-a-helper-program) em segundo plano em um intervalo                                                                                                                                | Configurações empresariais e gerenciadas | Managed                 |
| [`policyHelper.timeoutMs`](#policyhelper-timeoutms)                                                   | Defina quanto tempo Claude Code aguarda o [auxiliar](/docs/pt/managed-settings#compute-the-policy-with-a-helper-program)                                                                                                                                           | Configurações empresariais e gerenciadas | Managed                 |
| [`preferredNotifChannel`](#preferrednotifchannel)                                                     | Escolha um [sino de terminal ou notificação de desktop](/docs/pt/terminal-config#get-a-terminal-bell-or-notification) para conclusão de tarefa                                                                                                                     | Remoto, desktop e notificações           | Any file                |
| [`prefersReducedMotion`](#prefersreducedmotion)                                                       | [Reduza ou desative](/docs/pt/accessibility#accessibility-settings) animações de spinner, shimmer e flash                                                                                                                                                          | Interface e terminal                     | Any file                |
| [`processWrapper`](#processwrapper)                                                                   | Execute os processos em segundo plano de Claude Code através de um [inicializador corporativo](/docs/pt/corporate-launcher) no macOS e Linux                                                                                                                       | Agentes, sessões e worktrees             | User or managed         |
| [`promptCacheTtl`](#promptcachettl)                                                                   | Escolha o [tempo de vida do cache de prompt](/docs/pt/prompt-caching#cache-lifetime) para a conversa principal                                                                                                                                                     | Modelo e respostas                       | Any file                |
| [`promptSuggestionEnabled`](#promptsuggestionenabled)                                                 | Oculte as [sugestões de prompt](/docs/pt/interactive-mode#prompt-suggestions) acinzentadas na caixa de entrada                                                                                                                                                     | Interface e terminal                     | Any file                |
| [`prUrlTemplate`](#prurltemplate)                                                                     | Aponte links de PR para uma ferramenta de revisão de código interna em vez de github.com                                                                                                                                                                      | Git e atribuição                         | Any file                |
| [`remote.defaultEnvironmentId`](#remote-defaultenvironmentid)                                         | Escolha o [ambiente de nuvem](/docs/pt/cloud-environments) padrão para `claude --cloud`; um ID `ccpool_` auto-hospedado é somente leitura a partir de configurações de usuário e gerenciadas e `--settings`                                                        | Remoto, desktop e notificações           | Any file                |
| [`remoteControlAtStartup`](#remotecontrolatstartup)                                                   | Conecte o [Remote Control](/docs/pt/remote-control#enable-remote-control-for-all-sessions) automaticamente quando uma sessão começar                                                                                                                               | Remoto, desktop e notificações           | Any file                |
| [`requiredMaximumVersion`](#requiredmaximumversion)                                                   | [Recuse-se a iniciar](/docs/pt/setup#pin-a-minimum-version) em uma versão mais recente do que sua organização permite                                                                                                                                              | Atualizações e versionamento             | Managed                 |
| [`requiredMinimumVersion`](#requiredminimumversion)                                                   | [Recuse-se a iniciar](/docs/pt/setup#pin-a-minimum-version) em uma versão mais antiga do que sua organização exige                                                                                                                                                 | Atualizações e versionamento             | Managed                 |
| [`respectGitignore`](#respectgitignore)                                                               | Mantenha arquivos ignorados pelo git fora do [seletor de arquivo `@`](/docs/pt/interactive-mode#quick-commands)                                                                                                                                                    | Interface e terminal                     | Any file                |
| [`respondToBashCommands`](#respondtobashcommands)                                                     | Impeça Claude de responder após um comando shell [`!`](/docs/pt/interactive-mode#shell-mode-with-prefix) ser executado                                                                                                                                             | Interface e terminal                     | Any file                |
| [`sandbox`](#sandbox)                                                                                 | [Isole comandos Bash](/docs/pt/sandboxing) do seu sistema de arquivos e rede no macOS, Linux e WSL2                                                                                                                                                                | Configurações de sandbox                 | Any file                |
| [`sandbox.allowAppleEvents`](#sandbox-allowappleevents)                                               | Permita que [comandos em sandbox](/docs/pt/sandboxing) enviem Apple Events no macOS                                                                                                                                                                                | Configurações de sandbox                 | User or managed         |
| [`sandbox.allowUnsandboxedCommands`](#sandbox-allowunsandboxedcommands)                               | Permita que Claude tente novamente um comando bloqueado fora do [sandbox](/docs/pt/sandboxing#the-unsandboxed-retry-escape-hatch), ou proíba-o                                                                                                                     | Configurações de sandbox                 | Any file                |
| [`sandbox.autoAllowBashIfSandboxed`](#sandbox-autoallowbashifsandboxed)                               | Execute [comandos em sandbox](/docs/pt/sandboxing#auto-allow-mode) sem um prompt de permissão                                                                                                                                                                      | Configurações de sandbox                 | Any file                |
| [`sandbox.bwrapPath`](#sandbox-bwrappath)                                                             | Aponte o [sandbox](/docs/pt/sandboxing) para um binário bubblewrap fora de `PATH`                                                                                                                                                                                  | Configurações de sandbox                 | Managed                 |
| [`sandbox.credentials`](#sandbox-credentials)                                                         | Oculte ou mascare arquivos de credenciais e variáveis dentro do [sandbox](/docs/pt/sandboxing#protect-credentials)                                                                                                                                                 | Configurações de sandbox                 | Any file                |
| [`sandbox.credentials.allowPlaintextInject`](#sandbox-credentials-allowplaintextinject)               | Permita que [credenciais mascaradas](/docs/pt/sandboxing#mask-credentials) alcancem serviços HTTP simples em redes de teste confiáveis                                                                                                                             | Configurações de sandbox                 | User or managed         |
| [`sandbox.credentials.awsPairs`](#sandbox-credentials-awspairs)                                       | Vincule variáveis de chave AWS com nomes personalizados em uma credencial para [re-assinatura](/docs/pt/sandboxing#re-sign-aws-requests)                                                                                                                           | Configurações de sandbox                 | User or managed         |
| [`sandbox.credentials.envVars`](#sandbox-credentials-envvars)                                         | Desdefina ou mascare uma variável de ambiente dentro do [sandbox](/docs/pt/sandboxing#mask-environment-variables)                                                                                                                                                  | Configurações de sandbox                 | Any file                |
| [`sandbox.credentials.files`](#sandbox-credentials-files)                                             | Bloqueie ou mascare leituras de um arquivo de credenciais dentro do [sandbox](/docs/pt/sandboxing#mask-credential-files)                                                                                                                                           | Configurações de sandbox                 | Any file                |
| [`sandbox.credentials.sigv4`](#sandbox-credentials-sigv4)                                             | Escolha se solicitações [SigV4A AWS](/docs/pt/sandboxing#re-sign-aws-requests) de streaming, pré-assinadas ou falham ou passam                                                                                                                                     | Configurações de sandbox                 | User or managed         |
| [`sandbox.enabled`](#sandbox-enabled)                                                                 | Ative o [sandboxing de Bash](/docs/pt/sandboxing#get-started) no macOS, Linux e WSL2                                                                                                                                                                               | Configurações de sandbox                 | Any file                |
| [`sandbox.enableWeakerNestedSandbox`](#sandbox-enableweakernestedsandbox)                             | Execute o [sandbox](/docs/pt/sandboxing) do Linux dentro de um contêiner sem privilégios                                                                                                                                                                           | Configurações de sandbox                 | Any file                |
| [`sandbox.enableWeakerNetworkIsolation`](#sandbox-enableweakernetworkisolation)                       | Permita que `gh`, `gcloud` e `terraform` verifiquem TLS atrás de um proxy MITM dentro do [sandbox](/docs/pt/sandboxing#troubleshooting) no macOS                                                                                                                   | Configurações de sandbox                 | Any file                |
| [`sandbox.excludedCommands`](#sandbox-excludedcommands)                                               | Nomeie comandos que Claude Code pode executar fora do [sandbox](/docs/pt/sandboxing)                                                                                                                                                                               | Configurações de sandbox                 | Any file                |
| [`sandbox.failIfUnavailable`](#sandbox-failifunavailable)                                             | Recuse-se a iniciar quando o [sandbox](/docs/pt/sandboxing) não puder, em vez de executar sem sandbox                                                                                                                                                              | Configurações de sandbox                 | Any file                |
| [`sandbox.filesystem`](#sandbox-filesystem)                                                           | Controle quais caminhos [comandos em sandbox](/docs/pt/sandboxing#filesystem-isolation) podem ler e escrever                                                                                                                                                       | Configurações de sandbox                 | Any file                |
| [`sandbox.filesystem.allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly)       | Impeça desenvolvedores de reabrir [caminhos de leitura que sua organização bloqueou](/docs/pt/sandboxing#keep-developers-from-widening-the-policy)                                                                                                                 | Configurações de sandbox                 | Managed                 |
| [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread)                                       | Reabra a leitura dentro de uma região que [`denyRead`](#sandbox-filesystem-denyread) bloqueia                                                                                                                                                                 | Configurações de sandbox                 | Any file                |
| [`sandbox.filesystem.allowWrite`](#sandbox-filesystem-allowwrite)                                     | Adicione caminhos que [comandos em sandbox](/docs/pt/sandboxing) podem escrever                                                                                                                                                                                    | Configurações de sandbox                 | Any file                |
| [`sandbox.filesystem.denyRead`](#sandbox-filesystem-denyread)                                         | Bloqueie [comandos em sandbox](/docs/pt/sandboxing) de ler caminhos específicos                                                                                                                                                                                    | Configurações de sandbox                 | Any file                |
| [`sandbox.filesystem.denyWrite`](#sandbox-filesystem-denywrite)                                       | Bloqueie [comandos em sandbox](/docs/pt/sandboxing) de escrever em caminhos específicos                                                                                                                                                                            | Configurações de sandbox                 | Any file                |
| [`sandbox.filesystem.disabled`](#sandbox-filesystem-disabled)                                         | [Desative o isolamento de sistema de arquivos](/docs/pt/sandboxing#disable-filesystem-isolation) mantendo o isolamento de rede                                                                                                                                     | Configurações de sandbox                 | User or managed         |
| [`sandbox.ignoreViolations`](#sandbox-ignoreviolations)                                               | Silencie relatórios de violação para caminhos que um comando deve sondar                                                                                                                                                                                      | Configurações de sandbox                 | Any file                |
| [`sandbox.network`](#sandbox-network)                                                                 | Controle quais hosts, portas e sockets [comandos em sandbox](/docs/pt/sandboxing#network-isolation) alcançam                                                                                                                                                       | Configurações de sandbox                 | Any file                |
| [`sandbox.network.allowAllUnixSockets`](#sandbox-network-allowallunixsockets)                         | Permita que [comandos em sandbox](/docs/pt/sandboxing) se conectem a cada socket Unix                                                                                                                                                                              | Configurações de sandbox                 | Any file                |
| [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains)                                   | Pré-permita domínios para que [comandos em sandbox](/docs/pt/sandboxing) não solicitem permissão para eles                                                                                                                                                         | Configurações de sandbox                 | Any file                |
| [`sandbox.network.allowLocalBinding`](#sandbox-network-allowlocalbinding)                             | Permita que [comandos em sandbox](/docs/pt/sandboxing) se vinculem a portas localhost no macOS                                                                                                                                                                     | Configurações de sandbox                 | Any file                |
| [`sandbox.network.allowMachLookup`](#sandbox-network-allowmachlookup)                                 | Permita que ferramentas macOS [em sandbox](/docs/pt/sandboxing) como o iOS Simulator ou Playwright alcancem seus serviços XPC                                                                                                                                      | Configurações de sandbox                 | Any file                |
| [`sandbox.network.allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly)                 | Bloqueie a lista de permissões de rede para [configurações gerenciadas](/docs/pt/sandboxing#keep-developers-from-widening-the-policy)                                                                                                                              | Configurações de sandbox                 | Managed                 |
| [`sandbox.network.allowUnixSockets`](#sandbox-network-allowunixsockets)                               | Liste caminhos de socket Unix que [comandos em sandbox](/docs/pt/sandboxing) podem usar no macOS                                                                                                                                                                   | Configurações de sandbox                 | Any file                |
| [`sandbox.network.deniedDomains`](#sandbox-network-denieddomains)                                     | Bloqueie domínios para [comandos em sandbox](/docs/pt/sandboxing), mesmo dentro de um curinga permitido                                                                                                                                                            | Configurações de sandbox                 | Any file                |
| [`sandbox.network.httpProxyPort`](#sandbox-network-httpproxyport)                                     | Roteie o tráfego HTTP do [sandbox](/docs/pt/sandboxing#custom-proxy-configuration) através de seu próprio proxy                                                                                                                                                    | Configurações de sandbox                 | Any file                |
| [`sandbox.network.socksProxyPort`](#sandbox-network-socksproxyport)                                   | Roteie o tráfego SOCKS do [sandbox](/docs/pt/sandboxing#custom-proxy-configuration) através de seu próprio proxy                                                                                                                                                   | Configurações de sandbox                 | Any file                |
| [`sandbox.network.strictAllowlist`](#sandbox-network-strictallowlist)                                 | Negue hosts fora da [lista de permissões](/docs/pt/sandboxing#network-isolation) em vez de solicitar                                                                                                                                                               | Configurações de sandbox                 | User or managed         |
| [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate)                                       | Faça o [sandbox](/docs/pt/sandboxing#network-isolation) fazer proxy terminar TLS para que possa ler solicitações HTTPS                                                                                                                                             | Configurações de sandbox                 | User or managed         |
| [`sandbox.ripgrep`](#sandbox-ripgrep)                                                                 | Use seu próprio binário ripgrep dentro do [sandbox](/docs/pt/sandboxing)                                                                                                                                                                                           | Configurações de sandbox                 | User or managed         |
| [`sandbox.socatPath`](#sandbox-socatpath)                                                             | Aponte o proxy do [sandbox](/docs/pt/sandboxing) para um binário `socat` fora de `PATH`                                                                                                                                                                            | Configurações de sandbox                 | Managed                 |
| [`showClearContextOnPlanAccept`](#showclearcontextonplanaccept)                                       | Mostre uma opção "limpar contexto" na [tela de aceitação de plano](/docs/pt/permission-modes#review-and-approve-a-plan)                                                                                                                                            | Interface e terminal                     | Any file                |
| [`showThinkingSummaries`](#showthinkingsummaries)                                                     | Veja resumos do [pensamento](/docs/pt/model-config#extended-thinking) de Claude em vez de um stub recolhido                                                                                                                                                        | Modelo e respostas                       | Any file                |
| [`showTurnDuration`](#showturnduration)                                                               | Oculte a duração "Cooked for" após cada resposta                                                                                                                                                                                                              | Interface e terminal                     | Any file                |
| [`skillListingBudgetFraction`](#skilllistingbudgetfraction)                                           | Reserve mais ou menos contexto para a [listagem de skills](/docs/pt/skills#skill-descriptions-are-cut-short)                                                                                                                                                       | Memória e contexto                       | Any file                |
| [`skillListingMaxDescChars`](#skilllistingmaxdescchars)                                               | Limite o comprimento da descrição de cada skill na [listagem de skills](/docs/pt/skills#skill-descriptions-are-cut-short)                                                                                                                                          | Memória e contexto                       | Any file                |
| [`skillOverrides`](#skilloverrides)                                                                   | [Oculte ou recolha uma skill](/docs/pt/skills#override-skill-visibility-from-settings) sem editar seu SKILL.md                                                                                                                                                     | Plugins e skills                         | Any file                |
| [`skipAutoPermissionPrompt`](#skipautopermissionprompt)                                               | Pule o aviso único que Claude Code mostra quando você entra no [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) por conta própria em vez de através do padrão integrado                                                               | Configurações de permissão               | User or managed         |
| [`skipDangerousModePermissionPrompt`](#skipdangerousmodepermissionprompt)                             | Pule o diálogo de confirmação antes do [modo bypassPermissions](/docs/pt/permission-modes#skip-all-checks-with-bypasspermissions-mode)                                                                                                                             | Configurações de permissão               | User, local, or managed |
| [`skipWebFetchPreflight`](#skipwebfetchpreflight)                                                     | Pule a [verificação de nome de host WebFetch](/docs/pt/tools-reference#webfetch-tool-behavior) quando Anthropic estiver inacessível                                                                                                                                | Privacidade e telemetria                 | Any file                |
| [`spellcheck`](#spellcheck)                                                                           | Sublinhe palavras com erros de ortografia na entrada do prompt com um [verificador de ortografia](/docs/pt/interactive-mode#check-spelling-as-you-type) que você instala                                                                                           | Interface e terminal                     | User or managed         |
| [`spinnerTipsEnabled`](#spinnertipsenabled)                                                           | Oculte dicas no spinner enquanto Claude trabalha                                                                                                                                                                                                              | Interface e terminal                     | Any file                |
| [`spinnerTipsOverride`](#spinnertipsoverride)                                                         | Adicione suas próprias dicas à rotação do spinner, ou substitua as dicas integradas                                                                                                                                                                           | Interface e terminal                     | Any file                |
| [`spinnerVerbs`](#spinnerverbs)                                                                       | Adicione ou substitua os verbos mostrados enquanto uma volta é executada                                                                                                                                                                                      | Interface e terminal                     | Any file                |
| [`sshConfigs`](#sshconfigs)                                                                           | Adicione [conexões SSH](/docs/pt/desktop#pre-configure-ssh-connections-for-your-team) ao dropdown de ambiente Desktop                                                                                                                                              | Remoto, desktop e notificações           | User or managed         |
| [`sshHostAllowlist`](#sshhostallowlist)                                                               | Limite quais hosts as [sessões SSH do Desktop](/docs/pt/desktop#restrict-which-ssh-hosts-users-can-connect-to) podem alcançar                                                                                                                                      | Remoto, desktop e notificações           | Managed                 |
| [`statusLine`](#statusline)                                                                           | Execute seu próprio comando para renderizar uma [linha de status](/docs/pt/statusline) abaixo do prompt                                                                                                                                                            | Interface e terminal                     | Any file                |
| [`strictKnownMarketplaces`](#strictknownmarketplaces)                                                 | Lista de permissões das fontes de [marketplace](/docs/pt/plugins/overview) que os usuários podem adicionar e instalar                                                                                                                                              | Plugins e skills                         | Managed                 |
| [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)                                     | Bloqueie [skills](/docs/pt/skills), [agentes](/docs/pt/sub-agents), [hooks](/docs/pt/hooks) e [servidores MCP](/docs/pt/mcp) de fontes de usuário e projeto                                                                                                                       | Plugins e skills                         | Managed                 |
| [`strictPluginOnlyCustomization.agents`](#strictpluginonlycustomization-agents)                       | Restrinja [agentes](/docs/pt/sub-agents) a fontes de plugin e gerenciadas                                                                                                                                                                                          | Plugins e skills                         | Managed                 |
| [`strictPluginOnlyCustomization.hooks`](#strictpluginonlycustomization-hooks)                         | Restrinja [hooks](/docs/pt/hooks) a fontes de plugin e gerenciadas                                                                                                                                                                                                 | Plugins e skills                         | Managed                 |
| [`strictPluginOnlyCustomization.mcp`](#strictpluginonlycustomization-mcp)                             | Restrinja [servidores MCP](/docs/pt/mcp) a fontes de plugin e gerenciadas                                                                                                                                                                                          | Plugins e skills                         | Managed                 |
| [`strictPluginOnlyCustomization.skills`](#strictpluginonlycustomization-skills)                       | Restrinja [skills](/docs/pt/skills) a fontes de plugin e gerenciadas                                                                                                                                                                                               | Plugins e skills                         | Managed                 |
| [`subagentPromptCacheTtl`](#subagentpromptcachettl)                                                   | Escolha o [tempo de vida do cache de prompt](/docs/pt/prompt-caching#cache-lifetime) para subagentes e outras solicitações fora da conversa principal                                                                                                              | Modelo e respostas                       | Any file                |
| [`subagentStatusLine`](#subagentstatusline)                                                           | Reescreva linhas na [exibição de tarefa do subagente](/docs/pt/sub-agents) com seu próprio comando                                                                                                                                                                 | Interface e terminal                     | Any file                |
| [`switchModelsOnFlag`](#switchmodelsonflag)                                                           | Alterne modelos automaticamente ou pause quando um [classificador de segurança](/docs/pt/model-config#ask-before-switching) sinalizar uma solicitação                                                                                                              | Modelo e respostas                       | Any file                |
| [`syncClaudeAiPlugins`](#syncclaudeaiplugins)                                                         | Pare de carregar os [plugins ativados em sua conta claude.ai](/docs/pt/plugins/loading#synced-plugins) e pare de baixar novos                                                                                                                                      | Plugins e skills                         | User, local, or managed |
| [`syncClaudeAiSkills`](#syncclaudeaiskills)                                                           | Pare de carregar as [skills ativadas em sua conta claude.ai](/docs/pt/skills#how-synced-skills-behave) e pare de baixar novas                                                                                                                                      | Plugins e skills                         | User, local, or managed |
| [`syntaxHighlightingDisabled`](#syntaxhighlightingdisabled)                                           | Desative o destaque de sintaxe em diffs e blocos de código                                                                                                                                                                                                    | Interface e terminal                     | Any file                |
| [`taskOutputMaxChars`](#taskoutputmaxchars)                                                           | Removido na v2.1.277, junto com a ferramenta `TaskOutput` que dimensionava                                                                                                                                                                                    | Memória e contexto                       | Any file                |
| [`teammateDefaultModel`](#teammatedefaultmodel)                                                       | Removido na v2.1.234; veja [Especificar companheiros de equipe e modelos](/docs/pt/agent-teams#specify-teammates-and-models) para como Claude Code escolhe o modelo de um companheiro de equipe                                                                    | Configurações de config global           | Global config           |
| [`teammateMode`](#teammatemode)                                                                       | Escolha como [companheiros de equipe de agente exibem](/docs/pt/agent-teams#choose-a-display-mode)                                                                                                                                                                 | Agentes, sessões e worktrees             | Any file                |
| [`terminalProgressBarEnabled`](#terminalprogressbarenabled)                                           | Oculte a barra de progresso do terminal em terminais que a suportam                                                                                                                                                                                           | Interface e terminal                     | Any file                |
| [`terminalTitleFromRename`](#terminaltitlefromrename)                                                 | Impeça [`/rename`](/docs/pt/sessions#name-your-sessions) e `--name` de alterar o título da aba do terminal                                                                                                                                                         | Interface e terminal                     | Any file                |
| [`theme`](#theme)                                                                                     | Escolha o [tema de cor](/docs/pt/terminal-config#match-the-color-theme) da interface, integrado ou personalizado                                                                                                                                                   | Interface e terminal                     | Any file                |
| [`timeFormat`](#timeformat)                                                                           | Mostre os horários na interface em um relógio de 12 horas ou 24 horas, em UTC, ou com um padrão strftime                                                                                                                                                      | Interface e terminal                     | Any file                |
| [`timeZone`](#timezone)                                                                               | Mostre os horários na interface em um fuso horário diferente do seu sistema                                                                                                                                                                                   | Interface e terminal                     | Any file                |
| [`tui`](#tui)                                                                                         | Escolha o renderizador [tela cheia](/docs/pt/fullscreen) ou terminal clássico                                                                                                                                                                                      | Interface e terminal                     | Any file                |
| [`ultracode`](#ultracode)                                                                             | Faça Claude planejar um [workflow](/docs/pt/workflows#let-claude-decide-with-ultracode) para cada tarefa substancial sem ser solicitado                                                                                                                            | Modelo e respostas                       | Any file                |
| [`useAutoModeDuringPlan`](#useautomodeduringplan)                                                     | Permita que o classificador de [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) revise comandos shell no [modo de plano](/docs/pt/permission-modes#analyze-before-you-edit-with-plan-mode); defina `false` para obter prompts em vez disso | Configurações de permissão               | User, local, or managed |
| [`verbose`](#verbose)                                                                                 | Mostre a [saída completa da ferramenta](/docs/pt/cli-reference#cli-flags) em vez de resumos truncados; `viewMode` tem precedência quando ambos estão definidos                                                                                                     | Interface e terminal                     | Any file                |
| [`viewMode`](#viewmode)                                                                               | Inicie cada sessão na [visualização padrão, detalhada ou focada](/docs/pt/cli-reference#cli-flags)                                                                                                                                                                 | Interface e terminal                     | Any file                |
| [`vimInsertModeRemaps`](#viminsertmoderemaps)                                                         | Mapeie uma [sequência de modo INSERT](/docs/pt/interactive-mode#remap-insert-mode-key-sequences) de duas teclas como `jj` para Escape                                                                                                                              | Interface e terminal                     | User or managed         |
| [`voice`](#voice)                                                                                     | Ative a [ditação por voz](/docs/pt/voice-dictation) e escolha modo de manutenção ou toque                                                                                                                                                                          | Interface e terminal                     | Any file                |
| [`voiceEnabled`](#voiceenabled)                                                                       | Ative a [ditação por voz](/docs/pt/voice-dictation) com a forma de chave única mais antiga                                                                                                                                                                         | Interface e terminal                     | Any file                |
| [`wheelScrollAccelerationEnabled`](#wheelscrollaccelerationenabled)                                   | Desative a [aceleração de roda do mouse](/docs/pt/fullscreen#mouse-wheel-scrolling) na renderização em tela cheia                                                                                                                                                  | Interface e terminal                     | Any file                |
| [`workflowKeywordTriggerEnabled`](#workflowkeywordtriggerenabled)                                     | Permita que a palavra `ultracode` em um prompt inicie um [workflow](/docs/pt/workflows); defina `false` para digitá-la sem iniciar um                                                                                                                              | Hooks e automação                        | Any file                |
| [`workflowSizeGuideline`](#workflowsizeguideline)                                                     | Defina a contagem de agentes que Claude visa em [workflows dinâmicos](/docs/pt/workflows)                                                                                                                                                                          | Hooks e automação                        | Any file                |
| [`worktree`](#worktree)                                                                               | Configure como Claude Code cria git [worktrees](/docs/pt/worktrees)                                                                                                                                                                                                | Agentes, sessões e worktrees             | Any file                |
| [`worktree.baseRef`](#worktree-baseref)                                                               | Ramifique novos [worktrees](/docs/pt/worktrees) a partir do branch padrão remoto ou seu HEAD local                                                                                                                                                                 | Agentes, sessões e worktrees             | Any file                |
| [`worktree.bgIsolation`](#worktree-bgisolation)                                                       | Permita que sessões em segundo plano editem a cópia de trabalho sem um [worktree](/docs/pt/worktrees)                                                                                                                                                              | Agentes, sessões e worktrees             | Any file                |
| [`worktree.sparsePaths`](#worktree-sparsepaths)                                                       | Verifique apenas os diretórios que você precisa em cada [worktree](/docs/pt/worktrees)                                                                                                                                                                             | Agentes, sessões e worktrees             | Any file                |
| [`worktree.symlinkDirectories`](#worktree-symlinkdirectories)                                         | Crie symlinks de diretórios grandes em cada [worktree](/docs/pt/worktrees) em vez de duplicá-los                                                                                                                                                                   | Agentes, sessões e worktrees             | Any file                |
| [`wslInheritsWindowsSettings`](#wslinheritswindowssettings)                                           | Faça WSL ler [configurações gerenciadas](/docs/pt/managed-settings) da cadeia de política do Windows                                                                                                                                                               | Configurações empresariais e gerenciadas | Managed                 |

<h2 id="model-and-responses">
  Modelo e respostas
</h2>

Escolha quais modelos o Claude Code usa e como ele responde. Para saber como essas configurações interagem com o comando `/model` e variáveis de ambiente, consulte [Configuração de modelo](/docs/pt/model-config).

<h3 id="advisormodel">
  `advisorModel`
</h3>

Escolha qual modelo responde quando Claude chama a [ferramenta advisor](/docs/pt/advisor) do lado do servidor. Deixe-a sem definir para desativar o advisor. O advisor deve ser pelo menos tão capaz quanto seu modelo principal. Consulte [Escolha um modelo advisor](/docs/pt/advisor#choose-an-advisor-model) para os emparelhamentos aceitos e o que acontece quando você escolhe um que não é aceito.

Você normalmente não edita essa chave manualmente. Execute `/advisor` para abrir um seletor que mostra a escolha atual, os modelos que podem aconselhar e **Sem advisor**. Claude Code salva sua escolha nessa chave em `~/.claude/settings.json`. Se você escolher em um cliente [Remote Control](/docs/pt/remote-control) ou em uma sessão anexada a um worker remoto, a escolha se aplica apenas a essa sessão e não altera essa chave.

Se sua conta exigir o [consentimento de créditos de uso](/docs/pt/advisor#fable-advisor-and-usage-credits), aceite-o primeiro executando `/model fable`. Até fazer isso, escolher Fable em `/advisor` não salva nada e Claude Code diz para executar `/model fable` primeiro.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: string, um dos aliases `"fable"`, `"opus"` ou `"sonnet"`, que resolvem para a versão padrão atual do Claude Code dessa família de modelos, ou um ID de modelo completo como `"claude-opus-5-5"`
* **Padrão**: sem definir, então o advisor está desativado
* **Substituições por sessão**: `--advisor` tem precedência sobre essa chave para uma sessão. [`CLAUDE_CODE_DISABLE_ADVISOR_TOOL`](/docs/pt/env-vars) desativa o advisor, e essa chave não pode ativá-lo novamente

```json settings.json theme={null}
{
  "advisorModel": "opus"
}
```

A chave não tem efeito em provedores onde o advisor [não está disponível](/docs/pt/advisor#requirements), como Amazon Bedrock e Claude Platform na AWS. `"fable"` requer [acesso a Fable](/docs/pt/advisor#choose-an-advisor-model).

<h3 id="alwaysthinkingenabled">
  `alwaysThinkingEnabled`
</h3>

Desative o [pensamento estendido](/docs/pt/model-config#extended-thinking) para cada sessão definindo isso como `false`. O pensamento está ativado por padrão, então `true` não muda nada. A maioria das pessoas define isso através de `/config` em vez de editar o arquivo.

Em modelos que sempre pensam, como Opus 5.5 e os modelos Fable, `false` não tem efeito. Em [provedores de terceiros](/docs/pt/third-party-integrations), Claude Code omite o parâmetro `thinking` em vez de desativar o pensamento, então modelos de raciocínio adaptativo podem ainda pensar. Com o pensamento desativado na API Anthropic, Claude Code envia esforço `high` em vez de um nível superior para modelos que sabe [não aceitam essa combinação](/docs/pt/errors#effort-isnt-available-with-thinking-turned-off), como Opus 5.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: Boolean
  * `true`: sem efeito; o pensamento já está ativado
  * `false`: Claude Code desativa o pensamento estendido para cada sessão
* **Padrão**: sem definir, então o pensamento está ativado para modelos que o suportam
* **Substituições por sessão**: [`MAX_THINKING_TOKENS`](/docs/pt/env-vars) tem precedência sobre essa chave para uma sessão: `0` desativa o pensamento, sob os mesmos limites de modelo e provedor que `false`, e um valor positivo ativa o pensamento mesmo quando essa chave é `false`. Em modelos de raciocínio adaptativo, o número em si é ignorado

```json settings.json theme={null}
{
  "alwaysThinkingEnabled": false
}
```

<h3 id="availablemodels">
  `availableModels`
</h3>

Restrinja quais modelos as pessoas podem selecionar para a sessão principal, [subagentes](/docs/pt/sub-agents), [skills](/docs/pt/skills) e o [advisor](/docs/pt/advisor). Uma lista gerenciada restringe `/model`, `--model` e a chave `model` nos próprios arquivos de um desenvolvedor; um modelo fora dela não pode ser selecionado. Por si só, isso não afeta a opção Padrão; combine-o com [`enforceAvailableModels`](#enforceavailablemodels) para isso.

* **Escopo**: [`Qualquer arquivo`](#scopes). Implante-o em configurações gerenciadas para aplicá-lo a uma organização.
* **Tipo**: array de aliases de modelo ou IDs
* **Padrão**: sem definir, então cada modelo está disponível

Este exemplo permite que as pessoas selecionem apenas modelos Sonnet e Haiku:

```json settings.json theme={null}
{
  "availableModels": ["sonnet", "haiku"]
}
```

Consulte [Restrinja a seleção de modelo](/docs/pt/model-config#restrict-model-selection).

<h3 id="effortlevel">
  `effortLevel`
</h3>

Defina um [nível de esforço](/docs/pt/model-config#adjust-effort-level) padrão para modelos para os quais você não salvou um nível. Níveis mais baixos são mais rápidos e mais baratos em tarefas diretas, e níveis mais altos raciocinam mais profundamente em problemas complexos.

Quando você executa `/effort low`, `medium`, `high` ou `xhigh` em uma sessão interativa em sua máquina, Claude Code salva o nível para o modelo ativo em [`modelSettings`](#modelsettings) em vez de escrever essa chave. Antes da v2.1.251, `/effort` escrevia essa chave.

Dentro do mesmo arquivo de configurações, Claude Code usa o nível salvo de um modelo em vez dessa chave. [`modelSettings`](#modelsettings) indica a precedência entre arquivos.

Em uma sessão anexada a um worker remoto, em uma execução `-p` e no Agent SDK, `/effort` se aplica apenas a essa sessão. [Ajuste o nível de esforço](/docs/pt/model-config#adjust-effort-level) lista as escolhas interativas que também se aplicam apenas a essa sessão. A mensagem que `/effort` imprime diz qual aconteceu.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: string, um de:
  * `"low"`: o mínimo de raciocínio, para tarefas curtas, delimitadas, sensíveis à latência que não são sensíveis à inteligência
  * `"medium"`: reduz o uso de tokens para trabalho sensível a custos que pode fazer trade-off de alguma inteligência
  * `"high"`: equilibra o uso de tokens e inteligência
  * `"xhigh"`: raciocínio mais profundo com maior gasto de tokens
* **Padrão**: sem definir
* **Substituições por sessão**: `--effort` tem precedência sobre essa chave para uma sessão, e [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/pt/env-vars) tem precedência sobre ambas

```json settings.json theme={null}
{
  "effortLevel": "xhigh"
}
```

Em suas configurações de usuário, `~/.claude/settings.json`, essa chave é a forma mais antiga que `/effort` escrevia antes de salvar níveis por modelo, e continua se aplicando onde se aplicava antes, em Opus 5, Fable 5.1 e modelos anteriores. Opus 5.5 e modelos lançados após ele a ignoram e começam em seu próprio padrão até você salvar um nível para eles, que `/effort` escreve em [`modelSettings`](#modelsettings). Em configurações de projeto, local e gerenciada, e com `--settings`, essa chave se aplica a cada modelo.

<h3 id="enforceavailablemodels">
  `enforceAvailableModels`
</h3>

O seletor `/model` tem uma opção **Padrão** que resolve para seu [modelo padrão da organização](/docs/pt/model-config#organization-default-model) quando um se aplica, e caso contrário para o padrão do seu tipo de conta. Uma lista de permissões [`availableModels`](#availablemodels) limita os modelos que você pode nomear, mas por si só deixa **Padrão** sozinho, então **Padrão** ainda pode resolver para um modelo fora da lista. Esta chave fecha essa lacuna. Requer Claude Code v2.1.175 ou posterior.

Quando sua organização implanta qualquer configuração gerenciada, Claude Code lê essa chave apenas da fonte gerenciada e a ignora em seus outros arquivos.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: Boolean
  * `true`: quando **Padrão** resolveria para um modelo fora de `availableModels`, Claude Code o resolve para o primeiro modelo disponível na lista
  * `false`: **Padrão** resolve como usual, mesmo para um modelo fora de `availableModels`
* **Padrão**: `false`

Este exemplo restringe seleções nomeadas a modelos Sonnet e Haiku e faz **Padrão** resolver para o primeiro deles que está disponível:

```json settings.json theme={null}
{
  "availableModels": ["sonnet", "haiku"],
  "enforceAvailableModels": true
}
```

Esta chave não tem efeito quando `availableModels` está sem definir ou vazio. Consulte [Aplique a lista de permissões ao modelo Padrão](/docs/pt/model-config#enforce-the-allowlist-for-the-default-model). Requer Claude Code v2.1.175 ou posterior.

<h3 id="fallbackmodel">
  `fallbackModel`
</h3>

Nomeie modelos de backup para Claude Code tentar, em ordem, quando seu modelo principal está sobrecarregado ou indisponível. Claude Code muda para o próximo modelo disponível na cadeia para o resto da volta e mostra um aviso. Sem uma cadeia, Claude Code tenta novamente o mesmo modelo e depois exibe o erro do servidor, e você tenta novamente ou muda de modelos você mesmo.

Uma mudança significa uma volta com um [cache de prompt](/docs/pt/prompt-caching#switching-models) frio no modelo de fallback; sua próxima mensagem tenta o modelo principal primeiro novamente.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: array de aliases de modelo ou IDs; `"default"` expande para o modelo padrão
* **Padrão**: sem definir, então uma solicitação falhada não é tentada novamente em outro modelo
* **Substituições por sessão**: `--fallback-model` tem precedência sobre essa chave para uma sessão

Este exemplo tenta Sonnet 5 primeiro, depois Haiku 4.5, quando seu modelo principal falha:

```json settings.json theme={null}
{
  "fallbackModel": ["claude-sonnet-5", "claude-haiku-4-5"]
}
```

Diferentemente da maioria das configurações de array, essa chave não mescla entre arquivos de configurações: o arquivo de maior precedência que a define fornece toda a cadeia. Se seu arquivo de projeto define `["claude-sonnet-5"]` e seu arquivo de usuário define `["claude-haiku-4-5"]`, a cadeia é apenas `["claude-sonnet-5"]`. Claude Code mantém no máximo três modelos distintos permitidos da lista e ignora o resto. Consulte [Cadeias de modelo de fallback](/docs/pt/model-config#fallback-model-chains).

<h3 id="fastmode">
  `fastMode`
</h3>

Ative o [modo rápido](/docs/pt/fast-mode) para sessões onde está disponível, para trabalho interativo como iteração rápida ou depuração ao vivo onde você quer velocidade a um custo mais alto por token. Você normalmente não edita essa chave manualmente: executar `/fast` escreve `fastMode: true` em `~/.claude/settings.json`, e executá-lo novamente para desativar o modo rápido remove a chave. O modo rápido funciona apenas em Opus 5.5, Opus 5 e Opus 4.8: ativá-lo de outro modelo o muda para Opus, e mudar para um modelo não suportado o desativa. Consulte [Mude de modelos enquanto o modo rápido está ativado](/docs/pt/fast-mode#switch-models-while-fast-mode-is-on).

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: Boolean
  * `true`: Claude Code ativa o modo rápido para sessões onde está disponível
  * `false`: o modo rápido permanece desativado
* **Padrão**: sem definir, então o modo rápido está desativado
* **Substituições por sessão**: [`CLAUDE_CODE_DISABLE_FAST_MODE`](/docs/pt/env-vars) desativa o modo rápido para uma sessão, e essa chave não pode ativá-lo novamente

```json settings.json theme={null}
{
  "fastMode": true
}
```

<h3 id="fastmodepersessionoptin">
  `fastModePerSessionOptIn`
</h3>

Normalmente, executar `/fast` salva [`fastMode`](#fastmode) nas configurações de usuário de uma pessoa, então o modo rápido está ativado no início de cada sessão posterior. Defina essa chave como `true` para parar isso: um `fastMode: true` salvo não ativa mais o modo rápido no início da sessão, e cada pessoa tem que executar `/fast` em cada sessão que o quer. Claude Code deixa a chave `fastMode` em seu arquivo, então desativar essa chave restaura o comportamento antigo.

Proprietários em planos Team ou Enterprise podem implantá-lo em toda a organização através de [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings). Quando as configurações gerenciadas definem a chave, `/fast on` é recusado fora de sessões de terminal interativas e relata que sua organização desativou o modo rápido. Isso cobre [modo não interativo](/docs/pt/headless), a [extensão VS Code](/docs/pt/vs-code) e [sessões na nuvem](/docs/pt/claude-code-on-the-web).

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: Boolean
  * `true`: um `fastMode: true` salvo não ativa mais o modo rápido no início da sessão, então cada pessoa executa `/fast` em cada sessão que o quer; um `fastMode: true` passado com `--settings` ainda conta para essa sessão a menos que as configurações gerenciadas definam essa chave
  * `false`: um `fastMode: true` salvo ativa o modo rápido no início de cada sessão posterior
* **Padrão**: `false`

```json settings.json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

Consulte [Exija opt-in por sessão](/docs/pt/fast-mode#require-per-session-opt-in).

<h3 id="language">
  `language`
</h3>

Faça Claude responder em um idioma diferente do inglês por padrão. Não há lista fixa para respostas: Claude Code passa o valor verbatim para Claude como uma instrução para sempre responder nesse idioma, então qualquer nome de idioma que Claude possa ler funciona. Claude Code não verifica o valor, então um nome digitado incorretamente chega a Claude como escrito em vez de produzir um erro. O mesmo valor define o idioma para [ditado de voz](/docs/pt/voice-dictation#change-the-dictation-language), que tem uma lista fixa de [idiomas de ditado suportados](/docs/pt/voice-dictation#change-the-dictation-language), e para títulos de sessão gerados automaticamente.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: string, qualquer nome de idioma, como `"japanese"`, `"spanish"` ou `"french"`; Claude Code não o valida
* **Padrão**: sem definir; os títulos de sessão então correspondem ao idioma de sua conversa

```json settings.json theme={null}
{
  "language": "japanese"
}
```

<h3 id="maxeffortlevel">
  `maxEffortLevel`
</h3>

Limite o [nível de esforço](/docs/pt/model-config#adjust-effort-level) que uma sessão pode usar, deixando níveis mais baixos disponíveis. Qualquer nível mais alto funciona no limite em vez disso, incluindo um de `/effort`, o seletor `/model`, `--effort`, [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/pt/env-vars), a frontmatter `effort` de uma skill ou subagente, ou o padrão do próprio modelo. Claude Code aplica o limite em si antes de cada solicitação, então ele se mantém em cada provedor, incluindo Amazon Bedrock, Agent Platform do Google Cloud e Microsoft Foundry. Requer Claude Code v2.1.267 ou posterior.

* **Escopo**: [`Qualquer arquivo`](#scopes). Implante-o em configurações gerenciadas para aplicá-lo a uma organização. Quando vários escopos definem um limite, o mais baixo se aplica, então um limite definido em um escopo não pode ser aumentado de outro
* **Tipo**: string, um de `"low"`, `"medium"`, `"high"`, `"xhigh"` ou `"max"`. Um valor `"max"` não define limite
* **Padrão**: sem definir, então nenhum limite se aplica
* **Efeito no ultracode**: um limite abaixo de `xhigh` torna [ultracode](#ultracode) indisponível nos modelos aos quais o limite se aplica
* **Limites por modelo**: adicione `maxEffortLevel` à entrada [`modelSettings`](#modelsettings) de um modelo. Essa entrada substitui essa chave apenas para o modelo dentro da fonte de configurações que define ambas, como suas configurações de usuário ou uma [fonte gerenciada](/docs/pt/managed-settings#how-claude-code-combines-managed-sources). Defina `"max"` lá para isentar o modelo do limite dessa fonte; Claude Code ainda aplica limites de outras fontes

Este exemplo limita cada modelo a `medium` e isenta Sonnet 4.6:

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

Quando sua organização também define um [limite de esforço](/docs/pt/model-config#organization-effort-limits) para um modelo, o menor dos dois limites se aplica.

<h3 id="model">
  `model`
</h3>

Defina o modelo que cada nova sessão usa, então você não tem que escolher um com `/model` cada vez. Defini-lo aqui não o impede de mudar no meio da sessão. Se seu admin definiu um [modelo padrão da organização](/docs/pt/model-config#organization-default-model) para substituir a seleção do usuário, você obtém esse modelo mesmo quando define essa chave em configurações de usuário, projeto ou local.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: string, um alias de modelo ou ID de modelo completo
* **Padrão**: sem definir, então Claude Code usa o modelo padrão de sua conta
* **Substituições por sessão**: `--model` tem precedência sobre [`ANTHROPIC_MODEL`](/docs/pt/env-vars), e ambos têm precedência sobre essa chave para uma sessão, incluindo sobre um `model` gerenciado; uma lista [`availableModels`](#availablemodels) ainda se aplica à escolha

```json settings.json theme={null}
{
  "model": "claude-sonnet-5"
}
```

Um valor aqui supera [`ANTHROPIC_DEFAULT_MODEL`](/docs/pt/model-config#set-a-default-model-for-new-sessions), que Claude Code usa apenas quando nada mais seleciona um modelo.

<h3 id="modeloverrides">
  `modelOverrides`
</h3>

Mapeie IDs de modelo Anthropic para IDs de modelo específicos do provedor, como ARNs de perfil de inferência do Amazon Bedrock. Cada entrada do seletor de modelo então usa seu valor mapeado ao chamar a API do provedor. Administradores usam isso em [Amazon Bedrock, Agent Platform do Google Cloud e Microsoft Foundry](/docs/pt/model-config#override-model-ids-per-version) para rotear cada versão de modelo para um perfil de inferência específico, nome de versão ou implantação para governança, alocação de custos ou roteamento regional.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: objeto mapeando ID de modelo para ID de modelo do provedor
* **Padrão**: sem definir

Este exemplo roteia cada chamada para Opus 4.6 para o perfil de inferência Bedrock nomeado:

```json settings.json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-6": "arn:aws:bedrock:us-east-1:123456789012:inference-profile/example"
  }
}
```

Consulte [Substitua IDs de modelo por versão](/docs/pt/model-config#override-model-ids-per-version).

<h3 id="modelpicker">
  `modelPicker`
</h3>

Liste os modelos que o seletor `/model` oferece, na ordem que você os escreve e sob rótulos que você escolhe, então o seletor lista os modelos que sua organização executa, após o lineup integrado ou em vez dele. O `model` de cada linha é tomado verbatim, então aceita qualquer coisa que `--model` aceita: um alias como `opus`, um ID de modelo Anthropic ou um ID de formato de provedor para Amazon Bedrock, Agent Platform do Google Cloud, Microsoft Foundry ou um gateway LLM. Requer Claude Code v2.1.242 ou posterior.

* **Escopo**: [`Usuário ou gerenciado`](#scopes). Claude Code lê a chave de configurações gerenciadas, `--settings` e configurações de usuário, e a ignora em configurações de projeto e local para que um repositório que você clone não possa rotular novamente o seletor. O maior dos três que define a chave fornece todo o lineup, e Claude Code nunca combina lineups de duas fontes.
* **Tipo**: objeto com um array `options` de linhas e um Boolean `replaceBuiltInOptions` opcional
* **Padrão**: sem definir, então o seletor mostra o lineup integrado

Este exemplo adiciona duas implantações Bedrock após o lineup integrado, sob nomes que sua equipe reconhece:

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
  Campos para `modelPicker`
</h4>

A chave leva dois campos, um para as linhas em si e outro para se elas substituem o lineup integrado ou o adicionam.

| Campo                   | Tipo                                                                                        | O que faz                                                                                                                                                                                                                                                                                      |
| :---------------------- | :------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options`               | array de linhas, cada uma com um `model` obrigatório e um `label` e `description` opcionais | As linhas que o seletor mostra, nesta ordem, exceto que uma linha acinzentada se move para o final. Sem um `label`, Claude Code intitula a linha com o nome integrado para um modelo que conhece, ou o ID do modelo caso contrário, e sem uma `description` escreve uma segunda linha genérica |
| `replaceBuiltInOptions` | Boolean, padrão `false`                                                                     | Defina como `true` para mostrar apenas essas linhas, **Padrão** e uma linha para o modelo que a sessão já está usando. Deixe sem definir para adicionar essas linhas após o lineup integrado                                                                                                   |

Com `replaceBuiltInOptions` ativado, Claude Code oculta todas as outras linhas: o lineup integrado, as linhas que adiciona para entradas [`availableModels`](#availablemodels), os modelos que [descoberta de gateway](/docs/pt/llm-gateway-protocol#model-discovery) encontrou e [`ANTHROPIC_CUSTOM_MODEL_OPTION`](/docs/pt/model-config#add-a-custom-model-option). Com ele desativado, Claude Code pula um modelo listado que o lineup integrado já cobre. Um rótulo muda o que o seletor mostra, não qual modelo Claude Code executa.

Uma lista de permissões [`availableModels`](#availablemodels) ainda se aplica a essas linhas. Antes de adicionar um modelo listado à lista de permissões, leia [Comportamento de mesclagem](/docs/pt/model-config#merge-behavior): um ID de modelo específico estreita a entrada curinga de sua família. Claude Code também verifica cada linha contra a sessão antes de mostrar o seletor:

* **Descartada**: uma linha que Claude Code não pode servir, como um modelo aposentado ou um modelo ao qual sua organização não tem acesso
* **Acinzentada**: uma linha que você não pode selecionar ainda, mostrada com o motivo
* **Nenhuma linha sobrevive**: Claude Code mantém o lineup integrado, filtrado pela lista de permissões como usual

Claude Code descarta uma linha que não pode analisar e mantém o resto. Consulte [Corrija um arquivo de configurações quebrado](/docs/pt/settings#fix-a-broken-settings-file).

<h3 id="modelpricing">
  `modelPricing`
</h3>

Relate gastos nas taxas que sua organização paga em vez de preço de lista. Defina-o quando sua organização tem taxas contratadas, então os valores em dólares que os desenvolvedores veem correspondem à sua fatura. Claude Code aplica as taxas em `/usage`, a [linha de status](/docs/pt/statusline), o `total_cost_usd` do Agent SDK, o limite [`--max-budget-usd`](/docs/pt/cli-reference) e a métrica de custo [OpenTelemetry](/docs/pt/monitoring-usage) e eventos. Você fornece as taxas: Claude Code não as lê de seu contrato ou do Claude Console. Requer Claude Code v2.1.242 ou posterior.

* **Escopo**: [`Gerenciado`](#scopes). Implante a chave através de configurações gerenciadas pelo servidor, uma política MDM, um arquivo `managed-settings.json` ou um [auxiliar de política](/docs/pt/managed-settings#compute-the-policy-with-a-helper-program). Claude Code a ignora em configurações de usuário, projeto e local, em `--settings` e no Windows no [registro HKCU](/docs/pt/managed-settings#where-each-mechanism-stores-the-policy) gravável pelo usuário. Com configurações gerenciadas pelo servidor, cada sessão relata custos ao preço de lista até que a [busca de configurações](/docs/pt/server-managed-settings#fetch-and-caching-behavior) dessa sessão tenha confirmado a configuração. Um aplicativo host que incorpora Claude Code e define [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/pt/env-vars) pode fornecer uma tabela própria através da opção SDK [`managedSettings`](/docs/pt/agent-sdk/typescript#options), que Claude Code usa apenas quando nenhuma fonte gerenciada define a chave e apenas em Claude Code v2.1.246 ou posterior.
* **Tipo**: objeto com um `multiplier` opcional e um mapa `overrides` opcional
* **Padrão**: sem definir, então Claude Code relata preço de lista a menos que um aplicativo host forneça uma tabela

Defina `multiplier` sozinho para um desconto fixo, `overrides` sozinho para taxas por modelo ou ambos.

Este exemplo define taxas contratadas para Sonnet 4.6 e depois reduz cada figura, a linha Sonnet incluída, em 15%:

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

Defina `multiplier` acima de 1, até 10, para marcar cada figura para cima. Uma marcação requer Claude Code v2.1.271 ou posterior. Versões anteriores ignoram um `multiplier` acima de 1 com um aviso e mantêm o resto da configuração.

Para as etapas, incluindo como confirmar que as taxas estão em vigor, consulte [Relate gastos em suas taxas contratadas](/docs/pt/costs#report-spend-at-your-contracted-rates).

<span id="modelpricing-multiplier" />

<span id="modelpricing-overrides" />

<h4 id="fields-for-modelpricing">
  Campos para `modelPricing`
</h4>

| Campo        | Tipo                                                                                                                | O que faz                                                                                                                                                                                                                                                                     |
| :----------- | :------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `multiplier` | número maior que 0 e no máximo 10                                                                                   | Dimensiona cada custo que Claude Code calcula, independentemente de uma linha `overrides` cobri-lo. Abaixo de 1 é um desconto, acima de 1 uma marcação                                                                                                                        |
| `overrides`  | mapa de ID de modelo para um objeto de taxa com `input`, `output`, `cacheRead` e `cacheWrite`, cada um de 0 a 10000 | As taxas USD-por-milhão-de-tokens para esse modelo, todos os quatro obrigatórios. `cacheWrite` cobre tanto gravações de cache de cinco minutos quanto de uma hora. Consulte [Quais modelos uma linha `modelPricing` se aplica a](#which-models-a-modelpricing-row-applies-to) |

Claude Code usa as taxas de uma linha exatamente como você as escreveu, sem adicionar a sobretaxa do modo rápido ou a [taxa de inferência apenas para EUA](https://platform.claude.com/docs/en/about-claude/pricing). Se você também definir `multiplier`, Claude Code a aplica no topo das taxas da linha. Claude Code descarta uma linha com uma taxa que não pode analisar ou um `multiplier` que não pode analisar e mantém o resto; consulte [Corrija um arquivo de configurações quebrado](/docs/pt/settings#fix-a-broken-settings-file).

<h4 id="which-models-a-modelpricing-row-applies-to">
  Quais modelos uma linha `modelPricing` se aplica a
</h4>

Claude Code decide quais modelos uma linha se aplica a partir da chave da linha:

* **ID de um modelo integrado**: uma chave que Claude Code usa para um modelo integrado, independentemente de essa chave ser o próprio ID do modelo, como `claude-sonnet-4-6`, ou seu ID Bedrock, Agent Platform ou Foundry. Claude Code aplica a linha a cada ID de snapshot datado e ID específico do provedor desse modelo.
* **Qualquer outra chave**: uma chave que não é o ID de um modelo integrado, como um alias de modelo de gateway. Claude Code aplica a linha apenas a esse ID. Quando um ID de modelo corresponde exatamente a uma de suas chaves e também se enquadra em uma linha com chave de ID de modelo integrado, Claude Code usa a correspondência exata.
* **Um perfil de inferência de aplicação Bedrock**: uma vez que Claude Code resolveu o perfil para o modelo ao qual roteia, através de seu mapa [`modelOverrides`](#modeloverrides) ou da busca [`bedrock:GetInferenceProfile`](/docs/pt/amazon-bedrock#iam-configuration), Claude Code aplica a linha desse modelo ao perfil.

<h3 id="modelsettings">
  `modelSettings`
</h3>

Salve um [nível de esforço](/docs/pt/model-config#adjust-effort-level) para cada modelo que você usa. Requer Claude Code v2.1.251 ou posterior.

Em uma sessão interativa em sua máquina, quando você salva `low`, `medium`, `high` ou `xhigh` como seu padrão com `/effort` ou o controle deslizante de esforço do seletor `/model`, Claude Code escreve esse nível aqui sob o modelo que você está usando, então você raramente edita essa chave você mesmo. Quando você escolhe um desses níveis no [seletor de modelo da extensão VS Code](/docs/pt/vs-code#use-the-prompt-box), Claude Code salva-o aqui da mesma forma. A entrada [`effortLevel`](#effortlevel) lista as sessões onde `/effort` se aplica apenas a essa sessão.

Edite a chave manualmente para alterar ou remover um nível que você salvou.

Um `effortLevel` de um modelo aqui tem precedência sobre o [`effortLevel`](#effortlevel) de nível superior no mesmo arquivo de configurações. Entre arquivos, Claude Code resolve cada modelo separadamente: o arquivo de configurações de maior precedência [settings file](/docs/pt/settings#settings-precedence) que define um `effortLevel` para esse modelo ou o `effortLevel` de nível superior que [se aplica a esse modelo](#effortlevel) decide, então um `effortLevel` em configurações gerenciadas supera um nível que você salvou em configurações de usuário. [Ajuste o nível de esforço](/docs/pt/model-config#adjust-effort-level) lista o que mais pode substituir um nível salvo, como `--effort` no lançamento.

Para limitar o esforço de um modelo em vez de definir seu nível, adicione um campo [`maxEffortLevel`](#maxeffortlevel) à entrada desse modelo. O campo requer Claude Code v2.1.267 ou posterior.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: objeto mapeando um nome de modelo para um objeto com um campo `effortLevel`, um de `"low"`, `"medium"`, `"high"` ou `"xhigh"`, um campo [`maxEffortLevel`](#maxeffortlevel) ou ambos
* **Padrão**: sem definir

Claude Code escreve cada entrada sob o nome canônico do modelo, como `claude-opus-5-5`, e corresponde ao alias desse modelo, com sufixo de data, `[1m]` e IDs específicos do provedor reconhecidos à mesma entrada.

Este exemplo mantém Opus 5.5 em `high` enquanto outros modelos usam seus próprios níveis salvos ou padrão:

```json settings.json theme={null}
{
  "modelSettings": {
    "claude-opus-5-5": {
      "effortLevel": "high"
    }
  }
}
```

Execute `/effort auto` para limpar seu nível salvo para o modelo que você está usando. Claude Code deixa as outras entradas e qualquer `effortLevel` de nível superior em vigor.

<h3 id="outputstyle">
  `outputStyle`
</h3>

Selecione um [estilo de saída](/docs/pt/output-styles) por nome. Um estilo de saída é um conjunto salvo de instruções que muda o papel, tom e formato de saída de Claude, como os estilos Explanatory e Learning integrados ou um que você escreveu você mesmo.

Se você alterar essa chave durante uma sessão, Claude usa o novo estilo começando com sua próxima mensagem. Para o que essa mensagem custa em cache de prompt, consulte [Alterando estilo de saída](/docs/pt/prompt-caching#changing-output-style). Antes da v2.1.251, a edição se aplicava apenas depois que você executava `/clear` ou iniciava uma nova sessão.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: string, o nome de um estilo de saída [integrado](/docs/pt/output-styles#built-in-output-styles) ou [personalizado](/docs/pt/output-styles#create-a-custom-output-style)
* **Padrão**: sem definir, então Claude Code usa o estilo padrão

Este exemplo seleciona o estilo Explanatory integrado, que adiciona insights educacionais entre tarefas:

```json settings.json theme={null}
{
  "outputStyle": "Explanatory"
}
```

<h3 id="promptcachettl">
  `promptCacheTtl`
</h3>

Escolha quanto tempo o [cache de prompt](/docs/pt/prompt-caching) mantém a conversa principal. Esta chave se aplica a suas voltas interativas, `-p` e Agent SDK, juntamente com os auxiliares que Claude Code executa inline com elas. A vida útil de uma hora mantém o cache aquecido entre pausas mais longas, e a API [cobra cada gravação de cache a uma taxa mais alta](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing) do que na vida útil de cinco minutos. Requer Claude Code v2.1.242 ou posterior.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: string, um de:
  * `"5m"`: o cache se mantém por cinco minutos
  * `"1h"`: o cache se mantém por uma hora
* **Padrão**: sem definir, então cada solicitação de conversa principal obtém [sua vida útil padrão](/docs/pt/prompt-caching#which-ttl-each-request-gets)
* **Substituições por sessão**: [`FORCE_PROMPT_CACHING_5M`](/docs/pt/env-vars) tem precedência sobre tudo mais, depois [`CLAUDE_CODE_PROMPT_CACHE_TTL`](/docs/pt/env-vars), depois essa chave e por último [`ENABLE_PROMPT_CACHING_1H`](/docs/pt/env-vars)

Este exemplo mantém a conversa principal na vida útil de uma hora e deixa subagentes em cinco minutos:

```json settings.json theme={null}
{
  "promptCacheTtl": "1h",
  "subagentPromptCacheTtl": "5m"
}
```

Para o que cada vida útil custa, consulte [Vida útil do cache](/docs/pt/prompt-caching#cache-lifetime).

<h3 id="showthinkingsummaries">
  `showThinkingSummaries`
</h3>

Veja resumos do [pensamento estendido](/docs/pt/model-config#extended-thinking) de Claude em sessões interativas. Defina-o se você quer os resumos completos quando expande o pensamento com `Ctrl+O`. Quando sem definir ou `false`, a API Anthropic redige blocos de pensamento e Claude Code mostra um stub recolhido; provedores de terceiros não redagem.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: Boolean
  * `true`: você vê resumos completos de pensamento quando expande o pensamento com `Ctrl+O`
  * `false`: a API Anthropic redige blocos de pensamento e Claude Code mostra um stub recolhido
* **Padrão**: `false`

```json settings.json theme={null}
{
  "showThinkingSummaries": true
}
```

A redação muda apenas o que você vê, não o que o modelo gera. Para reduzir o gasto de pensamento, [reduza o orçamento ou desative o pensamento](/docs/pt/model-config#extended-thinking) em vez disso.

<h3 id="subagentpromptcachettl">
  `subagentPromptCacheTtl`
</h3>

Escolha quanto tempo o [cache de prompt](/docs/pt/prompt-caching) mantém as solicitações que Claude Code faz fora da conversa principal. Esta chave se aplica a [subagentes](/docs/pt/sub-agents), [workflows](/docs/pt/workflows) e as próprias solicitações de background e auxiliar de Claude Code, como compactação e títulos de sessão. A vida útil de uma hora mantém o cache aquecido entre pausas mais longas, e a API [cobra cada gravação de cache a uma taxa mais alta](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing) do que na vida útil de cinco minutos. Requer Claude Code v2.1.242 ou posterior.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: string, um de:
  * `"5m"`: o cache se mantém por cinco minutos
  * `"1h"`: o cache se mantém por uma hora
* **Padrão**: sem definir, então cada uma dessas solicitações obtém [sua vida útil padrão](/docs/pt/prompt-caching#which-ttl-each-request-gets)
* **Substituições por sessão**: [`FORCE_PROMPT_CACHING_5M`](/docs/pt/env-vars) tem precedência sobre tudo mais, depois [`CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`](/docs/pt/env-vars), depois essa chave, depois [`ENABLE_PROMPT_CACHING_1H`](/docs/pt/env-vars), que pede a vida útil de uma hora em cada solicitação. Para onde o valor de frontmatter próprio de um subagente se classifica, consulte [Escolha o TTL você mesmo](/docs/pt/prompt-caching#choose-the-ttl-yourself)

Este exemplo dá aos subagentes e às outras solicitações fora da conversa principal a vida útil de uma hora:

```json settings.json theme={null}
{
  "subagentPromptCacheTtl": "1h"
}
```

Esta chave cobre as solicitações que [`promptCacheTtl`](#promptcachettl) não cobre, então defina ambas para escolher uma vida útil para cada solicitação que Claude Code faz. Para como o cache de um subagente difere do cache da conversa principal, consulte [Subagentes e o cache](/docs/pt/prompt-caching#subagents-and-the-cache).

<h3 id="switchmodelsonflag">
  `switchModelsOnFlag`
</h3>

Escolha o que acontece quando um [classificador de segurança sinaliza uma solicitação](/docs/pt/model-config#automatic-model-fallback): mude para o modelo de fallback e continue, ou pause para que você possa escolher entre mudar e editar o prompt.

* **Escopo**: [`Qualquer arquivo`](#scopes). Aparece em `/config` como **Mude de modelos quando uma mensagem é sinalizada**.
* **Tipo**: Boolean
  * `true`: Claude Code muda para o modelo de fallback e continua
  * `false`: em uma sessão interativa Claude Code pausa para que você possa escolher entre mudar e editar o prompt; onde nenhum diálogo pode mostrar, como uma execução `-p`, a solicitação sinalizada termina como um erro
* **Padrão**: `true`, mude automaticamente

```json settings.json theme={null}
{
  "switchModelsOnFlag": false
}
```

Consulte [Pergunte antes de mudar](/docs/pt/model-config#ask-before-switching).

<h3 id="ultracode">
  `ultracode`
</h3>

Inicie sessões com [ultracode](/docs/pt/workflows#let-claude-decide-with-ultracode) ativado. Com ele ativado, Claude planeja um workflow para cada tarefa substancial em vez de esperar você pedir. Claude planeja workflows apenas quando [workflows dinâmicos](/docs/pt/workflows) estão habilitados para você, seu modelo suporta esforço `xhigh` e nenhum [limite de esforço](/docs/pt/model-config#organization-effort-limits) abaixo de `xhigh` se aplica. De qualquer forma, `ultracode: true` executa a sessão em esforço `xhigh` ou no limite quando um limite de esforço é mais baixo. Claude Code lê essa chave mas nunca a escreve: `/effort ultracode` ativa ultracode apenas para a sessão atual.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: Boolean
  * `true`: sessões começam em esforço `xhigh`, com ultracode ativado quando workflows dinâmicos estão habilitados para você, seu modelo suporta `xhigh` e nenhum limite de esforço está abaixo de `xhigh`
  * `false`: sessões começam com ultracode desativado
* **Padrão**: sem definir, então ultracode está desativado
* **Substituições por sessão**: `/effort ultracode` ativa ultracode para uma sessão sem essa chave. O sinalizador `--effort ultracode` também o ativa para uma sessão e requer Claude Code v2.1.203 ou posterior

```json settings.json theme={null}
{
  "ultracode": true
}
```

Ultracode executa a sessão em esforço `xhigh` e tem precedência sobre `effortLevel` e entradas [`modelSettings`](#modelsettings). Se um [limite de esforço](/docs/pt/model-config#organization-effort-limits) abaixo de `xhigh` se aplica ao modelo, como uma configuração [`maxEffortLevel`](#maxeffortlevel), a sessão executa no limite em vez disso e ultracode permanece desativado. Claude então não planeja workflows por conta própria, e `/effort` não oferece `ultracode`. Uma solicitação de controle `apply_flag_settings` do Agent SDK também aceita a chave.

<h2 id="permission-settings">
  Configurações de permissão
</h2>

Decida o que Claude pode fazer sem perguntar, em qual modo de permissão uma sessão começa e o que o classificador do modo automático permite. Para a sintaxe de regras e o modelo de permissão, consulte [Configurar permissões](/docs/pt/permissions).

<h3 id="allowmanagedpermissionrulesonly">
  `allowManagedPermissionRulesOnly`
</h3>

Torne as configurações gerenciadas a única fonte de regras de permissão. Claude Code então ignora as regras `allow`, `ask` e `deny` em arquivos de usuário, projeto, local e `--settings`, ignora `--allowedTools`, oculta as opções de sempre permitir nos prompts de permissão e para de salvar novas regras.

Quando [configurações pai de um host de incorporação](/docs/pt/managed-settings#let-an-embedding-host-add-policy) se aplicam, Claude Code as trata como parte da camada gerenciada. Ele descarta suas regras `allow` e `additionalDirectories`, e mantém suas regras `deny` e `ask` exceto regras `Read` e `Edit` cujo padrão começa com `!`. Um host não pode esculpir caminhos fora das regras gerenciadas com uma regra `!`, independentemente de você definir esta chave.

As regras `--disallowedTools` e as regras `deny` e `ask` da sessão atual ainda se aplicam, inclusive após Claude Code recarregar as configurações no meio da sessão. Elas apenas restringem, portanto não podem ampliar o que as regras gerenciadas concedem. Antes da v2.1.257, Claude Code descartava essas regras de linha de comando e de sessão no primeiro recarregamento de configurações.

Para o que um padrão `!` em uma regra `--disallowedTools` ou de sessão pode esculpir, consulte [Regras Read e Edit](/docs/pt/permissions#read-and-edit).

* **Escopo**: [`Managed`](#scopes)
* **Tipo**: Booleano
  * `true`: as configurações gerenciadas se tornam a única fonte de regras de permissão
  * `false`: Claude Code aplica regras de permissão de arquivos de usuário, projeto, local e `--settings` além das gerenciadas
* **Padrão**: não definido, portanto Claude Code aplica regras de permissão de configurações de usuário, projeto e local e de `--settings`, além das gerenciadas

```json managed-settings.json theme={null}
{
  "allowManagedPermissionRulesOnly": true
}
```

Esta chave não bloqueia a lista de permissões do servidor MCP; para isso, defina [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly). Consulte [Configurações apenas gerenciadas](/docs/pt/managed-settings#managed-only-settings).

<h3 id="automode">
  `autoMode`
</h3>

Adicione suas próprias regras ao que o classificador do [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) bloqueia e permite. Use-o para informar ao classificador quais repositórios, buckets e domínios sua organização confia, para que ele pare de bloquear operações internas rotineiras. O classificador é fornecido com [regras de permissão e bloqueio integradas](/docs/pt/auto-mode-config#inspect-the-defaults-and-your-effective-config). Inclua a string literal `"$defaults"` em um array para manter essas regras integradas nessa posição e adicione as suas ao redor; deixe-a de fora para substituí-las pelas suas.

* **Escopo**: [`User or managed`](#scopes)
* **Tipo**: objeto com arrays `environment`, `allow`, `soft_deny` e `hard_deny` de regras em prosa, mais o Booleano [`classifyAllShell`](#automode-classifyallshell)
* **Padrão**: não definido, portanto o classificador usa apenas suas [regras integradas](/docs/pt/auto-mode-config#inspect-the-defaults-and-your-effective-config)

Este exemplo mantém as regras `soft_deny` integradas, através de `"$defaults"`, e adiciona uma mais que bloqueia `terraform apply`:

```json settings.json theme={null}
{
  "autoMode": {
    "soft_deny": ["$defaults", "Never run terraform apply"]
  }
}
```

Quando mais de um desses arquivos define o mesmo array, Claude Code concatena as entradas. Para o formato de regra e como cada array é aplicado, consulte [Configurar modo automático](/docs/pt/auto-mode-config).

<h3 id="automode-classifyallshell">
  `autoMode.classifyAllShell`
</h3>

Envie cada comando Bash e PowerShell através do classificador do modo automático enquanto o modo automático está ativo. Por padrão, o modo automático suspende apenas regras de permissão que poderiam executar código arbitrário: regras de ferramenta inteira e curinga como `Bash(*)`, e prefixos de interpretador ou wrapper de shell como `Bash(python *)`. Um comando que qualquer outra regra de permissão corresponde, como `Bash(npm test)`, pula o classificador a menos que ele carregue [domínios permitidos por comando](/docs/pt/sandboxing#per-command-allowed-domains-in-auto-mode). Quando pula, um argumento destrutivo que o prefixo da regra não antecipou pode passar despercebido. Definir esta chave suspende cada regra de shell de permissão para a sessão para que o classificador veja cada comando. Requer Claude Code v2.1.193 ou posterior.

* **Escopo**: [`User or managed`](#scopes). Leia onde [`autoMode`](#automode) é lido.
* **Tipo**: Booleano
  * `true`: enquanto o modo automático está ativo, Claude Code envia cada comando Bash e PowerShell através do classificador e suspende suas regras de shell de permissão; fora do modo automático as regras ainda se aplicam
  * `false`: o modo automático suspende apenas regras de permissão que poderiam executar código arbitrário, como `Bash(*)` e `Bash(python *)`; um comando que qualquer outra regra de permissão corresponde pula o classificador a menos que ele carregue [domínios permitidos por comando](/docs/pt/sandboxing#per-command-allowed-domains-in-auto-mode), e cada outro comando de shell passa por ele
* **Padrão**: `false`

```json settings.json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

Consulte [Rotear todos os comandos de shell através do classificador](/docs/pt/auto-mode-config#route-all-shell-commands-through-the-classifier). Requer Claude Code v2.1.193 ou posterior.

<h3 id="disableautomode">
  `disableAutoMode`
</h3>

Remova o [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) do ciclo `Shift+Tab`. Qualquer sessão que de outra forma [começaria em modo automático](/docs/pt/permission-modes#which-mode-a-session-starts-in), seja de `--permission-mode auto`, um arquivo de configurações ou o padrão integrado, começa em `default` em vez disso. Administradores o definem em configurações gerenciadas para impedir que desenvolvedores em sua organização usem o modo automático.

* **Escopo**: [`Any file`](#scopes). Mais útil em [configurações gerenciadas](/docs/pt/managed-settings), onde os usuários não podem substituí-lo. Também aceito sob `permissions` como `permissions.disableAutoMode`.
* **Tipo**: a string `"disable"`
* **Padrão**: não definido

```json settings.json theme={null}
{
  "disableAutoMode": "disable"
}
```

<h3 id="permissions">
  `permissions`
</h3>

Controle quais ferramentas Claude pode usar sem perguntar, quais sempre solicitam e quais são bloqueadas, e defina o [modo de permissão](/docs/pt/permission-modes) em que uma sessão começa. Cada chave `permissions.*` abaixo se aninha sob este objeto.

* **Escopo**: [`Any file`](#scopes)
* **Tipo**: objeto com `allow`, `ask`, `deny`, `additionalDirectories`, `blockReadsOutsideWorkingDirectories`, `defaultMode`, `disableBypassPermissionsMode` e `disableAutoMode`
* **Padrão**: não definido

Este exemplo aprova comandos `npm run` sem perguntar, solicita antes de `git push`, bloqueia leituras de `.env` e inicia sessões em `acceptEdits`:

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

Os três arrays de regras compartilham uma sintaxe; consulte [Sintaxe de regra de permissão](#permission-rule-syntax) sob `permissions.allow`. Para como as regras de permissão de diferentes arquivos se combinam, consulte [como as regras de permissão se mesclam entre escopos](/docs/pt/permissions#settings-precedence); para como as chaves de configurações em geral se combinam, consulte [Precedência de configurações](/docs/pt/settings#settings-precedence) no guia de configurações.

<h3 id="useautomodeduringplan">
  `useAutoModeDuringPlan`
</h3>

Escolha se Claude Code usa o classificador do modo automático para revisar comandos de shell no modo de plano. Com o padrão `true`, o classificador revisa cada comando durante o planejamento quando o modo automático está disponível e você não vê nenhum prompt. Defina `false` para obter um prompt de permissão para cada comando fora do conjunto integrado somente leitura. Aparece em `/config` como **Use auto mode during plan**.

* **Escopo**: [`User, local, or managed`](#scopes). Um repositório não pode desativá-lo para você.
* **Tipo**: Booleano
  * `true`: o mesmo que não definido; quando o modo automático está disponível, o classificador revisa cada comando de shell durante o planejamento em vez de solicitar a você. Um `false` em qualquer um desses arquivos ainda o desativa
  * `false`: você obtém um prompt de permissão para cada comando fora do conjunto integrado somente leitura
* **Padrão**: `true`

```json settings.json theme={null}
{
  "useAutoModeDuringPlan": false
}
```

<h3 id="permissions-allow">
  `permissions.allow`
</h3>

Liste os usos de ferramentas que Claude Code aprova sem perguntar a você. Em uma regra MCP, `*` pode aparecer apenas no nome da ferramenta após o prefixo `mcp__<server>__`, como `mcp__github__get_*`; não pode aparecer no nome do servidor.

* **Escopo**: [`Any file`](#scopes)
* **Tipo**: array de strings de regra de permissão
* **Padrão**: não definido
* **Substituições por sessão**: `--allowedTools` adiciona regras de permissão para uma sessão, e uma regra de bloqueio de qualquer arquivo de configurações ainda bloqueia uma ferramenta que ela nomeia

Este exemplo aprova `git diff` e permite que Claude Code leia seu `.zshrc` sem perguntar:

```json settings.json theme={null}
{
  "permissions": {
    "allow": ["Bash(git diff *)", "Read(~/.zshrc)"]
  }
}
```

Claude Code aplica regras `allow` do `.claude/settings.json` de um projeto apenas depois que você aceita o [diálogo de confiança do espaço de trabalho](/docs/pt/permissions#project-allow-rules-and-workspace-trust) para essa pasta.

<h4 id="permission-rule-syntax">
  Sintaxe de regra de permissão
</h4>

As regras de permissão seguem o formato `Tool` ou `Tool(specifier)`. Claude Code avalia as regras `deny` primeiro, depois `ask`, depois `allow`, e a primeira correspondência decide independentemente de quão específica cada regra é; consulte a [ordem de avaliação de regra de permissão](/docs/pt/permissions#manage-permissions).

Cada linha mostra uma forma de regra e o que ela corresponde.

| Regra                          | O que ela corresponde                  |
| :----------------------------- | :------------------------------------- |
| `Bash`                         | Cada comando Bash                      |
| `Bash(npm run *)`              | Comandos começando com `npm run`       |
| `Read(./.env)`                 | Leituras do arquivo `.env`             |
| `WebFetch(domain:example.com)` | Solicitações de busca para example.com |

Para a sintaxe de regra completa, incluindo comportamento de curinga, padrões específicos de ferramenta para Read, Edit, WebFetch, MCP e regras de Agent, e as limitações de segurança de padrões Bash, consulte [Sintaxe de regra de permissão](/docs/pt/permissions#permission-rule-syntax).

<h3 id="permissions-ask">
  `permissions.ask`
</h3>

Liste os usos de ferramentas que solicitam sua confirmação mesmo em um modo de permissão que de outra forma os aprovaria, como `acceptEdits` ou `bypassPermissions`. No modo `dontAsk`, Claude Code nega um uso de ferramenta correspondente em vez de solicitar.

* **Escopo**: [`Any file`](#scopes)
* **Tipo**: array de strings de regra de permissão
* **Padrão**: não definido

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

Liste os usos de ferramentas que Claude Code bloqueia. Use-o para arquivos que contêm chaves de API, segredos ou valores de ambiente: Claude Code exclui arquivos correspondentes da descoberta de arquivos e resultados de pesquisa, nega leituras deles e bloqueia as [ferramentas Edit e Write](/docs/pt/permissions#read-and-edit) nos caminhos correspondentes.

As regras de bloqueio Read e Edit se aplicam às ferramentas de arquivo integradas de Claude, aos comandos de arquivo que Claude Code reconhece em Bash, como `cat`, `head`, `tail`, `sed` e `tee`, e aos destinos de [redirecionamentos](/docs/pt/permissions#redirections) de Bash como `> file` e `< file`; elas não se aplicam a um comando que lê arquivos sem nomeá-los, como `grep -r pattern .`, ou a subprocessos arbitrários, portanto para aplicação em nível de SO [ative a sandbox](/docs/pt/sandboxing).

* **Escopo**: [`Any file`](#scopes)
* **Tipo**: array de strings de regra de permissão
* **Padrão**: não definido
* **Substituições por sessão**: `--disallowedTools` adiciona regras de bloqueio para uma sessão ao lado desta chave

Este exemplo nega leituras de arquivos `.env`, o diretório `secrets` e um arquivo de credenciais, e bloqueia comandos `curl`:

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

Os nomes de ferramentas aceitam padrões glob, portanto `"*"` nega cada ferramenta e `"mcp__*"` nega cada ferramenta MCP. Claude Code ignora uma regra de bloqueio para a ferramenta [`EndConversation`](/docs/pt/tools-reference#endconversation-tool-behavior) enquanto qualquer outra ferramenta ainda estiver disponível para Claude. Uma regra de bloqueio `Bash` corresponde ao comando como Claude o escreve, portanto `Bash(curl *)` não para `/usr/bin/curl` ou `sh -c 'curl …'`; consulte [o que uma regra Bash não corresponde](/docs/pt/permissions#bash-rule-limits). Esta chave substitui a configuração `ignorePatterns` descontinuada.

<h3 id="permissions-additionaldirectories">
  `permissions.additionalDirectories`
</h3>

Dê a Claude acesso a arquivos em diretórios fora do que você começou, como [diretórios de trabalho](/docs/pt/permissions#working-directories) adicionais. A maioria da configuração `.claude/` [não é descoberta](/docs/pt/permissions#additional-directories-grant-file-access-not-configuration) desses diretórios.

* **Escopo**: [`Any file`](#scopes)
* **Tipo**: array de caminhos de diretório
* **Padrão**: não definido
* **Substituições por sessão**: `--add-dir` e `/add-dir` adicionam diretórios para uma sessão ao lado desta chave

```json settings.json theme={null}
{
  "permissions": {
    "additionalDirectories": ["../docs/"]
  }
}
```

Como as regras `allow`, as entradas no `.claude/settings.json` de um projeto entram em vigor apenas depois que você aceita o [diálogo de confiança do espaço de trabalho](/docs/pt/permissions#project-allow-rules-and-workspace-trust) para essa pasta.

<h3 id="permissions-blockreadsoutsideworkingdirectories">
  `permissions.blockReadsOutsideWorkingDirectories`
</h3>

Impeça Claude de ler caminhos fora dos [diretórios de trabalho](/docs/pt/permissions#working-directories) da sessão com as ferramentas Read, Grep, Glob e LSP, em cada modo de permissão incluindo `bypassPermissions`. Um comando Bash que lê um caminho correspondente através de um comando de arquivo que Claude Code reconhece, como `cat`, solicita a você mesmo no modo automático e no modo `bypassPermissions`. Requer Claude Code v2.1.257 ou posterior.

Um comando Bash que o analisador de shell não consegue rastrear, como um que muda de diretório mais de uma vez ou executa um subshell, solicita a você mesmo no modo automático e no modo `bypassPermissions`. O prompt aparece mesmo quando o comando não nomeia nenhum caminho fora dos diretórios de trabalho. Este prompt não se aplica quando o comando é executado na [sandbox](/docs/pt/sandboxing) e a sandbox aplica o bloqueio.

Claude Code também escreve `true` aqui quando você escolhe bloquear tais leituras no [prompt do modo automático antes da primeira leitura fora dos diretórios de trabalho](/docs/pt/permission-modes#first-read-outside-the-working-directories).

* **Escopo**: [`Any file`](#scopes). Se qualquer fonte de configurações definir `true`, o bloqueio se aplica, portanto um arquivo verificado de um repositório pode ativar o bloqueio para um projeto, mas não pode levantar um bloqueio que você definiu.
* **Tipo**: Booleano
  * `true`: leituras de arquivo fora dos diretórios de trabalho são bloqueadas
  * `false`: o mesmo que não definido; um `true` em qualquer outro arquivo de configurações ainda bloqueia
* **Padrão**: não definido, portanto leituras fora dos diretórios de trabalho seguem seu modo de permissão e regras

```json settings.json theme={null}
{
  "permissions": {
    "blockReadsOutsideWorkingDirectories": true
  }
}
```

Se apenas o arquivo de configurações verificado de um repositório adicionar um diretório, o bloqueio ainda se aplica a leituras lá. Quando [`autoMemoryDirectory`](#automemorydirectory) vem do `.claude/settings.json` do projeto, ou de um `.claude/settings.local.json` [tratado como fornecido pelo repositório](/docs/pt/permissions#when-your-local-settings-file-needs-trust), Claude Code não carrega nenhuma [memória automática](/docs/pt/memory#storage-location) desse diretório e não salva nenhuma nele. Os arquivos que Claude Code em si precisa permanecem legíveis, como suas skills, plugins, regras, agents, comandos e o arquivo de memória `CLAUDE.md` sob `~/.claude/`.

Quando a [sandbox](/docs/pt/sandboxing) está ativada, o bloqueio também nega aos comandos em sandbox acesso de leitura a diretórios iniciais e raízes de volume montado fora dos diretórios de trabalho. Uma repetição que precisa de aprovação para [executar fora da sandbox](/docs/pt/sandboxing#the-unsandboxed-retry-escape-hatch) solicita a você mesmo no modo `bypassPermissions`. Os arquivos que uma ferramenta lê do seu diretório inicial, como `~/.gitconfig`, são negados com o resto; reabra um caminho específico com [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread) quando uma ferramenta precisa dele.

Quando o diretório de trabalho da sessão é um [git worktree](/docs/pt/worktrees) vinculado, incluindo um que Claude Code entrou no meio da sessão, o diretório `.git` comum do repositório permanece legível e gravável para comandos em sandbox, para que o git continue funcionando lá.

<h3 id="permissions-defaultmode">
  `permissions.defaultMode`
</h3>

Defina o [modo de permissão](/docs/pt/permission-modes) em que novas sessões começam. Quando você deixa não definido, as sessões começam no [padrão integrado](/docs/pt/permission-modes#which-mode-a-session-starts-in) para seu plano e superfície.

* **Escopo**: [`Any file`](#scopes). `auto` e `bypassPermissions` não entram em vigor a partir de configurações de projeto ou local, portanto defina-os em `~/.claude/settings.json` em vez disso. Antes da v2.1.257, `bypassPermissions` entrava em vigor a partir de qualquer arquivo. Para conversas que a extensão VS Code inicia, Claude Code lê apenas valores de usuário, gerenciados e `--settings`.
* **Tipo**: string, uma de:
  * `"default"`: Claude Code executa apenas leituras sem perguntar
  * `"acceptEdits"`: Claude Code também executa edições de arquivo e comandos comuns do sistema de arquivos como `mkdir` e `mv` sem perguntar
  * `"plan"`: Claude Code lê e planeja, mas bloqueia edições até que você aprove um plano
  * `"auto"`: Claude Code executa tudo, com verificações de segurança em segundo plano
  * `"dontAsk"`: Claude Code nega automaticamente cada chamada que de outra forma solicitaria; leituras, outras ações que não precisam de aprovação e ferramentas pré-aprovadas ainda são executadas
  * `"bypassPermissions"`: Claude Code executa tudo sem perguntar
  * `"manual"`: um alias para `"default"`, em Claude Code v2.1.200 ou posterior
* **Padrão**: não definido
* **Substituições por sessão**: `--permission-mode` e seu equivalente `--dangerously-skip-permissions` para `bypassPermissions` têm precedência sobre esta chave para uma sessão

```json settings.json theme={null}
{
  "permissions": {
    "defaultMode": "acceptEdits"
  }
}
```

As regras de permissão se sobrepõem a cada modo: as regras `deny` bloqueiam em cada modo, incluindo `bypassPermissions`. Consulte [Modos de permissão](/docs/pt/permission-modes). `manual` nomeia o modo de permissão rotulado Manual na CLI e na extensão VS Code; o alias requer Claude Code v2.1.200 ou posterior. Em sessões na nuvem, Claude Code honra apenas `acceptEdits`, `plan`, `default` e `auto` desta chave. Para conversas que a extensão VS Code inicia, consulte [qual configuração a extensão lê para o modo de permissão inicial](/docs/pt/permission-modes#switch-permission-modes).

<h3 id="permissions-disablebypasspermissionsmode">
  `permissions.disableBypassPermissionsMode`
</h3>

Impeça que qualquer pessoa entre no modo `bypassPermissions`. Claude Code então rejeita o sinalizador `--dangerously-skip-permissions` e ignora uma [definição de agent](/docs/pt/sub-agents#permission-modes) `permissionMode: bypassPermissions`, portanto o subagent é executado com o modo de permissão da sessão pai.

* **Escopo**: [`Any file`](#scopes). Normalmente definido em [configurações gerenciadas](/docs/pt/managed-settings) para aplicar a política organizacional.
* **Tipo**: a string `"disable"`
* **Padrão**: não definido
* **Substituições por sessão**: esta chave tem precedência sobre `--dangerously-skip-permissions`, que Claude Code rejeita enquanto a chave está definida

```json settings.json theme={null}
{
  "permissions": {
    "disableBypassPermissionsMode": "disable"
  }
}
```

Antes da v2.1.223, Claude Code aplicava o modo de permissão do frontmatter mesmo com bypass desativado.

<h3 id="skipautopermissionprompt">
  `skipAutoPermissionPrompt`
</h3>

Pule o aviso único descrevendo o [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) que Claude Code mostra quando você entra no modo automático pela primeira vez, por exemplo através de suas próprias configurações ou do seletor de modo, em vez de quando o padrão integrado inicia uma sessão nele. Claude Code mostra esse aviso uma vez e depois registra que foi mostrado, portanto esta chave só importa onde o aviso ainda não apareceu.

* **Escopo**: [`User or managed`](#scopes). Um repositório não pode defini-lo para você.
* **Tipo**: Booleano
  * `true`: Claude Code pula o aviso
  * `false`: o mesmo que não definido; o aviso aparece uma vez a menos que outro desses arquivos defina `true`
* **Padrão**: não definido, portanto o aviso aparece uma vez

```json settings.json theme={null}
{
  "skipAutoPermissionPrompt": true
}
```

<h3 id="skipdangerousmodepermissionprompt">
  `skipDangerousModePermissionPrompt`
</h3>

Pule o diálogo de confirmação que Claude Code mostra antes de uma sessão entrar no modo `bypassPermissions`, seja de `--dangerously-skip-permissions` ou de `defaultMode: "bypassPermissions"`. Claude Code escreve `true` aqui em suas configurações de usuário quando você aceita esse diálogo uma vez.

* **Escopo**: [`User, local, or managed`](#scopes). Um repositório não confiável não pode pular o diálogo para você.
* **Tipo**: Booleano
  * `true`: Claude Code pula o diálogo de confirmação antes de uma sessão entrar no modo `bypassPermissions`
  * `false`: o mesmo que não definido; o diálogo aparece a menos que outro desses arquivos defina `true`
* **Padrão**: não definido, portanto o diálogo aparece

```json settings.json theme={null}
{
  "skipDangerousModePermissionPrompt": true
}
```

<h2 id="sandbox-settings">
  Configurações de sandbox
</h2>

Isole os comandos que Claude executa do seu sistema de arquivos, da sua rede e das suas credenciais. Para saber como o sandboxing funciona e os requisitos de plataforma, consulte [Sandboxing](/docs/pt/sandboxing).

<h3 id="sandbox">
  `sandbox`
</h3>

Isole os comandos Bash que Claude executa do seu sistema de arquivos e rede com [sandboxing](/docs/pt/sandboxing). Ative o sandbox com `enabled`, depois restrinja ou amplie o que os comandos em sandbox podem acessar com os sub-objetos `filesystem`, `network` e `credentials`. O sandbox é executado em macOS, Linux e WSL2.

* **Scope**: [`Any file`](#scopes)
* **Type**: object com `enabled`, `failIfUnavailable`, `autoAllowBashIfSandboxed`, `excludedCommands`, `allowUnsandboxedCommands`, `enableWeakerNestedSandbox`, `enableWeakerNetworkIsolation`, `allowAppleEvents`, `bwrapPath`, `socatPath`, `ignoreViolations` e `ripgrep`, além dos objetos `filesystem`, `network` e `credentials`
* **Default**: não definido, então Claude Code executa comandos sem sandbox

Isso ativa o sandbox, ignora prompts de permissão para comandos em sandbox, executa `docker` fora do sandbox, abre dois caminhos de escrita extras, oculta seu arquivo de credenciais AWS e pré-autoriza GitHub e npm:

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

Claude Code obtém o valor de uma chave booleana do escopo de configurações com maior precedência que a define, então um `enabled` ou `failIfUnavailable` gerenciado substitui qualquer coisa que um desenvolvedor defina. Ele mescla chaves de array em todos os escopos de configurações que a sessão carrega, então um desenvolvedor pode anexar entradas; consulte [Keep developers from widening the policy](/docs/pt/sandboxing#keep-developers-from-widening-the-policy) para os bloqueios somente gerenciados. Para exigir o sandbox para uma organização, consulte [Enforce sandboxing with managed settings](/docs/pt/sandboxing#enforce-sandboxing-with-managed-settings).

<h3 id="sandbox-enabled">
  `sandbox.enabled`
</h3>

Ative [sandboxing](/docs/pt/sandboxing) para comandos Bash. Quando você escolhe um modo no painel `/sandbox`, Claude Code escreve essa chave em `.claude/settings.local.json` para o projeto atual; defina-a em `~/.claude/settings.json` para fazer sandbox em todos os projetos.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code faz sandbox dos comandos Bash
  * `false`: Comandos Bash são executados sem sandbox
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true
  }
}
```

No Linux e WSL2, o sandbox precisa de `bubblewrap` e `socat`; consulte [Set up Linux and WSL2](/docs/pt/sandboxing#set-up-linux-and-wsl2). Quando o sandbox não consegue iniciar, Claude Code mostra um aviso e executa comandos sem sandbox, a menos que você também defina [`failIfUnavailable`](#sandbox-failifunavailable).

<h3 id="sandbox-failifunavailable">
  `sandbox.failIfUnavailable`
</h3>

Faça Claude Code sair com um erro na inicialização quando `sandbox.enabled` é `true` mas o sandbox não consegue iniciar, porque uma dependência está faltando ou a plataforma não é suportada. Sem isso, Claude Code mostra um aviso e executa comandos sem sandbox. Use-o em configurações gerenciadas quando sua organização exigir sandboxing como uma barreira rígida.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code sai com um erro na inicialização quando `sandbox.enabled` é `true` mas o sandbox não consegue iniciar
  * `false`: Claude Code mostra um aviso e executa comandos sem sandbox
* **Default**: `false`

Isso faz com que cada máquina gerenciada faça sandbox dos comandos ou recuse iniciar:

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "failIfUnavailable": true
  }
}
```

Consulte [Enforce sandboxing with managed settings](/docs/pt/sandboxing#enforce-sandboxing-with-managed-settings).

<h3 id="sandbox-autoallowbashifsandboxed">
  `sandbox.autoAllowBashIfSandboxed`
</h3>

Deixe Claude Code executar comandos Bash em sandbox sem um prompt de permissão. Comandos que não conseguem ser executados no sandbox ainda passam pelo fluxo de permissão regular, e regras `deny` e regras `ask` com escopo de conteúdo como `Bash(git push *)` ainda se aplicam; uma regra `ask` Bash simples é ignorada para comandos em sandbox. Defina-o como `false` para enviar comandos em sandbox também pelo fluxo de permissão regular, que a aba **Mode** do `/sandbox` chama modo de permissões regular.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code executa comandos Bash em sandbox sem um prompt de permissão, sujeito a regras `deny` e regras `ask` com escopo de conteúdo; `CLAUDE_CODE_SUBPROCESS_ENV_SCRUB` desativa a auto-autorização
  * `false`: comandos em sandbox passam pelo fluxo de permissão regular, então suas regras de autorização e modo de permissão decidem. A aba **Mode** do `/sandbox` chama isso modo de permissões regular
* **Default**: `true`

Isso mantém o sandbox ativado e envia comandos em sandbox pelo fluxo de permissão regular:

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "autoAllowBashIfSandboxed": false
  }
}
```

Consulte [Sandbox modes](/docs/pt/sandboxing#sandbox-modes) para saber o que o modo auto-autorização ainda solicita e como se comporta em modo de plano.

<h3 id="sandbox-excludedcommands">
  `sandbox.excludedCommands`
</h3>

Nomeie comandos que Claude Code executa fora do sandbox, como ferramentas que não funcionam sob ele. Cada entrada usa a mesma sintaxe do conteúdo de uma [regra de permissão](/docs/pt/permissions#permission-rule-syntax) `Bash(...)`: um comando exato, um prefixo como `docker *` ou um padrão curinga.

Suas entradas tiram uma chamada Bash do sandbox apenas quando cobrem cada comando nela, e algumas formas de chamada permanecem em sandbox mesmo assim. Uma entrada `docker *` sozinha não tira `npm ci && docker build .` do sandbox.

* **Scope**: [`Any file`](#scopes)
* **Type**: array de padrões de comando
* **Default**: não definido, então nenhum comando é excluído

```json settings.json theme={null}
{
  "sandbox": {
    "excludedCommands": ["docker *"]
  }
}
```

Claude Code mantém uma chamada Bash em sandbox quando ela tem uma destas formas, entre outras:

* Um comando começando com `sudo`, `eval` ou `xargs`
* Um `cd`, `pushd` ou `popd`, onde quer que apareça na chamada
* Uma substituição de comando, um subshell ou um bloco de fluxo de controle como `if` ou `for`
* Um redirecionamento, como `docker build . > build.log`, outro que não apenas duplica um descritor de arquivo, como `2>&1` faz
* Um nome de comando que vem de uma variável

Por exemplo, `cd build && docker compose up` permanece em sandbox sob uma entrada `docker *`, e adicionar uma entrada `cd` não muda isso.

Comandos excluídos ainda passam pelo fluxo de permissão regular. Exclusão é uma conveniência, não uma barreira de segurança: prefira [`filesystem.allowWrite`](#sandbox-filesystem-allowwrite) quando uma ferramenta só precisa escrever em algum lugar específico. Claude Code mescla entradas em todos os escopos de configurações que a sessão carrega, e não há bloqueio somente gerenciado para essa lista, então mantenha uma lista gerenciada estreita.

<h3 id="sandbox-allowunsandboxedcommands">
  `sandbox.allowUnsandboxedCommands`
</h3>

Deixe Claude tentar novamente um comando fora do sandbox com o parâmetro `dangerouslyDisableSandbox` depois que o sandbox o bloqueia. Defina-o como `false` para que Claude Code ignore esse parâmetro completamente e cada comando que Claude executa deve estar em sandbox ou aparecer em [`excludedCommands`](#sandbox-excludedcommands). A aba **Overrides** do `/sandbox` mostra esse estado como **Strict sandbox mode**. Use `false` em configurações gerenciadas para políticas que exigem sandboxing rigoroso.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude pode tentar novamente um comando fora do sandbox com o parâmetro `dangerouslyDisableSandbox` depois que o sandbox o bloqueia
  * `false`: Claude Code ignora esse parâmetro, então cada comando que Claude executa está em sandbox ou aparece em `excludedCommands`
* **Default**: `true`

Isso impõe modo de sandbox rigoroso para todos que as configurações gerenciadas cobrem:

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "allowUnsandboxedCommands": false
  }
}
```

Uma tentativa sem sandbox passa pelo fluxo de permissão regular, com um prompt em modo Manual. Consulte [The unsandboxed retry escape hatch](/docs/pt/sandboxing#the-unsandboxed-retry-escape-hatch).

Para ver quando comandos que você digita você mesmo no [prompt de modo shell `!`](/docs/pt/interactive-mode#shell-mode-with-prefix) são executados em sandbox, consulte [strict sandbox mode](/docs/pt/sandboxing#the-unsandboxed-retry-escape-hatch).

<h3 id="sandbox-filesystem">
  `sandbox.filesystem`
</h3>

Controle quais caminhos os comandos em sandbox podem ler e escrever. Por padrão, eles podem escrever no diretório de trabalho, no diretório temporário da sessão e em diretórios que você adiciona com `--add-dir`, `/add-dir` ou `permissions.additionalDirectories`, e podem ler o resto do sistema de arquivos, incluindo arquivos de credenciais. Amplie ou restrinja isso com as quatro listas de caminhos, ou desative a camada do sistema de arquivos com `disabled`. Consulte [Filesystem isolation](/docs/pt/sandboxing#filesystem-isolation) para os limites padrão.

* **Scope**: [`Any file`](#scopes)
* **Type**: object com arrays `allowWrite`, `denyWrite`, `denyRead` e `allowRead`, além dos booleanos `allowManagedReadPathsOnly` e `disabled`
* **Default**: não definido, então os limites padrão de leitura e escrita se aplicam

Isso permite que comandos em sandbox escrevam em um diretório de compilação e seu kubeconfig, e oculta seu arquivo de credenciais AWS:

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

Claude Code impõe essas listas no limite do sandbox do SO, então elas se aplicam a cada subprocesso que um comando em sandbox inicia, como `kubectl`, `terraform` ou `npm`. Claude Code adiciona suas [regras de permissão](/docs/pt/sandboxing#permission-rules) às mesmas listas: regras `Edit` allow e deny para `allowWrite` e `denyWrite`, regras `Read` deny para `denyRead` e regras `WebFetch(domain:...)` allow e deny para as listas de domínio [`network`](#sandbox-network).

A menos que um bloqueio somente gerenciado seja definido, Claude Code mescla cada lista nos arquivos de configurações que a sessão carrega. [`allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly) limita `allowRead` a entradas de configurações gerenciadas, e [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) faz o mesmo para domínios permitidos.

[Configure sandboxing](/docs/pt/sandboxing#configure-sandboxing) cobre fontes que você exclui com `--setting-sources`. Quando você edita uma lista durante uma sessão, Claude Code [aplica a mudança à sessão em execução](/docs/pt/settings#when-edits-take-effect).

<h4 id="sandbox-path-prefixes">
  Prefixos de caminho do sandbox
</h4>

Caminhos em `allowWrite`, `denyWrite`, `denyRead`, `allowRead` e [`credentials.files`](#sandbox-credentials-files) são resolvidos por seu prefixo:

| Prefixo             | Significado                                                                                              | Exemplo                                                                        |
| :------------------ | :------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------- |
| `/`                 | Caminho absoluto da raiz do sistema de arquivos                                                          | `/tmp/build` permanece `/tmp/build`                                            |
| `~/`                | Relativo ao diretório inicial                                                                            | `~/.kube` se torna `$HOME/.kube`                                               |
| `./` ou sem prefixo | Relativo à raiz do projeto para configurações de projeto, ou a `~/.claude` para configurações de usuário | `./output` em `.claude/settings.json` é resolvido para `<project-root>/output` |

O prefixo `//path` para caminhos absolutos também funciona. Se você usar `/path` com barra única esperando resolução relativa ao projeto, mude para `./path`. Essa sintaxe difere das [regras de permissão Read e Edit](/docs/pt/permissions#read-and-edit), que usam `//path` para absoluto e `/path` para relativo ao projeto: caminhos do sistema de arquivos do sandbox usam convenções padrão, então `/tmp/build` é um caminho absoluto.

Claude Code remove uma barra final de um caminho de diretório, então `~/.aws` e `~/.aws/` correspondem ao mesmo diretório. Antes da v2.1.224, Claude Code passava a barra final para o sandbox, e Claude ainda podia ler ou escrever caminhos sob uma entrada `denyRead` ou `denyWrite` escrita com uma.

Claude Code também remove um `/**` final, então `~/build/**` e `~/build` cobrem o mesmo diretório. Se um curinga como `*` funciona depende de qual lista a entrada está e da plataforma:

* **`allowWrite` e `denyWrite`**: em macOS, curingas funcionam. No Linux e WSL2, o sandbox monta caminhos concretos, então Claude Code ignora uma entrada que contém `*`, `?` ou `[` uma vez que o `/**` final é removido, e essa entrada não tem efeito. Claude Code adiciona os caminhos de suas regras de permissão `Edit` a essas listas, então o mesmo limite se aplica a elas, e a aba **Config** do `/sandbox` avisa sobre regras de permissão `Edit` e `Read` que contêm curingas.
* **`denyRead` e `allowRead`**: curingas funcionam em todas as plataformas. No Linux e WSL2, Claude Code expande uma entrada de leitura para os caminhos concretos que ela corresponde, o que não faz para as listas de escrita.

<h3 id="sandbox-filesystem-allowwrite">
  `sandbox.filesystem.allowWrite`
</h3>

Adicione caminhos onde comandos em sandbox podem escrever, além do diretório de trabalho, do diretório temporário da sessão e dos diretórios que você adicionou com `--add-dir`, `/add-dir` ou `permissions.additionalDirectories`. Use-o quando um subprocesso como `kubectl` ou uma ferramenta de compilação precisa escrever fora do projeto.

* **Scope**: [`Any file`](#scopes)
* **Type**: array de strings de caminho, usando os [prefixos de caminho do sandbox](#sandbox-path-prefixes)
* **Default**: não definido, então comandos em sandbox podem escrever no diretório de trabalho, no diretório temporário da sessão, em diretórios que você adicionou com `--add-dir` ou `/add-dir` e em diretórios em [`permissions.additionalDirectories`](#permissions-additionaldirectories)

Isso permite que uma compilação escreva sob `/tmp/build` e deixa `kubectl` atualizar seu kubeconfig:

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "allowWrite": ["/tmp/build", "~/.kube"]
    }
  }
}
```

Claude Code mescla entradas em todos os escopos de configurações que a sessão carrega: caminhos de usuário, projeto, local e gerenciado se combinam em vez de se substituírem, e Claude Code adiciona os caminhos de suas regras de permissão `Edit(...)` allow. Uma entrada `allowWrite` não pode levantar um [caminho protegido](/docs/pt/sandboxing#protected-paths).

<h3 id="sandbox-filesystem-denywrite">
  `sandbox.filesystem.denyWrite`
</h3>

Bloqueie comandos em sandbox de escrever em caminhos específicos, incluindo caminhos dentro de um diretório que é de outra forma gravável.

* **Scope**: [`Any file`](#scopes)
* **Type**: array de strings de caminho, usando os [prefixos de caminho do sandbox](#sandbox-path-prefixes)
* **Default**: não definido

Isso impede que comandos em sandbox alterem a configuração do sistema ou instalem binários:

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyWrite": ["/etc", "/usr/local/bin"]
    }
  }
}
```

Claude Code mescla entradas em todos os escopos de configurações que a sessão carrega e adiciona os caminhos de suas regras de permissão `Edit(...)` deny.

<h3 id="sandbox-filesystem-denyread">
  `sandbox.filesystem.denyRead`
</h3>

Bloqueie comandos em sandbox de ler caminhos específicos, como arquivos de credenciais que a política de leitura padrão exporia de outra forma. Para proteger um arquivo de credenciais e mantê-lo utilizável através do proxy do sandbox, consulte [`sandbox.credentials`](#sandbox-credentials) em vez disso.

* **Scope**: [`Any file`](#scopes)
* **Type**: array de strings de caminho, usando os [prefixos de caminho do sandbox](#sandbox-path-prefixes)
* **Default**: não definido, então comandos em sandbox mantêm o [acesso de leitura padrão](/docs/pt/sandboxing#filesystem-isolation), que inclui arquivos de credenciais como `~/.aws/credentials`

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyRead": ["~/.aws/credentials"]
    }
  }
}
```

Claude Code mescla entradas em todos os escopos de configurações que a sessão carrega e adiciona os caminhos de suas regras de permissão `Read(...)` deny. Quando [`filesystem.disabled`](#sandbox-filesystem-disabled) é `true`, Claude Code não impõe essas entradas.

<h3 id="sandbox-filesystem-allowread">
  `sandbox.filesystem.allowRead`
</h3>

Reabra a leitura para caminhos específicos dentro de uma região que [`denyRead`](#sandbox-filesystem-denyread) bloqueia, para construir acesso de leitura somente do espaço de trabalho. Uma entrada `denyRead` exata ou curinga permanece bloqueada dentro de um `allowRead` mais amplo, como a [tabela de sobreposição](/docs/pt/sandboxing#configure-sandboxing) mostra. Quando uma entrada `denyRead` curinga como `~/**/.env` corresponde a um diretório, Claude Code bloqueia leituras de seu conteúdo também. Antes da v2.1.236 em macOS, Claude Code reabrira os caminhos que uma entrada `denyRead` curinga correspondia onde uma entrada `allowRead` mais ampla as cobria, e deixava o conteúdo de um diretório correspondido legível.

* **Scope**: [`Any file`](#scopes)
* **Type**: array de strings de caminho, usando os [prefixos de caminho do sandbox](#sandbox-path-prefixes)
* **Default**: não definido

Isso bloqueia leituras de seu diretório inicial exceto o projeto em si:

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

Claude Code resolve uma entrada `.` para a raiz do projeto em configurações de projeto e para `~/.claude` em configurações de usuário. Claude Code mescla entradas em todos os arquivos de configurações que a sessão carrega, a menos que [`allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly) seja definido.

<h3 id="sandbox-filesystem-allowmanagedreadpathsonly">
  `sandbox.filesystem.allowManagedReadPathsOnly`
</h3>

Honre apenas as entradas [`allowRead`](#sandbox-filesystem-allowread) que vêm de configurações gerenciadas, para que desenvolvedores não possam reabrir acesso de leitura a caminhos que sua organização bloqueou. Claude Code ainda mescla entradas `denyRead` de todos os escopos de configurações que a sessão carrega.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code honra apenas as entradas `allowRead` de configurações gerenciadas
  * `false`: entradas `allowRead` mesclam de todos os escopos de configurações que a sessão carrega
* **Default**: `false`

Isso bloqueia leituras do diretório inicial, reabre `~/work` e impede que desenvolvedores reabram qualquer outra coisa:

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

Consulte [Keep developers from widening the policy](/docs/pt/sandboxing#keep-developers-from-widening-the-policy).

<h3 id="sandbox-filesystem-disabled">
  `sandbox.filesystem.disabled`
</h3>

Ignore isolamento do sistema de arquivos mantendo isolamento de rede. Comandos em sandbox obtêm acesso irrestrito de leitura e escrita ao sistema de arquivos do host, e sua saída de rede permanece confinada a [`network.allowedDomains`](#sandbox-network-alloweddomains). Use-o quando você faz sandbox para controlar onde os comandos se conectam em vez do que eles escrevem. Requer Claude Code v2.1.216 ou posterior.

* **Scope**: [`User or managed`](#scopes). Quando configurações gerenciadas configuram `sandbox.filesystem` de qualquer forma, ou listam uma entrada `sandbox.credentials.files` com `"mode": "deny"`, apenas configurações gerenciadas podem defini-lo.
* **Type**: Boolean
  * `true`: Claude Code ignora isolamento do sistema de arquivos e mantém isolamento de rede
  * `false`: isolamento do sistema de arquivos permanece ativado
* **Default**: `false`, então isolamento do sistema de arquivos permanece ativado

Isso deixa o sistema de arquivos aberto e confina a saída de rede para GitHub e npm:

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

Com a camada desativada, Claude Code não impõe entradas `denyRead` ou `credentials.files` `deny`, enquanto entradas `credentials.envVars` e entradas `mask` aplicadas continuam funcionando. [`autoAllowBashIfSandboxed`](#sandbox-autoallowbashifsandboxed) ainda padrão para `true`, então defina-o como `false` para continuar solicitando. Consulte [Disable filesystem isolation](/docs/pt/sandboxing#disable-filesystem-isolation) para a lista completa de fontes que podem defini-lo e o que muda quando o isolamento está desativado. Requer Claude Code v2.1.216 ou posterior.

<h3 id="sandbox-ignoreviolations">
  `sandbox.ignoreViolations`
</h3>

Silencie relatórios de violação de sandbox para caminhos que você espera que um comando sonde e seja recusado, como uma ferramenta que verifica `/etc/hosts` na inicialização, para que essas negações não apareçam como violações ou no que Claude vê. O sandbox ainda bloqueia o acesso; apenas o relatório é suprimido. As chaves são substrings para corresponder contra o comando, com `*` correspondendo a cada comando, e os valores são substrings da violação a ignorar para esse comando, como um caminho do sistema de arquivos.

* **Scope**: [`Any file`](#scopes)
* **Type**: object mapeando uma substring de comando para um array de substrings de violação, geralmente caminhos
* **Default**: não definido, então cada violação é relatada

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

Execute o sandbox do Linux dentro de um contêiner Docker sem privilégios, onde bubblewrap não consegue montar um `/proc` fresco. Em vez disso, o sandbox interno faz bind-mount do `/proc` existente do contêiner, o que expõe informações de processo que uma montagem fresca ocultaria. Isso reduz a segurança; use-o apenas quando o contêiner externo já fornece o isolamento que você precisa.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: o sandbox interno faz bind-mount do `/proc` existente do contêiner em vez de montar um fresco
  * `false`: o sandbox monta um `/proc` fresco, o que não funciona em um contêiner Docker sem privilégios
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "enableWeakerNestedSandbox": true
  }
}
```

Apenas Linux e WSL2. Consulte [Bubblewrap fails to start inside a container](/docs/pt/sandboxing#troubleshooting).

<h3 id="sandbox-enableweakernetworkisolation">
  `sandbox.enableWeakerNetworkIsolation`
</h3>

Deixe comandos em sandbox em macOS alcançar o serviço de confiança TLS do sistema, `com.apple.trustd.agent`. Ferramentas baseadas em Go como `gh`, `gcloud` e `terraform` precisam disso para verificar certificados TLS quando você usa [`network.httpProxyPort`](#sandbox-network-httpproxyport) com um proxy MITM e uma CA personalizada. Isso reduz a segurança abrindo um possível caminho de exfiltração de dados através do serviço de confiança.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: comandos em sandbox em macOS podem alcançar `com.apple.trustd.agent`
  * `false`: comandos em sandbox em macOS não podem alcançar o serviço de confiança TLS do sistema
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "enableWeakerNetworkIsolation": true
  }
}
```

Se você não usar um proxy MITM, liste as ferramentas que falham em [`excludedCommands`](#sandbox-excludedcommands) em vez disso; consulte [Go-based CLIs fail TLS verification on macOS](/docs/pt/sandboxing#troubleshooting).

<h3 id="sandbox-allowappleevents">
  `sandbox.allowAppleEvents`
</h3>

Deixe comandos em sandbox em macOS enviar Apple Events, que `open`, `osascript` e ferramentas que abrem URLs em um navegador precisam; sem isso, eles falham com erro `-600`. Isso remove isolamento de execução de código: comandos em sandbox podem iniciar outros aplicativos sem sandbox sem prompt do usuário e podem enviar comandos AppleScript para aplicativos em execução como Terminal, sujeito ao prompt de consentimento de automação por aplicativo do macOS (TCC).

* **Scope**: [`User or managed`](#scopes)
* **Type**: Boolean
  * `true`: comandos em sandbox em macOS podem enviar Apple Events
  * `false`: comandos em sandbox em macOS não podem enviar Apple Events, então `open` e `osascript` falham com erro `-600`
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "allowAppleEvents": true
  }
}
```

Para manter isolamento e ainda executar uma ferramenta assim, adicione-a a [`excludedCommands`](#sandbox-excludedcommands) em vez disso. Consulte [Apple Events on macOS](/docs/pt/sandboxing#security-limitations).

<h3 id="sandbox-ripgrep">
  `sandbox.ripgrep`
</h3>

Aponte o sandbox para um binário ripgrep seu em vez do que Claude Code usa, por exemplo, quando sua plataforma precisa de um `rg` construído diferentemente.

* **Scope**: [`User or managed`](#scopes)
* **Type**: object com `command`, o caminho para o binário ripgrep, e `args` opcional, um array de argumentos para prepender
* **Default**: não definido, então o sandbox usa o mesmo binário ripgrep que Claude Code. Esse é o binário agrupado, a menos que você defina [`USE_BUILTIN_RIPGREP`](/docs/pt/env-vars) para `0`

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

Aponte o sandbox para um binário bubblewrap instalado fora de `PATH`, como uma cópia fornecida em um host isolado. Claude Code usa o caminho tanto para a verificação de dependência de inicialização quanto quando envolve cada comando em sandbox.

* **Scope**: [`Managed`](#scopes). Claude Code o lê apenas de configurações gerenciadas para que um arquivo de usuário, projeto ou local não possa apontar o sandbox para um binário diferente.
* **Type**: string, um caminho absoluto; Claude Code descarta um caminho relativo e volta para busca em `PATH`
* **Default**: não definido, então Claude Code encontra `bwrap` em `PATH`

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "bwrapPath": "/opt/admin/bwrap"
  }
}
```

Apenas Linux e WSL2.

<h3 id="sandbox-socatpath">
  `sandbox.socatPath`
</h3>

Aponte o proxy de rede do sandbox para um binário `socat` instalado fora de `PATH`.

* **Scope**: [`Managed`](#scopes)
* **Type**: string, um caminho absoluto; Claude Code descarta um caminho relativo e volta para busca em `PATH`
* **Default**: não definido, então Claude Code encontra `socat` em `PATH`

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "socatPath": "/opt/admin/socat"
  }
}
```

Apenas Linux e WSL2.

<h3 id="sandbox-credentials">
  `sandbox.credentials`
</h3>

Declare os arquivos de credenciais e variáveis de ambiente para [proteger de comandos em sandbox](/docs/pt/sandboxing#protect-credentials). Cada entrada nomeia um arquivo `path` ou uma variável `name` e um `mode`: `deny` oculta a credencial dentro do sandbox, e `mask` mostra comandos em sandbox um espaço reservado enquanto o [proxy do sandbox](/docs/pt/sandboxing#mask-credentials) substitui o valor real em solicitações de saída. Claude Code protege apenas as entradas que você lista; não há lista de negação de credenciais integrada.

* **Scope**: [`Any file`](#scopes). Claude Code honra entradas `mask`, `allowPlaintextInject`, `awsPairs` e `sigv4` apenas de configurações de usuário, configurações gerenciadas e a flag `--settings`.
* **Type**: object com `files`, `envVars`, `allowPlaintextInject`, `awsPairs` e `sigv4`
* **Default**: não definido, então nenhuma credencial é protegida

Isso oculta seu arquivo de credenciais AWS e remove `GITHUB_TOKEN` de comandos em sandbox:

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

A proteção de arquivo `deny` faz parte da camada do sistema de arquivos, então não se aplica quando você [desativa isolamento do sistema de arquivos](/docs/pt/sandboxing#disable-filesystem-isolation); a proteção de variável de ambiente ainda se aplica.

<h4 id="invalid-credential-entries-in-managed-settings">
  Entradas de credenciais inválidas em configurações gerenciadas
</h4>

Quando uma entrada `sandbox.credentials` gerenciada falha na validação, Claude Code continua protegendo a credencial onde pode:

* Uma entrada em `files` ou `envVars` que ainda tem um `path` ou `name` válido e um `mode` de `mask` ou `deny`, como uma cujo padrão `extract` não tem grupo de captura, é degradada para `mode: "deny"` com um aviso, então a credencial permanece bloqueada, não mascarada, até você corrigir a entrada. Uma entrada `files` degradada fixa [`filesystem.disabled`](#sandbox-filesystem-disabled) como uma entrada `deny` explícita, e o aviso observa que seu bloqueio de leitura não é imposto se configurações gerenciadas desativarem isolamento do sistema de arquivos.
* Uma entrada com um `mode` desconhecido ou um `path` ou `name` inválido é removida.
* Cada caso avisa; se uma entrada é degradada ou removida, as entradas válidas restantes ainda são impostas, e um valor `credentials` totalmente inválido é descartado enquanto o resto de `sandbox` ainda se aplica.

Aplica-se em v2.1.191 e posterior; antes da v2.1.221, cada entrada inválida era removida. Para as outras chaves gerenciadas com manipulação por campo, consulte [Invalid entries in managed settings](/docs/pt/managed-settings#invalid-entries-in-managed-settings).

<h3 id="sandbox-credentials-files">
  `sandbox.credentials.files`
</h3>

Proteja arquivos ou diretórios de credenciais de comandos em sandbox. Com `"mode": "deny"`, Claude Code bloqueia leituras do caminho dentro do sandbox, o mesmo bloqueio de leitura que [`sandbox.filesystem.denyRead`](#sandbox-filesystem-denyread). Com `"mode": "mask"`, comandos em sandbox em Linux e WSL2 leem uma cópia sentinela do arquivo, e o proxy do sandbox substitui o valor real em solicitações de saída para `injectHosts` dessa entrada; em macOS o arquivo é ilegível dentro do sandbox em vez disso. `"mode": "mask"` requer Claude Code v2.1.221 ou posterior.

* **Scope**: [`Any file`](#scopes). Claude Code descarta entradas `mask` de `.claude/settings.json` de projeto e `.claude/settings.local.json` local.
* **Type**: array de objetos, cada um com `path` e um `mode` de `"deny"` ou `"mask"`, mais os [campos mask opcionais para arquivos](#mask-fields-for-files)
* **Default**: não definido, então nenhum arquivo de credenciais é protegido

Isso oculta seu arquivo de credenciais AWS e mascara o arquivo de hosts `gh`, substituindo o valor real apenas em solicitações para `api.github.com`:

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

Caminhos usam os mesmos [prefixos](#sandbox-path-prefixes) que as configurações `sandbox.filesystem.*`, e Claude Code mescla os arrays de todos os escopos de configurações que a sessão carrega. [Protect credentials](/docs/pt/sandboxing#protect-credentials) cobre o que ainda se aplica de fontes que você exclui com `--setting-sources`. `mask` entradas requerem Claude Code v2.1.221 ou posterior.

A substituição `mask` é executada apenas através do proxy do sandbox, então defina [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate) ou [`allowPlaintextInject`](#sandbox-credentials-allowplaintextinject) para redes de teste HTTP simples. `mask` se aplica a um único arquivo, então liste cada arquivo de credenciais individualmente. Claude Code aceita mas ignora os campos `mask` em uma entrada `deny`. [Mask credential files](/docs/pt/sandboxing#mask-credential-files) cobre quais fontes de configurações são honradas e quando uma entrada volta para `deny`.

<span id="sandbox-credentials-files-extract" />

<span id="sandbox-credentials-files-onextractnomatch" />

<span id="sandbox-credentials-files-decode" />

<span id="sandbox-credentials-files-maskclaims" />

<span id="sandbox-credentials-files-maskduplicates" />

<span id="sandbox-credentials-files-injecthosts" />

<h4 id="mask-fields-for-files">
  Campos mask para arquivos
</h4>

Uma entrada `mask` aceita esses campos opcionais. Sem `extract` ou `decode`, Claude Code substitui todo o conteúdo do arquivo por um sentinela. Em macOS com isolamento do sistema de arquivos ativado, Claude Code aplica uma entrada `mask` como `deny` antes de `extract` ou `decode` ser executado; consulte [Mask credential files](/docs/pt/sandboxing#mask-credential-files).

| Campo              | Tipo                                                                                                                    | O que faz                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| :----------------- | :---------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `extract`          | string, uma expressão regular com pelo menos um grupo de captura                                                        | Mascara apenas o texto capturado pelo grupo 1 de cada correspondência, então o resto do arquivo permanece analisável. Com `decode` também definido, Claude Code verifica cada captura como um JWT possível em vez de substituí-lo imediatamente. Requer v2.1.221 ou posterior                                                                                                                                                                                                                                                                                                                                             |
| `onExtractNoMatch` | `"warn"`, `"deny"` ou `"error"`; padrão `"warn"`                                                                        | O que acontece quando `extract` ou `decode` não encontra nada para mascarar. `warn` deixa o arquivo legível como está dentro do sandbox, `deny` o torna ilegível e `error` interrompe a configuração do sandbox até você corrigir a configuração. Claude Code trata `deny` como `error` quando o bloqueio de leitura não seria imposto, porque você [desativa isolamento do sistema de arquivos](/docs/pt/sandboxing#disable-filesystem-isolation) ou uma entrada [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread) reabre o caminho. Requer v2.1.221 ou posterior; o caso `decode` requer v2.1.224 ou posterior |
| `decode`           | a string `"jwt"`                                                                                                        | Encontre JSON Web Tokens (JWTs) no arquivo, com um padrão integrado ou com `extract` quando definido, verifique cada candidato e substitua-o por um token falso estruturalmente válido, então código dentro do sandbox que decodifica o token continua funcionando. Quando nenhum candidato verifica, `onExtractNoMatch` governa o resultado. Requer v2.1.224 ou posterior                                                                                                                                                                                                                                                |
| `maskClaims`       | array de strings, pelo menos um nome de claim; requer `decode`                                                          | Mascara apenas os claims de carga útil de nível superior nomeados dentro de cada JWT verificado e reconstrói o token ao redor da carga útil modificada, então os outros claims permanecem legíveis. Quando nenhum claim nomeado corresponde, `onExtractNoMatch` governa o resultado. Requer v2.1.224 ou posterior                                                                                                                                                                                                                                                                                                         |
| `maskDuplicates`   | Boolean, padrão `false`                                                                                                 | Também substitua cópias verbatim de cada valor mascarado em outro lugar no arquivo, como um segredo colado em um comentário. Claude Code corresponde substrings brutas, então reserve-o para segredos longos e de alta entropia. Consultado apenas quando `extract` ou `decode` está definido. Requer v2.1.221 ou posterior                                                                                                                                                                                                                                                                                               |
| `injectHosts`      | array de strings, cada um um host que [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) também admite | Restrinja os hosts onde o proxy do sandbox substitui o valor real. Quando não definido, o proxy o substitui em solicitações para cada host em `sandbox.network.allowedDomains`. Requer v2.1.221 ou posterior                                                                                                                                                                                                                                                                                                                                                                                                              |

Isso mascara apenas o valor `oauth_token` no arquivo de hosts `gh`, substitui cada outra cópia dele no arquivo, torna o arquivo ilegível se o padrão não corresponder a nada e substitui o token real apenas em solicitações para `api.github.com`:

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

Proteja variáveis de ambiente de comandos em sandbox. Com `"mode": "deny"`, Claude Code remove a variável do ambiente de comandos em sandbox. Com `"mode": "mask"`, comandos em sandbox veem um valor sentinela por sessão, e o proxy do sandbox substitui o valor real em solicitações de saída para `injectHosts` dessa entrada, então ferramentas como `gh` e `npm` continuam autenticando sem nunca manter a credencial real. `"mode": "mask"` requer Claude Code v2.1.199 ou posterior.

* **Scope**: [`Any file`](#scopes). Claude Code descarta entradas `mask` de `.claude/settings.json` de projeto e `.claude/settings.local.json` local.
* **Type**: array de objetos, cada um com `name` e um `mode` de `"deny"` ou `"mask"`, mais os [campos mask opcionais para variáveis de ambiente](#mask-fields-for-environment-variables)
* **Default**: não definido, então nenhuma variável de ambiente é protegida

Isso remove `NPM_TOKEN` de comandos em sandbox e mascara `GITHUB_TOKEN`, substituindo o valor real apenas em solicitações para `api.github.com`:

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

O `name` deve começar com uma letra ou sublinhado e conter apenas letras, dígitos e sublinhados. Claude Code mescla os arrays de todos os escopos de configurações que a sessão carrega e aplica `deny` quando a mesma variável aparece com ambos os modos. [Protect credentials](/docs/pt/sandboxing#protect-credentials) cobre o que ainda se aplica de fontes que você exclui com `--setting-sources`. `mask` entradas requerem Claude Code v2.1.199 ou posterior.

A substituição `mask` é executada apenas através do proxy do sandbox, então defina [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate) ou [`allowPlaintextInject`](#sandbox-credentials-allowplaintextinject) para redes de teste HTTP simples; consulte [Mask environment variables](/docs/pt/sandboxing#mask-environment-variables). Claude Code aceita mas ignora os campos `mask` em uma entrada `deny`.

<span id="sandbox-credentials-envvars-extract" />

<span id="sandbox-credentials-envvars-onextractnomatch" />

<span id="sandbox-credentials-envvars-decode" />

<span id="sandbox-credentials-envvars-maskclaims" />

<span id="sandbox-credentials-envvars-injecthosts" />

<h4 id="mask-fields-for-environment-variables">
  Campos mask para variáveis de ambiente
</h4>

Uma entrada `mask` aceita esses campos opcionais. Sem `extract` ou `decode`, Claude Code substitui todo o valor por um sentinela. `extract` e `decode` não podem ser combinados na mesma entrada.

| Campo              | Tipo                                                                                                                    | O que faz                                                                                                                                                                                                                                                                                                                                                                                                      |
| :----------------- | :---------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `extract`          | string, uma expressão regular com pelo menos um grupo de captura                                                        | Mascara apenas o texto capturado pelo grupo 1 de cada correspondência, como a senha dentro de uma string de conexão `DATABASE_URL`, então o resto do valor permanece analisável. Requer v2.1.224 ou posterior                                                                                                                                                                                                  |
| `onExtractNoMatch` | `"warn"`, `"deny"` ou `"error"`; padrão `"warn"`. Em uma entrada com `decode`, apenas `"warn"` é aceito                 | O que acontece quando `extract` não corresponde a nada. `warn` passa a variável através desmascarada, `deny` a desdefine dentro do sandbox e `error` interrompe a configuração do sandbox até você corrigir a configuração. Requer v2.1.224 ou posterior                                                                                                                                                       |
| `decode`           | a string `"jwt"`                                                                                                        | Verifique se o valor inteiro é um JWT e substitua-o por um token falso estruturalmente válido, então código dentro do sandbox que decodifica o token continua funcionando; o proxy substitui o token real inteiro na saída. Um valor que não verifica passa através desmascarado com um aviso. Requer v2.1.224 ou posterior                                                                                    |
| `maskClaims`       | array de strings, pelo menos um nome de claim; requer `decode`                                                          | Mascara apenas os claims de carga útil de nível superior nomeados dentro do JWT decodificado e reconstrói o token ao redor da carga útil modificada, então os outros claims permanecem legíveis. Quando nenhum claim nomeado corresponde, a variável passa através desmascarada com um aviso. Requer v2.1.224 ou posterior                                                                                     |
| `injectHosts`      | array de strings, cada um um host que [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) também admite | Restrinja os hosts onde o proxy do sandbox substitui o valor real. Quando não definido, o proxy o substitui em solicitações para cada host em `sandbox.network.allowedDomains`. Escreva um destino IPv6 como o endereço comprimido nu, como `"::1"`, não a forma entre colchetes; consulte [IPv6 destinations in `injectHosts`](/docs/pt/sandboxing#ipv6-destinations-in-injecthosts). Requer v2.1.199 ou posterior |

Isso mascara apenas a senha dentro de `DATABASE_URL`, desdefine a variável se o padrão não corresponder a nada e mascara um JWT em `SERVICE_JWT` enquanto deixa cada claim exceto `api_key` legível:

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

Permita substituição `mask` em solicitações HTTP simples bem como HTTPS terminado em TLS. Em HTTP simples a identidade upstream não é verificada e a credencial viaja em texto claro, então deixe isso desativado fora de redes de teste confiáveis. Requer Claude Code v2.1.199 ou posterior.

* **Scope**: [`User or managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code permite substituição `mask` em solicitações HTTP simples bem como HTTPS terminado em TLS
  * `false`: Claude Code permite substituição `mask` apenas em HTTPS terminado em TLS
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

Requer Claude Code v2.1.199 ou posterior.

<h3 id="sandbox-credentials-awspairs">
  `sandbox.credentials.awsPairs`
</h3>

Agrupe variáveis de ambiente mascaradas que formam uma credencial AWS para [re-assinatura SigV4](/docs/pt/sandboxing#re-sign-aws-requests) quando sua credencial vive em variáveis com nomes não padrão. Claude Code vincula o trio convencional `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` e `AWS_SESSION_TOKEN` automaticamente quando você mascara seus valores inteiros, então você precisa dessa chave apenas para outros nomes. Requer Claude Code v2.1.224 ou posterior.

* **Scope**: [`User or managed`](#scopes)
* **Type**: array de objetos, cada um com `accessKeyIdVar`, `secretAccessKeyVar` e opcionalmente `sessionTokenVar`, nomeando entradas `sandbox.credentials.envVars`
* **Default**: não definido, então apenas o trio convencional é emparelhado

Isso vincula três variáveis com nomes personalizados em uma credencial AWS para re-assinatura:

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

Cada variável nomeada deve ser uma entrada `mask` de valor inteiro em [`sandbox.credentials.envVars`](#sandbox-credentials-envvars), sem `extract` ou `decode`, e pode preencher apenas um slot em todos os pares.

<h3 id="sandbox-credentials-sigv4">
  `sandbox.credentials.sigv4`
</h3>

Escolha o que o proxy do sandbox faz com formas de solicitação AWS que [não consegue re-assinar](/docs/pt/sandboxing#re-sign-aws-requests): `streaming` para uploads de streaming aws-chunked, `presigned` para URLs pré-assinadas e `sigv4a` para assinaturas assimétricas SigV4A. Isso se aplica apenas a solicitações assinadas com a ID de chave de acesso de espaço reservado de um par mascarado. Requer Claude Code v2.1.224 ou posterior.

* **Scope**: [`User or managed`](#scopes)
* **Type**: object com `streaming`, `presigned` e `sigv4a`, cada um de:
  * `"deny"`: o proxy falha na solicitação
  * `"passthrough"`: o proxy encaminha a solicitação assinada com o espaço reservado mascarado, então a ferramenta recebe a própria rejeição da AWS
* **Default**: não definido, então cada forma é `"deny"`

Isso encaminha uploads de streaming em vez de falhá-los no proxy:

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

Com `deny`, o proxy falha na solicitação. Com `passthrough`, o proxy encaminha a solicitação com sua assinatura computada a partir do espaço reservado mascarado, então AWS a rejeita e a ferramenta chamadora recebe a própria resposta da AWS em vez de um erro de proxy.

<h3 id="sandbox-network">
  `sandbox.network`
</h3>

Controle quais hosts, portas e sockets os comandos em sandbox podem alcançar. O sandbox roteia o tráfego de saída através de um proxy que impõe essas listas; consulte [Network isolation](/docs/pt/sandboxing#network-isolation) para saber como o proxy decide e quando solicita.

* **Scope**: [`Any file`](#scopes). `strictAllowlist`, `allowManagedDomainsOnly` e `tlsTerminate` são lidos de menos fontes, como suas entradas dizem.
* **Type**: object com as sub-chaves abaixo
* **Default**: não definido, então nenhum domínio é pré-autorizado e o sandbox solicita cada novo host

Isso pré-autoriza GitHub e npm, bloqueia `uploads.github.com` e deixa comandos se vincularem a localhost:

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

Claude Code mescla as sub-chaves de array em escopos de configurações e as deduplica, então um projeto pode adicionar domínios à sua lista de usuário. As regras de permissão `WebFetch(domain:...)` allow e deny [permission rules](/docs/pt/sandboxing#permission-rules) alimentam as mesmas listas de allow e deny.

<h3 id="sandbox-network-allowunixsockets">
  `sandbox.network.allowUnixSockets`
</h3>

Liste os caminhos de socket Unix que comandos em sandbox podem se conectar em macOS. Claude Code ignora essa lista no Linux e WSL2, onde o filtro seccomp não consegue inspecionar caminhos de socket; use [`allowAllUnixSockets`](#sandbox-network-allowallunixsockets) em vez disso.

* **Scope**: [`Any file`](#scopes)
* **Type**: array de strings, cada um um caminho de socket
* **Default**: não definido, então o sandbox macOS bloqueia cada socket Unix

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowUnixSockets": ["~/.ssh/agent-socket"]
    }
  }
}
```

Um caminho de socket pode conceder acesso amplo: permitir `/var/run/docker.sock`, por exemplo, deixa um comando em sandbox controlar o daemon Docker. Consulte [Security limitations](/docs/pt/sandboxing#security-limitations).

<h3 id="sandbox-network-allowallunixsockets">
  `sandbox.network.allowAllUnixSockets`
</h3>

Deixe comandos em sandbox se conectarem a cada socket Unix. No Linux e WSL2, o [filtro seccomp](/docs/pt/sandboxing#set-up-linux-and-wsl2) do sandbox bloqueia chamadas `socket(AF_UNIX, ...)`, então essa é a única maneira de permitir sockets Unix lá. Quando o filtro está faltando, que `/sandbox` relata em sua aba Dependencies, o sandbox não bloqueia chamadas de socket Unix. Consulte [Set up Linux and WSL2](/docs/pt/sandboxing#set-up-linux-and-wsl2) para onde o filtro vem.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: comandos em sandbox podem se conectar a cada socket Unix
  * `false`: o sandbox bloqueia conexões de socket Unix: em macOS exceto os caminhos em `allowUnixSockets` e no Linux e WSL2 através do filtro seccomp quando está presente
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

No WSL2, `true` também reabre o socket interop que inicia binários Windows como `cmd.exe` e `powershell.exe`.

<h3 id="sandbox-network-allowlocalbinding">
  `sandbox.network.allowLocalBinding`
</h3>

Deixe comandos em sandbox se vincularem a portas localhost em macOS, por exemplo, para iniciar um servidor de desenvolvimento.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: comandos em sandbox podem se vincular a portas localhost em macOS
  * `false`: comandos em sandbox em macOS não podem se vincular a portas localhost
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

Liste nomes de serviço XPC e Mach adicionais que o sandbox macOS pode procurar. Ferramentas que se comunicam sobre XPC, como o iOS Simulator ou Playwright, precisam de seus serviços listados aqui.

* **Scope**: [`Any file`](#scopes)
* **Type**: array de strings, cada um um nome de serviço; um único `*` final corresponde a um prefixo e `"*"` sozinho corresponde a cada serviço
* **Default**: não definido

Isso permite cada serviço sob o prefixo `com.apple.coresimulator.`:

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

Pré-autorize domínios para tráfego de saída de comandos em sandbox, então o sandbox não solicita por eles. Curingas como `*.example.com` correspondem a subdomínios, e um sufixo `:port` opcional limita uma entrada a uma porta; uma entrada sem porta corresponde a cada porta.

* **Scope**: [`Any file`](#scopes). Apenas configurações gerenciadas quando [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) está definido.
* **Type**: array de strings, cada um um domínio, padrão curinga ou literal IP, com um sufixo `:port` opcional
* **Default**: não definido, então o sandbox solicita a primeira vez que um comando alcança um novo host

Isso pré-autoriza GitHub em cada porta, cada subdomínio npm e um host API em porta 443 apenas:

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org", "api.example.com:443"]
    }
  }
}
```

Escreva literais IPv6 entre colchetes, com uma porta opcional: `"[::1]"` permite cada porta e `"[::1]:443"` uma porta. A forma entre colchetes requer Claude Code v2.1.229 ou posterior. Consulte [IPv6 addresses in domain lists](/docs/pt/sandboxing#ipv6-addresses-in-domain-lists).

<h3 id="sandbox-network-denieddomains">
  `sandbox.network.deniedDomains`
</h3>

Bloqueie domínios para tráfego de saída de comandos em sandbox, usando a mesma sintaxe de curinga, porta e IPv6 que [`allowedDomains`](#sandbox-network-alloweddomains). Um domínio negado permanece bloqueado mesmo quando uma entrada `allowedDomains` também o corresponde.

* **Scope**: [`Any file`](#scopes)
* **Type**: array de strings, cada um um domínio, padrão curinga ou literal IP, com um sufixo `:port` opcional
* **Default**: não definido

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "deniedDomains": ["sensitive.cloud.example.com"]
    }
  }
}
```

Claude Code mescla essa lista de cada fonte de configurações que a sessão carrega mesmo quando `allowManagedDomainsOnly` está definido, então um desenvolvedor pode sempre apertar a lista de negação. Para literais IPv6, consulte [IPv6 addresses in domain lists](/docs/pt/sandboxing#ipv6-addresses-in-domain-lists).

Uma entrada escrita com o ponto final que marca um nome de domínio totalmente qualificado, como `example.com.`, bloqueia as mesmas conexões que `example.com`.

<h3 id="sandbox-network-strictallowlist">
  `sandbox.network.strictAllowlist`
</h3>

Negue acesso de comandos em sandbox a hosts fora da lista de permissões em vez de solicitar aprovação. A lista de permissões é [`allowedDomains`](#sandbox-network-alloweddomains) mais domínios de regras `WebFetch(domain:...)` allow, ou apenas as entradas de configurações gerenciadas quando [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) está definido. Requer Claude Code v2.1.219 ou posterior.

* **Scope**: [`User or managed`](#scopes). Um repositório não pode ativá-lo ou desativá-lo.
* **Type**: Boolean
  * `true`: Claude Code nega acesso de comandos em sandbox a hosts fora da lista de permissões
  * `false`: a menos que outro arquivo de configurações confiável defina `true`, Claude Code decide um host fora da lista de permissões por modo de permissão em vez de negá-lo imediatamente: em modo auto ele verifica o host contra os [domínios permitidos por comando](/docs/pt/sandboxing#per-command-allowed-domains-in-auto-mode) do comando, em modo `dontAsk` nega, em modo `bypassPermissions` e em sessões de modo de plano de terminal interativo onde bypass está disponível permite, e caso contrário pergunta a você
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

Claude Code impõe isso apenas para comandos em sandbox; ferramentas em processo como `WebFetch` ainda seguem suas [regras de permissão](/docs/pt/sandboxing#permission-rules). Quando qualquer uma das fontes honradas a define como `true`, ela permanece ativada. Consulte [Network isolation](/docs/pt/sandboxing#network-isolation). Requer Claude Code v2.1.219 ou posterior.

<h3 id="sandbox-network-allowmanageddomainsonly">
  `sandbox.network.allowManagedDomainsOnly`
</h3>

Bloqueie a lista de permissões de rede para o que as configurações gerenciadas definem. Claude Code então honra apenas `allowedDomains` e regras `WebFetch(domain:...)` allow de configurações gerenciadas, ignora domínios de configurações de usuário, projeto, local e `--settings`, e bloqueia um domínio não permitido automaticamente em vez de solicitar.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code honra apenas `allowedDomains` e regras `WebFetch(domain:...)` allow de configurações gerenciadas e bloqueia um domínio não permitido em vez de solicitar
  * `false`: domínios de configurações de usuário, projeto, local e `--settings` mesclam na lista de permissões
* **Default**: `false`

Isso bloqueia a lista de permissões para GitHub e npm e ignora qualquer domínio que desenvolvedores adicionem:

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

Domínios negados ainda mesclam de cada fonte que a sessão carrega. Consulte [Keep developers from widening the policy](/docs/pt/sandboxing#keep-developers-from-widening-the-policy).

<h3 id="sandbox-network-httpproxyport">
  `sandbox.network.httpProxyPort`
</h3>

Aponte o sandbox para seu próprio proxy HTTP em vez do que Claude Code executa. Organizações fazem isso para inspecionar tráfego HTTPS, aplicar suas próprias regras de filtragem ou registrar cada solicitação. Quando não definido, Claude Code inicia seu próprio proxy para tráfego HTTP.

* **Scope**: [`Any file`](#scopes)
* **Type**: number, uma porta TCP local
* **Default**: não definido, então Claude Code executa seu próprio proxy

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "httpProxyPort": 8080
    }
  }
}
```

Defina [`socksProxyPort`](#sandbox-network-socksproxyport) também se seu proxy deve carregar tráfego SOCKS também; com apenas um dos dois definido, Claude Code ainda executa seu próprio proxy para o outro protocolo. Consulte [Custom proxy configuration](/docs/pt/sandboxing#custom-proxy-configuration).

<h3 id="sandbox-network-socksproxyport">
  `sandbox.network.socksProxyPort`
</h3>

Aponte o sandbox para seu próprio proxy SOCKS5 em vez do que Claude Code executa. Quando não definido, Claude Code inicia seu próprio proxy para tráfego SOCKS.

* **Scope**: [`Any file`](#scopes)
* **Type**: number, uma porta TCP local
* **Default**: não definido, então Claude Code executa seu próprio proxy

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "socksProxyPort": 8081
    }
  }
}
```

Consulte [Custom proxy configuration](/docs/pt/sandboxing#custom-proxy-configuration).

<h3 id="sandbox-network-tlsterminate">
  `sandbox.network.tlsTerminate`
</h3>

Faça o proxy do sandbox terminar TLS para que ele possa ler o conteúdo de solicitações HTTPS. Isso é experimental, e a [substituição de credenciais](/docs/pt/sandboxing#mask-credentials) `mask` requer isso. Defina `{}` para gerar uma autoridade de certificado efêmera para a sessão, ou defina `caCertPath` e `caKeyPath` para usar a sua própria.

* **Scope**: [`User or managed`](#scopes). Um repositório não pode ativá-lo ou fornecer uma autoridade de certificado.
* **Type**: object com `caCertPath` e `caKeyPath` strings opcionais, cada um um caminho de arquivo
* **Default**: não definido, então o proxy não termina ou inspeciona TLS

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "tlsTerminate": {}
    }
  }
}
```

Quando mais de uma fonte honrada a define, Claude Code usa o valor da fonte com maior precedência: configurações gerenciadas, depois a flag `--settings`, depois configurações de usuário. Requer Claude Code v2.1.199 ou posterior.

<span id="context-and-memory" />

<h2 id="memory-and-context">
  Memória e contexto
</h2>

Controle o que Claude Code carrega no contexto, como ele compacta e onde mantém memória e planos. Veja [Gerenciar contexto](/docs/pt/context-window) e [Memória](/docs/pt/memory).

<h3 id="autocompactenabled">
  `autoCompactEnabled`
</h3>

Faça Claude Code [compactar a conversa automaticamente](/docs/pt/context-window#when-your-context-fills-up) quando o contexto se aproximar do limite. Aparece em `/config` como **Auto-compact**, e alternar lá escreve essa chave nas suas configurações de usuário.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code compacta a conversa automaticamente quando o contexto se aproxima do limite
  * `false`: Claude Code não compacta automaticamente
* **Default**: `true`
* **Per-session overrides**: [`DISABLE_AUTO_COMPACT`](/docs/pt/env-vars) desativa auto-compact para uma sessão; qualquer um dos dois que o desativar, o outro não pode reativá-lo

```json settings.json theme={null}
{
  "autoCompactEnabled": false
}
```

O comando manual `/compact` continua funcionando enquanto auto-compact está desativado.

<h3 id="autocompactwindow">
  `autoCompactWindow`
</h3>

Defina o quão cheio o contexto fica antes de Claude Code [compactar automaticamente](/docs/pt/context-window#when-your-context-fills-up).

* **Scope**: [`Any file`](#scopes)
* **Type**: número de tokens, de `100000` a `1000000`. Claude Code limita o valor à janela de contexto do seu modelo; a [visão geral de modelos](https://platform.claude.com/docs/en/about-claude/models/overview) lista a janela de cada modelo
* **Default**: não definido, então Claude Code escolhe uma janela ajustada para seu modelo
* **Per-session overrides**: [`--autocompact`](/docs/pt/cli-reference#cli-flags) tem precedência sobre essa chave para uma sessão, e [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/pt/env-vars) tem precedência sobre ambas

```json settings.json theme={null}
{
  "autoCompactWindow": 500000
}
```

Defina com o comando [`/autocompact`](/docs/pt/commands#all-commands), que escreve essa chave nas suas configurações de usuário. [Defina a janela auto-compact](/docs/pt/model-config#set-the-auto-compact-window) cobre como o comando, flag, variável e configuração interagem.

<h3 id="automemorydirectory">
  `autoMemoryDirectory`
</h3>

Armazene [memória automática](/docs/pt/memory#storage-location) em um diretório de sua escolha em vez do padrão por projeto.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, um caminho de diretório absoluto ou prefixado com `~/`
* **Default**: não definido, então Claude Code usa `~/.claude/projects/<project>/memory/`

```json settings.json theme={null}
{
  "autoMemoryDirectory": "~/my-memory-dir"
}
```

A partir das configurações de projeto ou local, Claude Code honra essa chave sob a mesma [regra de confiança de workspace que hooks](/docs/pt/permissions#what-runs-before-you-trust-a-folder), já que um repositório clonado pode fornecer esses arquivos.

<h3 id="automemoryenabled">
  `autoMemoryEnabled`
</h3>

Ative ou desative [memória automática](/docs/pt/memory#enable-or-disable-auto-memory). Quando `false`, Claude não lê ou escreve no diretório de memória automática. Você também pode alternar com `/memory` durante uma sessão, que escreve essa chave nas suas configurações de usuário.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: o mesmo que não definido; memória automática permanece ativada a menos que algo que tenha precedência sobre essa chave a desative para a sessão, como `--bare`, modo seguro ou `CLAUDE_CODE_DISABLE_AUTO_MEMORY`
  * `false`: Claude não lê ou escreve no diretório de memória automática
* **Default**: `true`
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_AUTO_MEMORY`](/docs/pt/env-vars) tem precedência sobre essa chave para uma sessão, em qualquer direção

```json settings.json theme={null}
{
  "autoMemoryEnabled": false
}
```

<h3 id="bashoutputmaxchars">
  `bashOutputMaxChars`
</h3>

Defina quantos caracteres da saída de um comando Bash ou PowerShell bem-sucedido [Claude recebe inline](/docs/pt/tools-reference#output-limits). Quando a saída passa do limite, Claude Code a salva em um arquivo e Claude recebe uma visualização curta mais o caminho do arquivo. Aumente o limite quando a saída do comando, como uma compilação detalhada ou um log completo de suite de testes, rotineiramente ultrapassa o padrão e você quer que Claude a leia sem abrir o arquivo. Requer Claude Code v2.1.261 ou posterior.

* **Scope**: [`Any file`](#scopes)
* **Type**: número de caracteres, um inteiro positivo. Claude Code limita o valor ao intervalo `4000` a `128000`
* **Default**: não definido, então Claude recebe até 30.000 caracteres inline

```json settings.json theme={null}
{
  "bashOutputMaxChars": 100000
}
```

Quando você define essa chave, Claude Code ignora a variável de ambiente [`BASH_MAX_OUTPUT_LENGTH`](/docs/pt/env-vars).

<h3 id="claudemd">
  `claudeMd`
</h3>

Injete instruções no estilo CLAUDE.md como memória gerenciada pela organização sem implantar um arquivo separado. Claude Code carrega o texto como uma entrada de memória gerenciada antes dos arquivos CLAUDE.md de usuário e projeto.

* **Scope**: [`Managed`](#scopes)
* **Type**: string, o texto de um arquivo CLAUDE.md; escreva como você faria o arquivo, Markdown incluído, com quebras de linha como `\n`
* **Default**: não definido

Este exemplo implanta duas regras como uma lista Markdown curta:

```json managed-settings.json theme={null}
{
  "claudeMd": "# Engineering rules\n\n- Always run make lint before committing.\n- Never push directly to main."
}
```

Veja [Implante CLAUDE.md em toda a organização](/docs/pt/memory#deploy-organization-wide-claude-md).

<h3 id="claudemdexcludes">
  `claudeMdExcludes`
</h3>

Pule arquivos `CLAUDE.md` específicos quando Claude Code carrega [memória](/docs/pt/memory#exclude-specific-claude-md-files). Em um grande monorepo, use para pular arquivos CLAUDE.md de outros times que não são relevantes para seu trabalho; [Exclua arquivos CLAUDE.md irrelevantes](/docs/pt/large-codebases#exclude-irrelevant-claude-md-files) no guia de grandes bases de código percorre esse caso. Padrões correspondem a caminhos de arquivo absolutos.

* **Scope**: [`Any file`](#scopes)
* **Type**: array de strings, cada uma um padrão glob ou caminho absoluto
* **Default**: não definido, então Claude Code carrega cada CLAUDE.md que encontra

```json settings.json theme={null}
{
  "claudeMdExcludes": ["**/vendor/**/CLAUDE.md"]
}
```

Exclusões se aplicam apenas a arquivos de memória de usuário, projeto e local; arquivos CLAUDE.md de política gerenciada não podem ser excluídos.

<span id="environment-variables" />

<h3 id="env">
  `env`
</h3>

Defina variáveis de ambiente para cada sessão e para os subprocessos que Claude Code inicia a partir dela. A maioria das variáveis na [referência de variáveis de ambiente](/docs/pt/env-vars) pode ir aqui, que é como você aplica uma a cada sessão ou a implanta para seu time. Configurações de projeto e local não podem definir [algumas delas](#variables-claude-code-ignores-in-env).

* **Scope**: [`Any file`](#scopes)
* **Type**: objeto mapeando nomes de variáveis para valores de string
* **Default**: não definido

Este exemplo desativa compactação automática e roteia solicitações de API através de um proxy:

```json settings.json theme={null}
{
  "env": {
    "DISABLE_AUTO_COMPACT": "1",
    "ANTHROPIC_BASE_URL": "https://proxy.example.com"
  }
}
```

<h4 id="how-env-values-interact-with-your-shell">
  Como valores `env` interagem com seu shell
</h4>

* Um valor aqui sobrescreve a mesma variável exportada em seu shell, e quando mais de um arquivo de configurações define uma variável, a [precedência mais alta](/docs/pt/settings#settings-precedence) se aplica. [Variáveis que Claude Code ignora em `env`](#variables-claude-code-ignores-in-env) lista as exceções para configurações de projeto e local.
* Para cancelar uma exportação de shell, defina a variável como `""`. Claude Code trata um valor vazio como não definido para seleção de provedor, e subprocessos herdam o valor vazio.
* `NO_COLOR` e `FORCE_COLOR` definidos aqui chegam apenas aos subprocessos. Para alterar as cores da própria interface de Claude Code, defina-as em seu shell antes de iniciar `claude`.
* Valores aqui são texto simples no arquivo de configurações e chegam a cada subprocesso que Claude Code inicia. Para um token de portador OTLP que gira, use [`otelHeadersHelper`](#otelheadershelper); para credenciais de API, use [`apiKeyHelper`](#apikeyhelper).

<h4 id="when-claude-code-applies-env-values">
  Quando Claude Code aplica valores `env`
</h4>

* Das configurações de usuário, `--settings` e configurações gerenciadas: na inicialização, e novamente na sessão em execução quando uma alteração salva modifica o `env` mesclado.
* Das configurações de projeto e local: depois que você confia no workspace, ou na inicialização no modo `-p`, que nunca mostra o diálogo de confiança, e novamente quando uma alteração salva modifica o `env` mesclado.
* Variáveis que Claude Code classifica como seguras, como seleção de modelo, timeouts e limites, e alternadores de recursos: na inicialização de cada arquivo de configurações, além das [variáveis que configurações de projeto e local não podem definir](#variables-claude-code-ignores-in-env).
* Depois que você [move a sessão com `/cd`](/docs/pt/permissions#move-the-session-to-another-directory) em v2.1.246 ou posterior: os valores `env` do novo diretório de projeto e local, além dos do diretório anterior.

<h4 id="variables-claude-code-ignores-in-env">
  Variáveis que Claude Code ignora em `env`
</h4>

* Configurações de projeto e local não podem definir variáveis que um repositório verificado não deveria controlar; defina-as em seu shell, configurações de usuário ou configurações gerenciadas. Claude Code descarta cada uma, além de alguns valores que desativam telemetria, e registra um aviso que você pode ver com `claude --debug`. Elas incluem:

  * Variáveis que escolhem onde Claude Code armazena ou escreve seus próprios arquivos: `CLAUDE_CONFIG_DIR`, `CLAUDE_CODE_TMPDIR` e as variáveis de diretório do sistema operacional como `HOME`, `TMPDIR`, `TMP`, `TEMP` e a família `XDG_*`.
  * Variáveis que exportam conteúdo de sessão: [`OTEL_LOG_RAW_API_BODIES`](/docs/pt/env-vars#variables) e o par de rastreamento beta detalhado `ENABLE_BETA_TRACING_DETAILED` e `BETA_TRACING_ENDPOINT`.
  * As variáveis do [exportador OpenTelemetry](/docs/pt/monitoring-usage) que ativam telemetria, escolhem para onde ela vai ou escolhem qual conteúdo ela captura:

    * `CLAUDE_CODE_ENABLE_TELEMETRY`, mais o par de telemetria aprimorada beta `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` e `ENABLE_ENHANCED_TELEMETRY_BETA`
    * Os seletores de exportador `OTEL_LOGS_EXPORTER`, `OTEL_METRICS_EXPORTER` e `OTEL_TRACES_EXPORTER`
    * As variáveis de conteúdo `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_ASSISTANT_RESPONSES`, `OTEL_LOG_TOOL_CONTENT` e `OTEL_LOG_TOOL_DETAILS`
    * Variáveis `OTEL_EXPORTER_OTLP_*` cujos nomes terminam em `_ENDPOINT`, `_HEADERS`, `_PROTOCOL`, `_CERTIFICATE`, `_CLIENT_KEY` ou `_INSECURE`, nas formas genérica e por sinal, como `OTEL_EXPORTER_OTLP_ENDPOINT` e `OTEL_EXPORTER_OTLP_METRICS_HEADERS`
    * `OTEL_EXPORTER_PROMETHEUS_HOST` e `OTEL_EXPORTER_PROMETHEUS_PORT`

    Apenas esses valores ainda se aplicam das configurações de projeto e local, porque desativam algo: `none` para os três seletores de exportador, e um valor desativado como `0` para `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_TOOL_CONTENT` e `OTEL_LOG_TOOL_DETAILS`. Tal valor sobrescreve a mesma variável em suas configurações de usuário, mas não uma que o ambiente do qual você inicia Claude Code, um arquivo `--settings` ou configurações gerenciadas definem.

    Quando um arquivo de configurações de projeto ou local define uma variável neste grupo, uma sessão interativa local mostra um aviso na inicialização. Execute `/status` ou `claude doctor` para ver quais Claude Code ignorou e quais desativaram telemetria; ambos listam nomes, nunca valores. Uma execução não interativa com `-p` ou uma sessão do Agent SDK não mostra aviso, então verifique se seu coletor ainda recebe dados depois que você atualizar. Se não receber, defina as variáveis em suas configurações de usuário, configurações gerenciadas, o ambiente do trabalho ou um arquivo que você passa com `--settings`.

    Ignorar este grupo em configurações de projeto e local requer Claude Code v2.1.282 ou posterior.
  * Variáveis que alteram como Claude Code inicia ou sincroniza, como `CLAUDE_CODE_PROCESS_WRAPPER`, `CLAUDE_CODE_SYNC_SKILLS`, `CLAUDE_CODE_SYNC_PLUGINS`, `CLAUDE_CODE_PLUGIN_CACHE_DIR` e `CLAUDE_CODE_PLUGIN_SEED_DIR`.

  Antes de v2.1.251, configurações de projeto e local podiam definir as variáveis nesta lista que escolhem onde Claude Code escreve seus arquivos ou que exportam conteúdo de sessão, exceto `HOME` e `XDG_CONFIG_HOME`.
* Variáveis de identidade que os ambientes de hospedagem de Claude Code possuem, como `CLAUDE_CODE_REMOTE` e `CLAUDE_CODE_ACCOUNT_UUID`, são ignoradas de cada arquivo.
* [`CLAUDE_CODE_MESSAGING_SOCKET` e `CLAUDE_CODE_MESSAGING_TOKEN`](/docs/pt/env-vars#variables), que Claude Code exporta a si mesmo, são ignoradas de cada arquivo. Ignorar a variável de socket requer Claude Code v2.1.224 ou posterior, e ignorar o token requer v2.1.228 ou posterior.
* [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/pt/sessions#name-the-project-directory-yourself), que Claude Code lê apenas do ambiente de inicialização, é ignorada de cada arquivo; requer v2.1.234 ou posterior.
* [`CLAUDE_CODE_RESTRICTED`](/docs/pt/env-vars#variables), que Claude Code lê apenas do ambiente de inicialização, é ignorada de cada arquivo.

<h3 id="filecheckpointingenabled">
  `fileCheckpointingEnabled`
</h3>

Faça Claude Code tirar snapshots de arquivos antes de cada edição para que [`/rewind`](/docs/pt/checkpointing) possa restaurá-los. Aparece em `/config` como **Rewind code (checkpoints)**, e alternar lá escreve essa chave nas suas configurações de usuário.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code tira snapshots de arquivos antes de cada edição para que `/rewind` possa restaurá-los
  * `false`: Claude Code não tira snapshots de arquivos, então `/rewind` não pode restaurá-los
* **Default**: `true`
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_FILE_CHECKPOINTING`](/docs/pt/env-vars) desativa checkpointing para uma sessão; qualquer um dos dois que o desativar, o outro não pode reativá-lo

```json settings.json theme={null}
{
  "fileCheckpointingEnabled": false
}
```

Em uma execução `-p` ou uma sessão do Agent SDK, Claude Code ignora essa chave. O SDK ativa checkpointing com sua opção `enableFileCheckpointing`, e uma execução `-p` simples precisa de `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=true`. Veja [File checkpointing in the Agent SDK](/docs/pt/agent-sdk/file-checkpointing).

<h3 id="plansdirectory">
  `plansDirectory`
</h3>

Escolha onde Claude Code armazena os arquivos de plano que escreve em [plan mode](/docs/pt/permission-modes#analyze-before-you-edit-with-plan-mode). Claude Code resolve o caminho relativo à raiz do projeto e mantém o padrão quando o caminho se resolve fora dela.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, um caminho relativo à raiz do projeto
* **Default**: não definido, então Claude Code usa `~/.claude/plans`

```json settings.json theme={null}
{
  "plansDirectory": "./plans"
}
```

<h3 id="skilllistingbudgetfraction">
  `skillListingBudgetFraction`
</h3>

A cada turno, Claude vê uma [listagem de suas skills](/docs/pt/skills#skill-descriptions-are-cut-short) com suas descrições, e Claude Code limita essa listagem a uma parte da janela de contexto. Quando a listagem está acima do limite, Claude Code mantém o nome de cada skill mas descarta as descrições das skills menos usadas, para que Claude ainda possa invocar essas skills mas seja menos provável escolher uma por conta própria. Aumente essa chave para manter mais descrições visíveis ao custo de mais contexto por turno.

* **Scope**: [`Any file`](#scopes)
* **Type**: número, uma fração maior que `0` e no máximo `1`
* **Default**: `0.01`, que reserva 1% da janela de contexto

```json settings.json theme={null}
{
  "skillListingBudgetFraction": 0.02
}
```

Para ver quanto contexto a listagem usa e quais skills contribuem mais, execute `/doctor`.

<h3 id="skilllistingmaxdescchars">
  `skillListingMaxDescChars`
</h3>

A cada turno, Claude vê uma [listagem de suas skills](/docs/pt/skills#skill-descriptions-are-cut-short) que mostra o texto `description` e `when_to_use` de cada skill. Essa chave limita quantos caracteres desse texto Claude Code mostra por skill; texto mais longo é cortado no limite.

* **Scope**: [`Any file`](#scopes)
* **Type**: número de caracteres, um inteiro positivo
* **Default**: `1536`

```json settings.json theme={null}
{
  "skillListingMaxDescChars": 2048
}
```

Aumente para manter descrições longas intactas ao custo de mais contexto por turno; diminua para caber mais skills sob [`skillListingBudgetFraction`](#skilllistingbudgetfraction).

<h3 id="taskoutputmaxchars">
  `taskOutputMaxChars`
</h3>

<Warning>
  Removido em v2.1.277, junto com a ferramenta `TaskOutput` que ele dimensionava. Defini-lo não tem efeito nas versões atuais. Claude lê um [arquivo de saída](/docs/pt/tools-reference#background-commands) de tarefa de background com `Read` em vez disso.
</Warning>

Até v2.1.276, você definia essa chave para o número de caracteres da saída de uma [tarefa de background](/docs/pt/tools-reference#background-commands) que Claude recebia inline quando lia a tarefa com a ferramenta `TaskOutput`.

<h2 id="interface-and-terminal">
  Interface e terminal
</h2>

Altere como Claude Code aparece e se comporta no seu terminal: tema, modo de editor, linha de status, spinner, notificações dentro da sessão e acessibilidade. Veja [Configuração de terminal](/docs/pt/terminal-config).

<h3 id="askuserquestiontimeout">
  `askUserQuestionTimeout`
</h3>

Deixe um diálogo [`AskUserQuestion`](/docs/pt/tools-reference) sem resposta continuar automaticamente após um período de tempo ocioso, enviando quaisquer opções que você já tivesse selecionado. Defina isso quando você se afastar e quiser que Claude continue sem você. Com o padrão, as perguntas aguardam até que você as responda. Requer Claude Code v2.1.200 ou posterior.

* **Scope**: [`User or managed`](#scopes)
* **Type**: string, um de `"60s"`, `"5m"`, `"10m"`, ou `"never"`
* **Default**: `"never"`
* **Per-session overrides**: [`CLAUDE_AFK_TIMEOUT_MS`](/docs/pt/env-vars) tem precedência sobre esta chave para uma sessão

```json settings.json theme={null}
{
  "askUserQuestionTimeout": "5m"
}
```

Aparece em `/config` como **Question auto-continue timeout**, que escreve esta chave nas configurações do usuário; Claude Code oculta a linha enquanto configurações gerenciadas ou a flag `--settings` definem a chave. Requer Claude Code v2.1.200 ou posterior.

<h3 id="autocontinueatusagelimit">
  `autoContinueAtUsageLimit`
</h3>

Depois que um limite de uso do claude.ai interrompe sua sessão, aguarde na sessão aberta e continue a tarefa automaticamente após a redefinição. Veja [Desativar continuação automática](/docs/pt/interactive-mode#turn-automatic-continue-off). Requer Claude Code v2.1.234 ou posterior.

* **Scope**: [`User or managed`](#scopes). Leia a partir das configurações do usuário, `--settings` e configurações gerenciadas apenas. Quando nenhum desses define a chave, um arquivo de configurações de projeto ou local que a define desativa o recurso em vez de ser ignorado.
* **Type**: Boolean
  * `true`: depois que um limite de uso do claude.ai interrompe sua sessão, Claude Code aguarda na sessão aberta e continua a tarefa automaticamente após a redefinição
  * `false`: Claude Code não inicia a espera por conta própria. Você ainda pode [iniciar uma espera você mesmo](/docs/pt/interactive-mode#start-a-wait-yourself) no menu de opções de limite de uso
* **Default**: `true`

```json settings.json theme={null}
{
  "autoContinueAtUsageLimit": false
}
```

Aparece em `/config` como **Continue automatically at usage limit**, que escreve esta chave nas configurações do usuário; Claude Code oculta a linha enquanto configurações gerenciadas ou a flag `--settings` definem a chave.

<h3 id="autoscrollenabled">
  `autoScrollEnabled`
</h3>

Siga a nova saída até o final da conversa em [renderização em tela cheia](/docs/pt/fullscreen). Desative-a para permanecer onde você rolou enquanto Claude continua trabalhando; prompts de permissão ainda rolam para a visualização.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: a conversa segue a nova saída até o final
  * `false`: você permanece onde rolou enquanto Claude continua trabalhando; prompts de permissão ainda aparecem abaixo da transcrição
* **Default**: `true`

```json settings.json theme={null}
{
  "autoScrollEnabled": false
}
```

Aparece em `/config` como **Auto-scroll** quando a renderização em tela cheia está ativada, que escreve esta chave nas configurações do usuário.

<h3 id="axscreenreader">
  `axScreenReader`
</h3>

Renderize saída amigável ao leitor de tela: texto simples sem bordas decorativas ou animações. O modo leitor de tela usa o renderizador clássico, portanto a configuração `tui` não tem efeito enquanto está ativo; [sessões em segundo plano](/docs/pt/agent-view) anexadas ainda renderizam em tela cheia.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code renderiza texto simples sem bordas decorativas ou animações, usando o renderizador clássico
  * `false`: Claude Code renderiza normalmente
* **Default**: unset, portanto o modo leitor de tela está desativado
* **Per-session overrides**: [`--ax-screen-reader`](/docs/pt/cli-reference#cli-flags) tem precedência sobre [`CLAUDE_AX_SCREEN_READER`](/docs/pt/env-vars), e ambos têm precedência sobre esta chave para uma sessão

```json settings.json theme={null}
{
  "axScreenReader": true
}
```

<h3 id="basheditdiffenabled">
  `bashEditDiffEnabled`
</h3>

Escolha se Claude Code registra quais arquivos foram alterados em um repositório Git enquanto um comando Bash é executado. Quando registra, você vê seu diff no terminal após o comando, e seus [hooks PostToolUse Bash](/docs/pt/hooks#bash) recebem a lista de arquivos alterados.

Um arquivo listado nem sempre é um que o comando alterou. Uma alteração que outro programa ou outra chamada Bash fez enquanto o comando era executado também pode aparecer lá.

Defina a chave como `true` para registrá-los em todos os modos de permissão. Requer Claude Code v2.1.269 ou posterior.

* **Scope**: [`User or managed`](#scopes). Um `true` conta apenas a partir de suas configurações de usuário, JSON passado com `--settings`, ou [configurações gerenciadas](/docs/pt/managed-settings), portanto um `true` em um arquivo `.claude/settings.json` ou `.claude/settings.local.json` de um repositório não pode ativar o registro. Um `false` em qualquer arquivo de repositório ainda o desativa a menos que um arquivo de [precedência mais alta](/docs/pt/settings#settings-precedence) defina `true`.
* **Type**: Boolean
* **Default**: unset, portanto Claude Code registra alterações no modo auto e modo `bypassPermissions` quando direciona Claude a editar arquivos através de Bash
* **Per-session overrides**: [`CLAUDE_CODE_BASH_EDIT_DIFF`](/docs/pt/env-vars) tem precedência sobre esta chave para uma sessão

```json settings.json theme={null}
{
  "bashEditDiffEnabled": true
}
```

<h3 id="companyannouncements">
  `companyAnnouncements`
</h3>

Mostre os anúncios da sua organização aos usuários na inicialização. Quando você lista mais de um, Claude Code escolhe um aleatoriamente para cada sessão; no primeiro lançamento de uma pessoa, ele mostra a primeira entrada.

* **Scope**: [`Any file`](#scopes)
* **Type**: array de strings
* **Default**: unset, portanto nenhum anúncio é exibido

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

Escolha se Bash ou PowerShell executa os comandos shell que você digita com o prefixo [`!`](/docs/pt/interactive-mode#shell-mode-with-prefix) na caixa de entrada, aqueles que Claude Code executa diretamente e adiciona à sessão.

`"powershell"` funciona apenas enquanto a [ferramenta PowerShell](/docs/pt/tools-reference#powershell-tool) está ativada. A ferramenta está ativada por padrão no Windows sem Git Bash, e no Windows com Git Bash para contas claude.ai e Console. Em sessões do Amazon Bedrock, da Plataforma de Agentes do Google Cloud e do Microsoft Foundry, e no macOS, Linux e WSL, defina `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` para ativar a ferramenta. Defina essa variável como `0` para desativar a ferramenta.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, um de:
  * `"bash"`: Claude Code executa seus comandos `!` em Bash
  * `"powershell"`: Claude Code executa seus comandos `!` em PowerShell
* **Default**: `"bash"`, ou `"powershell"` no Windows quando Bash não está disponível

```json settings.json theme={null}
{
  "defaultShell": "powershell"
}
```

Se o shell que você nomeou não estiver disponível, Claude Code usa o outro: `"powershell"` volta para Bash quando a ferramenta PowerShell está desativada, e `"bash"` volta para PowerShell quando Bash não está instalado.

<h3 id="dialogexpiry">
  `dialogExpiry`
</h3>

Defina o prazo para diálogos que Claude Code [encaminha para um cliente remoto](/docs/pt/remote-control#limitations), como um host Remote Control ou SDK, e para o diálogo de aprovação de uma [mensagem entre sessões retida](/docs/pt/cross-session-messaging#control-inbound-messages). No Claude Code v2.1.236 ou posterior, o mesmo prazo limita o prompt de consentimento de créditos de uso [Fable](/docs/pt/model-config#fable-and-usage-credits) no meio da sessão em uma sessão que pode não ter ninguém no terminal. Quando nenhuma resposta chega antes do prazo, Claude Code cancela o diálogo e continua com seu padrão sem ação. Requer Claude Code v2.1.224 ou posterior.

* **Scope**: [`User or managed`](#scopes)
* **Type**: string, um de `"60s"`, `"5m"`, `"10m"`, ou `"never"`, que desativa o prazo
* **Default**: `"5m"`
* **Per-session overrides**: [`CLAUDE_CODE_USER_DIALOG_TIMEOUT_MS`](/docs/pt/env-vars) tem precedência sobre esta chave para uma sessão

```json settings.json theme={null}
{
  "dialogExpiry": "10m"
}
```

Prompts de permissão e perguntas [`AskUserQuestion`](/docs/pt/tools-reference#askuserquestion-tool-behavior) usam seus próprios fluxos e não são regidos por este prazo. Aparece em `/config` como **Dialog expiry**, que escreve esta chave nas configurações do usuário; a linha requer Claude Code v2.1.232 ou posterior, e Claude Code a oculta enquanto configurações gerenciadas ou a flag `--settings` definem a chave.

<h3 id="editormode">
  `editorMode`
</h3>

Escolha o modo de vinculação de teclas para o prompt de entrada.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, um de:
  * `"normal"`: atalhos de teclado padrão na entrada do prompt
  * `"vim"`: edição no estilo vim com modos NORMAL, INSERT e VISUAL
* **Default**: `"normal"`

```json settings.json theme={null}
{
  "editorMode": "vim"
}
```

Aparece em `/config` como **Editor mode**, que escreve esta chave nas configurações do usuário.

<h3 id="emojicompletionenabled">
  `emojiCompletionEnabled`
</h3>

Mostre sugestões de emoji quando você digita `:` mais um código abreviado na entrada do prompt, e substitua um código abreviado concluído como `:heart:` por seu emoji. Defina como `false` para desativar ambos.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code mostra sugestões de emoji após `:` e substitui um código abreviado concluído por seu emoji
  * `false`: Claude Code não sugere emoji nem substitui códigos abreviados
* **Default**: `true`

```json settings.json theme={null}
{
  "emojiCompletionEnabled": false
}
```

Veja [Códigos abreviados de emoji](/docs/pt/interactive-mode#emoji-shortcodes). Requer Claude Code v2.1.217 ou posterior.

<span id="file-suggestion-settings" />

<h3 id="filesuggestion">
  `fileSuggestion`
</h3>

Execute seu próprio comando para fornecer preenchimento automático de caminho de arquivo `@` em vez da sugestão de arquivo integrada. A sugestão integrada usa travessia rápida do sistema de arquivos; um grande monorepo pode se sair melhor com indexação específica do projeto, como um índice de arquivo pré-construído.

* **Scope**: [`Any file`](#scopes). Sob os [portões de linha de status e sugestão de arquivo](#status-line-and-file-suggestion-gates), Claude Code desativa o comando ou executa apenas um valor gerenciado, e ignora o seu sem aviso.
* **Type**: objeto com `type`, sempre `"command"`, e `command`, o comando shell a executar
* **Default**: unset, portanto Claude Code usa a sugestão de arquivo integrada

```json settings.json theme={null}
{
  "fileSuggestion": {
    "type": "command",
    "command": "~/.claude/file-suggestion.sh"
  }
}
```

Depois de salvar isso, digite `@` seguido por parte de um caminho no prompt: as sugestões vêm da saída do seu comando.

<h4 id="command-input-and-output">
  Entrada e saída do comando
</h4>

Claude Code executa o comando com as mesmas variáveis de ambiente que [hooks](/docs/pt/hooks), incluindo `CLAUDE_PROJECT_DIR`, e para de aguardar após cinco segundos. O comando recebe JSON em stdin com um campo `query` contendo o que você digitou até agora:

```json theme={null}
{"query": "src/comp"}
```

Imprima caminhos de arquivo separados por nova linha em stdout. Claude Code mostra no máximo 15:

```text theme={null}
src/components/Button.tsx
src/components/Modal.tsx
src/components/Form.tsx
```

O script a seguir lê a consulta e a passa para um índice de arquivo de repositório:

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

Renderize badges clicáveis extras no rodapé abaixo da caixa de entrada quando um regex corresponde à saída de turno: resultados de ferramentas, incluindo conteúdo de arquivo e páginas buscadas, e respostas do próprio Claude. Use-o para transformar IDs impressos por CLIs de projeto, como ferramentas de revisão e rastreadores de problemas, em links de sessão.

* **Scope**: [`User or managed`](#scopes)
* **Type**: array de objetos, cada um com `type` definido como `"regex"`, um regex `pattern`, um template `url`, e um `label` opcional; placeholders `{name}` em `url` e `label` são preenchidos a partir de grupos de captura nomeados em `pattern`
* **Default**: unset, portanto nenhum badge é renderizado

Este exemplo corresponde a chaves de problema como `PROJ-1234` e constrói cada link a partir da chave capturada:

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

Com isso configurado, quando `PROJ-1234` aparece em um resultado de ferramenta ou na resposta de Claude, um badge `PROJ-1234` aparece no rodapé vinculando a `https://issues.example.com/browse/PROJ-1234`.

<h4 id="badge-constraints">
  Restrições de badge
</h4>

A URL, rótulo e contagem de badges de cada entrada são limitados da seguinte forma:

| Restrição          | Comportamento                                                                                                                                                                                                                        |
| :----------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Origem da URL      | Os valores capturados são codificados em URL e a URL construída deve compartilhar a origem literal do template. Uma captura pode preencher um segmento de caminho ou valor de consulta, mas não pode alterar para onde o link aponta |
| Comprimento da URL | URLs construídas com mais de 2048 caracteres são descartadas                                                                                                                                                                         |
| Esquema de URL     | Deve ser `https`, `http`, ou um esquema de link profundo de editor ou espaço de trabalho reconhecido: `vscode`, `vscode-insiders`, `cursor`, `windsurf`, `zed`, `jetbrains`, `idea`, `slack`, `linear`, `notion`, `figma`            |
| Rótulo             | Padrão para o texto correspondido e é truncado para 28 colunas de exibição                                                                                                                                                           |
| Contagem de badges | No máximo 5 badges são renderizados. O mais antigo é deslocado por correspondências mais recentes e `/clear` os remove                                                                                                               |

Quando um turno é concluído, Claude Code corresponde cada regex `pattern` da entrada contra a saída do turno no thread principal, portanto um regex lento bloqueia a UI até que termine. Quantificadores aninhados como `(a+)+$` podem levar exponencialmente tempo contra certas entradas e congelar a sessão, portanto mantenha cada `pattern` linear e evite aninhar `+` ou `*`.

Badges de rodapé são renderizados junto com uma [linha de status personalizada](/docs/pt/statusline) quando uma está configurada; nenhum substitui o outro. Use uma linha de status para uma linha orientada por script que calcula seu próprio conteúdo a partir de dados de sessão, e badges de rodapé para transformar IDs da conversa em links sem um script.

<h3 id="keybindingflavor">
  `keybindingFlavor`
</h3>

<Warning>
  Descontinuado desde v2.1.261 e não tem efeito. As teclas de edição de palavras do prompt sempre [seguem convenções readline](/docs/pt/interactive-mode#make-ctrl-w-delete-back-to-whitespace), como em Bash. Claude Code ainda aceita `keybindingFlavor`, portanto um arquivo de configurações que a define permanece válido.
</Warning>

Na v2.1.238 até v2.1.260, defini-la como `"readline"` fez `Ctrl+W` deletar de volta ao espaço em branco anterior em vez de apenas a palavra anterior.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, `"classic"` ou `"readline"`
* **Default**: unset

<h3 id="prefersreducedmotion">
  `prefersReducedMotion`
</h3>

Reduza ou desative animações de interface, como o spinner, shimmer e efeitos de flash. Aparece em `/config` como **Reduce motion**.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code reduz ou desativa animações de interface, como o spinner, shimmer e efeitos de flash
  * `false`: o mesmo que unset; Claude Code mostra suas animações
* **Default**: `false`

```json settings.json theme={null}
{
  "prefersReducedMotion": true
}
```

<h3 id="promptsuggestionenabled">
  `promptSuggestionEnabled`
</h3>

Mostre ou oculte [sugestões de prompt](/docs/pt/interactive-mode#prompt-suggestions), as previsões acinzentadas que aparecem na sua entrada de prompt. Defina como `false`, ou desative **Prompt suggestions** em `/config`, para ocultá-las.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: você vê sugestões de prompt na sua entrada de prompt
  * `false`: Claude Code oculta sugestões de prompt
* **Default**: `true`
* **Per-session overrides**: [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](/docs/pt/env-vars) tem precedência sobre esta chave para uma sessão

```json settings.json theme={null}
{
  "promptSuggestionEnabled": false
}
```

Sugestões de prompt precisam de uma conta claude.ai ou Console com telemetria ativada. No Amazon Bedrock, na Plataforma de Agentes do Google Cloud e no Microsoft Foundry, ou com telemetria desativada, como por [`DISABLE_TELEMETRY`](/docs/pt/env-vars), esta chave não tem efeito e apenas `CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=1` as ativa.

<h3 id="respectgitignore">
  `respectGitignore`
</h3>

Controle se o seletor de arquivo `@` deixa de fora arquivos que correspondem aos padrões `.gitignore`. Aparece em `/config` como **Respect .gitignore in file picker**.

* **Scope**: [`Any file`](#scopes). Quando nenhum arquivo de configurações a define, Claude Code volta para `respectGitignore` em `~/.claude.json`, que o toggle `/config` escreve.
* **Type**: Boolean
  * `true`: o seletor de arquivo `@` deixa de fora arquivos que correspondem aos padrões `.gitignore`
  * `false`: o seletor de arquivo `@` inclui arquivos que correspondem aos padrões `.gitignore`
* **Default**: `true`

```json settings.json theme={null}
{
  "respectGitignore": false
}
```

<h3 id="respondtobashcommands">
  `respondToBashCommands`
</h3>

Escolha se Claude responde depois que você executa um comando shell com o prefixo [`!`](/docs/pt/interactive-mode#shell-mode-with-prefix) na caixa de entrada. Por padrão, Claude Code adiciona a saída do comando à conversa e Claude responde a ela. Defina esta chave como `false` para adicionar a saída ao contexto sem uma resposta, para que você possa executar vários comandos e perguntar sobre eles juntos.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code adiciona a saída do comando à conversa e Claude responde a ela
  * `false`: Claude Code adiciona a saída ao contexto sem uma resposta
* **Default**: `true`

```json settings.json theme={null}
{
  "respondToBashCommands": false
}
```

Veja [Shell mode com prefixo `!`](/docs/pt/interactive-mode#shell-mode-with-prefix).

<h3 id="showclearcontextonplanaccept">
  `showClearContextOnPlanAccept`
</h3>

Quando Claude termina um plano em [modo de plano](/docs/pt/permission-modes#review-and-approve-a-plan), ele mostra um menu de aprovação. O planejamento pode usar muito contexto, portanto esta chave adiciona uma primeira opção a esse menu, **Yes, clear context and …**, que aprova o plano, limpa o contexto da conversa e começa a implementar apenas a partir do plano. O resto do rótulo nomeia o modo de permissão em que a sessão continua, e mostra quanto do seu contexto o planejamento usou.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: o menu de aprovação do plano obtém uma primeira opção, **Yes, clear context and …**, que aprova o plano e limpa o contexto da conversa
  * `false`: o menu de aprovação do plano não mostra nenhuma opção de limpar contexto
* **Default**: `false`

```json settings.json theme={null}
{
  "showClearContextOnPlanAccept": true
}
```

<h3 id="showturnduration">
  `showTurnDuration`
</h3>

Mostre ou oculte a mensagem de duração do turno após cada resposta, como "Cooked for 1m 6s · done 6:05 PM". O relógio após "done" mostra quando o turno terminou; [`timeFormat`](#timeformat) e [`timeZone`](#timezone) controlam seu formato e zona. Aparece em `/config` como **Show turn duration**.

* **Scope**: [`Any file`](#scopes). Um valor em `~/.claude.json` de uma versão anterior se aplica quando nenhum arquivo de configurações a define.
* **Type**: Boolean
  * `true`: você vê a mensagem de duração do turno após cada resposta
  * `false`: Claude Code oculta a mensagem de duração do turno
* **Default**: `true`

```json settings.json theme={null}
{
  "showTurnDuration": false
}
```

<h3 id="spellcheck">
  `spellcheck`
</h3>

Sublinhe palavras com erros de ortografia na entrada do prompt conforme você digita, usando um verificador de ortografia que você instala. Claude Code verifica apenas o texto na caixa de entrada. [Verificar ortografia conforme você digita](/docs/pt/interactive-mode#check-spelling-as-you-type) cobre a instalação de aspell, hunspell ou ispell e o que o verificador cobre. Requer Claude Code v2.1.235 ou posterior.

* **Scope**: [`User or managed`](#scopes). O bloco do nível mais alto que a define se aplica como um todo.
* **Type**: objeto com `enabled` (Boolean), `checker` (`"aspell"`, `"hunspell"`, `"ispell"`, ou `"auto"`), `language` (string, passada para o verificador como seu nome de dicionário), e `color` (string, um nome de cor de terminal, `#rrggbb`, `rgb(r,g,b)`, `ansi256(n)`, ou `ansi:<name>`)
* **Default**: unset, portanto a verificação de ortografia está desativada; `checker` padrão para `"auto"`, o primeiro dos três encontrado em `PATH`; `language` padrão para o próprio dicionário do verificador; `color` padrão para a cor de erro do tema

```json settings.json theme={null}
{
  "spellcheck": { "enabled": true, "language": "en_GB" }
}
```

<h3 id="spinnertipsenabled">
  `spinnerTipsEnabled`
</h3>

Enquanto Claude trabalha, a linha do spinner gira através de dicas curtas sobre recursos do Claude Code, como "Use Plan Mode para se preparar para uma solicitação complexa antes de fazer alterações. Pressione Shift+Tab duas vezes para ativar." Defina esta chave como `false` para ocultá-las. Aparece em `/config` como **Show tips**.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: você vê dicas no spinner enquanto Claude está trabalhando
  * `false`: Claude Code oculta dicas do spinner
* **Default**: `true`

```json settings.json theme={null}
{
  "spinnerTipsEnabled": false
}
```

<h3 id="spinnertipsoverride">
  `spinnerTipsOverride`
</h3>

Adicione suas próprias dicas às [dicas do spinner](#spinnertipsenabled) que Claude Code mostra enquanto Claude trabalha, ou substitua as dicas integradas pelas suas. Claude Code coloca suas dicas na mesma rotação que as integradas: ele escolhe a dica que não foi mostrada há mais tempo, pula dicas ainda em seu cooldown, e quebra empates por prioridade.

Se você definir [`spinnerTipsEnabled`](#spinnertipsenabled) como `false`, Claude Code oculta todas as dicas, incluindo as suas.

* **Scope**: [`Any file`](#scopes). Claude Code honra objetos de dica, `tipsFile`, `label` e `excludeDefault` das configurações do usuário, a flag `--settings` e configurações gerenciadas; a partir de configurações de projeto e local, ele lê apenas dicas de string simples.
* **Type**: objeto com campos `tips`, `tipsFile`, `label` e `excludeDefault`, cada um opcional
* **Default**: unset, portanto Claude Code mostra apenas as dicas integradas

Objetos de dica, `tipsFile`, `label` e a regra da linha Scope que configurações de projeto e local contribuem apenas com strings simples requerem Claude Code v2.1.247 ou posterior. Em versões anteriores, `excludeDefault` de um arquivo de projeto ou local também se aplica.

Cada entrada `tips` é uma string simples ou um objeto com estes campos:

| Campo              | Obrigatório | Descrição                                                                                                                                                                                                                    |
| :----------------- | :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`               | Sim         | Até 64 letras, dígitos, `.`, `_`, ou `-`. Claude Code baseia o histórico de exibição da dica nele, portanto o cooldown da dica sobrevive à reordenação da lista. De duas entradas com o mesmo id, Claude Code usa a primeira |
| `text`             | Sim         | A dica, uma linha de até 500 caracteres. Claude Code remove escapes ANSI e caracteres de controle e colapsa espaço em branco                                                                                                 |
| `cooldownSessions` | Não         | Sessões que Claude Code aguarda antes de mostrar a dica novamente, `0` a `1000`, padrão `0`                                                                                                                                  |
| `priority`         | Não         | Ordem entre dicas que não foram mostradas igualmente há muito tempo, maior primeiro, `-10` a `10`, padrão `0`                                                                                                                |

Claude Code lê uma string simples como uma dica com esses padrões e um id baseado em posição, portanto seu histórico de exibição é redefinido quando você reordena a lista. Dê a uma dica um `id` para manter seu histórico entre edições.

Claude Code lê no máximo 200 dicas entre `tips` e `tipsFile`, e descarta uma entrada inválida com um aviso de debug em vez de rejeitar o arquivo de configurações.

Use os campos restantes para nomear um arquivo de dicas, definir o prefixo e ocultar as dicas integradas:

* `tipsFile`: um caminho absoluto ou `~/` para um arquivo JSON local contendo um array das mesmas entradas, ou um objeto com um array `tips`, até 256 KB. Claude Code lê o arquivo uma vez por processo, portanto carrega suas edições na próxima inicialização. Você não pode defini-lo através de [configurações gerenciadas por servidor](/docs/pt/server-managed-settings); implante `tips` inline lá, ou implante o caminho em um `managed-settings.json` em disco.
* `label`: o prefixo que Claude Code mostra antes de dicas das configurações do usuário, `--settings` e configurações gerenciadas, até 40 caracteres. O padrão é `Tip`, o mesmo prefixo que as dicas integradas, e dicas de configurações de projeto e local sempre o usam.
* `excludeDefault`: defina como `true` para ocultar as dicas integradas e mostrar apenas as suas. Quando Claude Code não consegue carregar nenhuma de suas dicas, por exemplo porque `tipsFile` não existe ou cada entrada é inválida, ele mantém a rotação integrada em vez de um spinner vazio.

Quando mais de um arquivo de configurações define a chave, Claude Code mostra dicas de todos eles e pega `tipsFile`, `label` e `excludeDefault` de qualquer um das configurações gerenciadas, a flag `--settings` e configurações do usuário que seja o de precedência mais alta que define cada um.

Este exemplo, em suas configurações do usuário, adiciona uma dica de string simples e uma dica de objeto à rotação sob o prefixo `Acme tip`:

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

Cada campo no exemplo muda uma coisa sobre como Claude Code mostra as dicas:

* `label`: Claude Code mostra ambas as dicas como `Acme tip: ...` em vez de `Tip: ...`.
* A string simples: Claude Code lhe dá os padrões, portanto pode aparecer novamente na próxima sessão.
* `id`: Claude Code baseia o histórico de exibição da segunda dica em `gateway-errors`, portanto seu cooldown ainda se aplica depois que você adiciona ou reordena dicas.
* `cooldownSessions`: depois que Claude Code mostra a dica `gateway-errors`, ele não mostra essa dica novamente até cinco sessões depois.
* `priority`: quando a dica `gateway-errors` e outra dica não foram mostradas pelo mesmo número de sessões, por exemplo quando nenhuma foi mostrada ainda, Claude Code mostra `gateway-errors` primeiro. A string simples tem a prioridade padrão, `0`.

Enquanto Claude trabalha, Claude Code mostra suas dicas no spinner com seu prefixo, como `Acme tip: Run /review before opening a PR`.

<h3 id="spinnerverbs">
  `spinnerVerbs`
</h3>

Enquanto um turno está em progresso, o spinner mostra um verbo rotativo como "Accomplishing", "Architecting", ou "Baking". Use esta chave para adicionar seus próprios verbos a essa rotação ou substituir a lista integrada pela sua.

* **Scope**: [`Any file`](#scopes)
* **Type**: objeto com um array `verbs` de strings e `mode`, um de:
  * `"append"`: Claude Code adiciona seus verbos ao conjunto integrado
  * `"replace"`: Claude Code mostra apenas seus verbos
* **Default**: unset, portanto Claude Code usa os verbos integrados

Este exemplo adiciona dois verbos ao conjunto integrado:

```json settings.json theme={null}
{
  "spinnerVerbs": {
    "mode": "append",
    "verbs": ["Pondering", "Crafting"]
  }
}
```

No modo `"replace"` com um array `verbs` vazio, Claude Code mantém os verbos integrados.

<h3 id="statusline">
  `statusLine`
</h3>

Execute seu próprio comando para renderizar uma [linha de status](/docs/pt/statusline) abaixo do prompt com contexto como o modelo, custo ou branch git. Campos opcionais ajustam espaçamento, adicionam re-execuções periódicas e ocultam o indicador de modo vim integrado quando seu script renderiza `vim.mode` em si.

* **Scope**: [`Any file`](#scopes). Quando [`allowManagedHooksOnly`](#allowmanagedhooksonly) está ativado, ou [`disableAllHooks`](#disableallhooks) está definido fora das configurações gerenciadas, apenas o valor das configurações gerenciadas é executado.
* **Type**: objeto com `type` definido como `"command"` e uma string `command`, mais `padding` opcional como um número de caracteres, `refreshInterval` como um número de segundos, mínimo `1`, e `hideVimModeIndicator` como um Boolean
* **Default**: unset, portanto nenhuma linha de status

Este exemplo imprime o nome do modelo e o uso de contexto, e adiciona dois caracteres de espaçamento horizontal:

```json settings.json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
    "padding": 2
  }
}
```

O exemplo precisa de [`jq`](https://jqlang.org/) instalado e é executado em um shell. Para equivalentes PowerShell e Git Bash, veja [Configuração do Windows](/docs/pt/statusline#windows-configuration); para a configuração completa, veja [Configurar manualmente uma linha de status](/docs/pt/statusline#manually-configure-a-status-line).

<h3 id="subagentstatusline">
  `subagentStatusLine`
</h3>

Quando Claude executa [subagentes](/docs/pt/sub-agents), Claude Code os lista em uma exibição de tarefa abaixo do prompt, uma linha por subagente mostrando `name · description · token count`. Esta chave permite que você execute seu próprio comando para reescrever essas linhas, por exemplo para mostrar o uso de contexto de cada subagente como uma porcentagem. Em cada atualização, Claude Code envia as linhas visíveis como um objeto JSON em stdin, com um array `tasks` carregando `id`, `name`, `status`, `model`, `tokenCount` de cada subagente e mais, e substitui a linha para cada `id` que você escreve de volta como uma linha `{"id", "content"}`. Linhas que você não escreve de volta mantêm a renderização padrão.

* **Scope**: [`Any file`](#scopes). Quando [`allowManagedHooksOnly`](#allowmanagedhooksonly) está ativado, ou [`disableAllHooks`](#disableallhooks) está definido fora das configurações gerenciadas, apenas o valor das configurações gerenciadas é executado.
* **Type**: objeto com `type` definido como `"command"` e uma string `command`
* **Default**: unset, portanto Claude Code renderiza as linhas padrão

```json settings.json theme={null}
{
  "subagentStatusLine": {
    "type": "command",
    "command": "jq -c '.tasks[] | {id, content: \"\\(.name): \\(.tokenCount) tokens\"}'"
  }
}
```

Veja [Linhas de status de subagente](/docs/pt/statusline#subagent-status-lines).

<h3 id="syntaxhighlightingdisabled">
  `syntaxHighlightingDisabled`
</h3>

Claude Code colore código por linguagem nos diffs, blocos de código e visualizações de arquivo que mostra no terminal, com seu highlighter integrado; nenhum plugin ou servidor de linguagem está envolvido. Defina esta chave como `true` para mostrá-los como texto simples em vez disso, por exemplo se as cores entrarem em conflito com seu tema de terminal ou desacelerem um leitor de tela.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code desativa o destaque de sintaxe em diffs, blocos de código e visualizações de arquivo
  * `false`: Claude Code destaca a sintaxe
* **Default**: `false`

```json settings.json theme={null}
{
  "syntaxHighlightingDisabled": true
}
```

<h3 id="terminalprogressbarenabled">
  `terminalProgressBarEnabled`
</h3>

Alguns terminais podem mostrar um indicador de progresso na aba ou na barra de tarefas para o programa em execução neles. Enquanto Claude está trabalhando, Claude Code relata um estado em progresso ao terminal, para que você possa ver de outra aba ou janela se a sessão ainda está ocupada. O indicador permanece visível após o turno terminar enquanto [subagentes em segundo plano](/docs/pt/sub-agents#run-subagents-in-foreground-or-background) ou [fluxos de trabalho dinâmicos](/docs/pt/workflows) ainda estão em execução, e é limpo assim que a sessão fica ociosa.

Claude Code o relata apenas em terminais que suportam o indicador: ConEmu, Ghostty 1.2.0 ou posterior, e iTerm2 3.6.6 ou posterior. Defina esta chave como `false` para impedir que Claude Code o relate. Aparece em `/config` como **Terminal progress bar**.

* **Scope**: [`Any file`](#scopes). Um valor em `~/.claude.json` de uma versão anterior se aplica quando nenhum arquivo de configurações a define.
* **Type**: Boolean
  * `true`: você vê a barra de progresso do terminal em terminais que a suportam
  * `false`: Claude Code oculta a barra de progresso do terminal
* **Default**: `true`

```json settings.json theme={null}
{
  "terminalProgressBarEnabled": false
}
```

<h3 id="terminaltitlefromrename">
  `terminalTitleFromRename`
</h3>

Claude Code define o título da aba do seu terminal. Por padrão, ele usa um título que gera a partir da conversa, e uma vez que você dá à sessão um [nome](/docs/pt/sessions#name-your-sessions) com `/rename` ou `--name`, a aba mostra esse nome em vez disso. Defina esta chave como `false` para manter o título gerado na aba mesmo depois de nomear a sessão. O nome em si ainda se aplica, portanto `/resume <name>` e o seletor de sessão o encontram.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: o título da aba do terminal mostra o nome da sessão que você definiu
  * `false`: a aba mantém o título que Claude Code gera a partir de sua conversa
* **Default**: `true`

```json settings.json theme={null}
{
  "terminalTitleFromRename": false
}
```

Para impedir que Claude Code atualize o título do terminal completamente, defina [`CLAUDE_CODE_DISABLE_TERMINAL_TITLE`](/docs/pt/env-vars) como `1` em vez disso.

<h3 id="theme">
  `theme`
</h3>

Escolha o tema de cor para a interface. Aparece em `/config` como **Theme**.

* **Scope**: [`Any file`](#scopes). Um valor em `~/.claude.json` de uma versão anterior se aplica quando nenhum arquivo de configurações a define.
* **Type**: string, um de:
  * `"auto"`: corresponde ao fundo claro ou escuro do seu terminal
  * `"dark"`: o tema escuro
  * `"light"`: o tema claro
  * `"dark-daltonized"`: o tema escuro com cores amigáveis ao daltônico
  * `"light-daltonized"`: o tema claro com cores amigáveis ao daltônico
  * `"dark-ansi"`: o tema escuro usando apenas a paleta de cores ANSI do seu terminal
  * `"light-ansi"`: o tema claro usando apenas a paleta de cores ANSI do seu terminal
  * `"custom:<slug>"` ou `"custom:<plugin-name>:<slug>"`: um tema personalizado de `~/.claude/themes/` ou um plugin
* **Default**: `"dark"`

```json settings.json theme={null}
{
  "theme": "light-daltonized"
}
```

Veja [Criar um tema personalizado](/docs/pt/terminal-config#create-a-custom-theme).

<h3 id="timeformat">
  `timeFormat`
</h3>

Escolha como Claude Code escreve os horários que mostra na interface, como o `done 6:05 PM` no final de cada mensagem de duração de turno e os timestamps no [visualizador de transcrição](/docs/pt/interactive-mode#transcript-viewer). Para escolher uma predefinição, execute `/config` e defina **Time format**. Requer Claude Code v2.1.257 ou posterior.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, um de:
  * `"auto"`: o mesmo que unset; cada hora mantém seu formato integrado, que segue sua localidade na mensagem de duração de turno
  * `"12-hour"`: um relógio de 12 horas
  * `"24-hour"`: um relógio de 24 horas
  * `"24-hour-utc"`: um relógio de 24 horas em UTC com `Z` após os minutos, como `18:05Z`; Claude Code ignora [`timeZone`](#timezone) para esta predefinição
  * Um padrão strftime como `"%H:%M"`: Claude Code escreve cada hora com o padrão. Qualquer valor que contenha um `%` é um padrão, e qualquer outro valor fora das predefinições conta como `"auto"`
* **Default**: `"auto"`

```json settings.json theme={null}
{
  "timeFormat": "24-hour"
}
```

`/config` oferece apenas as predefinições, portanto para usar um padrão strftime, adicione a chave a um arquivo de configurações. Este exemplo mostra cada hora como um relógio de 24 horas de dois dígitos:

```json settings.json theme={null}
{
  "timeFormat": "%H:%M"
}
```

A mensagem de duração de turno e o visualizador de transcrição então mostram horários como `18:05`. No visualizador de transcrição, o padrão é o timestamp inteiro, portanto adicione diretivas de data quando quiser a data lá. Este exemplo coloca a data na frente do relógio:

```json settings.json theme={null}
{
  "timeFormat": "%Y-%m-%d %H:%M"
}
```

As mesmas superfícies então mostram horários como `2026-09-01 18:05`.

<h3 id="timezone">
  `timeZone`
</h3>

Mostre os horários na interface em um fuso horário diferente do seu sistema. Defina como um [nome de fuso horário IANA](https://www.iana.org/time-zones), como `"UTC"` ou `"Europe/Dublin"`. Os horários que [`timeFormat`](#timeformat) controla então mostram nesta zona. Se `timeFormat` for `"24-hour-utc"`, os horários permanecem em UTC e Claude Code ignora esta chave. `/config` não tem linha para esta chave, portanto defina em um arquivo de configurações. Requer Claude Code v2.1.257 ou posterior.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, um nome de fuso horário IANA. Quando Claude Code não reconhece o nome, usa seu fuso horário do sistema
* **Default**: unset, portanto os horários mostram em seu fuso horário do sistema

```json settings.json theme={null}
{
  "timeZone": "Europe/Dublin"
}
```

<h3 id="tui">
  `tui`
</h3>

Escolha o renderizador de UI do terminal. Use `"fullscreen"` para o renderizador [alt-screen](/docs/pt/fullscreen) sem cintilação com scrollback virtualizado, ou `"default"` para o renderizador clássico de tela principal. Executar `/tui fullscreen` ou `/tui default` escreve esta chave para você.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, um de:
  * `"default"`: o renderizador clássico de tela principal
  * `"fullscreen"`: o renderizador alt-screen sem cintilação com scrollback virtualizado
* **Default**: unset, portanto Claude Code [escolhe o renderizador para você](/docs/pt/fullscreen#fullscreen-by-default)
* **Per-session overrides**: [`CLAUDE_CODE_NO_FLICKER`](/docs/pt/env-vars) e [`CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN`](/docs/pt/env-vars) têm precedência sobre esta chave para uma sessão: `CLAUDE_CODE_NO_FLICKER=1` ativa tela cheia, e `CLAUDE_CODE_NO_FLICKER=0` ou `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1` a desativa; quando ambos estão definidos, Claude Code a desativa

```json settings.json theme={null}
{
  "tui": "fullscreen"
}
```

Sob tmux `-CC` ou sobre SSH para Windows, Claude Code mantém o renderizador clássico a menos que você defina `CLAUDE_CODE_NO_FLICKER=1`. Sessões em segundo plano abertas de [agent view](/docs/pt/agent-view) sempre usam o renderizador em tela cheia independentemente desta configuração.

<h3 id="verbose">
  `verbose`
</h3>

Por padrão, a transcrição colapsa cada chamada de ferramenta para um resumo curto, como o comando que Claude executou e uma contagem de linhas de sua saída, e você pressiona `Ctrl+O` para alternar a transcrição inteira para a visualização expandida quando quiser os detalhes. Defina esta chave como `true` para mostrar a entrada e saída completas de cada chamada de ferramenta inline conforme acontece, o que é útil quando você está depurando um hook, um servidor MCP ou um comando shell longo. Aparece em `/config` como **Verbose output**.

* **Scope**: [`Any file`](#scopes). Um valor em `~/.claude.json` de uma versão anterior se aplica quando nenhum arquivo de configurações a define.
* **Type**: Boolean
  * `true`: você vê saída completa de ferramenta
  * `false`: você vê resumos truncados de saída de ferramenta
* **Default**: `false`
* **Per-session overrides**: [`--verbose`](/docs/pt/cli-reference#cli-flags) tem precedência sobre esta chave para uma sessão

```json settings.json theme={null}
{
  "verbose": true
}
```

Um valor [`viewMode`](#viewmode) ou uma seleção sticky `/focus` substitui esta chave a cada sessão.

<h3 id="viewmode">
  `viewMode`
</h3>

Defina a visualização de transcrição em que Claude Code começa: `"default"`, `"verbose"`, ou `"focus"`. Quando definido, substitui tanto a seleção sticky `/focus` quanto a configuração [`verbose`](#verbose).

* **Scope**: [`Any file`](#scopes)
* **Type**: string, um de:
  * `"default"`: a transcrição normal com saída de ferramenta truncada
  * `"verbose"`: a transcrição com saída de ferramenta completa
  * `"focus"`: apenas seu último prompt, um resumo de uma linha de chamadas de ferramenta com diffstats de edição, e a resposta final. A visualização de foco precisa do [renderizador em tela cheia](#tui)
* **Default**: unset, portanto a configuração `verbose` e sua última escolha `/focus` se aplicam
* **Per-session overrides**: [`--verbose`](/docs/pt/cli-reference#cli-flags) tem precedência sobre esta chave para uma sessão

```json settings.json theme={null}
{
  "viewMode": "focus"
}
```

<h3 id="viminsertmoderemaps">
  `vimInsertModeRemaps`
</h3>

Mapeie sequências de INSERT-mode de duas teclas para Escape no [modo de editor vim](/docs/pt/interactive-mode#vim-editor-mode). Cada chave é exatamente dois caracteres imprimíveis digitados em sequência, e `"<Esc>"` é o único alvo suportado; Claude Code ignora outras entradas. Requer Claude Code v2.1.208 ou posterior.

* **Scope**: [`User or managed`](#scopes). Um repositório não pode remapear seus pressionamentos de tecla.
* **Type**: objeto mapeando uma sequência de dois caracteres para `"<Esc>"`
* **Default**: unset

```json settings.json theme={null}
{
  "vimInsertModeRemaps": {
    "jj": "<Esc>"
  }
}
```

Não tem efeito a menos que `editorMode` seja `"vim"`. Veja [Remapear sequências de tecla de INSERT-mode](/docs/pt/interactive-mode#remap-insert-mode-key-sequences). Requer Claude Code v2.1.208 ou posterior.

<h3 id="voice">
  `voice`
</h3>

Ative [ditado por voz](/docs/pt/voice-dictation) e escolha como a tecla de ditado se comporta. Claude Code escreve este objeto para você quando você executa `/voice`.

* **Scope**: [`Any file`](#scopes)
* **Type**: objeto com `enabled` como um Boolean, `autoSubmit` como um Boolean que se aplica apenas no modo hold, e `mode`, um de:
  * `"hold"`: você segura a tecla de ditado enquanto fala e a solta para parar
  * `"tap"`: você toca a tecla uma vez para começar a gravar e novamente para enviar
* **Default**: unset, portanto o ditado está desativado; quando `enabled` é `true` e `mode` é unset, Claude Code usa `"hold"`

Este exemplo ativa o ditado e faz a tecla tocar uma vez para começar a gravar e novamente para enviar:

```json settings.json theme={null}
{
  "voice": {
    "enabled": true,
    "mode": "tap"
  }
}
```

`autoSubmit` envia o prompt quando você solta a tecla no modo hold. O ditado por voz requer uma conta claude.ai.

<h3 id="voiceenabled">
  `voiceEnabled`
</h3>

<Warning>
  Descontinuado desde v2.1.92, quando o objeto [`voice`](#voice) o substituiu. Claude Code ainda o lê para que arquivos de configurações mais antigos continuem funcionando, mas novas configurações devem definir `voice.enabled`.
</Warning>

Ative o ditado por voz com o formulário Boolean único que precede o objeto `voice`. Quando ambos estão definidos, `voice.enabled` se aplica.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: o ditado por voz está ativado quando você está conectado com uma conta claude.ai e a política da sua organização permite voz, a menos que `voice.enabled` esteja definido
  * `false`: o ditado por voz está desativado, a menos que `voice.enabled` esteja definido
* **Default**: unset

```json settings.json theme={null}
{
  "voiceEnabled": true
}
```

<h3 id="wheelscrollaccelerationenabled">
  `wheelScrollAccelerationEnabled`
</h3>

Acelere a velocidade de rolagem da roda do mouse durante rolagens rápidas em [renderização em tela cheia](/docs/pt/fullscreen#mouse-wheel-scrolling). Defina como `false` para uma taxa de rolagem constante por entalhe de roda.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code acelera a velocidade de rolagem da roda do mouse durante rolagens rápidas
  * `false`: Claude Code rola a uma taxa constante por entalhe de roda
* **Default**: `true`

```json settings.json theme={null}
{
  "wheelScrollAccelerationEnabled": false
}
```

<h2 id="git-and-attribution">
  Git e atribuição
</h2>

Controle a atribuição que Claude Code adiciona aos commits e pull requests e como funciona com git.

<span id="attribution-settings" />

<h3 id="attribution">
  `attribution`
</h3>

Personalize a atribuição que Claude Code adiciona aos commits git e pull requests. Os commits recebem um [git trailer](https://git-scm.com/docs/git-interpret-trailers) como `Co-Authored-By` por padrão; as descrições de pull request recebem texto simples. Defina cada parte separadamente com as sub-chaves abaixo.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: objeto com strings `commit` e `pr` e um Boolean `sessionUrl`, ou `false` para ocultar toda a atribuição. O valor `false` requer Claude Code v2.1.281 ou posterior; versões anteriores o rejeitam e [pulam todo o arquivo de configurações do usuário, projeto ou local](/docs/pt/settings#fix-a-broken-settings-file) que o contém
* **Padrão**: não definido, então Claude Code usa a atribuição padrão mostrada em cada sub-chave

Para ocultar toda a atribuição, defina `attribution` como `false`. Em um arquivo de configurações que versões anteriores também leem, defina [`commit`](#attribution-commit) e [`pr`](#attribution-pr) como strings vazias e [`sessionUrl`](#attribution-sessionurl) como `false` em vez disso.

Este exemplo substitui a atribuição de commit, remove a atribuição de pull request e descarta o link da sessão:

```json settings.json theme={null}
{
  "attribution": {
    "commit": "Generated with AI\n\nCo-Authored-By: AI <ai@example.com>",
    "pr": "",
    "sessionUrl": false
  }
}
```

Depois que você definir `commit` ou `pr`, Claude Code ignora a configuração `includeCoAuthoredBy` descontinuada e usa seu texto padrão para qualquer um dos dois que você deixou não definido.

Claude Code informa a Claude que suas próprias instruções sobre atribuição, como uma regra CLAUDE.md ou [memory](/docs/pt/memory), têm precedência sobre essas linhas de commit e PR, a menos que a linha esteja definida em [managed settings](/docs/pt/managed-settings).

<h3 id="includecoauthoredby">
  `includeCoAuthoredBy`
</h3>

<Warning>
  Descontinuado desde v2.0.62, quando [`attribution`](#attribution) o substituiu. Claude Code ainda o lê, mas novas configurações devem definir `attribution`.
</Warning>

Use [`attribution`](#attribution) em vez disso, que substitui esta chave e permite que você altere ou oculte o trailer de commit, o texto de pull request e o link da sessão separadamente. Claude Code ainda honra `includeCoAuthoredBy: false` de arquivos de configuração anteriores a `attribution`, mas o ignora depois que você define `attribution.commit` ou `attribution.pr`.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: Boolean
  * `true`: o mesmo que não definido; Claude Code adiciona o trailer de commit e o texto de atribuição de pull request
  * `false`: Claude Code omite tanto o trailer de commit quanto o texto de atribuição de pull request, a menos que `attribution` defina `commit` ou `pr`, caso em que as regras de [`attribution`](#attribution) se aplicam
* **Padrão**: `true`

```json settings.json theme={null}
{
  "includeCoAuthoredBy": false
}
```

Para ocultar toda a atribuição, consulte [`attribution`](#attribution).

<h3 id="includegitinstructions">
  `includeGitInstructions`
</h3>

Claude Code fornece a Claude duas partes relacionadas a git de contexto: suas instruções integradas sobre como escrever commits e pull requests, na descrição da ferramenta Bash, e um snapshot de status git do seu repositório. O snapshot contém o branch atual, o branch principal, saída de `git status` e commits recentes. Claude Code o lê quando uma conversa começa.

Defina esta chave como `false` para deixar ambas de fora, por exemplo quando você usa suas próprias skills de fluxo de trabalho git.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: Boolean
  * `true`: Claude Code inclui suas instruções integradas de fluxo de trabalho de commit e pull request e o snapshot de status git. As sessões em nuvem nunca incluem o snapshot
  * `false`: Claude Code deixa ambas de fora
* **Padrão**: `true`
* **Substituições por sessão**: [`CLAUDE_CODE_DISABLE_GIT_INSTRUCTIONS`](/docs/pt/env-vars) tem precedência sobre esta chave para uma sessão

```json settings.json theme={null}
{
  "includeGitInstructions": false
}
```

<h3 id="prurltemplate">
  `prUrlTemplate`
</h3>

Aponte os links de PR que Claude Code renderiza, no badge de rodapé e em resumos de resultado de ferramenta, para uma ferramenta de revisão de código interna em vez de `github.com`. Claude Code substitui `{host}`, `{owner}`, `{repo}`, `{number}` e `{url}` da URL de PR. Os links de [solicitação de merge GitLab](/docs/pt/interactive-mode#gitlab-merge-requests) em ambas as superfícies mantêm sua URL GitLab.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: string, um template de URL usando qualquer um dos cinco placeholders
* **Padrão**: não definido

```json settings.json theme={null}
{
  "prUrlTemplate": "https://reviews.example.com/{owner}/{repo}/pull/{number}"
}
```

Claude Code aplica o template apenas aos links que renderiza; um número de PR que Claude escreve em uma mensagem, como `#123`, permanece como Claude o escreveu. Uma URL que não tem a forma `/pull/<number>` é deixada inalterada.

<h3 id="attribution-commit">
  `attribution.commit`
</h3>

Defina o texto de atribuição que Claude Code adiciona aos commits git, incluindo qualquer trailer. Defina como uma string vazia para ocultar a atribuição de commit.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: string
* **Padrão**: não definido, então Claude Code adiciona `Co-Authored-By: <name> <noreply@anthropic.com>`. O nome é o modelo ativo da sessão, como `Claude Sonnet 5`.
  * Quando Claude Code reconhece o modelo como um modelo Claude mas não consegue confirmar sua versão exata, escreve `Claude` sozinho.
  * Quando não consegue corresponder o ID do modelo a nenhum modelo Claude, como um modelo de terceiros servido através de um [`ANTHROPIC_BASE_URL`](/docs/pt/env-vars) customizado, escreve `Claude Code`.

Este exemplo substitui o trailer padrão por uma linha customizada e um trailer `Co-Authored-By` customizado:

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

Defina o texto de atribuição que Claude Code adiciona às descrições de pull request. Defina como uma string vazia para ocultar a atribuição de pull request.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: string
* **Padrão**: não definido, então Claude Code adiciona `🤖 Generated with [Claude Code](https://claude.com/claude-code)`

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

Escolha se Claude Code anexa o link de sessão claude.ai quando faz commit ou abre um pull request de uma sessão [cloud](/docs/pt/claude-code-on-the-web) ou [Remote Control](/docs/pt/remote-control). Claude Code adiciona o link como um trailer `Claude-Session` em commits e como um link em descrições de pull request. Defina como `false` para omitir o link.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: Boolean
  * `true`: Claude Code anexa o link de sessão claude.ai quando faz commit ou abre um pull request de uma sessão cloud ou Remote Control
  * `false`: Claude Code omite o link
* **Padrão**: `true`

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
  Hooks e automação
</h2>

Registre hooks, restrinja quais hooks são executados e controle fluxos de trabalho. Para eventos de hook e payloads, consulte a [referência de hooks](/docs/pt/hooks).

<h3 id="allowedhttphookurls">
  `allowedHttpHookUrls`
</h3>

Limite quais URLs os [HTTP hooks](/docs/pt/hooks#http-hook-fields) podem atingir. Quando você define essa chave, Claude Code executa um HTTP hook apenas se sua URL corresponder a um dos padrões e bloqueia o resto sem executá-los; um array vazio bloqueia todos os HTTP hooks.

* **Escopo**: [`Qualquer arquivo`](#scopes). Arrays são mesclados entre arquivos de configurações.
* **Tipo**: array de padrões de URL, com `*` como curinga
* **Padrão**: não definido, portanto qualquer URL é permitida

Este exemplo permite qualquer URL sob `https://hooks.example.com/` e qualquer URL `http://localhost`:

```json settings.json theme={null}
{
  "allowedHttpHookUrls": ["https://hooks.example.com/*", "http://localhost:*"]
}
```

A correspondência de nome de host não diferencia maiúsculas de minúsculas e trata `hooks.example.com.`, com o ponto final que marca um nome de domínio totalmente qualificado, da mesma forma que `hooks.example.com`, que é como o DNS os trata. A lista de permissões se aplica a hooks de todas as fontes, incluindo configurações gerenciadas.

<h3 id="allowmanagedhooksonly">
  `allowManagedHooksOnly`
</h3>

Restrinja a execução de hooks apenas aos hooks que sua organização implanta.

* **Escopo**: [`Gerenciado`](#scopes)
* **Tipo**: Booleano
  * `true`: apenas hooks gerenciados são executados, além de hooks do Agent SDK e hooks de plugins que suas configurações gerenciadas forçam a ativar. Consulte [O que é executado sob `allowManagedHooksOnly`](#what-runs-under-allowmanagedhooksonly)
  * `false`: hooks de todos os escopos de configurações e plugins são executados
* **Padrão**: não definido, portanto hooks de todos os escopos de configurações e plugins são executados

```json managed-settings.json theme={null}
{
  "allowManagedHooksOnly": true
}
```

<h4 id="what-runs-under-allowmanagedhooksonly">
  O que é executado sob `allowManagedHooksOnly`
</h4>

Quando você o define como `true`, Claude Code altera quais hooks e comandos semelhantes a hooks são carregados:

* **Hooks gerenciados e SDK são executados**: hooks de configurações gerenciadas e hooks que o [Agent SDK](/docs/pt/agent-sdk/overview) registra em processo
* **Hooks de plugins forçadamente ativados são executados**: hooks de plugins que suas configurações gerenciadas forçam a ativar através de [`enabledPlugins`](#enabledplugins). Claude Code corresponde ao ID completo `plugin@marketplace`, portanto um plugin com o mesmo nome de um marketplace diferente permanece bloqueado. Isso permite que você distribua hooks verificados através de um marketplace da organização enquanto bloqueia tudo o mais
* **Tudo o mais é bloqueado**: hooks de usuário, projeto e local, hooks de outros plugins e hooks declarados no frontmatter do agente
* **Plugins com origem em comando são desativados**: Claude Code também desativa plugins com uma [`command` source](/docs/pt/plugins/marketplace-reference#command-plugin-source), incluindo plugins forçadamente ativados em `enabledPlugins` gerenciado, a menos que você defina [`disableCommandPluginSources`](#disablecommandpluginsources) explicitamente como `false`
* **Comandos `headersHelper` do marketplace são bloqueados**: Claude Code também bloqueia comandos [`headersHelper`](/docs/pt/plugins/host-marketplace#authenticate-archive-downloads) do marketplace a menos que [`disableCommandPluginSources`](#disablecommandpluginsources) seja explicitamente definido como `false`, exceto para um marketplace que as próprias configurações gerenciadas declaram. Requer Claude Code v2.1.238 ou posterior
* **Linha de status e sugestão de arquivo restringem-se a configurações gerenciadas**: Claude Code lê [`statusLine`](/docs/pt/statusline), [`fileSuggestion`](#filesuggestion) e [`subagentStatusLine`](/docs/pt/statusline#subagent-status-lines) apenas de configurações gerenciadas, seguindo os [gates de linha de status e sugestão de arquivo](#status-line-and-file-suggestion-gates)

O comando [`/goal`](/docs/pt/goal) não pode ser executado enquanto essa chave está definida, porque depende de hooks.

<h3 id="disableallhooks">
  `disableAllHooks`
</h3>

Desative [hooks](/docs/pt/hooks#disable-or-remove-hooks), qualquer [linha de status](/docs/pt/statusline) personalizada e qualquer comando [sugestão de arquivo](#filesuggestion) personalizado. Use-o para desativar todos esses temporariamente sem deletá-los de suas configurações.

* **Escopo**: [`Qualquer arquivo`](#scopes). Apenas configurações gerenciadas podem desativar hooks gerenciados.
* **Tipo**: Booleano
  * `true`: Claude Code desativa hooks, qualquer linha de status personalizada e qualquer comando de sugestão de arquivo personalizado
  * `false`: hooks, a linha de status e o comando de sugestão de arquivo são executados
* **Padrão**: não definido, portanto hooks são executados

```json settings.json theme={null}
{
  "disableAllHooks": true
}
```

O alcance depende de qual arquivo carrega a chave:

* **Em configurações gerenciadas**: Claude Code desativa todos os hooks configurados, incluindo os gerenciados, e continua executando os hooks que o [Agent SDK](/docs/pt/agent-sdk/overview) registra em processo
* **Em qualquer outro arquivo de configurações**: Claude Code desativa hooks de usuário, projeto, local e plugin; hooks gerenciados, hooks do Agent SDK e hooks de plugins forçadamente ativados em [`enabledPlugins`](#enabledplugins) gerenciado continuam sendo executados

Manter hooks do Agent SDK em execução quando configurações gerenciadas definem essa chave requer Claude Code v2.1.242 ou posterior.

O comando [`/goal`](/docs/pt/goal) não pode ser executado enquanto hooks estão desativados, e o menu `/hooks` mostra um aviso em vez de seus hooks.

<h4 id="status-line-and-file-suggestion-gates">
  Gates de linha de status e sugestão de arquivo
</h4>

Claude Code toma duas decisões para `statusLine`, `fileSuggestion` e `subagentStatusLine`, nesta ordem:

* **Desativado completamente**: quando configurações gerenciadas definem `disableAllHooks`, ou quando a pasta não é confiável sob a mesma [regra de confiança de workspace que hooks em arquivos de configurações](/docs/pt/permissions#what-runs-before-you-trust-a-folder)
* **Restringido a configurações gerenciadas**: quando [`allowManagedHooksOnly`](#allowmanagedhooksonly) está definido, quando `disableAllHooks` é `true` fora de configurações gerenciadas após [precedência de configurações](/docs/pt/hooks#disable-or-remove-hooks) ser aplicada, ou quando você inicia Claude Code com `--safe-mode`

Sob restrição, Claude Code executa um valor gerenciado se um for implantado. Caso contrário, ele ignora seu valor sem aviso: a linha de status é desativada e o autocomplete `@` volta para a sugestão de arquivo integrada.

<h3 id="disableworkflows">
  `disableWorkflows`
</h3>

Desative [fluxos de trabalho dinâmicos](/docs/pt/workflows#turn-workflows-off) e os comandos de fluxo de trabalho agrupados para todos que suas configurações alcançam, como uma organização através de configurações gerenciadas. Para ativar ou desativar fluxos de trabalho apenas para você, use [`enableWorkflows`](#enableworkflows) em vez disso, que o toggle **Dynamic workflows** em `/config` escreve em suas configurações de usuário.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: Booleano
  * `true`: Claude Code desativa fluxos de trabalho dinâmicos e os comandos de fluxo de trabalho agrupados para todos que suas configurações alcançam
  * `false`: o mesmo que não definido; se fluxos de trabalho estão ativados então segue [`enableWorkflows`](#enableworkflows) e o padrão do seu plano
* **Padrão**: `false`
* **Substituições por sessão**: [`CLAUDE_CODE_DISABLE_WORKFLOWS`](/docs/pt/env-vars) desativa fluxos de trabalho por uma sessão; qualquer um dos dois que os desativa, o outro não pode ativá-los novamente

```json settings.json theme={null}
{
  "disableWorkflows": true
}
```

<h3 id="enableworkflows">
  `enableWorkflows`
</h3>

Ative ou desative [fluxos de trabalho dinâmicos](/docs/pt/workflows) para você quando o padrão do seu plano não é o que você quer. Aparece em `/config` como **Dynamic workflows**, que escreve essa chave em suas configurações de usuário e a remove novamente quando você alterna de volta para o padrão do seu plano. Para desativar fluxos de trabalho para todos a partir de configurações gerenciadas, use [`disableWorkflows`](#disableworkflows) em vez disso.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: Booleano
  * `true`: Claude Code ativa fluxos de trabalho dinâmicos para você
  * `false`: Claude Code desativa fluxos de trabalho dinâmicos para você
* **Padrão**: não definido, portanto fluxos de trabalho estão ativados a menos que você esteja no plano Pro, onde estão desativados
* **Substituições por sessão**: [`CLAUDE_CODE_DISABLE_WORKFLOWS`](/docs/pt/env-vars) desativa fluxos de trabalho por uma sessão, e `true` aqui não pode ativá-los novamente enquanto estiver definido

```json settings.json theme={null}
{
  "enableWorkflows": true
}
```

[`disableWorkflows`](#disableworkflows) e a política de fluxos de trabalho da sua organização também têm precedência: `enableWorkflows: true` não pode ativar fluxos de trabalho novamente enquanto qualquer fonte os desativa. Claude Code oculta a linha `/config` enquanto uma fonte diferente de suas configurações de usuário define `enableWorkflows`, ou define `disableWorkflows` como `true`.

<h3 id="hooks">
  `hooks`
</h3>

Execute seus próprios comandos, prompts, agentes, requisições HTTP ou ferramentas MCP como [hooks](/docs/pt/hooks) em pontos do ciclo de vida do Claude Code, como antes de uma chamada de ferramenta ou quando uma sessão inicia; a [referência de hooks](/docs/pt/hooks#hook-events) lista todos os eventos, seu payload e seus códigos de saída. Cada evento mapeia para uma lista de grupos de matcher, e cada grupo lista os handlers a executar quando o matcher se aplica.

* **Escopo**: [`Qualquer arquivo`](#scopes). Hooks são mesclados entre arquivos em vez de se substituírem, e hooks de configurações gerenciadas não podem ser removidos de outros arquivos.
* **Tipo**: objeto com chave por [evento de hook](/docs/pt/hooks#hook-events); cada valor é um array de grupos `{ "matcher", "hooks" }` cujas entradas `hooks` têm um `type` de `"command"`, `"prompt"`, `"agent"`, `"http"` ou `"mcp_tool"`
* **Padrão**: não definido, portanto nenhum hook é executado

Este exemplo executa um script antes de cada chamada de ferramenta Bash:

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

Para cada evento, padrão de matcher e campo de handler, consulte a [referência de hooks](/docs/pt/hooks#configuration). Para desativar hooks, consulte [`disableAllHooks`](#disableallhooks); para limitar hooks aos que sua organização implanta, consulte [`allowManagedHooksOnly`](#allowmanagedhooksonly).

<h3 id="httphookallowedenvvars">
  `httpHookAllowedEnvVars`
</h3>

Um [HTTP hook](/docs/pt/hooks#http-hook-fields) pode colocar o valor de uma variável de ambiente em um cabeçalho de requisição, por exemplo um cabeçalho `Authorization: Bearer $HOOK_TOKEN`, mas apenas para variáveis que o hook lista em seu próprio `allowedEnvVars`. Esta chave define um limite externo nessa lista para cada HTTP hook: um hook pode usar uma variável apenas se tanto seu próprio `allowedEnvVars` quanto esta chave a nomearem. Use-a para impedir que um hook leia um segredo que não deveria, mesmo quando a definição do hook pede por isso.

* **Escopo**: [`Qualquer arquivo`](#scopes). Arrays são mesclados entre arquivos de configurações.
* **Tipo**: array de nomes de variáveis de ambiente
* **Padrão**: não definido, portanto a lista `allowedEnvVars` de cada hook se aplica

Este exemplo limita a interpolação de cabeçalho a `MY_TOKEN` e `HOOK_SECRET`:

```json settings.json theme={null}
{
  "httpHookAllowedEnvVars": ["MY_TOKEN", "HOOK_SECRET"]
}
```

A lista de permissões se aplica a hooks de todas as fontes, incluindo configurações gerenciadas.

<h3 id="workflowkeywordtriggerenabled">
  `workflowKeywordTriggerEnabled`
</h3>

Escolha se digitar a palavra-chave `ultracode` em um prompt dispara um [fluxo de trabalho dinâmico](/docs/pt/workflows#ask-for-a-workflow-in-your-prompt). Defina como `false` para digitar a palavra sem disparar um.

* **Escopo**: [`Qualquer arquivo`](#scopes). Aparece em `/config` como **Ultracode keyword trigger**.
* **Tipo**: Booleano
  * `true`: digitar `ultracode` em um prompt dispara um fluxo de trabalho dinâmico
  * `false`: você pode digitar a palavra sem disparar um
* **Padrão**: `true`

```json settings.json theme={null}
{
  "workflowKeywordTriggerEnabled": false
}
```

A configuração de esforço `ultracode`, `/workflows` e comandos de fluxo de trabalho salvos não são afetados.

<h3 id="workflowsizeguideline">
  `workflowSizeGuideline`
</h3>

Defina a [contagem de agentes que Claude visa](/docs/pt/workflows#set-a-size-guideline) nos fluxos de trabalho dinâmicos que escreve. Claude Code envia o valor para Claude como conselho, não um limite imposto: `"small"` pede menos de 5 agentes, `"medium"` menos de 10 e `"large"` menos de 50. Escolha `"small"` quando você quer limitar o que um fluxo de trabalho gasta. Requer Claude Code v2.1.219 ou posterior.

* **Escopo**: [`Qualquer arquivo`](#scopes). Um valor lá tem precedência sobre a escolha **Dynamic workflow size** em `/config`, que Claude Code armazena em `~/.claude.json`, e Claude Code oculta essa linha enquanto um arquivo de configurações define a chave.
* **Tipo**: string, um de:
  * `"unrestricted"`: sem diretriz, portanto Claude dimensiona o fluxo de trabalho para a tarefa
  * `"small"`: Claude visa menos de 5 agentes
  * `"medium"`: Claude visa menos de 10 agentes
  * `"large"`: Claude visa menos de 50 agentes
* **Padrão**: `"medium"`, ou `"small"` quando você está conectado em um plano Pro com Claude Code v2.1.271 ou posterior

```json settings.json theme={null}
{
  "workflowSizeGuideline": "small"
}
```

Requer Claude Code v2.1.219 ou posterior; na v2.1.202 até v2.1.218, defina a diretriz em `/config` em vez disso.

<span id="plugin-configuration" />

<span id="manage-plugins" />

<span id="plugin-settings" />

<h2 id="plugins-and-skills">
  Plugins e skills
</h2>

Ative plugins, registre marketplaces, restrinja quais fontes de plugin sua organização permite e controle quais skills são carregadas. Para instalar e construir plugins, consulte [Plugins](/docs/pt/plugins/overview).

<h3 id="disablebundledskills">
  `disableBundledSkills`
</h3>

Desative os [skills](/docs/pt/skills) e workflows inclusos no Claude Code. Claude Code remove completamente os skills e workflows agrupados, enquanto comandos integrados como `/init` permanecem digitáveis, mas ficam ocultos do modelo.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code remove skills e workflows agrupados e oculta comandos integrados como `/init` do modelo
  * `false`: skills agrupados são carregados
* **Default**: não definido, portanto skills agrupados são carregados
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_BUNDLED_SKILLS`](/docs/pt/env-vars) definido como `1` desativa skills agrupados por uma sessão; qualquer um dos dois que os desativar, o outro não pode reativá-los

```json settings.json theme={null}
{
  "disableBundledSkills": true
}
```

Skills de plugins, `.claude/skills/` e `.claude/commands/` não são afetados. `/doctor` permanece digitável como os comandos integrados; para ocultá-lo, defina [`DISABLE_DOCTOR_COMMAND`](/docs/pt/env-vars) em vez disso.

<h3 id="disableskillshellexecution">
  `disableSkillShellExecution`
</h3>

Desative a execução de shell inline para blocos `` !`...` `` e ` ```! ` em [skills](/pt/skills) e comandos personalizados de fontes de usuário, projeto, plugin ou diretório adicional. Claude Code substitui cada comando por `[shell command execution disabled by policy]` em vez de executá-lo.

* **Scope**: [`Any file`](#scopes). Um `true` em configurações gerenciadas não pode ser substituído por `false` em outro lugar.
* **Type**: Boolean
  * `true`: Claude Code substitui cada comando de shell inline por `[shell command execution disabled by policy]` em vez de executá-lo
  * `false`: shell inline é executado
* **Default**: não definido, portanto shell inline é executado

```json settings.json theme={null}
{
  "disableSkillShellExecution": true
}
```

Skills agrupados e skills implantados através de configurações gerenciadas não são afetados.

<h3 id="skilloverrides">
  `skillOverrides`
</h3>

Oculte ou recolha um [skill](/docs/pt/skills#override-skill-visibility-from-settings) sem editar seu `SKILL.md`. Claude Code aplica o valor sob o nome de cada skill à lista de skills que Claude vê e ao seu autocompletar `/`.

* **Scope**: [`Any file`](#scopes). O menu `/skills` escreve em `.claude/settings.local.json`.
* **Type**: objeto mapeando nome do skill para um de:
  * `"on"`: Claude vê o skill e você pode digitar `/name`
  * `"name-only"`: Claude vê o skill pelo nome sem sua descrição
  * `"user-invocable-only"`: Claude não vê o skill, mas você ainda pode digitar `/name`
  * `"off"`: Claude não vê o skill e `/name` fica oculto do autocompletar
* **Default**: não definido, portanto cada skill é `"on"`

Este exemplo lista `legacy-context` para Claude apenas pelo nome e oculta `deploy` de Claude e do autocompletar `/`:

```json settings.json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

Overrides não se aplicam a skills de plugin, que você gerencia através de `/plugin`.

Em configurações gerenciadas e arquivos passados com `--settings`, uma chave em um alias de skill agrupado, como `checkup` para `/doctor`, também se aplica ao skill; consulte [como chaves de alias se combinam com chaves no próprio nome do skill](/docs/pt/skills#override-skill-visibility-from-settings).

<h3 id="syncclaudeaiskills">
  `syncClaudeAiSkills`
</h3>

Desative o download dos [skills habilitados para sua conta claude.ai](/docs/pt/skills#how-synced-skills-behave). Claude Code os baixa em `~/.claude/skills/synced/` em [sessões de terminal onde você entra com sua conta claude.ai](/docs/pt/skills#where-synced-skills-load), interativas ou não interativas, e em sessões Cowork e cloud. Defina `false` para parar esse download e parar de carregar os skills que já sincronizou. Claude Code honra apenas `false`: `true` é o mesmo que não definido e não ativa a sincronização onde está desativada.

* **Scope**: [`User, local, or managed`](#scopes), e arquivos passados com `--settings`. Um repositório não pode desativá-lo para você.
* **Type**: Boolean
  * `false`: Claude Code para de baixar skills sincronizados e para de carregar os já em `~/.claude/skills/synced/`. Em configurações de usuário ou gerenciadas, também os move para `~/.claude/skills/.trash/`
  * `true`: o mesmo que não definido
* **Default**: não definido, portanto sessões conectadas com sua conta claude.ai sincronizam seus skills

Este exemplo impede uma máquina de baixar os skills da conta em qualquer sessão:

```json settings.json theme={null}
{
  "syncClaudeAiSkills": false
}
```

<h3 id="syncclaudeaiplugins">
  `syncClaudeAiPlugins`
</h3>

Desative o download dos [plugins habilitados para sua conta claude.ai](/docs/pt/plugins/loading#synced-plugins). Claude Code os baixa em `~/.claude/plugins/synced/` no início de sessões de terminal onde você entra com sua conta claude.ai e em sessões Cowork, e carrega cada um como `<name>@synced`. Defina `false` para parar esse download e parar de carregar os plugins que já sincronizou. Claude Code honra apenas `false`: `true` é o mesmo que não definido e não ativa a sincronização onde está desativada. Requer Claude Code v2.1.273 ou posterior.

* **Scope**: [`User, local, or managed`](#scopes), e arquivos passados com `--settings`. Um repositório não pode desativá-lo para você.
* **Type**: Boolean
  * `false`: Claude Code para de baixar plugins sincronizados e para de carregar os já em `~/.claude/plugins/synced/`. Em configurações de usuário ou gerenciadas, também os move para `~/.claude/plugins/.trash/`
  * `true`: o mesmo que não definido
* **Default**: não definido, portanto sessões conectadas com sua conta claude.ai sincronizam seus plugins

Para desativar um plugin sincronizado em vez de todos eles, defina `"<name>@synced": false` em [`enabledPlugins`](#enabledplugins).

Este exemplo impede uma máquina de baixar os plugins da conta em qualquer sessão:

```json settings.json theme={null}
{
  "syncClaudeAiPlugins": false
}
```

<h3 id="allowedchannelplugins">
  `allowedChannelPlugins`
</h3>

Escolha quais plugins de [channel](/docs/pt/channels) podem enviar mensagens para sessões em sua organização. Quando você o define, Claude Code usa sua lista no lugar da lista de permissões padrão da Anthropic; cada entrada nomeia um plugin e o marketplace de onde vem.

* **Scope**: [`Managed`](#scopes)
* **Type**: array de objetos, cada um com strings `marketplace` e `plugin`. Uma entrada pode ser uma string `"plugin@marketplace"` como `"telegram@claude-plugins-official"`, que Claude Code trata como o objeto equivalente. A forma de string requer Claude Code v2.1.267 ou posterior; versões anteriores rejeitam todo o valor `allowedChannelPlugins` quando contém uma
* **Default**: não definido, portanto Claude Code usa a lista de permissões padrão da Anthropic

Este exemplo ativa channels e permite apenas o plugin Telegram do marketplace oficial da Anthropic:

```json managed-settings.json theme={null}
{
  "channelsEnabled": true,
  "allowedChannelPlugins": [
    { "marketplace": "claude-plugins-official", "plugin": "telegram" }
  ]
}
```

Um array vazio bloqueia cada plugin de channel.

Esta chave entra em vigor uma vez que channels passam pela porta [`channelsEnabled`](#channelsenabled) para a conta: em planos Team e Enterprise, e em contas Console com configurações gerenciadas, isso significa `channelsEnabled: true`. Consulte [Restringir quais plugins de channel podem ser executados](/docs/pt/channels#restrict-which-channel-plugins-can-run).

<h3 id="blockedmarketplaces">
  `blockedMarketplaces`
</h3>

Bloqueie fontes de marketplace de plugin para sua organização. Claude Code verifica a lista de bloqueio ao adicionar marketplace e ao instalar, atualizar, atualizar e auto-atualizar plugin, portanto um marketplace que alguém adicionou antes de você definir a política não pode ser usado para buscar plugins. Fontes bloqueadas são verificadas antes do download, portanto nunca tocam o sistema de arquivos.

Se você definir esta chave no [console de administração claude.ai](/docs/pt/server-managed-settings), claude.ai também a aplica quando qualquer pessoa em sua organização adiciona um marketplace de um repositório git no claude.ai, como [Como restrições funcionam](/docs/pt/plugins/org#restrict-what-users-can-install) descreve.

* **Scope**: [`Managed`](#scopes)
* **Type**: array de objetos de fonte de marketplace, nas mesmas formas que [`strictKnownMarketplaces`](#allowed-source-types)
* **Default**: não definido, portanto nenhum marketplace é bloqueado

Este exemplo bloqueia um repositório GitHub como fonte de marketplace:

```json managed-settings.json theme={null}
{
  "blockedMarketplaces": [
    { "source": "github", "repo": "untrusted/plugins" }
  ]
}
```

Uma entrada `github` pode usar a forma [owner-wildcard](#owner-wildcards) `"owner/*"` para bloquear cada repositório sob esse proprietário GitHub, que requer Claude Code v2.1.223 ou posterior. Adicione `{ "source": "skills-dir" }` para parar Claude Code carregando plugins [`@skills-dir`](/docs/pt/plugins/loading#plugins-shared-through-a-repository) de `~/.claude/skills/` sem restringir nenhum marketplace. Consulte [Restrições de marketplace gerenciadas](/docs/pt/plugins/org#restrict-what-users-can-install).

<h3 id="channelsenabled">
  `channelsEnabled`
</h3>

Permita [channels](/docs/pt/channels) para sua organização. Em planos Team e Enterprise do claude.ai, Claude Code bloqueia channels até você definir isso como `true`. Para contas do [Anthropic Console](/docs/pt/authentication#claude-console-authentication) que autenticam com uma chave API, channels são permitidos por padrão. Se sua organização implanta configurações gerenciadas, Claude Code bloqueia channels nessas contas também até você definir esta chave como `true`.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code permite channels para sua organização
  * `false`: o mesmo que não definido; se channels são bloqueados depende de seu plano, como o Default diz
* **Default**: não definido; channels são bloqueados em planos Team e Enterprise e em contas Console com configurações gerenciadas, e permitidos em planos Pro e Max e em contas Console sem configurações gerenciadas

```json managed-settings.json theme={null}
{
  "channelsEnabled": true
}
```

Para restringir quais plugins podem se registrar como channels uma vez ativados, defina [`allowedChannelPlugins`](#allowedchannelplugins). Consulte [Controles empresariais](/docs/pt/channels#enterprise-controls).

<h3 id="disablecommandpluginsources">
  `disableCommandPluginSources`
</h3>

Bloqueie a [fonte de plugin `command`](/docs/pt/plugins/marketplace-reference#command-plugin-source), que instala um plugin executando um comando declarado pelo marketplace na máquina do usuário. Quando você o define como `true`, Claude Code nunca executa o comando, não instala ou atualiza plugins originários de comando, e para de carregar os já instalados. Defina como `false` para permitir explicitamente. Sempre que bloqueia fontes de comando, seja você o definindo como `true` ou deixando não definido sob [`allowManagedHooksOnly`](#allowmanagedhooksonly), também bloqueia comandos [`headersHelper`](/docs/pt/plugins/host-marketplace#authenticate-archive-downloads) do marketplace, exceto para um marketplace que as próprias configurações gerenciadas declaram. Requer Claude Code v2.1.229 ou posterior, e o bloqueio `headersHelper` requer v2.1.238 ou posterior.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code nunca executa o comando declarado pelo marketplace, não instala ou atualiza plugins originários de comando, e para de carregar os já instalados
  * `false`: Claude Code permite plugins originários de comando explicitamente
* **Default**: não definido, portanto Claude Code segue [`allowManagedHooksOnly`](#allowmanagedhooksonly): uma organização que restringe execução de hook a configurações gerenciadas também obtém fontes de comando desativadas

```json managed-settings.json theme={null}
{
  "disableCommandPluginSources": true
}
```

Requer Claude Code v2.1.229 ou posterior.

<h3 id="pluginsuggestionmarketplaces">
  `pluginSuggestionMarketplaces`
</h3>

Nomeie os marketplaces cujos plugins podem aparecer como sugestões de instalação contextual, em dicas de spinner e fixadas no topo da aba **Discover** do `/plugin`. A dica integrada de design de frontend de primeira parte não é afetada. Sugestões vêm da declaração `relevance` de cada plugin em sua entrada de marketplace.

* **Scope**: [`Managed`](#scopes)
* **Type**: array de nomes de marketplace
* **Default**: não definido, portanto nenhuma sugestão declarada por marketplace aparece

```json managed-settings.json theme={null}
{
  "pluginSuggestionMarketplaces": ["acme-corp-plugins"]
}
```

Um nome entra em vigor apenas quando o marketplace é registrado na máquina e sua fonte registrada também é declarada nas mesmas configurações gerenciadas, seja como a entrada [`extraKnownMarketplaces`](#extraknownmarketplaces) para esse nome ou como uma entrada de [`strictKnownMarketplaces`](#strictknownmarketplaces). Claude Code ignora um marketplace registrado de uma fonte diferente sob um nome na lista de permissões. O marketplace oficial é isento do requisito de fonte: permitir apenas seu nome é suficiente, já que esse nome só pode se registrar da fonte Anthropic oficial. Consulte [Sugerir plugins por contexto](/docs/pt/plugins/relevance).

<h3 id="plugintrustmessage">
  `pluginTrustMessage`
</h3>

Adicione o próprio texto de sua organização ao aviso de confiança de plugin que Claude Code mostra antes da instalação, por exemplo para confirmar que plugins de seu marketplace interno são verificados.

* **Scope**: [`Managed`](#scopes)
* **Type**: string
* **Default**: não definido, portanto Claude Code mostra apenas o aviso padrão

```json managed-settings.json theme={null}
{
  "pluginTrustMessage": "All plugins from our marketplace are approved by IT"
}
```

<h3 id="strictknownmarketplaces">
  `strictKnownMarketplaces`
</h3>

Restrinja quais fontes de marketplace de plugin as pessoas em sua organização podem adicionar e instalar plugins. Claude Code aplica a lista de permissões ao adicionar marketplace e ao instalar, atualizar, atualizar e auto-atualizar plugin, antes de qualquer operação de rede ou sistema de arquivos, portanto um marketplace que alguém adicionou antes de você definir a política não pode ser usado para buscar plugins uma vez que sua fonte não corresponda mais. Usuários bloqueados veem um erro nomeando a política gerenciada.

Se você definir esta chave no [console de administração claude.ai](/docs/pt/server-managed-settings), claude.ai também a aplica quando qualquer pessoa em sua organização adiciona um marketplace de um repositório git no claude.ai, como [Como restrições funcionam](/docs/pt/plugins/org#restrict-what-users-can-install) descreve.

* **Scope**: [`Managed`](#scopes)
* **Type**: array de objetos de fonte de marketplace; consulte [Tipos de fonte permitidos](#allowed-source-types)
* **Default**: não definido, portanto usuários podem adicionar qualquer marketplace. Um array vazio é um bloqueio completo que bloqueia cada fonte de marketplace, incluindo o marketplace oficial da Anthropic

Este exemplo permite dois repositórios GitHub, um fixado à ref `v2.0`, e uma URL `marketplace.json` hospedada:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/approved-plugins" },
    { "source": "github", "repo": "acme-corp/security-tools", "ref": "v2.0" },
    { "source": "url", "url": "https://plugins.example.com/marketplace.json" }
  ]
}
```

Você também pode escrever esta chave como `allowedMarketplaces`; [Aliases de chave de marketplace](#marketplace-key-aliases) descreve como Claude Code trata o alias e qual versão o aceita. Esta chave é uma porta de política: controla o que usuários podem adicionar, mas não registra nada. Para restringir e pré-registrar em um arquivo, consulte [Combinar com `extraKnownMarketplaces`](#combine-with-extraknownmarketplaces). Para a visualização voltada ao usuário, consulte [Restrições de marketplace gerenciadas](/docs/pt/plugins/org#restrict-what-users-can-install).

<h4 id="allowed-source-types">
  Tipos de fonte permitidos
</h4>

Cada entrada abaixo mostra uma entrada de lista de permissões por tipo de fonte e os campos que aceita. A maioria dos tipos corresponde exatamente; `hostPattern` e `pathPattern` correspondem por regex, e entradas `github` podem usar um [owner wildcard](#owner-wildcards).

| Source        | Example entry                                                                                                                   | Fields                                                                                                                                               |
| :------------ | :------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------- |
| `github`      | `{ "source": "github", "repo": "acme-corp/plugins", "ref": "main", "path": "marketplace" }`                                     | `repo` obrigatório; `ref` é um branch ou tag; `path` é um subdiretório                                                                               |
| `git`         | `{ "source": "git", "url": "https://gitlab.example.com/tools/plugins.git", "ref": "production" }`                               | `url` obrigatório; `ref` e `path` como para `github`                                                                                                 |
| `url`         | `{ "source": "url", "url": "https://plugins.example.com/marketplace.json", "headers": { "Authorization": "Bearer ${TOKEN}" } }` | `url` obrigatório; `headers` adiciona cabeçalhos HTTP para acesso autenticado                                                                        |
| `file`        | `{ "source": "file", "path": "/opt/acme-corp/plugins/marketplace.json" }`                                                       | `path` obrigatório, o caminho absoluto para um arquivo `marketplace.json`                                                                            |
| `directory`   | `{ "source": "directory", "path": "/opt/acme-corp/approved-marketplaces" }`                                                     | `path` obrigatório, o caminho absoluto para um diretório contendo `.claude-plugin/marketplace.json`                                                  |
| `hostPattern` | `{ "source": "hostPattern", "hostPattern": "^github\\.example\\.com$" }`                                                        | `hostPattern` obrigatório, um regex correspondido em qualquer lugar no host do marketplace; ancorá-lo com `^` e `$` para corresponder o host inteiro |
| `pathPattern` | `{ "source": "pathPattern", "pathPattern": "^/opt/approved/" }`                                                                 | `pathPattern` obrigatório, um regex correspondido em qualquer lugar no `path` de fontes `file` e `directory`; comece com `^` para fixar um prefixo   |
| `skills-dir`  | `{ "source": "skills-dir" }`                                                                                                    | Sem campos. Opta a varredura de plugin `~/.claude/skills/` de volta                                                                                  |

Três tipos de fonte carregam regras além da tabela:

* **`url`**: um marketplace de URL baixa apenas o arquivo `marketplace.json`, e Claude Code não busca arquivos de plugin por caminho relativo desse servidor, portanto seus plugins devem usar uma [plugin source](/docs/pt/plugins/marketplace-reference#plugin-sources) diferente de um caminho relativo, como uma URL de arquivo, que pode estar no mesmo host. Para plugins com caminhos relativos, use um marketplace baseado em Git. Consulte [Plugins com caminhos relativos falham em marketplaces baseados em URL](/docs/pt/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces).
* **`hostPattern`**: use-o para permitir cada marketplace em um GitHub Enterprise interno ou servidor GitLab sem listar cada repositório. Claude Code corresponde fontes `github` contra `github.com`, pega o hostname de fontes `url`, e o pega de fontes `git` dependendo da forma da [git URL](https://git-scm.com/docs/git-clone#_git_urls):

  * Uma URL com um esquema, como `https://` ou `ssh://`: o hostname na URL.
  * Um endereço SSH sem esquema, na forma `user@host:path` do git, como `git@git.example.com:tools/plugins.git`: o host entre `@` e `:`, que é o host ao qual git se conecta.
  * Qualquer outra forma sem esquema: sem host, portanto nenhuma entrada `strictKnownMarketplaces` `hostPattern` corresponde. Para uma `hostPattern` `blockedMarketplaces`, Claude Code pega um host de um conjunto mais amplo de formas, portanto uma entrada de lista de bloqueio ainda pode corresponder tal forma. Antes de v2.1.234, uma `hostPattern` `strictKnownMarketplaces` também correspondia algumas formas que git não trata como endereços SSH.

  Fontes `file` e `directory` não têm host e nunca correspondem a uma entrada `hostPattern`.
* **`pathPattern`**: use-o para permitir marketplaces do sistema de arquivos ao lado de entradas `hostPattern` para fontes de rede. `".*"` permite cada caminho local; um padrão mais estreito como `"^/opt/approved/"` restringe a um diretório.

Qualquer lista de permissões, mesmo uma vazia, também para Claude Code carregando plugins [`@skills-dir`](/docs/pt/plugins/loading#plugins-shared-through-a-repository) de `~/.claude/skills/`. Adicione a entrada `{ "source": "skills-dir" }` para continuar carregando-os; a entrada não tem significado fora desta chave e `blockedMarketplaces`.

<h4 id="owner-wildcards">
  Owner wildcards
</h4>

Uma entrada `github` cujo valor `repo` é `"<owner>/*"` corresponde cada repositório sob esse proprietário GitHub. Owner wildcards requerem Claude Code v2.1.223 ou posterior e funcionam apenas em `strictKnownMarketplaces` e `blockedMarketplaces`. Em qualquer outro lugar uma fonte `github` aparece, como `extraKnownMarketplaces` ou `/plugin marketplace add`, o valor `repo` deve nomear um único repositório. Antes de v2.1.223, Claude Code comparava a entrada literalmente, portanto uma entrada de lista de permissões não correspondia a nenhum repositório e uma entrada de lista de bloqueio não bloqueava nada; entradas de repositório único são aplicadas em cada versão.

Esta entrada permite qualquer repositório de marketplace na organização `acme-corp`:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/*" }
  ]
}
```

Apenas a posição de nome de repositório inteiro pode ser um wildcard. Claude Code ignora entradas como `*`, `*/plugins`, ou `acme-corp/tools-*` como inválidas, portanto não correspondem a nenhum repositório.

As regras de correspondência diferem entre as duas configurações:

| Rule                      | `strictKnownMarketplaces`                                                                                                                                                          | `blockedMarketplaces`                                                                 |
| ------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| Matching source spellings | Apenas forma `owner/repo`. Uma git URL que clona o mesmo repositório não corresponde                                                                                               | Qualquer grafia, incluindo git URLs que resolvem para o mesmo repositório github.com  |
| Owner case                | Sensível a maiúsculas/minúsculas, como correspondência de entrada exata                                                                                                            | Insensível a maiúsculas/minúsculas                                                    |
| `ref`                     | Segue as regras de entrada exata: uma entrada com um `ref` corresponde apenas fontes com esse ref exato, e uma entrada sem um corresponde apenas fontes que não especificam um ref | Uma entrada sem um `ref` bloqueia todos os refs dos repositórios que corresponde      |
| `path`                    | Mais flexível que as regras de entrada exata: uma entrada com um `path` requer esse valor exato, enquanto uma entrada sem um corresponde qualquer caminho dentro do repositório    | Uma entrada sem um `path` bloqueia todos os caminhos dos repositórios que corresponde |

<h4 id="exact-matching">
  Correspondência exata
</h4>

Para cada tipo de fonte exceto entradas `github` de owner-wildcard e as entradas `hostPattern` e `pathPattern` correspondidas por regex, Claude Code permite uma adição de usuário apenas quando a fonte de marketplace corresponde a uma entrada exatamente. Para as fontes baseadas em git `github` e `git`, correspondência exata inclui os campos opcionais:

* O `repo` ou `url` deve corresponder exatamente
* O campo `ref` deve corresponder exatamente, ou ambos devem ser indefinidos
* O campo `path` deve corresponder exatamente, ou ambos devem ser indefinidos

Por exemplo, Claude Code trata cada par abaixo como duas fontes diferentes:

* `{ "source": "github", "repo": "acme-corp/plugins" }` e `{ "source": "github", "repo": "acme-corp/plugins", "ref": "main" }`
* `{ "source": "github", "repo": "acme-corp/plugins", "path": "marketplace" }` e `{ "source": "github", "repo": "acme-corp/plugins" }`

<h4 id="allow-only-the-official-marketplace">
  Permitir apenas o marketplace oficial
</h4>

Para permitir apenas o marketplace oficial da Anthropic e nada mais, liste seu repositório:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "anthropics/claude-plugins-official" }
  ]
}
```

Com esta entrada, Claude Code mantém um marketplace oficial já registrado disponível e, em uma máquina nova, registra o marketplace automaticamente na primeira vez que você inicia uma sessão de terminal interativa. O registro automático mais comumente perde:

* Ambientes não interativos que executam antes da primeira sessão de terminal interativa da máquina.
* Máquinas onde Claude Code já executou uma sessão de terminal interativa sob uma política que bloqueou o marketplace, como o bloqueio de array vazio. Claude Code registra a tentativa bloqueada e não tenta novamente após a política mudar.

Nessas máquinas, adicione o marketplace a [`extraKnownMarketplaces`](#extraknownmarketplaces) no mesmo `managed-settings.json` para que Claude Code o registre automaticamente, ou execute `claude plugin marketplace add anthropics/claude-plugins-official`.

<h4 id="combine-with-extraknownmarketplaces">
  Combinar com `extraKnownMarketplaces`
</h4>

As duas chaves fazem trabalhos diferentes. Esta tabela as compara:

| Aspect            | `strictKnownMarketplaces`                        | `extraKnownMarketplaces`                                                                                                           |
| ----------------- | ------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------- |
| Purpose           | Aplicação de política organizacional             | Conveniência de equipe                                                                                                             |
| Settings file     | Apenas configurações gerenciadas                 | Qualquer arquivo de configurações                                                                                                  |
| Behavior          | Bloqueia adições não permitidas                  | Registra marketplaces ausentes                                                                                                     |
| When enforced     | Antes de operações de rede e sistema de arquivos | Imediatamente de configurações de usuário ou gerenciadas; após o diálogo de confiança de workspace para arquivos de um repositório |
| Can be overridden | Não, precedência mais alta                       | Sim, por configurações de precedência mais alta                                                                                    |
| Source format     | Objeto de fonte direto                           | Marketplace nomeado com um objeto `source` aninhado                                                                                |

Para restringir e pré-registrar um marketplace para todos os usuários, defina ambos em `managed-settings.json`:

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

Com apenas `strictKnownMarketplaces` definido, usuários ainda podem adicionar um marketplace permitido eles mesmos com `/plugin marketplace add`. O marketplace oficial da Anthropic é o único que Claude Code registra automaticamente, e apenas quando a lista de permissões o permite. [Permitir apenas o marketplace oficial](#allow-only-the-official-marketplace) lista as máquinas que perde.

<h3 id="strictpluginonlycustomization">
  `strictPluginOnlyCustomization`
</h3>

Bloqueie skills, agents, hooks e servidores MCP de fontes de usuário e projeto, portanto podem vir apenas de plugins ou configurações gerenciadas. Combine com [`strictKnownMarketplaces`](#strictknownmarketplaces) para controlar a cadeia de suprimento de customização completa: a lista de permissões de marketplace controla quais plugins usuários podem instalar.

* **Scope**: [`Managed`](#scopes)
* **Type**: `true` para bloquear todos os quatro tipos de customização, ou um array nomeando os tipos a bloquear, de `"skills"`, `"agents"`, `"hooks"` e `"mcp"`
* **Default**: não definido, portanto nada é bloqueado

Este exemplo bloqueia skills e hooks e deixa agents e servidores MCP desbloqueados:

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["skills", "hooks"]
}
```

As quatro entradas de sub-chave abaixo listam o que cada superfície bloqueia e o que ainda carrega. Claude Code ignora nomes de superfície que não reconhece em vez de falhar no arquivo de configurações, portanto você pode adicionar novos nomes de superfície antes de cada cliente ter atualizado.

<h3 id="strictpluginonlycustomization-skills">
  `strictPluginOnlyCustomization.skills`
</h3>

Bloqueie a superfície `skills`. Claude Code para de carregar skills de `~/.claude/skills/` e `.claude/skills/`, comandos personalizados de `~/.claude/commands/` e `.claude/commands/`, skills sob diretórios `--add-dir`, e skills sincronizados de sua conta claude.ai, e continua carregando skills de plugin, skills agrupados e skills no diretório de política gerenciada.

* **Scope**: [`Managed`](#scopes)
* **Type**: a string `"skills"` no array [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default**: não bloqueado

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["skills"]
}
```

<h3 id="strictpluginonlycustomization-agents">
  `strictPluginOnlyCustomization.agents`
</h3>

Bloqueie a superfície `agents`. Claude Code para de carregar agents de `~/.claude/agents/` e `.claude/agents/`, e continua carregando agents de plugin, agents integrados e agents no diretório de política gerenciada.

* **Scope**: [`Managed`](#scopes)
* **Type**: a string `"agents"` no array [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default**: não bloqueado

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["agents"]
}
```

<h3 id="strictpluginonlycustomization-hooks">
  `strictPluginOnlyCustomization.hooks`
</h3>

Bloqueie a superfície `hooks`. Claude Code para de executar hooks de configurações de usuário, projeto e local `settings.json`, e continua executando hooks de plugin e hooks em configurações gerenciadas.

* **Scope**: [`Managed`](#scopes)
* **Type**: a string `"hooks"` no array [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default**: não bloqueado

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["hooks"]
}
```

<h3 id="strictpluginonlycustomization-mcp">
  `strictPluginOnlyCustomization.mcp`
</h3>

Bloqueie a superfície `mcp`. Claude Code para de carregar servidores MCP de `~/.claude.json` e `.mcp.json`, e continua carregando servidores MCP de plugin, servidores [`managed-mcp.json`](/docs/pt/managed-mcp) e servidores de [`managedMcpServers`](#managedmcpservers).

* **Scope**: [`Managed`](#scopes)
* **Type**: a string `"mcp"` no array [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default**: não bloqueado

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["mcp"]
}
```

<h3 id="enabledplugins">
  `enabledPlugins`
</h3>

Ative ou desative [plugins](/docs/pt/plugins/overview) individuais, codificados por `plugin-name@marketplace-name`. Um plugin sem entrada em nenhum escopo volta para seu valor [`defaultEnabled`](/docs/pt/plugins/manifest-reference#fields). Quando você ativa ou desativa um plugin com `/plugin` ou `claude plugin enable`, Claude Code escreve esta chave para você.

* **Scope**: [`Any file`](#scopes)
* **Type**: objeto mapeando `plugin-name@marketplace-name` para um Boolean
* **Default**: não definido, portanto cada plugin segue seu valor `defaultEnabled`

Este exemplo ativa dois plugins do marketplace `team-tools` e desativa um de `personal`:

```json settings.json theme={null}
{
  "enabledPlugins": {
    "code-formatter@team-tools": true,
    "deployment-tools@team-tools": true,
    "experimental-features@personal": false
  }
}
```

Cada escopo serve um propósito diferente:

* **User settings**: suas preferências pessoais de plugin
* **Project settings**: plugins compartilhados com todos no repositório
* **Local settings**: overrides por máquina, gitignored quando Claude Code salva uma configuração lá
* **Managed settings**: política em toda a organização. Um plugin definido como `false` aqui é bloqueado da instalação em cada escopo e oculto do marketplace

Configurações de projeto têm precedência sobre configurações de usuário, portanto definir um plugin como `false` em `~/.claude/settings.json` não desativa um plugin que o `.claude/settings.json` do projeto ativa. Para optar por não participar de um plugin ativado pelo projeto em sua máquina, defina-o como `false` em `.claude/settings.local.json`. Plugins forçadamente ativados por configurações gerenciadas não podem ser desativados desta forma, já que configurações gerenciadas substituem configurações locais.

Ativar um plugin de uma fonte externa como um repositório GitHub ou pacote npm no `.claude/settings.json` de um projeto não o instala para outras pessoas. Em cada caminho que carrega plugins, Claude Code relata o plugin como não instalado até que cada usuário o [instale eles mesmos](/docs/pt/plugins/org#require-plugins-per-repository).

<h3 id="extraknownmarketplaces">
  `extraKnownMarketplaces`
</h3>

Registre marketplaces de plugin adicionais por nome, para que pessoas que abram o repositório, ou todos que suas configurações gerenciadas alcançam, obtenham o marketplace sem adicioná-lo eles mesmos. Claude Code registra cada marketplace que ainda não conhece. Se um plugin que [`enabledPlugins`](#enabledplugins) nomeia dele instala depende da fonte do plugin e qual arquivo o ativa; essa entrada tem as regras.

* **Scope**: [`Any file`](#scopes). Claude Code honra entradas no `.claude/settings.json` ou `.claude/settings.local.json` de um repositório apenas após você aceitar o diálogo de confiança de workspace para essa pasta; em uma pasta que você não confiou, incluindo uma execução `-p` lá, ignora-as sem mensagem.
* **Type**: objeto mapeando um nome de marketplace para um objeto com um objeto `source` e um Boolean `autoUpdate` opcional
* **Default**: não definido

Este exemplo registra um marketplace GitHub e um marketplace de uma URL git auto-hospedada:

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

[O que é executado antes de você confiar em uma pasta](/docs/pt/permissions#what-runs-before-you-trust-a-folder) compara a porta de confiança com o outro conteúdo que um repositório pode fornecer. Você também pode escrever esta chave como `additionalMarketplaces`; consulte [Aliases de chave de marketplace](#marketplace-key-aliases).

Defina `"autoUpdate": true` ao lado de `source` para fazer Claude Code atualizar esse marketplace e atualizar seus plugins instalados em segundo plano após a inicialização. Quando omitido, `claude-plugins-official` e a maioria dos outros marketplaces oficiais da Anthropic padrão para `true`, e marketplaces de terceiros padrão para `false`. Consulte [Configurar auto-atualizações](/docs/pt/plugins/install#keep-plugins-updated).

Quando mais de um arquivo de configurações define uma entrada de marketplace sob o mesmo nome, Claude Code usa a entrada do arquivo de [precedência mais alta](/docs/pt/settings#settings-precedence) inteiro. Essa entrada substitui a entrada de precedência mais baixa e não herda nenhum de seus campos, portanto uma redefinição não pode combinar `source.headers` de credencial de um arquivo com uma URL que outro arquivo controla. Antes de v2.1.228, Claude Code mesclava entradas de mesmo nome campo por campo, portanto uma entrada em um arquivo de precedência mais alta poderia herdar campos que não definiu, incluindo `headers` de outro arquivo.

<h4 id="marketplace-source-types">
  Tipos de fonte de marketplace
</h4>

O objeto `source` toma uma destas formas:

* **`github`**: um repositório GitHub, com `repo`
* **`git`**: qualquer URL git, com `url`
* **`url`**: uma URL direta para um arquivo `marketplace.json`, com `url` e `headers` opcional e `headersHelper` para acesso autenticado. `headersHelper` nomeia um comando que imprime cabeçalhos cujos valores são muito efêmeros para listar em `headers`, e requer Claude Code v2.1.238 ou posterior
* **`file`**: um caminho local para um arquivo `marketplace.json`, com `path`
* **`directory`**: um caminho do sistema de arquivos local, com `path`, apenas para desenvolvimento
* **`settings`**: um marketplace inline declarado diretamente no arquivo de configurações sem um repositório hospedado, com `name` e `plugins`

O tipo de fonte `git` funciona com qualquer serviço de hospedagem git, incluindo GitLab auto-hospedado e Bitbucket. Claude Code clona o repositório com a mesma autenticação que `git clone` usaria nessa máquina: helpers de credencial configurados ou chaves SSH. Um token de provedor como `GITHUB_TOKEN` entra em vigor através de um helper de credencial que o lê. Consulte [Repositórios privados](/docs/pt/plugins/host-marketplace#grant-access-to-a-private-marketplace) para detalhes de configuração.

Para fontes `github` e `git`, Claude Code nunca baixa conteúdo de [Git LFS](https://git-lfs.com) quando clona o repositório de marketplace para adicioná-lo ou atualizá-lo. Arquivos rastreados por LFS são verificados como arquivos de ponteiro, e a saída de adição ou atualização relata quantos.

O campo `skipLfs` dentro do objeto `source` é aceito e não tem efeito. Antes de v2.1.274, Claude Code baixava conteúdo de LFS a menos que você definisse `"skipLfs": true`.

Para uma fonte `url`, defina `headersHelper` dentro do objeto `source` quando a credencial em `headers` expira e um comando tem que produzir uma nova. Requer Claude Code v2.1.238 ou posterior. Para o que o comando deve imprimir e onde Claude Code o executa, consulte [Escrever o comando headersHelper](/docs/pt/plugins/host-marketplace#write-the-headershelper-command), e para os casos onde Claude Code não o executa, consulte [Quando Claude Code pula um comando headersHelper](/docs/pt/plugins/host-marketplace#when-claude-code-skips-a-headershelper-command-or-drops-its-output). Uma vez que você defina `headersHelper` em uma URL de marketplace `https://`, Claude Code executa o comando em dois pontos, reutilizando a saída de uma execução por até 60 segundos:

* Antes de cada busca desse `marketplace.json` do marketplace, incluindo uma atualização posterior. Claude Code envia os cabeçalhos impressos com essa busca.
* Antes de cada download de arquivo de plugin na origem da URL do marketplace, significando o mesmo esquema, host e porta. Claude Code envia a saída com esse download, e nenhum outro download obtém os cabeçalhos.

Claude Code ignora qualquer `headersHelper` definido no `.claude/settings.json` ou `.claude/settings.local.json` de um diretório que você adiciona com [`--add-dir`](/docs/pt/permissions#what-runs-before-you-trust-a-folder), em uma fonte `url` e em uma entrada de plugin inline, e envia apenas os `headers` fixos definidos naquele arquivo. [Como usuários aceitam um comando headersHelper](/docs/pt/plugins/host-marketplace#how-users-accept-a-headershelper-command) cobre os outros arquivos de configurações.

Plugins listados em uma fonte `settings` devem referenciar fontes externas como GitHub ou npm, e o `name` deve corresponder à chave de marketplace. Você ainda ativa cada plugin separadamente em `enabledPlugins`. Este exemplo declara um plugin inline:

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

Uma entrada de plugin sob `source: 'settings'` cuja própria `source` é um [`archive`](/docs/pt/plugins/marketplace-reference#archive-plugin-source) pode definir `headers` para o download de arquivo. Se o valor que você colocaria em `headers` é efêmero, como um token que seu registro cria sob demanda, defina um comando `headersHelper` em vez disso. Uma entrada pode definir ambos. Ambos os campos requerem Claude Code v2.1.238 ou posterior.

Claude Code envia os `headers` da entrada, e o que o comando imprime, com o download de arquivo daquele plugin e com nenhum outro download. Claude Code executa o comando apenas quando um usuário [instala ou atualiza apenas aquele plugin](/docs/pt/plugins/host-marketplace#how-users-accept-a-headershelper-command). Três regras adicionais dependem de qual arquivo contém a entrada:

* **`strict`**: diferentemente de uma entrada no `marketplace.json` de um marketplace, uma entrada em configurações não precisa de `"strict": false`, porque um arquivo de configurações não carrega campos de manifesto para inline. Consulte [Modo strict](/docs/pt/plugins/marketplace-reference#strict-mode).
* **Folder trust**: para uma entrada no `.claude/settings.json` ou `.claude/settings.local.json` de um projeto, Claude Code executa o comando apenas após o usuário também ter [confiado naquela pasta](/docs/pt/permissions#what-runs-before-you-trust-a-folder).
* **Header filter**: Claude Code descarta [nomes de cabeçalho de roteamento de solicitação e identidade de cliente](/docs/pt/plugins/host-marketplace#when-claude-code-skips-a-headershelper-command-or-drops-its-output) de uma entrada no `.claude/settings.json` ou `.claude/settings.local.json` de um projeto, porque um repositório pode fornecer esses arquivos. Claude Code aplica o mesmo filtro a uma entrada de catálogo e a uma entrada no diretório de configurações `--add-dir`, e nenhum filtro a uma entrada em suas configurações de usuário, um arquivo `--settings` ou configurações gerenciadas.

<h4 id="marketplace-key-aliases">
  Aliases de chave de marketplace
</h4>

No Claude Code v2.1.232 ou posterior, você pode escrever `extraKnownMarketplaces` como `additionalMarketplaces` e `strictKnownMarketplaces` como `allowedMarketplaces`. Claude Code trata cada alias como segue:

* Versões anteriores ignoram o alias, portanto mantenha a grafia canônica em um arquivo que versões mais antigas também leem, como um arquivo de configurações gerenciadas para uma frota com versões mistas de Claude Code.
* Em qualquer arquivo de configurações que aceita a chave canônica, Claude Code lê o alias exatamente como lê a chave canônica.
* Claude Code pode reescrever `additionalMarketplaces` para `extraKnownMarketplaces` quando atualiza o arquivo.
* Se você definir ambas as grafias em um arquivo, Claude Code usa o valor canônico e ignora o alias.

<h3 id="pluginconfigs">
  `pluginConfigs`
</h3>

Armazene as respostas não sensíveis que você dá ao diálogo de configuração [`userConfig`](/docs/pt/plugins/manifest-reference#user-configuration) de um plugin, codificadas por ID de plugin. Claude Code escreve esta chave para suas configurações de usuário quando você preenche o diálogo, portanto você não precisa editá-la manualmente. Claude Code armazena opções sensíveis no Keychain do macOS em vez disso, voltando para `~/.claude/.credentials.json` quando o Keychain rejeita a escrita; em plataformas sem um keychain suportado, armazena em `~/.claude/.credentials.json`.

* **Scope**: [`User or managed`](#scopes)
* **Type**: objeto mapeando um ID de plugin para um objeto com um campo `options`, mapeando cada nome de opção para uma string, número, Boolean ou array de strings, e um campo `mcpServers` opcional mantendo valores de configuração de usuário por servidor na mesma forma
* **Default**: não definido

Este exemplo armazena a opção `api_endpoint` para o plugin `deployer` de `acme-tools`:

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

Plugins integrados armazenam suas opções sob a mesma chave com um sufixo `@builtin`. Por exemplo, a configuração [**Project instructions**](/docs/pt/memory#choose-which-instruction-files-load) que controla se Claude Code lê arquivos `AGENTS.md` é `pluginConfigs["agents-md@builtin"].options.instructionFiles`.

Claude Code ignora entradas de projeto e local porque substitui esses valores em configurações de hook de plugin, MCP e LSP, e um repositório clonado não deve ser capaz de fornecê-los. Antes de v2.1.207, configurações de projeto e local também eram lidas.

<h2 id="mcp">
  MCP
</h2>

Controle quais servidores MCP o Claude Code se conecta e quais uma organização permite. Veja [Conectar a ferramentas externas com MCP](/docs/pt/mcp) e [Configuração MCP gerenciada](/docs/pt/managed-mcp).

<h3 id="allowallclaudeaimcps">
  `allowAllClaudeAiMcps`
</h3>

Carregue os [conectores claude.ai](/docs/pt/mcp#use-mcp-servers-from-claude-ai) que o Claude Code busca por si mesmo junto com um `managed-mcp.json` implantado. Sem essa chave, `managed-mcp.json` assume controle exclusivo dos servidores MCP e suprime esses conectores.

* **Escopo**: [`Managed`](#scopes). Os usuários não podem reativar conectores que o controle exclusivo suprimiu.
* **Tipo**: Booleano
  * `true`: Claude Code carrega os conectores claude.ai junto com um `managed-mcp.json` implantado
  * `false`: um `managed-mcp.json` implantado assume controle exclusivo dos servidores MCP e suprime os conectores claude.ai [que o Claude Code busca por si mesmo](/docs/pt/mcp#how-connectors-reach-claude-code)
* **Padrão**: `false`, portanto um `managed-mcp.json` implantado suprime os conectores claude.ai que o Claude Code busca por si mesmo

```json managed-settings.json theme={null}
{
  "allowAllClaudeAiMcps": true
}
```

[`allowedMcpServers`](#allowedmcpservers) e [`deniedMcpServers`](#deniedmcpservers) ainda se aplicam aos conectores que essa chave carrega. Os conectores entregues a uma [sessão na nuvem](/docs/pt/claude-code-on-the-web) cujo host carrega um `managed-mcp.json`, como um executor auto-hospedado, permanecem suprimidos. Veja [Permitir conectores claude.ai junto com o conjunto gerenciado](/docs/pt/managed-mcp#allow-claude-ai-connectors-alongside-the-managed-set).

<h3 id="allowedmcpservers">
  `allowedMcpServers`
</h3>

Crie uma lista de permissões dos servidores MCP que as pessoas podem adicionar. O Claude Code bloqueia qualquer servidor que não corresponda a uma entrada onde quer que seja definido, incluindo servidores de plugins, servidores passados com `--mcp-config` e servidores do claude.ai.

Servidores integrados como Claude no Chrome, o servidor `ide` ao qual o Claude Code se conecta em um [VS Code](/docs/pt/vs-code#the-built-in-ide-mcp-server) ou [JetBrains](/docs/pt/jetbrains#the-built-in-ide-mcp-server) IDE em execução, e servidores que a própria CLI configura estão isentos da lista de permissões, e a lista de negação ainda se aplica a eles. Servidores `type: "sdk"` em processo estão isentos de ambas as listas; o [aplicativo que iniciou a sessão](/docs/pt/mcp#how-connectors-reach-claude-code) os registra.

Os servidores que sua organização entrega também estão isentos da lista de permissões, e a lista de negação ainda se aplica a eles. A isenção cobre cada entrada [`managedMcpServers`](#managedmcpservers) e qualquer entrada [`managed-mcp.json`](/docs/pt/managed-mcp#exclusive-control-with-managed-mcp-json) cujos valores não usam expansão `${VAR}`. Veja [Como um servidor é avaliado](/docs/pt/managed-mcp#how-a-server-is-evaluated) para a ordem de verificação completa. Antes da v2.1.259, servidores do `managed-mcp.json` também tinham que corresponder.

* **Escopo**: [`Any file`](#scopes). As entradas de cada arquivo se mesclam em uma lista de permissões, a menos que [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly) esteja definido. Implante-o em configurações gerenciadas para aplicá-lo.
* **Tipo**: matriz de objetos, cada um com exatamente uma chave: `serverName`, uma string limitada a letras, números, hífens e sublinhados; `serverCommand`, uma matriz do comando e seus argumentos correspondidos exatamente; ou `serverUrl`, um padrão de URL com curingas `*`
* **Padrão**: não definido, portanto cada servidor é permitido; uma matriz vazia bloqueia cada servidor que os usuários adicionam

Este exemplo permite apenas o servidor stdio que o comando `npx` listado inicia:

```json settings.json theme={null}
{
  "allowedMcpServers": [
    { "serverCommand": ["npx", "-y", "@modelcontextprotocol/server-filesystem"] }
  ]
}
```

Uma entrada [`deniedMcpServers`](#deniedmcpservers) tem precedência, portanto um servidor em ambas as listas é bloqueado. Uma vez que a lista contém qualquer entrada `serverCommand`, um servidor stdio deve corresponder a uma entrada `serverCommand`, e uma vez que contém qualquer entrada `serverUrl`, um servidor remoto deve corresponder a uma entrada `serverUrl`: uma correspondência `serverName` não admite mais esse tipo de servidor. Veja [Controle baseado em política com listas de permissões e negação](/docs/pt/managed-mcp#policy-based-control-with-allowlists-and-denylists).

<h3 id="allowmanagedmcpserversonly">
  `allowManagedMcpServersOnly`
</h3>

Faça da lista de permissões gerenciada a única que se aplica. O Claude Code então lê [`allowedMcpServers`](#allowedmcpservers) apenas das configurações gerenciadas e ignora listas de permissões nas configurações de usuário, projeto e local; [`deniedMcpServers`](#deniedmcpservers) ainda se mescla de cada escopo de configurações, portanto os usuários ainda podem bloquear servidores para si mesmos. Os administradores o definem para que as próprias configurações de um usuário não possam ampliar o que a lista de permissões gerenciada permite.

* **Escopo**: [`Managed`](#scopes)
* **Tipo**: Booleano
  * `true`: Claude Code lê `allowedMcpServers` apenas das configurações gerenciadas e ignora listas de permissões nas configurações de usuário, projeto e local
  * `false`: listas de permissões de cada escopo de configurações se mesclam
* **Padrão**: `false`, portanto listas de permissões de cada escopo de configurações se mesclam

Este exemplo bloqueia a lista de permissões para configurações gerenciadas e permite apenas o servidor nomeado `github`:

```json managed-settings.json theme={null}
{
  "allowManagedMcpServersOnly": true,
  "allowedMcpServers": [
    { "serverName": "github" }
  ]
}
```

Os usuários ainda podem adicionar servidores MCP por conta própria; apenas servidores que correspondem à lista de permissões gerenciada são carregados. Veja [Restringir a lista de permissões apenas às configurações gerenciadas](/docs/pt/managed-mcp#restrict-the-allowlist-to-managed-settings-only).

<h3 id="deniedmcpservers">
  `deniedMcpServers`
</h3>

Bloqueie servidores MCP específicos. O Claude Code recusa carregar um servidor correspondente onde quer que seja definido, incluindo servidores de plugins, servidores passados com `--mcp-config`, servidores do `managed-mcp.json`, servidores do [`managedMcpServers`](#managedmcpservers) e os conectores claude.ai [que ele busca por si mesmo](/docs/pt/mcp#how-connectors-reach-claude-code). Servidores `type: "sdk"` em processo estão isentos; o aplicativo que iniciou a sessão os registra.

* **Escopo**: [`Any file`](#scopes). As entradas de cada arquivo se mesclam em uma lista de negação, e [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly) não muda isso. Implante-o em configurações gerenciadas para aplicá-lo.
* **Tipo**: matriz de objetos, cada um com exatamente uma chave: `serverName`, uma string, portanto o nome de exibição de um conector claude.ai como `"claude.ai Slack"` funciona; `serverCommand`, uma matriz do comando e seus argumentos correspondidos exatamente; ou `serverUrl`, um padrão de URL com curingas `*`
* **Padrão**: não definido, portanto nenhum servidor é bloqueado; uma matriz vazia também não bloqueia nada

```json settings.json theme={null}
{
  "deniedMcpServers": [
    { "serverName": "filesystem" }
  ]
}
```

A lista de negação tem precedência sobre [`allowedMcpServers`](#allowedmcpservers), portanto um servidor em ambas as listas é bloqueado. Veja [Controle baseado em política com listas de permissões e negação](/docs/pt/managed-mcp#policy-based-control-with-allowlists-and-denylists).

<h3 id="disableclaudeaiconnectors">
  `disableClaudeAiConnectors`
</h3>

Desative os [conectores MCP claude.ai](/docs/pt/mcp#use-mcp-servers-from-claude-ai) [que o Claude Code busca por si mesmo](/docs/pt/mcp#how-connectors-reach-claude-code), para que ele não os busque nem se conecte a eles. Um `true` em qualquer arquivo de configurações se aplica: um `.claude/settings.json` de projeto verificado pode optar por desativar esses conectores para um repositório, mas um `false` no nível do projeto não pode substituir um `true` no nível do usuário ou gerenciado.

* **Escopo**: [`Any file`](#scopes)
* **Tipo**: Booleano
  * `true`: Claude Code não busca nem se conecta a esses conectores
  * `false`: o mesmo que não definido; Claude Code busca seus conectores a menos que outro arquivo de configurações ou `ENABLE_CLAUDEAI_MCP_SERVERS` os desative
* **Padrão**: `false`, portanto Claude Code busca seus conectores
* **Substituições por sessão**: [`ENABLE_CLAUDEAI_MCP_SERVERS`](/docs/pt/env-vars) definido como `false` desativa conectores para uma sessão; qualquer um dos dois que os desativa, o outro não pode ligá-los novamente

```json settings.json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

Os servidores que você passa explicitamente com `--mcp-config` não são afetados. Para bloquear conectores individuais em vez de todos eles, use [`deniedMcpServers`](#deniedmcpservers). Veja [Desativar conectores claude.ai](/docs/pt/mcp#disable-claude-ai-connectors).

<h3 id="disabledmcpjsonservers">
  `disabledMcpjsonServers`
</h3>

Rejeite servidores específicos definidos no arquivo `.mcp.json` de um projeto para que o Claude Code nunca se conecte a eles ou peça sua aprovação. Uma rejeição em qualquer arquivo de configurações se aplica, incluindo um `.claude/settings.json` de projeto verificado no repositório.

* **Escopo**: [`Any file`](#scopes)
* **Tipo**: matriz de strings, os nomes dos servidores conforme aparecem em `.mcp.json`
* **Padrão**: não definido

```json settings.json theme={null}
{
  "disabledMcpjsonServers": ["filesystem"]
}
```

O Claude Code escreve essa chave em `.claude/settings.local.json` quando você rejeita um servidor na caixa de diálogo de aprovação. `claude mcp get <name>` mostra um servidor rejeitado como `✘ Rejected (see disabledMcpjsonServers in settings)`. A rejeição tem precedência sobre [`enabledMcpjsonServers`](#enabledmcpjsonservers) e [`enableAllProjectMcpServers`](#enableallprojectmcpservers).

<h3 id="enableallprojectmcpservers">
  `enableAllProjectMcpServers`
</h3>

Aprove cada servidor MCP definido em arquivos `.mcp.json` de projeto sem um prompt. O Claude Code escreve essa chave em `.claude/settings.local.json` quando você escolhe aprovar todos os servidores na caixa de diálogo de aprovação.

* **Escopo**: [`Any file`](#scopes). Em uma pasta cuja caixa de diálogo de confiança você não aceitou, o Claude Code a honra das configurações de usuário, configurações gerenciadas e `--settings` e a ignora no arquivo de projeto compartilhado, tanto na sessão quanto para `claude mcp list` e `claude mcp get`; [Aprovações de servidor de projeto e confiança de espaço de trabalho](/docs/pt/mcp#project-server-approvals-and-workspace-trust) diz quando um `.claude/settings.local.json` não rastreado conta também.
* **Tipo**: Booleano
  * `true`: Claude Code aprova cada servidor MCP definido em arquivos `.mcp.json` de projeto sem um prompt
  * `false`: Claude Code pede que você aprove cada servidor. Em uma pasta confiável, um `false` em um arquivo de precedência mais alta substitui um `true` em um de precedência mais baixa; em uma pasta que você não confiou, um `true` em qualquer arquivo honrado é suficiente
* **Padrão**: não definido, portanto Claude Code pede que você aprove cada servidor

```json settings.json theme={null}
{
  "enableAllProjectMcpServers": true
}
```

Uma entrada [`disabledMcpjsonServers`](#disabledmcpjsonservers) ainda rejeita um servidor.

<h3 id="enabledmcpjsonservers">
  `enabledMcpjsonServers`
</h3>

Aprove servidores específicos definidos em arquivos `.mcp.json` de projeto para que o Claude Code se conecte a eles sem perguntar. O Claude Code escreve essa chave em `.claude/settings.local.json` quando você aprova um servidor na caixa de diálogo de aprovação.

* **Escopo**: [`Any file`](#scopes). Em uma pasta cuja caixa de diálogo de confiança você não aceitou, o Claude Code a honra das configurações de usuário, configurações gerenciadas e `--settings` e a ignora no arquivo de projeto compartilhado, tanto na sessão quanto para `claude mcp list` e `claude mcp get`; [Aprovações de servidor de projeto e confiança de espaço de trabalho](/docs/pt/mcp#project-server-approvals-and-workspace-trust) diz quando um `.claude/settings.local.json` não rastreado conta também.
* **Tipo**: matriz de strings, os nomes dos servidores conforme aparecem em `.mcp.json`
* **Padrão**: não definido

Este exemplo aprova os servidores `memory` e `github` do `.mcp.json` do projeto:

```json settings.json theme={null}
{
  "enabledMcpjsonServers": ["memory", "github"]
}
```

Uma entrada [`disabledMcpjsonServers`](#disabledmcpjsonservers) ainda rejeita um servidor.

<h3 id="managedmcpservers">
  `managedMcpServers`
</h3>

Forneça servidores MCP remotos para cada usuário a partir de configurações gerenciadas. Os usuários mantêm os servidores que adicionam por conta própria e não podem editar ou remover os que você fornece. Requer Claude Code v2.1.259 ou posterior.

* **Escopo**: [`Managed`](#scopes). O Claude Code descarta a chave com um aviso nas configurações de usuário, projeto e local, e não a lê na guia Code do aplicativo Claude Desktop em uma implantação de terceiros ou nas sessões Cowork do aplicativo, onde o Claude Desktop fornece e bloqueia os servidores MCP dessas sessões.
* **Tipo**: objeto com chave de nome do servidor. Cada entrada tem a forma `.mcp.json` para um servidor `http` ou `sse`: uma `url` `https://` obrigatória e opcionalmente `headers`, `oauth` e as outras opções HTTP e SSE. O Claude Code descarta entradas que falham na validação, e [O que uma entrada pode conter](/docs/pt/managed-mcp#what-an-entry-can-contain) lista as condições
* **Padrão**: não definido, portanto as configurações gerenciadas não fornecem servidores

Este exemplo fornece um servidor HTTP nomeado `search`:

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

Para precedência, como servidores fornecidos se combinam com `managed-mcp.json` e as listas de permissões e negação, e o que os usuários veem, veja [Fornecer servidores através de configurações gerenciadas](/docs/pt/managed-mcp#provide-servers-through-managed-settings).

<h2 id="agents-sessions-and-worktrees">
  Agentes, sessões e worktrees
</h2>

Defina o agente padrão, controle colegas de equipe e mensagens entre sessões, e configure worktrees. Veja [Subagentes](/docs/pt/sub-agents) e [Worktrees](/docs/pt/worktrees).

<h3 id="agent">
  `agent`
</h3>

Execute a thread principal como um [subagente](/docs/pt/sub-agents#invoke-subagents-explicitly) nomeado, para que Claude Code aplique o prompt do sistema, restrições de ferramentas e modelo desse subagente à sua sessão. A mesma chave define o agente padrão para sessões que você despacha de `claude agents`.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: string, o nome de um agente integrado ou personalizado
* **Padrão**: não definido, portanto a thread principal é executada como o agente padrão do Claude Code
* **Substituições por sessão**: `--agent` tem precedência sobre esta chave para uma sessão

```json settings.json theme={null}
{
  "agent": "code-reviewer"
}
```

O próprio `settings.json` de um plugin também pode fornecer esta chave; veja [Envie configurações padrão com seu plugin](/docs/pt/plugins/components#default-settings).

<h3 id="crosssessioninbound">
  `crossSessionInbound`
</h3>

Escolha o que esta sessão faz com [mensagens chegando de suas outras sessões do Claude Code](/docs/pt/cross-session-messaging#control-inbound-messages). Quando nenhum valor se aplica, Claude Code decide por mensagem das classes de modo de permissão das duas sessões. Requer Claude Code v2.1.224 ou posterior.

* **Escopo**: [`Qualquer arquivo`](#scopes). Um valor de projeto ou local se aplica apenas quando é mais restritivo do que o valor de configurações gerenciadas, a flag `--settings` ou configurações de usuário fornecem.
* **Tipo**: string, um de:
  * `"accept"`: Claude Code entrega a mensagem ao Claude
  * `"hold"`: Claude Code mostra um aviso para a mensagem sem entregá-la
  * `"refuse"`: Claude Code descarta a mensagem
* **Padrão**: não definido, portanto Claude Code decide por mensagem

```json settings.json theme={null}
{
  "crossSessionInbound": "hold"
}
```

Claude Code lê as configurações gerenciadas primeiro, depois a flag `--settings`, depois as configurações de usuário, e aplica o primeiro valor encontrado. `refuse` é mais restritivo do que `hold`, e `hold` é mais restritivo do que `accept`. Quando nenhuma das fontes confiáveis define um valor, um `hold` ou `refuse` de projeto ou local ainda se aplica, substituindo o padrão por mensagem. Em sessões com mensagens entre sessões, esta chave aparece em `/config` como **Mensagens de suas outras sessões**, que a escreve nas configurações de usuário; a linha requer Claude Code v2.1.232 ou posterior, e Claude Code a oculta enquanto a flag `--settings` ou as configurações gerenciadas definem a chave.

Claude Code [avisa](/docs/pt/errors#crosssessioninbound-must-be-one-of-accept-hold-refuse) quando você define um valor que não reconhece. Enquanto esse valor estiver presente em um arquivo de usuário, projeto, local ou `--settings`, Claude Code retém mensagens de entrada, mesmo quando uma fonte que tem precedência define `accept`. Um `refuse` que outra fonte define ainda se aplica. Corrija ou remova o valor para limpar a retenção.

Quando o valor não reconhecido está em [configurações gerenciadas](/docs/pt/managed-settings), Claude Code o trata como `refuse` até que um administrador o corrija. Antes da v2.1.248, Claude Code ignorava um valor não reconhecido sem aviso.

<h3 id="disableagentview">
  `disableAgentView`
</h3>

Desative [agentes de fundo e visualização de agente](/docs/pt/agent-view): `claude agents`, `--bg`, `/background` e o supervisor sob demanda. Defina-o em [configurações gerenciadas](/docs/pt/managed-settings) para aplicá-lo a uma organização.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: Boolean
  * `true`: Claude Code desativa `claude agents`, `--bg`, `/background` e o supervisor sob demanda
  * `false`: a visualização de agente está disponível
* **Padrão**: não definido, portanto a visualização de agente está disponível
* **Substituições por sessão**: [`CLAUDE_CODE_DISABLE_AGENT_VIEW`](/docs/pt/env-vars) desativa a visualização de agente para uma sessão; qualquer um dos dois que a desativar, o outro não pode reativá-la

```json settings.json theme={null}
{
  "disableAgentView": true
}
```

<h3 id="isolatepeermachines">
  `isolatePeerMachines`
</h3>

Exija sua aprovação explícita antes que `SendMessage` do Claude alcance uma de suas sessões além desta máquina; veja [Exigir aprovação para mensagens entre máquinas](/docs/pt/cross-session-messaging#require-approval-for-cross-machine-messages). O prompt de aprovação aparece mesmo no [modo `bypassPermissions`](/docs/pt/permission-modes#skip-all-checks-with-bypasspermissions-mode).

* **Escopo**: [`Qualquer arquivo`](#scopes). Um `true` de qualquer escopo se aplica, portanto um arquivo de projeto verificado pode ativar o requisito, mas não desativá-lo.
* **Tipo**: Boolean
  * `true`: Claude Code pede sua aprovação antes que `SendMessage` do Claude alcance uma de suas sessões além desta máquina
  * `false`: mensagens entre máquinas não solicitam
* **Padrão**: não definido, portanto mensagens entre máquinas não solicitam

```json settings.json theme={null}
{
  "isolatePeerMachines": true
}
```

A aprovação de `SendMessage` entre máquinas requer Claude Code v2.1.224 ou posterior.

<h3 id="processwrapper">
  `processWrapper`
</h3>

No macOS e Linux, coloque um comando de inicializador corporativo na frente dos [processos de fundo que Claude Code inicia](/docs/pt/corporate-launcher#what-the-launcher-covers). Claude Code executa o inicializador com sua própria linha de comando anexada, portanto o inicializador deve executar no Claude Code; veja [Execute Claude Code atrás de um inicializador corporativo](/docs/pt/corporate-launcher) para o contrato do inicializador. Requer Claude Code v2.1.210 ou posterior.

* **Escopo**: [`Usuário ou gerenciado`](#scopes)
* **Tipo**: string, o comando do inicializador como um prefixo argv, como um caminho absoluto com argumentos opcionais
* **Padrão**: não definido, portanto os processos de fundo iniciam sem encapsulamento
* **Substituições por sessão**: [`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/pt/env-vars) tem precedência sobre esta chave para uma sessão

```json settings.json theme={null}
{
  "processWrapper": "/opt/corp/launcher --profile claude"
}
```

Claude Code ignora o inicializador no Windows e inicia cada processo sem encapsulamento. Requer Claude Code v2.1.210 ou posterior.

<h3 id="teammatemode">
  `teammateMode`
</h3>

Escolha onde Claude Code mostra colegas de equipe da [equipe de agentes](/docs/pt/agent-teams): dentro do seu painel de terminal principal, ou em painéis divididos quando seu terminal os suporta. Veja [Escolha um modo de exibição](/docs/pt/agent-teams#choose-a-display-mode).

* **Escopo**: [`Qualquer arquivo`](#scopes). Claude Code também lê um valor deixado em `~/.claude.json` por versões antigas.
* **Tipo**: string, um de:
  * `"in-process"`: colegas de equipe são executados dentro do seu painel de terminal principal
  * `"auto"`: painéis divididos quando você está executando dentro do tmux, ou dentro do iTerm2 com `it2` no seu `PATH` ou tmux instalado; em processo caso contrário
  * `"tmux"`: painéis divididos usando tmux ou iTerm2, detectados do seu terminal
  * `"iterm2"`: painéis divididos nativos do iTerm2 através do CLI `it2`
* **Padrão**: `"in-process"`
* **Substituições por sessão**: `--teammate-mode` tem precedência sobre esta chave para uma sessão

```json settings.json theme={null}
{
  "teammateMode": "auto"
}
```

<span id="worktree-settings" />

<h3 id="worktree">
  `worktree`
</h3>

Configure como Claude Code cria e gerencia [git worktrees](/docs/pt/worktrees) para `--worktree`, a ferramenta `EnterWorktree` e subagentes isolados e sessões de fundo.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: objeto com `baseRef`, `symlinkDirectories`, `sparsePaths` e `bgIsolation`
* **Padrão**: não definido

Este exemplo ramifica novas worktrees do seu `HEAD` atual e cria links simbólicos de `node_modules` em cada uma:

```json settings.json theme={null}
{
  "worktree": {
    "baseRef": "head",
    "symlinkDirectories": ["node_modules"]
  }
}
```

Para copiar arquivos ignorados pelo git como `.env` em novas worktrees, adicione um [arquivo `.worktreeinclude`](/docs/pt/worktrees#copy-gitignored-files-into-worktrees) à raiz do seu projeto em vez de uma configuração.

<h3 id="worktree-baseref">
  `worktree.baseRef`
</h3>

Escolha de qual ref novas worktrees se ramificam. `"fresh"` se ramifica de `origin/<default-branch>` para uma árvore limpa correspondente ao remoto; `"head"` se ramifica do seu `HEAD` local atual, portanto commits não enviados e estado de branch de recurso estão presentes na worktree.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: string, um de:
  * `"fresh"`: novas worktrees se ramificam de `origin/<default-branch>`
  * `"head"`: novas worktrees se ramificam do seu `HEAD` local atual, incluindo commits não enviados
* **Padrão**: `"fresh"`

```json settings.json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

Dentro de uma worktree vinculada, `"head"` resolve para o `HEAD` dessa worktree, não para o checkout principal.

<h3 id="worktree-symlinkdirectories">
  `worktree.symlinkDirectories`
</h3>

Crie links simbólicos de diretórios do repositório principal em cada worktree para que você não duplique diretórios grandes no disco.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: array de strings, caminhos de diretório relativos à raiz do repositório
* **Padrão**: não definido, portanto Claude Code não cria links simbólicos de nenhum diretório

Este exemplo cria links simbólicos de `node_modules` e `.cache` do repositório principal em cada nova worktree:

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

Faça checkout apenas dos diretórios listados em cada worktree através do git sparse-checkout. Claude Code escreve apenas esses diretórios mais arquivos no nível raiz no disco, o que é mais rápido em monorepos grandes; veja [Faça checkout apenas dos diretórios que você precisa](/docs/pt/large-codebases#check-out-only-the-directories-you-need).

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: array de strings, caminhos de diretório relativos à raiz do repositório
* **Padrão**: não definido, portanto cada worktree faz checkout de toda a árvore

Este exemplo faz checkout apenas de `packages/my-app` e `shared/utils`, mais arquivos no nível raiz, em cada worktree:

```json settings.json theme={null}
{
  "worktree": {
    "sparsePaths": ["packages/my-app", "shared/utils"]
  }
}
```

Enquanto uma worktree esparsa existe, git ativa `extensions.worktreeConfig` no `.git/config` compartilhado do repositório.

<h3 id="worktree-bgisolation">
  `worktree.bgIsolation`
</h3>

Escolha como [sessões de fundo](/docs/pt/agent-view#how-file-edits-are-isolated) isolam suas edições de arquivo. Com `"worktree"`, Claude Code bloqueia `Edit` e `Write` no checkout principal até que a sessão chame `EnterWorktree`; com `"none"`, trabalhos de fundo editam a cópia de trabalho diretamente. Defina `"none"` para um repositório onde git worktrees são impraticáveis.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: string, um de:
  * `"worktree"`: Claude Code bloqueia `Edit` e `Write` no checkout principal até que a sessão chame `EnterWorktree`
  * `"none"`: trabalhos de fundo editam a cópia de trabalho diretamente
* **Padrão**: `"worktree"`

```json settings.json theme={null}
{
  "worktree": {
    "bgIsolation": "none"
  }
}
```

Fora de um repositório git, um [hook `WorktreeCreate`](/docs/pt/worktrees#non-git-version-control) que falha libera o bloqueio para que a sessão possa editar o diretório de trabalho no local; essa liberação requer Claude Code v2.1.203 ou posterior.

<h2 id="remote-desktop-and-notifications">
  Remoto, desktop e notificações
</h2>

Configure o Controle Remoto, ambientes em nuvem, o aplicativo desktop e as notificações que o Claude Code envia quando precisa de você. Veja [Controle Remoto](/docs/pt/remote-control).

<h3 id="agentpushnotifenabled">
  `agentPushNotifEnabled`
</h3>

Permita que o Claude envie uma notificação por push para seu telefone quando decidir que vale a pena enviar uma, por exemplo, quando uma tarefa longa termina. O Claude Code sincroniza essa escolha com sua conta, e as notificações chegam enquanto o [Controle Remoto](/docs/pt/remote-control) está conectado. Aparece em `/config` como **Push quando Claude decidir**.

* **Escopo**: [`Qualquer arquivo`](#scopes). O Claude Code também lê um valor deixado em `~/.claude.json` por versões antigas.
* **Tipo**: Boolean
  * `true`: Claude pode enviar uma notificação por push para seu telefone quando decidir que vale a pena enviar uma
  * `false`: Claude não envia essas notificações
* **Padrão**: `false`

```json settings.json theme={null}
{
  "agentPushNotifEnabled": true
}
```

Veja [Notificações por push móvel](/docs/pt/remote-control#mobile-push-notifications).

<h3 id="awaysummaryenabled">
  `awaySummaryEnabled`
</h3>

Mostre um resumo de sessão de uma linha quando você retorna ao terminal após alguns minutos ausente. Defina como `false`, ou desative **Resumo da sessão** em `/config`, para parar o resumo.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: Boolean
  * `true`: você vê um resumo de sessão de uma linha quando retorna após alguns minutos ausente
  * `false`: Claude Code não mostra nenhum resumo
* **Padrão**: não definido, então o resumo está ativado
* **Substituições por sessão**: [`CLAUDE_CODE_ENABLE_AWAY_SUMMARY`](/docs/pt/env-vars) tem precedência sobre essa chave para uma sessão, em qualquer direção

```json settings.json theme={null}
{
  "awaySummaryEnabled": false
}
```

O Claude Code nunca mostra o resumo em modo não interativo.

<h3 id="disableartifact">
  `disableArtifact`
</h3>

<Warning>
  Descontinuado e substituído por [`enableArtifact`](#enableartifact). O Claude Code ainda honra `disableArtifact: true` como equivalente a `enableArtifact: false`, e ignora `disableArtifact: false`.
</Warning>

Use [`enableArtifact`](#enableartifact) em vez disso para desativar a ferramenta [Artifact](/docs/pt/artifacts), que publica a saída da sessão como uma página da web privada em claude.ai. Quando você desativa a linha **Artifacts** em `/config`, o Claude Code escreve `enableArtifact` em suas configurações de usuário e limpa essa chave.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: Boolean
  * `true`: Claude Code desativa a ferramenta Artifact para cada sessão à qual o arquivo se aplica, e nenhum outro arquivo a ativa novamente. Antes da v2.1.242, um arquivo com precedência mais alta poderia substituir um `true` de um arquivo com precedência mais baixa em vez da chave agir como um bloqueio
  * `false`: ignorado; para deixar a ferramenta ativada, remova a chave
* **Padrão**: não definido, então a ferramenta segue a [disponibilidade](/docs/pt/artifacts#availability) de sua conta
* **Substituições por sessão**: [`CLAUDE_CODE_DISABLE_ARTIFACT`](/docs/pt/env-vars) definido como `1` desativa a ferramenta para uma sessão

```json settings.json theme={null}
{
  "disableArtifact": true
}
```

[Desativar artifacts](/docs/pt/artifacts#disable-artifacts) lista todas as maneiras de desativar a ferramenta.

<h3 id="disabledeeplinkregistration">
  `disableDeepLinkRegistration`
</h3>

Impeça que o Claude Code registre o manipulador de protocolo `claude-cli://` com o sistema operacional, o que ele faz após você enviar o primeiro prompt de uma sessão interativa. [Deep links](/docs/pt/deep-links) permitem que ferramentas externas abram uma sessão do Claude Code com um prompt pré-preenchido. Defina isso em ambientes onde o registro do manipulador de protocolo é restrito ou gerenciado separadamente.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: a string `"disable"`
* **Padrão**: não definido, então Claude Code registra o manipulador

```json settings.json theme={null}
{
  "disableDeepLinkRegistration": "disable"
}
```

<h3 id="disabledesktoplocalsessions">
  `disableDesktopLocalSessions`
</h3>

Desative sessões de Code que executam no dispositivo no [aplicativo desktop](/docs/pt/desktop#local-sessions-on-managed-devices), para implantações onde os desenvolvedores devem trabalhar em máquinas remotas via SSH. Na aba Code, o ambiente **Local** permanece no menu suspenso de ambiente, mas fica acinzentado e não pode ser selecionado, com uma dica de ferramenta dizendo que sua organização o desativou; no Windows, a entrada WSL fica acinzentada da mesma forma, embora se as sessões WSL executam em um dispositivo gerenciado seja [governado separadamente](/docs/pt/admin-setup#wsl-sessions-in-claude-code-desktop). Novas sessões usam como padrão a primeira [conexão SSH](/docs/pt/desktop#ssh-sessions) se uma estiver configurada, e o aplicativo se recusa a iniciar ou retomar uma sessão no dispositivo, incluindo uma conexão SSH de volta para a mesma máquina. Sessões SSH para outros hosts e sessões em nuvem não são afetadas. O aplicativo desktop lê essa chave; o CLI do terminal a ignora. Requer Claude Desktop v1.37937.0 ou posterior.

* **Escopo**: [`Gerenciado`](#scopes)
* **Tipo**: Boolean; apenas o Boolean JSON `true` tem efeito
  * `true`: o aplicativo desktop não oferece sessões de Code no dispositivo; sessões locais existentes permanecem listadas, mas não podem continuar
  * `false`: sessões locais permanecem disponíveis
* **Padrão**: não definido, então sessões locais estão disponíveis

```json managed-settings.json theme={null}
{
  "disableDesktopLocalSessions": true
}
```

O aplicativo desktop ignora qualquer outro valor, e um valor que não seja um Boolean, como a string `"true"` ou `1`, também registra um aviso. Combine com [`sshConfigs`](#sshconfigs) para que os usuários cheguem a uma conexão funcionando, e com [`sshHostAllowlist`](#sshhostallowlist) para limitar quais hosts eles podem alcançar. Veja [Sessões locais em dispositivos gerenciados](/docs/pt/desktop#local-sessions-on-managed-devices).

O Claude Desktop fornece sessões de Code com política derivada de sua configuração de desktop, por exemplo, a lista de permissões de saída, sandbox do sistema de arquivos e restrições de MCP em implantações de terceiros. O Claude Code ignora essas configurações pai sempre que uma [fonte de administrador](/docs/pt/managed-settings#how-claude-code-combines-managed-sources) está presente: configurações gerenciadas pelo servidor, uma política de MDM ou nível do SO, ou um arquivo de configurações gerenciadas. Implantar essa chave através de uma delas em um dispositivo que não tinha nenhuma antes, como em implantações de terceiros, portanto, impede que as políticas derivadas do desktop se apliquem. [Deixe um host de incorporação adicionar política](/docs/pt/managed-settings#let-an-embedding-host-add-policy) cobre quando as configurações pai ainda podem se mesclar; isso vale para qualquer chave que você implante dessa forma, não apenas essa.

<h3 id="disableremotecontrol">
  `disableRemoteControl`
</h3>

Desative o [Controle Remoto](/docs/pt/remote-control): Claude Code então recusa `claude remote-control`, a flag `--remote-control`, auto-inicialização e o alternador em sessão, e relata que a política de sua organização o desativou. Coloque em [configurações gerenciadas](/docs/pt/managed-settings) para aplicação de MDM por dispositivo.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: Boolean
  * `true`: Claude Code recusa `claude remote-control`, a flag `--remote-control`, auto-inicialização e o alternador em sessão
  * `false`: Controle Remoto permanece disponível
* **Padrão**: `false`

```json settings.json theme={null}
{
  "disableRemoteControl": true
}
```

<h3 id="enableartifact">
  `enableArtifact`
</h3>

Desative a ferramenta [Artifact](/docs/pt/artifacts), que publica a saída da sessão como uma página da web privada em claude.ai. Quando você desativa a linha **Artifacts** em `/config`, o Claude Code escreve essa chave em suas configurações de usuário, então você geralmente não a edita manualmente. Requer Claude Code v2.1.196 ou posterior.

* **Escopo**: [`Qualquer arquivo`](#scopes). Cada arquivo pode desativar a ferramenta, e nenhum pode ativá-la novamente.
* **Tipo**: Boolean
  * `false`: Claude Code desativa a ferramenta Artifact para cada sessão à qual o arquivo se aplica
  * `true`: o mesmo que deixar a chave não definida, porque nunca substitui um `false` de outro arquivo, de [`CLAUDE_CODE_DISABLE_ARTIFACT`](/docs/pt/env-vars), ou da [configuração de administrador](/docs/pt/artifacts#manage-artifacts-for-your-organization) de sua organização
* **Padrão**: não definido, então a ferramenta segue a [disponibilidade](/docs/pt/artifacts#availability) de sua conta

```json settings.json theme={null}
{
  "enableArtifact": false
}
```

Enquanto uma fonte diferente de suas próprias configurações de usuário mantém a ferramenta desativada, o Claude Code oculta a linha **Artifacts** em `/config`, porque ativá-la lá não mudaria nada. [Desativar artifacts](/docs/pt/artifacts#disable-artifacts) lista todas as maneiras de desativar a ferramenta. Antes da v2.1.242, o Claude Code ignorava essa chave em configurações de projeto e locais, e um arquivo mais alto na [pilha de precedência](/docs/pt/settings#settings-precedence) poderia ativar a ferramenta novamente sobre um `false` de um arquivo mais baixo.

<h3 id="inputneedednotifenabled">
  `inputNeededNotifEnabled`
</h3>

Receba uma notificação por push em seu telefone quando um prompt de permissão ou pergunta estiver aguardando sua entrada. O Claude Code envia essas apenas enquanto o [Controle Remoto](/docs/pt/remote-control) está conectado. Aparece em `/config` como **Push quando ações forem necessárias**.

* **Escopo**: [`Qualquer arquivo`](#scopes). O Claude Code também lê um valor deixado em `~/.claude.json` por versões antigas.
* **Tipo**: Boolean
  * `true`: você recebe uma notificação por push em seu telefone quando um prompt de permissão ou pergunta está aguardando, enquanto o Controle Remoto está conectado
  * `false`: Claude Code não envia essas notificações
* **Padrão**: `false`

```json settings.json theme={null}
{
  "inputNeededNotifEnabled": true
}
```

Veja [Notificações por push móvel](/docs/pt/remote-control#mobile-push-notifications).

<h3 id="preferrednotifchannel">
  `preferredNotifChannel`
</h3>

Escolha como o Claude Code o notifica quando uma tarefa é concluída ou um prompt de permissão está aguardando. Aparece em `/config` como **Notificações locais**.

* **Escopo**: [`Qualquer arquivo`](#scopes). O Claude Code também lê um valor deixado em `~/.claude.json` por versões antigas.
* **Tipo**: string, uma de:
  * `"auto"`: Claude Code envia uma notificação de desktop no iTerm2, Ghostty e Kitty, toca a campainha no Terminal.app apenas quando sua campainha audível está desativada, e não faz nada em outro lugar
  * `"terminal_bell"`: Claude Code toca o caractere de campainha em qualquer terminal
  * `"iterm2"`: Claude Code envia uma notificação de desktop do iTerm2
  * `"iterm2_with_bell"`: Claude Code envia uma notificação de desktop do iTerm2 e toca a campainha
  * `"kitty"`: Claude Code envia uma notificação de desktop do Kitty
  * `"ghostty"`: Claude Code envia uma notificação de desktop do Ghostty
  * `"notifications_disabled"`: Claude Code não envia notificação
* **Padrão**: `"auto"`

```json settings.json theme={null}
{
  "preferredNotifChannel": "terminal_bell"
}
```

Com `"auto"`, o Claude Code envia uma notificação de desktop no iTerm2, Ghostty e Kitty. No Terminal.app, ele toca o caractere de campainha apenas quando você desativou a campainha audível do Terminal, e em outros terminais não faz nada. Defina `"terminal_bell"` para tocar o caractere de campainha em qualquer terminal. Veja [Obter uma campainha de terminal ou notificação](/docs/pt/terminal-config#get-a-terminal-bell-or-notification).

<h3 id="remote-defaultenvironmentid">
  `remote.defaultEnvironmentId`
</h3>

Escolha o [ambiente em nuvem](/docs/pt/cloud-environments) padrão para sessões em nuvem que você cria a partir da CLI, como com `claude --cloud`. O Claude Code escreve essa chave em suas configurações de usuário quando você escolhe um ambiente com [`/remote-env`](/docs/pt/cloud-environments#select-an-environment-from-the-cli).

* **Escopo**: [`Qualquer arquivo`](#scopes). Para um ID de ambiente auto-hospedado, configurações de usuário ou gerenciadas, ou a flag `--settings` apenas.
* **Tipo**: string, um ID de ambiente como `env_...` ou `ccpool_...`
* **Padrão**: não definido, então Claude Code usa o ambiente hospedado pela Anthropic quando sua lista tem um, e caso contrário, o primeiro ambiente em sua lista que não é um [ambiente de ponte de Controle Remoto](/docs/pt/cloud-environments#the-default-environment), ou o primeiro ambiente quando todos são ambientes de ponte
* **Substituições por sessão**: `--environment` tem precedência sobre essa chave para a sessão em nuvem que cria

```json settings.json theme={null}
{
  "remote": {
    "defaultEnvironmentId": "env_0123abcd"
  }
}
```

Um ID de ambiente hospedado pela Anthropic, que começa com `env_`, segue a precedência de configurações padrão, então um valor nas configurações de projeto de um repositório substitui sua escolha no nível de usuário. Um ID de [ambiente auto-hospedado](/docs/pt/self-hosted-environments), que começa com `ccpool_`, é honrado apenas de configurações de usuário, configurações gerenciadas e a flag `--settings`; Claude Code ignora um nas configurações de projeto ou locais de um repositório, e `/remote-env` mostra qual valor foi ignorado, então um arquivo verificado não pode direcionar sessões para um ambiente auto-hospedado que você não escolheu.

<h3 id="remotecontrolatstartup">
  `remoteControlAtStartup`
</h3>

Conecte o [Controle Remoto](/docs/pt/remote-control) automaticamente quando cada sessão interativa inicia, em vez de esperar por `/remote-control`. Defina como `true` para ativar a conexão automática, `false` para desativá-la. Aparece em `/config` como **Ativar Controle Remoto para todas as sessões**.

* **Escopo**: [`Qualquer arquivo`](#scopes). O Claude Code também lê um valor deixado em `~/.claude.json` por versões antigas.
* **Tipo**: Boolean
  * `true`: Claude Code conecta o Controle Remoto automaticamente quando cada sessão interativa inicia
  * `false`: Claude Code espera por `/remote-control`
* **Padrão**: não definido, então a conexão automática segue o padrão de administrador de sua organização quando um está definido, e caso contrário, o padrão atual do Claude Code
* **Substituições por sessão**: `--remote-control` ativa o Controle Remoto para uma sessão mesmo quando essa chave é `false`, e nenhuma flag o desativa para uma sessão

```json settings.json theme={null}
{
  "remoteControlAtStartup": true
}
```

Claude Code ignora um `true` de configurações de projeto ou locais, então um repositório pode desativar a conexão automática para seu checkout, mas não pode ativá-la. Para o comportamento completo por escopo, veja [Ativar Controle Remoto para todas as sessões](/docs/pt/remote-control#enable-remote-control-for-all-sessions) e as [chaves de segurança onde o valor mais restritivo se aplica](/docs/pt/settings#security-keys-where-the-stricter-value-applies).

<h3 id="sshconfigs">
  `sshConfigs`
</h3>

Adicione conexões SSH ao menu suspenso do ambiente [Desktop](/docs/pt/desktop#pre-configure-ssh-connections-for-your-team). Administradores a usam para distribuir conexões compartilhadas para uma equipe. Conexões que você define em configurações gerenciadas aparecem como gerenciadas, então os usuários podem selecioná-las, mas não podem editá-las ou deletá-las no aplicativo.

* **Escopo**: [`Usuário ou gerenciado`](#scopes). O aplicativo desktop lê essa chave.
* **Tipo**: array de objetos, cada um com `id`, `name` e `sshHost` obrigatórios e `sshPort` e `sshIdentityFile` opcionais
* **Padrão**: não definido

Este exemplo adiciona uma conexão chamada `Dev VM` que se conecta a `user@dev.example.com`:

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

Limite os hosts aos quais uma [sessão SSH do Desktop](/docs/pt/desktop#restrict-which-ssh-hosts-users-can-connect-to) pode se conectar. Apenas o aplicativo Desktop lê essa chave; a CLI não. Padrões são insensíveis a maiúsculas: `*` corresponde a qualquer host, `*.example.com` corresponde a `example.com` e cada subdomínio, e qualquer outra coisa é uma correspondência exata contra o nome do host após a resolução de `~/.ssh/config`. Um array vazio desativa sessões SSH.

* **Escopo**: [`Gerenciado`](#scopes)
* **Tipo**: array de padrões de nome de host
* **Padrão**: não definido, então qualquer host é permitido

Este exemplo permite `devboxes.example.com` e seus subdomínios, mais o host exato `bastion.example.com`:

```json managed-settings.json theme={null}
{
  "sshHostAllowlist": ["*.devboxes.example.com", "bastion.example.com"]
}
```

<span id="authentication-and-login" />

<h2 id="authentication-and-providers">
  Autenticação e provedores
</h2>

Forneça credenciais através de scripts auxiliares e, para organizações, force um método de login ou organização. Veja [Autenticação](/docs/pt/authentication).

<h3 id="apikeyhelper">
  `apiKeyHelper`
</h3>

Execute seu próprio comando para produzir a credencial que Claude Code envia com solicitações de modelo. Claude Code executa o comando através do shell do sistema, `/bin/sh` no macOS e Linux e `cmd` no Windows, e envia sua saída como ambos os cabeçalhos `X-Api-Key` e `Authorization: Bearer`. Use-o para credenciais dinâmicas ou rotativas, como tokens de curta duração obtidos de um cofre.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: string, uma linha de comando do shell
* **Padrão**: não definido, portanto Claude Code não executa um auxiliar

```json settings.json theme={null}
{
  "apiKeyHelper": "/bin/generate_temp_api_key.sh"
}
```

Claude Code armazena em cache o valor e executa novamente o comando nestes casos:

* Após o tempo de vida do cache, cinco minutos por padrão ou o intervalo que você define com [`CLAUDE_CODE_API_KEY_HELPER_TTL_MS`](/docs/pt/env-vars).
* Quando uma solicitação para a API Anthropic, diretamente ou através de um [gateway LLM](/docs/pt/llm-gateway), falha com `401` ou `403`.
* Antes de enviar uma solicitação para a API Anthropic, diretamente ou através de um gateway LLM, quando a saída em cache é um JWT que expirou após o auxiliar produzi-lo. Requer Claude Code v2.1.246 ou posterior.

Os dois últimos casos se aplicam apenas quando a saída do auxiliar é a credencial que Claude Code envia e `ANTHROPIC_AUTH_TOKEN` não está definido.

Em sessões interativas, quando o comando vem das configurações do projeto ou local, Claude Code não o executa até que você aceite o prompt de confiança do workspace. Veja [Gerenciamento de credenciais](/docs/pt/authentication#credential-management).

<h3 id="awsauthrefresh">
  `awsAuthRefresh`
</h3>

Execute seu próprio comando, como `aws sso login`, para atualizar as credenciais em seu diretório `.aws` quando as que Claude Code tem para [Amazon Bedrock](/docs/pt/amazon-bedrock) deixarem de funcionar. Claude Code verifica as credenciais atuais em relação ao STS primeiro e executa o comando apenas quando essa verificação falha, depois lê o diretório `.aws` atualizado.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: string, uma linha de comando do shell
* **Padrão**: não definido, portanto Claude Code não atualiza credenciais AWS para você

```json settings.json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile myprofile"
}
```

Use esta chave quando seu fluxo de atualização escreve em `.aws`; use [`awsCredentialExport`](#awscredentialexport) quando ele imprime credenciais. Veja [configuração avançada de credenciais](/docs/pt/amazon-bedrock#advanced-credential-configuration).

<h3 id="awscredentialexport">
  `awsCredentialExport`
</h3>

Execute seu próprio comando que imprime credenciais AWS como JSON, para que Claude Code possa chamar [Amazon Bedrock](/docs/pt/amazon-bedrock) com credenciais que não residem em seu diretório `.aws`. Claude Code aceita a forma de saída `aws sts` e a forma plana `aws configure export-credentials`, e limita as credenciais ao seu próprio cliente Bedrock, portanto os comandos do shell que Claude Code executa ainda veem suas credenciais ambientes.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: string, uma linha de comando do shell
* **Padrão**: não definido, portanto Claude Code usa a cadeia de credenciais AWS ambiente

```json settings.json theme={null}
{
  "awsCredentialExport": "/bin/generate_aws_grant.sh"
}
```

Diferentemente de [`awsAuthRefresh`](#awsauthrefresh), Claude Code sempre executa este comando quando está definido, sem verificar as credenciais ambiente primeiro. Veja [configuração avançada de credenciais](/docs/pt/amazon-bedrock#advanced-credential-configuration).

<h3 id="forceloginmethod">
  `forceLoginMethod`
</h3>

Restrinja qual tipo de conta as pessoas podem usar para fazer login. Defina `"claudeai"` para permitir apenas contas claude.ai, `"console"` para permitir apenas contas Claude Console, ou `"gateway"` para enviar as pessoas para um [gateway na nuvem](/docs/pt/claude-apps-gateway) em vez de um login de primeira parte. Os administradores o definem em configurações gerenciadas e o emparelham com [`forceLoginOrgUUID`](#forceloginorguuid) para manter os logins claude.ai dos desenvolvedores dentro de uma organização. Se você o definir como `"claudeai"` ou `"console"` em qualquer arquivo de configurações, Claude Code também para de oferecer o [login Console sem chave](/docs/pt/authentication#sign-in-without-an-api-key) nas sessões às quais esse arquivo se aplica.

* **Escopo**: [`Qualquer arquivo`](#scopes). Claude Code honra `"gateway"` apenas de uma fonte gerenciada na máquina: `managed-settings.json`, a plist do macOS ou registro HKLM do Windows, ou um auxiliar de política. Ele trata `"gateway"` como não definido em configurações de usuário, projeto, local, HKCU e gerenciadas por servidor, a mesma regra que [`forceLoginGatewayUrl`](#forcelogingatewayurl).
* **Tipo**: string, um de:
  * `"claudeai"`: apenas contas claude.ai podem fazer login
  * `"console"`: apenas contas Claude Console podem fazer login
  * `"gateway"`: Claude Code envia as pessoas para um gateway na nuvem em vez de um login de primeira parte
* **Padrão**: não definido, portanto as pessoas escolhem um método de login

```json settings.json theme={null}
{
  "forceLoginMethod": "claudeai"
}
```

Cada caminho de login de primeira parte aplica a restrição, incluindo a [extensão VS Code](/docs/pt/vs-code), o Agent SDK, `claude setup-token` e `/install-github-app`, exceto a tela de login interativa do terminal, acessada por `/login` ou onboarding de primeira execução, que pré-seleciona o método sem aplicá-lo. Antes da v2.1.212, apenas logins de terminal o aplicavam. Veja [Restringir login à sua organização](/docs/pt/authentication#restrict-login-to-your-organization) para como cada caminho de login, credenciais de ambiente e provedores de terceiros são tratados.

Quando uma fonte gerenciada na máquina define `"gateway"`, Claude Code não usa um login restante, chave API ou credencial `apiKeyHelper`. Veja [A política do administrador requer um login de gateway na nuvem](/docs/pt/errors#administrator-policy-requires-a-cloud-gateway-sign-in) para a mensagem que cada um produz. Se você selecionar um provedor de nuvem através de `CLAUDE_CODE_USE_BEDROCK` ou uma variável de ambiente similar, a sessão não precisa do login do gateway. Antes da v2.1.261, Claude Code usava um login restante nessas máquinas.

<h3 id="forcelogingatewayurl">
  `forceLoginGatewayUrl`
</h3>

Defina a URL do gateway à qual a tela `/login` Cloud gateway se conecta, para que as pessoas alcancem seu [gateway na nuvem](/docs/pt/claude-apps-gateway) sem digitar seu endereço. A tela não tem campo de URL: com esta chave definida, ela mostra a URL do seu gateway e se conecta quando a pessoa pressiona Enter; sem ela, diz a elas para entrar em contato com seu administrador de TI.

Ou esta chave ou `forceLoginMethod: "gateway"` torna a máquina apenas gateway, portanto `/login` abre na tela Cloud gateway sem seletor de método de login. Veja [A política do administrador requer um login de gateway na nuvem](/docs/pt/errors#administrator-policy-requires-a-cloud-gateway-sign-in) para o que acontece com um login de primeira parte restante ou chave API. Defina ambas as chaves para que a tela se conecte em vez de mostrar um erro.

* **Escopo**: [`Gerenciado`](#scopes). Leia apenas de uma fonte na máquina: `managed-settings.json`, a plist do macOS ou registro HKLM do Windows, ou um auxiliar de política. Claude Code a ignora em configurações HKCU e gerenciadas por servidor.
* **Tipo**: string, uma URL completa incluindo o esquema
* **Padrão**: não definido, portanto a tela Cloud gateway mostra um erro dizendo às pessoas para entrar em contato com seu administrador de TI

```json managed-settings.json theme={null}
{
  "forceLoginGatewayUrl": "https://claude-gateway.example.com"
}
```

Se o valor não for uma URL válida, a tela de login a relata, e o resto do arquivo de configurações gerenciadas ainda se aplica. Veja [Defina a URL do gateway](/docs/pt/claude-apps-gateway#set-the-gateway-url).

<h3 id="forceloginorguuid">
  `forceLoginOrgUUID`
</h3>

De uma fonte gerenciada, exija que logins de contas claude.ai pertençam a uma organização Anthropic, fornecida como um único UUID, ou a qualquer uma de várias organizações, fornecidas como um array. De qualquer arquivo de configurações, Claude Code também usa um único UUID para pré-selecionar essa organização durante um login claude.ai ou Claude Console, e não pré-seleciona nada para um array. Se você definir a chave em qualquer arquivo de configurações, Claude Code também para de oferecer o [login Console sem chave](/docs/pt/authentication#sign-in-without-an-api-key) nas sessões às quais esse arquivo se aplica e cria uma chave API.

* **Escopo**: [`Qualquer arquivo`](#scopes). Apenas uma fonte gerenciada aplica a restrição; um único UUID em qualquer outro arquivo de configurações pré-seleciona a organização durante o login sem restringi-la.
* **Tipo**: string, um UUID, ou array de strings, vários UUIDs
* **Padrão**: não definido, portanto qualquer organização pode fazer login

Este exemplo aceita logins de qualquer uma de duas organizações sem pré-selecionar uma:

```json managed-settings.json theme={null}
{
  "forceLoginOrgUUID": ["xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx", "yyyyyyyy-yyyy-yyyy-yyyy-yyyyyyyyyyyy"]
}
```

Se uma fonte gerenciada define um array vazio, ou um valor que Claude Code não consegue analisar, Claude Code bloqueia cada login com uma mensagem de configuração incorreta.

Veja [Restringir login à sua organização](/docs/pt/authentication#restrict-login-to-your-organization) para como Claude Code trata logins Claude Console, os outros caminhos de login e credenciais de ambiente.

<h3 id="gatewayinternalnetworks">
  `gatewayInternalNetworks`
</h3>

Declare os blocos IPv4 públicos dos quais sua organização numera sua rede interna, para que `/login` aceite um [gateway na nuvem](/docs/pt/claude-apps-gateway) lá. Requer Claude Code v2.1.268 ou posterior.

Sem esta chave, `/login` se conecta a qualquer gateway em um endereço privado e nada mais. Com ela, `/login` também aceita um gateway dentro de um bloco listado, apenas sobre uma conexão direta. O endereço próprio da máquina nessa conexão também deve estar dentro do mesmo bloco.

* **Escopo**: [`Gerenciado`](#scopes). Leia apenas de uma fonte na máquina: `managed-settings.json`, a plist do macOS ou registro HKLM do Windows, ou um auxiliar de política. Claude Code a ignora em configurações HKCU e gerenciadas por servidor.
* **Tipo**: array de strings, no máximo quatro blocos IPv4 CIDR, cada um `/8` a `/32`, não se sobrepondo um ao outro, e nenhum se sobrepondo ao espaço privado.
* **Padrão**: não definido, portanto `/login` aceita apenas gateways em endereços privados

```json managed-settings.json theme={null}
{
  "gatewayInternalNetworks": ["203.0.113.0/24"]
}
```

Substitua o intervalo de documentação no exemplo pelo seu próprio bloco. Claude Code recusa os intervalos de documentação, os intervalos que clientes VPN e NAT64 usam localmente, e espaço reservado que nenhuma rede é numerada, como multicast.

Se uma entrada for inválida, ou o valor não for uma lista de strings, `/login` nomeia o problema e recusa cada novo login de gateway na nuvem na máquina até que você corrija o valor. Os logins existentes continuam funcionando. Veja [Permitir um gateway no espaço de endereço público que você possui](/docs/pt/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) para as regras completas e o que os desenvolvedores veem.

<h3 id="gcpauthrefresh">
  `gcpAuthRefresh`
</h3>

Execute seu próprio comando para atualizar as Credenciais Padrão de Aplicativo do Google Cloud quando Claude Code descobrir que expiraram ou não podem ser carregadas, para que as solicitações da [Plataforma de Agente do Google Cloud](/docs/pt/google-vertex-ai) continuem funcionando sem você se autenticar novamente manualmente.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: string, uma linha de comando do shell
* **Padrão**: não definido, portanto o erro de credencial de Claude Code diz a você para executar `gcloud auth application-default login` você mesmo

```json settings.json theme={null}
{
  "gcpAuthRefresh": "gcloud auth application-default login"
}
```

Veja [configuração avançada de credenciais](/docs/pt/google-vertex-ai#advanced-credential-configuration).

<h3 id="otelheadershelper">
  `otelHeadersHelper`
</h3>

Execute seu próprio comando para gerar os cabeçalhos que Claude Code envia com exportações OpenTelemetry, para backends cujos tokens giram. Claude Code o executa na inicialização e periodicamente depois disso, e espera um objeto JSON de valores de cabeçalho de string em stdout.

* **Escopo**: [`Qualquer arquivo`](#scopes)
* **Tipo**: string, um caminho executável ou uma linha de comando do shell
* **Padrão**: não definido, portanto Claude Code não adiciona cabeçalhos gerados por auxiliar

```json settings.json theme={null}
{
  "otelHeadersHelper": "/bin/generate_otel_headers.sh"
}
```

Defina o intervalo de atualização com [`CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`](/docs/pt/env-vars). Veja [Cabeçalhos dinâmicos](/docs/pt/monitoring-usage#dynamic-headers) para os requisitos do script e o que acontece quando o auxiliar falha.

<h2 id="updates-and-versioning">
  Atualizações e versionamento
</h2>

Escolha um canal de atualização e, para organizações, fixe as versões que as pessoas podem executar. Consulte [Atualizar Claude Code](/docs/pt/setup#update-claude-code).

<h3 id="autoupdateschannel">
  `autoUpdatesChannel`
</h3>

Escolha qual [canal de lançamento](/docs/pt/setup#configure-release-channel) as atualizações automáticas em segundo plano e `claude update` seguem. Defina `"stable"` para uma versão que é tipicamente cerca de uma semana antiga e pula lançamentos com regressões maiores, ou `"latest"` para o lançamento mais recente.

* **Escopo**: [`Any file`](#scopes). Defina-o em configurações gerenciadas para impor um canal em toda a sua organização.
* **Tipo**: string, um de:
  * `"latest"`: as atualizações seguem o lançamento mais recente
  * `"stable"`: as atualizações seguem uma versão que é tipicamente cerca de uma semana antiga e pula lançamentos com regressões maiores
* **Padrão**: não definido, então Claude Code segue `"latest"`

```json settings.json theme={null}
{
  "autoUpdatesChannel": "stable"
}
```

Claude Code escreve `"stable"` em suas configurações de usuário quando você o escolhe em **Auto-update channel** em `/config`, e remove a chave quando você volta para latest lá. `claude install stable` e `claude install latest` também salvam o canal que você nomeia. Mudar de `"latest"` para `"stable"` em `/config` pergunta se você deseja permitir um downgrade ou permanecer em sua versão atual; permanecer define [`minimumVersion`](#minimumversion). Instalações do Homebrew ignoram esta chave: o cask `claude-code` rastreia stable e `claude-code@latest` rastreia latest, e `claude update` defere para `brew upgrade`. Para desativar atualizações automáticas completamente, defina [`DISABLE_AUTOUPDATER`](/docs/pt/setup#disable-auto-updates) em `env`.

<h3 id="minimumversion">
  `minimumVersion`
</h3>

Impeça que atualizações automáticas em segundo plano e `claude update` instalem qualquer versão abaixo desta, para que mudar para o canal `"stable"` não o faça fazer downgrade de um build `"latest"` mais recente. Claude Code escreve esta chave para você quando você escolhe permanecer em sua versão atual ao mudar de canais em `/config`, e a limpa quando você volta para `"latest"`.

* **Escopo**: [`Any file`](#scopes). Defina-o em configurações gerenciadas para fixar um mínimo em toda a organização que as configurações de usuário e projeto não possam reduzir.
* **Tipo**: string, um número de versão como `"2.1.100"`; um valor que não é uma versão válida é ignorado
* **Padrão**: não definido, então as atualizações podem instalar qualquer versão que o canal oferece

Este exemplo segue o canal stable e recusa instalar qualquer versão abaixo de 2.1.100:

```json settings.json theme={null}
{
  "autoUpdatesChannel": "stable",
  "minimumVersion": "2.1.100"
}
```

Esta chave apenas restringe atualizações. Para fazer Claude Code recusar iniciar abaixo de uma versão, use [`requiredMinimumVersion`](#requiredminimumversion) em vez disso. Consulte [Fixar uma versão mínima](/docs/pt/setup#pin-a-minimum-version).

<h3 id="requiredmaximumversion">
  `requiredMaximumVersion`
</h3>

Defina a versão mais recente do Claude Code que sua organização permite iniciar. Quando a versão em execução é mais recente, Claude Code sai na inicialização e diz ao usuário para instalar uma versão aprovada através do método aprovado de sua organização; `claude install <version>` também pode funcionar. Requer Claude Code v2.1.163 ou posterior.

* **Escopo**: [`Managed`](#scopes). Claude Code não dá aviso quando ignora a chave em outro lugar.
* **Tipo**: string, um número de versão como `"2.1.150"`; um valor que não é uma versão válida é ignorado
* **Padrão**: não definido, então nenhum limite superior se aplica

```json managed-settings.json theme={null}
{
  "requiredMaximumVersion": "2.1.150"
}
```

Atualizações automáticas em segundo plano e `claude update` pulam versões acima do limite, então uma instalação dentro do intervalo permanece dentro dele. `claude update`, `claude install` e `claude doctor` continuam funcionando acima do limite para que os usuários possam se recuperar. Emparelhe-o com [`requiredMinimumVersion`](#requiredminimumversion) para impor um intervalo.

<h3 id="requiredminimumversion">
  `requiredMinimumVersion`
</h3>

Defina a versão mais antiga do Claude Code que sua organização permite iniciar. Quando a versão em execução é mais antiga, Claude Code sai na inicialização e diz ao usuário para atualizar através do método aprovado de sua organização. A verificação é executada apenas na inicialização, então uma sessão que já está em execução continua. Requer Claude Code v2.1.163 ou posterior.

* **Escopo**: [`Managed`](#scopes). Claude Code não dá aviso quando ignora a chave em outro lugar.
* **Tipo**: string, um número de versão como `"2.1.150"`; um valor que não é uma versão válida é ignorado
* **Padrão**: não definido, então nenhum piso se aplica

```json managed-settings.json theme={null}
{
  "requiredMinimumVersion": "2.1.150"
}
```

`claude update`, `claude install` e `claude doctor` continuam funcionando abaixo do piso para que os usuários possam se recuperar. Diferentemente de [`minimumVersion`](#minimumversion), que apenas previne downgrades, esta chave bloqueia a inicialização. Emparelhe-o com [`requiredMaximumVersion`](#requiredmaximumversion) para impor um intervalo.

<h2 id="tools">
  Ferramentas
</h2>

Desative ferramentas específicas no [aplicativo de desktop Claude Code](/docs/pt/desktop). A CLI do terminal ignora essas chaves. Para as próprias ferramentas, consulte [Ferramentas disponíveis para Claude](/docs/pt/tools-reference).

<h3 id="browserexternalpagetools">
  `browserExternalPageTools`
</h3>

Impeça que Claude use suas ferramentas para ler ou agir em páginas externas no [painel Navegador](/docs/pt/desktop#browse-external-sites) do aplicativo de desktop. As pessoas em sua organização ainda podem abrir sites externos por conta própria, e as visualizações de servidor de desenvolvimento local continuam funcionando com as ferramentas de Claude. O aplicativo de desktop lê essa chave; a CLI do terminal a ignora.

* **Escopo**: [`Managed`](#scopes)
* **Tipo**: string, `"disabled"`; o aplicativo de desktop também aceita `"disable"`, em ambos os casos
* **Padrão**: não definido, portanto as ferramentas de Claude funcionam em páginas externas

```json managed-settings.json theme={null}
{
  "browserExternalPageTools": "disabled"
}
```

Qualquer outro valor deixa as ferramentas de Claude ativadas, e uma string não vazia que não seja um dos dois valores aceitos registra um aviso. Para bloquear sites externos para pessoas e Claude igualmente, defina [`disableBrowserExternalNavigation`](#disablebrowserexternalnavigation) em vez disso. Consulte [Restringir navegação externa para sua organização](/docs/pt/desktop#restrict-external-browsing-for-your-organization).

<h3 id="disablebrowserexternalnavigation">
  `disableBrowserExternalNavigation`
</h3>

Desative a navegação externa no [painel Navegador](/docs/pt/desktop#browse-external-sites) do aplicativo de desktop para pessoas e Claude igualmente. As visualizações de servidor de desenvolvimento localhost continuam funcionando. O aplicativo de desktop lê essa chave; a CLI do terminal a ignora.

* **Escopo**: [`Managed`](#scopes)
* **Tipo**: Booleano; apenas o Booleano JSON `true` tem efeito
  * `true`: o aplicativo de desktop desativa a navegação externa no painel Navegador para pessoas e Claude igualmente; as visualizações de localhost continuam funcionando
  * `false`: a navegação externa permanece ativada
* **Padrão**: não definido, portanto a navegação externa está ativada

```json managed-settings.json theme={null}
{
  "disableBrowserExternalNavigation": true
}
```

O aplicativo de desktop ignora qualquer outro valor, e um valor que não seja um Booleano, como a string `"true"` ou `1`, também registra um aviso. Para deixar a navegação externa ativada mas manter as ferramentas de Claude desativadas em páginas externas, defina [`browserExternalPageTools`](#browserexternalpagetools) em vez disso. Consulte [Restringir navegação externa para sua organização](/docs/pt/desktop#restrict-external-browsing-for-your-organization).

<h3 id="disablemobilesimulatortools">
  `disableMobileSimulatorTools`
</h3>

Bloqueie as ferramentas de Claude para o [painel iOS Simulator](/docs/pt/desktop-ios-simulator#turn-off-simulator-access) do aplicativo de desktop. As pessoas mantêm o uso manual do painel; apenas o acesso de Claude é removido, e ninguém pode ativá-lo novamente de dentro do aplicativo. O aplicativo de desktop lê essa chave; a CLI do terminal a ignora.

* **Escopo**: [`Managed`](#scopes)
* **Tipo**: Booleano; apenas o Booleano JSON `true` tem efeito
  * `true`: o aplicativo de desktop bloqueia as ferramentas de Claude para o painel iOS Simulator
  * `false`: as ferramentas de simulador de Claude seguem a alternância de configurações de cada pessoa no aplicativo de desktop
* **Padrão**: não definido, portanto as ferramentas de simulador de Claude seguem a alternância de configurações de cada pessoa no aplicativo de desktop

```json managed-settings.json theme={null}
{
  "disableMobileSimulatorTools": true
}
```

O aplicativo de desktop ignora qualquer outro valor, e um valor que não seja um Booleano, como a string `"true"` ou `1`, também registra um aviso.

<span id="data-and-privacy" />

<h2 id="privacy-and-telemetry">
  Privacidade e telemetria
</h2>

Controle por quanto tempo o Claude Code mantém dados de sessão e o que envia. Os switches que desativam métricas de uso e relatórios de erro são variáveis de ambiente, não chaves de configuração: defina `DISABLE_TELEMETRY`, `DISABLE_ERROR_REPORTING` ou `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` na chave [`env`](#env) ou no shell. [Serviços de telemetria](/docs/pt/data-usage#telemetry-services) diz o que cada um para. Duas exceções desativam de um arquivo de configurações: [`feedbackDrafts`](#feedbackdrafts) abaixo para feedback redigido por Claude, e [`feedbackSurveyRate`](#feedbacksurveyrate) abaixo para a pesquisa de sessão.

<h3 id="cleanupperioddays">
  `cleanupPeriodDays`
</h3>

Defina quantos dias o Claude Code mantém [transcrições de sessão e outros dados de aplicação](/docs/pt/claude-directory#cleaned-up-automatically) antes de deletá-los. O Claude Code executa a exclusão como uma varredura em segundo plano após uma sessão iniciar, desde que possa determinar com segurança o período de retenção.

* **Escopo**: [`Any file`](#scopes)
* **Tipo**: número de dias, um número inteiro, mínimo `1`
* **Padrão**: `30`

```json settings.json theme={null}
{
  "cleanupPeriodDays": 20
}
```

Definir `0` falha na validação, então escolha um valor grande como `3650` para retenção prolongada. Para impedir que o Claude Code escreva transcrições, consulte [Armazenamento em texto simples](/docs/pt/claude-directory#plaintext-storage).

<h3 id="desktopsessioncleanupperioddays">
  `desktopSessionCleanupPeriodDays`
</h3>

Defina um limite de idade em dias para as transcrições de sessões que você iniciou ou continuou mais recentemente no Claude Desktop ou Cowork. Sem essa chave, o Claude Code [mantém essas transcrições em qualquer idade](/docs/pt/claude-directory#cleaned-up-automatically). O Claude Code deleta cada uma assim que fica mais antiga que tanto esse limite quanto [`cleanupPeriodDays`](#cleanupperioddays), então com `cleanupPeriodDays` em seu padrão de 30, um valor de `7` ainda as mantém por 30 dias. Quando configurações gerenciadas definem `cleanupPeriodDays`, esse período se aplica em vez disso e essa chave é ignorada. Requer Claude Code v2.1.248 ou posterior.

* **Escopo**: [`User or managed`](#scopes). O Claude Code também lê a chave de um arquivo que você passa com `--settings` e a ignora em configurações de projeto e locais.
* **Tipo**: número de dias, um número inteiro, mínimo `0`
* **Padrão**: `0`, que não define limite de idade

```json settings.json theme={null}
{
  "desktopSessionCleanupPeriodDays": 90
}
```

<h3 id="feedbackdrafts">
  `feedbackDrafts`
</h3>

Controle [feedback redigido por Claude](/docs/pt/tools-reference#sendfeedback-tool-behavior): se Claude pode enfileirar rascunhos de feedback para você revisar, e se o Claude Code mostra um card quando Claude enfileira um.

* **Escopo**: [`User or managed`](#scopes)
* **Tipo**: string, um de `"notify"`, `"quiet"` ou `"off"`
  * `"notify"`: O Claude Code mostra um card acima do prompt quando Claude enfileira um rascunho, até [três cards em uma sessão](/docs/pt/tools-reference#what-you-see-when-claude-drafts) por padrão
  * `"quiet"`: Claude redige sem um card. Você vê a contagem de rascunhos enfileirados no rodapé do prompt e os revisa em `/feedback`
  * `"off"`: O Claude Code remove a ferramenta SendFeedback, então Claude não pode enfileirar rascunhos
* **Padrão**: `"notify"`
* **Substituições por sessão**: [`CLAUDE_CODE_SEND_FEEDBACK`](/docs/pt/env-vars) definido como `0` desativa o recurso para uma sessão

```json settings.json theme={null}
{
  "feedbackDrafts": "quiet"
}
```

Aparece em `/config` como **Claude-drafted feedback**, que escreve essa chave em suas configurações de usuário. Você vê a linha `/config` apenas em sessões [onde Claude pode redigir feedback](/docs/pt/tools-reference#sessions-without-claude-drafted-feedback); definir `"off"` não a oculta, então você pode ativar o recurso novamente a partir da mesma linha. Um valor em configurações gerenciadas tem precedência sobre sua configuração de usuário, então quando um administrador define essa chave, a linha mostra o valor gerenciado e alterá-lo não tem efeito. O Claude Code ignora essa chave em configurações de projeto e locais.

<h3 id="feedbacksurveyrate">
  `feedbackSurveyRate`
</h3>

Defina a probabilidade de que a [pesquisa de qualidade de sessão](/docs/pt/data-usage#session-quality-surveys) apareça quando uma sessão for elegível para ela. Defina `0` para impedir que a pesquisa apareça.

* **Escopo**: [`Any file`](#scopes)
* **Tipo**: número entre `0` e `1`
* **Padrão**: não definido, então o Claude Code usa a taxa que a Anthropic define remotamente, ou sua taxa integrada de `0.005` no Amazon Bedrock, na Agent Platform do Google Cloud e no Microsoft Foundry, que não recebem configuração remota
* **Substituições por sessão**: [`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY`](/docs/pt/env-vars) definido como `1` desativa a pesquisa para uma sessão qualquer que seja a taxa que essa chave define

```json settings.json theme={null}
{
  "feedbackSurveyRate": 0.05
}
```

A mesma taxa se aplica à pesquisa na extensão VS Code.

<h3 id="skipwebfetchpreflight">
  `skipWebFetchPreflight`
</h3>

Pule a [verificação de segurança de domínio WebFetch](/docs/pt/data-usage#webfetch-domain-safety-check), que envia cada nome de host solicitado para `api.anthropic.com` antes de buscar. Defina `true` em ambientes que bloqueiam tráfego para Anthropic, como Amazon Bedrock, Agent Platform do Google Cloud ou implantações do Microsoft Foundry com saída restritiva.

* **Escopo**: [`Any file`](#scopes)
* **Tipo**: Boolean
  * `true`: O Claude Code pula a verificação de segurança de domínio WebFetch
  * `false`: a verificação é executada antes da primeira busca para cada nome de host em uma sessão, e novamente para um nome de host cuja verificação anterior foi bloqueada ou falhou
* **Padrão**: não definido, então a verificação é executada antes da primeira busca para cada nome de host em uma sessão

```json settings.json theme={null}
{
  "skipWebFetchPreflight": true
}
```

Com a verificação ignorada, WebFetch tenta qualquer URL sem consultar a lista de bloqueio, então emparelhe com [regras de permissão `WebFetch`](/docs/pt/permissions#webfetch) se precisar restringir quais domínios Claude pode alcançar.

<span id="managed-policy" />

<h2 id="enterprise-and-managed-settings">
  Configurações empresariais e gerenciadas
</h2>

Chaves que uma organização usa para calcular, atualizar e combinar configurações gerenciadas. Consulte [Configurar configurações gerenciadas](/docs/pt/admin-setup).

<h3 id="disablesideloadflags">
  `disableSideloadFlags`
</h3>

Rejeite os sinalizadores CLI `--plugin-dir`, `--plugin-url`, `--agents` e `--mcp-config` na inicialização, que os usuários poderiam passar para contornar [`strictKnownMarketplaces`](#strictknownmarketplaces) em uma única execução. Claude Code sai com um erro nomeando os sinalizadores rejeitados e aplica a mesma verificação a superfícies que iniciam o CLI com esses sinalizadores internamente, atualmente [Cowork](/docs/pt/desktop) sessões locais no aplicativo desktop. Em [sessões na nuvem](/docs/pt/claude-code-on-the-web), Claude Code descarta os servidores MCP que o servidor entregou através de `--mcp-config`, exceto entradas `type: "sdk"` em processo, e inicia a sessão. Requer Claude Code v2.1.193 ou posterior.

* **Escopo**: [`Managed`](#scopes)
* **Tipo**: Booleano
  * `true`: Claude Code rejeita `--plugin-dir`, `--plugin-url`, `--agents` e `--mcp-config` na inicialização e sai com um erro nomeando-os, exceto que em sessões na nuvem ele descarta os servidores MCP que o servidor entregou através de `--mcp-config`, exceto entradas `type: "sdk"` em processo, e inicia a sessão
  * `false`: Claude Code aceita esses sinalizadores
* **Padrão**: `false`

```json managed-settings.json theme={null}
{
  "disableSideloadFlags": true
}
```

Claude Code ainda aceita um `--mcp-config` cujos servidores são todas entradas `type: "sdk"` em processo, então o Agent SDK e a extensão VS Code continuam funcionando. Os usuários ainda podem adicionar servidores com `claude mcp add` ou um arquivo `.mcp.json`; para controle por servidor, defina [`allowedMcpServers`](/docs/pt/managed-mcp) também. Requer Claude Code v2.1.193 ou posterior.

A mesma verificação cobre pastas de plugins nomeadas na variável de ambiente [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/pt/env-vars#variables), que requer Claude Code v2.1.280 ou posterior. Quando a variável nomeia uma pasta, Claude Code sai com o mesmo erro, e o erro diz para desconfigurar a variável.

Em sessões na nuvem, Claude Code também ignora atualizações MCP entregues pelo servidor no meio da sessão, o caminho por trás da configuração de sessão na nuvem e SDK `setMcpServers()` que alcançam essas sessões. Entradas `type: "sdk"` em processo permanecem isentas lá também. Antes da v2.1.239, um `--mcp-config` entregue pelo servidor bloqueava uma sessão na nuvem de iniciar.

<h3 id="forceremotesettingsrefresh">
  `forceRemoteSettingsRefresh`
</h3>

Bloqueie a inicialização do CLI até que Claude Code tenha buscado recentemente [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings). Se a busca falhar, Claude Code sai em vez de continuar com configurações em cache ou nenhuma. Defina-o quando seu ambiente não puder aceitar nem mesmo uma breve janela em que uma sessão seja executada sem sua política gerenciada.

Quando a chave não está definida, Claude Code não bloqueia a inicialização na busca, embora quando o desenvolvedor se conecta na inicialização ele aguarde até cinco segundos pela busca. Uma sessão de gateway na nuvem sempre aguarda e sai se o gateway não puder ser alcançado.

* **Escopo**: [`Managed`](#scopes). Claude Code honra um `true` de qualquer fonte gerenciada controlada por administrador, mesmo uma que não seja a fonte de prioridade mais alta.
* **Tipo**: Booleano
  * `true`: Claude Code bloqueia a inicialização até ter buscado recentemente configurações gerenciadas pelo servidor e sai se a busca falhar
  * `false`: Claude Code não bloqueia a inicialização na busca, embora em uma inicialização de conexão ele aguarde até cinco segundos pela busca
* **Padrão**: `false`

```json managed-settings.json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

Defina-o em um perfil MDM ou no arquivo de configurações gerenciadas para impor inicialização com falha fechada antes da primeira carga útil do servidor chegar. Claude Code aplica a verificação apenas em sessões que buscam configurações gerenciadas pelo servidor, então uma sessão que [não as busca](/docs/pt/server-managed-settings#platform-availability) inicia sem aguardar. Os subcomandos `claude auth` estão isentos, então os usuários podem se autenticar novamente quando credenciais expiradas são o motivo da falha da busca. Consulte [Impor inicialização com falha fechada](/docs/pt/server-managed-settings#enforce-fail-closed-startup).

<h3 id="managedsourcesbehavior">
  `managedSourcesBehavior`
</h3>

Escolha se Claude Code aplica apenas a [fonte gerenciada](/docs/pt/managed-settings#how-claude-code-combines-managed-sources) de prioridade mais alta que sua organização entrega, ou combina todas as fontes de administrador que entrega. Por padrão, Claude Code pega a fonte de prioridade mais alta que carrega uma [chave de política](/docs/pt/managed-settings#how-claude-code-combines-managed-sources) e ignora o resto. Uma chave de política é qualquer chave de configurações diferente desta e `wslInheritsWindowsSettings`. Portanto, uma vez que configurações gerenciadas pelo servidor ou uma política MDM entreguem uma chave de política, um arquivo `managed-settings.json` contribui apenas com as [chaves que Claude Code lê de todas as fontes de administrador](/docs/pt/managed-settings#keys-read-from-every-admin-source). Com `"merge"`, todas as fontes de administrador que você entrega contribuem suas chaves para uma política combinada. Requer Claude Code v2.1.242 ou posterior.

Defina `"merge"` apenas onde todas as fontes [classificadas](/docs/pt/managed-settings#how-claude-code-combines-managed-sources) abaixo da sua mais alta estão sob controle de um administrador, porque Claude Code então adiciona entradas de uma fonte inferior, como regras `permissions.allow`, à política.

* **Escopo**: [`Managed`](#scopes). Claude Code lê essa chave da fonte de prioridade mais alta que carrega esta chave ou uma chave de política, e ignora essa chave em todas as fontes classificadas mais baixo, então uma fonte inferior não pode optar por se combinar com a fonte acima dela. Nem o registro HKCU do Windows nem [configurações pai de um host de incorporação](/docs/pt/managed-settings#let-an-embedding-host-add-policy) participam da mesclagem.
* **Tipo**: string, um de:
  * `"first-wins"`: a fonte de prioridade mais alta que carrega uma chave de política fornece a política, e fontes inferiores contribuem apenas com as [chaves que Claude Code lê de todas as fontes de administrador](/docs/pt/managed-settings#keys-read-from-every-admin-source)
  * `"merge"`: todas as fontes de administrador que você entrega contribuem suas chaves, combinadas pelas regras abaixo
* **Padrão**: `"first-wins"`

Entregue a chave na fonte de prioridade mais alta que você implanta. Uma máquina que nunca recebe configurações gerenciadas pelo servidor precisa da chave em seu perfil MDM também, porque Claude Code lê a chave da fonte de prioridade mais alta que a carrega ou uma chave de política. Um arquivo `managed-settings.json` é a fonte de administrador classificada mais baixa, então `"merge"` definido lá não tem fonte abaixo dela para combinar. Em configurações gerenciadas pelo servidor, a chave se parece com isto:

```json theme={null}
{
  "managedSourcesBehavior": "merge"
}
```

Sob `"merge"`, Claude Code combina cada chave por seu tipo. Esta tabela fornece a regra para cada tipo. As linhas de lista de restrição, valores tomados inteiros e apenas fonte mais alta nomeiam todas as chaves que cobrem, e as outras linhas fornecem exemplos:

| Tipo de chave                              | Como Claude Code a combina                                                                                                                                                              | Chaves                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :----------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Listas                                     | Combina entradas de todas as fontes                                                                                                                                                     | [`permissions.allow`](#permissions-allow), [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) e outras chaves de lista                                                                                                                                                                                                                                                                                                                                                                                         |
| Bloqueios                                  | Aplica o valor mais rigoroso que qualquer fonte define. Quando nenhuma fonte define um valor rigoroso, aplica um valor mais flexível apenas da fonte mais alta                          | [`allowManagedPermissionRulesOnly`](#allowmanagedpermissionrulesonly), [`permissions.disableBypassPermissionsMode`](#permissions-disablebypasspermissionsmode) e outros bloqueios booleanos ou enum                                                                                                                                                                                                                                                                                                                             |
| Listas de restrição                        | Pega a lista inteira da fonte mais alta que a define, sem adicionar entradas de fontes inferiores. Quando a fonte mais alta não define uma, pega inteira da próxima fonte abaixo        | [`availableModels`](#availablemodels), [`allowedMcpServers`](#allowedmcpservers), [`strictKnownMarketplaces`](#strictknownmarketplaces), [`allowedChannelPlugins`](#allowedchannelplugins) e a cadeia [`fallbackModel`](#fallbackmodel)                                                                                                                                                                                                                                                                                         |
| Valores tomados inteiros                   | Pega o valor inteiro da fonte mais alta que o define, sem combinar entradas ou campos de fontes inferiores. Quando a fonte mais alta não o define, pega inteiro da próxima fonte abaixo | [`sandbox.credentials.awsPairs`](#sandbox-credentials-awspairs), [`sandbox.ripgrep`](#sandbox-ripgrep)                                                                                                                                                                                                                                                                                                                                                                                                                          |
| Servidores MCP fornecidos                  | Combina os nomes de servidor de todas as fontes. Quando duas fontes definem o mesmo nome, aplica a entrada inteira da fonte mais alta                                                   | [`managedMcpServers`](#managedmcpservers)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Lê apenas da fonte de prioridade mais alta | Lê a chave apenas da fonte de prioridade mais alta que carrega uma chave de política, então o valor de uma fonte inferior é ignorado mesmo quando a fonte mais alta não define nenhum   | [`apiKeyHelper`](#apikeyhelper), [`awsAuthRefresh`](#awsauthrefresh), [`awsCredentialExport`](#awscredentialexport), [`gcpAuthRefresh`](#gcpauthrefresh), [`otelHeadersHelper`](#otelheadershelper), `proxyAuthHelper`, [`forceLoginOrgUUID`](#forceloginorguuid), os valores `"claudeai"` e `"console"` de [`forceLoginMethod`](#forceloginmethod), [`parentSettingsBehavior`](#parentsettingsbehavior), [`modelPicker`](#modelpicker), [`policyHelper`](#policyhelper), [`permissions.defaultMode`](#permissions-defaultmode) |
| `env`                                      | [Mescla por variável em fontes de administrador](/docs/pt/managed-settings#keys-read-from-every-admin-source), sob ambos `"first-wins"` e `"merge"`                                          | [`env`](#env)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Todas as outras chaves                     | Pega o valor da fonte mais alta que o define                                                                                                                                            | [`cleanupPeriodDays`](#cleanupperioddays), [`model`](#model)                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |

Pegar `sandbox.credentials.awsPairs` e `sandbox.ripgrep` inteiros requer Claude Code v2.1.257 ou posterior.

Algumas chaves adicionam uma condição que a tabela não mostra:

* **[`policyHelper`](#policyhelper)**: Claude Code a honra apenas quando a fonte mais alta que carrega uma chave de política é uma política MDM ou um arquivo de configurações gerenciadas, então sob configurações gerenciadas pelo servidor ela não se aplica.
* **[`modelOverrides`](#modeloverrides)**: emparelha com `availableModels`. Claude Code pega `modelOverrides` da fonte mais alta que a define, a menos que uma fonte mais alta defina `availableModels` sem `modelOverrides`. Nesse caso, ele ignora `modelOverrides` de todas as fontes.
* **[`forceLoginGatewayUrl`](#forcelogingatewayurl), [`gatewayInternalNetworks`](#gatewayinternalnetworks) e o valor `"gateway"` de [`forceLoginMethod`](#forceloginmethod)**: Claude Code nunca lê nenhum deles de configurações gerenciadas pelo servidor, então um valor lá não se aplica nem oculta um definido em uma política MDM ou arquivo de configurações gerenciadas. Entre as fontes de administrador na máquina, apenas a fonte classificada mais alta que carrega uma chave de política as fornece, independentemente de configurações gerenciadas pelo servidor também estarem presentes.

Para confirmar quais fontes se combinaram em uma máquina, execute `/status` e [leia a linha `Setting sources`](/docs/pt/managed-settings#read-the-source-in-/status).

<h3 id="parentsettingsbehavior">
  `parentSettingsBehavior`
</h3>

Escolha se Claude Code aplica configurações gerenciadas fornecidas por um processo host de incorporação, como o Agent SDK ou uma extensão IDE, quando um nível gerenciado implantado por administrador também está presente. Com `"first-wins"`, Claude Code descarta as configurações fornecidas pelo host; com `"merge"`, ele as aplica sob o nível de administrador através de um filtro apenas restritivo. Defina `"merge"` quando um host precisa passar suas próprias restrições para as sessões que inicia, por exemplo Claude Desktop entregando uma lista de permissões de saída de um gateway.

* **Escopo**: [`Managed`](#scopes). Claude Code a lê da fonte gerenciada controlada por administrador de prioridade mais alta.
* **Tipo**: string, um de:
  * `"first-wins"`: Claude Code descarta as configurações fornecidas pelo host quando um nível gerenciado implantado por administrador está presente
  * `"merge"`: Claude Code aplica as configurações fornecidas pelo host sob o nível de administrador através de um filtro apenas restritivo
* **Padrão**: `"first-wins"`

```json managed-settings.json theme={null}
{
  "parentSettingsBehavior": "merge"
}
```

Esta chave não tem efeito quando nenhum nível gerenciado implantado por administrador existe: as configurações do host então se aplicam como o único nível gerenciado, ainda filtrado para valores restritivos. Para os limites do filtro e como as fontes gerenciadas interagem, consulte [Configurações pai de hosts de incorporação](/docs/pt/managed-settings#parent-settings-from-embedding-hosts) e [Restringir configurações pai](/docs/pt/claude-apps-gateway#restrict-parent-settings).

<span id="compute-managed-settings-with-a-policy-helper" />

<h3 id="policyhelper">
  `policyHelper`
</h3>

Execute um executável que você implanta que calcula configurações gerenciadas na inicialização, para que você possa derivar política da postura do dispositivo, identidade ou um serviço remoto em vez de um arquivo estático. Claude Code executa o auxiliar antes de aceitar o primeiro prompt e trata as configurações que emite como as configurações gerenciadas para a sessão.

* **Escopo**: [`Managed`](#scopes). Leia do plist macOS, do registro HKLM do Windows ou do arquivo de configurações gerenciadas. Claude Code lê a chave da fonte gerenciada de prioridade mais alta que carrega uma [chave de política](/docs/pt/managed-settings#how-claude-code-combines-managed-sources) e executa o auxiliar apenas quando essa fonte é uma daquelas três; ele ignora a chave em configurações gerenciadas pelo servidor, no registro HKCU e em configurações pai fornecidas pelo host.
* **Tipo**: objeto com `path`, `timeoutMs` e `refreshIntervalMs`
* **Padrão**: não definido, então nenhum auxiliar é executado

Quando configurações gerenciadas pelo servidor entregam a política na inicialização, elas têm precedência sobre a fonte do auxiliar e o auxiliar não é executado.

Se uma busca de configurações posterior relatar que as configurações gerenciadas pelo servidor foram removidas, Claude Code executa o auxiliar nesse ponto em vez de aguardar a próxima inicialização. Sua saída governa o resto da sessão, e uma execução que falha encerra a sessão com a mesma mensagem que uma [execução de inicialização falhada](#helper-failures).

Este exemplo executa o auxiliar com um tempo limite de 5 segundos e o re-executa a cada cinco minutos:

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
  Escreva a saída do auxiliar
</h4>

Claude Code executa o auxiliar sem argumentos, define `CLAUDE_CODE_VERSION` em seu ambiente e lê um envelope JSON de stdout, limitado a 1 MiB.

Coloque as configurações sob uma chave `managedSettings`. Um objeto de configurações simples sem chave `managedSettings` analisa com `managedSettings` indefinido e não aplica nada, e Claude Code não relata nenhum erro:

```json theme={null}
{
  "managedSettings": {
    "permissions": { "deny": ["Read(//etc/secrets/**)"] }
  }
}
```

Quando o auxiliar emite `managedSettings`, esse objeto se torna a única fonte de configurações gerenciadas para a execução: Claude Code ignora as fontes MDM, arquivo e HKCU, lê as [chaves entre fontes](/docs/pt/managed-settings#keys-read-from-every-admin-source) apenas da saída do auxiliar e nunca mescla [configurações pai](/docs/pt/managed-settings#parent-settings-from-embedding-hosts).

A verificação de inicialização `forceRemoteSettingsRefresh` é executada antes do auxiliar e lê qualquer fonte de administrador. Um auxiliar que sai com `0` com um envelope que omite `managedSettings` não contribui com nenhuma configuração gerenciada, e as outras fontes se aplicam como usual.

<h4 id="helper-failures">
  Falhas do auxiliar
</h4>

Uma execução do auxiliar falha quando:

* `path` quebra as regras em [`policyHelper.path`](#policyhelper-path).
* Nenhum arquivo regular está em `path`. Claude Code verifica o arquivo antes de iniciar o auxiliar, dentro do mesmo orçamento `timeoutMs`, então uma montagem de rede sem resposta pode causar a falha da execução.
* O auxiliar sai com não-zero, ainda está em execução quando `timeoutMs` decorre ou não inicia, por exemplo porque não é executável.
* O auxiliar escreve mais de 1 MiB para stdout ou stderr.
* stdout não é um único objeto JSON, ou seu `managedSettings` tem uma [violação de esquema que Claude Code não pode reparar](/docs/pt/managed-settings#find-entries-claude-code-dropped).

Quando a execução de inicialização falha, Claude Code imprime o motivo e se recusa a iniciar. Após uma saída com não-zero, o motivo inclui stderr do auxiliar, ou seu stdout quando stderr está vazio. Após um tempo limite, o motivo nomeia o limite `timeoutMs` e não inclui nenhuma saída do auxiliar. A recusa cobre sessões interativas, `claude -p`, sessões do Agent SDK, [sessões em segundo plano](/docs/pt/agent-view) e a maioria dos subcomandos.

A recusa é deliberada, então um auxiliar que precisa de resiliência de interrupção deve servir de seu próprio cache e sair com `0`.

Quando uma atualização em segundo plano falha, Claude Code mantém a última política bem-sucedida em vigor, e `/status` mostra a atualização falhada com seu motivo até que uma atualização tenha sucesso. Cada atualização é executada sob o mesmo `timeoutMs` e regras de falha que a execução de inicialização.

Com `--debug`, Claude Code escreve stderr do auxiliar de cada execução para o [log de depuração](/docs/pt/debug-your-config).

Claude Code relata um valor `policyHelper` inválido como uma [entrada descartada](/docs/pt/managed-settings#find-entries-claude-code-dropped) e inicia a sessão nas configurações gerenciadas restantes sem executar um auxiliar. Valores inválidos incluem uma string de caminho simples e um `timeoutMs` abaixo de [seu mínimo](#policyhelper-timeoutms).

Para desativar um auxiliar, remova a chave da fonte que a define.

<h3 id="policyhelper-path">
  `policyHelper.path`
</h3>

Nomeie o executável do auxiliar que Claude Code executa. Para o que acontece quando o caminho quebra as regras abaixo, consulte [Falhas do auxiliar](#helper-failures).

* **Escopo**: [`Managed`](#scopes). Leia do plist macOS, do registro HKLM do Windows ou do arquivo de configurações gerenciadas, onde [`policyHelper`](#policyhelper) é lido.
* **Tipo**: string, um caminho absoluto em forma normalizada, sem segmentos `.` ou `..`; no Windows, um caminho de letra de unidade ou UNC que termina em `.exe`
* **Padrão**: nenhum; obrigatório quando `policyHelper` está definido

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

Defina quanto tempo Claude Code aguarda o auxiliar antes de tratar a execução como falhada. Uma execução com tempo limite falha da mesma forma que uma saída com não-zero, então na inicialização Claude Code se recusa a iniciar.

* **Escopo**: [`Managed`](#scopes). Leia do plist macOS, do registro HKLM do Windows ou do arquivo de configurações gerenciadas, onde [`policyHelper`](#policyhelper) é lido.
* **Tipo**: inteiro, milissegundos, mínimo `1000`
* **Padrão**: `10000`

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

Faça Claude Code re-executar o auxiliar em segundo plano em um intervalo para que mudanças de política alcancem uma sessão em execução. Quando uma atualização tem sucesso, sua saída substitui as configurações gerenciadas anteriores sem uma reinicialização; quando uma atualização falha, Claude Code mantém a política que já tem.

* **Escopo**: [`Managed`](#scopes). Leia do plist macOS, do registro HKLM do Windows ou do arquivo de configurações gerenciadas, onde [`policyHelper`](#policyhelper) é lido.
* **Tipo**: inteiro, milissegundos: `0` para desabilitar atualização, caso contrário pelo menos `60000`
* **Padrão**: não definido, então Claude Code executa o auxiliar uma vez na inicialização

Este exemplo re-executa o auxiliar a cada cinco minutos:

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

Faça Claude Code no WSL ler configurações gerenciadas da cadeia de política do Windows, com HKLM e o arquivo de configurações gerenciadas do Windows tendo prioridade sobre `/etc/claude-code` e HKCU abaixo. Enquanto a cadeia está ativada, Claude Code lê `/etc/claude-code` apenas quando nenhum arquivo de configurações gerenciadas ou drop-in sob `C:\Program Files\ClaudeCode\` entrega uma [chave de política](/docs/pt/managed-settings#how-claude-code-combines-managed-sources). Defina-o para estender a política que você já implanta no Windows para sessões WSL na mesma máquina, para que sigam as mesmas regras que sessões de host. Claude Code a honra apenas quando definida na chave de registro HKLM ou em um arquivo de configurações gerenciadas ou drop-in sob `C:\Program Files\ClaudeCode\`, ambos exigindo administrador do Windows para escrever.

* **Escopo**: [`Managed`](#scopes). Em uma fonte do Windows controlada por administrador.
* **Tipo**: Booleano
  * `true`: Claude Code no WSL lê configurações gerenciadas da cadeia de política do Windows e lê `/etc/claude-code` apenas quando nenhum arquivo de configurações gerenciadas ou drop-in sob `C:\Program Files\ClaudeCode\` entrega uma [chave de política](/docs/pt/managed-settings#how-claude-code-combines-managed-sources)
  * `false`: WSL lê apenas `/etc/claude-code`
* **Padrão**: `false`, então WSL lê apenas `/etc/claude-code`

```json managed-settings.json theme={null}
{
  "wslInheritsWindowsSettings": true
}
```

Uma vez que uma fonte de administrador ativa a cadeia, a política HKCU se une a ela no WSL apenas quando HKCU também define a chave como `true`. Essa cópia não ativa a cadeia por si só. Uma fonte do Windows que contém apenas essa chave não conta como uma fonte de política, então uma fonte de prioridade mais baixa ainda fornece a política. Esta chave não tem efeito no Windows nativo.

<h2 id="global-config-settings">
  Configurações globais
</h2>

Salve essas chaves em `~/.claude.json`, não em um arquivo de configurações. Claude Code as ignora em qualquer outro lugar. Claude Code e `/config` escrevem a maioria delas para você, e você também pode editá-las manualmente.

<h3 id="autoconnectide">
  `autoConnectIde`
</h3>

Conecte a um IDE em execução automaticamente quando você inicia Claude Code a partir de um terminal externo. Aparece em `/config` como **Auto-conectar ao IDE (terminal externo)** quando você executa Claude Code fora de um terminal VS Code ou JetBrains.

* **Escopo**: [`Global config`](#scopes)
* **Tipo**: Booleano
  * `true`: Claude Code se conecta a um IDE em execução automaticamente quando você o inicia a partir de um terminal externo
  * `false`: Claude Code não se conecta automaticamente a partir de um terminal externo; dentro de um terminal VS Code ou JetBrains, ou com `--ide`, ele ainda se conecta
* **Padrão**: `false`
* **Substituições por sessão**: [`CLAUDE_CODE_AUTO_CONNECT_IDE`](/docs/pt/env-vars) tem precedência sobre essa chave para uma sessão, em qualquer direção

```json ~/.claude.json theme={null}
{
  "autoConnectIde": true
}
```

Claude Code ignora essa chave em `settings.json`.

<h3 id="autoinstallideextension">
  `autoInstallIdeExtension`
</h3>

Instale a extensão Claude Code IDE automaticamente quando você executa Claude Code a partir de um terminal VS Code. Aparece em `/config` como **Auto-instalar extensão IDE** quando você executa Claude Code dentro de um terminal VS Code ou JetBrains.

* **Escopo**: [`Global config`](#scopes)
* **Tipo**: Booleano
  * `true`: Claude Code instala a extensão IDE automaticamente quando você a executa a partir de um terminal VS Code
  * `false`: Claude Code não instala a extensão automaticamente
* **Padrão**: `true`
* **Substituições por sessão**: [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](/docs/pt/env-vars) definido como `1` pula a instalação para uma sessão mesmo quando essa chave é `true`

```json ~/.claude.json theme={null}
{
  "autoInstallIdeExtension": false
}
```

Claude Code ignora essa chave em `settings.json`.

<h3 id="copyonselect">
  `copyOnSelect`
</h3>

Copie texto para sua área de transferência automaticamente quando você terminar de selecioná-lo com o mouse em [renderização em tela cheia](/docs/pt/fullscreen#use-the-mouse) ou [visualização de agente](/docs/pt/agent-view). Aparece em `/config` como **Copiar ao selecionar** enquanto a renderização em tela cheia está ativada.

* **Escopo**: [`Global config`](#scopes)
* **Tipo**: Booleano
  * `true`: Claude Code copia texto para sua área de transferência quando você termina de selecioná-lo
  * `false`: selecionar texto deixa sua área de transferência inalterada, e você [copia a seleção com um atalho de teclado](/docs/pt/fullscreen#use-the-mouse) em vez disso
* **Padrão**: `true`

```json ~/.claude.json theme={null}
{
  "copyOnSelect": false
}
```

Claude Code ignora essa chave em `settings.json`.

<h3 id="difftool">
  `diffTool`
</h3>

Escolha onde Claude Code mostra o diff de uma mudança `Edit` ou `Write` que ele propõe quando um IDE [VS Code](/docs/pt/vs-code) ou [JetBrains](/docs/pt/jetbrains#features) está conectado: `"auto"` o abre no visualizador de diff do IDE, `"terminal"` o mantém no terminal. Aparece em `/config` como **Diff tool** apenas enquanto Claude Code está conectado a um IDE VS Code ou JetBrains.

* **Escopo**: [`Global config`](#scopes)
* **Tipo**: string, um de:
  * `"auto"`: Claude Code abre o diff no visualizador de diff do IDE quando um IDE VS Code ou JetBrains está conectado
  * `"terminal"`: Claude Code mantém o diff no terminal
* **Padrão**: `"auto"`

```json ~/.claude.json theme={null}
{
  "diffTool": "terminal"
}
```

Claude Code ignora essa chave em `settings.json`.

<h3 id="externaleditorcontext">
  `externalEditorContext`
</h3>

Quando você pressiona `Ctrl+G`, Claude Code abre o prompt que você está digitando em seu [editor externo](/docs/pt/interactive-mode#general-controls). Com essa chave ativada, o buffer do editor começa com a resposta anterior de Claude como linhas de comentário `#`, para que você possa lê-la enquanto escreve, e Claude Code remove essas linhas quando você salva. Aparece em `/config` como **Mostrar última resposta no editor externo**.

* **Escopo**: [`Global config`](#scopes)
* **Tipo**: Booleano
  * `true`: o buffer do editor começa com a resposta anterior de Claude como linhas de comentário `#`, que Claude Code remove quando você salva
  * `false`: o buffer do editor abre apenas com seu prompt
* **Padrão**: `false`

```json ~/.claude.json theme={null}
{
  "externalEditorContext": true
}
```

Com ela ativada, o buffer que Claude Code abre se parece com isto, e apenas o texto abaixo da linha marcadora é enviado como seu prompt:

```text theme={null}
# ─── Última resposta de Claude (para referência; removida ao salvar) ───
# Adicionei o loop de repetição a fetchUser em src/api.ts e um teste
# para o caso de timeout. Quer que eu conecte a mesma repetição a
# fetchOrders?
# ─── Escreva sua resposta abaixo desta linha ──────────────────────────

Sim, e limite a três tentativas.
```

Claude Code mantém as últimas 50 linhas da resposta e marca o corte com `# … (saída anterior truncada)`.

Claude Code ignora essa chave em `settings.json`.

<h3 id="permissionexplainerenabled">
  `permissionExplainerEnabled`
</h3>

<Warning>
  Removido na v2.1.257, junto com a explicação do comando `Ctrl+E` nos prompts de permissão Bash e PowerShell. Defini-lo não tem efeito nas versões atuais.
</Warning>

Até v2.1.256, você poderia pressionar `Ctrl+E` em um prompt de permissão Bash ou PowerShell para ver uma explicação gerada pelo modelo do comando, e definir essa chave como `false` para desativar esse atalho.

* **Escopo**: [`Global config`](#scopes). Na v2.1.256 e anteriores.
* **Tipo**: Booleano
* **Padrão**: `true`

<h3 id="teammatedefaultmodel">
  `teammateDefaultModel`
</h3>

<Warning>
  Removido na v2.1.234, junto com sua linha `/config` **Modelo padrão de colega de equipe**. Defini-lo não tem efeito nas versões atuais.
</Warning>

Até v2.1.233, você definia essa chave para o modelo de [equipe de agente](/docs/pt/agent-teams#specify-teammates-and-models) colegas de equipe que seu prompt não nomeou um modelo para: um alias como `"sonnet"`, ou `null` para seguir o modelo do líder. Para o modelo que Claude Code escolhe para esses colegas de equipe agora, veja [especificar colegas de equipe e modelos](/docs/pt/agent-teams#specify-teammates-and-models).

* **Escopo**: [`Global config`](#scopes). Na v2.1.233 e anteriores.
* **Tipo**: string, um alias de modelo ou ID de modelo completo, ou `null`
* **Padrão**: não definido

<h2 id="see-also">
  Veja também
</h2>

* [Configurar permissões](/docs/pt/permissions): sintaxe de regras, modos de permissão e confiança do workspace
* [Variáveis de ambiente](/docs/pt/env-vars): todas as variáveis `CLAUDE_*`, `ANTHROPIC_*` e de provedor que Claude Code lê
* [Ferramentas disponíveis para Claude](/docs/pt/tools-reference): as ferramentas integradas e quais precisam de aprovação
* [Arquivos de configuração de exemplo](/docs/pt/settings-example): um arquivo pessoal, um arquivo de equipe e um arquivo gerenciado de uma organização
* [Configurar configurações gerenciadas](/docs/pt/admin-setup): como as organizações decidem o que impor
* [Implantar configurações gerenciadas](/docs/pt/managed-settings): mecanismos de entrega, precedência dentro da camada gerenciada e entradas inválidas em configurações gerenciadas
* [Depurar sua configuração](/docs/pt/debug-your-config): `claude doctor` e o diálogo Erro de Configurações
