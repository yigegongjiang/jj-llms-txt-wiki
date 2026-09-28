> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Perluas agen dengan skills

> Kontrol skill mana yang dapat Claude panggil dalam sesi Claude Agent SDK, dispatch perintah berdasarkan nama, dan buat skills yang sesi Anda temukan

Agent Skills memperluas Claude dengan kemampuan khusus yang Claude panggil ketika relevan. Skills dikemas sebagai file `SKILL.md` yang berisi instruksi, deskripsi, dan sumber daya pendukung opsional. Halaman ini juga mencakup [perintah dalam sesi Agent SDK](#commands-in-agent-sdk-sessions).

Untuk informasi komprehensif tentang skills, termasuk manfaat, arsitektur, dan panduan penulisan, lihat [ikhtisar Agent Skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview).

<h2 id="how-skills-work-with-the-agent-sdk">
  Cara skills bekerja dengan Agent SDK
</h2>

Saat menggunakan Claude Agent SDK, skills adalah:

* **Didefinisikan sebagai artefak filesystem**: Anda membuat setiap skill sebagai file `SKILL.md` di direktorinya sendiri, seperti `.claude/skills/<name>/SKILL.md`
* **Dimuat dari filesystem**: SDK memuat skills dari lokasi filesystem yang diatur oleh `settingSources` (TypeScript) atau `setting_sources` (Python)
* **Ditemukan secara otomatis**: Setelah pengaturan filesystem dimuat, SDK menemukan metadata skill saat startup dari direktori pengguna dan proyek, dan memuat konten penuh ketika Claude memanggil skill
* **Dipanggil oleh model**: Claude secara otomatis memilih kapan menggunakannya berdasarkan konteks
* **Dipanggil oleh pengguna**: Anda dispatch skill secara langsung dengan mengirim `/<name>` dalam prompt. Lihat [Perintah dalam sesi Agent SDK](#commands-in-agent-sdk-sessions)
* **Disaring melalui opsi `skills`**: Skills yang ditemukan diaktifkan secara default. Berikan daftar nama skill, `"all"`, atau `[]` untuk mengontrol skill mana yang dapat Claude panggil

Tidak seperti subagents, yang dapat Anda definisikan dalam [opsi `agents`](/docs/id/agent-sdk/subagents#programmatic-definition-recommended), Anda membuat skills sebagai file di disk. SDK tidak menyediakan API programatis untuk mendaftarkan mereka.

<Note>
  Skills ditemukan melalui sumber pengaturan filesystem. Dengan opsi `query()` default, SDK memuat sumber pengguna dan proyek, jadi skills di `~/.claude/skills/`, `<cwd>/.claude/skills/`, dan `.claude/skills/` di direktori induk mana pun dari `<cwd>` hingga akar repositori tersedia. Sumber proyek juga mencakup `<dir>/.claude/skills/` di setiap direktori yang Anda lewatkan melalui `additionalDirectories` (TypeScript) atau `add_dirs` (Python), karena SDK meneruskan direktori tersebut ke Claude Code sebagai [`--add-dir`](/docs/id/skills#skills-from-additional-directories). Jika Anda menetapkan `settingSources` secara eksplisit, sertakan `'project'` untuk mempertahankan skills proyek dan direktori tambahan serta `'user'` untuk mempertahankan skills pribadi Anda, atau gunakan [opsi `plugins`](/docs/id/agent-sdk/plugins) untuk memuat skills dari jalur tertentu.
</Note>

<h2 id="use-skills-with-the-agent-sdk">
  Gunakan skills dengan Agent SDK
</h2>

Atur opsi `skills` pada `query()` untuk mengontrol skill mana yang dapat Claude panggil dalam sesi. Ketika dihilangkan, skills yang ditemukan diaktifkan dan alat Skill tersedia, sesuai dengan perilaku CLI. Berikan `"all"` untuk membiarkan Claude memanggil setiap skill yang ditemukan, daftar nama skill untuk mengizinkan hanya yang tersebut, atau `[]` untuk membiarkan Claude tidak memanggil apa pun.

Misalnya, untuk membiarkan Claude memanggil hanya dua skill bernama:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(skills=["pdf", "docx"])
  ```

  ```typescript TypeScript theme={null}
  const options = { skills: ["pdf", "docx"] };
  ```
</CodeGroup>

<h3 id="set-up-skills-in-a-session">
  Siapkan skills dalam sesi
</h3>

Ketika Anda menetapkan `skills`, SDK secara otomatis menambahkan alat Skill ke `allowedTools`. Jika Anda juga meneruskan daftar `tools` eksplisit, sertakan `"Skill"` dalam daftar tersebut sehingga Claude dapat memanggil skills.

Setelah dikonfigurasi, Claude secara otomatis menemukan skills dari filesystem dan memanggilnya ketika relevan dengan permintaan pengguna.

Contoh berikut mengaktifkan setiap skill yang ditemukan dalam sesi dan pra-menyetujui alat yang skills biasanya butuhkan. Contoh menetapkan `cwd` ke direktori kerja proses saat ini, jadi jalankan dari dalam proyek yang memiliki direktori `.claude/skills/` di direktori saat ini atau induk mana pun hingga akar repositori:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import os

  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      options = ClaudeAgentOptions(
          cwd=os.getcwd(),  # .claude/skills/ here or in a parent directory
          setting_sources=["user", "project"],  # Load skills from filesystem
          skills="all",  # Let Claude invoke every discovered skill
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
      cwd: process.cwd(), // .claude/skills/ here or in a parent directory
      settingSources: ["user", "project"], // Load skills from filesystem
      skills: "all", // Let Claude invoke every discovered skill
      allowedTools: ["Read", "Write", "Bash"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

<h3 id="confirm-skills-loaded">
  Konfirmasi skills dimuat
</h3>

Dekat awal aliran, SDK menghasilkan pesan sistem dengan subtype `init`. Periksa array `skills` untuk mengonfirmasi skills Anda dimuat sebelum Claude mulai bekerja. Array mencakup skills yang dapat dipanggil pengguna yang telah Anda definisikan dengan bidang frontmatter `description` atau `when_to_use`, bersama dengan [skills bundel yang disertakan dengan Claude Code](/docs/id/skills#bundled-skills).

Array hanya mencantumkan skills yang dapat dipanggil pengguna. Skill dengan [`user-invocable: false`](/docs/id/skills#control-who-invokes-a-skill) dalam frontmatter dimuat dan tetap tersedia untuk Claude, tetapi tidak muncul dalam array. Array mencantumkan skills yang sama apakah atau tidak mereka ada dalam daftar `skills` Anda.

<h3 id="allow-only-specific-skills">
  Izinkan hanya skills tertentu
</h3>

Untuk membiarkan Claude memanggil hanya skills tertentu, berikan nama mereka dalam daftar `skills`. Nama cocok dengan bidang `name` di `SKILL.md` atau nama direktori skill. Gunakan `plugin:skill` untuk skills yang disediakan plugin.

Daftar hanya mengambil nama skill yang tepat. Jika entri tidak dapat berfungsi sebagai nama yang tepat, `query()` menolak daftar sebelum sesi dimulai. Lihat [Kesalahan nama skill tidak valid](#invalid-skill-name-error) untuk aturan nama dan kesalahan yang setiap SDK angkat.

Model tidak melihat skills yang tidak tercantum dan alat Skill menolaknya, sementara file mereka tetap di disk dan tetap dapat diakses melalui Read dan Bash. Membatasi daftar tidak membatasi [dispatch berdasarkan nama](#dispatch-commands-by-name).

Untuk membiarkan Claude memanggil setiap skill yang ditemukan, berikan `skills: "all"` daripada wildcard.

<h2 id="commands-in-agent-sdk-sessions">
  Perintah dalam sesi Agent SDK
</h2>

Bagian ini adalah dokumentasi perintah SDK. Perintah adalah apa pun yang Anda jalankan dengan mengirim `/<name>` dalam prompt. Entri pada permukaan perintah berbeda dalam apa yang mendukung mereka:

* **Perintah bawaan**: menjalankan logika yang dikodekan ke dalam proses Claude Code yang SDK jalankan, misalnya `/compact`
* **Skills bundel**: artefak prompt yang disertakan dengan Claude Code, misalnya `/code-review`
* **Skills Anda**: artefak prompt yang Anda buat, masing-masing direktori yang menyimpan file `SKILL.md`. Nama skill yang dapat dipanggil pengguna bergabung dengan permukaan secara otomatis, jadi mendispatch `/security-check` Anda sendiri dan menjalankan bawaan bekerja dengan cara yang sama
* **File perintah khusus**: bentuk artefak yang lebih lama dengan perilaku yang sama, file Markdown datar di `.claude/commands/` yang nama file mereka menjadi nama perintah. Skills adalah penerus yang direkomendasikan

Secara default, baik Anda maupun Claude dapat memanggil skill apa pun. Anda dapat membatasi jalur mana pun melalui [frontmatter](/docs/id/skills#control-who-invokes-a-skill) skill. Untuk definisi perintah dan skill, lihat entri [Perintah](/docs/id/glossary#command) dan [Skill](/docs/id/glossary#skill) dalam glosarium. Lihat [Perintah dalam Claude Code](/docs/id/commands) untuk setiap bawaan dan [Perluas Claude dengan skills](/docs/id/skills) untuk panduan lengkap kedua bentuk artefak.

<h3 id="discover-available-commands">
  Temukan perintah yang tersedia
</h3>

Anda dapat mendispatch perintah yang bekerja tanpa terminal interaktif melalui SDK. Pesan `system/init` mencantumkan yang tersedia dalam sesi Anda di bidang `slash_commands`. Perintah yang membutuhkan terminal interaktif, seperti `/theme` dan `/terminal-setup`, tidak muncul dalam daftar. Akses bidang ketika sesi Anda dimulai:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Hello Claude",
    options: { maxTurns: 1 }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      console.log("Available commands:", message.slash_commands);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage


  async def main():
      async for message in query(prompt="Hello Claude", options=ClaudeAgentOptions(max_turns=1)):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print("Available commands:", message.data["slash_commands"])


  asyncio.run(main())
  ```
</CodeGroup>

Daftar yang dicetak mencampur perintah bawaan, skills bundel, skills yang dapat dipanggil pengguna Anda, dan file `.claude/commands/`:

```text theme={null}
Available commands: ["clear", "compact", "context", "usage", "code-review", "verify", "security-check", ...]
```

Skill dengan [`user-invocable: false`](/docs/id/skills#control-who-invokes-a-skill) dalam frontmatter tidak muncul dalam daftar ini atau dalam array `skills` dari [Konfirmasi skills dimuat](#confirm-skills-loaded). Sesi yang mengonfigurasi [server MCP](/docs/id/agent-sdk/mcp) juga dapat mengekspos [prompt MCP sebagai perintah](/docs/id/mcp#use-mcp-prompts-as-commands).

<h3 id="dispatch-commands-by-name">
  Dispatch perintah berdasarkan nama
</h3>

Kirim perintah dengan memasukkannya dalam string prompt Anda, dengan cara yang sama Anda mengirim teks biasa. Dispatch tidak bergantung pada opsi `skills`. Mengirim `/<name>` menjalankan skill yang dapat dipanggil pengguna bahkan ketika daftar `skills` Anda menghilangkannya. Perintah yang bertindak pada riwayat percakapan, seperti `/compact`, membutuhkan pesan sebelumnya untuk bekerja dengan.

Sebuah `/<name>` yang tidak cocok dengan perintah apa pun dalam sesi maupun perintah Claude Code bawaan tidak gagal query. Claude Code mengirim prompt ke Claude sebagai pesan biasa, dengan catatan bahwa perintah tidak berjalan, jadi query menghabiskan putaran model dan mengembalikan balasan Claude. Sebelum v2.1.274, sebuah `/<name>` yang tidak cocok dengan apa pun mengembalikan `Unknown command: /<name>` sebagai hasil tanpa putaran model.

Sebuah `/<name>` yang cocok dengan perintah Claude Code bawaan yang tidak tersedia dalam sesi, seperti `/theme`, mengembalikan `/theme isn't available in this environment.` sebagai hasil tanpa putaran model.

<Note>
  Perintah dapat mencapai batas `maxTurns` / `max_turns` seperti prompt lainnya, mengakhiri query dengan hasil kesalahan daripada `success`. Untuk kontrak hasil kesalahan, lihat [Tangani hasil](/docs/id/agent-sdk/agent-loop#handle-the-result). Jika perintah Anda mungkin mencapai batas, bungkus loop dalam `try`/`catch` di TypeScript atau `try`/`except` di Python, seperti yang ditunjukkan dalam [Input Pesan Tunggal](/docs/id/agent-sdk/streaming-vs-single-mode#single-message-input), atau atur `maxTurns` cukup tinggi agar pekerjaan selesai.
</Note>

<h3 id="compact-history-with-/compact">
  Kompres riwayat dengan `/compact`
</h3>

Perintah `/compact` mengurangi ukuran riwayat percakapan Anda dengan merangkum pesan yang lebih lama sambil mempertahankan konteks penting. Pemadatan membutuhkan percakapan yang ada dengan cukup pesan sebelumnya untuk dirangkum. Contoh ini memiliki percakapan terlebih dahulu, kemudian memadatkannya dan membaca pesan sistem `compact_boundary` yang melaporkan hasilnya:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Compaction needs existing history, so have a conversation first
  try {
    for await (const message of query({
      prompt: "Explain what this project does",
      options: { maxTurns: 2 }
    })) {
      if (message.type === "result" && message.subtype === "success") {
        console.log(message.result);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so the follow-up query below still runs.
    console.error(`Session ended with an error: ${error}`);
  }

  // Compact the same conversation
  for await (const message of query({
    prompt: "/compact",
    options: { continue: true, maxTurns: 1 }
  })) {
    if (message.type === "system" && message.subtype === "compact_boundary") {
      console.log("Compaction completed");
      console.log("Pre-compaction tokens:", message.compact_metadata.pre_tokens);
      console.log("Trigger:", message.compact_metadata.trigger);
      // Example output:
      // Compaction completed
      // Pre-compaction tokens: 1842
      // Trigger: manual
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage, SystemMessage


  async def main():
      # Compaction needs existing history, so have a conversation first
      try:
          async for message in query(
              prompt="Explain what this project does",
              options=ClaudeAgentOptions(max_turns=2),
          ):
              if isinstance(message, ResultMessage) and message.subtype == "success":
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so the follow-up query below still runs.
          print(f"Session ended with an error: {error}")

      # Compact the same conversation
      async for message in query(
          prompt="/compact",
          options=ClaudeAgentOptions(continue_conversation=True, max_turns=1),
      ):
          if isinstance(message, SystemMessage) and message.subtype == "compact_boundary":
              print("Compaction completed")
              print("Pre-compaction tokens:", message.data["compact_metadata"]["pre_tokens"])
              print("Trigger:", message.data["compact_metadata"]["trigger"])
              # Example output:
              # Compaction completed
              # Pre-compaction tokens: 1842
              # Trigger: manual


  asyncio.run(main())
  ```
</CodeGroup>

<Note>
  Pesan `compact_boundary` hanya tiba ketika pemadatan berjalan. Tanpa apa pun untuk dirangkum, `/compact` melaporkan alasannya daripada menaikkan. Jalannya masih berakhir dengan hasil `success` dan tidak ada pesan `compact_boundary`, dan teks hasil membawa alasannya, misalnya `Not enough messages to compact.` setelah pertukaran pendek tunggal. Panggilan `query()` satu kali yang segar dimulai dengan konteks kosong, jadi gunakan pola ini dalam sesi dengan putaran sebelumnya, misalnya dalam [mode input streaming](/docs/id/agent-sdk/streaming-vs-single-mode) atau saat melanjutkan sesi.
</Note>

<h3 id="reset-context-with-/clear">
  Atur ulang konteks dengan `/clear`
</h3>

Perintah `/clear` mengatur ulang percakapan ke konteks kosong, jadi prompt berikutnya dimulai tanpa riwayat percakapan sebelumnya. Percakapan sebelumnya tetap di disk. Anda dapat kembali ke percakapan itu dengan meneruskan ID sesinya ke [opsi `resume`](/docs/id/agent-sdk/sessions#resume-by-id).

`/clear` berguna dalam [mode input streaming](/docs/id/agent-sdk/streaming-vs-single-mode), di mana Anda mengirim beberapa prompt melalui koneksi tunggal. Untuk panggilan `query()` satu kali, setiap panggilan sudah dimulai dengan konteks kosong, jadi mengirim `/clear` tidak memiliki efek praktis. Mulai `query()` baru sebagai gantinya.

<h2 id="create-skills">
  Buat skills
</h2>

Buat setiap skill sebagai direktori yang berisi file `SKILL.md` dengan frontmatter YAML dan konten Markdown. Bidang `description` menentukan kapan Claude memanggil skill Anda.

**Contoh struktur direktori**:

```text theme={null}
.claude/skills/security-check/
└── SKILL.md
```

<h3 id="choose-a-discovery-level">
  Pilih tingkat penemuan
</h3>

Simpan skills di salah satu dari dua tingkat penemuan paling umum [discovery levels](/docs/id/skills#where-skills-live):

* **Skills proyek**: `.claude/skills/`, hanya tersedia di proyek saat ini
* **Skills pribadi**: `~/.claude/skills/`, tersedia di semua proyek Anda

Jika Anda memiliki file perintah khusus yang ada di `.claude/commands/`, mereka terus bekerja. File perintah di `.claude/commands/deploy.md` membuat `/deploy` dan bekerja dengan cara yang sama seperti skill di `.claude/skills/deploy/SKILL.md`. Jika file perintah dan skill berbagi nama, lihat [Selesaikan skills yang berbagi nama](/docs/id/skills#resolve-skills-that-share-a-name) untuk yang mana yang berjalan. SDK memuat file `.claude/commands/` dan `~/.claude/commands/` dari dua cakupan yang sama dengan skills. Lihat [Perluas Claude dengan skills](/docs/id/skills) untuk panduan lengkap kedua bentuk artefak.

<h3 id="create-and-dispatch-your-first-skill">
  Buat dan dispatch skill pertama Anda
</h3>

Untuk melihat alur lengkapnya, buat `.claude/skills/security-check/SKILL.md`:

```markdown theme={null}
---
name: security-check
description: Run a security vulnerability scan
---

Analyze the codebase for security vulnerabilities including:
- SQL injection risks
- XSS vulnerabilities
- Exposed credentials
- Insecure configurations
```

Setelah file ada, skill tersedia melalui SDK. Claude memanggilnya ketika permintaan cocok dengan deskripsinya, dan Anda dapat mendispatchnya secara langsung:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "/security-check",
    options: { maxTurns: 10 }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      async for message in query(
          prompt="/security-check", options=ClaudeAgentOptions(max_turns=10)
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

Jalankan yang berhasil berakhir dengan hasil `success` yang teksnya membawa temuan pemindaian. Terhadap aplikasi Express kecil dengan masalah yang ditanam, teks hasil dimulai:

```text theme={null}
**Security scan of `app.js` — 4 findings (most severe first):**

1. **SQL Injection** (line 8) — `req.query.name` is concatenated directly into the SQL string. Trivially exploitable (`' OR '1'='1`, `'; DROP TABLE users;--`). **Fix:** use parameterized queries, e.g. `db.query("SELECT * FROM users WHERE name = ?", [req.query.name], cb)`.
...
```

Nama skill juga muncul dalam array `slash_commands` pesan init.

<Note>
  Claude Code mencakup skills bundel `code-review` dan `verify`. Jika Anda memberi nama file `.claude/commands/` setelah salah satunya, misalnya `.claude/commands/code-review.md`, file perintah mengaburkan skill bundel dan `slash_commands` mencantumkan nama sekali.
</Note>

<h2 id="pre-approve-tools-for-skills">
  Pra-setujui alat untuk skills
</h2>

<Note>
  Untuk skills proyek dan pribadi, Claude Code menerapkan bidang frontmatter [`allowed-tools`](/docs/id/skills#pre-approve-tools-for-a-skill) dalam sesi SDK. Anda juga dapat pra-menyetujui alat untuk skills ini melalui opsi `allowedTools` (`allowed_tools` di Python) dalam konfigurasi query Anda. Skills [disinkronkan dari claude.ai](/docs/id/skills#how-claude-code-handles-the-frontmatter-of-a-synced-skill) mengikuti aturan frontmatter mereka sendiri.
</Note>

Skills berjalan dengan alat sesi. Contoh di bawah pra-menyetujui `Read`, `Grep`, dan `Glob` dengan `allowedTools` (`allowed_tools` di Python), jadi Claude dapat memeriksa file saat menjalankan [skill security-check](#create-and-dispatch-your-first-skill) tanpa berhenti untuk persetujuan:

<CodeGroup>
  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],  # Load skills from filesystem
      skills="all",
      allowed_tools=["Read", "Grep", "Glob"],
  )


  async def main():
      async for message in query(prompt="Check this project for security issues", options=options):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Check this project for security issues",
    options: {
      settingSources: ["user", "project"], // Load skills from filesystem
      skills: "all",
      allowedTools: ["Read", "Grep", "Glob"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Dalam aliran, pemanggilan skill muncul sebagai penggunaan alat Skill, diikuti oleh panggilan Read pada file proyek. Jalannya berakhir dengan hasil `success` yang teksnya membawa temuan.

Daftar pra-menyetujui alat bernama daripada membatasi yang lain. Untuk alur izin lengkap, termasuk mode izin dan callback `canUseTool`, lihat [Izin](/docs/id/agent-sdk/permissions).

<h2 id="troubleshooting">
  Pemecahan Masalah
</h2>

<h3 id="skills-not-found">
  Skills tidak ditemukan
</h3>

**Periksa konfigurasi settingSources**: SDK menemukan skills melalui sumber pengaturan `user` dan `project`. Jika Anda menetapkan `settingSources`/`setting_sources` secara eksplisit dan menghilangkan sumber tersebut, SDK tidak memuat skills:

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
  const optionsWithoutSkills = {
    settingSources: [],
    skills: "all"
  };

  // Skills loaded: user and project sources included
  const optionsWithSkills = {
    settingSources: ["user", "project"],
    skills: "all"
  };
  ```
</CodeGroup>

Untuk direktori skill mana yang dimuat setiap sumber, lihat [tabel sumber filesystem](/docs/id/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources). Untuk detail lebih lanjut tentang `settingSources`/`setting_sources`, lihat [referensi SDK TypeScript](/docs/id/agent-sdk/typescript#settingsource) atau [referensi SDK Python](/docs/id/agent-sdk/python#settingsource).

**Periksa direktori kerja**: SDK memuat skills dari `.claude/skills/` dalam opsi `cwd` dan di setiap direktori induk hingga akar repositori. Pastikan `cwd` menunjuk ke atau di bawah direktori yang berisi `.claude/skills/`, dalam repositori yang sama:

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

Lihat [Gunakan skills dengan Agent SDK](#use-skills-with-the-agent-sdk) untuk pola lengkapnya.

**Verifikasi lokasi filesystem**:

```bash theme={null}
# Check project skills
ls .claude/skills/*/SKILL.md

# Check personal skills
ls ~/.claude/skills/*/SKILL.md
```

<h3 id="skill-not-being-used">
  Skill tidak digunakan
</h3>

**Periksa opsi `skills`**: jika Anda meneruskan daftar `skills`, konfirmasi nama skill disertakan. Ketika Claude mencoba memanggil skill yang tidak tercantum, alat Skill mengembalikan `Skill <name> is not in this session's skills allowlist`. Tambahkan nama ke daftar Anda, atau dispatch skill secara langsung dengan mengirim `/<name>` dalam prompt, yang bekerja tanpa pencatatan.

**Periksa deskripsi**: pastikan itu spesifik dan mencakup kata kunci yang relevan. Lihat [praktik terbaik Agent Skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions) untuk panduan tentang menulis deskripsi yang efektif.

<h3 id="invalid-skill-name-error">
  Kesalahan nama skill tidak valid
</h3>

Ketika nama dalam daftar `skills` Anda tidak dapat berfungsi sebagai nama skill yang tepat, `query()` menolak daftar sebelum memulai proses Claude Code. Nama yang memicu penolakan mencakup:

* Nama kosong
* Nama yang berisi tanda kurung, koma, atau karakter kontrol
* Nama yang diisi dengan spasi putih
* Bentuk wildcard seperti `*` telanjang atau akhiran `:*`

Setiap SDK menampilkan penolakan secara berbeda:

<Tabs>
  <Tab title="TypeScript">
    SDK TypeScript melempar `Error` yang menyatakan aturan yang entri langgar. Misalnya, `skills: ["docs:*"]` melempar:

    ```text theme={null}
    Invalid skill name "docs:*": wildcard-suffix names are not allowed; list each skill by its exact name.
    ```

    Nama kosong melaporkan `Skill names must be non-empty strings.`

    Sebelum TypeScript Agent SDK 0.3.221, SDK tidak menjalankan pemeriksaan ini.
  </Tab>

  <Tab title="Python">
    SDK Python menaikkan `ValueError` yang menyatakan aturan yang entri langgar. Misalnya, `skills=["docs:*"]` menaikkan:

    ```text theme={null}
    ValueError: Invalid skill name 'docs:*': wildcard-suffix names are not allowed; list each skill by its exact name.
    ```

    Nama kosong melaporkan `Skill names must be non-empty strings`.

    Sebelum Python Agent SDK 0.2.129, SDK tidak menjalankan pemeriksaan ini.
  </Tab>
</Tabs>

<h3 id="additional-troubleshooting">
  Pemecahan masalah tambahan
</h3>

Untuk pemecahan masalah skills umum, seperti kesalahan sintaks YAML dan debugging, lihat [bagian pemecahan masalah skills Claude Code](/docs/id/skills#troubleshooting).

<h2 id="next-steps">
  Langkah berikutnya
</h2>

[Panduan skills Claude Code](/docs/id/skills) mencakup penulisan secara mendalam. Panduan tersebut berlaku untuk sesi SDK. Mulai dengan bagian ini:

* [Referensi frontmatter](/docs/id/skills#frontmatter-reference): setiap bidang yang didukung
* [Berikan argumen ke skills](/docs/id/skills#pass-arguments-to-skills): `$ARGUMENTS`, `$0`, `$1`, dan penumpukan skill. [Tabel substitusi lengkap](/docs/id/skills#available-string-substitutions) menambahkan argumen bernama dan variabel `${CLAUDE_*}`
* [Injeksi konteks dinamis](/docs/id/skills#inject-dynamic-context): baris `` !`command` `` yang berjalan sebelum Claude melihat konten skill
* [Pilih di mana skills dimuat](/docs/id/skills#where-skills-live): setiap lokasi skill, namespace plugin, dan skill mana yang berjalan ketika dua berbagi nama

<h2 id="related-resources">
  Sumber daya terkait
</h2>

* [Perintah dalam Claude Code](/docs/id/commands): permukaan perintah lengkap, termasuk setiap bawaan
* [Ikhtisar Agent Skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview): ikhtisar konseptual, manfaat, dan arsitektur
* [Praktik terbaik Agent Skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices): panduan penulisan untuk skills yang efektif
* [Buku resep Agent Skills](https://platform.claude.com/cookbook/skills-notebooks-01-skills-introduction): contoh skills dan template
* [Subagents dalam SDK](/docs/id/agent-sdk/subagents): agen berbasis filesystem serupa dengan opsi programatis
* [Ikhtisar SDK](/docs/id/agent-sdk/overview): konsep SDK umum
* [Referensi SDK TypeScript](/docs/id/agent-sdk/typescript): dokumentasi API lengkap
* [Referensi SDK Python](/docs/id/agent-sdk/python): dokumentasi API lengkap
