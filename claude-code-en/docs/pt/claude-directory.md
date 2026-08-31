> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Explore o diretório .claude

> Onde Claude Code lê CLAUDE.md, settings.json, hooks, skills, commands, subagents, workflows, rules e auto memory. Explore o diretório .claude em seu projeto e ~/.claude em seu diretório home.

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
        tips: ['Target under 200 lines. Longer files still load in full but may reduce adherence', <>CLAUDE.md loads into every session. If something only matters for specific tasks, move it to a <A href="/docs/en/skills">skill</A> or a path-scoped <A href="/docs/en/memory#organize-rules-with-claude/rules/">rule</A> so it loads only when needed</>, 'List the commands you run most, like build, test, and format, so Claude knows them without you spelling them out each time', <>Run <C>/memory</C> to open and edit CLAUDE.md from within a session</>, <>Also works at <C>.claude/CLAUDE.md</C> if you prefer to keep the project root clean</>],
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
          contains: [<><A href="/docs/en/permissions">permissions</A>: allow, deny, or prompt before Claude uses specific tools or commands</>, <><A href="/docs/en/hooks">hooks</A>: run your own scripts on events like before a tool call or after a file edit</>, <><A href="/docs/en/statusline">statusLine</A>: customize the line shown at the bottom while Claude works</>, <><A href="/docs/en/settings-reference#available-settings">model</A>: pick a default model for this project</>, <><A href="/docs/en/settings-reference#environment-variables">env</A>: environment variables set in every session</>, <><A href="/docs/en/output-styles">outputStyle</A>: select a custom system-prompt style from output-styles/</>],
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
          when: 'Applied at session start when selected via the outputStyle setting',
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
          oneLiner: 'Custom system-prompt sections that adjust how Claude works',
          when: 'Applied at session start when selected via the outputStyle setting',
          description: [<>Each markdown file defines an output style: a section appended to the system prompt that, by default, also drops the built-in software-engineering task instructions. Use this to adapt Claude Code for uses beyond coding, or to add teaching or review modes.</>, <>Select a built-in or custom style with <C>/config</C> or the <C>outputStyle</C> key in settings. Styles here are available in every project; project-level styles with the same name take precedence.</>],
          tips: ['Built-in styles Default, Proactive, Concise, Explanatory, and Learning are included with Claude Code; custom styles go here', <>Set <C>keep-coding-instructions: true</C> in frontmatter to keep the default task instructions alongside your additions</>, 'Changes take effect on the next session since the system prompt is fixed at startup for caching'],
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
            description: <>This style appends instructions to the system prompt: Claude adds a "Why this approach" note after each task and leaves TODO(human) markers for changes under 10 lines instead of writing them itself. Select it by setting <C>outputStyle</C> to the filename without .md, or to the <C>name</C> field if you set one in frontmatter.</>,
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

Claude Code lê instruções, configurações, skills, subagents e memória do seu diretório de projeto e de `~/.claude` em seu diretório home. Confirme arquivos de projeto no git para compartilhá-los com sua equipe; arquivos em `~/.claude` são configuração pessoal que se aplica em todos os seus projetos.

No Windows, `~/.claude` é resolvido para `%USERPROFILE%\.claude`. Se você definir [`CLAUDE_CONFIG_DIR`](/docs/pt/env-vars), cada caminho `~/.claude` nesta página fica sob esse diretório.

A maioria dos usuários apenas edita `CLAUDE.md` e `settings.json`. O resto do diretório é opcional: adicione skills, rules ou subagents conforme necessário.

<h2 id="explore-the-directory">
  Explore o diretório
</h2>

Clique em arquivos na árvore para ver o que cada um faz, quando carrega e um exemplo.

<ClaudeExplorer />

<h2 id="what’s-not-shown">
  O que não é mostrado
</h2>

O explorador cobre arquivos que você cria e edita. Alguns arquivos relacionados vivem em outro lugar:

| Arquivo                 | Localização                    | Propósito                                                                                                                                                                                                                                                                                 |
| ----------------------- | ------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `managed-settings.json` | Nível do sistema, varia por SO | Configurações impostas pela empresa que você não pode substituir. Veja [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings).                                                                                                                                            |
| `CLAUDE.local.md`       | Raiz do projeto                | Suas preferências privadas para este projeto, carregadas junto com CLAUDE.md. Crie manualmente e adicione a `.gitignore`.                                                                                                                                                                 |
| Plugins instalados      | `~/.claude/plugins`            | Marketplaces clonados, versões de plugins instalados e dados por plugin, gerenciados por comandos `claude plugin`. Versões órfãs são deletadas 7 dias após uma atualização ou desinstalação de plugin. Veja [cache de plugins](/docs/pt/plugins-reference#plugin-caching-and-file-resolution). |

`~/.claude` também contém dados que Claude Code escreve conforme você trabalha: transcrições, histórico de prompts, snapshots de arquivos, caches e logs. Veja [dados da aplicação](#application-data) abaixo.

<h2 id="choose-the-right-file">
  Escolha o arquivo certo
</h2>

Diferentes tipos de personalização vivem em arquivos diferentes. Use esta tabela para encontrar onde uma mudança pertence.

| Você quer                                                       | Editar                                   | Escopo            | Referência                                              |
| :-------------------------------------------------------------- | :--------------------------------------- | :---------------- | :------------------------------------------------------ |
| Dar a Claude contexto e convenções do projeto                   | `CLAUDE.md`                              | projeto ou global | [Memory](/docs/pt/memory)                                    |
| Permitir ou bloquear chamadas de ferramentas específicas        | `settings.json` `permissions` ou `hooks` | projeto ou global | [Permissions](/docs/pt/permissions), [Hooks](/docs/pt/hooks)      |
| Executar um script antes ou depois de chamadas de ferramentas   | `settings.json` `hooks`                  | projeto ou global | [Hooks](/docs/pt/hooks)                                      |
| Definir variáveis de ambiente para a sessão                     | `settings.json` `env`                    | projeto ou global | [Settings](/docs/pt/settings#available-settings)             |
| Manter substituições pessoais fora do git                       | `settings.local.json`                    | apenas projeto    | [Escopos de configurações](/docs/pt/settings#settings-files) |
| Adicionar um prompt ou capacidade que você invoca com `/name`   | `skills/<name>/SKILL.md`                 | projeto ou global | [Skills](/docs/pt/skills)                                    |
| Definir um subagent especializado com suas próprias ferramentas | `agents/*.md`                            | projeto ou global | [Subagents](/docs/pt/sub-agents)                             |
| Orquestrar muitos subagents a partir de um script               | `workflows/*.js`                         | projeto ou global | [Dynamic workflows](/docs/pt/workflows)                      |
| Conectar ferramentas externas sobre MCP                         | `.mcp.json`                              | apenas projeto    | [MCP](/docs/pt/mcp)                                          |
| Mudar como Claude formata respostas                             | `output-styles/*.md`                     | projeto ou global | [Output styles](/docs/pt/output-styles)                      |

<h2 id="file-reference">
  Referência de arquivos
</h2>

Esta tabela lista cada arquivo que o explorador cobre. Arquivos de escopo de projeto vivem em seu repositório sob `.claude/` (ou na raiz para `CLAUDE.md`, `.mcp.json` e `.worktreeinclude`). Arquivos de escopo global vivem em `~/.claude/` e se aplicam em todos os projetos.

<Note>
  Várias coisas podem substituir o que você coloca nesses arquivos:

  * [Configurações gerenciadas](/docs/pt/server-managed-settings) implantadas por sua organização têm precedência sobre tudo
  * Flags CLI como `--permission-mode` ou `--settings` substituem `settings.json` para essa sessão
  * Algumas variáveis de ambiente têm precedência sobre sua configuração equivalente, mas isso varia: verifique a [referência de variáveis de ambiente](/docs/pt/env-vars) para cada uma

  Veja [precedência de configurações](/docs/pt/settings#settings-precedence) para a ordem completa.
</Note>

Clique em um nome de arquivo para abrir esse nó no explorador acima.

| Arquivo                                             | Escopo           | Confirmar | O que faz                                                                                                              | Referência                                                        |
| --------------------------------------------------- | ---------------- | --------- | ---------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| [`CLAUDE.md`](#ce-claude-md)                        | Projeto e global | ✓         | Instruções carregadas a cada sessão                                                                                    | [Memory](/docs/pt/memory)                                              |
| [`rules/*.md`](#ce-rules)                           | Projeto e global | ✓         | Instruções com escopo de tópico, opcionalmente com gate de caminho                                                     | [Rules](/docs/pt/memory#organize-rules-with-claude/rules/)             |
| [`settings.json`](#ce-settings-json)                | Projeto e global | ✓         | Permissões, hooks, variáveis de env, padrões de modelo                                                                 | [Settings](/docs/pt/settings)                                          |
| [`settings.local.json`](#ce-settings-local-json)    | Apenas projeto   |           | Suas substituições pessoais, auto-gitignored                                                                           | [Escopos de configurações](/docs/pt/settings#settings-files)           |
| [`.mcp.json`](#ce-mcp-json)                         | Apenas projeto   | ✓         | Servidores MCP compartilhados pela equipe                                                                              | [Escopos MCP](/docs/pt/mcp#mcp-installation-scopes)                    |
| [`.worktreeinclude`](#ce-worktreeinclude)           | Apenas projeto   | ✓         | Arquivos gitignored para copiar em novos worktrees                                                                     | [Worktrees](/docs/pt/worktrees#copy-gitignored-files-into-worktrees)   |
| [`skills/<name>/SKILL.md`](#ce-skills)              | Projeto e global | ✓         | Prompts reutilizáveis invocados com `/name` ou auto-invocados                                                          | [Skills](/docs/pt/skills)                                              |
| [`commands/*.md`](#ce-commands)                     | Projeto e global | ✓         | Prompts de arquivo único; mesmo mecanismo que skills                                                                   | [Skills](/docs/pt/skills)                                              |
| [`output-styles/*.md`](#ce-output-styles)           | Projeto e global | ✓         | Seções de prompt do sistema personalizadas                                                                             | [Output styles](/docs/pt/output-styles)                                |
| [`agents/*.md`](#ce-agents)                         | Projeto e global | ✓         | Definições de subagents com seu próprio prompt e ferramentas                                                           | [Subagents](/docs/pt/sub-agents)                                       |
| [`workflows/*.js`](#ce-workflows)                   | Projeto e global | ✓         | Scripts de workflow dinâmicos escritos por Claude e salvos de `/workflows`; cada arquivo se torna um comando `/<name>` | [Dynamic workflows](/docs/pt/workflows)                                |
| [`agent-memory/<name>/`](#ce-agent-memory)          | Projeto e global | ✓         | Memória persistente para subagents                                                                                     | [Memória persistente](/docs/pt/sub-agents#enable-persistent-memory)    |
| [`~/.claude.json`](#ce-claude-json)                 | Apenas global    |           | Estado da aplicação, OAuth, toggles de UI, servidores MCP pessoais                                                     | [Configuração global](/docs/pt/settings#global-config-settings)        |
| [`projects/<project>/memory/`](#ce-global-projects) | Apenas global    |           | Auto memory: notas de Claude para si mesmo entre sessões                                                               | [Auto memory](/docs/pt/memory#auto-memory)                             |
| [`keybindings.json`](#ce-keybindings)               | Apenas global    |           | Atalhos de teclado personalizados                                                                                      | [Keybindings](/docs/pt/keybindings)                                    |
| [`themes/*.json`](#ce-themes)                       | Apenas global    |           | Temas de cores personalizados                                                                                          | [Temas personalizados](/docs/pt/terminal-config#create-a-custom-theme) |

<h2 id="troubleshoot-configuration">
  Solucione problemas de configuração
</h2>

Se uma configuração, hook ou arquivo não está tendo efeito, veja [Debug your configuration](/docs/pt/debug-your-config) para os comandos de inspeção e uma tabela de busca por sintoma.

<h2 id="application-data">
  Dados da aplicação
</h2>

Além da configuração que você cria, `~/.claude` contém dados que Claude Code escreve durante sessões. Esses arquivos são texto simples. Qualquer coisa que passa por uma ferramenta aterrissa em uma transcrição no disco: conteúdo de arquivos, saída de comando, texto colado.

<h3 id="cleaned-up-automatically">
  Limpos automaticamente
</h3>

Arquivos nos caminhos abaixo são deletados na inicialização uma vez que têm mais de [`cleanupPeriodDays`](/docs/pt/settings#available-settings). O padrão é 30 dias.

| Caminho sob `~/.claude/`                     | Conteúdo                                                                                                                                                                                                                                                                                        |
| -------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `projects/<project>/<session>.jsonl`         | Transcrição de conversa completa: cada mensagem, chamada de ferramenta e resultado de ferramenta                                                                                                                                                                                                |
| `projects/<project>/<session>/subagents/`    | Transcrições de conversa de [subagent](/docs/pt/sub-agents), removidas com a transcrição de sessão pai quando envelhecem                                                                                                                                                                             |
| `projects/<project>/<session>/tool-results/` | Grandes saídas de ferramentas derramadas em arquivos separados                                                                                                                                                                                                                                  |
| `file-history/<session>/`                    | Snapshots pré-edição de arquivos que Claude alterou, usados para [restauração de checkpoint](/docs/pt/checkpointing). Mantém snapshots para os 100 checkpoints mais recentes; arquivos de snapshot que nenhum checkpoint retido referencia são deletados, exceto o primeiro snapshot de cada arquivo |
| `plans/`                                     | Arquivos de plano escritos durante [plan mode](/docs/pt/permission-modes#analyze-before-you-edit-with-plan-mode)                                                                                                                                                                                     |
| `debug/`                                     | Logs de debug por sessão, escritos apenas quando você inicia com `--debug` ou executa `/debug`                                                                                                                                                                                                  |
| `paste-cache/`, `image-cache/`               | Conteúdo de pastes grandes e imagens anexadas                                                                                                                                                                                                                                                   |
| `session-env/`                               | Metadados de ambiente por sessão                                                                                                                                                                                                                                                                |
| `tasks/`                                     | Listas de tarefas por sessão escritas pelas ferramentas de tarefa                                                                                                                                                                                                                               |
| `shell-snapshots/`                           | Aliases, funções e opções de shell capturadas na inicialização e aplicadas pela [ferramenta Bash](/docs/pt/tools-reference#bash-tool-behavior) a cada comando. Removido na saída limpa. A limpeza remove qualquer um deixado após um crash.                                                          |
| `backups/`                                   | Cópias com timestamp de `~/.claude.json` tiradas antes de migrações de configuração                                                                                                                                                                                                             |
| `feedback-bundles/`                          | Arquivos de transcrição reduzidos escritos por `/feedback` em provedores de terceiros ou quando nenhuma credencial Anthropic está configurada, para enviar à sua equipe de conta Anthropic                                                                                                      |
| `todos/`, `statsig/`, `logs/`                | Diretórios legados de versões mais antigas. Não mais escritos. A limpeza remove seu conteúdo e depois o diretório vazio.                                                                                                                                                                        |

<h3 id="kept-until-you-delete-them">
  Mantidos até você deletá-los
</h3>

Os caminhos a seguir não são cobertos pela limpeza automática e persistem indefinidamente.

| Caminho sob `~/.claude/` | Conteúdo                                                                                                                                                                                               |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `history.jsonl`          | Cada prompt que você digitou, com timestamp e caminho do projeto. Usado para recall de seta para cima.                                                                                                 |
| `stats-cache.json`       | Contagens agregadas de token e custo mostradas por `/usage`                                                                                                                                            |
| `remote-settings.json`   | Cópia em cache de [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings) para sua organização. Presente apenas quando sua organização as configurou. Atualizado em cada inicialização. |

Outros arquivos pequenos de cache e lock aparecem dependendo de quais recursos você usa e são seguros para deletar.

<h3 id="plaintext-storage">
  Armazenamento em texto simples
</h3>

Transcrições e histórico não são criptografados em repouso. Permissões de arquivo do SO são a única proteção. Se uma ferramenta lê um arquivo `.env` ou um comando imprime uma credencial, esse valor é escrito em `projects/<project>/<session>.jsonl`. Para reduzir exposição:

* Diminua `cleanupPeriodDays` para encurtar quanto tempo as transcrições são mantidas
* Defina a variável de ambiente [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/pt/env-vars) para pular a escrita de transcrições e histórico de prompts em qualquer modo. Em modo não-interativo, você pode passar `--no-session-persistence` junto com `-p`, ou definir `persistSession: false` no Agent SDK.
* Use [regras de permissão](/docs/pt/permissions) para negar leituras de arquivos de credencial

<h3 id="clear-local-data">
  Limpar dados locais
</h3>

Execute `claude project purge` para deletar o estado que Claude Code mantém para um projeto. O comando requer Claude Code v2.1.124 ou posterior. Ele deleta:

* Transcrições e memória automática sob `projects/`
* Entradas por sessão de `tasks/`, `debug/` e `file-history/`
* Linhas de prompt correspondentes em `history.jsonl`
* A entrada do projeto em `~/.claude.json`

O comando imprime o plano de exclusão completo e pede confirmação antes de remover qualquer coisa.

Visualize o plano sem deletar nada:

```bash theme={null}
claude project purge ~/work/my-repo --dry-run
```

Delete com um único prompt de confirmação:

```bash theme={null}
claude project purge ~/work/my-repo
```

Omita o caminho para escolher um projeto de uma lista interativa.

Pule o prompt de confirmação para uso em scripts:

```bash theme={null}
claude project purge ~/work/my-repo --yes
```

Passe `--all` em vez de um caminho para limpar o estado de todos os projetos de uma vez, o que deleta `history.jsonl` completamente em vez de filtrá-lo. Passe `-i` para percorrer o plano de exclusão um item por vez.

O comando deixa `shell-snapshots/` e `backups/` sozinhos porque esses não têm escopo de projeto, e avisa sobre eles na saída do plano. Ele sai com status 1 se nenhum estado corresponder ao caminho fornecido.

Você também pode deletar qualquer um dos caminhos de dados da aplicação acima manualmente. Novas sessões não são afetadas. A tabela abaixo mostra o que você perde para sessões passadas.

| Deletar                                                                                                                                                                                      | Você perde                                                |
| -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------- |
| `~/.claude/projects/`                                                                                                                                                                        | Resume, continue e rewind para sessões passadas           |
| `~/.claude/history.jsonl`                                                                                                                                                                    | Recall de prompt de seta para cima                        |
| `~/.claude/file-history/`                                                                                                                                                                    | Restauração de checkpoint para sessões passadas           |
| `~/.claude/stats-cache.json`                                                                                                                                                                 | Totais históricos mostrados por `/usage`                  |
| `~/.claude/remote-settings.json`                                                                                                                                                             | Nada. Re-buscado na próxima inicialização.                |
| `~/.claude/debug/`, `~/.claude/plans/`, `~/.claude/paste-cache/`, `~/.claude/image-cache/`, `~/.claude/session-env/`, `~/.claude/tasks/`, `~/.claude/shell-snapshots/`, `~/.claude/backups/` | Nada voltado para o usuário                               |
| `~/.claude/todos/`, `~/.claude/statsig/`, `~/.claude/logs/`                                                                                                                                  | Nada. Diretórios legados não escritos por versões atuais. |

Não delete `~/.claude.json`, `~/.claude/settings.json` ou `~/.claude/plugins/`: esses contêm sua autenticação, preferências e plugins instalados.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Gerenciar memória de Claude](/docs/pt/memory): escrever e organizar CLAUDE.md, rules e auto memory
* [Configurar configurações](/docs/pt/settings): definir permissões, hooks, variáveis de ambiente e padrões de modelo
* [Criar skills](/docs/pt/skills): construir prompts e workflows reutilizáveis
* [Configurar subagents](/docs/pt/sub-agents): definir agentes especializados com seu próprio contexto
