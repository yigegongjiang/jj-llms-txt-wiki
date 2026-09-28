> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Eskalasi keputusan sulit dengan alat advisor

> Pasangkan model utama Anda dengan model advisor yang lebih kuat yang dikonsultasikan Claude pada momen-momen kunci selama tugas.

<Note>
  Alat advisor bersifat eksperimental dan memerlukan API Anthropic. Alat ini tidak tersedia di Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform, atau Microsoft Foundry. Perilaku, harga, dan ketersediaan dapat berubah.
</Note>

Alat advisor memungkinkan Claude untuk berkonsultasi dengan model kedua yang biasanya lebih kuat pada momen-momen kunci selama tugas, seperti sebelum berkomitmen pada pendekatan, ketika terjebak pada kesalahan berulang, atau sebelum menyatakan tugas selesai. Advisor menerima percakapan lengkap, termasuk setiap pemanggilan alat dan hasilnya, dan mengembalikan panduan yang diterapkan Claude sebelum melanjutkan.

Advisor berjalan di sisi server pada infrastruktur Anthropic sebagai [server tool](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool), tersedia untuk akun berlangganan dan berbasis API. Anda memilih model mana yang bertindak sebagai advisor, dan Claude memutuskan kapan memanggilnya.

Halaman ini mencakup cara mengaktifkan advisor, pasangan model mana yang diterima, apa yang ditampilkan Claude selama konsultasi, dan bagaimana penggunaan advisor ditagih.

<h2 id="when-to-use-the-advisor">
  Kapan menggunakan advisor
</h2>

Advisor cocok untuk tugas multi-langkah yang panjang di mana sebagian besar giliran bersifat rutin tetapi kualitas rencana menentukan hasilnya. Contohnya termasuk refaktor besar, sesi debugging di mana kesalahan terus berulang, dan tugas yang ingin Anda periksa secara independen sebelum Claude menyatakan selesai.

Ini menambah nilai lebih sedikit pada tugas pendek di mana ada sedikit untuk direncanakan, atau pada pekerjaan di mana setiap giliran memerlukan model terkuat. Untuk itu, [ubah model utama](/docs/id/model-config#setting-your-model) sebagai gantinya, atau lihat [bagaimana advisor dibandingkan dengan opusplan dan subagents](#compare-with-related-features) untuk cara lain mendapatkan pendapat kedua.

<h2 id="enable-the-advisor">
  Aktifkan advisor
</h2>

Anda dapat mengatur model advisor dengan tiga cara:

* **Perintah `/advisor`**: atur atau ubah advisor di tengah sesi dan simpan sebagai default Anda
* **Pengaturan `advisorModel`**: konfigurasi default persisten di [file pengaturan](/docs/id/settings) Anda
* **Bendera `--advisor`**: atur advisor untuk sesi tunggal saat peluncuran

Masing-masing dari ini mengaktifkan advisor untuk sesi yang model utamanya [mendukungnya](#choose-an-advisor-model). Setelah sesi dimulai, Claude Code menampilkan notifikasi `Advisor Tool (experimental) is on and may use more tokens · /advisor`. Untuk berhenti menggunakan advisor, lihat [Matikan advisor](#turn-the-advisor-off).

Pada beberapa paket, Fable sebagai advisor juga memerlukan [persetujuan satu kali Anda untuk menagih penggunaan Fable ke kredit penggunaan](/docs/id/model-config#fable-and-usage-credits). Untuk apa yang terjadi sebelum Anda memberikan persetujuan itu, lihat [Advisor Fable dan kredit penggunaan](#fable-advisor-and-usage-credits).

<h3 id="use-the-/advisor-command">
  Gunakan perintah `/advisor`
</h3>

Jalankan `/advisor` tanpa argumen untuk membuka pemilih yang mencantumkan model advisor yang tersedia, atau teruskan model secara langsung:

```
/advisor opus
```

Perintah mengonfirmasi dengan `Advisor set to` diikuti dengan nama model advisor. Pilihan Anda disimpan ke `advisorModel` dalam pengaturan pengguna Anda dan bertahan di seluruh sesi, kecuali dalam kasus yang [entri `advisorModel`](/docs/id/settings-reference#advisormodel) sebutkan sebagai berlaku hanya untuk sesi saat ini.

Perintah ini juga berfungsi di mana tidak ada pemilih terminal: dalam [mode non-interaktif](/docs/id/headless) dengan `-p`, dalam Agent SDK, dalam aplikasi desktop, dan melalui [Remote Control](/docs/id/remote-control). Ini memerlukan Claude Code v2.1.260 atau lebih baru. Di permukaan tersebut:

* Jalankan `/advisor` tanpa argumen untuk mencetak model advisor saat ini dan alias yang diterimanya.
* Jalankan `/advisor` dengan model, seperti `/advisor opus`, untuk mengaturnya.
* Jalankan `/advisor off` untuk mematikannya.

Claude Code tidak memanggil advisor yang disimpan yang allowlist [`availableModels`](/docs/id/model-config#restrict-model-selection) organisasi Anda kecualikan. Untuk menggunakan advisor, pilih model yang diizinkan dengan `/advisor`. Claude Code masih menyimpan advisor yang model utama Anda saat ini tidak mendukung. Advisor itu diaktifkan setelah Anda beralih ke [model utama yang kompatibel](#choose-an-advisor-model) dengan [`/model`](/docs/id/model-config#setting-your-model). Jika API sudah menolak advisor yang disimpan dalam percakapan saat ini, itu tetap mati sampai `/clear` atau `/compact`, bahkan setelah Anda beralih model.

Pada beberapa paket, Fable sebagai advisor juga memerlukan [persetujuan satu kali Anda untuk menagih penggunaan Fable ke kredit penggunaan](/docs/id/model-config#fable-and-usage-credits). Untuk apa yang dilakukan `/advisor fable` sebelum Anda memberikan persetujuan itu, lihat [Advisor Fable dan kredit penggunaan](#fable-advisor-and-usage-credits).

<h3 id="set-advisormodel-in-settings">
  Atur `advisorModel` dalam pengaturan
</h3>

Untuk mengonfigurasi advisor sebagai default tanpa membuka sesi, aturnya di file pengaturan Anda:

```json theme={null}
{
  "advisorModel": "opus"
}
```

<h3 id="use-the-advisor-flag">
  Gunakan bendera `--advisor`
</h3>

Untuk mengatur advisor untuk sesi tunggal tanpa mengubah pengaturan yang disimpan, luncurkan dengan bendera:

```bash theme={null}
claude --advisor opus
```

Claude Code menggunakan bendera sebagai pengganti pengaturan `advisorModel` untuk sesi itu. Ini tidak mencantumkan `--advisor` dalam `claude --help`. Claude Code keluar dengan kesalahan saat peluncuran jika:

* Model utama sesi tidak mendukung advisor
* Model yang diminta, seperti Haiku, tidak dapat bertindak sebagai advisor
* Allowlist [`availableModels`](/docs/id/model-config#restrict-model-selection) organisasi Anda mengecualikan model yang diminta
* Anda meminta Fable dan akun Anda masih memerlukan [persetujuan kredit penggunaan](#fable-advisor-and-usage-credits)

Jika Anda memulai [sesi latar belakang](/docs/id/agent-view) dengan `--advisor` dan salah satu dari ini berlaku, Claude Code memulai sesi tanpa advisor sebagai gantinya dari keluar.

<h2 id="choose-an-advisor-model">
  Pilih model advisor
</h2>

Advisor harus setidaknya sama mampu dengan model utama. Advisor yang diterima untuk setiap model utama adalah:

| Model utama            | Advisor yang diterima                     | Catatan                                                                              |
| ---------------------- | ----------------------------------------- | ------------------------------------------------------------------------------------ |
| Haiku 4.5              | Fable, Opus, Sonnet                       | Haiku dapat memanggil advisor tetapi tidak dapat bertindak sebagai advisor           |
| Sonnet 4.6             | Fable, Opus, Sonnet                       |                                                                                      |
| Sonnet 5               | Fable, Opus 4.7 atau lebih baru, Sonnet 5 | Advisor Sonnet 4.6 ditolak, dan API menolak advisor Opus 4.6                         |
| Opus 4.6               | Fable, Opus, Sonnet 5                     | Advisor Sonnet 4.6 ditolak                                                           |
| Opus 4.7 atau Opus 4.8 | Fable, dan Opus 4.7 atau lebih baru       | Advisor Opus 4.6 atau Sonnet ditolak                                                 |
| Opus 5.5 atau Opus 5   | Fable, dan Opus 5 atau lebih baru         | Advisor Opus 4.6 atau Sonnet ditolak, dan API menolak advisor Opus 4.7 atau Opus 4.8 |
| Fable 5                | Fable 5.1 atau Fable 5                    | Advisor Opus atau Sonnet ditolak                                                     |
| Fable 5.1              | Fable 5.1                                 | Advisor Opus atau Sonnet ditolak, dan API menolak advisor Fable 5                    |

Fable 5.1 memerlukan Claude Code v2.1.257 atau lebih baru. Kedua model Fable memerlukan [akses Fable](/docs/id/model-config#work-with-fable).

Atur advisor sebagai `fable`, `opus`, atau `sonnet`. Alias ini diselesaikan ke versi default bawaan Claude Code untuk setiap keluarga model, yang berkembang dengan rilis Claude Code baru. Anda juga dapat melewatkan ID model lengkap seperti `claude-opus-5-5`.

Subagent mewarisi advisor yang dikonfigurasi dan menerapkan pemeriksaan pasangan yang sama terhadap model mereka sendiri.

Claude Code memvalidasi pasangan sebelum mengirim permintaan, dan API memvalidasinya lagi:

* Untuk advisor yang tabel cantumkan sebagai ditolak, Claude Code tidak melampirkannya ke permintaan model utama. Output perintah `/advisor` dan notifikasi menunjukkan hal ini. Subagent yang model mereka sendiri memenuhi pasangan mungkin masih menggunakan advisor.
* Untuk advisor yang tabel cantumkan sebagai ditolak oleh API, Claude Code melampirkannya dan API menolaknya. Claude Code kemudian mengirim ulang permintaan tersebut tanpa advisor, dan sisa percakapan berjalan tanpa advisor, sehingga Anda tidak melihat kesalahan dan tidak mendapatkan panggilan advisor. Pilih advisor yang diterima dengan `/advisor`; perubahan berlaku setelah `/clear` atau `/compact` dan dalam sesi baru.
* Jika model utama atau advisor adalah model yang Claude Code tidak kenali, advisor tidak dilampirkan.

<h3 id="fable-advisor-and-usage-credits">
  Advisor Fable dan kredit penggunaan
</h3>

Pada beberapa paket, penggunaan Fable ditagihkan ke kredit penggunaan, dan Fable sebagai advisor ditagihkan dengan cara yang sama. Jika akun Anda memerlukan [persetujuan satu kali untuk menagihkan penggunaan Fable ke kredit penggunaan](/docs/id/model-config#fable-and-usage-credits), Claude Code memintanya ketika Anda memilih model Fable dengan `/model` dan tidak menerapkan Fable sebagai advisor sampai Anda telah menerima persetujuan tersebut.

Sebelum Anda menerimanya, Claude Code tidak menyimpan Fable sebagai advisor ketika Anda mengetik `/advisor fable` atau memilih Fable di pemilih `/advisor`. Ini mengarahkan Anda ke `/model fable` sebagai gantinya. Dengan `claude --advisor fable`, Claude Code keluar saat peluncuran dengan pesan yang mengarahkan ke `/model fable`. Dalam [sesi latar belakang](#use-the-advisor-flag), sesi dimulai tanpa advisor alih-alih keluar. Dengan Fable yang sudah disimpan sebagai `advisorModel` Anda, Claude Code mengirim permintaan tanpa advisor. Dalam sesi interaktif yang model utamanya mendukung advisor, sesi juga menampilkan notifikasi yang mengarahkan ke `/model fable`.

Untuk menerima persetujuan, jalankan `/model fable` dan pilih untuk melanjutkan di Fable. Claude Code mencatat persetujuan dan [menyimpan Fable sebagai model pilihan Anda](/docs/id/model-config#default-model-setting). Kemudian pilih Fable sebagai advisor.

<h3 id="common-model-pairings">
  Pasangan model umum
</h3>

Pasangan apa pun yang diterima berfungsi. Kombinasi ini menyeimbangkan biaya terhadap kemampuan dengan cara yang berbeda:

| Pasangan                      | Kapan menggunakan                                                                                                                                                                    |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Sonnet utama + advisor Opus   | Sonnet menangani pekerjaan rutin dan eskalasi perencanaan, kegagalan ambigu, dan pemeriksaan penyelesaian ke Opus                                                                    |
| Sonnet utama + advisor Fable  | Panduan Fable pada titik keputusan tanpa menjalankan Fable di seluruh. Memerlukan akses Fable                                                                                        |
| Haiku utama + advisor Opus    | Model utama dengan biaya terendah dengan perencanaan yang kuat. Harapkan biaya lebih tinggi daripada Haiku saja tetapi lebih rendah daripada beralih model utama ke Sonnet atau Opus |
| Opus utama + advisor Opus     | Opus kedua meninjau yang pertama. Berguna untuk tugas berisiko tinggi di mana pemeriksaan independen lebih penting daripada biaya                                                    |
| Fable utama + advisor Fable   | Pasangan kemampuan tertinggi ketika Fable tersedia. Claude Code tidak menerapkan advisor Opus atau Sonnet ke model utama Fable                                                       |
| Sonnet utama + advisor Sonnet | Pendapat kedua dengan biaya lebih rendah untuk menangkap pengawasan rutin                                                                                                            |

<h2 id="when-claude-consults-the-advisor">
  Kapan Claude berkonsultasi dengan advisor
</h2>

Claude memutuskan kapan memanggil advisor. Cenderung berkonsultasi sebelum berkomitmen pada pendekatan, ketika kesalahan terus berulang, dan sebelum menyatakan tugas selesai, tetapi waktu didorong oleh model daripada berbasis aturan.

Anda dapat meminta konsultasi dalam prompt Anda dengan cara yang sama seperti Anda meminta alat apa pun, misalnya `consult the advisor before you continue`. Tidak ada pengaturan untuk membatasi atau memaksa panggilan advisor; jika Anda ingin Claude berkonsultasi lebih atau kurang sering selama tugas, katakan dalam instruksi Anda.

<h2 id="what-you-see-during-a-session">
  Apa yang Anda lihat selama sesi
</h2>

Ketika Claude memanggil advisor, transkrip menampilkan baris `Advising` dengan nama model advisor saat panggilan sedang berlangsung. Ketika hasilnya kembali, baris melaporkan apakah advisor memberikan panduan:

* **Reviewed**: baris mengkonfirmasi bahwa advisor telah meninjau percakapan. Ketika advisor mengembalikan panduan yang dapat dibaca, tekan `Ctrl+O` untuk membacanya.
* **Declined**: baris berbunyi `Advisor declined to advise on this request`. Jika advisor memberikan alasan, tekan `Ctrl+O` untuk membacanya.

Claude umumnya mengikuti panduan advisor, tetapi beradaptasi ketika bukti miliknya bertentangan dengan klaim spesifik: jika langkah yang direkomendasikan gagal saat dicoba, atau isi file bertentangan dengan saran, Claude menampilkan konflik daripada mengikuti panduan secara tidak terbatas.

Advisor selalu menerima percakapan lengkap, dan Claude mengontrol waktu. Untuk kontrol lebih atau konfigurasi berbeda, lihat [bagaimana advisor dibandingkan dengan subagents dan opusplan](#compare-with-related-features).

<h2 id="cost">
  Biaya
</h2>

Ketika Claude memanggil advisor, model advisor membaca percakapan, jadi setiap panggilan mengonsumsi token pada tarif model advisor sebagai tambahan dari penggunaan model utama Anda. Bagaimana token advisor ditagih tergantung pada cara Anda membayar:

* **Penagihan API**: Anda membayar tarif input dan output model advisor untuk token advisor
* **Paket berlangganan**: penggunaan advisor dihitung terhadap batas penggunaan paket Anda, kecuali bahwa advisor Fable ditagih ke [kredit penggunaan](/docs/id/model-config#fable-and-usage-credits) pada paket di mana penggunaan Fable melakukannya

Jika akun Anda memerlukan persetujuan kredit penggunaan, advisor Fable tidak ditagih apa pun sebelum Anda memberikannya, karena Claude Code [tidak menerapkan pilihan](#fable-advisor-and-usage-credits) sampai saat itu.

Claude memanggil advisor pada titik keputusan daripada pada setiap giliran, jadi memasangkan model utama yang lebih cepat dengan advisor yang lebih kuat biasanya biaya lebih rendah daripada menjalankan model yang lebih kuat di seluruh. Penggunaan advisor dihitung terhadap total sesi yang ditampilkan oleh [`/usage`](/docs/id/costs#track-your-costs).

Untuk bagaimana token advisor dilaporkan dalam respons API, lihat [Usage and billing](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool#usage-and-billing) dalam dokumentasi Claude API.

<h2 id="impact-on-prompt-caching">
  Dampak pada prompt caching
</h2>

Mengaktifkan atau menonaktifkan advisor di tengah sesi tidak membatalkan [prompt cache](/docs/id/prompt-caching) model utama Anda. Tidak seperti [mengubah model](/docs/id/prompt-caching#switching-models), mengalihkan `/advisor` menjaga awalan yang di-cache tetap utuh, dan panduan yang dikembalikan advisor di-cache sebagai bagian dari transkrip pada giliran berikutnya.

Pembacaan percakapan model advisor sendiri tidak di-cache. Setiap panggilan advisor memproses transkrip lengkap baru, tanpa penggunaan kembali di antara panggilan.

<h2 id="requirements">
  Persyaratan
</h2>

Alat advisor memerlukan semua hal berikut:

* **Hanya API Anthropic**: advisor adalah alat yang dieksekusi server. Alat ini tidak tersedia di Amazon Bedrock, Claude Platform on AWS, Google Cloud's Agent Platform, atau Microsoft Foundry. Melalui [LLM gateway](/docs/id/llm-gateway) yang dikonfigurasi dengan `ANTHROPIC_BASE_URL`, ketersediaan tergantung pada apakah gateway meneruskan permintaan utuh ke API Anthropic. Jika gateway atau upstream-nya tidak mengenali alat advisor, lihat [Pengulangan otomatis dan penerusan kesalahan](/docs/id/llm-gateway-protocol#automatic-retry-and-error-forwarding) untuk mengetahui bagaimana Claude Code merespons.
* **Model utama yang didukung**: Fable, Opus 4.6 atau lebih baru, Sonnet 4.6 atau lebih baru, atau Haiku 4.5. Lihat [Pilih model advisor](#choose-an-advisor-model) untuk mengetahui advisor mana yang diterima masing-masing.
* **Pengambilan bendera fitur**: Claude Code mengaktifkan advisor melalui bendera fitur yang diambilnya dari Anthropic. Dalam sesi di mana variabel yang mematikan pengambilan bendera diatur, seperti `DISABLE_TELEMETRY`, advisor tetap mati. Lihat [Fitur yang memerlukan pengambilan bendera fitur](/docs/id/env-vars#features-that-need-feature-flag-fetching).

<h2 id="turn-the-advisor-off">
  Matikan advisor
</h2>

Untuk berhenti menggunakan advisor, jalankan `/advisor off` atau pilih **No advisor** di pemilih `/advisor`:

```
/advisor off
```

Untuk menonaktifkan alat advisor sepenuhnya, atur `CLAUDE_CODE_DISABLE_ADVISOR_TOOL=1`. Perintah `/advisor` menjadi tidak tersedia dan `advisorModel` yang dikonfigurasi apa pun diabaikan. Bendera `--advisor` diterima tetapi tidak memiliki efek. Lihat [Environment variables](/docs/id/env-vars).

<h2 id="compare-with-related-features">
  Bandingkan dengan fitur terkait
</h2>

Advisor adalah salah satu dari beberapa cara untuk menggabungkan kekuatan model. Pilih berdasarkan kapan Anda ingin model kedua terlibat.

| Pendekatan                                                       | Kapan model yang lebih kuat berjalan                                                                                                                | Bagaimana dimulai                                   |
| ---------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------- |
| Alat advisor                                                     | Pada titik keputusan di tengah tugas                                                                                                                | Claude memanggilnya ketika memerlukan panduan       |
| [`opusplan`](/docs/id/model-config#opusplan-model-setting)            | Selama mode rencana ketika [diizinkan oleh `availableModels`](/docs/id/model-config#restrict-model-selection), kemudian beralih ke Sonnet untuk eksekusi | Anda memasuki mode rencana                          |
| [Subagents](/docs/id/sub-agents#choose-a-model) dengan `model` diatur | Untuk seluruh subtask yang didelegasikan                                                                                                            | Claude mendelegasikan, atau Anda memanggil subagent |
| [`/model`](/docs/id/model-config#setting-your-model)                  | Dari permintaan berikutnya dan seterusnya                                                                                                           | Anda beralih model                                  |

<h2 id="see-also">
  Lihat juga
</h2>

* [Model configuration](/docs/id/model-config): ubah model, atur tingkat upaya, dan gunakan `opusplan`
* [Manage costs effectively](/docs/id/costs): lacak penggunaan token di seluruh model
* [Advisor tool in the Claude API](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool): pahami alat server yang mendasar, atau gunakan langsung dari Messages API
* [The advisor strategy](https://claude.com/blog/the-advisor-strategy): mengapa memasangkan model utama yang cepat dengan advisor yang lebih kuat berfungsi
