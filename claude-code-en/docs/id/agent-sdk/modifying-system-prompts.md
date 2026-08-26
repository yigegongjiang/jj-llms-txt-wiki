> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Memodifikasi system prompts

> Pilih antara preset `claude_code` dan system prompt kustom, serta sesuaikan perilaku dengan CLAUDE.md, output styles, append, atau prompt yang sepenuhnya kustom.

System prompts mendefinisikan perilaku Claude, kemampuan, dan gaya respons. Mulai dari preset `claude_code` untuk alat coding seperti CLI atau IDE di mana manusia mengawasi dan mengarahkan pekerjaan. Tulis prompt Anda sendiri untuk agen dengan permukaan, identitas, atau model izin yang berbeda.

Halaman ini mencakup:

* [Cara kerja system prompts](#how-system-prompts-work), dengan tabel keputusan untuk memilih antara preset, preset dengan `append`, dan prompt kustom
* [Sesuaikan perilaku agen](#customize-agent-behavior) dengan file CLAUDE.md, output styles, `append`, atau string kustom
* [Bandingkan empat pendekatan](#compare-the-four-approaches) berdasarkan persistensi, cakupan, dan apa yang mereka pertahankan
* [Gabungkan pendekatan](#combine-approaches) untuk melapisi metode kustomisasi bersama-sama

<h2 id="how-system-prompts-work">
  Cara kerja system prompts
</h2>

Sebuah system prompt adalah set instruksi awal yang membentuk bagaimana Claude berperilaku sepanjang percakapan. Agent SDK memiliki tiga titik awal untuk itu:

* **Default minimal**: ketika Anda tidak menetapkan `systemPrompt` di TypeScript atau `system_prompt` di Python, SDK menggunakan prompt minimal yang mencakup tool calling tetapi menghilangkan panduan coding Claude Code, gaya respons, dan konteks proyek. Ini berbeda dari `claude -p`, yang menggunakan prompt Claude Code lengkap secara default. Jika Anda bermigrasi dari CLI dan menginginkan perilaku yang cocok, atur preset `claude_code`.
* **Preset `claude_code`**: system prompt lengkap yang digunakan CLI Claude Code, dengan instruksi penggunaan tool, panduan gaya dan pemformatan kode, aturan nada dan verbositas respons, instruksi keamanan dan keselamatan, serta konteks tentang direktori kerja dan lingkungan. Atur `systemPrompt: { type: "preset", preset: "claude_code" }` di TypeScript atau `system_prompt={"type": "preset", "preset": "claude_code"}` di Python, secara opsional dengan `append` untuk menambahkan instruksi Anda sendiri di akhir.
* **String kustom**: prompt yang Anda tulis sendiri. SDK hanya mengirimkan apa yang Anda berikan.

<h3 id="decide-on-a-starting-point">
  Tentukan titik awal
</h3>

Faktor penentu adalah seberapa dekat agen Anda menyerupai Claude Code: agen coding yang beroperasi di repositori, dengan manusia mengamati output streaming dan mengarahkan pekerjaan. Semakin jauh produk Anda dari itu, semakin banyak Anda akan ingin menulis prompt Anda sendiri.

| Anda membangun                                                                                                                    | Gunakan                              | Apa yang Anda dapatkan                                                                                                                   |
| :-------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| Alat coding seperti CLI atau IDE di mana manusia mengamati dan mengarahkan, dan default Claude Code adalah apa yang Anda inginkan | Preset `claude_code`                 | Prompt Claude Code lengkap: panduan tool, aturan keselamatan, respons ramah terminal, kesadaran konvensi repo                            |
| Jenis alat yang sama, ditambah aturan khusus produk seperti standar coding, format output, atau konteks domain                    | Preset `claude_code` dengan `append` | Semuanya di atas, dengan instruksi Anda ditambahkan setelah preset. Tidak ada yang dihapus, jadi ini adalah kustomisasi risiko terendah  |
| Agen dengan permukaan, identitas, atau model izin yang berbeda, atau agen non-coding                                              | String prompt kustom                 | Hanya apa yang Anda tulis. Anda bertanggung jawab untuk mengganti panduan tool dan instruksi keselamatan yang masih dibutuhkan agen Anda |
| Loop tool-calling tipis tanpa persona agen, di mana Anda menyediakan semua perilaku dalam prompt pengguna                         | Tidak ada opsi `systemPrompt`        | Default minimal: dukungan tool-calling dan tidak ada yang lain                                                                           |

"Berbeda dari Claude Code" biasanya berarti salah satu dari berikut:

* **Permukaan berbeda**: output tidak dibaca di terminal oleh orang yang memicunya. Chat UI, konsumen output terstruktur, dan otomasi non-coding masing-masing memerlukan prompt yang sesuai dengan cara output mereka dirender dan ditinjau. Otomasi coding tanpa pengawasan, seperti pekerjaan CI yang memperbaiki kesalahan lint atau meninjau diff, masih sesuai dengan preset karena pekerjaan itu sendiri adalah apa yang preset ditulis untuk.
* **Identitas berbeda**: agen tidak boleh menyajikan dirinya sebagai Claude Code. Bot dukungan, asisten analisis data, atau agen khusus domain apa pun memerlukan nama, cakupan, dan persona mereka sendiri.
* **Model izin berbeda**: agen berjalan secara otonom tanpa manusia menyetujui setiap langkah, atau beroperasi pada set sumber daya yang sempit. Prompt Claude Code mengasumsikan manusia berada dalam loop dengan akses ke set tool lengkap.
* **Tugas non-coding**: sebagian besar prompt Claude Code adalah panduan coding. Untuk penelitian, konten, atau agen operasi, panduan itu bersaing dengan instruksi yang benar-benar Anda butuhkan.

Tabel [perbandingan](#compare-the-four-approaches) menunjukkan apa yang dipertahankan oleh setiap metode kustomisasi.

<h2 id="customize-agent-behavior">
  Sesuaikan perilaku agen
</h2>

Output styles, `append`, dan string prompt khusus masing-masing mengubah system prompt secara langsung. CLAUDE.md mengambil jalur yang berbeda: SDK membacanya dan menyuntikkan kontennya ke dalam percakapan sebagai konteks proyek, bukan ke dalam system prompt, sehingga membentuk perilaku bersama dengan system prompt apa pun yang Anda pilih. [Skills](/docs/id/agent-sdk/skills), [hooks](/docs/id/agent-sdk/hooks), dan [permissions](/docs/id/agent-sdk/permissions) juga membentuk perilaku di luar system prompt dan tercakup di halaman terpisahnya.

<h3 id="claude-md-files-for-project-level-instructions">
  File CLAUDE.md untuk instruksi tingkat proyek
</h3>

File CLAUDE.md memberikan Claude konteks proyek dan instruksi yang persisten. SDK menyuntikkan kontennya ke dalam percakapan, bukan ke dalam system prompt, sehingga mereka bekerja dengan konfigurasi system prompt apa pun. Untuk apa yang harus dimasukkan dalam CLAUDE.md, di mana menempatkannya, dan cara menulis instruksi yang efektif, lihat [Bagaimana Claude mengingat proyek Anda](/docs/id/memory). Bagian ini mencakup apa yang spesifik untuk SDK: bagaimana CLAUDE.md dimuat.

SDK membaca CLAUDE.md ketika sumber pengaturan yang sesuai diaktifkan: `'project'` memuat `CLAUDE.md` atau `.claude/CLAUDE.md` dari direktori kerja, dan `'user'` memuat `~/.claude/CLAUDE.md`. Opsi `query()` default mengaktifkan kedua sumber, sehingga CLAUDE.md dimuat secara otomatis. Jika Anda menetapkan `settingSources` di TypeScript atau `setting_sources` di Python secara eksplisit, sertakan sumber yang Anda butuhkan. Pemuatan CLAUDE.md dikendalikan oleh sumber pengaturan, bukan oleh preset `claude_code`.

<h4 id="load-claude-md-with-the-sdk">
  Muat CLAUDE.md dengan SDK
</h4>

Untuk memuat CLAUDE.md, atur `settingSources` untuk menyertakan level tempat CLAUDE.md Anda berada. Contoh di bawah memuat CLAUDE.md tingkat proyek bersama dengan preset `claude_code`, sehingga Claude memiliki prompt agen coding lengkap dan konvensi proyek Anda:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const messages = [];

  for await (const message of query({
    prompt: "Add a new React component for user profiles",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code" // Use Claude Code's system prompt
      },
      settingSources: ["project"] // Loads CLAUDE.md from project
    }
  })) {
    messages.push(message);
  }

  // Now Claude has access to your project guidelines from CLAUDE.md
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  messages = []

  async for message in query(
      prompt="Add a new React component for user profiles",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",  # Use Claude Code's system prompt
          },
          setting_sources=["project"],  # Loads CLAUDE.md from project
      ),
  ):
      messages.append(message)

  # Now Claude has access to your project guidelines from CLAUDE.md
  ```
</CodeGroup>

CLAUDE.md persisten di semua sesi dalam proyek, dibagikan dengan tim Anda melalui git, dan ditemukan secara otomatis tanpa perubahan kode. Tidak dimuat jika Anda melewatkan array `settingSources` kosong.

<h3 id="output-styles-for-persistent-configurations">
  Output styles untuk konfigurasi persisten
</h3>

Output styles adalah konfigurasi yang disimpan yang memodifikasi system prompt Claude. Mereka disimpan sebagai file markdown dan dapat digunakan kembali di berbagai sesi dan proyek.

<h4 id="create-an-output-style">
  Buat output style
</h4>

Output style adalah file markdown dengan [frontmatter](/docs/id/output-styles#frontmatter) untuk metadata, diikuti oleh konten prompt. Simpan ke `~/.claude/output-styles/` untuk style tingkat pengguna yang tersedia di setiap proyek, atau `.claude/output-styles/` di repositori Anda untuk style tingkat proyek yang dapat Anda commit dan bagikan dengan tim Anda.

Secara default, output style khusus menggantikan instruksi rekayasa perangkat lunak dari preset `claude_code` dengan instruksi Anda sendiri. Untuk mempertahankannya dan melapisi instruksi Anda di atasnya, atur `keep-coding-instructions: true` di frontmatter. Pertahankan mereka ketika agen Anda masih melakukan pekerjaan rekayasa perangkat lunak. Tinggalkan mereka ketika Anda mengganti peran sepenuhnya.

Contoh di bawah mendefinisikan persona code-review yang mempertahankan instruksi coding, karena meninjau kode masih mendapat manfaat dari panduan keamanan dan kualitas kode Claude Code. Simpan sebagai `~/.claude/output-styles/code-reviewer.md` untuk membuatnya tersedia di berbagai proyek:

```markdown ~/.claude/output-styles/code-reviewer.md theme={null}
---
name: Code Reviewer
description: Thorough code review assistant
keep-coding-instructions: true
---

You are an expert code reviewer.

For every code submission:
1. Check for bugs and security issues
2. Evaluate performance
3. Suggest improvements
4. Rate code quality (1-10)
```

<h4 id="activate-an-output-style">
  Aktifkan output style
</h4>

Setelah dibuat, aktifkan output styles melalui:

* **CLI**: jalankan `/config` dan pilih output style
* **Settings**: atur `outputStyle` di `.claude/settings.local.json`
* **TypeScript SDK**: atur `outputStyle` di dalam objek `settings` inline yang dilewatkan ke `query()`, atau arahkan `settings` ke file pengaturan yang menetapkannya. `outputStyle` bukan field `Options` tingkat atas:

  ```typescript theme={null}
  const options = { settings: { outputStyle: "Explanatory" } };
  ```

Python SDK tidak memiliki opsi untuk memilih output style secara terprogram. Untuk deployment hanya kode di mana Anda tidak dapat menulis ke `.claude/settings.local.json`, gunakan `append` atau string prompt khusus sebagai gantinya.

**Catatan untuk pengguna SDK:** Output styles dimuat ketika Anda menyertakan `settingSources: ['user']` atau `settingSources: ['project']` (TypeScript) / `setting_sources=["user"]` atau `setting_sources=["project"]` (Python) dalam opsi Anda.

<h3 id="append-to-the-claude_code-preset">
  Tambahkan ke preset `claude_code`
</h3>

Anda dapat menggunakan preset Claude Code dengan properti `append` untuk menambahkan instruksi khusus Anda sambil mempertahankan semua fungsionalitas bawaan.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const messages = [];

  for await (const message of query({
    prompt: "Help me write a Python function to calculate fibonacci numbers",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: "Always include detailed docstrings and type hints in Python code."
      }
    }
  })) {
    messages.push(message);
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  messages = []

  async for message in query(
      prompt="Help me write a Python function to calculate fibonacci numbers",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",
              "append": "Always include detailed docstrings and type hints in Python code.",
          }
      ),
  ):
      messages.append(message)
      if isinstance(message, AssistantMessage):
          print(message.content)
  ```
</CodeGroup>

<h4 id="improve-prompt-caching-across-users-and-machines">
  Tingkatkan prompt caching di berbagai pengguna dan mesin
</h4>

Secara default, dua sesi yang menggunakan preset `claude_code` dan teks `append` yang sama masih tidak dapat berbagi entri cache prompt jika berjalan dari direktori kerja yang berbeda. Ini karena preset menyematkan konteks per-sesi dalam system prompt sebelum teks `append` Anda: direktori kerja, apakah itu repositori git, platform, shell aktif, versi OS, dan jalur auto-memory. Perbedaan apa pun dalam konteks itu menghasilkan system prompt yang berbeda dan cache miss. Konten CLAUDE.md tidak mempengaruhi cache system prompt karena SDK menyuntikkannya ke dalam percakapan, bukan system prompt.

Untuk membuat system prompt identik di berbagai sesi, atur `excludeDynamicSections: true` di TypeScript atau `"exclude_dynamic_sections": True` di Python. Konteks per-sesi berpindah ke pesan pengguna pertama, meninggalkan hanya preset statis dan teks `append` Anda dalam system prompt sehingga konfigurasi identik berbagi entri cache di berbagai pengguna dan mesin.

<Note>
  `excludeDynamicSections` memerlukan `@anthropic-ai/claude-agent-sdk` v0.2.98 atau lebih baru, atau `claude-agent-sdk` v0.1.58 atau lebih baru untuk Python. Ini hanya berlaku untuk bentuk objek preset dan tidak berpengaruh ketika `systemPrompt` adalah string.
</Note>

Contoh berikut memasangkan blok `append` bersama dengan `excludeDynamicSections` sehingga armada agen yang berjalan dari direktori berbeda dapat menggunakan kembali system prompt yang di-cache sama:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Triage the open issues in this repo",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: "You operate Acme's internal triage workflow. Label issues by component and severity.",
        excludeDynamicSections: true
      }
    }
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Triage the open issues in this repo",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",
              "append": "You operate Acme's internal triage workflow. Label issues by component and severity.",
              "exclude_dynamic_sections": True,
          },
      ),
  ):
      ...
  ```
</CodeGroup>

**Tradeoffs:** direktori kerja, flag git-repo, platform, shell aktif, versi OS, dan jalur auto-memory masih mencapai Claude, tetapi sebagai bagian dari pesan pengguna pertama daripada system prompt. Instruksi dalam pesan pengguna memiliki bobot sedikit lebih rendah daripada teks yang sama dalam system prompt, sehingga Claude mungkin mengandalkannya lebih sedikit saat bernalar tentang direktori saat ini atau jalur auto-memory. Aktifkan opsi ini ketika penggunaan kembali cache lintas-sesi lebih penting daripada konteks lingkungan yang paling otoritatif.

Untuk flag yang setara dalam mode CLI non-interaktif, lihat [`--exclude-dynamic-system-prompt-sections`](/docs/id/cli-reference).

<h3 id="custom-system-prompts">
  Custom system prompts
</h3>

Anda dapat menyediakan string khusus sebagai `systemPrompt` untuk mengganti default sepenuhnya dengan instruksi Anda sendiri.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const customPrompt = `You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices`;

  const messages = [];

  for await (const message of query({
    prompt: "Create a data processing pipeline",
    options: {
      systemPrompt: customPrompt
    }
  })) {
    messages.push(message);
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  custom_prompt = """You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices"""

  messages = []

  async for message in query(
      prompt="Create a data processing pipeline",
      options=ClaudeAgentOptions(system_prompt=custom_prompt),
  ):
      messages.append(message)
      if isinstance(message, AssistantMessage):
          print(message.content)
  ```
</CodeGroup>

<h2 id="compare-the-four-approaches">
  Perbandingan keempat pendekatan
</h2>

Keempat metode kustomisasi berbeda dalam hal di mana mereka berada, bagaimana mereka dibagikan, dan apa yang mereka pertahankan dari preset `claude_code`.

| Fitur                   | CLAUDE.md        | Output Styles              | `systemPrompt` dengan append | Custom `systemPrompt`       |
| ----------------------- | ---------------- | -------------------------- | ---------------------------- | --------------------------- |
| **Persistence**         | File per-proyek  | Disimpan sebagai file      | Hanya sesi                   | Hanya sesi                  |
| **Reusability**         | Per-proyek       | Di berbagai proyek         | Duplikasi kode               | Duplikasi kode              |
| **Management**          | Di filesystem    | CLI + file                 | Dalam kode                   | Dalam kode                  |
| **Default tools**       | Dipertahankan    | Dipertahankan              | Dipertahankan                | Hilang (kecuali disertakan) |
| **Built-in safety**     | Dipertahankan    | Dipertahankan              | Dipertahankan                | Harus ditambahkan           |
| **Environment context** | Otomatis         | Otomatis                   | Otomatis                     | Harus disediakan            |
| **Customization level** | Hanya penambahan | Ganti atau perluas default | Hanya penambahan             | Kontrol penuh               |
| **Version control**     | Dengan proyek    | Ya                         | Dengan kode                  | Dengan kode                 |
| **Scope**               | Spesifik proyek  | Pengguna atau proyek       | Sesi kode                    | Sesi kode                   |

"Dengan append" berarti menggunakan `systemPrompt: { type: "preset", preset: "claude_code", append: "..." }` di TypeScript atau `system_prompt={"type": "preset", "preset": "claude_code", "append": "..."}` di Python. CLAUDE.md tidak mengubah prompt sistem itu sendiri: SDK menyuntikkan kontennya ke dalam percakapan sebagai konteks proyek.

<h2 id="use-cases-and-best-practices">
  Use cases dan best practices
</h2>

<h3 id="when-to-use-claude-md">
  Kapan menggunakan CLAUDE.md
</h3>

Gunakan CLAUDE.md untuk instruksi yang harus berlaku untuk setiap sesi dalam proyek, terlepas dari system prompt mana yang digunakan sesi tersebut: standar coding, perintah umum, konteks arsitektur, dan konvensi tim. CLAUDE.md berkomitmen pada repositori Anda, sehingga tetap sinkron dengan kode yang dijelaskannya. Lihat [When to add to CLAUDE.md](/docs/id/memory#when-to-add-to-claude-md) untuk panduan lengkap.

File CLAUDE.md dimuat ketika sumber pengaturan `project` diaktifkan, yang merupakan default untuk opsi `query()`. Jika Anda menetapkan `settingSources` dalam TypeScript atau `setting_sources` dalam Python secara eksplisit, sertakan `'project'` untuk terus memuat CLAUDE.md tingkat proyek.

<h3 id="when-to-use-output-styles">
  Kapan menggunakan output styles
</h3>

Output styles adalah untuk persona yang ingin Anda gunakan kembali di seluruh CLI dan SDK tanpa mengubah kode aplikasi. Karena mereka berada sebagai file di `.claude/output-styles`, persona yang sama tersedia dari `/config` di CLI dan dari sesi SDK apa pun yang memuat sumber pengaturan yang sesuai.

**Terbaik untuk:**

* Perubahan perilaku persisten di berbagai sesi
* Konfigurasi bersama tim
* Asisten khusus seperti code reviewer, data scientist, atau asisten DevOps
* Modifikasi prompt kompleks yang memerlukan versioning

**Contoh:**

* Membuat asisten optimasi SQL khusus
* Membangun code reviewer yang berfokus pada keamanan
* Mengembangkan asisten pengajaran dengan pedagogi spesifik

<h3 id="when-to-use-systemprompt-with-append">
  Kapan menggunakan `systemPrompt` dengan append
</h3>

Gunakan `append` ketika preset `claude_code` sudah sesuai dengan produk Anda dan Anda hanya perlu menambahkan instruksi tambahan. Anda mempertahankan panduan tool preset, aturan keamanan, dan konvensi coding tanpa mengimplementasikannya kembali.

**Terbaik untuk:**

* Menambahkan standar coding atau preferensi spesifik
* Menyesuaikan pemformatan output
* Menambahkan pengetahuan domain-spesifik
* Memodifikasi verbositas respons
* Meningkatkan perilaku default Claude Code tanpa kehilangan instruksi tool

<h3 id="when-to-use-custom-systemprompt">
  Kapan menggunakan custom `systemPrompt`
</h3>

Gunakan prompt kustom ketika permukaan agen, identitas, atau model izin Anda berbeda dari Claude Code, seperti yang dijelaskan dalam [Decide on a starting point](#decide-on-a-starting-point). Anda menentukan set instruksi lengkap, termasuk panduan tool dan aturan keamanan apa pun yang dibutuhkan agen Anda.

**Terbaik untuk:**

* Kontrol penuh atas perilaku Claude
* Tugas khusus sesi tunggal
* Menguji strategi prompt baru
* Situasi di mana tool default tidak diperlukan
* Membangun agen khusus dengan perilaku unik

<h2 id="combine-approaches">
  Menggabungkan pendekatan
</h2>

Metode-metode ini dapat dikomposisi. Gaya output yang persisten atau CLAUDE.md menetapkan perilaku jangka panjang, dan `append` menambahkan instruksi spesifik sesi di atasnya tanpa menyentuh konfigurasi yang disimpan.

<h3 id="combine-an-output-style-with-session-specific-additions">
  Menggabungkan gaya output dengan penambahan spesifik sesi
</h3>

Contoh di bawah ini mengasumsikan gaya output Code Reviewer sudah aktif. Blok `append` menambahkan area fokus spesifik sesi di atas persona, sehingga sesi review tunggal dapat memprioritaskan OAuth dan penyimpanan token tanpa mengubah gaya output yang disimpan:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Assuming "Code Reviewer" output style is active (via /config or settings)
  // Add session-specific focus areas
  const messages = [];

  for await (const message of query({
    prompt: "Review this authentication module",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: `
          For this review, prioritize:
          - OAuth 2.0 compliance
          - Token storage security
          - Session management
        `
      }
    }
  })) {
    messages.push(message);
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  # Assuming "Code Reviewer" output style is active (via /config or settings)
  # Add session-specific focus areas
  messages = []

  async for message in query(
      prompt="Review this authentication module",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",
              "append": """
              For this review, prioritize:
              - OAuth 2.0 compliance
              - Token storage security
              - Session management
              """,
          }
      ),
  ):
      messages.append(message)
  ```
</CodeGroup>

<h2 id="see-also">
  Lihat juga
</h2>

* [Output styles](/docs/id/output-styles): buat, kelola, dan bagikan output styles untuk CLI, termasuk format file dan lokasi penyimpanan
* [Bagaimana Claude mengingat proyek Anda](/docs/id/memory): apa yang harus dimasukkan dalam CLAUDE.md, di mana menempatkannya, dan cara menulis instruksi proyek yang efektif
* [Referensi TypeScript SDK](/docs/id/agent-sdk/typescript): tipe `Options` lengkap, termasuk `systemPrompt`, `settingSources`, dan `settings`
* [Referensi Python SDK](/docs/id/agent-sdk/python): tipe `ClaudeAgentOptions` lengkap, termasuk `system_prompt` dan `setting_sources`
* [Settings](/docs/id/settings): referensi `settings.json`, termasuk di mana output styles dan konfigurasi lainnya disimpan
