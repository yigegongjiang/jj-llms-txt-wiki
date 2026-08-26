> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Prompt-Bibliothek

> Kopieren Sie Prompts für Claude Code, kategorisiert nach Aufgabe und Rolle.

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

Dies ist eine Bibliothek von Prompts zum Kopieren in Claude Code. Nutzen Sie sie, um neue Arbeitsweisen zu erkunden, die Sie noch nicht ausprobiert haben, oder wenn Sie nicht wissen, wo Sie anfangen sollen.

Die Prompts stammen aus verschiedenen Anthropic-Leitfäden, darunter [Häufige Arbeitsabläufe](/docs/de/common-workflows), [Best Practices](/docs/de/best-practices) und [Wie Anthropic-Teams Claude Code nutzen](https://claude.com/blog/how-anthropic-teams-use-claude-code). Sie sind Ausgangspunkte und keine Skripte. Öffnen Sie **Warum das funktioniert** unter einem beliebigen Prompt, um das Muster dahinter zu sehen, damit Sie Ihre eigenen Prompts schreiben können.

export const labels = {
  startHere: "Hier beginnen",
  startHereHeader: "Fünf Prompts zum Ausprobieren",
  showAll: "Alle {n} Prompts anzeigen",
  search: "Prompts durchsuchen…",
  clear: "Löschen",
  prompt: "Prompt",
  prompts: "Prompts",
  noMatch: "Keine Prompts gefunden",
  fillAndCopy: "Ausfüllen und kopieren",
  copyThis: "Diesen Prompt kopieren",
  hintBefore: "Geben Sie in die",
  hintChip: "hervorgehobenen",
  hintAfter: "Felder ein, um anzupassen, und kopieren Sie dann.",
  copy: "Kopieren",
  copied: "Kopiert",
  whyWorks: "Warum das funktioniert",
  makeItStick: "Zum Merken",
  from: "Von",
  paste: {
    mockup: "Fügen Sie Ihr Mockup-Bild ein, ziehen Sie es herein oder erwähnen Sie es mit @, und senden Sie dann folgendes:",
    design: "Fügen Sie Ihr Design-Bild ein, ziehen Sie es herein oder erwähnen Sie es mit @, und senden Sie dann folgendes:",
    screenshot: "Fügen Sie Ihren Screenshot ein, ziehen Sie ihn herein oder erwähnen Sie ihn mit @, und senden Sie dann folgendes:",
    plan: "Fügen Sie Ihre Plan-Ausgabe zuerst in den Prompt ein, und senden Sie dann folgendes:",
    error: "Fügen Sie die Fehlerausgabe zuerst in den Prompt ein, und senden Sie dann folgendes:",
    csv: "Ziehen Sie Ihre Datei in den Prompt, oder ersetzen Sie den Pfad unten durch eine @-Erwähnung Ihrer eigenen:"
  },
  needsLabel: "Benötigt",
  needs: {
    tracker: "Ihren Issue-Tracker, der als [claude.ai-Connector](/docs/de/mcp#use-mcp-servers-from-claude-ai) oder [MCP-Server](/docs/de/mcp) hinzugefügt wurde.",
    gh: "die authentifizierte [gh CLI](https://cli.github.com) oder GitHub, das als [claude.ai-Connector](/docs/de/mcp#use-mcp-servers-from-claude-ai) hinzugefügt wurde.",
    browser: "eine Möglichkeit für Claude, das Ergebnis zu rendern und einen Screenshot zu erstellen. Die [Desktop-App](/docs/de/desktop#preview-your-app) hat dies bereits integriert. Im Terminal installieren Sie die [Chrome-Erweiterung](/docs/de/chrome) oder einen Playwright-[MCP](/docs/de/mcp)-Server.",
    db: "Ihr Data Warehouse oder Log Store, das als [claude.ai-Connector](/docs/de/mcp#use-mcp-servers-from-claude-ai) oder [MCP-Server](/docs/de/mcp) hinzugefügt wurde."
  }
};

export const tagLabels = {
  understand: "Verstehen",
  plan: "Planen",
  prototype: "Prototyp",
  build: "Erstellen",
  test: "Testen",
  refactor: "Umgestalten",
  review: "Überprüfen",
  steer: "Steuern",
  debug: "Debuggen",
  git: "Git",
  release: "Veröffentlichung",
  data: "Daten",
  automate: "Automatisieren",
  pm: "Produkt",
  design: "Design",
  docs: "Dokumentation",
  marketing: "Marketing",
  security: "Sicherheit",
  ops: "Bereitschaft"
};

export const phaseLabels = {
  discover: "Entdecken",
  design: "Design",
  build: "Erstellen",
  ship: "Versand",
  operate: "Betrieb"
};

export const sourceLabels = {
  workflows: "Häufige Arbeitsabläufe",
  teams: "Wie Anthropic-Teams Claude Code nutzen",
  legal: "Wie Anthropic Claude in der Rechtsabteilung nutzt",
  cybersecurity: "Wie Anthropic Claude in der Cybersicherheit nutzt",
  "best-practices": "Best Practices",
  ebook: "Leitfaden zum Skalieren von agentengestütztem Coding"
};

export const catLabels = {
  Onboard: "Onboarding",
  Understand: "Verstehen",
  Plan: "Planen",
  Prototype: "Prototyp",
  Implement: "Implementieren",
  Test: "Testen",
  Refactor: "Umgestalten",
  Review: "Überprüfen",
  Steer: "Steuern",
  Git: "Git",
  Release: "Veröffentlichung",
  Debug: "Debuggen",
  Incident: "Vorfall",
  Data: "Daten",
  Automate: "Automatisieren"
};

export const text = {
  "get-oriented-in-a": {
    title: "Sich in einem neuen Repository orientieren",
    teaches: "Beschreiben Sie, was Sie wissen möchten, nicht welche Dateien Sie lesen sollen. Claude erkundet das Projekt selbst und gibt eine Zusammenfassung zurück, wie es zusammenhängt.",
    next: "Führen Sie `/init` aus, um `CLAUDE.md` einzurichten, damit Claude sich daran erinnert"
  },
  "explain-unfamiliar-code": {
    title: "Unbekannten Code erklären",
    teaches: "Nennen Sie die Datei und sagen Sie, in welchem Format Sie die Antwort haben möchten. Ersetzen Sie die HTML-Seite durch ein Diagramm, Aufzählungspunkte oder was auch immer zu Ihrer Lernweise passt.",
    next: "Legen Sie einen Ausgabestil fest, damit Claude immer in Ihrem bevorzugten Format erklärt"
  },
  "find-where-something-happens": {
    title: "Finden Sie, wo etwas passiert",
    teaches: "Suchen Sie nach Verhalten statt nach Dateinamen. Die Suche funktioniert auch, wenn Sie nicht wissen, wie die Datei heißt oder in welchem Verzeichnis sie sich befindet."
  },
  "see-what-depends-on": {
    title: "Überprüfen Sie, was bricht, bevor Sie löschen",
    teaches: "Fragen Sie, bevor Sie etwas entfernen. Die Liste der Aufrufer und nachgelagerten Auswirkungen zeigt Ihnen, ob Sie eine einzeilige Bereinigung oder eine Änderung vor sich haben, die Sie koordinieren müssen."
  },
  "trace-how-code-evolved": {
    title: "Verfolgen Sie, wie sich Code entwickelt hat",
    teaches: "Verweisen Sie auf die Commit-Historie, wenn die Frage das Warum ist, nicht das Was. Claude liest das Protokoll und die Blame für jede Versionskontrolle, die Sie verwenden, und erklärt die Entscheidungen hinter der aktuellen Implementierung."
  },
  "scope-a-change-before": {
    title: "Bestimmen Sie den Umfang einer Änderung, bevor Sie beginnen",
    teaches: "Schätzen Sie die Arbeit, bevor Sie sie in eine Roadmap aufnehmen. Die Dateiliste zeigt Ihnen, ob Sie ein einzelnes Komponente oder eine übergreifende Änderung vor sich haben."
  },
  "ask-the-codebase-a": {
    title: "Stellen Sie der Codebasis eine Produktfrage",
    teaches: "Geben Sie Ihre Rolle an, damit die Antwort auf der richtigen Ebene liegt. Claude erklärt, was das Produkt tatsächlich aus dem Quellcode tut, ohne dass Sie ihn lesen müssen.",
    next: "Legen Sie einen Ausgabestil fest, damit Claude Antworten immer auf dieser Ebene präsentiert"
  },
  "plan-a-multi-file": {
    title: "Planen Sie eine Änderung mit mehreren Dateien, bevor Sie Code anfassen",
    teaches: "Das Hinzufügen von \"noch nicht bearbeiten\" trennt Erkundung von Änderungen, sodass Sie den Ansatz sehen, bevor sich Code bewegt. Um Plan-First zur Standardeinstellung für jeden Prompt zu machen, drücken Sie Shift+Tab für [Plan-Modus](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode)."
  },
  "draft-a-spec-by": {
    title: "Entwerfen Sie eine Spezifikation durch ein Interview",
    teaches: "Bitten Sie darum, interviewt zu werden, anstatt die Spezifikation selbst zu schreiben. Claude stellt Ihnen strukturierte Fragen, bis die Anforderungen vollständig sind, und schreibt das Ergebnis dann in eine Datei.",
    next: "Speichern Sie Ihre Interviewfragen als `/spec`-Skill, damit jede Spezifikation auf die gleiche Weise beginnt"
  },
  "turn-a-meeting-into": {
    title: "Verwandeln Sie ein Meeting in Tickets",
    teaches: "Überspringen Sie den Transkriptionsschritt. Claude extrahiert Aktionselemente aus der unstrukturierten Eingabe und schreibt sie direkt über [MCP](/docs/de/mcp) in Ihren Tracker, sodass Sie die Tickets überprüfen, nicht das Transkript.",
    next: "Speichern Sie dies als `/tickets`-Skill"
  },
  "map-edge-cases-before": {
    title: "Kartografieren Sie Grenzfälle vor dem Erstellen",
    teaches: "Fragen Sie, was fehlt, nicht was vorhanden ist. Claude listet die Fehlerzustände, leeren Zustände und Grenzfälle auf, die ein Happy-Path-Design normalerweise überspringt."
  },
  "turn-a-mockup-into": {
    title: "Verwandeln Sie ein Mockup in einen funktionierenden Prototyp",
    teaches: "Ein anklickbarer Prototyp beantwortet Fragen, die ein statisches Mockup nicht kann. Geben Sie den funktionierenden Code an die Entwicklung statt, die Interaktionen in einem Dokument zu erklären."
  },
  "implement-from-a-screenshot": {
    title: "Implementieren Sie aus einem Screenshot und überprüfen Sie selbst",
    teaches: "Dies gibt Claude eine Überprüfungsschleife: Es rendert, vergleicht mit dem Quellbild und iteriert, ohne dass Sie jede Lücke aufzeigen.",
    next: "Verwenden Sie `/goal`, um Claude zum Iterieren zu bringen, bis die Screenshots übereinstimmen"
  },
  "follow-an-existing-pattern": {
    title: "Folgen Sie einem vorhandenen Muster",
    teaches: "Verweisen Sie auf Code, den Sie bereits mögen. Ohne Referenz verwendet Claude allgemeine Best Practices. Mit einer Referenz entspricht es den Konventionen, die Ihre Codebasis tatsächlich verwendet.",
    next: "Bitten Sie Claude, das Muster, das es befolgt hat, in `CLAUDE.md` zu schreiben, damit zukünftige Sitzungen es ohne die Referenz abgleichen"
  },
  "add-a-small-well": {
    title: "Fügen Sie eine kleine, gut definierte Funktion hinzu",
    teaches: "Geben Sie die Ein- und Ausgaben an, nicht wie Sie sie erstellen. Claude findet, wo ähnlicher Code lebt, und fügt Ihren daneben hinzu."
  },
  "build-a-small-internal": {
    title: "Erstellen Sie ein kleines internes Tool von Grund auf",
    teaches: "Sie benötigen kein Projekt, kein Framework und keinen Build-Schritt. Beschreiben Sie das Tool und bitten Sie Claude, es zu öffnen, damit Sie es sofort funktionieren sehen."
  },
  "work-an-issue-end": {
    title: "Arbeiten Sie ein Problem von Anfang bis Ende",
    teaches: "Geben Sie die Ticketnummer an, nicht eine Zusammenfassung. Claude liest das vollständige Ticket selbst, sodass Anforderungen, die Sie vergessen würden zu erwähnen, durchkommen, und es validiert die Änderung, bevor es Bericht erstattet."
  },
  "find-and-update-copy": {
    title: "Finden und aktualisieren Sie Text in der gesamten Codebasis",
    teaches: "Fragen Sie nach Varianten und sagen Sie, was Sie überspringen sollen. Claude findet Formulierungen, die eine wörtliche Suche vermissen würde, und lässt Test-Fixtures und Verlauf unberührt, sodass Sie nur den Text überprüfen, den Benutzer tatsächlich sehen."
  },
  "draft-from-past-examples": {
    title: "Entwerfen Sie ein Dokument aus früheren Beispielen",
    teaches: "Verweisen Sie auf einen Ordner mit abgeschlossener Arbeit, anstatt Ihren Stil zu beschreiben. Claude lernt die Struktur und Stimme aus dem, was Sie bereits versendet haben, sodass der erste Entwurf wie einer von Ihnen klingt.",
    next: "Speichern Sie die Stimme als Skill, damit jeder Entwurf dort beginnt"
  },
  "write-tests-run-them": {
    title: "Schreiben Sie Tests, führen Sie sie aus, beheben Sie Fehler",
    teaches: "Bitten Sie darum, zu schreiben, auszuführen und zu beheben, damit Claude iteriert, ohne auf Anweisungen zu warten.",
    next: "Führen Sie `/init` aus, damit Claude Ihren Test-Befehl automatisch lernt"
  },
  "drive-implementation-from-tests": {
    title: "Treiben Sie die Implementierung von Tests an",
    teaches: "Test-getriebene Entwicklung: Die Tests definieren, wann die Arbeit abgeschlossen ist, und Claude iteriert die Implementierung, bis sie bestanden werden."
  },
  "fill-gaps-from-a": {
    title: "Füllen Sie Lücken aus einem Abdeckungsbericht",
    teaches: "Verweisen Sie auf den Abdeckungsbericht, anstatt zu raten, was nicht getestet wird. Claude liest die tatsächlichen Zahlen und schreibt Tests für die Dateien, die sie am meisten benötigen.",
    next: "Legen Sie dies als `/goal` fest, damit Claude weiterhin Tests schreibt, bis die Abdeckung das Ziel erreicht"
  },
  "port-code-between-languages": {
    title: "Portieren Sie Code in eine andere Sprache",
    teaches: "Sagen Sie, was Sie beibehalten möchten, nicht nur die Zielsprache. Das Benennen der API oder des Verhaltens, das gleich bleiben muss, gibt Claude einen Vertrag, um den Port dagegen zu überprüfen."
  },
  "generate-docs-for-code": {
    title: "Generieren Sie Dokumentation für undokumentierten Code",
    teaches: "Nennen Sie den Umfang und das Format. Claude findet, was fehlt, und entspricht dem Kommentarstil, der bereits in der Datei vorhanden ist, sodass die neue Dokumentation wie der Rest klingt."
  },
  "migrate-a-pattern-across": {
    title: "Migrieren Sie ein Muster in der gesamten Codebasis",
    teaches: "Beschreiben Sie das alte Muster und das neue. Claude zuerst aufzufordern, jeden Ort zu identifizieren, bedeutet, dass die Aufrufstellen in der Antwort aufgelistet sind, sodass Sie überprüfen können, dass keine übersehen wurden."
  },
  "optimize-against-a-measurable": {
    title: "Optimieren Sie gegen ein messbares Ziel",
    teaches: "Das Angeben der Metrik und des Ziels gibt Claude eine klare Definition von Fertig.",
    next: "Legen Sie dies als `/goal` fest, damit Claude weiterhin misst und iteriert, bis es die Zahl erreicht"
  },
  "fix-a-precise-visual": {
    title: "Beheben Sie einen präzisen visuellen Fehler",
    teaches: "Präzises visuelles Feedback erhält eine präzise Reparatur. Geben Sie das genaue Element, die Messung und den Viewport an.",
    next: "Fügen Sie ein Vorschau-Tool hinzu, damit Claude den Fix selbst fotografiert und überprüft"
  },
  "review-your-changes-before": {
    title: "Überprüfen Sie Ihre Änderungen, bevor Sie committen",
    teaches: "Fangen Sie Probleme ab, während sie noch billig zu beheben sind. Claude liest die geänderten Dateien vollständig, nicht nur die Diff-Zeilen, sodass es Probleme erkennt, die eine schnelle Selbstüberprüfung vermisst.",
    next: "Führen Sie `/code-review` aus, um die gleiche Überprüfung in einem Befehl durchzuführen"
  },
  "review-a-pull-request": {
    title: "Überprüfen Sie einen Pull Request",
    teaches: "Claude überprüft mit der gesamten Codebasis im Kontext, nicht nur dem Diff. Es liest den geänderten Code und das, was er aufruft, sodass es Probleme erkennt, die eine Diff-only-Überprüfung vermissen würde.",
    next: "Schalten Sie dies für jeden PR mit Code Review ein"
  },
  "review-infrastructure-changes-before": {
    title: "Überprüfen Sie Infrastrukturänderungen vor dem Anwenden",
    teaches: "Plan-Ausgabe ist dicht und schwer zu scannen. Das Einfügen erhalten Sie eine Klartext-Zusammenfassung dessen, was tatsächlich geändert wird, bevor Sie es anwenden."
  },
  "run-a-security-review": {
    title: "Führen Sie eine Sicherheitsüberprüfung mit einem Subagenten durch",
    teaches: "Ein [Subagent](/docs/de/sub-agents) führt die Prüfung in seinem eigenen Kontextfenster durch und meldet eine Zusammenfassung zurück, sodass eine lange Sicherheitsüberprüfung Ihre Hauptsitzung nicht ausfüllt. Der integrierte allgemeine Subagent handhabt dies ohne zusätzliche Einrichtung.",
    next: "Richten Sie einen dedizierten Sicherheitsüberprüfungs-Subagenten ein, den Ihr ganzes Team verwenden kann"
  },
  "review-content-before-sending": {
    title: "Fangen Sie Probleme vor der formalen Überprüfung ab",
    teaches: "Erhalten Sie einen ersten Durchgang, bevor ein Mensch Zeit damit verbringt. Nennen Sie die Bedenken, die Sie überprüft haben möchten, damit die Überprüfung fokussiert ist, und beheben Sie dann, was sie findet, und senden Sie einen saubereren Entwurf.",
    next: "Erfassen Sie Ihre Überprüfungs-Checkliste als Skill, den Ihr ganzes Team ausführen kann"
  },
  "course-correct-a-wrong": {
    title: "Korrigieren Sie einen falschen Ansatz",
    teaches: "Nennen Sie die Einschränkung, die Claude verpasst hat, nicht nur dass es falsch ist. Ein spezifischer Grund gibt Claude eine konkrete Einschränkung, um beim erneuten Versuch zu erfüllen, anstatt erneut zu raten.",
    next: "Drücken Sie `Esc` zweimal, um das Rewind-Menü zu öffnen und Code und Konversation wiederherzustellen, damit der erneute Versuch sauber beginnt"
  },
  "narrow-the-scope-of": {
    title: "Verengen Sie den Umfang einer Änderung",
    teaches: "Wenn die Richtung richtig ist, aber die Änderung zu breit wurde, bitten Sie Claude, einen Teil davon zu behalten, anstatt alles zurückzuspulen. Eine angegebene Grenze hält eine kleine Reparatur davon ab, eine Umgestaltung zu werden."
  },
  "turn-a-correction-into": {
    title: "Verwandeln Sie eine Korrektur in eine Regel",
    teaches: "Eine Korrektur im Chat wird nicht mit Ihrem Team geteilt. Eine Regel im [CLAUDE.md](/docs/de/memory) des Projekts wird geteilt, sobald Sie sie committen, und Claude liest sie am Anfang jeder Sitzung.",
    next: "Öffnen Sie `/memory`, um zu überprüfen, was Claude geschrieben hat"
  },
  "resolve-merge-conflicts": {
    title: "Lösen Sie Merge-Konflikte",
    teaches: "Sagen Sie, welchen Zustand Sie möchten, nicht welche Marker Sie behalten sollen. Das Fragen nach der Begründung macht den Merge überprüfbar, anstatt eine Black Box zu sein."
  },
  "commit-with-a-generated": {
    title: "Committen Sie mit einer generierten Nachricht",
    teaches: "Lassen Sie Claude die Nachricht aus dem Diff ableiten. Sie entspricht dem vorhandenen Commit-Stil Ihres Repositorys."
  },
  "open-a-pull-request": {
    title: "Öffnen Sie einen Pull Request aus einem Ticket",
    teaches: "Überspringen Sie den Kontextwechsel zwischen Tracker, Editor und GitHub. Ein Prompt liest die Spezifikation, nimmt die Änderung vor und öffnet den PR."
  },
  "draft-release-notes-from": {
    title: "Entwerfen Sie Versionshinweise aus der Git-Historie",
    teaches: "Geben Sie zwei Referenzpunkte und die gewünschte Struktur an. Claude liest das Commit-Protokoll zwischen ihnen und entwirft ein Changelog, das Sie bearbeiten können.",
    next: "Speichern Sie dies als `/changelog`-Skill"
  },
  "write-a-ci-workflow": {
    title: "Schreiben Sie einen CI-Workflow",
    teaches: "Beschreiben Sie, wann er ausgeführt werden soll und was er tun soll; die YAML wird für Sie generiert, abgestimmt auf die Build- und Test-Befehle Ihres Projekts."
  },
  "find-and-fix-a": {
    title: "Finden und beheben Sie einen fehlgeschlagenen Test",
    teaches: "Beschreiben Sie das Symptom; Sie müssen nicht wissen, welche Datei kaputt ist. Claude führt den Test aus, um den Fehler zu sehen, verfolgt ihn in die Quelle und behebt ihn."
  },
  "investigate-a-reported-error": {
    title: "Untersuchen Sie einen gemeldeten Fehler",
    teaches: "Beschreiben Sie das Symptom und den Ort; Claude liest den relevanten Codepfad und verfolgt wahrscheinliche Ursachen. Fügen Sie Stack Traces oder Protokolle ein, wenn Sie diese haben.",
    next: "Fügen Sie einen Deeplink in Ihrem Runbook ein, der Claude mit diesem Prompt vorab ausgefüllt öffnet"
  },
  "fix-a-build-error": {
    title: "Beheben Sie einen Build-Fehler an der Wurzel",
    teaches: "Das Fragen nach Grundursache und Überprüfung verhindert oberflächliche Patches, die den Fehler unterdrücken, ohne ihn zu beheben."
  },
  "investigate-a-production-incident": {
    title: "Untersuchen Sie einen Produktionsvorfall",
    teaches: "Listen Sie die Evidenzquellen auf, um zu korrelieren, nicht die Schritte, die Sie unternehmen sollen. Claude liest Protokolle, Git-Historie und Konfiguration zusammen, um die Ursache einzugrenzen.",
    next: "Verbinden Sie Sentry oder Ihren Log Store über MCP"
  },
  "query-logs-in-plain": {
    title: "Fragen Sie Protokolle in einfachem Englisch ab",
    teaches: "Stellen Sie die Frage, anstatt das SQL zu schreiben. Claude erstellt die Abfrage, führt sie gegen Ihre verbundenen Protokolle aus und zeigt sowohl die Abfrage als auch das Ergebnis, sodass Sie überprüfen können, was ausgeführt wurde."
  },
  "diagnose-from-a-console": {
    title: "Diagnose aus einem Konsolen-Screenshot",
    teaches: "Cloud-Konsolen zeigen Ihnen das Problem, aber nicht die Befehle, um es zu beheben. Claude liest den Screenshot und übersetzt das Dashboard in die kubectl-, gcloud- oder aws-Befehle, die ausgeführt werden sollen."
  },
  "analyze-a-data-file": {
    title: "Analysieren Sie eine Datendatei",
    teaches: "Eine einmalige Frage benötigt kein einmaliges Skript. Verweisen Sie auf eine Datei in Ihrem Projektordner und Claude liest sie direkt, findet die Muster und schreibt die Ausgabe, wo Sie fragen.",
    next: "Verbinden Sie die Datenquelle über MCP, anstatt Dateien zu exportieren"
  },
  "generate-variations-from-performance": {
    title: "Generieren Sie Variationen aus Leistungsdaten",
    teaches: "Geben Sie die Einschränkung am Anfang an, damit die Generierung innerhalb der Grenze bleibt. Claude liest die Metriken, wählt aus, was ersetzt werden soll, und produziert Alternativen, die passen.",
    next: "Verbinden Sie die Anzeigenplattform über MCP, anstatt eine Datei zu exportieren"
  },
  "turn-a-recurring-task": {
    title: "Verwandeln Sie eine wiederkehrende Aufgabe in einen Skill",
    teaches: "Nennen Sie die Schritte einmal; verwenden Sie sie als Befehl erneut. Claude schreibt einen [Skill](/docs/de/skills), den jeder in Ihrem Team ausführen kann."
  },
  "add-a-hook-for": {
    title: "Fügen Sie einen Hook für wiederholtes Verhalten hinzu",
    teaches: "Hooks machen ein Verhalten automatisch, anstatt etwas, das Sie sich merken müssen, um zu fragen. Beschreiben Sie den Trigger und die Aktion und Claude schreibt die [Hook](/docs/de/hooks)-Konfiguration."
  },
  "connect-a-tool-with": {
    title: "Verbinden Sie ein Tool mit MCP",
    teaches: "Verbinden Sie die Quelle einmal, anstatt Daten jede Sitzung einzufügen. Nach [MCP](/docs/de/mcp)-Einrichtung liest Claude direkt aus dem Tool, wenn Sie danach fragen."
  },
  "capture-what-to-remember": {
    title: "Erfassen Sie, was Sie sich für das nächste Mal merken möchten",
    teaches: "Fragen Sie, bevor Sie vergessen. Claude weiß, was es diese Sitzung herausfinden musste, und schlägt [CLAUDE.md](/docs/de/memory)-Einträge vor, damit die nächste Sitzung mit diesem Kontext beginnt."
  }
};

<PromptLibrary text={text} labels={labels} tagLabels={tagLabels} phaseLabels={phaseLabels} sourceLabels={sourceLabels} catLabels={catLabels} />

<h2 id="what-makes-these-prompts-work">
  Was macht diese Prompts funktionieren
</h2>

Die obigen Prompts teilen einige Muster. Sie zu erkennen hilft Ihnen, jeden Prompt hier an Ihre eigene Aufgabe anzupassen.

**Beschreiben Sie das Ergebnis, nicht die Schritte.** Sagen Sie, was Sie möchten, und lassen Sie Claude die Dateien finden. Der folgende Prompt funktioniert, ohne einen einzigen Dateipfad zu nennen.

```text theme={null}
add rate limiting to the public API and make sure existing tests still pass
```

**Geben Sie ihm eine Möglichkeit, seine eigene Arbeit zu überprüfen.** Bitten Sie darum, auszuführen, zu testen, zu vergleichen oder zu überprüfen, im gleichen Prompt, damit Claude iteriert, anstatt nach einem Versuch zu stoppen.

```text theme={null}
write the migration, run it against the dev database, and confirm the schema matches
```

**Verweisen Sie auf eine Referenz.** Nennen Sie eine vorhandene Datei, einen Test oder ein Muster, um zu entsprechen, sodass der neue Code konsistent mit dem ist, was Sie bereits haben.

```text theme={null}
add a settings page that follows the same layout as the profile page
```

**Geben Sie das messbare Ziel an.** Wenn das Ziel Leistung oder Abdeckung ist, geben Sie die Metrik und den Schwellenwert an, damit die Fertigstellung eindeutig ist.

```text theme={null}
get the bundle size under 200KB and show me what you removed
```

**Geben Sie das Artefakt an.** Fügen Sie Fehler, Protokolle, Screenshots und Plan-Ausgabe direkt in den Prompt ein, oder geben Sie `@` ein, um auf eine Datei zu verweisen. Claude liest die Quelle, anstatt Ihre Beschreibung davon.

```text theme={null}
why is the build failing? @build.log
```

**Sagen Sie, wie Sie die Antwort möchten.** Nennen Sie das Format, die Länge oder das Publikum, damit die Erklärung passt, wie Sie sie verwenden werden. Um ein Format zur Standardeinstellung für jede Antwort zu machen, legen Sie einen [Ausgabestil](/docs/de/output-styles) fest.

```text theme={null}
explain how the payment retry logic works as an HTML page with a diagram, then open it in my browser
```

Weitere Informationen zu jedem Muster finden Sie unter [Best Practices](/docs/de/best-practices).

<h2 id="where-these-come-from">
  Woher diese kommen
</h2>

Diese Prompts basieren auf Mustern aus veröffentlichten Anthropic-Ressourcen. Jede Karte verlinkt auf ihre Quelle:

* [Häufige Arbeitsabläufe](/docs/de/common-workflows): Schritt-für-Schritt-Leitfäden für die Kernaufgaben
* [Best Practices](/docs/de/best-practices): Prompting-Muster und Projekteinrichtung
* [Wie Anthropic-Teams Claude Code nutzen](https://claude.com/blog/how-anthropic-teams-use-claude-code): Echte Arbeitsabläufe aus Engineering-, Produkt-, Design- und Daten-Teams mit tiefgreifenden Einblicken in [Recht](https://claude.com/blog/how-anthropic-uses-claude-legal), [Marketing](https://claude.com/blog/how-anthropic-uses-claude-marketing) und [Cybersicherheit](https://claude.com/blog/how-anthropic-uses-claude-cybersecurity)
* [Leitfaden zum Skalieren von agentengestütztem Coding](https://resources.anthropic.com/hubfs/Scaling%20agentic%20coding%20across%20your%20organization.pdf): Der Enterprise-Adoptionsleitfaden

Für Video-Walkthroughs dieser Muster siehe den kostenlosen [Claude Code in Action](https://anthropic.skilljar.com/claude-code-in-action)-Kurs auf der Anthropic Academy.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

Die Prompts auf dieser Seite sind Ausgangspunkte. Sobald einer für Ihr Projekt funktioniert, besteht der nächste Schritt darin, ihn wiederholbar zu machen: Speichern Sie ihn als [Skill](/docs/de/skills), damit jeder in Ihrem Team ihn als `/Befehl` ausführen kann, und notieren Sie die Konventionen, die Claude in [CLAUDE.md](/docs/de/memory) gelernt hat, damit jede Sitzung mit diesem Kontext beginnt, anstatt Claude es neu zu lernen. Für größere oder riskantere Änderungen zeigt [Plan-Modus](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode) Ihnen die Dateiliste, bevor Änderungen vorgenommen werden.

Wenn Sie Claude Code in einem Team einführen, siehe [Verwaltung](/docs/de/admin-setup) für verwaltete Einstellungen und Richtlinien sowie [Kosten und Nutzung](/docs/de/costs) für die Abrechnung dieser Arbeit in Ihrem Plan.
