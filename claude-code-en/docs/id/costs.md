> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Kelola biaya secara efektif

> Lacak penggunaan token, tetapkan batas pengeluaran tim, dan kurangi biaya Claude Code dengan manajemen konteks, pemilihan model, pengaturan pemikiran yang diperluas, dan hook prapemrosesan.

Claude Code mengenakan biaya berdasarkan konsumsi token API. Untuk harga paket langganan (Pro, Max, Team, Enterprise), lihat [claude.com/pricing](https://claude.com/pricing). Biaya per pengembang bervariasi luas berdasarkan pemilihan model, ukuran basis kode, dan pola penggunaan seperti menjalankan beberapa instans atau otomasi.

Di seluruh penyebaran perusahaan, biaya rata-rata adalah sekitar \$13 per pengembang per hari aktif dan \$150-250 per pengembang per bulan, dengan biaya tetap di bawah \$30 per hari aktif untuk 90% pengguna. Untuk memperkirakan pengeluaran untuk tim Anda sendiri, mulai dengan kelompok pilot kecil dan gunakan alat pelacakan di bawah untuk membangun baseline sebelum peluncuran yang lebih luas.

Halaman ini mencakup cara [melacak biaya Anda](#track-your-costs), [mengelola biaya untuk organisasi Anda](#manage-costs-for-your-organization), dan [mengurangi penggunaan token](#reduce-token-usage).

<h2 id="track-your-costs">
  Lacak biaya Anda
</h2>

<h3 id="using-the-/usage-command">
  Menggunakan perintah `/usage`
</h3>

<Note>
  Blok Session dalam `/usage` menampilkan penggunaan token API dan dimaksudkan untuk pengguna API. Pelanggan Claude Max dan Pro memiliki penggunaan yang disertakan dalam langganan mereka, jadi angka biaya sesi tidak relevan untuk tujuan penagihan. Pelanggan melihat bilah penggunaan paket, statistik aktivitas, dan rincian penggunaan di layar yang sama.
</Note>

Blok Session di bagian atas `/usage` menampilkan statistik penggunaan token terperinci untuk sesi Anda saat ini. Angka dolar adalah perkiraan yang dihitung secara lokal dari jumlah token dan mungkin berbeda dari tagihan aktual Anda. Untuk penagihan yang berwenang, lihat halaman Penggunaan di [Claude Console](https://platform.claude.com/usage).

```text theme={null}
Total cost:            $0.55
Total duration (API):  6m 19.7s
Total duration (wall): 6h 33m 10.2s
Total code changes:    0 lines added, 0 lines removed
```

Pada paket Pro, Max, Team, atau Enterprise, `/usage` juga menampilkan rincian tentang apa yang diperhitungkan terhadap batas paket Anda. Ini mengatribusikan penggunaan terbaru ke skills, subagents, plugins, dan server MCP individual, masing-masing ditampilkan sebagai persentase dari total. Tekan `d` atau `w` untuk beralih antara 24 jam terakhir dan 7 hari terakhir. Angka-angka tersebut bersifat perkiraan dan dihitung dari riwayat sesi lokal di mesin ini, jadi penggunaan dari perangkat lain atau claude.ai tidak disertakan.

Ketika permintaan untuk batas paket Anda gagal, paling sering karena endpoint penggunaan dibatasi laju, `/usage` menampilkan bilah penggunaan terakhir yang dimuat di mesin ini dalam 60 menit terakhir, bersama dengan catatan `Showing last-known usage` yang menyatakan berapa lama yang lalu data tersebut diambil. Tekan `r` untuk mencoba lagi; percobaan ulang yang berhasil menggantikan bilah terakhir yang diketahui dengan data segar. Tanpa snapshot dari 60 menit terakhir, `/usage` melaporkan bahwa endpoint penggunaan dibatasi laju dan menawarkan pintasan percobaan ulang yang sama. Sebelum v2.1.208, permintaan yang dibatasi laju dalam sesi yang belum memuat penggunaan selalu menampilkan kesalahan tanpa bilah.

Di [ekstensi VS Code](/docs/id/vs-code#check-account-and-usage), rincian yang sama muncul dalam dialog Account & usage dengan toggle Day dan Week. Memerlukan Claude Code v2.1.174 atau lebih baru.

<h3 id="set-a-spend-limit-on-pro-and-max">
  Tetapkan batas pengeluaran pada Pro dan Max
</h3>

Pada paket Pro dan Max, perintah `/usage-credits` membuka dialog di CLI tempat Anda mengelola [kredit penggunaan](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans). Dari dialog Anda dapat:

* Mengaktifkan kredit penggunaan untuk akun Anda
* Membeli lebih banyak kredit penggunaan, baik paket yang terdaftar atau jumlah khusus
* Menetapkan, mengubah, atau menghapus batas pengeluaran bulanan Anda
* Mengonfigurasi auto-reload, yang secara otomatis membeli lebih banyak kredit penggunaan ketika saldo Anda turun di bawah ambang batas yang Anda tetapkan

Pada versi Claude Code sebelum v2.1.207 dan pada akun tempat dialog dalam CLI tidak tersedia, `/usage-credits` membuka halaman penagihan kredit penggunaan di browser Anda. Pada paket Team dan Enterprise, anggota dengan akses penagihan mendapatkan halaman browser yang sama, dan anggota tanpa akses penagihan mengirim permintaan dari CLI meminta admin mereka untuk mengaktifkan kredit penggunaan atau menaikkan batas.

Mengubah batas pengeluaran bulanan memerlukan akses penagihan pada akun. Jika Anda mencapai batas sementara Anda masih memiliki kredit penggunaan yang tersedia, Claude Code meminta Anda untuk menaikkan atau menghapusnya sehingga Anda dapat melanjutkan tanpa meninggalkan CLI.

Jumlah yang Anda ketik ke dalam dialog, seperti jumlah pembelian khusus, batas pengeluaran bulanan, atau ambang batas auto-reload dan target, harus berupa digit, secara opsional diikuti oleh titik dan satu atau dua digit desimal, misalnya `20` atau `20.50`. Input apa pun yang lain, termasuk koma, menampilkan kesalahan inline dan tidak disimpan. Versi sebelum v2.1.207 tidak menampilkan dialog dan membuka halaman penagihan.

Claude Code meminta Anda untuk mengetik `yes` untuk mengonfirmasi setiap pembelian dan setiap perubahan auto-reload, berapa pun jumlahnya, dan konfirmasi pembelian menampilkan total setelah pajak yang Anda setujui. Mengubah batas pengeluaran bulanan meminta konfirmasi yang diketik sama hanya di atas \$1.000, atau di atas 1.000 unit mata uang penagihan non-dolar AS. Sebelum v2.1.208, pembelian dan perubahan auto-reload menggunakan ambang batas itu juga, jadi jumlah yang lebih kecil melalui alur dialog standar tanpa langkah `yes` yang diketik tambahan.

Bidang jumlah terbuka dengan nilai yang disarankan sebelumnya, dan digit pertama yang Anda ketik menggantikan saran alih-alih menambahkannya. Layar yang mengaktifkan kredit penggunaan terbuka dengan Cancel dipilih, jadi mengaktifkannya memerlukan pemilihan yang disengaja daripada Enter yang tersesat. Keduanya memerlukan Claude Code v2.1.208 atau lebih baru.

<h2 id="manage-costs-for-your-organization">
  Mengelola biaya untuk organisasi Anda
</h2>

Kontrol mana yang Anda miliki tergantung pada bagaimana organisasi Anda mengakses Claude Code: paket Claude for Teams atau Enterprise, Claude Console, atau penyedia cloud. Pada paket Teams dan Enterprise, penggunaan ditarik dari tunjangan kursi setiap anggota. Di Console dan di penyedia cloud, penggunaan ditagih per token ke organisasi Anda. Jika organisasi Anda mencampur metode masuk, setiap pengembang diukur sesuai dengan yang mereka autentikasi.

Tabel memetakan setiap pengaturan ke tempat Anda melihat pengeluaran, tempat Anda membatasinya, dan bagaimana Anda menarik angka per pengguna.

| Pengaturan Anda                                                                           | Lihat pengeluaran                                                                                                                                   | Batasi pengeluaran                       | Pelaporan per pengguna                                                                                                                                                                                                           |
| :---------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Claude for Teams atau Enterprise](#claude-for-teams-and-enterprise)                      | [Laporan pengeluaran dalam analitik organisasi](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) | Batas pengeluaran dalam pengaturan admin | [CSV laporan pengeluaran](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans); [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) di Enterprise |
| [Claude Console (API)](#claude-console)                                                   | [Halaman penggunaan Console](https://platform.claude.com/usage)                                                                                     | Batas pengeluaran ruang kerja            | [Dashboard Console](https://platform.claude.com/claude-code), [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api)                                                       |
| [Amazon Bedrock, Google Cloud's Agent Platform, atau Microsoft Foundry](#cloud-providers) | Konsol penagihan cloud Anda                                                                                                                         | Kontrol anggaran cloud Anda              | [OpenTelemetry](/docs/id/monitoring-usage) atau [gateway LLM](/docs/id/llm-gateway)                                                                                                                                                        |

[Ekspor OpenTelemetry](/docs/id/monitoring-usage) bekerja pada setiap pengaturan dan merupakan satu-satunya opsi yang mengalirkan metrik token dan biaya per pengguna ke dalam tumpukan observabilitas Anda sendiri secara real-time.

<h3 id="claude-for-teams-and-enterprise">
  Claude for Teams dan Enterprise
</h3>

Pada paket Claude for Teams dan Enterprise, penggunaan Claude Code setiap anggota ditarik dari tunjangan per-kursi yang direset pada jendela lima jam bergulir dan jendela mingguan. Tunjangan dibagikan dengan Claude chat dan Cowork, dan ukurannya tergantung pada [tingkat kursi](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan) anggota (Standard atau Premium). Kontrol Anda berada di konsol admin claude.ai, bukan Claude Console.

* **Lihat pengeluaran**: [laporan pengeluaran dalam analitik organisasi](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) menunjukkan pengeluaran perkiraan per pengguna dan per model, dengan ekspor CSV, diperbarui setiap hari. Laporan mencakup pengeluaran kredit penggunaan dan muncul setelah kredit penggunaan diaktifkan. Penggunaan dalam tunjangan kursi tidak diukur dalam dolar.
* **Lihat adopsi**: [dashboard analitik](https://claude.ai/analytics/claude-code) menunjukkan pengguna aktif harian, sesi, dan metrik kontribusi, dengan ekspor CSV data kontribusi. Lihat [lacak penggunaan tim dengan analitik](/docs/id/analytics).
* **Batasi pengeluaran**: tunjangan kursi adalah batas default. Untuk membiarkan anggota melanjutkan melampauinya, aktifkan [kredit penggunaan](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) dan tetapkan batas pengeluaran di tingkat organisasi, grup, atau anggota individual.
* **Tarik angka per pengguna**: pada paket Enterprise, [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) mengembalikan laporan penggunaan dan biaya per pengguna di seluruh permukaan Claude, termasuk Claude Code. Pemilik Utama membuat kunci dengan cakupan `read:analytics` di [claude.ai/analytics/api-keys](https://claude.ai/analytics/api-keys). Pada paket Teams, ekspor [CSV laporan pengeluaran](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans), yang mencantumkan penggunaan token dan pengeluaran perkiraan per pengguna dan per model.

[Panduan konsumsi Claude Enterprise](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide) adalah referensi perencanaan untuk admin. Ini menjelaskan bagaimana konsumsi berbeda di seluruh Claude chat, Claude Code, dan Cowork, dan memberikan titik awal dolar per pengguna untuk penganggaran. Anggaran lebih banyak untuk kursi coding daripada kursi chat: setiap putaran Claude Code membawa konten file, panggilan alat, dan penalaran multi-langkah, jadi satu sesi debugging dapat mengonsumsi lebih dari sehari chat.

<h3 id="claude-console">
  Claude Console
</h3>

Organisasi API mengelola pengeluaran Claude Code melalui [ruang kerja](https://platform.claude.com/docs/en/build-with-claude/workspaces). Anda dapat [menetapkan batas pengeluaran ruang kerja](https://platform.claude.com/docs/en/build-with-claude/workspaces#workspace-limits) pada total pengeluaran Claude Code dan [melihat pelaporan biaya dan penggunaan](https://platform.claude.com/docs/en/build-with-claude/workspaces#usage-and-cost-tracking) di Console.

<Note>
  Ketika Anda pertama kali mengautentikasi Claude Code dengan akun Claude Console Anda, ruang kerja yang disebut "Claude Code" secara otomatis dibuat untuk Anda. Ruang kerja ini menyediakan pelacakan dan manajemen biaya terpusat untuk semua penggunaan Claude Code di organisasi Anda. Anda tidak dapat membuat kunci API untuk ruang kerja ini; ini secara eksklusif untuk autentikasi dan penggunaan Claude Code.

  Untuk organisasi dengan batas laju kustom, lalu lintas Claude Code di ruang kerja ini dihitung terhadap batas laju API keseluruhan organisasi Anda. Anda dapat menetapkan [batas laju ruang kerja](https://platform.claude.com/docs/id/api/rate-limits#setting-lower-limits-for-workspaces) di halaman Batas ruang kerja ini di Claude Console untuk membatasi bagian Claude Code dan melindungi beban kerja produksi lainnya.
</Note>

Untuk pelaporan per pengguna, [dashboard Console](https://platform.claude.com/claude-code) menunjukkan pengeluaran dan baris yang diterima per anggota, dan [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) mengembalikan metrik harian per pengguna yang sama secara terprogram dengan [kunci API Admin](https://platform.claude.com/settings/admin-keys). Lihat [analitik untuk pelanggan API](/docs/id/analytics#access-analytics-for-api-customers).

<h4 id="rate-limit-recommendations">
  Rekomendasi batas laju
</h4>

Saat menyiapkan Claude Code untuk tim, pertimbangkan rekomendasi Token Per Minute (TPM) dan Request Per Minute (RPM) per pengguna ini berdasarkan ukuran organisasi Anda:

| Ukuran tim       | TPM per pengguna | RPM per pengguna |
| ---------------- | ---------------- | ---------------- |
| 1-5 pengguna     | 200k-300k        | 5-7              |
| 5-20 pengguna    | 100k-150k        | 2.5-3.5          |
| 20-50 pengguna   | 50k-75k          | 1.25-1.75        |
| 50-100 pengguna  | 25k-35k          | 0.62-0.87        |
| 100-500 pengguna | 15k-20k          | 0.37-0.47        |
| 500+ pengguna    | 10k-15k          | 0.25-0.35        |

Misalnya, jika Anda memiliki 200 pengguna, Anda mungkin meminta 20k TPM untuk setiap pengguna, atau 4 juta total TPM (200\*20.000 = 4 juta).

TPM per pengguna menurun seiring pertumbuhan ukuran tim karena lebih sedikit pengguna yang cenderung menggunakan Claude Code secara bersamaan di organisasi yang lebih besar. Batas laju ini berlaku di tingkat organisasi, bukan per pengguna individual, yang berarti pengguna individual dapat sementara mengonsumsi lebih dari bagian yang dihitung mereka ketika orang lain tidak secara aktif menggunakan layanan.

<Note>
  Jika Anda mengantisipasi skenario dengan penggunaan bersamaan yang tidak biasa tinggi (seperti sesi pelatihan langsung dengan kelompok besar), Anda mungkin memerlukan alokasi TPM yang lebih tinggi per pengguna.
</Note>

<h3 id="cloud-providers">
  Penyedia cloud
</h3>

Di Amazon Bedrock, Google Cloud's Agent Platform, dan Microsoft Foundry, Claude Code ditagih per token ke akun cloud Anda, dan kontrol pengeluaran berada di konsol penagihan penyedia cloud Anda. Claude Code tidak mengirim metrik dari cloud Anda kembali ke Anthropic, jadi [dashboard analitik](/docs/id/analytics) dan Claude Code Analytics API tidak mencakup penggunaan ini.

Untuk atribusi biaya per pengguna, Anda memiliki tiga opsi:

* **OpenTelemetry**: [ekspor metrik](/docs/id/monitoring-usage) dari mesin setiap pengembang ke tumpukan observabilitas Anda sendiri. Ini memberi Anda penghitungan token per pengguna, biaya, dan aktivitas alat terlepas dari penyedia.
* **Gateway aplikasi Claude**: [gateway aplikasi Claude](/docs/id/claude-apps-gateway) yang di-host sendiri menyediakan atribusi penggunaan per pengguna, metrik OTLP dengan penghitungan token, dan [batas pengeluaran per pengguna](/docs/id/claude-apps-gateway-spend-limits) pada penyedia ini.
* **Gateway LLM**: arahkan semua lalu lintas Claude Code melalui proxy yang melacak pengeluaran per kunci. Beberapa perusahaan besar melaporkan menggunakan [LiteLLM](/docs/id/llm-gateway), alat sumber terbuka yang [melacak pengeluaran berdasarkan kunci](https://docs.litellm.ai/docs/proxy/virtual_keys#tracking-spend). Proyek ini tidak berafiliasi dengan Anthropic dan belum diaudit untuk keamanan.

<h3 id="when-a-developer-asks-about-a-limit">
  Ketika pengembang menanyakan tentang batas
</h3>

Pengembang biasanya membawa pertanyaan batas ke admin mereka, jadi membantu mengetahui batas mana yang mereka capai. Tiga situasi berarti hal yang berbeda:

* **"Anda telah mencapai batas sesi Anda" atau "Anda telah mencapai batas mingguan Anda"**: jendela penggunaan berbasis kursi pada paket berlangganan. Jendela ini dibagikan di semua model, jadi beralih model dengan `/model` tidak mengembalikan akses, meskipun itu membuat pengembang terus bekerja setelah pesan "Anda telah mencapai batas Opus Anda" khusus model. Pesan menunjukkan kapan jendela direset, dan pengembang dapat menjalankan `/usage-credits` untuk meminta penggunaan di luar tunjangan jika Anda memiliki [kredit penggunaan](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) diaktifkan. Lihat [kesalahan batas penggunaan](/docs/id/errors#youve-hit-your-session-limit).
* **Peringatan konteks atau auto-compact**: bukan batas penggunaan. Percakapan telah tumbuh dekat dengan ukuran input maksimum model, dan Claude Code merangkum riwayat yang lebih lama untuk membebaskan ruang. Arahkan pengembang ke [kurangi penggunaan token](#reduce-token-usage).
* **Pengeluaran yang tidak terduga tinggi pada paket API atau penyedia cloud**: biasanya dapat dilacak kembali ke sesi panjang yang tidak pernah dihapus atau Opus yang ditinggalkan sebagai model default. Kebiasaan berdampak tertinggi untuk dibagikan adalah membersihkan antara tugas yang tidak terkait dan mencocokkan model dengan pekerjaan, keduanya tercakup dalam [kurangi penggunaan token](#reduce-token-usage).

<h3 id="agent-team-token-costs">
  Biaya token tim agen
</h3>

[Tim agen](/docs/id/agent-teams) menjalankan beberapa instans Claude Code, masing-masing dengan jendela konteks sendiri. Penggunaan token diskalakan dengan jumlah rekan kerja aktif dan berapa lama masing-masing berjalan.

Untuk menjaga biaya tim agen tetap dapat dikelola:

* Gunakan Sonnet untuk rekan kerja. Ini menyeimbangkan kemampuan dan biaya untuk tugas koordinasi.
* Jaga tim tetap kecil. Setiap rekan kerja menjalankan jendela konteks sendiri, jadi penggunaan token kira-kira sebanding dengan ukuran tim.
* Jaga prompt spawn tetap fokus. Rekan kerja memuat CLAUDE.md, server MCP, dan skills secara otomatis, tetapi semuanya dalam prompt spawn menambah konteks mereka dari awal.
* Bersihkan tim ketika pekerjaan selesai. Setiap rekan kerja aktif terus mengonsumsi token sampai keluar atau sesi berakhir.
* Tim agen dinonaktifkan secara default. Atur `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` di [settings.json](/docs/id/settings) atau lingkungan Anda untuk mengaktifkannya. Lihat [aktifkan tim agen](/docs/id/agent-teams#enable-agent-teams).

<h2 id="reduce-token-usage">
  Kurangi penggunaan token
</h2>

Biaya token diskalakan dengan ukuran konteks: semakin banyak konteks yang diproses Claude, semakin banyak token yang Anda gunakan. Claude Code secara otomatis mengoptimalkan biaya melalui [prompt caching](/docs/id/prompt-caching), yang mengurangi biaya untuk konten berulang seperti prompt sistem, dan auto-compact, yang merangkum riwayat percakapan saat mendekati batas konteks.

Strategi berikut membantu Anda menjaga konteks tetap kecil dan mengurangi biaya per pesan.

<h3 id="manage-context-proactively">
  Kelola konteks secara proaktif
</h3>

Gunakan `/usage` untuk memeriksa penggunaan token Anda saat ini, atau [konfigurasi baris status Anda](/docs/id/statusline#context-window-usage) untuk menampilkannya secara berkelanjutan.

* **Bersihkan antar tugas**: Gunakan `/clear` untuk memulai segar saat beralih ke pekerjaan yang tidak terkait. Konteks basi membuang token pada setiap pesan berikutnya. Gunakan `/rename` sebelum membersihkan sehingga Anda dapat dengan mudah menemukan sesi nanti, kemudian `/resume` untuk kembali ke sana.
* **Tambahkan instruksi compaction kustom**: `/compact Focus on code samples and API usage` memberi tahu Claude apa yang harus dipertahankan selama perangkuman.

Anda juga dapat menyesuaikan perilaku compaction di file CLAUDE.md Anda di root proyek Anda:

```markdown theme={null}
# Compact instructions

When you are using compact, please focus on test output and code changes
```

<h3 id="choose-the-right-model">
  Pilih model yang tepat
</h3>

Sonnet menangani sebagian besar tugas pengkodean dengan baik dan biayanya lebih rendah dari Opus. Cadangkan Opus untuk keputusan arsitektur yang kompleks atau penalaran multi-langkah. Gunakan `/model` untuk beralih model di tengah sesi, atau atur default di `/config`. Untuk tugas subagent sederhana, tentukan `model: haiku` di [konfigurasi subagent](/docs/id/sub-agents#choose-a-model) Anda.

<h3 id="reduce-mcp-server-overhead">
  Kurangi overhead server MCP
</h3>

Definisi alat MCP adalah [ditunda secara default](/docs/id/mcp#scale-with-mcp-tool-search), jadi hanya nama alat yang masuk ke konteks sampai Claude menggunakan alat tertentu. Jalankan `/context` untuk melihat apa yang mengonsumsi ruang.

* **Lebih suka alat CLI jika tersedia**: Alat seperti `gh`, `aws`, `gcloud`, dan `sentry-cli` masih lebih efisien konteks daripada server MCP karena mereka tidak menambahkan daftar per-alat apa pun. Claude dapat menjalankan perintah CLI secara langsung.
* **Nonaktifkan server yang tidak digunakan**: Jalankan `/mcp` untuk melihat server yang dikonfigurasi dan nonaktifkan yang tidak Anda gunakan secara aktif.

<h3 id="install-code-intelligence-plugins-for-typed-languages">
  Instal plugin kecerdasan kode untuk bahasa yang diketik
</h3>

[Plugin kecerdasan kode](/docs/id/discover-plugins#code-intelligence) memberi Claude navigasi simbol yang tepat daripada pencarian berbasis teks, mengurangi pembacaan file yang tidak perlu saat menjelajahi kode yang tidak dikenal. Satu panggilan "go to definition" menggantikan apa yang mungkin merupakan grep diikuti dengan membaca beberapa file kandidat. Server bahasa yang diinstal juga melaporkan kesalahan tipe secara otomatis setelah pengeditan, jadi Claude menangkap kesalahan tanpa menjalankan compiler.

<h3 id="offload-processing-to-hooks-and-skills">
  Offload pemrosesan ke hooks dan skills
</h3>

[Hooks](/docs/id/hooks) kustom dapat memproses data sebelum Claude melihatnya. Alih-alih Claude membaca file log 10.000 baris untuk menemukan kesalahan, hook dapat grep untuk `ERROR` dan mengembalikan hanya baris yang cocok, mengurangi konteks dari puluhan ribu token menjadi ratusan.

[Skill](/docs/id/skills) dapat memberi Claude pengetahuan domain sehingga tidak harus menjelajahi. Misalnya, skill "codebase-overview" dapat mendeskripsikan arsitektur proyek Anda, direktori kunci, dan konvensi penamaan. Ketika Claude memanggil skill, ia mendapatkan konteks ini segera daripada menghabiskan token membaca beberapa file untuk memahami struktur.

Misalnya, hook PreToolUse ini memfilter output tes untuk menampilkan hanya kegagalan:

<Tabs>
  <Tab title="settings.json">
    Tambahkan ini ke [settings.json](/docs/id/settings#settings-files) Anda untuk menjalankan hook sebelum setiap perintah Bash:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "~/.claude/hooks/filter-test-output.sh"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="filter-test-output.sh">
    Hook memanggil skrip ini. Buat folder dengan `mkdir -p ~/.claude/hooks`, simpan skrip di bawah sebagai `~/.claude/hooks/filter-test-output.sh`, dan buat dapat dieksekusi dengan `chmod +x ~/.claude/hooks/filter-test-output.sh`. Ini memeriksa apakah perintah adalah test runner dan memodifikasinya untuk menampilkan hanya kegagalan:

    ```bash theme={null}
    #!/bin/bash
    input=$(cat)
    cmd=$(echo "$input" | jq -r '.tool_input.command')

    # If running tests, filter to show only failures
    if [[ "$cmd" =~ ^(npm test|pytest|go test) ]]; then
      filtered_cmd="$cmd 2>&1 | grep -A 5 -E '(FAIL|ERROR|error:)' | head -100"
      echo "{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"permissionDecision\":\"allow\",\"updatedInput\":{\"command\":\"$filtered_cmd\"}}}"
    else
      echo "{}"
    fi
    ```
  </Tab>
</Tabs>

<h3 id="move-instructions-from-claude-md-to-skills">
  Pindahkan instruksi dari CLAUDE.md ke skills
</h3>

File [CLAUDE.md](/docs/id/memory) Anda dimuat ke konteks saat awal sesi. Jika berisi instruksi terperinci untuk alur kerja spesifik (seperti ulasan PR atau migrasi database), token tersebut ada bahkan ketika Anda melakukan pekerjaan yang tidak terkait. [Skills](/docs/id/skills) dimuat sesuai permintaan hanya saat dipanggil, jadi memindahkan instruksi khusus ke skills menjaga konteks dasar Anda tetap lebih kecil. Bertujuan untuk menjaga CLAUDE.md di bawah 200 baris dengan hanya menyertakan hal-hal penting.

<h3 id="adjust-extended-thinking">
  Sesuaikan pemikiran yang diperluas
</h3>

Pemikiran yang diperluas diaktifkan secara default karena secara signifikan meningkatkan kinerja pada tugas perencanaan dan penalaran yang kompleks. Token pemikiran ditagih sebagai token output, dan anggaran default dapat mencapai puluhan ribu token per permintaan tergantung pada model. Untuk tugas yang lebih sederhana di mana penalaran mendalam tidak diperlukan, Anda dapat mengurangi biaya dengan menurunkan [tingkat upaya](/docs/id/model-config#adjust-effort-level) dengan `/effort` atau di `/model`, menonaktifkan pemikiran di `/config`, atau, pada model dengan [anggaran pemikiran tetap](/docs/id/model-config#adaptive-reasoning-and-fixed-thinking-budgets), menurunkan anggaran dengan menetapkan [variabel lingkungan](/docs/id/env-vars) `MAX_THINKING_TOKENS`, misalnya `MAX_THINKING_TOKENS=8000`. Model adaptive-reasoning mengabaikan anggaran bukan nol, jadi gunakan tingkat upaya di sana. Menonaktifkan pemikiran tidak tersedia di Fable 5, yang selalu menggunakan pemikiran yang diperluas.

<h3 id="delegate-verbose-operations-to-subagents">
  Delegasikan operasi verbose ke subagents
</h3>

Menjalankan tes, mengambil dokumentasi, atau memproses file log dapat mengonsumsi konteks yang signifikan. Delegasikan ini ke [subagents](/docs/id/sub-agents#isolate-high-volume-operations) sehingga output verbose tetap dalam konteks subagent sementara hanya ringkasan yang kembali ke percakapan utama Anda.

<h3 id="manage-agent-team-costs">
  Kelola biaya tim agen
</h3>

Tim agen menggunakan sekitar 7x lebih banyak token daripada sesi standar ketika rekan kerja berjalan dalam plan mode, karena setiap rekan kerja mempertahankan jendela konteks sendiri dan berjalan sebagai instans Claude terpisah. Jaga tugas tim tetap kecil dan mandiri untuk membatasi penggunaan token per rekan kerja. Lihat [tim agen](/docs/id/agent-teams) untuk detail.

<h3 id="write-specific-prompts">
  Tulis prompt spesifik
</h3>

Permintaan yang tidak jelas seperti "tingkatkan basis kode ini" memicu pemindaian luas. Permintaan spesifik seperti "tambahkan validasi input ke fungsi login di auth.ts" memungkinkan Claude bekerja secara efisien dengan pembacaan file minimal.

<h3 id="work-efficiently-on-complex-tasks">
  Bekerja secara efisien pada tugas yang kompleks
</h3>

Untuk pekerjaan yang lebih lama atau lebih kompleks, kebiasaan ini membantu menghindari token yang terbuang dari mengambil jalan yang salah:

* **Gunakan plan mode untuk tugas yang kompleks**: Tekan Shift+Tab untuk memasuki [plan mode](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode) sebelum implementasi. Claude menjelajahi basis kode dan mengusulkan pendekatan untuk persetujuan Anda, mencegah pekerjaan ulang yang mahal ketika arah awal salah.
* **Koreksi kursus lebih awal**: Jika Claude mulai menuju arah yang salah, tekan Escape untuk berhenti segera. Gunakan `/rewind` atau tekan dua kali Escape untuk mengembalikan percakapan dan kode ke checkpoint sebelumnya.
* **Berikan target verifikasi**: Sertakan kasus uji, tempel tangkapan layar, atau tentukan output yang diharapkan dalam prompt Anda. Ketika Claude dapat memverifikasi pekerjaan sendiri, ia menangkap masalah sebelum Anda perlu meminta perbaikan.
* **Uji secara bertahap**: Tulis satu file, uji, kemudian lanjutkan. Ini menangkap masalah lebih awal ketika murah untuk diperbaiki.

<h2 id="background-token-usage">
  Penggunaan token latar belakang
</h2>

Claude Code menggunakan token untuk beberapa fungsi latar belakang bahkan saat menganggur:

* **Perangkuman percakapan**: Pekerjaan latar belakang yang merangkum percakapan sebelumnya untuk fitur `claude --resume`
* **Pemrosesan perintah**: Beberapa perintah seperti `/usage` dapat menghasilkan permintaan untuk memeriksa status

Proses latar belakang ini mengonsumsi sejumlah kecil token (biasanya di bawah \$0,04 per sesi) bahkan tanpa interaksi aktif.

<h2 id="understanding-changes-in-claude-code-behavior">
  Memahami perubahan dalam perilaku Claude Code
</h2>

Claude Code secara teratur menerima pembaruan yang dapat mengubah cara fitur bekerja, termasuk pelaporan biaya. Jalankan `claude --version` untuk memeriksa versi Anda saat ini. Untuk pertanyaan penagihan spesifik, hubungi dukungan Anthropic melalui [akun Konsol](https://platform.claude.com/login) Anda.
