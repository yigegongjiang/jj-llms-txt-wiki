> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# .claude ディレクトリを探索する

> Claude Code が CLAUDE.md、settings.json、hooks、skills、commands、subagents、workflows、rules、auto memory を読み込む場所。プロジェクト内の .claude ディレクトリとホームディレクトリの ~/.claude を探索します。

export const ClaudeExplorer = () => {
  const A = useMemo(() => ({href, children}) => <a href={href} style={{
    color: 'var(--ce-accent)',
    textDecoration: 'none',
    borderBottom: '1px dotted var(--ce-accent)'
  }}>{children}</a>, []);
  const C = useMemo(() => ({children}) => <code style={{
    fontFamily: 'var(--ce-mono)',
    fontSize: '0.92em',
    padding: '1px 4px',
    borderRadius: '3px',
    background: 'var(--ce-surface)',
    border: '0.5px solid var(--ce-border-subtle)'
  }}>{children}</code>, []);
  const commandsNote = useMemo(() => <>Commands and skills are now the same mechanism. For new workflows, use <A href="/docs/en/skills">skills/</A> instead: same <C>/name</C> invocation, plus you can bundle supporting files.</>, []);
  const FILE_TREE = useMemo(() => ({
    project: {
      label: 'your-project/',
      children: [{
        id: 'claude-md',
        label: 'CLAUDE.md',
        type: 'file',
        icon: 'md',
        color: '#6A9BCC',
        badge: 'committed',
        oneLiner: 'Project instructions Claude reads every session',
        when: 'Loaded into context at the start of every session',
        description: 'Project-specific instructions that shape how Claude works in this repository. Put your conventions, common commands, and architectural context here so Claude operates with the same assumptions your team does.',
        tips: ['Target under 200 lines. Longer files still load in full but may reduce adherence', <>CLAUDE.md loads into every session. If something only matters for specific tasks, move it to a <A href="/docs/en/skills">skill</A> or a path-scoped <A href="/docs/en/memory#organize-rules-with-claude/rules/">rule</A> so it loads only when needed</>, 'List the commands you run most, like build, test, and format, so Claude knows them without you spelling them out each time', <>Run <C>/memory</C> to open and edit CLAUDE.md from within a session</>, <>Also works at <C>.claude/CLAUDE.md</C> if you prefer to keep the project root clean</>, <>If your repo already has an <C>AGENTS.md</C> for other coding agents, Claude Code <A href="/docs/en/memory#agents-md">can read that</A> on its own or alongside CLAUDE.md</>],
        exampleIntro: 'This example is for a TypeScript and React project. It lists the build and test commands, the framework conventions Claude should follow, and project-specific rules like export style and file layout.',
        example: `# Project conventions

## Commands
- Build: \`npm run build\`
- Test: \`npm test\`
- Lint: \`npm run lint\`

## Stack
- TypeScript with strict mode
- React 19, functional components only

## Rules
- Named exports, never default exports
- Tests live next to source: \`foo.ts\` -> \`foo.test.ts\`
- All API routes return \`{ data, error }\` shape`,
        docsLink: '/en/memory'
      }, {
        id: 'mcp-json',
        label: '.mcp.json',
        type: 'file',
        icon: 'json',
        color: '#9B7BC4',
        badge: 'committed',
        oneLiner: 'Project-scoped MCP servers, shared with your team',
        when: <>Servers connect when the session begins. Tool schemas are deferred by default and load on demand via <A href="/docs/en/mcp#scale-with-mcp-tool-search">tool search</A></>,
        description: <>Configures Model Context Protocol (MCP) servers that give Claude access to external tools: databases, APIs, browsers, and more. This file holds the project-scoped servers your whole team uses. Personal servers you want to keep to yourself go in <C>~/.claude.json</C> instead.</>,
        tips: [<>Use environment variable references for secrets: <C>{'${NOTION_TOKEN}'}</C></>, <>Lives at the project root, not inside <C>.claude/</C></>, <>For servers only you need, run <C>claude mcp add --scope user</C>. This writes to <C>~/.claude.json</C> instead of <C>.mcp.json</C></>],
        exampleIntro: <>This example configures the Notion MCP server so Claude can read and update pages in your workspace. The <C>{'${NOTION_TOKEN}'}</C> reference is read from your shell environment when Claude Code starts the server, so the token never lands in the file.</>,
        example: `{
  "mcpServers": {
    "notion": {
      "command": "npx",
      "args": ["-y", "@notionhq/notion-mcp-server"],
      "env": {
        "NOTION_TOKEN": "\${NOTION_TOKEN}"
      }
    }
  }
}`,
        docsLink: '/en/mcp'
      }, {
        id: 'worktreeinclude',
        label: '.worktreeinclude',
        type: 'file',
        icon: 'md',
        color: '#8FA876',
        badge: 'committed',
        oneLiner: 'Gitignored files to copy into new worktrees',
        when: <>Read when Claude creates a git worktree via <C>--worktree</C>, the <C>EnterWorktree</C> tool, or subagent <C>isolation: worktree</C></>,
        description: <>Lists gitignored files to copy from your main repository into each new worktree. Worktrees are fresh checkouts, so untracked files like <C>.env</C> are missing by default. Patterns here use <C>.gitignore</C> syntax. Only files that match a pattern and are also gitignored get copied, so tracked files are never duplicated.</>,
        tips: [<>Lives at the project root, not inside <C>.claude/</C></>, <>Git-only: if you configure a <A href="/docs/en/hooks#worktreecreate">WorktreeCreate hook</A> for a different VCS, this file is not read. Copy files inside your hook script instead</>, <>Also applies to parallel sessions in the <A href="/docs/en/desktop#work-in-parallel-with-sessions">desktop app</A></>],
        exampleIntro: 'This example copies your local environment files and a secrets config into every worktree Claude creates. Comments start with # and blank lines are ignored, same as .gitignore.',
        example: `# Local environment
.env
.env.local

# API credentials
config/secrets.json`,
        docsLink: '/en/worktrees#copy-gitignored-files-into-worktrees'
      }, {
        id: 'dot-claude',
        label: '.claude/',
        type: 'folder',
        icon: 'folder',
        color: 'var(--ce-accent)',
        oneLiner: 'Project-level configuration, rules, and extensions',
        description: 'Everything Claude Code reads that is specific to this project. If you use git, commit most files here so your team shares them; a few, like settings.local.json, are gitignored when Claude Code saves settings to them. Each file badge shows which.',
        children: [{
          id: 'settings-json',
          label: 'settings.json',
          type: 'file',
          icon: 'json',
          color: 'var(--ce-text-3)',
          badge: 'committed',
          oneLiner: 'Permissions, hooks, and configuration',
          when: <>Overrides global <C>~/.claude/settings.json</C>. Local settings, CLI flags, and managed settings override this</>,
          description: 'Settings that Claude Code applies directly. Permissions control which commands and tools Claude can use; hooks run your scripts at specific points in a session. Unlike CLAUDE.md, which Claude reads as guidance, these are enforced whether Claude follows them or not.',
          contains: [<><A href="/docs/en/permissions">permissions</A>: allow, deny, or prompt before Claude uses specific tools or commands</>, <><A href="/docs/en/hooks">hooks</A>: run your own scripts on events like before a tool call or after a file edit</>, <><A href="/docs/en/statusline">statusLine</A>: customize the line shown at the bottom while Claude works</>, <><A href="/docs/en/settings-reference#available-settings">model</A>: pick a default model for this project</>, <><A href="/docs/en/settings-reference#environment-variables">env</A>: environment variables set in every session</>, <><A href="/docs/en/output-styles">outputStyle</A>: select a custom output style from output-styles/</>],
          tips: [<>Bash permission patterns support wildcards: <C>Bash(npm test *)</C> matches any command starting with <C>npm test</C></>, <>Array settings like <C>permissions.allow</C> combine across all scopes; scalar settings like <C>model</C> use the most specific value</>],
          exampleIntro: <>This example allows <C>npm test</C> and <C>npm run</C> commands without prompting, blocks <C>rm -rf</C>, and runs Prettier on files after Claude edits or writes them.</>,
          example: `{
  "permissions": {
    "allow": [
      "Bash(npm test *)",
      "Bash(npm run *)"
    ],
    "deny": [
      "Bash(rm -rf *)"
    ]
  },
  "hooks": {
    "PostToolUse": [{
      "matcher": "Edit|Write",
      "hooks": [{
        "type": "command",
        "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write"
      }]
    }]
  }
}`,
          docsLink: '/en/settings'
        }, {
          id: 'settings-local-json',
          label: 'settings.local.json',
          type: 'file',
          icon: 'json',
          color: 'var(--ce-text-3)',
          badge: 'gitignored',
          oneLiner: 'Your personal settings overrides for this project',
          when: 'Highest of the user-editable settings files; CLI flags and managed settings still take precedence',
          description: 'Personal settings that take precedence over the project defaults. Same JSON format as settings.json, gitignored when Claude Code saves a setting to it. Use this when you need different permissions or defaults than the team config.',
          tips: [<>Same schema as settings.json. Array settings like <C>permissions.allow</C> combine across scopes; scalar settings like <C>model</C> use the local value</>, <>When Claude Code saves a setting to this file in a repository that doesn't already ignore it, it adds <C>**/.claude/settings.local.json</C> to your global git excludes file: <C>core.excludesFile</C> from your global git config when it's set to an absolute or <C>~</C>-prefixed path, otherwise <C>$XDG_CONFIG_HOME/git/ignore</C>, or <C>~/.config/git/ignore</C>. To share the ignore rule with your team, also add it to the project <C>.gitignore</C></>],
          exampleIntro: 'This example adds Docker permissions on top of whatever the team settings.json allows.',
          example: `{
  "permissions": {
    "allow": [
      "Bash(docker *)"
    ]
  }
}`,
          docsLink: '/en/settings'
        }, {
          id: 'rules',
          label: 'rules/',
          type: 'folder',
          icon: 'folder',
          color: '#9B7BC4',
          oneLiner: 'Topic-scoped instructions, optionally gated by file paths',
          when: <>Rules without <C>paths:</C> load at session start. Rules with <C>paths:</C> load when a matching file enters context</>,
          description: [<>Project instructions split into topic files that can load conditionally based on file paths. A rule without <C>paths:</C> frontmatter loads at session start like CLAUDE.md; a rule with <C>paths:</C> loads only when Claude reads a matching file.</>, <>Like CLAUDE.md, rules are guidance Claude reads, not configuration Claude Code enforces. For guaranteed behavior use <A href="/docs/en/hooks">hooks</A> or <A href="/docs/en/permissions">permissions</A>.</>],
          tips: [<>Use <C>paths:</C> frontmatter with globs to scope rules to directories or file types</>, <>Subdirectories work: <C>.claude/rules/frontend/react.md</C> is discovered automatically</>, 'When CLAUDE.md approaches 200 lines, start splitting into rules'],
          docsLink: '/en/memory#organize-rules-with-claude/rules/',
          children: [{
            id: 'rule-testing',
            label: 'testing.md',
            type: 'file',
            icon: 'md',
            color: '#9B7BC4',
            badge: 'committed',
            oneLiner: 'Test conventions scoped to test files',
            when: <>Loaded when Claude reads a file matching the <C>paths:</C> globs below</>,
            description: <>An example rule that only loads when Claude is working on test files. The <C>paths:</C> globs in the frontmatter define which files trigger it; here, anything ending in .test.ts or .test.tsx. For other files, this rule is not loaded into context.</>,
            example: `---
paths:
  - "**/*.test.ts"
  - "**/*.test.tsx"
---

# Testing Rules

- Use descriptive test names: "should [expected] when [condition]"
- Mock external dependencies, not internal modules
- Clean up side effects in afterEach`
          }, {
            id: 'rule-api',
            label: 'api-design.md',
            type: 'file',
            icon: 'md',
            color: '#9B7BC4',
            badge: 'committed',
            oneLiner: 'API conventions scoped to backend code',
            when: <>Loaded when Claude reads a file matching the <C>paths:</C> glob below</>,
            description: <>A second example showing a rule scoped to backend code. The <C>paths:</C> glob matches files under src/api/, so these conventions load only when Claude is editing API routes.</>,
            example: `---
paths:
  - "src/api/**/*.ts"
---

# API Design Rules

- All endpoints must validate input with Zod schemas
- Return shape: { data: T } | { error: string }
- Rate limit all public endpoints`
          }]
        }, {
          id: 'skills',
          label: 'skills/',
          type: 'folder',
          icon: 'folder',
          color: '#D4A843',
          oneLiner: 'Reusable prompts you or Claude invoke by name',
          when: <>Invoked with <C>/skill-name</C> or when Claude matches the task to a skill</>,
          description: <>Each skill is a folder with a SKILL.md file plus any supporting files it needs. By default, both you and Claude can invoke a skill. Use frontmatter to control that: <C>disable-model-invocation: true</C> for user-only workflows like <C>/deploy</C>, or <C>user-invocable: false</C> to hide from the <C>/</C> menu while Claude can still invoke it.</>,
          tips: [<>Skills accept arguments: <C>/deploy staging</C> passes "staging" as <C>$ARGUMENTS</C>. Use <C>$0</C>, <C>$1</C>, and so on for positional access</>, <>The <C>description</C> frontmatter determines when Claude auto-invokes the skill</>, 'Bundle reference docs alongside SKILL.md. Claude knows the skill directory path and can read supporting files when you mention them'],
          docsLink: '/en/skills',
          children: [{
            id: 'skill-review',
            label: 'security-review/',
            type: 'folder',
            icon: 'folder',
            color: '#D4A843',
            oneLiner: 'A skill bundling SKILL.md with supporting files',
            children: [{
              id: 'skill-review-md',
              label: 'SKILL.md',
              type: 'file',
              icon: 'md',
              color: '#D4A843',
              badge: 'committed',
              oneLiner: 'Entrypoint: trigger, invocability, instructions',
              when: <>User types <C>/security-review &lt;target&gt;</C>; Claude cannot auto-invoke this skill</>,
              description: [<>This skill uses <C>disable-model-invocation: true</C> so only you can trigger it; Claude never invokes it on its own.</>, <>The <C>!`...`</C> line runs a shell command and injects its output into the prompt. <C>$ARGUMENTS</C> substitutes whatever you typed after the skill name. Claude sees the skill directory path, so mentioning a bundled file like checklist.md lets Claude read it.</>],
              example: `---
description: Reviews code changes for security vulnerabilities, authentication gaps, and injection risks
disable-model-invocation: true
argument-hint: <branch-or-path>
---

## Diff to review

!\`git diff $ARGUMENTS\`

Audit the changes above for:

1. Injection vulnerabilities (SQL, XSS, command)
2. Authentication and authorization gaps
3. Hardcoded secrets or credentials

Use checklist.md in this skill directory for the full review checklist.

Report findings with severity ratings and remediation steps.`
            }, {
              id: 'skill-checklist',
              label: 'checklist.md',
              type: 'file',
              icon: 'md',
              color: '#D4A843',
              badge: 'committed',
              oneLiner: 'Supporting file bundled with the skill',
              when: 'Claude reads it on demand while running the skill',
              description: <>Skills can bundle any supporting files: reference docs, templates, scripts. The skill directory path is prepended to SKILL.md, so Claude can read bundled files by name. For scripts in bash injection commands, use the <C>{'${CLAUDE_SKILL_DIR}'}</C> placeholder.</>,
              example: `# Security Review Checklist

## Input Validation
- [ ] All user input sanitized before DB queries
- [ ] File upload MIME types validated
- [ ] Path traversal prevented on file operations

## Authentication
- [ ] JWT tokens expire after 24 hours
- [ ] API keys stored in environment variables
- [ ] Passwords hashed with bcrypt or argon2`
            }]
          }]
        }, {
          id: 'commands',
          label: 'commands/',
          type: 'folder',
          icon: 'folder',
          color: '#788C5D',
          oneLiner: <>Single-file prompts invoked with <C>/name</C></>,
          note: commandsNote,
          when: <>User types <C>/command-name</C></>,
          description: <>A file at <C>commands/deploy.md</C> creates <C>/deploy</C> the same way a skill at <C>skills/deploy/SKILL.md</C> does, and both can be auto-invoked by Claude. Skills use a directory with SKILL.md, letting you bundle reference docs, templates, or scripts alongside the prompt.</>,
          tips: [<>Use <C>$ARGUMENTS</C> in the file to accept parameters: <C>/fix-issue 123</C></>, 'If a skill and command share a name, the skill takes precedence', 'New commands should usually be skills instead; commands remain supported'],
          docsLink: '/en/skills',
          children: [{
            id: 'cmd-example',
            label: 'fix-issue.md',
            type: 'file',
            icon: 'md',
            color: '#788C5D',
            badge: 'committed',
            oneLiner: <>Invoked as <C>/fix-issue &lt;number&gt;</C></>,
            note: commandsNote,
            description: [<>An example command for fixing a GitHub issue. Type <C>/fix-issue 123</C> and the <C>!`...`</C> line runs <C>gh issue view 123</C> in your shell, injecting the output into the prompt before Claude sees it.</>, <><C>$ARGUMENTS</C> substitutes whatever you typed after the command name. For positional access, use <C>$0</C> <C>$1</C> and so on.</>],
            example: `---
argument-hint: <issue-number>
---

!\`gh issue view $ARGUMENTS\`

Investigate and fix the issue above.

1. Trace the bug to its root cause
2. Implement the fix
3. Write or update tests
4. Summarize what you changed and why`
          }]
        }, {
          id: 'output-styles',
          label: 'output-styles/',
          type: 'folder',
          icon: 'folder',
          color: '#5AA7A7',
          oneLiner: 'Project-scoped output styles, if your team shares any',
          when: 'Files read at startup; the style you select with outputStyle applies to every response',
          description: <>Output styles are usually personal, so most live in <C>~/.claude/output-styles/</C>. Put one here if your team shares a style, like a review mode everyone uses. See <A href="#ce-global-output-styles">the Global tab</A> for the full explanation and example.</>,
          docsLink: '/en/output-styles',
          children: []
        }, {
          id: 'agents',
          label: 'agents/',
          type: 'folder',
          icon: 'folder',
          color: '#C46686',
          oneLiner: 'Specialized subagents with their own context window',
          when: 'Runs in its own context window when you or Claude invoke it',
          description: 'Each markdown file defines a subagent with its own system prompt, tool access, and optionally its own model. Subagents run in a fresh context window, keeping the main conversation clean. Useful for parallel work or isolated tasks.',
          tips: ['Each agent gets a fresh context window, separate from your main session', <>Restrict tool access per agent with the <C>tools:</C> frontmatter field</>, 'Type @ and pick an agent from the autocomplete to delegate directly'],
          docsLink: '/en/sub-agents',
          children: [{
            id: 'agent-reviewer',
            label: 'code-reviewer.md',
            type: 'file',
            icon: 'md',
            color: '#C46686',
            badge: 'committed',
            oneLiner: 'Subagent for isolated code review',
            when: 'Claude spawns it for review tasks, or you @-mention it from the autocomplete',
            description: <>An example subagent restricted to read-only tools. The <C>description</C> frontmatter tells Claude when to delegate to it automatically; <C>tools:</C> limits it to Read, Grep, and Glob so it can inspect code but never edit. The body becomes the subagent's system prompt.</>,
            example: `---
name: code-reviewer
description: Reviews code for correctness, security, and maintainability
tools: Read, Grep, Glob
---

You are a senior code reviewer. Review for:

1. Correctness: logic errors, edge cases, null handling
2. Security: injection, auth bypass, data exposure
3. Maintainability: naming, complexity, duplication

Every finding must include a concrete fix.`
          }]
        }, {
          id: 'workflows',
          label: 'workflows/',
          type: 'folder',
          icon: 'folder',
          color: '#C46686',
          oneLiner: 'Dynamic workflow scripts that orchestrate many subagents',
          when: 'Loaded at startup; each file becomes a /<name> command',
          description: <>Each <C>.js</C> file is a <A href="/docs/en/workflows">dynamic workflow</A>: a script the runtime executes to spawn and coordinate many subagents. Workflows are written by Claude and saved here from <C>/workflows</C> rather than authored from scratch.</>,
          tips: [<>Save a run from <C>/workflows</C> with <C>s</C> to create one of these</>, <>A project workflow takes precedence over a personal one in <C>~/.claude/workflows/</C> with the same name</>],
          docsLink: '/en/workflows'
        }, {
          id: 'agent-memory',
          label: 'agent-memory/',
          type: 'folder',
          icon: 'folder',
          color: '#C46686',
          badge: 'committed',
          autogen: true,
          oneLiner: 'Subagent persistent memory, separate from your main session auto memory',
          when: 'First 200 lines (capped at 25KB) of MEMORY.md loaded into the subagent system prompt when it runs',
          description: <>Subagents with <C>memory: project</C> in their frontmatter get a dedicated memory directory here. This is distinct from your <A href="/docs/en/memory#auto-memory">main session auto memory</A> at <C>~/.claude/projects/</C>: each subagent reads and writes its own MEMORY.md, not yours.</>,
          tips: [<>Only created for subagents that set the <C>memory:</C> frontmatter field</>, <>This directory holds project-scoped subagent memory, meant to be shared with your team. To keep memory out of version control use <C>memory: local</C>, which writes to <C>.claude/agent-memory-local/</C> instead. For cross-project memory use <C>memory: user</C>, which writes to <C>~/.claude/agent-memory/</C></>, <>The main session auto memory is a different feature; see <C>~/.claude/projects/</C> in the Global tab</>],
          docsLink: '/en/sub-agents#enable-persistent-memory',
          children: [{
            id: 'agent-memory-sub',
            label: '<agent-name>/',
            type: 'folder',
            icon: 'folder',
            color: '#C46686',
            autogen: true,
            children: [{
              id: 'agent-memory-md',
              label: 'MEMORY.md',
              type: 'file',
              icon: 'md',
              color: '#C46686',
              badge: 'committed',
              autogen: true,
              oneLiner: 'The subagent writes and maintains this file automatically',
              when: 'Loaded into the subagent system prompt when the subagent starts',
              description: <>Works the same as your <A href="/docs/en/memory#auto-memory">main auto memory</A>: the subagent creates and updates this file itself. You do not write it. The subagent reads it at the start of each task and writes back what it learns.</>,
              example: `# code-reviewer memory

## Patterns seen
- Project uses custom Result<T, E> type, not exceptions
- Auth middleware expects Bearer token in Authorization header
- Tests use factory functions in test/factories/

## Recurring issues
- Missing null checks on API responses (src/api/*)
- Unhandled promise rejections in background jobs`
            }]
          }]
        }]
      }]
    },
    global: {
      label: '~/',
      children: [{
        id: 'claude-json',
        label: '.claude.json',
        type: 'file',
        icon: 'json',
        color: 'var(--ce-text-3)',
        badge: 'local',
        oneLiner: 'App state and UI preferences',
        when: <>Read at session start for your preferences and MCP servers. Claude Code writes back to it when you change settings in <C>/config</C> or approve trust prompts</>,
        description: <>Holds state that does not belong in settings.json: theme, OAuth session, per-project trust decisions, your personal MCP servers, and UI toggles. Mostly managed through <C>/config</C> rather than editing directly.</>,
        tips: [<>IDE toggles like <C>autoConnectIde</C> and <C>externalEditorContext</C> live here, not in settings.json</>, <>The <C>projects</C> key tracks per-project state like trust-dialog acceptance and last-session metrics. Permission rules you approve in-session go to <C>.claude/settings.local.json</C> instead</>, <>MCP servers here are yours only: user scope applies across all projects, local scope is per-project but not committed. Team-shared servers go in <C>.mcp.json</C> at the project root instead</>],
        example: `{
  "autoConnectIde": true,
  "externalEditorContext": true,
  "mcpServers": {
    "my-tools": {
      "command": "npx",
      "args": ["-y", "@example/mcp-server"]
    }
  }
}`,
        docsLink: '/en/settings-reference#global-config-settings'
      }, {
        id: 'global-dot-claude',
        label: '.claude/',
        type: 'folder',
        icon: 'folder',
        color: 'var(--ce-accent)',
        oneLiner: 'Your personal configuration across all projects',
        description: 'The global counterpart to your project .claude/ directory. Files here apply to every project you work in and are never committed to any repository.',
        children: [{
          id: 'global-claude-md',
          label: 'CLAUDE.md',
          type: 'file',
          icon: 'md',
          color: '#6A9BCC',
          badge: 'local',
          oneLiner: 'Personal preferences across every project',
          when: 'Loaded at the start of every session, in every project',
          description: 'Your global instruction file. Loaded alongside the project CLAUDE.md at session start, so both are in context together. When instructions conflict, project-level instructions take priority. Keep this to preferences that apply everywhere: response style, commit format, personal conventions.',
          tips: ['Keep it short since it loads into context for every project, alongside that project\'s own CLAUDE.md', 'Good for response style, commit format, and personal conventions'],
          example: `# Global preferences

- Keep explanations concise
- Use conventional commit format
- Show the terminal command to verify changes
- Prefer composition over inheritance`,
          docsLink: '/en/memory'
        }, {
          id: 'global-settings',
          label: 'settings.json',
          type: 'file',
          icon: 'json',
          color: 'var(--ce-text-3)',
          badge: 'local',
          oneLiner: 'Default settings for all projects',
          when: 'Your defaults. Project and local settings.json override any keys you also set there',
          description: [<>Same keys as project <C>settings.json</C>: permissions, hooks, model, environment variables, and the rest. Put settings here that you want in every project, like permissions you always allow, a preferred model, or a notification hook that runs regardless of which project you're in.</>, <>Settings follow a precedence order: project <C>settings.json</C> overrides any matching keys you set here. This is different from CLAUDE.md, where global and project files are both loaded into context rather than merged key by key.</>],
          example: `{
  "permissions": {
    "allow": [
      "Bash(git log *)",
      "Bash(git diff *)"
    ]
  }
}`,
          docsLink: '/en/settings'
        }, {
          id: 'keybindings',
          label: 'keybindings.json',
          type: 'file',
          icon: 'json',
          color: 'var(--ce-text-3)',
          badge: 'local',
          oneLiner: 'Custom keyboard shortcuts',
          when: 'Read at session start and hot-reloaded when you edit the file',
          description: <>Rebind keyboard shortcuts in the interactive CLI. Run <C>/keybindings</C> to create or open this file with a schema reference. Ctrl+C, Ctrl+D, Ctrl+M, and Caps Lock are reserved and cannot be rebound.</>,
          exampleIntro: <>This example binds <C>Ctrl+E</C> to open your external editor and unbinds <C>Ctrl+U</C> by setting it to <C>null</C>. The <C>context</C> field scopes bindings to a specific part of the CLI, here the main chat input.</>,
          example: `{
  "$schema": "https://www.schemastore.org/claude-code-keybindings.json",
  "$docs": "https://code.claude.com/docs/en/keybindings",
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "ctrl+e": "chat:externalEditor",
        "ctrl+u": null
      }
    }
  ]
}`,
          docsLink: '/en/keybindings'
        }, {
          id: 'themes',
          label: 'themes/',
          type: 'folder',
          icon: 'folder',
          color: '#5AA7A7',
          oneLiner: 'Custom color themes',
          when: <>Read at session start and hot-reloaded when files change. Listed in <C>/theme</C></>,
          description: <>Each <C>.json</C> file defines a custom color theme: a built-in <C>base</C> preset plus an <C>overrides</C> map of color tokens. Create one interactively with <C>/theme</C> or write the JSON by hand. Selecting a custom theme stores <C>custom:&lt;slug&gt;</C> as your theme preference.</>,
          example: `{
  "name": "Dracula",
  "base": "dark",
  "overrides": {
    "claude": "#bd93f9",
    "error": "#ff5555",
    "success": "#50fa7b"
  }
}`,
          docsLink: '/en/terminal-config#create-a-custom-theme',
          children: []
        }, {
          id: 'global-projects',
          label: 'projects/',
          type: 'folder',
          icon: 'folder',
          color: '#E8A45C',
          autogen: true,
          oneLiner: "Auto memory: Claude's notes to itself, per project",
          when: 'MEMORY.md loaded at session start; topic files read on demand',
          description: 'Auto memory lets Claude accumulate knowledge across sessions without you writing anything. Claude saves notes as it works: build commands, debugging insights, architecture notes. Each project gets its own memory directory keyed by the repository path.',
          tips: [<>On by default. Toggle with <C>/memory</C> or <C>autoMemoryEnabled</C> in settings</>, 'MEMORY.md is the index loaded each session. The first 200 lines, or 25KB, whichever comes first, are read', 'Topic files like debugging.md are read on demand, not at startup', 'These are plain markdown. Edit or delete them anytime'],
          docsLink: '/en/memory#auto-memory',
          children: [{
            id: 'memory-dir',
            label: '<project>/memory/',
            type: 'folder',
            icon: 'folder',
            color: '#E8A45C',
            autogen: true,
            oneLiner: "Claude's accumulated knowledge for one project",
            children: [{
              id: 'memory-md',
              label: 'MEMORY.md',
              type: 'file',
              icon: 'md',
              color: '#E8A45C',
              badge: 'local',
              autogen: true,
              oneLiner: 'Claude writes and maintains this file automatically',
              when: 'First 200 lines (capped at 25KB) loaded at session start',
              description: 'Claude creates and updates this file as it works; you do not write it yourself. It acts as an index that Claude reads at the start of every session, pointing to topic files for detail. You can edit or delete it, but Claude will keep updating it.',
              example: `# Memory Index

## Project
- [build-and-test.md](build-and-test.md): npm run build (~45s), Vitest, dev server on 3001
- [architecture.md](architecture.md): API client singleton, refresh-token auth

## Reference
- [debugging.md](debugging.md): auth token rotation and DB connection troubleshooting`,
              docsLink: '/en/memory'
            }, {
              id: 'memory-topic',
              label: 'debugging.md',
              type: 'file',
              icon: 'md',
              color: '#E8A45C',
              badge: 'local',
              autogen: true,
              oneLiner: 'Topic notes Claude writes when MEMORY.md gets long',
              when: 'Claude reads this when a related task comes up',
              description: 'An example of a topic file Claude creates when MEMORY.md grows too long. Claude picks the filename based on what it splits out: debugging.md, architecture.md, build-commands.md, or similar. You never create these yourself. Claude reads a topic file back only when the current task relates to it.',
              example: `---
name: Debugging patterns
description: Auth token rotation and database connection troubleshooting for this project
type: reference
---

## Auth Token Issues
- Refresh token rotation: old token invalidated immediately
- If 401 after refresh: check clock skew between client and server

## Database Connection Drops
- Connection pool: max 10 in dev, 50 in prod
- Always check \`docker compose ps\` first`
            }]
          }]
        }, {
          id: 'global-rules',
          label: 'rules/',
          type: 'folder',
          icon: 'folder',
          color: '#9B7BC4',
          oneLiner: 'User-level rules that apply to every project',
          when: <>Rules without <C>paths:</C> load at session start. Rules with <C>paths:</C> load when a matching file enters context</>,
          description: 'Same as project .claude/rules/ but applies everywhere. Use this for conventions you want across all your work, like personal code style or commit message format.',
          docsLink: '/en/memory#organize-rules-with-claude/rules/',
          children: []
        }, {
          id: 'global-skills',
          label: 'skills/',
          type: 'folder',
          icon: 'folder',
          color: '#D4A843',
          oneLiner: 'Personal skills available in every project',
          when: <>Invoked with <C>/skill-name</C> in any project</>,
          description: 'Skills you built for yourself that work everywhere. Same structure as project skills: each is a folder with SKILL.md, scoped to your user account instead of a single project.',
          docsLink: '/en/skills',
          children: []
        }, {
          id: 'global-commands',
          label: 'commands/',
          type: 'folder',
          icon: 'folder',
          color: '#788C5D',
          oneLiner: 'Personal single-file commands available in every project',
          note: commandsNote,
          when: <>User types <C>/command-name</C> in any project</>,
          description: 'Same as project commands/ but scoped to your user account. Each markdown file becomes a command available everywhere.',
          docsLink: '/en/skills',
          children: []
        }, {
          id: 'global-output-styles',
          label: 'output-styles/',
          type: 'folder',
          icon: 'folder',
          color: '#5AA7A7',
          oneLiner: 'Custom instruction sets that adjust how Claude works',
          when: 'Files read at startup; the style you select with outputStyle applies to every response',
          description: [<>Each markdown file defines an output style: a set of instructions for Claude that, by default, also replaces the built-in software-engineering task instructions. Use this to adapt Claude Code for uses beyond coding, or to add teaching or review modes.</>, <>Select a built-in or custom style with <C>/output-style</C>, <C>/config</C>, or the <C>outputStyle</C> key in settings. Styles here are available in every project; project-level styles with the same name take precedence.</>],
          tips: ['Built-in styles Default, Proactive, Concise, Explanatory, and Learning are included with Claude Code; custom styles go here', <>Set <C>keep-coding-instructions: true</C> in frontmatter to keep the default task instructions alongside your additions</>, 'Switching styles mid-session applies from your next message; in the terminal, a style file you create or edit mid-session is picked up after a restart'],
          docsLink: '/en/output-styles',
          children: [{
            id: 'output-style-example',
            label: 'teaching.md',
            type: 'file',
            icon: 'md',
            color: '#5AA7A7',
            badge: 'local',
            oneLiner: 'Example style that adds explanations and leaves small changes for you',
            when: <>Active when <C>outputStyle</C> in settings is set to <C>teaching</C></>,
            description: <>With this style, Claude adds a "Why this approach" note after each task and leaves TODO(human) markers for changes under 10 lines instead of writing them itself. Select it by setting <C>outputStyle</C> to the filename without .md, or to the <C>name</C> field if you set one in frontmatter.</>,
            example: `---
description: Explains reasoning and asks you to implement small pieces
keep-coding-instructions: true
---

After completing each task, add a brief "Why this approach" note
explaining the key design decision.

When a change is under 10 lines, ask the user to implement it
themselves by leaving a TODO(human) marker instead of writing it.`
          }]
        }, {
          id: 'global-agents',
          label: 'agents/',
          type: 'folder',
          icon: 'folder',
          color: '#C46686',
          oneLiner: 'Personal subagents available in every project',
          when: 'Claude delegates or you @-mention in any project',
          description: 'Subagents defined here are available across all your projects. Same format as project agents.',
          docsLink: '/en/sub-agents',
          children: []
        }, {
          id: 'global-workflows',
          label: 'workflows/',
          type: 'folder',
          icon: 'folder',
          color: '#C46686',
          oneLiner: 'Personal dynamic workflows available in every project',
          when: 'Loaded at startup; each file becomes a /<name> command',
          description: <>Workflow scripts saved here are available across all your projects. A project workflow with the same name in <C>.claude/workflows/</C> takes precedence.</>,
          docsLink: '/en/workflows',
          children: []
        }, {
          id: 'global-agent-memory',
          label: 'agent-memory/',
          type: 'folder',
          icon: 'folder',
          color: '#C46686',
          autogen: true,
          oneLiner: <>Persistent memory for subagents with <C>memory: user</C></>,
          when: 'Loaded into the subagent system prompt when the subagent starts',
          description: <>Subagents with <C>memory: user</C> in their frontmatter store knowledge here that persists across all projects. For project-scoped subagent memory, see <C>.claude/agent-memory/</C> instead.</>,
          docsLink: '/en/sub-agents#enable-persistent-memory',
          children: []
        }]
      }]
    }
  }), []);
  const BADGE_STYLES = useMemo(() => ({
    committed: {
      bg: 'rgba(85,138,66,0.08)',
      color: 'var(--ce-badge-committed)',
      border: 'rgba(85,138,66,0.15)',
      label: 'committed'
    },
    gitignored: {
      bg: 'rgba(217,119,87,0.06)',
      color: 'var(--ce-badge-gitignored)',
      border: 'rgba(217,119,87,0.15)',
      label: 'gitignored'
    },
    local: {
      bg: 'rgba(115,114,108,0.06)',
      color: 'var(--ce-badge-local)',
      border: 'rgba(115,114,108,0.12)',
      label: 'local only'
    },
    autogen: {
      bg: 'rgba(232,164,92,0.1)',
      color: 'var(--ce-badge-autogen)',
      border: 'rgba(232,164,92,0.2)',
      label: 'Claude writes'
    }
  }), []);
  const allNodes = useMemo(() => {
    const flatten = (nodes, acc, path, parentId) => {
      for (const node of nodes) {
        const nextPath = [...path, node.label];
        acc[node.id] = {
          ...node,
          path: nextPath,
          parentId
        };
        if (node.children) flatten(node.children, acc, nextPath, node.id);
      }
      return acc;
    };
    const project = flatten(FILE_TREE.project.children, {}, [FILE_TREE.project.label]);
    const global = flatten(FILE_TREE.global.children, {}, [FILE_TREE.global.label]);
    for (const id in project) project[id].root = 'project';
    for (const id in global) global[id].root = 'global';
    return {
      ...project,
      ...global
    };
  }, [FILE_TREE]);
  const allFolderIds = useMemo(() => Object.keys(allNodes).filter(id => allNodes[id].type === 'folder'), [allNodes]);
  const DEFAULT_EXPANDED = ['dot-claude', 'rules', 'skills', 'skill-review', 'commands', 'agents', 'agent-memory', 'agent-memory-sub', 'global-dot-claude', 'global-output-styles', 'global-projects', 'memory-dir'];
  const [mounted, setMounted] = useState(false);
  const [activeRoot, setActiveRoot] = useState('project');
  const [selectedId, setSelectedId] = useState('claude-md');
  const [expandedFolders, setExpandedFolders] = useState(() => new Set(DEFAULT_EXPANDED));
  const [forceMobile, setForceMobile] = useState(false);
  const [copiedId, setCopiedId] = useState(null);
  const [isFullscreen, setIsFullscreen] = useState(false);
  const copyTimeoutRef = useRef(null);
  const rootRef = useRef(null);
  useEffect(() => {
    setMounted(true);
    const applyHash = scroll => {
      const hash = window.location.hash.slice(1);
      if (!hash.startsWith('ce-')) return;
      const id = hash.slice(3);
      const node = allNodes[id];
      if (!node) return;
      setActiveRoot(node.root);
      setSelectedId(id);
      setExpandedFolders(new Set(allFolderIds));
      if (scroll && rootRef.current) rootRef.current.scrollIntoView({
        behavior: 'smooth',
        block: 'start'
      });
    };
    applyHash(false);
    const onHashChange = () => applyHash(true);
    const onFsChange = () => setIsFullscreen(!!document.fullscreenElement);
    window.addEventListener('hashchange', onHashChange);
    document.addEventListener('fullscreenchange', onFsChange);
    return () => {
      if (copyTimeoutRef.current) clearTimeout(copyTimeoutRef.current);
      window.removeEventListener('hashchange', onHashChange);
      document.removeEventListener('fullscreenchange', onFsChange);
    };
  }, []);
  useEffect(() => {
    if (!mounted || !rootRef.current) return;
    const hash = window.location.hash.slice(1);
    if (hash.startsWith('ce-') && allNodes[hash.slice(3)]) {
      rootRef.current.scrollIntoView({
        behavior: 'smooth',
        block: 'start'
      });
    }
  }, [mounted]);
  if (!mounted) return null;
  const selected = allNodes[selectedId];
  const tree = FILE_TREE[activeRoot];
  const isCopied = copiedId === selected.id;
  const toggleFolder = id => {
    const next = new Set(expandedFolders);
    next.has(id) ? next.delete(id) : next.add(id);
    setExpandedFolders(next);
  };
  const switchRoot = root => {
    if (root === activeRoot) return;
    setActiveRoot(root);
    const firstId = FILE_TREE[root].children[0].id;
    setSelectedId(firstId);
    try {
      history.replaceState(null, '', '#ce-' + firstId);
    } catch (e) {}
  };
  const toggleFullscreen = () => {
    if (!rootRef.current) return;
    if (document.fullscreenElement) document.exitFullscreen(); else rootRef.current.requestFullscreen().catch(() => {});
  };
  const selectNode = n => {
    setSelectedId(n.id);
    if (n.type === 'folder' && !expandedFolders.has(n.id)) toggleFolder(n.id);
    try {
      history.replaceState(null, '', '#ce-' + n.id);
    } catch (e) {}
  };
  const iconBtn = {
    width: 28,
    flexShrink: 0,
    borderRadius: '6px',
    border: 'none',
    cursor: 'pointer',
    background: 'transparent',
    color: 'var(--ce-text-4)',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center'
  };
  const visibleFolderIds = allFolderIds.filter(id => allNodes[id].root === activeRoot);
  const allExpanded = visibleFolderIds.every(id => expandedFolders.has(id));
  const toggleAllFolders = () => {
    const next = new Set(expandedFolders);
    visibleFolderIds.forEach(id => allExpanded ? next.delete(id) : next.add(id));
    setExpandedFolders(next);
  };
  const onTreeKeyDown = e => {
    if (!['ArrowDown', 'ArrowUp', 'ArrowRight', 'ArrowLeft'].includes(e.key)) return;
    const visible = [];
    const walk = nodes => {
      for (const n of nodes) {
        visible.push(n.id);
        if (n.children && expandedFolders.has(n.id)) walk(n.children);
      }
    };
    walk(tree.children);
    const i = visible.indexOf(selectedId);
    if (i === -1) return;
    e.preventDefault();
    if (e.key === 'ArrowDown' && i < visible.length - 1) selectNode(allNodes[visible[i + 1]]); else if (e.key === 'ArrowUp' && i > 0) selectNode(allNodes[visible[i - 1]]); else if (e.key === 'ArrowRight' && selected.type === 'folder') {
      if (!expandedFolders.has(selectedId)) toggleFolder(selectedId); else if (selected.children && selected.children.length) selectNode(allNodes[selected.children[0].id]);
    } else if (e.key === 'ArrowLeft') {
      if (selected.type === 'folder' && expandedFolders.has(selectedId)) toggleFolder(selectedId); else if (selected.parentId) selectNode(allNodes[selected.parentId]);
    }
  };
  const copyExample = (id, text) => {
    const done = () => {
      setCopiedId(id);
      if (copyTimeoutRef.current) clearTimeout(copyTimeoutRef.current);
      copyTimeoutRef.current = setTimeout(() => setCopiedId(null), 2000);
    };
    const fallback = () => {
      const ta = document.createElement('textarea');
      ta.value = text;
      ta.style.position = 'fixed';
      ta.style.opacity = '0';
      document.body.appendChild(ta);
      ta.select();
      try {
        if (document.execCommand('copy')) done();
      } catch (e) {}
      document.body.removeChild(ta);
    };
    if (navigator.clipboard) {
      navigator.clipboard.writeText(text).then(done, fallback);
    } else {
      fallback();
    }
  };
  const renderIcon = (icon, color, size) => {
    const sz = size || 14;
    if (icon === 'folder') {
      return <svg width={sz} height={sz} viewBox="0 0 14 14" fill="none">
          <path d="M1.5 3.5a1 1 0 0 1 1-1h2.6l1 1.2h5.4a1 1 0 0 1 1 1v5.8a1 1 0 0 1-1 1h-9a1 1 0 0 1-1-1V3.5z" fill={color} fillOpacity="0.15" stroke={color} strokeWidth="1" />
        </svg>;
    }
    if (icon === 'json') {
      return <svg width={sz} height={sz} viewBox="0 0 14 14" fill="none">
          <rect x="2" y="1.5" width="10" height="11" rx="1.5" fill={color} fillOpacity="0.15" stroke={color} strokeWidth="1" />
          <text x="7" y="9" fontSize="6" fontFamily="monospace" fill={color} textAnchor="middle" fontWeight="700">{'{}'}</text>
        </svg>;
    }
    return <svg width={sz} height={sz} viewBox="0 0 14 14" fill="none">
        <rect x="2" y="1.5" width="10" height="11" rx="1.5" fill={color} fillOpacity="0.15" stroke={color} strokeWidth="1" />
        <line x1="4.5" y1="5" x2="9.5" y2="5" stroke={color} strokeWidth="1" />
        <line x1="4.5" y1="7" x2="9.5" y2="7" stroke={color} strokeWidth="1" />
        <line x1="4.5" y1="9" x2="8" y2="9" stroke={color} strokeWidth="1" />
      </svg>;
  };
  const renderNode = (node, depth) => {
    const isFolder = node.type === 'folder';
    const isExpanded = expandedFolders.has(node.id);
    const isSelected = selectedId === node.id;
    return <div key={node.id}>
        <button role="treeitem" tabIndex={-1} onClick={() => selectNode(node)} aria-selected={isSelected} aria-expanded={isFolder ? isExpanded : undefined} style={{
      display: 'flex',
      alignItems: 'center',
      gap: '5px',
      width: '100%',
      padding: `4px 8px 4px ${8 + depth * 16}px`,
      background: isSelected ? 'var(--ce-accent-bg)' : 'transparent',
      borderTop: 'none',
      borderRight: 'none',
      borderBottom: 'none',
      borderLeft: isSelected ? '2px solid var(--ce-accent)' : '2px solid transparent',
      outline: 'none',
      cursor: 'pointer',
      textAlign: 'left',
      fontFamily: 'var(--ce-mono)',
      fontSize: '13.5px',
      color: isSelected ? 'var(--ce-accent)' : 'var(--ce-text-2)',
      fontWeight: isSelected ? 550 : 400,
      transition: 'all 0.1s'
    }}>
          {isFolder ? <span onClick={e => {
      e.stopPropagation();
      toggleFolder(node.id);
    }} style={{
      fontSize: '14px',
      color: 'var(--ce-text-4)',
      width: '20px',
      height: '20px',
      display: 'inline-flex',
      alignItems: 'center',
      justifyContent: 'center',
      cursor: 'pointer',
      borderRadius: '4px',
      marginLeft: '-6px',
      flexShrink: 0
    }} onMouseEnter={e => {
      e.currentTarget.style.background = 'var(--ce-arrow-hover)';
      e.currentTarget.style.color = 'var(--ce-text-2)';
    }} onMouseLeave={e => {
      e.currentTarget.style.background = 'transparent';
      e.currentTarget.style.color = 'var(--ce-text-4)';
    }}>{isExpanded ? '▾' : '▸'}</span> : <span style={{
      width: '14px',
      flexShrink: 0
    }} />}
          {renderIcon(node.icon, node.color)}
          <span style={{
      flex: 1,
      overflow: 'hidden',
      textOverflow: 'ellipsis',
      whiteSpace: 'nowrap'
    }}>{node.label}</span>
          {node.badge && BADGE_STYLES[node.badge] && <span title={BADGE_STYLES[node.badge].label} style={{
      width: 6,
      height: 6,
      borderRadius: '50%',
      background: BADGE_STYLES[node.badge].color,
      flexShrink: 0,
      opacity: 0.7
    }} />}
        </button>
        {isFolder && isExpanded && node.children && <div role="group">{node.children.map(child => renderNode(child, depth + 1))}</div>}
      </div>;
  };
  return <>
    <style>{`
      .ce-root {
        --ce-mono: var(--font-mono, ui-monospace, monospace);
        --ce-accent: #D97757;
        --ce-accent-bg: rgba(217,119,87,0.06);
        --ce-accent-border: rgba(217,119,87,0.12);
        --ce-bg: #fff;
        --ce-surface: #FAFAF7;
        --ce-surface-hover: #F0EEE6;
        --ce-border: #E8E6DC;
        --ce-border-subtle: #F0EEE6;
        --ce-text: #141413;
        --ce-text-2: #5E5D59;
        --ce-text-3: #73726C;
        --ce-text-4: #9C9A92;
        --ce-text-5: #B8B6AE;
        --ce-sep: #D1CFC5;
        --ce-code-header: #F5F4ED;
        --ce-code-bg: #1A1918;
        --ce-arrow-hover: rgba(0,0,0,0.08);
        --ce-badge-committed: #3d6b2e;
        --ce-badge-gitignored: #b85c3a;
        --ce-badge-local: #5e5d59;
        --ce-badge-autogen: #b07520;
        --ce-when-text: #4a7fb5;
      }
      .dark .ce-root {
        --ce-bg: #1a1918;
        --ce-surface: #232221;
        --ce-surface-hover: #2e2d2b;
        --ce-border: #3a3936;
        --ce-border-subtle: #2e2d2b;
        --ce-text: #e8e6dc;
        --ce-text-2: #c4c2b8;
        --ce-text-3: #9c9a92;
        --ce-text-4: #73726c;
        --ce-text-5: #5e5d59;
        --ce-sep: #4a4946;
        --ce-code-header: #2e2d2b;
        --ce-code-bg: #0d0d0c;
        --ce-arrow-hover: rgba(255,255,255,0.08);
        --ce-badge-committed: #6fa85c;
        --ce-badge-gitignored: #e08a60;
        --ce-badge-local: #9c9a92;
        --ce-badge-autogen: #e8a45c;
        --ce-when-text: #8bb4e0;
      }
      .ce-mobile-fallback { display: none; border: 1px solid rgba(0,0,0,0.1); background: rgba(0,0,0,0.03); }
      .dark .ce-mobile-fallback { border-color: rgba(255,255,255,0.15); background: rgba(255,255,255,0.04); }
      @media (max-width: 700px) {
        .ce-root:not(.ce-force) { display: none !important; }
        .ce-mobile-fallback { display: block; }
      }
    `}</style>
    {!forceMobile && <div className="ce-mobile-fallback" style={{
    padding: '14px 16px',
    borderRadius: '8px',
    fontSize: '14px'
  }}>
      The interactive explorer works best on a larger screen. See the <a href="#file-reference" style={{
    color: '#D97757'
  }}>file reference table</a> below, or <button onClick={() => setForceMobile(true)} style={{
    border: 'none',
    background: 'none',
    padding: 0,
    color: '#D97757',
    textDecoration: 'underline',
    cursor: 'pointer',
    font: 'inherit'
  }}>show the explorer anyway</button>.
    </div>}
    <div ref={rootRef} className={forceMobile ? 'ce-root ce-force' : 'ce-root'} style={{
    borderRadius: isFullscreen ? 0 : '12px',
    border: '1px solid var(--ce-border)',
    background: 'var(--ce-bg)',
    display: 'flex',
    alignItems: 'stretch',
    overflow: 'hidden',
    fontFamily: 'var(--font-sans, -apple-system, sans-serif)',
    ...isFullscreen && ({
      height: '100vh'
    })
  }}>
      {}
      <div style={{
    width: 'min(240px, 35%)',
    minWidth: '180px',
    flexShrink: 0,
    borderRight: '1px solid var(--ce-border-subtle)',
    background: 'var(--ce-surface)',
    display: 'flex',
    flexDirection: 'column'
  }}>
        <div style={{
    padding: '8px 8px 4px',
    borderBottom: '1px solid var(--ce-border-subtle)',
    display: 'flex',
    gap: '4px'
  }}>
          {['project', 'global'].map(root => <button key={root} onClick={() => switchRoot(root)} style={{
    flex: 1,
    padding: '6px 0',
    borderRadius: '6px',
    border: 'none',
    cursor: 'pointer',
    fontFamily: 'var(--ce-mono)',
    fontSize: '11.5px',
    background: activeRoot === root ? 'var(--ce-accent-bg)' : 'transparent',
    color: activeRoot === root ? 'var(--ce-accent)' : 'var(--ce-text-4)',
    fontWeight: activeRoot === root ? 600 : 430
  }}>
              {root === 'project' ? 'Project' : 'Global (~/)'}
            </button>)}
          <button onClick={toggleAllFolders} title={allExpanded ? 'Collapse all' : 'Expand all'} style={{
    ...iconBtn,
    fontSize: 11
  }}>
            {allExpanded ? '⊟' : '⊞'}
          </button>
          <button onClick={toggleFullscreen} title={isFullscreen ? 'Exit fullscreen' : 'Fullscreen'} style={{
    ...iconBtn,
    fontSize: 13
  }}>
            {isFullscreen ? '⤡' : '⛶'}
          </button>
        </div>
        <div role="tree" aria-label="Configuration files" tabIndex={0} onKeyDown={onTreeKeyDown} style={{
    padding: '6px 0',
    overflowY: 'auto',
    flex: 1,
    outline: 'none'
  }}>
          {tree.children.map(node => renderNode(node, 0))}
        </div>
      </div>

      {}
      <div style={{
    flex: 1,
    minWidth: 0,
    padding: '20px 24px',
    minHeight: '400px',
    overflowY: 'auto'
  }}>
            <span aria-live="polite" style={{
    position: 'absolute',
    width: 1,
    height: 1,
    overflow: 'hidden',
    clip: 'rect(0 0 0 0)'
  }}>{selected.label} selected</span>
            {}
            <div style={{
    fontFamily: 'var(--ce-mono)',
    fontSize: '11px',
    color: 'var(--ce-text-4)',
    marginBottom: '10px',
    cursor: 'default'
  }}>
              {selected.path.map((seg, i) => <span key={i}>
                  <span style={{
    color: i === selected.path.length - 1 ? 'var(--ce-accent)' : 'var(--ce-text-4)'
  }}>{seg.replace(/\/$/, '')}</span>
                  {i < selected.path.length - 1 && <span style={{
    color: 'var(--ce-sep)'
  }}> / </span>}
                </span>)}
            </div>

            {}
            <div style={{
    display: 'flex',
    alignItems: 'flex-start',
    gap: '10px',
    marginBottom: '10px'
  }}>
              <span style={{
    flexShrink: 0,
    display: 'flex'
  }}>{renderIcon(selected.icon, selected.color, 24)}</span>
              <div style={{
    flex: 1,
    minWidth: 0
  }}>
                <div style={{
    fontSize: '22px',
    fontWeight: 600,
    color: 'var(--ce-text)',
    letterSpacing: '-0.3px',
    lineHeight: '26px'
  }}>{selected.label}</div>
                {selected.oneLiner && <div style={{
    fontSize: '15px',
    color: 'var(--ce-text-3)',
    marginTop: '3px'
  }}>{selected.oneLiner}</div>}
              </div>
              <div style={{
    display: 'flex',
    gap: '4px',
    flexShrink: 0
  }}>
                {[selected.autogen && 'autogen', selected.badge].filter(Boolean).map(k => {
    const s = BADGE_STYLES[k];
    if (!s) return null;
    return <span key={k} style={{
      fontFamily: 'var(--ce-mono)',
      fontSize: '10px',
      fontWeight: 600,
      textTransform: 'uppercase',
      letterSpacing: '0.3px',
      padding: '2px 6px',
      borderRadius: '4px',
      background: s.bg,
      color: s.color,
      border: `0.5px solid ${s.border}`
    }}>{s.label}</span>;
  })}
              </div>
            </div>

            {}
            {selected.note && <div style={{
    padding: '10px 12px',
    borderRadius: '8px',
    marginBottom: '14px',
    background: 'rgba(217,119,87,0.06)',
    border: '1px solid rgba(217,119,87,0.2)',
    borderLeft: '3px solid var(--ce-accent)',
    fontSize: '15px',
    color: 'var(--ce-text-2)',
    lineHeight: 1.6
  }}>
                {selected.note}
              </div>}

            {}
            {selected.when && <div style={{
    padding: '8px 12px',
    borderRadius: '6px',
    background: 'rgba(106,155,204,0.06)',
    border: '0.5px solid rgba(106,155,204,0.12)',
    fontSize: '15px',
    color: 'var(--ce-when-text)',
    marginBottom: '16px'
  }}>
                <div style={{
    fontSize: '10px',
    fontWeight: 700,
    textTransform: 'uppercase',
    letterSpacing: '0.4px',
    opacity: 0.65,
    marginBottom: '3px'
  }}>When it loads</div>
                <div style={{
    fontWeight: 500
  }}>{selected.when}</div>
              </div>}

            {}
            {selected.description && <div style={{
    fontSize: '16px',
    color: 'var(--ce-text-2)',
    lineHeight: 1.65,
    marginBottom: '16px'
  }}>
                {Array.isArray(selected.description) ? selected.description.map((para, i) => <div key={i} style={{
    marginBottom: i < selected.description.length - 1 ? '12px' : 0
  }}>{para}</div>) : selected.description}
              </div>}

            {}
            {selected.contains && selected.contains.length > 0 && <div style={{
    marginBottom: '16px'
  }}>
                <div style={{
    fontSize: '11px',
    fontWeight: 700,
    color: 'var(--ce-text-4)',
    textTransform: 'uppercase',
    letterSpacing: '0.4px',
    marginBottom: '8px'
  }}>Common keys</div>
                {selected.contains.map((item, i) => <div key={i} style={{
    display: 'flex',
    gap: '7px',
    fontSize: '15px',
    color: 'var(--ce-text-2)',
    lineHeight: 1.5,
    marginBottom: '5px'
  }}>
                    <span style={{
    fontSize: '7px',
    color: 'var(--ce-text-4)',
    marginTop: '6px'
  }}>●</span>
                    <span>{item}</span>
                  </div>)}
              </div>}

            {}
            {selected.tips && selected.tips.length > 0 && <div style={{
    padding: '12px 14px',
    borderRadius: '8px',
    background: 'var(--ce-surface)',
    border: '1px solid var(--ce-border-subtle)',
    marginBottom: '16px'
  }}>
                <div style={{
    fontSize: '11px',
    fontWeight: 700,
    color: 'var(--ce-accent)',
    textTransform: 'uppercase',
    letterSpacing: '0.4px',
    marginBottom: '6px'
  }}>Tips</div>
                {selected.tips.map((tip, i) => <div key={i} style={{
    display: 'flex',
    gap: '7px',
    fontSize: '14.5px',
    color: 'var(--ce-text-2)',
    marginBottom: i < selected.tips.length - 1 ? '5px' : 0
  }}>
                    <span style={{
    fontSize: '7px',
    color: 'var(--ce-accent)',
    marginTop: '6px'
  }}>●</span>
                    <span>{tip}</span>
                  </div>)}
              </div>}

            {}
            {selected.example && <div style={{
    marginBottom: '16px'
  }}>
                {selected.exampleIntro && <div style={{
    fontSize: '15px',
    color: 'var(--ce-text-2)',
    lineHeight: 1.6,
    marginBottom: '10px'
  }}>
                    {selected.exampleIntro}
                  </div>}
                <div style={{
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    padding: '6px 10px',
    background: 'var(--ce-code-header)',
    border: '1px solid var(--ce-border)',
    borderRadius: '8px 8px 0 0'
  }}>
                  <span style={{
    fontFamily: 'var(--ce-mono)',
    fontSize: '11px',
    fontWeight: 600,
    color: 'var(--ce-text-3)'
  }}>{selected.label}</span>
                  <button onClick={() => copyExample(selected.id, selected.example)} style={{
    padding: '3px 8px',
    borderRadius: '4px',
    fontSize: '11px',
    fontWeight: 600,
    cursor: 'pointer',
    transition: 'all 0.15s',
    background: isCopied ? 'rgba(85,138,66,0.08)' : 'var(--ce-code-header)',
    border: isCopied ? '0.5px solid rgba(85,138,66,0.2)' : '0.5px solid var(--ce-border)',
    color: isCopied ? '#558A42' : 'var(--ce-text-3)'
  }}>
                    {isCopied ? '✓ Copied' : 'Copy'}
                  </button>
                </div>
                <pre style={{
    margin: 0,
    padding: '12px 14px',
    background: 'var(--ce-code-bg)',
    color: '#E8E6DC',
    fontFamily: 'var(--ce-mono)',
    fontSize: '13px',
    lineHeight: 1.65,
    borderRadius: '0 0 8px 8px',
    overflowX: 'auto',
    whiteSpace: 'pre'
  }}>{selected.example}</pre>
              </div>}

            {}
            {selected.docsLink && <a href={selected.docsLink} style={{
    display: 'inline-flex',
    padding: '5px 12px',
    borderRadius: '6px',
    background: 'var(--ce-accent-bg)',
    border: '1px solid var(--ce-accent-border)',
    color: 'var(--ce-accent)',
    fontSize: '12px',
    fontWeight: 600,
    textDecoration: 'none'
  }}>Full docs →</a>}

            {}
            {selected.children && selected.children.length > 0 && <div style={{
    marginTop: '20px'
  }}>
                <div style={{
    fontSize: '11px',
    fontWeight: 700,
    color: 'var(--ce-text-4)',
    textTransform: 'uppercase',
    letterSpacing: '0.4px',
    marginBottom: '8px'
  }}>Contents</div>
                <div style={{
    display: 'flex',
    flexDirection: 'column',
    gap: '4px'
  }}>
                  {selected.children.map(child => <button key={child.id} onClick={() => selectNode(child)} style={{
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
    padding: '6px 8px',
    width: '100%',
    background: 'var(--ce-surface)',
    borderRadius: '6px',
    border: 'none',
    cursor: 'pointer',
    textAlign: 'left',
    transition: 'background 0.1s'
  }} onMouseEnter={e => e.currentTarget.style.background = 'var(--ce-surface-hover)'} onMouseLeave={e => e.currentTarget.style.background = 'var(--ce-surface)'}>
                      {renderIcon(child.icon, child.color, 13)}
                      <span style={{
    fontFamily: 'var(--ce-mono)',
    fontSize: '12px',
    color: 'var(--ce-text-2)'
  }}>{child.label}</span>
                      {child.oneLiner && <span style={{
    fontSize: '11px',
    color: 'var(--ce-text-4)',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    whiteSpace: 'nowrap'
  }}>{child.oneLiner}</span>}
                    </button>)}
                </div>
              </div>}
      </div>
    </div>
    </>;
};

Claude Code は、プロジェクトディレクトリとホームディレクトリの `~/.claude` から、指示、設定、skills、subagents、メモリを読み込みます。プロジェクトファイルを git にコミットしてチームと共有します。`~/.claude` 内のファイルは、すべてのプロジェクトに適用される個人設定です。

Windows では、`~/.claude` は `%USERPROFILE%\.claude` に解決されます。[`CLAUDE_CONFIG_DIR`](/docs/ja/env-vars) を設定した場合、このページのすべての `~/.claude` パスはそのディレクトリの下に存在します。

ほとんどのユーザーは `CLAUDE.md` と `settings.json` のみを編集します。リポジトリに他のコーディングエージェント用の `AGENTS.md` が既にある場合、Claude Code は [それを独立して、または `CLAUDE.md` と一緒に読み込むことができます](/docs/ja/memory#agents-md)。ディレクトリの残りはオプションです。必要に応じて skills、rules、または subagents を追加してください。

<h2 id="explore-the-directory">
  ディレクトリを探索する
</h2>

ツリー内のファイルをクリックして、各ファイルの機能、読み込みタイミング、および例を確認してください。

<ClaudeExplorer />

<h2 id="what’s-not-shown">
  表示されないもの
</h2>

エクスプローラーは、あなたが作成および編集するファイルをカバーしています。いくつかの関連ファイルは他の場所にあります。

| ファイル                    | 場所                                | 目的                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| ----------------------- | --------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `managed-settings.json` | システムレベル、OS によって異なる                | エンタープライズが強制する設定で、[限定的な例外](/docs/ja/settings#security-keys-where-the-stricter-value-applies)を除いてオーバーライドできません。[ファイルの保存場所](/docs/ja/managed-settings#deploy-a-managed-settings-file)と [Claude Code が使用する管理ソース](/docs/ja/managed-settings#precedence-within-the-managed-tier)を参照してください。                                                                                                                                                                                                                                                                                                                                                                                                                |
| `CLAUDE.local.md`       | プロジェクトルート                         | このプロジェクトの個人的な設定で、CLAUDE.md と一緒に読み込まれます。手動で作成し、`.gitignore` に追加してください。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `AGENTS.md`             | プロジェクトルート、`.claude/`、または任意のディレクトリ | AI コーディングエージェント向けに作成するプロジェクト指示。Claude Code は[それを読み込む](/docs/ja/memory#agents-md)ことができます。これは独立して、または `CLAUDE.md` と一緒に読み込まれます。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| インストール済みプラグイン           | `~/.claude/plugins`               | クローンされたマーケットプレイス、インストール済みプラグインバージョン、`installed_plugins.json` インストール記録、およびプラグインごとのデータで、`claude plugin` コマンドで管理されます。プラグイン[あなたの claude.ai アカウントから同期](/docs/ja/plugins/loading#synced-plugins)は `~/.claude/plugins/synced/` にダウンロードされます。リンクモードでマーケットプレイス [`command` ソース](/docs/ja/plugins/marketplace-reference#command-plugin-source)からインストールされたプラグインの場合、Claude Code はコピーの代わりにここにリンクを保存し、プラグインのファイルはコマンドが出力するディレクトリに留まります。`command` ソースには Claude Code v2.1.229 以降が必要です。ローカルディレクトリマーケットプレイスで相対パスでリストされているプラグインも、キャッシュコピーではなく、ソースディレクトリから[その場で読み込まれます](/docs/ja/plugins/loading#find-plugins-on-disk)。[プラグインキャッシング](/docs/ja/plugins/loading#find-plugins-on-disk)を参照して、孤立したバージョンがどのようにクリーンアップされるかを確認してください。 |

`~/.claude` はまた、Claude Code があなたが作業する際に書き込むデータも保持しています。トランスクリプト、プロンプト履歴、ファイルスナップショット、キャッシュ、およびログです。下記の[アプリケーションデータ](#application-data)を参照してください。

<h2 id="choose-the-right-file">
  適切なファイルを選択する
</h2>

異なる種類のカスタマイズは異なるファイルに存在します。このテーブルを使用して、変更がどこに属するかを見つけてください。

| 実行したいこと                           | 編集                                        | スコープ           | リファレンス                                        |
| :-------------------------------- | :---------------------------------------- | :------------- | :-------------------------------------------- |
| Claude にプロジェクトコンテキストと規約を提供する      | `CLAUDE.md`                               | プロジェクトまたはグローバル | [メモリ](/docs/ja/memory)                             |
| 特定のツール呼び出しを許可またはブロックする            | `settings.json` `permissions` または `hooks` | プロジェクトまたはグローバル | [パーミッション](/docs/ja/permissions)、[Hooks](/docs/ja/hooks) |
| ツール呼び出しの前後にスクリプトを実行する             | `settings.json` `hooks`                   | プロジェクトまたはグローバル | [Hooks](/docs/ja/hooks)                            |
| セッションの環境変数を設定する                   | `settings.json` `env`                     | プロジェクトまたはグローバル | [設定](/docs/ja/settings-reference#all-settings)     |
| 個人的なオーバーライドを git から除外する           | `settings.local.json`                     | プロジェクトのみ       | [設定スコープ](/docs/ja/settings#where-settings-live)    |
| `/name` で呼び出すプロンプトまたは機能を追加する      | `skills/<name>/SKILL.md`                  | プロジェクトまたはグローバル | [Skills](/docs/ja/skills)                          |
| 独自のツールを持つ特化した subagent を定義する      | `agents/*.md`                             | プロジェクトまたはグローバル | [Subagents](/docs/ja/sub-agents)                   |
| スクリプトから多くの subagent をオーケストレーションする | `workflows/*.js`                          | プロジェクトまたはグローバル | [動的ワークフロー](/docs/ja/workflows)                     |
| MCP 経由で外部ツールを接続する                 | `.mcp.json`                               | プロジェクトのみ       | [MCP](/docs/ja/mcp)                                |
| Claude がレスポンスをフォーマットする方法を変更する     | `output-styles/*.md`                      | プロジェクトまたはグローバル | [出力スタイル](/docs/ja/output-styles)                   |

<h2 id="file-reference">
  ファイルリファレンス
</h2>

このテーブルは、エクスプローラーがカバーするすべてのファイルをリストしています。プロジェクトスコープのファイルはリポジトリの `.claude/` の下に存在します（`CLAUDE.md`、`.mcp.json`、`.worktreeinclude` はルートにあります）。グローバルスコープのファイルは `~/.claude/` に存在し、すべてのプロジェクトに適用されます。

<Note>
  これらのファイルに入れたものをオーバーライドできるいくつかのことがあります。

  * 組織によってデプロイされた[管理設定](/docs/ja/server-managed-settings)はすべてに優先します。ただし[設定の優先順位の例外](/docs/ja/settings#exceptions-to-managed-settings-precedence)の下の例外を除きます
  * `--permission-mode` や `--settings` などの CLI フラグはそのセッションの `settings.json` をオーバーライドします
  * 一部の環境変数は同等の設定に優先しますが、これは異なります。各設定について[環境変数リファレンス](/docs/ja/env-vars)を確認してください

  完全な順序については[設定の優先順位](/docs/ja/settings#settings-precedence)を参照してください。
</Note>

ファイル名をクリックして、上記のエクスプローラーでそのノードを開きます。

| ファイル                                                | スコープ           | コミット | 機能                                                                        | リファレンス                                                          |
| --------------------------------------------------- | -------------- | ---- | ------------------------------------------------------------------------- | --------------------------------------------------------------- |
| [`CLAUDE.md`](#ce-claude-md)                        | プロジェクトおよびグローバル | ✓    | 毎セッション読み込まれる指示                                                            | [メモリ](/docs/ja/memory)                                               |
| [`rules/*.md`](#ce-rules)                           | プロジェクトおよびグローバル | ✓    | トピックスコープの指示、オプションでパスゲート                                                   | [ルール](/docs/ja/memory#organize-rules-with-claude/rules/)             |
| [`settings.json`](#ce-settings-json)                | プロジェクトおよびグローバル | ✓    | 権限、hooks、環境変数、モデルデフォルト                                                    | [設定](/docs/ja/settings)                                              |
| [`settings.local.json`](#ce-settings-local-json)    | プロジェクトのみ       |      | 個人的なオーバーライド、Claude Code が設定を保存するときに gitignore                             | [設定スコープ](/docs/ja/settings#where-settings-live)                      |
| [`.mcp.json`](#ce-mcp-json)                         | プロジェクトのみ       | ✓    | チーム共有 MCP サーバー                                                            | [MCP スコープ](/docs/ja/mcp#mcp-installation-scopes)                     |
| [`.worktreeinclude`](#ce-worktreeinclude)           | プロジェクトのみ       | ✓    | 新しい worktrees にコピーする gitignore ファイル                                       | [Worktrees](/docs/ja/worktrees#copy-gitignored-files-into-worktrees) |
| [`skills/<name>/SKILL.md`](#ce-skills)              | プロジェクトおよびグローバル | ✓    | `/name` で呼び出される、または自動呼び出される再利用可能なプロンプト                                    | [Skills](/docs/ja/skills)                                            |
| [`commands/*.md`](#ce-commands)                     | プロジェクトおよびグローバル | ✓    | シングルファイルプロンプト。skills と同じメカニズム                                             | [Skills](/docs/ja/skills)                                            |
| [`output-styles/*.md`](#ce-output-styles)           | プロジェクトおよびグローバル | ✓    | Claude の動作方法を調整するカスタム指示セット                                                | [出力スタイル](/docs/ja/output-styles)                                     |
| [`agents/*.md`](#ce-agents)                         | プロジェクトおよびグローバル | ✓    | 独自のプロンプトとツールを持つ subagent 定義                                               | [Subagents](/docs/ja/sub-agents)                                     |
| [`workflows/*.js`](#ce-workflows)                   | プロジェクトおよびグローバル | ✓    | Claude によって書かれた動的ワークフロースクリプト、`/workflows` から保存。各ファイルは `/<name>` コマンドになります | [動的ワークフロー](/docs/ja/workflows)                                       |
| [`agent-memory/<name>/`](#ce-agent-memory)          | プロジェクトおよびグローバル | ✓    | subagents の永続メモリ                                                          | [永続メモリ](/docs/ja/sub-agents#enable-persistent-memory)                |
| [`~/.claude.json`](#ce-claude-json)                 | グローバルのみ        |      | アプリ状態、OAuth、UI トグル、個人 MCP サーバー                                            | [グローバル設定](/docs/ja/settings-reference#global-config-settings)        |
| [`projects/<project>/memory/`](#ce-global-projects) | グローバルのみ        |      | Auto memory：Claude のセッション間のメモ                                             | [Auto memory](/docs/ja/memory#auto-memory)                           |
| [`keybindings.json`](#ce-keybindings)               | グローバルのみ        |      | カスタムキーボードショートカット                                                          | [キーバインディング](/docs/ja/keybindings)                                    |
| [`themes/*.json`](#ce-themes)                       | グローバルのみ        |      | カスタムカラーテーマ                                                                | [カスタムテーマ](/docs/ja/terminal-config#create-a-custom-theme)            |

<h2 id="frontmatter-fields-by-file">
  ファイル別フロントマターフィールド
</h2>

スキル、コマンドファイル、サブエージェント、出力スタイル、およびルールは、ファイルの上部にある YAML [フロントマター](/docs/ja/glossary#frontmatter)から設定を読み込み、それぞれが独自のフィールドセットを受け入れます。このテーブルは、各ファイルのフィールド名をリストアップし、それらを説明するリファレンスへのリンクを提供します。

| ファイル                     | フロントマターフィールド                                                                                                                                                                                                                                                                  | リファレンス                                                         |
| ------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------- |
| `skills/<name>/SKILL.md` | `name`, `description`, `when_to_use`, `argument-hint`, `arguments`, `disable-model-invocation`, `user-invocable`, `allowed-tools`, `disallowed-tools`, `model`, `effort`, `context`, `agent`, `background`, `hooks`, `paths`, `shell`, `metadata`, `license`, `compatibility` | [スキルフロントマター](/docs/ja/skills#frontmatter-reference)                 |
| `commands/*.md`          | `name` と `paths` を除くスキルフィールド                                                                                                                                                                                                                                                  | [スキルフロントマター](/docs/ja/skills#frontmatter-reference)                 |
| `agents/*.md`            | `name`, `description`, `tools`, `disallowedTools`, `model`, `permissionMode`, `maxTurns`, `skills`, `mcpServers`, `hooks`, `memory`, `background`, `effort`, `isolation`, `color`, `initialPrompt`, `omitClaudeMd`, `experimental`                                            | [サブエージェントフロントマター](/docs/ja/sub-agents#supported-frontmatter-fields) |
| `output-styles/*.md`     | `name`, `description`, `keep-coding-instructions`, `force-for-plugin`                                                                                                                                                                                                         | [出力スタイルフロントマター](/docs/ja/output-styles#frontmatter)                 |
| `rules/*.md`             | `paths`                                                                                                                                                                                                                                                                       | [ルールフロントマター](/docs/ja/memory#rules-frontmatter-reference)           |

[プラグイン](/docs/ja/plugins/components#agents)に含まれるエージェントは、サブエージェントフィールドのサブセットに対応しています。

<h2 id="troubleshoot-configuration">
  設定をトラブルシューティングする
</h2>

設定、hook、またはファイルが有効になっていない場合は、[設定をデバッグする](/docs/ja/debug-your-config)を参照して、検査コマンドと症状優先ルックアップテーブルを確認してください。

<h2 id="application-data">
  アプリケーションデータ
</h2>

作成したコンフィグ以外に、`~/.claude` には Claude Code がセッション中に書き込むデータが保存されます。これらのファイルはプレーンテキストです。ツールを通過するすべてのものは、ディスク上のトランスクリプトに書き込まれます。ファイルの内容、コマンド出力、貼り付けたテキストなどです。

<h3 id="cleaned-up-automatically">
  自動的にクリーンアップされるもの
</h3>

Claude Code は、[`cleanupPeriodDays`](/docs/ja/settings-reference#cleanupperioddays) より古いファイルを以下のパスから削除します。ただし、保持期間を安全に判定できる場合に限ります。デフォルトは 30 日で、最小値は 1 です。`0` に設定するとバリデーションエラーが発生します。同じ経過日数の閾値が、[孤立した worktrees](/docs/ja/worktrees#clean-up-subagent-and-background-session-worktrees) の自動削除にも適用されます。

| `~/.claude/` 下のパス                                                                                                              | 内容                                                                                                                                                                                                                          |
| ------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `projects/<project>/<session>.jsonl`                                                                                           | 完全な会話トランスクリプト：すべてのメッセージ、ツール呼び出し、ツール結果                                                                                                                                                                                       |
| `projects/<project>/<session>.orphaned-<timestamp>-<suffix>.jsonl`、`projects/<project>/<session>.jsonl.superseded-<timestamp>` | Claude Code が上書きまたは削除する代わりに脇に置いたセッションの前のトランスクリプト。セッションピッカーには表示されません                                                                                                                                                         |
| `projects/<project>/<session>/subagents/`                                                                                      | [Subagent](/docs/ja/sub-agents) の会話トランスクリプト。親セッションのトランスクリプトが経過時間で削除されるときに削除されます                                                                                                                                                  |
| `projects/<project>/<session>/tool-results/`                                                                                   | 別ファイルにこぼれた大きなツール出力                                                                                                                                                                                                          |
| `file-history/<session>/`                                                                                                      | Claude Code が変更したファイルの編集前スナップショット。[チェックポイント復元](/docs/ja/checkpointing) に使用されます。最新 100 個のチェックポイントのスナップショットを保持します。保持されているチェックポイントが参照していないスナップショットファイルは削除されます。ただし、各ファイルの最初のスナップショットは除きます                                            |
| `plans/`                                                                                                                       | [プランモード](/docs/ja/permission-modes#analyze-before-you-edit-with-plan-mode) 中に書き込まれたプランファイル                                                                                                                                       |
| `debug/`                                                                                                                       | セッションごとのデバッグログ。デバッグログが有効な場合に書き込まれます。例えば、[`--debug`](/docs/ja/cli-reference#cli-flags) で起動するか `/debug` を実行する場合です                                                                                                                  |
| `paste-cache/`                                                                                                                 | 大きな貼り付けの内容                                                                                                                                                                                                                  |
| `image-cache/<session>/`                                                                                                       | Claude Code v2.1.274 以前によって保存された添付画像。以降のバージョンは、貼り付けられた画像と添付された画像を `~/.claude` の外に保存します。[`CLAUDE_CODE_TMPDIR`](/docs/ja/env-vars) が制御する temp ディレクトリの下の各セッションの `images/` ディレクトリに保存されます。スイープは他のセッションの残されたディレクトリをここから削除します。経過時間に関係なく |
| `uploads/<session>/`                                                                                                           | Web またはモバイルアプリから添付したファイル、およびモバイルアプリから添付した写真。[Remote Control](/docs/ja/remote-control) セッションにメッセージを送信する場合です。[クラウドセッション](/docs/ja/claude-code-on-the-web) への添付は、代わりにそのセッション自体のクラウド環境に保存され、マシン上には保存されません                                 |
| `session-env/`                                                                                                                 | セッションごとの環境メタデータ                                                                                                                                                                                                             |
| `tasks/`                                                                                                                       | タスクツールによって書き込まれたタスクリスト。リストごとに 1 つのディレクトリ                                                                                                                                                                                    |
| `shell-snapshots/`                                                                                                             | 起動時にキャプチャされたエイリアス、関数、シェルオプション。[Bash ツール](/docs/ja/tools-reference#bash-tool-behavior) によって各コマンドに適用されます。クリーンな終了時に削除されます。スイープはクラッシュ後に残されたものをクリアします                                                                                 |
| `backups/`                                                                                                                     | `~/.claude.json` の以前のバージョン。Claude Code がファイルを書き直すときにコピーされます。Claude Code は最新の 5 つと、解析できなかったバージョンのコピーを保持します                                                                                                                   |
| `feedback-bundles/`                                                                                                            | `/feedback` によってサードパーティプロバイダーに書き込まれた、または Anthropic 認証情報が設定されていない場合に書き込まれた、編集済みトランスクリプトアーカイブ。Anthropic アカウントチームに送信するためのものです                                                                                                  |
| `feedback/drafts/`                                                                                                             | キューに入った [Claude が作成したフィードバック](/docs/ja/tools-reference#sendfeedback-tool-behavior)。`/feedback` でのレビューを待機中です。`cleanupPeriodDays` または 30 日のいずれか短い方の後にスイープされます。キューが 10 ドラフトの上限に達すると、Claude Code は最も古いドラフトを削除して場所を作ります               |
| `usage-data/`                                                                                                                  | [`/insights`](/docs/ja/costs#analyze-your-usage-patterns) によって書き込まれた `report.html` とタイムスタンプ付きレポートコピー。それらを構築するために使用されるキャッシュされたセッションごとの分析データ                                                                                       |
| `skills/.trash/`、`plugins/.trash/`                                                                                             | claude.ai から同期された [Skills](/docs/ja/skills#how-synced-skills-behave) と [plugins](/docs/ja/plugins/loading#synced-plugins)。Claude Code が削除したもの。削除する代わりにここに移動されるため、ファイルを復元できます                                                          |
| `todos/`、`statsig/`、`logs/`                                                                                                    | 古いバージョンのレガシーディレクトリ。現在は書き込まれていません。スイープはその内容を削除してから空のディレクトリを削除します                                                                                                                                                             |

`sessions/` のセッションファイル、自動メモリ、Claude Desktop および Cowork トランスクリプトは、それぞれ独自の保持ルールに従います：

* **`sessions/`**：実行中の各セッションに 1 つの小さなファイルを保持します。同時セッションとクラッシュを検出するために使用されます。経過時間ベースのスイープの対象ではありません。Claude Code はセッションが終了するときに各ファイルを削除し、次の起動時にクラッシュの残骸をクリアします。
* **自動メモリ**：スイープはプロジェクトの [自動メモリ](/docs/ja/memory#auto-memory) ディレクトリ `projects/<project>/memory/` 内のメモリファイルを削除しません。Claude Code はそのディレクトリを削除するのは、保持期間全体が空だった場合のみです。v2.1.228 より前では、スイープはメモリディレクトリ内のフォルダをセッションデータとして扱い、その下の古いファイルを削除できました。
* **Claude Desktop および Cowork トランスクリプト**：Claude Code は Claude Desktop または Cowork で開始または最後に続行したセッションのトランスクリプトを任意の経過時間で保持します。これらのトランスクリプトに経過時間の制限を設定するには、[`desktopSessionCleanupPeriodDays`](/docs/ja/settings-reference#desktopsessioncleanupperioddays) を設定します。[管理設定](/docs/ja/managed-settings) が `cleanupPeriodDays` を設定する場合、Claude Code はその期間後にこれらのトランスクリプトを削除します。Claude Code v2.1.248 以降が必要です。以前のバージョンは `cleanupPeriodDays` の後に削除します。

Claude Code は以下の場合、経過時間ベースのスイープをスキップします：

* **ベアモード**：[`--bare`](/docs/ja/headless#start-faster-with-bare-mode) で `claude -p` を実行する場合、Claude Code はそのセッションでスイープを実行しません。
* **一時停止されたスイープ**：Claude Code が保持期間を安全に判定できない場合、保持クリーンアップスイープを一時停止します。[`retention_sweep` イベント](/docs/ja/monitoring-usage#retention-sweep-event) は、それを一時停止する各設定をリストします。原因が読み込みまたは解析できない設定ファイル、または `cleanupPeriodDays` または `desktopSessionCleanupPeriodDays` が明示的に設定された設定エラーの場合、Claude Code は設定エラーを修正するまで `/status` に警告も表示します。[管理設定](/docs/ja/server-managed-settings) が `cleanupPeriodDays` を提供する場合、Claude Code はどちらの場合でも管理値でスイープを実行します。

<h3 id="kept-until-you-delete-them">
  削除するまで保持されるもの
</h3>

保持クリーンアップスイープは以下のパスを削除しません。Claude Code はそれらを削除するまで保持します。ただし、ログアウト時に削除する 2 つのキャッシュは除きます。

| `~/.claude/` 下のパス      | 内容                                                                                                                                                                                                                     |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `history.jsonl`        | 入力したすべてのプロンプト。タイムスタンプとプロジェクトパス付き。上矢印リコール、`Ctrl+R` 履歴検索、`!` シェルコマンド補完に使用されます                                                                                                                                            |
| `stats-cache.json`     | `/usage` で表示される集計トークンおよびコスト数                                                                                                                                                                                           |
| `remote-settings.json` | [サーバー管理設定](/docs/ja/server-managed-settings) のキャッシュコピー。組織用。または、組織が何も設定していない場合は `{}`。セッションが [それらを取得](/docs/ja/server-managed-settings#platform-availability) する場合にのみ存在します。Claude Code は起動時と、セッション中は 1 時間ごとに更新を確認します。ログアウト時に削除されます |
| `cache/changelog.md`   | Claude Code チェンジログのキャッシュコピー。`/release-notes` で表示されます。バックグラウンドで更新されます                                                                                                                                                   |
| `policy-limits.json`   | 組織の機能ポリシー設定のキャッシュ。一部のアカウントタイプにのみ存在します。自動的に更新されます。`policy-limits.json.stamp.json` サイドカーは、キャッシュがどのアカウントまたは API キーに属しているかを記録します。Claude Code はログアウト時に両方のファイルを削除します                                                         |

<span id="state-files-to-keep" />

使用する機能に応じて、他のファイルが表示されます。キャッシュとロックファイルは削除しても安全です。これらの状態ファイルを保持してください：

* `.credentials.json`：[ログイン認証情報](/docs/ja/authentication#credential-management)
* `agent-memory/`：[subagent メモリ](/docs/ja/sub-agents#enable-persistent-memory)
* `jobs/` および `daemon/`：[バックグラウンドセッション](/docs/ja/agent-view#where-state-is-stored) 状態

<h3 id="plaintext-storage">
  プレーンテキストストレージ
</h3>

トランスクリプトと履歴は保存時に暗号化されません。OS ファイル権限のみが保護です。ツールが `.env` ファイルを読み込むか、コマンドが認証情報を出力する場合、その値は `projects/<project>/<session>.jsonl` に書き込まれます。露出を減らすには：

* `cleanupPeriodDays` を低くして、Claude Code がトランスクリプトを保持する期間を短縮します
* [`desktopSessionCleanupPeriodDays`](/docs/ja/settings-reference#desktopsessioncleanupperioddays) を設定して、Claude Desktop および Cowork トランスクリプトにも経過時間の制限を設定します
* [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/ja/env-vars) 環境変数を設定して、任意のモードでトランスクリプトとプロンプト履歴の書き込みをスキップします。非対話モードでは、代わりに `-p` と一緒に `--no-session-persistence` を渡すか、TypeScript Agent SDK で `persistSession: false` を設定できます。Python SDK には同等のオプションはありません。
* [権限ルール](/docs/ja/permissions) を使用して、認証情報ファイルの読み込みを拒否します

<h3 id="clear-local-data">
  ローカルデータをクリア
</h3>

`claude project purge` を実行して、Claude Code が 1 つのプロジェクトに対して保持する状態を削除します。以下を削除します：

* `projects/` 下のトランスクリプトと自動メモリ
* セッションごとの `tasks/`、`debug/`、`file-history/` エントリ
* `history.jsonl` の一致するプロンプト行
* `~/.claude.json` のプロジェクトエントリ

プロジェクトのセッションで貼り付けたまたは添付した画像は、`~/.claude` ではなく Claude Code の temp ディレクトリに保存されるため、パージはそれらを削除しません。[保持クリーンアップスイープ](#cleaned-up-automatically) は、`cleanupPeriodDays` より古くなると削除します。

コマンドは完全な削除計画を出力し、何かを削除する前に確認を求めます。

以下の例は、プレースホルダーとして `~/work/my-repo` を使用しています。プロジェクトへのパスに置き換えてください。パスに一致する状態がない場合、コマンドはエラーを出力して終了ステータス 1 で終了します。

何も削除せずに計画をプレビューします：

```bash theme={null}
claude project purge ~/work/my-repo --dry-run
```

計画は各一致項目とそれが含まれる理由をリストします：

```text theme={null}
Purge plan for /home/user/work/my-repo:

  dir:    /home/user/.claude/projects/-home-user-work-my-repo
           project transcripts (.jsonl) and memory/
  config: projects["/home/user/work/my-repo"]
           project entry in ~/.claude.json (trust, history, MCP servers)
  filter: /home/user/.claude/history.jsonl
           12 prompt(s) typed in this project

shell-snapshots/ are not project-scoped and will not be touched
backups/ may still contain this project entry in old .claude.json snapshots (/home/user/.claude/backups); at most 5 are kept and they rotate out automatically
Dry run: 3 item(s) would be deleted.
```

単一の確認プロンプトで削除します：

```bash theme={null}
claude project purge ~/work/my-repo
```

コマンドは同じ計画を出力してから、`Delete 3 item(s) for /home/user/work/my-repo? This cannot be undone. [y/N]` と尋ね、`y` と答えた場合のみ削除します。

パスを省略して、対話的なリストからプロジェクトを選択します。

スクリプトで使用するために確認プロンプトをスキップします：

```bash theme={null}
claude project purge ~/work/my-repo --yes
```

パスの代わりに `--all` を渡して、すべてのプロジェクトの状態を一度にパージします。これは `history.jsonl` をフィルタリングするのではなく完全に削除します。`-i` を渡して、削除計画を 1 つずつステップスルーします。

コマンドは `shell-snapshots/` と `backups/` をそのままにしておきます。これらはプロジェクトスコープではないため、計画出力で警告します。

[保持する状態ファイル](#state-files-to-keep) を除いて、上記のアプリケーションデータパスのいずれかを手動で削除することもできます。新しいセッションは影響を受けません。以下の表は、過去のセッションで失うものを示しています。

| 削除                                                                                                               | 失うもの                                                                                                                         |
| ---------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| `~/.claude/projects/`                                                                                            | 過去のセッションの再開、続行、巻き戻し、およびすべてのプロジェクトの自動メモリ                                                                                      |
| `~/.claude/history.jsonl`                                                                                        | 上矢印プロンプトリコール、`Ctrl+R` 履歴検索、`!` シェルコマンド補完                                                                                     |
| `~/.claude/paste-cache/`                                                                                         | リコールされたプロンプトの貼り付けたテキスト。[大きなコンテンツを貼り付ける](/docs/ja/terminal-config#paste-large-content) を参照してください                                   |
| `~/.claude/uploads/`                                                                                             | 過去の [Remote Control](/docs/ja/remote-control) セッションがパスで参照する添付ファイル                                                                 |
| `~/.claude/file-history/`                                                                                        | 過去のセッションのチェックポイント復元                                                                                                          |
| `~/.claude/stats-cache.json`                                                                                     | `/usage` で表示される履歴合計                                                                                                          |
| `~/.claude/usage-data/`                                                                                          | 過去の [`/insights`](/docs/ja/costs#analyze-your-usage-patterns) レポートと、それらを構築するために使用されたキャッシュされた分析データ                                 |
| `~/.claude/feedback-bundles/`                                                                                    | Anthropic アカウントチームにまだ送信していないフィードバックとバグレポートアーカイブ                                                                              |
| `~/.claude/feedback/drafts/`                                                                                     | 送信していない [Claude が作成したフィードバック](/docs/ja/tools-reference#sendfeedback-tool-behavior)                                                |
| `~/.claude/remote-settings.json`                                                                                 | なし。次の起動時に再取得されます                                                                                                             |
| `~/.claude/cache/changelog.md`                                                                                   | なし。バックグラウンドで更新されます                                                                                                           |
| `~/.claude/policy-limits.json`                                                                                   | なし。自動的に更新されます                                                                                                                |
| `~/.claude/tasks/`                                                                                               | 再開されたセッションが取得するタスクリスト                                                                                                        |
| `~/.claude/skills/.trash/`、`~/.claude/plugins/.trash/`                                                           | Claude Code が削除した [同期されたスキル](/docs/ja/skills#how-synced-skills-behave) と [同期されたプラグイン](/docs/ja/plugins/loading#synced-plugins) を復元する機会 |
| `~/.claude/debug/`、`~/.claude/plans/`、`~/.claude/session-env/`、`~/.claude/shell-snapshots/`、`~/.claude/backups/` | ユーザー向けのなし                                                                                                                    |
| `~/.claude/todos/`、`~/.claude/statsig/`、`~/.claude/logs/`、`~/.claude/image-cache/`                               | なし。現在のバージョンで書き込まれていないレガシーディレクトリ                                                                                              |

`~/.claude.json`、`~/.claude/settings.json`、`~/.claude/plugins/` は削除しないでください。これらは認証、設定、インストール済みプラグインを保持しています。

<h2 id="related-resources">
  関連リソース
</h2>

* [Claude のメモリを管理する](/docs/ja/memory)：CLAUDE.md、rules、auto memory を書き込んで整理します
* [設定を構成する](/docs/ja/settings)：パーミッション、hooks、環境変数、モデルデフォルトを設定します
* [Skills を作成する](/docs/ja/skills)：再利用可能なプロンプトとワークフローを構築します
* [Subagents を構成する](/docs/ja/sub-agents)：独自のコンテキストを持つ特化したエージェントを定義します
