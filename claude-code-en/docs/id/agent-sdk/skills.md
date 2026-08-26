> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent Skills dalam SDK

> Perluas Claude dengan kemampuan khusus menggunakan Agent Skills dalam Claude Agent SDK

<h2 id="overview">
  Ikhtisar
</h2>

Agent Skills memperluas Claude dengan kemampuan khusus yang Claude secara otomatis memanggil ketika relevan. Skills dikemas sebagai file `SKILL.md` yang berisi instruksi, deskripsi, dan sumber daya pendukung opsional.

Untuk informasi komprehensif tentang Skills, termasuk manfaat, arsitektur, dan panduan penulisan, lihat [ikhtisar Agent Skills](https://platform.claude.com/docs/id/agents-and-tools/agent-skills/overview).

<h2 id="how-skills-work-with-the-sdk">
  Cara Skills Bekerja dengan SDK
</h2>

Saat menggunakan Claude Agent SDK, Skills adalah:

1. **Didefinisikan sebagai artefak filesystem**: Dibuat sebagai file `SKILL.md` di direktori tertentu (`.claude/skills/`)
2. **Dimuat dari filesystem**: Skills dimuat dari lokasi filesystem yang diatur oleh `settingSources` (TypeScript) atau `setting_sources` (Python)
3. **Ditemukan secara otomatis**: Setelah pengaturan filesystem dimuat, metadata Skill ditemukan saat startup dari direktori pengguna dan proyek; konten penuh dimuat saat dipicu
4. **Dipanggil oleh model**: Claude secara otomatis memilih kapan menggunakannya berdasarkan konteks
5. **Disaring melalui opsi `skills`**: Skills yang ditemukan diaktifkan secara default. Berikan daftar nama skill, `"all"`, atau `[]` untuk mengontrol mana yang tersedia dalam sesi

Tidak seperti subagents (yang dapat didefinisikan secara programatis), Skills harus dibuat sebagai artefak filesystem. SDK tidak menyediakan API programatis untuk mendaftarkan Skills.

<Note>
  Skills ditemukan melalui sumber pengaturan filesystem. Dengan opsi `query()` default, SDK memuat sumber pengguna dan proyek, jadi skills di `~/.claude/skills/`, `<cwd>/.claude/skills/`, dan `.claude/skills/` di direktori induk mana pun dari `<cwd>` hingga akar repositori tersedia. Jika Anda menetapkan `settingSources` secara eksplisit, sertakan `'user'` atau `'project'` untuk mempertahankan penemuan skill, atau gunakan [opsi `plugins`](/docs/id/agent-sdk/plugins) untuk memuat skills dari jalur tertentu.
</Note>

<h2 id="using-skills-with-the-sdk">
  Menggunakan Skills dengan SDK
</h2>

Atur opsi `skills` pada `query()` untuk mengontrol Skills mana yang tersedia untuk sesi. Ketika dihilangkan, Skills yang ditemukan diaktifkan dan alat Skill tersedia, sesuai dengan perilaku CLI. Berikan `"all"` untuk mengaktifkan setiap Skill yang ditemukan, daftar nama Skill untuk mengaktifkan hanya yang tersebut, atau `[]` untuk menonaktifkan semua. Ketika Anda menetapkan `skills`, SDK secara otomatis menambahkan alat Skill ke `allowedTools`. Jika Anda juga meneruskan daftar `tools` eksplisit, sertakan `"Skill"` dalam daftar tersebut sehingga Claude dapat memanggil skills.

Setelah dikonfigurasi, Claude secara otomatis menemukan Skills dari filesystem dan memanggilnya ketika relevan dengan permintaan pengguna.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      options = ClaudeAgentOptions(
          cwd="/path/to/project",  # Project with .claude/skills/
          setting_sources=["user", "project"],  # Load Skills from filesystem
          skills="all",  # Enable every discovered Skill
          allowed_tools=["Read", "Write", "Bash"],
      )

      async for message in query(
          prompt="Help me process this PDF document", options=options
      ):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Help me process this PDF document",
    options: {
      cwd: "/path/to/project", // Project with .claude/skills/
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all", // Enable every discovered Skill
      allowedTools: ["Read", "Write", "Bash"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Untuk mengaktifkan hanya Skills tertentu, berikan nama mereka. Nama cocok dengan bidang `name` di `SKILL.md` atau nama direktori Skill. Gunakan `plugin:skill` untuk Skills yang disediakan plugin.

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(skills=["pdf", "docx"])
  ```

  ```typescript TypeScript theme={null}
  const options = { skills: ["pdf", "docx"] };
  ```
</CodeGroup>

Opsi `skills` adalah filter konteks, bukan sandbox. Skills yang tidak tercantum disembunyikan dari model dan ditolak oleh alat Skill, tetapi file mereka tetap di disk dan dapat diakses melalui Read dan Bash.

<h2 id="skill-locations">
  Lokasi Skill
</h2>

Skills dimuat dari direktori filesystem berdasarkan konfigurasi `settingSources`/`setting_sources` Anda:

* **Project Skills** (`.claude/skills/`): Dibagikan dengan tim Anda melalui git - dimuat ketika `setting_sources` mencakup `"project"`
* **User Skills** (`~/.claude/skills/`): Skills pribadi di semua proyek - dimuat ketika `setting_sources` mencakup `"user"`
* **Plugin Skills**: Disertakan dengan plugin Claude Code yang diinstal

<h2 id="creating-skills">
  Membuat Skills
</h2>

Skills didefinisikan sebagai direktori yang berisi file `SKILL.md` dengan frontmatter YAML dan konten Markdown. Bidang `description` menentukan kapan Claude memanggil Skill Anda.

**Contoh struktur direktori**:

```bash theme={null}
.claude/skills/processing-pdfs/
└── SKILL.md
```

Untuk panduan lengkap tentang membuat Skills, termasuk struktur SKILL.md, Skills multi-file, dan contoh, lihat:

* [Agent Skills dalam Claude Code](/docs/id/skills): Panduan lengkap dengan contoh
* [Agent Skills Best Practices](https://platform.claude.com/docs/id/agents-and-tools/agent-skills/best-practices): Panduan penulisan dan konvensi penamaan

<h2 id="tool-restrictions">
  Pembatasan Alat
</h2>

<Note>
  Bidang frontmatter `allowed-tools` di SKILL.md hanya didukung saat menggunakan Claude Code CLI secara langsung. **Ini tidak berlaku saat menggunakan Skills melalui SDK**.

  Saat menggunakan SDK, kontrol akses alat melalui opsi `allowedTools` utama dalam konfigurasi query Anda.
</Note>

Untuk mengontrol akses alat untuk Skills dalam aplikasi SDK, gunakan `allowedTools` untuk pra-persetujuan alat tertentu. Tanpa callback `canUseTool`, apa pun yang tidak ada dalam daftar ditolak:

<Note>
  Pernyataan impor dari contoh pertama diasumsikan dalam cuplikan kode berikut.
</Note>

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],  # Load Skills from filesystem
      skills="all",
      allowed_tools=["Read", "Grep", "Glob"],
  )

  async for message in query(prompt="Analyze the codebase structure", options=options):
      print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "Analyze the codebase structure",
    options: {
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all",
      allowedTools: ["Read", "Grep", "Glob"],
      permissionMode: "dontAsk" // Deny anything not in allowedTools
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

<h2 id="discovering-available-skills">
  Menemukan Skills yang Tersedia
</h2>

Untuk melihat Skills mana yang tersedia dalam aplikasi SDK Anda, cukup tanyakan kepada Claude:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],  # Load Skills from filesystem
      skills="all",
  )

  async for message in query(prompt="What Skills are available?", options=options):
      print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "What Skills are available?",
    options: {
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all"
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Claude akan mencantumkan Skills yang tersedia berdasarkan direktori kerja saat ini dan plugin yang diinstal.

<h2 id="testing-skills">
  Menguji Skills
</h2>

Uji Skills dengan mengajukan pertanyaan yang cocok dengan deskripsi mereka:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      cwd="/path/to/project",
      setting_sources=["user", "project"],  # Load Skills from filesystem
      skills="all",
      allowed_tools=["Read", "Bash"],
  )

  async for message in query(prompt="Extract text from invoice.pdf", options=options):
      print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "Extract text from invoice.pdf",
    options: {
      cwd: "/path/to/project",
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all",
      allowedTools: ["Read", "Bash"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Claude secara otomatis memanggil Skill yang relevan jika deskripsi cocok dengan permintaan Anda.

<h2 id="troubleshooting">
  Pemecahan Masalah
</h2>

<h3 id="skills-not-found">
  Skills Tidak Ditemukan
</h3>

**Periksa konfigurasi settingSources**: Skills ditemukan melalui sumber pengaturan `user` dan `project`. Jika Anda menetapkan `settingSources`/`setting_sources` secara eksplisit dan menghilangkan sumber tersebut, skills tidak dimuat:

<CodeGroup>
  ```python Python theme={null}
  # Skills not loaded: setting_sources excludes user and project
  options = ClaudeAgentOptions(setting_sources=[], skills="all")

  # Skills loaded: user and project sources included
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],
      skills="all",
  )
  ```

  ```typescript TypeScript theme={null}
  // Skills not loaded: settingSources excludes user and project
  const options = {
    settingSources: [],
    skills: "all"
  };

  // Skills loaded: user and project sources included
  const options = {
    settingSources: ["user", "project"],
    skills: "all"
  };
  ```
</CodeGroup>

Untuk detail lebih lanjut tentang `settingSources`/`setting_sources`, lihat [referensi SDK TypeScript](/docs/id/agent-sdk/typescript#settingsource) atau [referensi SDK Python](/docs/id/agent-sdk/python#settingsource).

**Periksa direktori kerja**: SDK memuat Skills dari `.claude/skills/` dalam opsi `cwd` dan di setiap direktori induk hingga akar repositori. Pastikan `cwd` menunjuk ke atau di bawah direktori yang berisi `.claude/skills/`, dalam repositori yang sama:

<CodeGroup>
  ```python Python theme={null}
  # Ensure your cwd points to the directory containing .claude/skills/
  options = ClaudeAgentOptions(
      cwd="/path/to/project",  # .claude/skills/ here or in a parent directory
      setting_sources=["user", "project"],  # Loads skills from these sources
      skills="all",
  )
  ```

  ```typescript TypeScript theme={null}
  // Ensure your cwd points to the directory containing .claude/skills/
  const options = {
    cwd: "/path/to/project", // .claude/skills/ here or in a parent directory
    settingSources: ["user", "project"], // Loads skills from these sources
    skills: "all"
  };
  ```
</CodeGroup>

Lihat bagian "Menggunakan Skills dengan SDK" di atas untuk pola lengkapnya.

**Verifikasi lokasi filesystem**:

```bash theme={null}
# Check project Skills
ls .claude/skills/*/SKILL.md

# Check personal Skills
ls ~/.claude/skills/*/SKILL.md
```

<h3 id="skill-not-being-used">
  Skill Tidak Digunakan
</h3>

**Periksa opsi `skills`**: Jika Anda melewatkan daftar `skills`, konfirmasi nama skill disertakan. Melewatkan `[]` menonaktifkan semua skills.

**Periksa deskripsi**: Pastikan itu spesifik dan mencakup kata kunci yang relevan. Lihat [Agent Skills Best Practices](https://platform.claude.com/docs/id/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions) untuk panduan tentang menulis deskripsi yang efektif.

<h3 id="additional-troubleshooting">
  Pemecahan Masalah Tambahan
</h3>

Untuk pemecahan masalah Skills umum (sintaks YAML, debugging, dll.), lihat [bagian pemecahan masalah Claude Code Skills](/docs/id/skills#troubleshooting).

<h2 id="related-documentation">
  Dokumentasi Terkait
</h2>

<h3 id="skills-guides">
  Panduan Skills
</h3>

* [Agent Skills dalam Claude Code](/docs/id/skills): Panduan Skills lengkap dengan pembuatan, contoh, dan pemecahan masalah
* [Agent Skills Overview](https://platform.claude.com/docs/id/agents-and-tools/agent-skills/overview): Ikhtisar konseptual, manfaat, dan arsitektur
* [Agent Skills Best Practices](https://platform.claude.com/docs/id/agents-and-tools/agent-skills/best-practices): Panduan penulisan untuk Skills yang efektif
* [Agent Skills Cookbook](https://platform.claude.com/cookbook/skills-notebooks-01-skills-introduction): Contoh Skills dan template

<h3 id="sdk-resources">
  Sumber Daya SDK
</h3>

* [Subagents dalam SDK](/docs/id/agent-sdk/subagents): Agen berbasis filesystem serupa dengan opsi programatis
* [Slash Commands dalam SDK](/docs/id/agent-sdk/slash-commands): Perintah yang dipanggil pengguna
* [SDK Overview](/docs/id/agent-sdk/overview): Konsep SDK umum
* [Referensi SDK TypeScript](/docs/id/agent-sdk/typescript): Dokumentasi API lengkap
* [Referensi SDK Python](/docs/id/agent-sdk/python): Dokumentasi API lengkap
