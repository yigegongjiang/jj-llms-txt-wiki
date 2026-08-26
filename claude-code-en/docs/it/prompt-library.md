> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Libreria di prompt

> Copia e incolla prompt per Claude Code, etichettati per attività e ruolo.

export const PromptLibrary = ({text = {}, labels = {}, tagLabels = {}, phaseLabels = {}, sourceLabels = {}, catLabels = {}}) => {
  const RAW = useMemo(() => [{
    id: 'get-oriented-in-a',
    sdlc: 'discover',
    cat: 'Onboard',
    startN: 1,
    roles: [],
    prompt: 'give me an overview of this codebase: architecture, key directories, and how the pieces connect',
    nextHref: '/en/memory',
    src: 'workflows'
  }, {
    id: 'explain-unfamiliar-code',
    sdlc: 'discover',
    cat: 'Understand',
    roles: [],
    prompt: 'explain what {path} does and how data flows through it. write it up as {format}',
    slots: {
      path: 'src/scheduler/queue.ts',
      format: 'an HTML page with a diagram, then open it in my browser'
    },
    nextHref: '/en/output-styles',
    src: 'workflows'
  }, {
    id: 'find-where-something-happens',
    sdlc: 'discover',
    cat: 'Understand',
    startN: 2,
    roles: [],
    prompt: 'where do we {behavior}?',
    slots: {
      behavior: 'validate uploaded file types'
    },
    src: 'workflows'
  }, {
    id: 'see-what-depends-on',
    sdlc: 'discover',
    cat: 'Understand',
    roles: [],
    prompt: 'what would break if I deleted {target}?',
    slots: {
      target: 'the retryWithBackoff helper'
    },
    src: 'workflows'
  }, {
    id: 'trace-how-code-evolved',
    sdlc: 'discover',
    cat: 'Understand',
    roles: [],
    prompt: 'look through the commit history of {path} and summarize how it evolved and why',
    slots: {
      path: 'internal/auth/session.go'
    },
    src: 'best-practices'
  }, {
    id: 'scope-a-change-before',
    sdlc: 'discover',
    cat: 'Understand',
    roles: ['pm', 'design'],
    prompt: 'which files would I need to touch to {change}?',
    slots: {
      change: 'add a dark mode toggle to settings'
    },
    src: 'teams'
  }, {
    id: 'ask-the-codebase-a',
    sdlc: 'discover',
    cat: 'Understand',
    roles: ['pm'],
    prompt: 'I am a {role}. walk me through what happens when a user {action}, from the UI down to the result',
    slots: {
      role: 'PM',
      action: 'clicks Export to PDF'
    },
    nextHref: '/en/output-styles',
    src: 'teams'
  }, {
    id: 'plan-a-multi-file',
    sdlc: 'design',
    cat: 'Plan',
    roles: ['pm', 'design'],
    prompt: 'plan how to refactor the {target} to {goal}. list the files you would change, but don\'t edit anything yet',
    slots: {
      target: 'payment module',
      goal: 'support multiple currencies'
    },
    src: 'workflows'
  }, {
    id: 'draft-a-spec-by',
    sdlc: 'design',
    cat: 'Plan',
    roles: ['pm'],
    prompt: 'I want to build {feature}. interview me about implementation, UX, edge cases, and tradeoffs until we have covered everything, then write the spec to SPEC.md',
    slots: {
      feature: 'per-workspace rate limits'
    },
    nextHref: '/en/skills',
    src: 'best-practices'
  }, {
    id: 'turn-a-meeting-into',
    sdlc: 'design',
    cat: 'Plan',
    roles: ['pm'],
    prompt: 'read {input} and write up the action items, then create a {tracker} ticket for each with acceptance criteria',
    slots: {
      input: '@meeting-notes.md',
      tracker: 'Linear'
    },
    needs: 'tracker',
    nextHref: '/en/skills',
    src: 'teams'
  }, {
    id: 'map-edge-cases-before',
    sdlc: 'design',
    cat: 'Plan',
    roles: ['design', 'pm'],
    prompt: 'list the error states, empty states, and edge cases for {feature} that the design needs to cover',
    slots: {
      feature: 'the file upload flow'
    },
    src: 'teams'
  }, {
    id: 'turn-a-mockup-into',
    sdlc: 'design',
    cat: 'Prototype',
    roles: ['design', 'pm', 'marketing'],
    paste: 'mockup',
    prompt: 'here is a mockup. build a working prototype I can click through, matching the layout and states shown',
    src: 'teams'
  }, {
    id: 'implement-from-a-screenshot',
    sdlc: 'design',
    cat: 'Prototype',
    roles: ['design'],
    paste: 'design',
    needs: 'browser',
    prompt: 'implement this design, then take a screenshot of the result, compare it to the original, and fix any differences',
    nextHref: '/en/goal',
    src: 'best-practices'
  }, {
    id: 'follow-an-existing-pattern',
    sdlc: 'build',
    cat: 'Implement',
    roles: [],
    prompt: 'look at how {example} is implemented to understand the pattern, then build {new} the same way',
    slots: {
      example: 'the GitHub webhook handler',
      new: 'a Stripe webhook handler'
    },
    nextHref: '/en/memory',
    src: 'best-practices'
  }, {
    id: 'generate-docs-for-code',
    sdlc: 'build',
    cat: 'Implement',
    roles: ['docs'],
    prompt: 'find {scope} without {format} comments and add them, matching the style already used in the file',
    slots: {
      scope: 'the public functions in src/auth/',
      format: 'JSDoc'
    },
    src: 'workflows'
  }, {
    id: 'add-a-small-well',
    sdlc: 'build',
    cat: 'Implement',
    roles: [],
    prompt: 'add a {endpoint} endpoint that returns {payload}',
    slots: {
      endpoint: '/health',
      payload: 'the app version and uptime'
    },
    src: 'workflows'
  }, {
    id: 'build-a-small-internal',
    sdlc: 'build',
    cat: 'Implement',
    roles: ['pm', 'design', 'marketing', 'docs'],
    prompt: 'create a {tool} using HTML, CSS, and vanilla JavaScript, then open it in my browser',
    slots: {
      tool: 'drag-and-drop Kanban board with three columns'
    },
    src: 'teams'
  }, {
    id: 'work-an-issue-end',
    sdlc: 'build',
    cat: 'Implement',
    roles: [],
    prompt: 'read issue #{issue}, implement the fix, and run the tests',
    slots: {
      issue: '312'
    },
    needs: 'gh',
    src: 'workflows'
  }, {
    id: 'find-and-update-copy',
    sdlc: 'build',
    cat: 'Implement',
    roles: ['design', 'docs', 'marketing'],
    prompt: 'find every place we say "{copy}" or a close variant, show me each one in context, then update them all to "{new}". leave tests and the changelog alone',
    slots: {
      copy: 'Sign up free',
      new: 'Start free trial'
    },
    src: 'teams'
  }, {
    id: 'draft-from-past-examples',
    sdlc: 'build',
    cat: 'Implement',
    roles: ['docs', 'marketing', 'pm'],
    prompt: 'read the {examples} in {folder} to learn the structure and voice, then draft a new one for {topic}',
    slots: {
      examples: 'privacy impact assessments',
      folder: 'legal/pia/',
      topic: 'the new analytics integration'
    },
    nextHref: '/en/skills',
    src: 'legal'
  }, {
    id: 'write-tests-run-them',
    sdlc: 'build',
    cat: 'Test',
    startN: 4,
    roles: [],
    prompt: 'write tests for {path}, run them, and fix any failures',
    slots: {
      path: 'app/parsers/feed.py'
    },
    nextHref: '/en/memory',
    src: 'workflows'
  }, {
    id: 'drive-implementation-from-tests',
    sdlc: 'build',
    cat: 'Test',
    roles: [],
    prompt: 'write tests for {feature} first, then implement it until they pass',
    slots: {
      feature: 'the password reset flow'
    },
    src: 'ebook'
  }, {
    id: 'fill-gaps-from-a',
    sdlc: 'build',
    cat: 'Test',
    roles: [],
    prompt: 'read {report} and add tests for the lowest-covered files until each is above {target}%',
    slots: {
      report: 'coverage/coverage-summary.json',
      target: '80'
    },
    nextHref: '/en/goal',
    src: 'workflows'
  }, {
    id: 'migrate-a-pattern-across',
    sdlc: 'build',
    cat: 'Refactor',
    roles: [],
    prompt: 'migrate everything from {from} to {to}: identify every place that needs to change, then make the changes',
    slots: {
      from: 'the old logging API',
      to: 'the structured logger'
    },
    src: 'workflows'
  }, {
    id: 'port-code-between-languages',
    sdlc: 'build',
    cat: 'Refactor',
    roles: [],
    prompt: 'port {source} to {target}, keeping the same {keep}',
    slots: {
      source: 'this Python module',
      target: 'Rust',
      keep: 'public API and test behavior'
    },
    src: 'teams'
  }, {
    id: 'optimize-against-a-measurable',
    sdlc: 'build',
    cat: 'Refactor',
    roles: ['data'],
    prompt: 'optimize {target} to bring {metric} from {current} down to under {goal}',
    slots: {
      target: 'the search query',
      metric: 'p95 latency',
      current: '2s',
      goal: '500ms'
    },
    nextHref: '/en/goal',
    src: 'ebook'
  }, {
    id: 'fix-a-precise-visual',
    sdlc: 'build',
    cat: 'Refactor',
    roles: ['design'],
    prompt: 'the {element} extends {amount} beyond the {container} on {viewport}. fix it.',
    slots: {
      element: 'login button',
      amount: '20px',
      container: 'card border',
      viewport: 'mobile'
    },
    nextHref: '/en/desktop#preview-your-app',
    src: 'ebook'
  }, {
    id: 'review-your-changes-before',
    sdlc: 'build',
    cat: 'Review',
    startN: 5,
    roles: [],
    prompt: 'review my uncommitted changes and flag anything that looks risky before I commit',
    nextHref: '/en/commands',
    src: 'workflows'
  }, {
    id: 'review-a-pull-request',
    sdlc: 'build',
    cat: 'Review',
    roles: [],
    prompt: 'review PR #{pr} and summarize what changed, then list any concerns',
    slots: {
      pr: '247'
    },
    needs: 'gh',
    nextHref: '/en/code-review',
    src: 'workflows'
  }, {
    id: 'review-infrastructure-changes-before',
    sdlc: 'build',
    cat: 'Review',
    roles: ['security', 'ops'],
    paste: 'plan',
    prompt: 'here is my Terraform plan output. what is this going to do, and is anything here going to cause problems?',
    src: 'teams'
  }, {
    id: 'run-a-security-review',
    sdlc: 'build',
    cat: 'Review',
    roles: ['security'],
    prompt: 'use a subagent to review {path} for security issues and report what it finds',
    slots: {
      path: 'src/api/'
    },
    nextHref: '/en/sub-agents',
    src: 'best-practices'
  }, {
    id: 'review-content-before-sending',
    sdlc: 'build',
    cat: 'Review',
    roles: ['marketing', 'docs'],
    prompt: 'review {file} for {concerns} and list anything I should fix before it goes to {reviewer}',
    slots: {
      file: 'launch-post.md',
      concerns: 'unsupported claims, missing attributions, and brand-guideline issues',
      reviewer: 'legal'
    },
    nextHref: '/en/skills',
    src: 'legal'
  }, {
    id: 'course-correct-a-wrong',
    sdlc: 'build',
    cat: 'Steer',
    roles: [],
    prompt: 'that is not right: {feedback}. try a different approach',
    slots: {
      feedback: 'the function signature needs to stay backward-compatible'
    },
    nextHref: '/en/checkpointing',
    src: 'best-practices'
  }, {
    id: 'narrow-the-scope-of',
    sdlc: 'build',
    cat: 'Steer',
    roles: [],
    prompt: 'that is too much. keep only the changes to {scope} and undo your other edits',
    slots: {
      scope: 'the validation logic in src/forms/'
    },
    src: 'best-practices'
  }, {
    id: 'turn-a-correction-into',
    sdlc: 'build',
    cat: 'Steer',
    roles: [],
    prompt: 'you keep {mistake}. add a rule to CLAUDE.md so this stops happening',
    slots: {
      mistake: 'using default exports when this project uses named exports'
    },
    nextHref: '/en/memory',
    src: 'best-practices'
  }, {
    id: 'resolve-merge-conflicts',
    sdlc: 'ship',
    cat: 'Git',
    roles: [],
    prompt: 'resolve the merge conflicts in this branch and explain what you kept from each side',
    src: 'workflows'
  }, {
    id: 'commit-with-a-generated',
    sdlc: 'ship',
    cat: 'Git',
    roles: [],
    prompt: 'commit these changes with a message that summarizes what I did',
    src: 'workflows'
  }, {
    id: 'open-a-pull-request',
    sdlc: 'ship',
    cat: 'Git',
    roles: [],
    prompt: 'find the {tracker} ticket about {topic} and open a PR that implements it',
    slots: {
      tracker: 'Linear',
      topic: 'the login timeout'
    },
    needs: 'tracker',
    src: 'workflows'
  }, {
    id: 'draft-release-notes-from',
    sdlc: 'ship',
    cat: 'Release',
    roles: ['pm', 'docs', 'marketing'],
    prompt: 'compare {from} to {to} and draft release notes grouped by feature, fix, and breaking change',
    slots: {
      from: 'v2.3.0',
      to: 'v2.4.0'
    },
    nextHref: '/en/skills',
    src: 'workflows'
  }, {
    id: 'write-a-ci-workflow',
    sdlc: 'ship',
    cat: 'Release',
    roles: ['ops'],
    prompt: 'write a GitHub Actions workflow that {steps} on every push to {branch}',
    slots: {
      steps: 'runs the tests and deploys to staging',
      branch: 'main'
    },
    src: 'workflows'
  }, {
    id: 'find-and-fix-a',
    sdlc: 'operate',
    cat: 'Debug',
    startN: 3,
    roles: [],
    prompt: 'the {test} test is failing, find out why and fix it',
    slots: {
      test: 'UserAuth'
    },
    src: 'workflows'
  }, {
    id: 'investigate-a-reported-error',
    sdlc: 'operate',
    cat: 'Debug',
    roles: ['ops'],
    prompt: 'users are seeing {symptom} on {where}. investigate and tell me what is going on',
    slots: {
      symptom: '500 errors',
      where: '/api/settings'
    },
    nextHref: '/en/web-quickstart#pre-fill-sessions',
    src: 'workflows'
  }, {
    id: 'fix-a-build-error',
    sdlc: 'operate',
    cat: 'Debug',
    roles: ['ops'],
    paste: 'error',
    prompt: 'here is a build error. fix the root cause and verify the build succeeds',
    src: 'best-practices'
  }, {
    id: 'investigate-a-production-incident',
    sdlc: 'operate',
    cat: 'Incident',
    roles: ['ops', 'security'],
    prompt: '{symptom}. check the logs, recent deploys, and config changes, then tell me the most likely cause',
    slots: {
      symptom: 'the checkout endpoint started returning 500s an hour ago'
    },
    nextHref: '/en/mcp',
    src: 'workflows'
  }, {
    id: 'diagnose-from-a-console',
    sdlc: 'operate',
    cat: 'Incident',
    roles: ['ops', 'data'],
    paste: 'screenshot',
    prompt: 'here is a screenshot of {console}. walk me through why {resource} is failing and give me the exact commands to fix it',
    slots: {
      console: 'the GCP Kubernetes dashboard',
      resource: 'this pod'
    },
    src: 'teams'
  }, {
    id: 'query-logs-in-plain',
    sdlc: 'operate',
    cat: 'Incident',
    roles: ['security', 'ops', 'data'],
    prompt: 'show me all {events} for {scope} over {timeframe}. write the query, run it, and tell me what stands out',
    slots: {
      events: 'failed logins',
      scope: 'the auth service',
      timeframe: 'the past 24 hours'
    },
    needs: 'db',
    src: 'cybersecurity'
  }, {
    id: 'analyze-a-data-file',
    sdlc: 'operate',
    cat: 'Data',
    roles: ['data', 'pm', 'marketing'],
    paste: 'csv',
    prompt: 'read {file}, summarize the key patterns, and write the results to {output}',
    slots: {
      file: '@reports/q1-signups.csv',
      output: 'an HTML page with charts, then open it in my browser'
    },
    nextHref: '/en/mcp',
    src: 'teams'
  }, {
    id: 'generate-variations-from-performance',
    sdlc: 'operate',
    cat: 'Data',
    roles: ['marketing', 'data'],
    paste: 'csv',
    prompt: 'read {file}, find the underperforming {items}, and generate {n} new variations that stay under {limit} characters',
    slots: {
      file: '@ads-performance.csv',
      items: 'headlines',
      n: '20',
      limit: '90'
    },
    nextHref: '/en/mcp',
    src: 'teams'
  }, {
    id: 'turn-a-recurring-task',
    sdlc: 'operate',
    cat: 'Automate',
    roles: [],
    prompt: 'create a /{name} skill for this project that {steps}',
    slots: {
      name: 'ship',
      steps: 'runs the linter and tests, then drafts a commit message'
    },
    src: 'workflows'
  }, {
    id: 'add-a-hook-for',
    sdlc: 'operate',
    cat: 'Automate',
    roles: [],
    prompt: 'write a hook that {action} after every {event}',
    slots: {
      action: 'runs prettier',
      event: 'edit to a .ts or .tsx file'
    },
    src: 'best-practices'
  }, {
    id: 'connect-a-tool-with',
    sdlc: 'operate',
    cat: 'Automate',
    roles: [],
    prompt: 'set up the {server} MCP server so you can read my {data} directly',
    slots: {
      server: 'Sentry',
      data: 'error reports'
    },
    src: 'workflows'
  }, {
    id: 'capture-what-to-remember',
    sdlc: 'operate',
    cat: 'Automate',
    roles: ['pm', 'docs'],
    prompt: 'summarize what we did this session and suggest what to add to CLAUDE.md',
    src: 'teams'
  }], []);
  const PROMPTS = useMemo(() => {
    if (typeof window !== 'undefined') {
      const rawIds = new Set(RAW.map(p => p.id));
      RAW.forEach(p => {
        if (!text[p.id]) console.warn('[prompt-library] no text[] entry for id:', p.id);
      });
      Object.keys(text).forEach(k => {
        if (!rawIds.has(k)) console.warn('[prompt-library] orphaned text[] key:', k);
      });
    }
    return RAW.map(p => ({
      ...p,
      title: p.id,
      teaches: '',
      ...text[p.id] || ({})
    }));
  }, [RAW, text]);
  const L = labels;
  const TL = k => tagLabels[k] || k;
  const CAT_TAG = useMemo(() => ({
    Onboard: 'understand',
    Understand: 'understand',
    Plan: 'plan',
    Prototype: 'prototype',
    Implement: 'build',
    Test: 'test',
    Refactor: 'refactor',
    Review: 'review',
    Steer: 'steer',
    Git: 'git',
    Release: 'release',
    Debug: 'debug',
    Incident: 'debug',
    Data: 'data',
    Automate: 'automate'
  }), []);
  const TAGS = useMemo(() => ['understand', 'plan', 'prototype', 'build', 'test', 'refactor', 'review', 'steer', 'debug', 'git', 'release', 'data', 'automate', 'pm', 'design', 'docs', 'marketing', 'security', 'ops'], []);
  const tagsOf = p => [CAT_TAG[p.cat], ...p.roles || []];
  const doc = useMemo(() => {
    const p = typeof window !== 'undefined' ? window.location.pathname : '';
    const base = p.startsWith('/docs/') ? '/docs' : '';
    const m = p.slice(base.length).match(/^\/([a-z]{2}(?:-[A-Z]{2})?)\//);
    const locale = m ? m[1] : 'en';
    return href => {
      if (!href || href[0] !== '/' || href[1] === '/') return href;
      return base + (href.startsWith('/en/') ? '/' + locale + href.slice(3) : href);
    };
  }, []);
  const linkify = s => {
    const out = [];
    let last = 0;
    const re = /\[([^\]]+)\]\(([^)]+)\)/g;
    for (let m; m = re.exec(s); ) {
      if (m.index > last) out.push(s.slice(last, m.index));
      out.push(<a key={m.index} href={doc(m[2])}>{m[1]}</a>);
      last = re.lastIndex;
    }
    if (last < s.length) out.push(s.slice(last));
    return out;
  };
  const codeify = s => s.split(/(`[^`]+`)/g).map((part, i) => part[0] === '`' ? <code key={i}>{part.slice(1, -1)}</code> : part);
  const SOURCES = useMemo(() => ({
    'workflows': '/en/common-workflows',
    'teams': 'https://claude.com/blog/how-anthropic-teams-use-claude-code',
    'legal': 'https://claude.com/blog/how-anthropic-uses-claude-legal',
    'cybersecurity': 'https://claude.com/blog/how-anthropic-uses-claude-cybersecurity',
    'best-practices': '/en/best-practices',
    'ebook': 'https://resources.anthropic.com/hubfs/Scaling%20agentic%20coding%20across%20your%20organization.pdf'
  }), []);
  const [mounted, setMounted] = useState(false);
  const [q, setQ] = useState('');
  const [start, setStart] = useState(true);
  const [sel, setSel] = useState(null);
  const [openId, setOpenId] = useState(null);
  const [copied, setCopied] = useState(null);
  const [fills, setFills] = useState({});
  const copyTimer = useRef(null);
  useEffect(() => {
    setMounted(true);
    return () => clearTimeout(copyTimer.current);
  }, []);
  const setFill = (id, key, val) => setFills(f => ({
    ...f,
    [id + '.' + key]: val
  }));
  const fillOf = (p, key) => {
    const v = fills[p.id + '.' + key];
    return v !== undefined ? v : p.slots && p.slots[key] !== undefined ? p.slots[key] : '';
  };
  const assemble = p => p.prompt.replace(/\{(\w+)\}/g, (_, k) => fillOf(p, k) || p.slots && p.slots[k] || k);
  const preview = p => p.prompt.replace(/\{(\w+)\}/g, (_, k) => p.slots && p.slots[k] || k);
  const bodyText = p => preview(p) + ' ' + p.teaches.replace(/\[([^\]]+)\]\([^)]+\)/g, '$1') + ' ' + (p.next || '');
  const widthFor = s => (s || '').length + 3 + 'ch';
  const ql = q.trim().toLowerCase();
  const toggleTag = k => {
    setStart(false);
    setSel(s => !ql && s === k ? null : k);
  };
  const clear = () => {
    setStart(false);
    setSel(null);
    setQ('');
  };
  const results = useMemo(() => {
    const list = PROMPTS.filter(p => {
      if (ql) return p.title.toLowerCase().includes(ql) || bodyText(p).toLowerCase().includes(ql);
      if (start) return !!p.startN;
      if (sel) return tagsOf(p).includes(sel);
      return true;
    });
    if (ql) return list;
    if (start) return list.sort((a, b) => a.startN - b.startN);
    if (sel) return list.sort((a, b) => (a.roles || []).length - (b.roles || []).length || (b.sdlc === 'operate') - (a.sdlc === 'operate'));
    return list;
  }, [PROMPTS, ql, start, sel]);
  const matchSnippet = p => {
    if (!ql || p.title.toLowerCase().includes(ql)) return null;
    const txt = bodyText(p);
    const at = txt.toLowerCase().indexOf(ql);
    if (at < 0) return null;
    const lo = Math.max(0, at - 30), hi = Math.min(txt.length, at + ql.length + 50);
    return [lo > 0 ? '…' : '', txt.slice(lo, at), <mark key="m">{txt.slice(at, at + ql.length)}</mark>, txt.slice(at + ql.length, hi), hi < txt.length ? '…' : ''];
  };
  const grouped = useMemo(() => {
    if (start && !q.trim()) return [];
    const g = {};
    for (const p of results) {
      const key = p.sdlc + '|' + p.cat;
      (g[key] = g[key] || ({
        sdlc: p.sdlc,
        cat: p.cat,
        items: []
      })).items.push(p);
    }
    return Object.values(g);
  }, [results, start, q]);
  const copy = async (str, id) => {
    try {
      await navigator.clipboard.writeText(str);
    } catch {
      const ta = document.createElement('textarea');
      ta.value = str;
      ta.setAttribute('readonly', '');
      ta.style.position = 'fixed';
      ta.style.opacity = '0';
      document.body.appendChild(ta);
      ta.select();
      document.execCommand('copy');
      document.body.removeChild(ta);
    }
    clearTimeout(copyTimer.current);
    setCopied(id);
    copyTimer.current = setTimeout(() => setCopied(null), 1600);
  };
  const promptBody = p => {
    if (!p.slots) return <code>{p.prompt}</code>;
    const parts = p.prompt.split(/(\{\w+\})/g);
    return <code>
        {parts.map((part, idx) => {
      const m = part.match(/^\{(\w+)\}$/);
      if (!m) return <span key={idx}>{part}</span>;
      const k = m[1];
      const val = fillOf(p, k);
      return <input key={idx} type="text" className="pl-slot" value={val} placeholder={p.slots[k] || k} aria-label={k} style={{
        width: widthFor(val || p.slots[k])
      }} onChange={e => setFill(p.id, k, e.target.value)} onFocus={e => e.target.select()} onClick={e => e.stopPropagation()} />;
    })}
      </code>;
  };
  const card = p => {
    const open = openId === p.id;
    const srcHref = SOURCES[p.src];
    const srcLabel = sourceLabels[p.src];
    const snip = matchSnippet(p);
    return <div key={p.id} className={'pl-card' + (open ? ' pl-open' : '')}>
        <button type="button" className="pl-head" onClick={() => setOpenId(open ? null : p.id)} aria-expanded={open}>
          <span className="pl-title">{p.title}</span>
          {!!p.startN && <span className="pl-chip">{L.startHere} · {p.startN}</span>}
        </button>
        {snip ? <div className="pl-match">{snip}</div> : <code className="pl-prompt-preview">{preview(p)}</code>}
        {open && <div className="pl-body">
            <div className="pl-label">{p.slots ? L.fillAndCopy : L.copyThis}</div>
            {p.needs && L.needs && L.needs[p.needs] && <div className="pl-hint pl-needs">
                <span className="pl-needs-label">{L.needsLabel}</span> {linkify(L.needs[p.needs])}
              </div>}
            {p.paste && L.paste && L.paste[p.paste] && <div className="pl-hint pl-paste">{L.paste[p.paste]}</div>}
            {p.slots && <div className="pl-hint">
                {L.hintBefore} <span className="pl-hint-chip">{L.hintChip}</span> {L.hintAfter}
              </div>}
            <div className="pl-prompt-box">
              <span className="pl-caret">{'❯'}</span>
              {promptBody(p)}
              <button type="button" className="pl-copy" onClick={() => copy(assemble(p), p.id)}>
                {copied === p.id ? L.copied : L.copy}
              </button>
            </div>
            <div className="pl-label">{L.whyWorks}</div>
            <div className="pl-teaches">{linkify(p.teaches)}</div>
            {p.nextHref && p.next && <div className="pl-next">
                <span className="pl-next-label">{L.makeItStick}</span>
                <a href={doc(p.nextHref)}>{codeify(p.next)} →</a>
              </div>}
            {srcLabel && <div className="pl-src">{L.from} {srcHref ? <a href={doc(srcHref)}>{srcLabel}</a> : srcLabel}</div>}
          </div>}
      </div>;
  };
  const STYLES = useMemo(() => `
.pl {
  --pl-accent: #D97757;
  --pl-accent-bg: rgba(217,119,87,0.07);
  --pl-bg: #fff;
  --pl-surface: #FAFAF7;
  --pl-border: #E8E6DC;
  --pl-border-subtle: rgba(31,30,29,0.08);
  --pl-text: #141413;
  --pl-text-2: #5E5D59;
  --pl-text-3: #73726C;
  --pl-text-4: #9C9A92;
  --pl-mono: var(--font-mono, ui-monospace, SFMono-Regular, Menlo, monospace);
  font-family: 'Anthropic Sans', -apple-system, BlinkMacSystemFont, sans-serif;
  font-size: 16px; color: var(--pl-text); margin: 8px 0 32px;
}
.dark .pl {
  --pl-bg: #1f1e1d;
  --pl-surface: #262624;
  --pl-border: #3d3d3a;
  --pl-border-subtle: rgba(240,238,230,0.08);
  --pl-text: #f0eee6;
  --pl-text-2: #bfbdb4;
  --pl-text-3: #91908a;
  --pl-text-4: #73726c;
}
.pl *, .pl *::before, .pl *::after { box-sizing: border-box; }
.pl button { font-family: inherit; cursor: pointer; }
.pl a { color: var(--pl-accent); text-decoration: none; }
.pl a:hover { text-decoration: underline; }

.pl-search {
  display: flex; align-items: center; gap: 10px;
  padding: 14px 18px; background: var(--pl-surface);
  border: 1px solid var(--pl-border); border-radius: 12px;
  margin-bottom: 14px;
}
.pl-search input {
  flex: 1; border: none; outline: none; background: transparent;
  font-size: 16px; color: var(--pl-text);
}
.pl-search input::placeholder { color: var(--pl-text-4); }

.pl-tags { display: flex; gap: 8px; flex-wrap: wrap; align-items: center; margin-bottom: 18px; }
.pl-tag {
  padding: 7px 14px; border: 1px solid var(--pl-border); background: var(--pl-bg);
  font-size: 14px; color: var(--pl-text-2); border-radius: 999px;
}
.pl-tag:hover { background: var(--pl-surface); }
.pl-tag.pl-on { background: var(--pl-text); border-color: var(--pl-text); color: var(--pl-bg); }
.pl-tag.pl-start { color: var(--pl-accent); font-weight: 500; }
.pl-tag.pl-start.pl-on { background: var(--pl-accent); border-color: var(--pl-accent); color: #fff; }
.pl-tags.pl-dim .pl-tag { opacity: 0.5; }
.pl-tags.pl-dim .pl-tag:hover { opacity: 1; }
.pl-sep { width: 1px; height: 22px; background: var(--pl-border); margin: 0 4px; }
.pl-clear { border: none; background: none; font-size: 13px; color: var(--pl-text-4); padding: 4px 6px; }
.pl-clear:hover { color: var(--pl-text-2); }
.pl-count { margin-left: auto; font-size: 14px; color: var(--pl-text-4); }

.pl-group-h {
  font-size: 12px; letter-spacing: 0.08em; text-transform: uppercase;
  color: var(--pl-text-4); margin: 24px 0 12px;
}
.pl-group-h .pl-phase { color: var(--pl-text-3); }
.pl-card {
  border: 1px solid var(--pl-border-subtle); border-radius: 10px;
  margin-bottom: 12px; background: var(--pl-bg); overflow: hidden;
  padding: 14px 18px;
}
.pl-card.pl-open { border-color: var(--pl-border); background: var(--pl-surface); }
.pl-head {
  width: 100%; display: flex; align-items: baseline; gap: 12px;
  border: none; background: transparent; text-align: left; padding: 0;
}
.pl-head:focus-visible { outline: 2px solid var(--pl-accent); outline-offset: 2px; border-radius: 6px; }
.pl-title {
  flex: 1; font-size: 17px; font-weight: 500; color: var(--pl-text);
  white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
}
.pl-prompt-preview {
  display: block; font-family: var(--pl-mono); font-size: 13.5px; color: var(--pl-text-3);
  margin-top: 6px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
}
.pl-chip {
  font-size: 11px; letter-spacing: 0.05em; text-transform: uppercase;
  padding: 3px 9px; border-radius: 999px; flex-shrink: 0;
  background: var(--pl-accent-bg); color: var(--pl-accent);
}

.pl-body { margin-top: 14px; padding-top: 14px; border-top: 1px solid var(--pl-border-subtle); }
.pl-label {
  font-size: 11.5px; letter-spacing: 0.08em; text-transform: uppercase;
  color: var(--pl-text-4); margin: 12px 0 8px;
}
.pl-prompt-box {
  display: flex; align-items: center; gap: 10px;
  padding: 14px 16px; background: #141413; color: #f0eee6;
  border-radius: 8px; font-family: var(--pl-mono); font-size: 15px;
}
.pl-caret { color: var(--pl-accent); flex-shrink: 0; }
.pl-prompt-box code { flex: 1; background: none; padding: 0; color: inherit; white-space: pre-wrap; line-height: 1.9; }
.pl-slot {
  font-family: var(--pl-mono); font-size: inherit;
  background: rgba(217,119,87,0.15); color: #f0eee6;
  border: none; border-bottom: 1.5px dashed var(--pl-accent);
  border-radius: 4px 4px 0 0; padding: 2px 6px; margin: 0 1px;
  outline: none; min-width: 6ch; max-width: 100%;
  box-sizing: content-box; cursor: text;
}
.pl-slot:hover { background: rgba(217,119,87,0.22); }
.pl-slot:focus { background: rgba(217,119,87,0.28); border-bottom-style: solid; }
.pl-slot::placeholder { color: rgba(240,238,230,0.4); font-style: italic; }
.pl-hint { font-size: 14px; color: var(--pl-text-3); margin: 0 0 10px; }
.pl-paste { color: var(--pl-text-2); }
.pl-needs { color: var(--pl-text-2); }
.pl-needs-label {
  display: inline-block; font-size: 10.5px; letter-spacing: 0.06em;
  text-transform: uppercase; padding: 2px 7px; margin-right: 6px;
  border-radius: 4px; background: var(--pl-accent-bg); color: var(--pl-accent);
}
.pl-hint-chip {
  font-family: var(--pl-mono); font-size: 0.92em;
  background: var(--pl-accent-bg); color: var(--pl-accent);
  border-bottom: 1.5px dashed var(--pl-accent);
  border-radius: 3px 3px 0 0; padding: 1px 5px;
}
.pl-copy {
  font-size: 12.5px; padding: 6px 12px; border-radius: 6px;
  background: var(--pl-accent); color: #fff; border: none; flex-shrink: 0;
}
.pl-teaches { display: block; font-size: 15.5px; color: var(--pl-text-2); margin: 4px 0 0; line-height: 1.6; }
.pl-match {
  display: block; font-size: 13.5px; color: var(--pl-text-3);
  margin-top: 6px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
}
.pl-match mark { background: var(--pl-accent-bg); color: var(--pl-text); padding: 1px 2px; border-radius: 3px; }
.pl-next {
  display: flex; align-items: baseline; gap: 10px;
  margin: 14px 0 0; padding: 10px 12px;
  background: var(--pl-accent-bg); border-radius: 8px; font-size: 14.5px;
}
.pl-next-label {
  font-size: 11px; letter-spacing: 0.06em; text-transform: uppercase;
  color: var(--pl-accent); font-weight: 600; flex-shrink: 0;
}
.pl-src { display: block; font-size: 14px; color: var(--pl-text-4); margin: 14px 0 0; }

.pl-show-all {
  display: block; width: 100%; padding: 14px; margin-top: 4px;
  border: 1px dashed var(--pl-border); border-radius: 10px;
  background: transparent; font-size: 15px; color: var(--pl-accent);
  text-align: center;
}
.pl-show-all:hover { background: var(--pl-accent-bg); border-style: solid; }

.pl-empty {
  padding: 32px; text-align: center; color: var(--pl-text-4);
  border: 1px dashed var(--pl-border); border-radius: 10px;
}
`, []);
  if (!mounted) return <div className="pl" style={{
    minHeight: 480
  }} />;
  return <div className="pl">
      <style>{STYLES}</style>

      <div className="pl-search">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" style={{
    color: 'var(--pl-text-4)'
  }}>
          <circle cx="11" cy="11" r="7" /><line x1="21" y1="21" x2="16.65" y2="16.65" />
        </svg>
        <input type="text" placeholder={L.search} value={q} onChange={e => {
    setQ(e.target.value);
    if (e.target.value) setStart(false);
  }} aria-label={L.search} />
      </div>

      <div className={'pl-tags' + (ql ? ' pl-dim' : '')}>
        <button type="button" className={'pl-tag pl-start' + (!ql && start ? ' pl-on' : '')} onClick={() => {
    setQ('');
    setStart(!start);
    if (!start) setSel(null);
  }}>
          ★ {L.startHere}
        </button>
        <span className="pl-sep" />
        {TAGS.map(k => <button key={k} type="button" aria-pressed={!ql && sel === k} className={'pl-tag' + (!ql && sel === k ? ' pl-on' : '')} onClick={() => {
    setQ('');
    toggleTag(k);
  }}>
            {TL(k)}
          </button>)}
        {(start || sel || q) && <button type="button" className="pl-clear" onClick={clear}>{L.clear}</button>}
        <span className="pl-count">{results.length} {results.length === 1 ? L.prompt : L.prompts}</span>
      </div>

      {results.length === 0 ? <div className="pl-empty">
          {L.noMatch} {ql ? <code>{q}</code> : null} <button type="button" className="pl-clear" onClick={clear}>{L.clear}</button>
        </div> : !ql && start ? <div>
          <div className="pl-group-h">{L.startHereHeader}</div>
          {results.map(card)}
          <button type="button" className="pl-show-all" onClick={clear}>
            {L.showAll && L.showAll.replace('{n}', PROMPTS.length)} →
          </button>
        </div> : grouped.map(g => <div key={g.sdlc + '|' + g.cat}>
            <div className="pl-group-h"><span className="pl-phase">{phaseLabels[g.sdlc] || g.sdlc}</span> · {catLabels[g.cat] || g.cat}</div>
            {g.items.map(card)}
          </div>)}
    </div>;
};

Questa è una libreria di prompt da copiare in Claude Code. Utilizzatela per esplorare modi di lavorare che non avete ancora provato, o quando non siete sicuri da dove iniziare.

I prompt sono raccolti da varie guide Anthropic, tra cui [Flussi di lavoro comuni](/docs/it/common-workflows), [Best practice](/docs/it/best-practices) e [Come i team di Anthropic utilizzano Claude Code](https://claude.com/blog/how-anthropic-teams-use-claude-code). Sono punti di partenza piuttosto che script. Aprite **Perché funziona** sotto qualsiasi prompt per vedere il modello dietro di esso, così potrete scrivere i vostri.

export const labels = {
  startHere: "Iniziate da qui",
  startHereHeader: "Cinque prompt da provare per primi",
  showAll: "Mostra tutti i {n} prompt",
  search: "Cerca prompt…",
  clear: "Cancella",
  prompt: "prompt",
  prompts: "prompt",
  noMatch: "Nessun prompt corrisponde",
  fillAndCopy: "Compila e copia",
  copyThis: "Copia questo prompt",
  hintBefore: "Digitate nei",
  hintChip: "campi evidenziati",
  hintAfter: "per personalizzare, quindi copiate.",
  copy: "Copia",
  copied: "Copiato",
  whyWorks: "Perché funziona",
  makeItStick: "Rendilo permanente",
  from: "Da",
  paste: {
    mockup: "Incollate, trascinate o @-menzionate la vostra immagine mockup, quindi inviate questo:",
    design: "Incollate, trascinate o @-menzionate la vostra immagine di design, quindi inviate questo:",
    screenshot: "Incollate, trascinate o @-menzionate il vostro screenshot, quindi inviate questo:",
    plan: "Incollate l'output del vostro piano nel prompt per primo, quindi inviate questo:",
    error: "Incollate l'output dell'errore nel prompt per primo, quindi inviate questo:",
    csv: "Trascinate il vostro file nel prompt, o sostituite il percorso sottostante con una @-menzione della vostra:"
  },
  needsLabel: "Richiede",
  needs: {
    tracker: "il vostro issue tracker aggiunto come [connettore claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai) o [server MCP](/docs/it/mcp).",
    gh: "la [gh CLI](https://cli.github.com) autenticata, o GitHub aggiunto come [connettore claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai).",
    browser: "un modo per Claude di renderizzare e fare uno screenshot del risultato. L'[app Desktop](/docs/it/desktop#preview-your-app) ha questa funzione integrata. Nel terminale, installate l'[estensione Chrome](/docs/it/chrome) o un server MCP [Playwright](/docs/it/mcp).",
    db: "il vostro data warehouse o log store aggiunto come [connettore claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai) o [server MCP](/docs/it/mcp)."
  }
};

export const tagLabels = {
  understand: "Comprendere",
  plan: "Pianificare",
  prototype: "Prototipo",
  build: "Costruire",
  test: "Test",
  refactor: "Refactoring",
  review: "Revisione",
  steer: "Guidare",
  debug: "Debug",
  git: "Git",
  release: "Release",
  data: "Dati",
  automate: "Automatizzare",
  pm: "Prodotto",
  design: "Design",
  docs: "Documentazione",
  marketing: "Marketing",
  security: "Sicurezza",
  ops: "On-call"
};

export const phaseLabels = {
  discover: "Scoprire",
  design: "Progettare",
  build: "Costruire",
  ship: "Rilasciare",
  operate: "Operare"
};

export const sourceLabels = {
  workflows: "Flussi di lavoro comuni",
  teams: "Come i team di Anthropic utilizzano Claude Code",
  legal: "Come Anthropic utilizza Claude in Legal",
  cybersecurity: "Come Anthropic utilizza Claude in Cybersecurity",
  "best-practices": "Best practice",
  ebook: "Guida al scaling del coding agentico"
};

export const catLabels = {
  Onboard: "Onboard",
  Understand: "Comprendere",
  Plan: "Pianificare",
  Prototype: "Prototipo",
  Implement: "Implementare",
  Test: "Test",
  Refactor: "Refactoring",
  Review: "Revisione",
  Steer: "Guidare",
  Git: "Git",
  Release: "Release",
  Debug: "Debug",
  Incident: "Incidente",
  Data: "Dati",
  Automate: "Automatizzare"
};

export const text = {
  "get-oriented-in-a": {
    title: "Orientarsi in un nuovo repository",
    teaches: "Descrivete quello che volete sapere, non quali file leggere. Claude esplora il progetto autonomamente e restituisce un riepilogo di come si incastra.",
    next: "Eseguite `/init` per configurare `CLAUDE.md` in modo che Claude lo ricordi ogni sessione"
  },
  "explain-unfamiliar-code": {
    title: "Spiegare codice non familiare",
    teaches: "Nominate il file e dite in quale formato volete la risposta. Sostituite la pagina HTML con un diagramma, punti elenco, o quello che si adatta al vostro modo di imparare.",
    next: "Impostate uno stile di output in modo che Claude spieghi sempre nel vostro formato preferito"
  },
  "find-where-something-happens": {
    title: "Trovare dove accade qualcosa",
    teaches: "Cercate per comportamento invece che per nome file. La ricerca funziona anche quando non sapete come si chiama il file o in quale directory si trova."
  },
  "see-what-depends-on": {
    title: "Controllare cosa si rompe prima di eliminare",
    teaches: "Chiedete prima di rimuovere qualcosa. L'elenco dei chiamanti e degli effetti a valle vi dice se state guardando una pulizia di una riga o un cambiamento che dovete coordinare."
  },
  "trace-how-code-evolved": {
    title: "Tracciare come il codice si è evoluto",
    teaches: "Puntate alla cronologia dei commit quando la domanda è il perché, non il cosa. Claude legge il log e il blame per qualsiasi sistema di controllo versione utilizzate e spiega le decisioni dietro l'implementazione attuale."
  },
  "scope-a-change-before": {
    title: "Definire l'ambito di un cambiamento prima di iniziare",
    teaches: "Dimensionate il lavoro prima di impegnarvi in una roadmap. L'elenco dei file vi dice se state guardando un componente o un cambiamento trasversale."
  },
  "ask-the-codebase-a": {
    title: "Fare una domanda di prodotto alla codebase",
    teaches: "Dichiarate il vostro ruolo in modo che la risposta sia al livello giusto. Claude spiega cosa il prodotto effettivamente fa dal codice sorgente, senza che voi abbiate bisogno di leggerlo.",
    next: "Impostate uno stile di output in modo che Claude sempre presenti le risposte a questo livello"
  },
  "plan-a-multi-file": {
    title: "Pianificare un cambiamento multi-file prima di toccare il codice",
    teaches: "Aggiungere \"non modificare ancora\" separa l'esplorazione dai cambiamenti, così vedete l'approccio prima che qualsiasi codice si muova. Per rendere plan-first il default su ogni prompt, premete Shift+Tab per [plan mode](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode)."
  },
  "draft-a-spec-by": {
    title: "Bozza di una specifica tramite intervista",
    teaches: "Chiedete di essere intervistati invece di scrivere la specifica voi stessi. Claude vi pone domande strutturate finché i requisiti non sono completi, quindi scrive il risultato in un file.",
    next: "Salvate le vostre domande di intervista come una skill `/spec` in modo che ogni specifica inizi allo stesso modo"
  },
  "turn-a-meeting-into": {
    title: "Trasformare una riunione in ticket",
    teaches: "Saltate il passaggio della trascrizione. Claude estrae gli elementi d'azione dall'input non strutturato e li scrive direttamente nel vostro tracker tramite [MCP](/docs/it/mcp), così rivedete i ticket, non la trascrizione.",
    next: "Salvate questo come una skill `/tickets`"
  },
  "map-edge-cases-before": {
    title: "Mappare i casi limite prima di costruire",
    teaches: "Chiedete cosa manca, non cosa c'è. Claude elenca gli stati di errore, gli stati vuoti e i casi limite che un design happy-path tende a saltare."
  },
  "turn-a-mockup-into": {
    title: "Trasformare un mockup in un prototipo funzionante",
    teaches: "Un prototipo cliccabile risponde a domande che un mockup statico non può. Consegnate il codice funzionante all'ingegneria invece di spiegare le interazioni in un documento."
  },
  "implement-from-a-screenshot": {
    title: "Implementare da uno screenshot e auto-verificare",
    teaches: "Questo dà a Claude un ciclo di verifica: renderizza, confronta con l'immagine sorgente, e itera senza che voi indichiate ogni lacuna.",
    next: "Utilizzate `/goal` per mantenere Claude iterando finché gli screenshot non corrispondono"
  },
  "follow-an-existing-pattern": {
    title: "Seguire un modello esistente",
    teaches: "Puntate al codice che vi piace già. Senza un riferimento, Claude usa come default le best practice generali. Con uno, corrisponde alle convenzioni che la vostra codebase effettivamente usa.",
    next: "Chiedete a Claude di scrivere il modello che ha seguito in `CLAUDE.md` in modo che le sessioni future lo corrispondano senza il riferimento"
  },
  "add-a-small-well": {
    title: "Aggiungere una piccola funzionalità ben definita",
    teaches: "Dichiarate gli input e gli output, non come costruirla. Claude trova dove vive il codice simile e aggiunge il vostro accanto."
  },
  "build-a-small-internal": {
    title: "Costruire un piccolo strumento interno da zero",
    teaches: "Non avete bisogno di un progetto, un framework, o un passo di build. Descrivete lo strumento e chiedete a Claude di aprirlo in modo che lo vediate funzionare immediatamente."
  },
  "work-an-issue-end": {
    title: "Lavorare un problema da capo a fondo",
    teaches: "Date il numero del problema, non un riepilogo. Claude legge il ticket completo stesso, così i requisiti che dimentichereste di menzionare vengono attraversati, e valida il cambiamento prima di riferire."
  },
  "find-and-update-copy": {
    title: "Trovare e aggiornare il testo in tutta la codebase",
    teaches: "Chiedete varianti e dite cosa saltare. Claude trova fraseologie che una ricerca letterale perderebbe e lascia i fixture di test e la cronologia intatti, così rivedete solo il testo che gli utenti effettivamente vedono."
  },
  "draft-from-past-examples": {
    title: "Bozza di un documento da esempi passati",
    teaches: "Puntate a una cartella di lavoro finita invece di descrivere il vostro stile. Claude impara la struttura e la voce da quello che avete già spedito, così la prima bozza legge come una delle vostre.",
    next: "Salvate la voce come una skill in modo che ogni bozza inizi lì"
  },
  "write-tests-run-them": {
    title: "Scrivere test, eseguirli, correggere i fallimenti",
    teaches: "Chiedete di scrivere, eseguire e correggere insieme in modo che Claude iteri senza fermarsi per istruzioni.",
    next: "Eseguite `/init` in modo che Claude impari automaticamente il vostro comando di test"
  },
  "drive-implementation-from-tests": {
    title: "Guidare l'implementazione dai test",
    teaches: "Test-driven development: i test definiscono quando il lavoro è completo, e Claude itera sull'implementazione finché non passano."
  },
  "fill-gaps-from-a": {
    title: "Riempire le lacune da un rapporto di copertura",
    teaches: "Puntate al rapporto di copertura invece di indovinare cosa non è testato. Claude legge i numeri effettivi e scrive test per i file che ne hanno più bisogno.",
    next: "Impostate questo come un `/goal` in modo che Claude continui a scrivere test finché la copertura non raggiunge l'obiettivo"
  },
  "port-code-between-languages": {
    title: "Portare il codice in un'altra lingua",
    teaches: "Dite cosa preservare, non solo il linguaggio di destinazione. Nominare l'API o il comportamento che deve rimanere lo stesso dà a Claude un contratto per verificare il port."
  },
  "generate-docs-for-code": {
    title: "Generare documentazione per codice non documentato",
    teaches: "Nominate l'ambito e il formato. Claude trova cosa manca e corrisponde allo stile di commento già nel file, così la nuova documentazione legge come il resto."
  },
  "migrate-a-pattern-across": {
    title: "Migrare un modello in tutta la codebase",
    teaches: "Descrivete il vecchio modello e il nuovo. Chiedere a Claude di identificare prima ogni posto significa che i siti di chiamata sono elencati nella risposta, così potete controllare che nessuno sia stato perso."
  },
  "optimize-against-a-measurable": {
    title: "Ottimizzare rispetto a un obiettivo misurabile",
    teaches: "Dichiarare la metrica e l'obiettivo dà a Claude una chiara definizione di fatto.",
    next: "Impostate questo come un `/goal` in modo che Claude continui a misurare e iterare finché non raggiunge il numero"
  },
  "fix-a-precise-visual": {
    title: "Correggere un bug visivo preciso",
    teaches: "Un feedback visivo preciso ottiene una correzione precisa. Dichiarate l'elemento esatto, la misurazione e il viewport.",
    next: "Aggiungete uno strumento di anteprima in modo che Claude faccia uno screenshot e verifichi la correzione stessa"
  },
  "review-your-changes-before": {
    title: "Rivedere i vostri cambiamenti prima di eseguire il commit",
    teaches: "Catturate i problemi mentre sono ancora economici da correggere. Claude legge i file modificati per intero, non solo le righe diff, così individua i problemi che una rapida auto-revisione perde.",
    next: "Eseguite `/code-review` per lo stesso controllo in un comando"
  },
  "review-a-pull-request": {
    title: "Rivedere una pull request",
    teaches: "Claude rivede con l'intera codebase in contesto, non solo il diff. Legge il codice modificato e quello che chiama, così cattura i problemi che una revisione solo diff perderebbe.",
    next: "Attivate questo per ogni PR con Code Review"
  },
  "review-infrastructure-changes-before": {
    title: "Rivedere i cambiamenti dell'infrastruttura prima di applicare",
    teaches: "L'output del piano è denso e difficile da scansionare. Incollarlo vi dà un riepilogo in linguaggio semplice di cosa effettivamente cambierà prima di applicarlo."
  },
  "run-a-security-review": {
    title: "Eseguire una revisione di sicurezza con un subagent",
    teaches: "Un [subagent](/docs/it/sub-agents) esegue l'audit nel suo proprio context window e riferisce un riepilogo, così una lunga revisione di sicurezza non riempie la vostra sessione principale. Il subagent generico integrato gestisce questo senza configurazione aggiuntiva.",
    next: "Configurate un subagent dedicato security-review che tutto il vostro team può usare"
  },
  "review-content-before-sending": {
    title: "Catturare i problemi prima della revisione formale",
    teaches: "Ottenete un primo passaggio prima che un umano spenda tempo su di esso. Nominate i problemi che volete controllati in modo che la revisione sia focalizzata, quindi correggete quello che trova e inviate una bozza più pulita.",
    next: "Catturate la vostra lista di controllo di revisione come una skill che tutto il vostro team può eseguire"
  },
  "course-correct-a-wrong": {
    title: "Correggere il corso di un approccio sbagliato",
    teaches: "Nominate il vincolo che Claude ha perso, non solo che è sbagliato. Una ragione specifica dà a Claude un vincolo concreto da soddisfare al nuovo tentativo, invece di indovinare di nuovo.",
    next: "Premete `Esc` due volte per aprire il menu di rewind e ripristinare il codice e la conversazione in modo che il nuovo tentativo inizi pulito"
  },
  "narrow-the-scope-of": {
    title: "Restringere l'ambito di un cambiamento",
    teaches: "Quando la direzione è giusta ma il cambiamento è diventato troppo ampio, chiedete a Claude di mantenere parte di esso piuttosto che fare il rewind di tutto. Un confine dichiarato mantiene una piccola correzione dal diventare un refactoring."
  },
  "turn-a-correction-into": {
    title: "Trasformare una correzione in una regola",
    teaches: "Una correzione in chat non è condivisa con il vostro team. Una regola nel [CLAUDE.md](/docs/it/memory) del progetto è condivisa una volta che la eseguite il commit, e Claude la legge all'inizio di ogni sessione.",
    next: "Aprite `/memory` per rivedere quello che Claude ha scritto"
  },
  "resolve-merge-conflicts": {
    title: "Risolvere i conflitti di merge",
    teaches: "Dite quale stato volete, non quali marcatori mantenere. Chiedere il ragionamento rende il merge revisionabile invece di una scatola nera."
  },
  "commit-with-a-generated": {
    title: "Eseguire il commit con un messaggio generato",
    teaches: "Lasciate che Claude derivi il messaggio dal diff. Corrisponde allo stile di commit esistente del vostro repository."
  },
  "open-a-pull-request": {
    title: "Aprire una pull request da un ticket",
    teaches: "Saltate il cambio di contesto tra tracker, editor e GitHub. Un prompt legge la specifica, fa il cambiamento e apre la PR."
  },
  "draft-release-notes-from": {
    title: "Bozza di note di rilascio dalla cronologia git",
    teaches: "Date due punti di riferimento e la struttura che volete. Claude legge il log dei commit tra di loro e bozza un changelog che potete modificare.",
    next: "Salvate questo come una skill `/changelog`"
  },
  "write-a-ci-workflow": {
    title: "Scrivere un workflow CI",
    teaches: "Descrivete quando dovrebbe eseguire e cosa dovrebbe fare; lo YAML è generato per voi, abbinato ai vostri comandi di build e test del progetto."
  },
  "find-and-fix-a": {
    title: "Trovare e correggere un test fallito",
    teaches: "Descrivete il sintomo; non avete bisogno di sapere quale file è rotto. Claude esegue il test per vedere il fallimento, lo traccia nel sorgente e lo corregge."
  },
  "investigate-a-reported-error": {
    title: "Investigare un errore segnalato",
    teaches: "Descrivete il sintomo e la posizione; Claude legge il percorso di codice rilevante e traccia le cause probabili. Incollate stack trace o log se li avete.",
    next: "Mettete un deeplink nel vostro runbook che apre Claude con questo prompt pre-compilato"
  },
  "fix-a-build-error": {
    title: "Correggere un errore di build alla radice",
    teaches: "Chiedere la causa radice e la verifica previene patch superficiali che sopprimono l'errore senza correggerlo."
  },
  "investigate-a-production-incident": {
    title: "Investigare un incidente di produzione",
    teaches: "Elencate le fonti di evidenza da correlare, non i passaggi da intraprendere. Claude legge log, cronologia git e config insieme per restringere la causa.",
    next: "Collegate Sentry o il vostro log store tramite MCP"
  },
  "query-logs-in-plain": {
    title: "Interrogare i log in inglese semplice",
    teaches: "Fate la domanda invece di scrivere l'SQL. Claude costruisce la query, la esegue contro i vostri log collegati, e mostra sia la query che il risultato in modo che possiate controllare cosa è stato eseguito."
  },
  "diagnose-from-a-console": {
    title: "Diagnosticare da uno screenshot della console",
    teaches: "Le console cloud vi mostrano il problema ma non i comandi per correggerlo. Claude legge lo screenshot e traduce il dashboard nei comandi kubectl, gcloud o aws da eseguire."
  },
  "analyze-a-data-file": {
    title: "Analizzare un file di dati",
    teaches: "Una domanda una tantum non ha bisogno di uno script una tantum. Puntate a un file nella vostra cartella di progetto e Claude lo legge direttamente, trova i modelli e scrive l'output dove lo chiedete.",
    next: "Collegate la fonte di dati tramite MCP invece di esportare file"
  },
  "generate-variations-from-performance": {
    title: "Generare variazioni dai dati di performance",
    teaches: "Dichiarate il vincolo all'inizio in modo che la generazione rimanga entro il limite. Claude legge le metriche, sceglie cosa sostituire e produce alternative che si adattano.",
    next: "Collegate la piattaforma pubblicitaria tramite MCP invece di esportare un file"
  },
  "turn-a-recurring-task": {
    title: "Trasformare un'attività ricorrente in una skill",
    teaches: "Nominate i passaggi una volta; riutilizzateli come comando. Claude scrive una [skill](/docs/it/skills) che chiunque nel vostro team può eseguire."
  },
  "add-a-hook-for": {
    title: "Aggiungere un hook per il comportamento ripetuto",
    teaches: "Gli hook rendono un comportamento automatico invece di qualcosa che dovete ricordare di chiedere. Descrivete il trigger e l'azione e Claude scrive la configurazione [hook](/docs/it/hooks)."
  },
  "connect-a-tool-with": {
    title: "Connettere uno strumento con MCP",
    teaches: "Collegate la fonte una volta invece di incollare i dati ogni sessione. Dopo la configurazione [MCP](/docs/it/mcp), Claude legge dallo strumento direttamente quando lo chiedete."
  },
  "capture-what-to-remember": {
    title: "Catturare cosa ricordare per la prossima volta",
    teaches: "Chiedete prima di dimenticare. Claude sa cosa ha dovuto capire questa sessione e propone voci [CLAUDE.md](/docs/it/memory) in modo che la sessione successiva inizi con quel contesto."
  }
};

<PromptLibrary text={text} labels={labels} tagLabels={tagLabels} phaseLabels={phaseLabels} sourceLabels={sourceLabels} catLabels={catLabels} />

<h2 id="what-makes-these-prompts-work">
  Cosa rende questi prompt efficaci
</h2>

I prompt sopra condividono alcuni modelli. Riconoscerli vi aiuta ad adattare qualsiasi prompt qui al vostro compito.

**Descrivete il risultato, non i passaggi.** Dite quello che volete e lasciate che Claude trovi i file. Il prompt sottostante funziona senza nominare un singolo percorso di file.

```text theme={null}
add rate limiting to the public API and make sure existing tests still pass
```

**Dategli un modo per controllare il suo lavoro.** Chiedete di eseguire, testare, confrontare o verificare nello stesso prompt in modo che Claude iteri invece di fermarsi dopo un tentativo.

```text theme={null}
write the migration, run it against the dev database, and confirm the schema matches
```

**Puntate a un riferimento.** Nominate un file, test o modello esistente da abbinare in modo che il nuovo codice sia coerente con quello che avete già.

```text theme={null}
add a settings page that follows the same layout as the profile page
```

**Dichiarate l'obiettivo misurabile.** Quando l'obiettivo è performance o copertura, date la metrica e la soglia in modo che il completamento sia inequivocabile.

```text theme={null}
get the bundle size under 200KB and show me what you removed
```

**Dategli l'artefatto.** Incollate errori, log, screenshot e output del piano direttamente nel prompt, o digitate `@` per fare riferimento a un file. Claude legge la fonte invece della vostra descrizione di essa.

```text theme={null}
why is the build failing? @build.log
```

**Dite come volete la risposta.** Nominate il formato, la lunghezza o il pubblico in modo che la spiegazione si adatti a come la userete. Per rendere un formato il default per ogni risposta, impostate uno [stile di output](/docs/it/output-styles).

```text theme={null}
explain how the payment retry logic works as an HTML page with a diagram, then open it in my browser
```

Per ulteriori informazioni su ogni modello, vedete [best practice](/docs/it/best-practices).

<h2 id="where-these-come-from">
  Da dove vengono questi
</h2>

Questi prompt si basano su modelli da risorse Anthropic pubblicate. Ogni card collega alla sua fonte:

* [Flussi di lavoro comuni](/docs/it/common-workflows): guide passo dopo passo per i compiti principali
* [Best practice](/docs/it/best-practices): modelli di prompting e configurazione del progetto
* [Come i team di Anthropic utilizzano Claude Code](https://claude.com/blog/how-anthropic-teams-use-claude-code): flussi di lavoro reali da team di ingegneria, prodotto, design e dati, con approfondimenti su [legal](https://claude.com/blog/how-anthropic-uses-claude-legal), [marketing](https://claude.com/blog/how-anthropic-uses-claude-marketing) e [cybersecurity](https://claude.com/blog/how-anthropic-uses-claude-cybersecurity)
* [Guida al scaling del coding agentico](https://resources.anthropic.com/hubfs/Scaling%20agentic%20coding%20across%20your%20organization.pdf): la guida all'adozione aziendale

Per video tutorial di questi modelli, vedete il corso gratuito [Claude Code in Action](https://anthropic.skilljar.com/claude-code-in-action) su Anthropic Academy.

<h2 id="related-resources">
  Risorse correlate
</h2>

I prompt su questa pagina sono punti di partenza. Una volta che uno funziona per il vostro progetto, il passo successivo è renderlo ripetibile: salvatelo come una [skill](/docs/it/skills) in modo che chiunque nel vostro team possa eseguirlo come un `/command`, e registrate le convenzioni che Claude ha imparato in [CLAUDE.md](/docs/it/memory) in modo che ogni sessione inizi con quel contesto invece che Claude lo riapprenda. Per cambiamenti più grandi o più rischiosi, [plan mode](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode) vi mostra l'elenco dei file prima che qualsiasi modifica accada.

Se state introducendo Claude Code in un team, vedete [administration](/docs/it/admin-setup) per le impostazioni gestite e la politica, e [costs and usage](/docs/it/costs) per come questo lavoro è fatturato nel vostro piano.
