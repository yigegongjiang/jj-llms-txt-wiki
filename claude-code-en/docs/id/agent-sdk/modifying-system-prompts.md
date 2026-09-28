> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Memodifikasi system prompts

> Pilih antara preset `claude_code` dan system prompt kustom, serta sesuaikan perilaku dengan CLAUDE.md, output styles, append, atau prompt yang sepenuhnya kustom.

System prompts mendefinisikan perilaku Claude, kemampuan, dan gaya respons. Mulai dari preset `claude_code` untuk alat coding seperti CLI atau IDE di mana manusia mengawasi dan mengarahkan pekerjaan. Tulis prompt Anda sendiri untuk agen dengan permukaan, identitas, atau model izin yang berbeda.

<h2 id="how-system-prompts-work">
  Cara kerja system prompts
</h2>

Sebuah system prompt adalah set instruksi awal yang membentuk bagaimana Claude berperilaku sepanjang percakapan. Agent SDK memiliki tiga titik awal untuk itu:

* **Default minimal**: ketika Anda tidak menetapkan `systemPrompt` di TypeScript atau `system_prompt` di Python, SDK menggunakan prompt minimal yang mencakup tool calling tetapi menghilangkan sisa konten preset `claude_code`, termasuk instruksi keamanan dan keselamatannya serta konteksnya tentang direktori kerja dan lingkungan. Ini berbeda dari `claude -p`, yang menggunakan system prompt Claude Code secara default. Jika Anda bermigrasi dari CLI dan menginginkan perilaku yang cocok, atur preset `claude_code`.
* **Preset `claude_code`**: system prompt yang digunakan CLI Claude Code, dengan instruksi penggunaan tool, instruksi keamanan dan keselamatan, dan konteks tentang direktori kerja dan lingkungan. Atur `systemPrompt: { type: "preset", preset: "claude_code" }` di TypeScript atau `system_prompt={"type": "preset", "preset": "claude_code"}` di Python, secara opsional dengan `append` untuk menambahkan instruksi Anda sendiri di akhir.
* **String kustom**: prompt yang Anda tulis sendiri. SDK hanya mengirimkan apa yang Anda berikan.

<h3 id="decide-on-a-starting-point">
  Tentukan titik awal
</h3>

Faktor penentu adalah seberapa dekat agen Anda menyerupai Claude Code: agen coding yang beroperasi di repositori, dengan manusia mengamati output streaming dan mengarahkan pekerjaan. Semakin jauh produk Anda dari itu, semakin banyak Anda akan ingin menulis prompt Anda sendiri.

| Anda membangun                                                                                                                    | Gunakan                              | Apa yang Anda dapatkan                                                                                                                   |
| :-------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| Alat coding seperti CLI atau IDE di mana manusia mengamati dan mengarahkan, dan default Claude Code adalah apa yang Anda inginkan | Preset `claude_code`                 | Prompt Claude Code, termasuk panduan tool, aturan keselamatan, dan konteks lingkungan                                                    |
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

`append` dan string prompt khusus masing-masing mengubah system prompt secara langsung, dan output style mengubah instruksi yang Claude Code berikan kepada Claude untuk setiap respons. CLAUDE.md mengambil jalur yang berbeda: SDK membacanya dan menyuntikkan kontennya ke dalam percakapan sebagai konteks proyek, sehingga membentuk perilaku bersama dengan system prompt apa pun yang Anda pilih. [Skills](/docs/id/agent-sdk/skills), [hooks](/docs/id/agent-sdk/hooks), dan [permissions](/docs/id/agent-sdk/permissions) juga membentuk perilaku di luar system prompt dan tercakup di halaman terpisahnya.

<h3 id="claude-md-files-for-project-level-instructions">
  File CLAUDE.md untuk instruksi tingkat proyek
</h3>

File CLAUDE.md memberikan Claude konteks proyek dan instruksi yang persisten. SDK menyuntikkan kontennya ke dalam percakapan dan membiarkan system prompt tidak berubah, sehingga mereka bekerja dengan konfigurasi system prompt apa pun. Untuk apa yang harus dimasukkan dalam CLAUDE.md, di mana menempatkannya, dan cara menulis instruksi yang efektif, lihat [Kapan menambahkan ke CLAUDE.md](/docs/id/memory#when-to-add-to-claude-md) dan sisanya dari [Bagaimana Claude mengingat proyek Anda](/docs/id/memory). Bagian ini mencakup apa yang spesifik untuk SDK: bagaimana CLAUDE.md dimuat.

SDK membaca CLAUDE.md ketika sumber pengaturan yang sesuai diaktifkan: `'project'` memuat `CLAUDE.md` atau `.claude/CLAUDE.md` dari direktori kerja, dan `'user'` memuat `~/.claude/CLAUDE.md`. Opsi `query()` default mengaktifkan kedua sumber, sehingga CLAUDE.md dimuat secara otomatis. Jika Anda menetapkan `settingSources` di TypeScript atau `setting_sources` di Python secara eksplisit, sertakan sumber yang Anda butuhkan. Pemuatan CLAUDE.md dikendalikan oleh sumber pengaturan, bukan oleh preset `claude_code`.

<h4 id="load-claude-md-with-the-sdk">
  Muat CLAUDE.md dengan SDK
</h4>

Untuk memuat CLAUDE.md, atur `settingSources` untuk menyertakan level tempat Anda menyimpan CLAUDE.md Anda. Contoh di bawah memuat CLAUDE.md tingkat proyek bersama dengan preset `claude_code`, sehingga Claude memiliki prompt agen coding lengkap dan konvensi proyek Anda:

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
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  messages = []


  async def main():
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


  asyncio.run(main())

  # Now Claude has access to your project guidelines from CLAUDE.md
  ```
</CodeGroup>

Ketika Anda menjalankan salah satu contoh, SDK melakukan streaming pesan saat Claude bekerja: pesan inisialisasi sistem, pesan asisten, pesan pengguna yang membawa hasil alat, dan pesan hasil akhir dengan hasil sesi.

CLAUDE.md persisten di semua sesi dalam proyek, dibagikan dengan tim Anda melalui git, dan ditemukan secara otomatis tanpa perubahan kode. Tidak dimuat jika Anda melewatkan array `settingSources` kosong.

<h3 id="output-styles-for-persistent-configurations">
  Output styles untuk konfigurasi persisten
</h3>

Output styles adalah konfigurasi yang disimpan dari instruksi yang mengubah peran, nada, dan format output Claude. Mereka disimpan sebagai file markdown dan dapat digunakan kembali di berbagai sesi dan proyek.

<h4 id="create-an-output-style">
  Buat output style
</h4>

Output style adalah file markdown dengan [frontmatter](/docs/id/output-styles#frontmatter) untuk metadata, diikuti oleh konten prompt. Simpan ke `~/.claude/output-styles/` untuk style tingkat pengguna yang tersedia di setiap proyek, atau `.claude/output-styles/` di repositori Anda untuk style tingkat proyek yang dapat Anda commit dan bagikan dengan tim Anda.

Output style khusus meninggalkan instruksi rekayasa perangkat lunak dari preset `claude_code` dan menggunakan instruksi Anda sendiri. Untuk mempertahankannya dan melapisi instruksi Anda di atasnya, atur `keep-coding-instructions: true` di frontmatter. Instruksi tersebut hanya ada dalam system prompt lengkap Claude Code, jadi pengaturan ini tidak berpengaruh dalam sesi pada system prompt yang lebih pendek, yang Anda pin aktif atau nonaktif dengan [`CLAUDE_CODE_SIMPLE_SYSTEM_PROMPT`](/docs/id/env-vars#variables). Pertahankan mereka ketika agen Anda masih melakukan pekerjaan rekayasa perangkat lunak. Tinggalkan mereka ketika Anda mengganti peran sepenuhnya.

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

* **CLI**: jalankan `/output-style <style>`, misalnya `/output-style concise`, atau jalankan `/config` dan pilih satu. Perintah `/output-style` memerlukan Claude Code v2.1.269 atau lebih baru.
* **Settings**: atur `outputStyle` di `.claude/settings.local.json`
* **TypeScript SDK**: atur `outputStyle` di dalam objek `settings` inline yang dilewatkan ke `query()`, atau arahkan `settings` ke file pengaturan yang menetapkannya. `outputStyle` bukan field `Options` tingkat atas:

  ```typescript theme={null}
  const options = { settings: { outputStyle: "Explanatory" } };
  ```

Dalam Python SDK, atur `outputStyle` melalui opsi `settings`, yang mengambil string JSON seperti `'{"outputStyle": "Explanatory"}'` atau jalur ke file pengaturan yang menetapkannya.

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
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  messages = []


  async def main():
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


  asyncio.run(main())
  ```
</CodeGroup>

<h4 id="improve-prompt-caching-across-users-and-machines">
  Tingkatkan prompt caching di berbagai pengguna dan mesin
</h4>

Secara default, dua sesi yang menggunakan preset `claude_code` dan teks `append` yang sama masih tidak dapat berbagi entri cache prompt jika berjalan dari direktori kerja yang berbeda. Ini karena preset menyematkan konteks per-sesi dalam system prompt sebelum teks `append` Anda: direktori kerja, apakah itu repositori git, platform, shell aktif, versi OS, dan jalur auto-memory. Perbedaan apa pun dalam konteks itu menghasilkan system prompt yang berbeda dan cache miss. Konten CLAUDE.md tidak mempengaruhi cache system prompt karena SDK menyuntikkannya ke dalam percakapan, bukan system prompt.

Untuk membuat system prompt identik di berbagai sesi, atur `excludeDynamicSections: true` di TypeScript atau `"exclude_dynamic_sections": True` di Python. Konteks per-sesi berpindah ke pesan pengguna pertama, meninggalkan hanya preset statis dan teks `append` Anda dalam system prompt sehingga konfigurasi identik berbagi entri cache di berbagai pengguna dan mesin.

<Note>
  `excludeDynamicSections` memerlukan `@anthropic-ai/claude-agent-sdk` v0.2.98 atau lebih baru, atau `claude-agent-sdk` v0.1.58 atau lebih baru untuk Python. Atur pada bentuk objek preset saja. SDK mengabaikannya ketika Anda melewatkan prompt khusus alih-alih preset; untuk menjaga instruksi prompt khusus yang di-cache dalam TypeScript SDK, lihat [Cache bagian statis dari prompt khusus](#cache-the-static-part-of-a-custom-prompt).
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
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
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


  asyncio.run(main())
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
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  custom_prompt = """You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices"""

  messages = []


  async def main():
      async for message in query(
          prompt="Create a data processing pipeline",
          options=ClaudeAgentOptions(system_prompt=custom_prompt),
      ):
          messages.append(message)
          if isinstance(message, AssistantMessage):
              print(message.content)


  asyncio.run(main())
  ```
</CodeGroup>

Di Python, muat prompt khusus besar dari file dengan `system_prompt={"type": "file", "path": "..."}` alih-alih meneruskannya sebagai string. Python SDK melewatkan prompt string sebagai satu argumen baris perintah ke subprocess CLI, sehingga prompt yang melebihi batas panjang argumen OS gagal saat pemijahan proses sebelum permintaan API apa pun dikirim. Di Linux kesalahannya adalah `Argument list too long`. Lihat [`SystemPromptFile`](/docs/id/agent-sdk/python#systempromptfile) untuk ambang batas platform dan perilaku Windows.

<h4 id="cache-the-static-part-of-a-custom-prompt">
  Cache bagian statis dari prompt khusus
</h4>

Dalam TypeScript SDK, Anda dapat melewatkan prompt khusus sebagai array string alih-alih satu string, dengan penanda `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` antara bagian statis dan sisanya. Gunakan ini ketika prompt Anda menggabungkan instruksi yang sama pada setiap permintaan dengan konteks yang berubah per permintaan, seperti pelanggan atau tiket yang ditangani agen. Ketika Anda melewatkan kedua bagian sebagai satu string, perubahan pada bagian per-permintaan mengubah seluruh system prompt, sehingga instruksi statis melewatkan cache juga. Bentuk array tidak tersedia dalam Python SDK; [`ClaudeAgentOptions`](/docs/id/agent-sdk/python#claudeagentoptions) mencantumkan bentuk yang `system_prompt` terima.

<Note>
  SDK membagi prompt hanya ketika memanggil Claude API secara langsung atau berjalan di [Claude Platform on AWS](/docs/id/claude-platform-on-aws). Dalam setiap konfigurasi lain, seperti Amazon Bedrock, Agent Platform Google Cloud, Microsoft Foundry, atau [LLM gateway](/docs/id/llm-gateway-connect), dan kapan pun Anda menetapkan [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/id/llm-gateway-protocol#disable-pre-release-capabilities), SDK mengirim seluruh prompt sebagai satu blok, sama seperti melewatkan satu string.
</Note>

Untuk membagi prompt, impor `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` dari `@anthropic-ai/claude-agent-sdk` dan lewatkan sebagai elemen array terpisahnya antara dua bagian. SDK mengirim string sebelum penanda sebagai satu blok teks dan string setelahnya sebagai blok kedua, masing-masing dengan titik henti cache-nya sendiri. Dalam contoh di bawah, agen dukungan memuat instruksi triase dari file dan menerima detail tentang satu tiket pada setiap permintaan, sehingga instruksi tetap di-cache sementara detail tiket berubah:

```typescript TypeScript theme={null}
import { readFile } from "node:fs/promises";
import { query, SYSTEM_PROMPT_DYNAMIC_BOUNDARY } from "@anthropic-ai/claude-agent-sdk";

// Identical on every request
const instructions = await readFile("triage-instructions.md", "utf8");
// Different on every request
const ticketContext = "Customer plan: Enterprise. Other open tickets from this customer: 3.";

for await (const message of query({
  prompt: "Triage ticket 4821",
  options: {
    systemPrompt: [instructions, SYSTEM_PROMPT_DYNAMIC_BOUNDARY, ticketContext]
  }
})) {
  // ...
}
```

[Track cache tokens](/docs/id/agent-sdk/cost-tracking#track-cache-tokens) menjelaskan field `cache_creation_input_tokens` dan `cache_read_input_tokens` pada setiap pesan hasil.

SDK merakit blok dari array sebagai berikut:

* SDK menggabungkan string di setiap sisi penanda dengan baris kosong di antara mereka dan menghapus penanda itu sendiri, sehingga teks penanda tidak mencapai Claude.
* Jika Anda menyertakan penanda lebih dari sekali, yang pertama adalah pemisahan dan SDK menghapus yang lain.
* Jika Anda meninggalkan penanda, SDK menggabungkan semua string menjadi satu blok, sama seperti melewatkan satu string.

Dengan flag [`--system-prompt` atau `--system-prompt-file`](/docs/id/cli-reference#system-prompt-flags) CLI, prompt adalah satu string, sehingga tidak ada array untuk membawa penanda. Sertakan baris yang hanya berisi `__SYSTEM_PROMPT_DYNAMIC_BOUNDARY__` antara bagian statis dan per-permintaan sebagai gantinya. Claude Code membagi prompt pada baris pertama seperti itu menjadi dua blok yang sama dan menghapus baris itu. Memerlukan Claude Code v2.1.275 atau lebih baru.

Dalam SDK, lebih suka bentuk array, yang membawa batas tanpa baris penanda.

<h3 id="change-the-prompt-of-an-existing-session">
  Ubah prompt dari sesi yang ada
</h3>

Secara default, jika Anda melewatkan `append` atau prompt khusus yang berbeda ketika Anda kembali ke sesi dengan `resume` atau `continue`, Claude tidak melihatnya pada giliran berikutnya. Claude Code mencatat system prompt pada permintaan pertama sesi dan menggunakan kembali catatan itu sampai sesi dikompakkan. Teks baru berlaku setelah pemadatan itu, atau dalam sesi baru.

<h4 id="update-claude’s-instructions-mid-session">
  Perbarui instruksi Claude di tengah-sesi
</h4>

Jika instruksi yang Anda masukkan dalam system prompt perlu berubah saat sesi berjalan, misalnya karena pengguna Anda beralih agen ke mode read-only atau mengedit konfigurasinya di aplikasi Anda, kirim instruksi baru dalam percakapan alih-alih mengubah `systemPrompt`:

* **Dalam pesan Anda berikutnya**: sertakan instruksi baru dalam pesan pengguna berikutnya yang Anda kirim.
* **Dari hook**: kembalikan [`additionalContext`](/docs/id/hooks#add-context-for-claude) dari callback hook `UserPromptSubmit` atau `PostToolUse` [hook callback](/docs/id/agent-sdk/hooks#outputs), ditulis sebagai pernyataan faktual seperti "Workspace sekarang read-only". SDK menyisipkan teks ke dalam percakapan pada titik di mana hook dipecat, sehingga prompt yang dicatat tetap tidak berubah.

<h4 id="turn-recording-off-while-you-iterate-on-wording">
  Matikan pencatatan saat Anda mengulangi redaksi
</h4>

Saat Anda mengulangi redaksi prompt dan ingin setiap edit mencapai sesi yang Anda lanjutkan, atur `snapshot` ke false pada bentuk objek system prompt. Claude Code kemudian membangun kembali prompt pada setiap permintaan. Field tersedia pada bentuk preset dan custom dari [`systemPrompt`](/docs/id/agent-sdk/typescript#options) di TypeScript dan dari [`system_prompt`](/docs/id/agent-sdk/python#systempromptpreset) di Python, dan memerlukan `@anthropic-ai/claude-agent-sdk` v0.3.257 atau lebih baru, atau `claude-agent-sdk` v0.2.153 atau lebih baru.

Pertahankan pencatatan aktif dalam produksi. Dengan pencatatan mati, `append` atau prompt khusus yang berbeda pada sesi yang dilanjutkan mencapai Claude pada giliran berikutnya, dan permintaan itu tidak dapat menggunakan kembali [prompt cache](/docs/id/prompt-caching#how-the-cache-is-organized) sesi. Di mana API memberlakukan [preserved thinking](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking), Claude juga kehilangan pemikirannya dari giliran sebelumnya.

Di luar [cloud sessions](/docs/id/cloud-environments), jika Anda memulai Claude Code dalam [bare mode](/docs/id/headless#start-faster-with-bare-mode) dengan melewatkan `--bare` melalui `extraArgs` atau menetapkan `CLAUDE_CODE_SIMPLE=1`, pencatatan tetap mati kecuali Anda menetapkan `snapshot: true`.

Pencatatan `append` atau prompt khusus secara default memerlukan Claude Code v2.1.265 atau lebih baru, yang TypeScript Agent SDK bundel dari v0.3.265 dan Python Agent SDK dari v0.2.153. Sebelum Claude Code v2.1.268, sesi yang tidak [mengambil feature flags](/docs/id/env-vars#features-that-need-feature-flag-fetching), termasuk sesi di Amazon Bedrock, Agent Platform Google Cloud, dan Microsoft Foundry, membangun kembali prompt pada setiap permintaan dan `snapshot` tidak berpengaruh.

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

<h2 id="combine-approaches">
  Menggabungkan pendekatan
</h2>

Pendekatan-pendekatan ini dapat digabungkan. Gaya output yang persisten atau CLAUDE.md menetapkan perilaku jangka panjang, dan `append` menambahkan instruksi spesifik sesi di atasnya tanpa menyentuh konfigurasi yang disimpan.

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
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  # Assuming "Code Reviewer" output style is active (via /config or settings)
  # Add session-specific focus areas
  messages = []


  async def main():
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


  asyncio.run(main())
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
